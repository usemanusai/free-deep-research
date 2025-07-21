// Comprehensive Tests for Event Store Implementation
// Phase 4.1: Event Sourcing Foundation

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_store::{
        EventStore, EventStoreConfig,
        events::{ResearchWorkflowEvent, AIAgentEvent, ResearchMethodology, EventFactory},
        aggregates::{ResearchWorkflowAggregate, AggregateRoot},
        serialization::JsonEventSerializer,
        snapshots::{SnapshotStore, PostgresSnapshotStorage, SnapshotConfig},
        replay::{EventReplayService, ReplayConfig},
        error::EventStoreError,
    };
    use chrono::Utc;
    use sqlx::PgPool;
    use std::sync::Arc;
    use uuid::Uuid;

    // Test utilities
    pub async fn setup_test_db() -> PgPool {
        // In a real test environment, you'd set up a test database
        // For now, we'll use a mock or skip database-dependent tests
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://test:test@localhost/test_fdr".to_string());
        
        match PgPool::connect(&database_url).await {
            Ok(pool) => {
                // Run migrations
                sqlx::migrate!("../../infrastructure/database/migrations")
                    .run(&pool)
                    .await
                    .expect("Failed to run migrations");
                pool
            }
            Err(_) => {
                // Skip database tests if no test database available
                panic!("Test database not available. Set TEST_DATABASE_URL environment variable.");
            }
        }
    }

    #[tokio::test]
    async fn test_event_store_creation() {
        let pool = setup_test_db().await;
        let config = EventStoreConfig::default();
        let serializer = Arc::new(JsonEventSerializer::new());
        
        let event_store = EventStore::new(pool, config, serializer);
        
        // Test that event store was created successfully
        assert_eq!(event_store.config.snapshot_frequency, 100);
    }

    #[tokio::test]
    async fn test_append_and_read_events() {
        let pool = setup_test_db().await;
        let config = EventStoreConfig::default();
        let serializer = Arc::new(JsonEventSerializer::new());
        let event_store = EventStore::new(pool, config, serializer);

        let stream_id = Uuid::new_v4();
        let methodology = ResearchMethodology {
            name: "Test Method".to_string(),
            steps: vec!["Step 1".to_string(), "Step 2".to_string()],
            ai_agents: vec!["agent1".to_string()],
            estimated_duration_minutes: 30,
        };

        let events: Vec<Box<dyn DomainEvent>> = vec![
            Box::new(ResearchWorkflowEvent::WorkflowCreated {
                workflow_id: stream_id,
                name: "Test Workflow".to_string(),
                query: "Test Query".to_string(),
                methodology: methodology.clone(),
                created_at: Utc::now(),
                correlation_id: Some(Uuid::new_v4()),
            }),
            Box::new(ResearchWorkflowEvent::ExecutionStarted {
                workflow_id: stream_id,
                started_at: Utc::now(),
                correlation_id: Some(Uuid::new_v4()),
            }),
        ];

        // Append events
        let version = event_store
            .append_events(stream_id, events, None)
            .await
            .expect("Failed to append events");

        assert_eq!(version, 2);

        // Read events back
        let read_events = event_store
            .read_events(stream_id, None, None)
            .await
            .expect("Failed to read events");

        assert_eq!(read_events.len(), 2);
        assert_eq!(read_events[0].event_type(), "research.workflow.created");
        assert_eq!(read_events[1].event_type(), "research.workflow.started");
    }

    #[tokio::test]
    async fn test_optimistic_concurrency_control() {
        let pool = setup_test_db().await;
        let config = EventStoreConfig::default();
        let serializer = Arc::new(JsonEventSerializer::new());
        let event_store = EventStore::new(pool, config, serializer);

        let stream_id = Uuid::new_v4();
        let methodology = ResearchMethodology {
            name: "Test Method".to_string(),
            steps: vec![],
            ai_agents: vec![],
            estimated_duration_minutes: 10,
        };

        let event: Vec<Box<dyn DomainEvent>> = vec![
            Box::new(ResearchWorkflowEvent::WorkflowCreated {
                workflow_id: stream_id,
                name: "Test Workflow".to_string(),
                query: "Test Query".to_string(),
                methodology,
                created_at: Utc::now(),
                correlation_id: None,
            }),
        ];

        // First append should succeed
        let version1 = event_store
            .append_events(stream_id, event.clone(), Some(0))
            .await
            .expect("First append should succeed");

        assert_eq!(version1, 1);

        // Second append with wrong expected version should fail
        let result = event_store
            .append_events(stream_id, event, Some(0))
            .await;

        assert!(matches!(result, Err(EventStoreError::ConcurrencyConflict { .. })));
    }

    #[tokio::test]
    async fn test_aggregate_creation_and_operations() {
        let id = Uuid::new_v4();
        let methodology = ResearchMethodology {
            name: "Deep Research".to_string(),
            steps: vec!["Search".to_string(), "Analyze".to_string(), "Synthesize".to_string()],
            ai_agents: vec!["researcher".to_string(), "analyst".to_string()],
            estimated_duration_minutes: 45,
        };

        // Create aggregate
        let mut aggregate = ResearchWorkflowAggregate::create_workflow(
            id,
            "Test Research Workflow".to_string(),
            "What are the latest trends in AI?".to_string(),
            methodology,
        ).expect("Failed to create workflow");

        assert_eq!(aggregate.get_id(), id);
        assert_eq!(aggregate.get_version(), 1);
        assert_eq!(aggregate.get_uncommitted_events().len(), 1);

        // Start execution
        aggregate.start_execution().expect("Failed to start execution");
        assert_eq!(aggregate.get_version(), 2);
        assert_eq!(aggregate.get_uncommitted_events().len(), 2);

        // Create task
        let task_id = Uuid::new_v4();
        aggregate.create_task(
            task_id,
            "web_search".to_string(),
            Some("researcher".to_string()),
        ).expect("Failed to create task");

        assert_eq!(aggregate.get_version(), 3);
        assert_eq!(aggregate.state.tasks.len(), 1);

        // Complete task
        let task_results = serde_json::json!({
            "findings": ["AI is advancing rapidly", "Machine learning is mainstream"],
            "sources": ["https://example.com/ai-trends"],
            "confidence": 0.85
        });

        aggregate.complete_task(task_id, task_results).expect("Failed to complete task");
        assert_eq!(aggregate.get_version(), 4);

        let task = aggregate.get_task(task_id).expect("Task should exist");
        assert_eq!(task.status, crate::event_store::aggregates::TaskStatus::Completed);
    }

    #[tokio::test]
    async fn test_snapshot_functionality() {
        let pool = setup_test_db().await;
        let storage = Arc::new(PostgresSnapshotStorage::new(pool));
        let config = SnapshotConfig::default();
        let snapshot_store = SnapshotStore::with_config(storage, config);

        let stream_id = Uuid::new_v4();
        let snapshot_data = serde_json::json!({
            "id": stream_id,
            "name": "Test Workflow",
            "status": "running",
            "version": 5
        });

        // Save snapshot
        snapshot_store
            .save_snapshot(stream_id, snapshot_data.clone(), 5)
            .await
            .expect("Failed to save snapshot");

        // Load snapshot
        let loaded = snapshot_store
            .load_latest_snapshot(stream_id)
            .await
            .expect("Failed to load snapshot");

        assert!(loaded.is_some());
        let (loaded_data, version) = loaded.unwrap();
        assert_eq!(loaded_data, snapshot_data);
        assert_eq!(version, 5);

        // Test snapshot statistics
        let stats = snapshot_store
            .get_stats(stream_id)
            .await
            .expect("Failed to get stats");

        assert_eq!(stats.total_snapshots, 1);
        assert_eq!(stats.latest_version, 5);
    }

    #[tokio::test]
    async fn test_event_serialization() {
        let serializer = JsonEventSerializer::new();
        let methodology = ResearchMethodology {
            name: "Test Method".to_string(),
            steps: vec!["Step 1".to_string()],
            ai_agents: vec!["agent1".to_string()],
            estimated_duration_minutes: 20,
        };

        let event = ResearchWorkflowEvent::WorkflowCreated {
            workflow_id: Uuid::new_v4(),
            name: "Test Workflow".to_string(),
            query: "Test Query".to_string(),
            methodology,
            created_at: Utc::now(),
            correlation_id: Some(Uuid::new_v4()),
        };

        // Test serialization
        let serialized = serializer
            .serialize(&event)
            .expect("Failed to serialize event");

        assert!(serialized.is_object());
        assert!(serialized.get("workflow_id").is_some());
        assert!(serialized.get("name").is_some());

        // Test deserialization
        let metadata = crate::event_store::events::EventMetadata {
            event_id: Uuid::new_v4(),
            stream_id: Uuid::new_v4(),
            event_type: "research.workflow.created".to_string(),
            event_version: 1,
            sequence_number: 1,
            timestamp: Utc::now(),
            correlation_id: None,
            causation_id: None,
        };

        let deserialized = serializer
            .deserialize(&serialized, "research.workflow.created", metadata)
            .expect("Failed to deserialize event");

        assert_eq!(deserialized.event_type(), "research.workflow.created");
    }

    #[tokio::test]
    async fn test_event_factory() {
        let correlation_id = Uuid::new_v4();
        let factory = EventFactory::with_correlation_id(correlation_id);

        let workflow_id = Uuid::new_v4();
        let methodology = ResearchMethodology {
            name: "Factory Test".to_string(),
            steps: vec![],
            ai_agents: vec![],
            estimated_duration_minutes: 15,
        };

        let event = factory.create_workflow_created(
            workflow_id,
            "Factory Test Workflow".to_string(),
            "Factory Test Query".to_string(),
            methodology,
        );

        assert_eq!(event.correlation_id(), Some(correlation_id));
        assert_eq!(event.event_type(), "research.workflow.created");
    }

    #[tokio::test]
    async fn test_error_handling() {
        // Test invalid aggregate operations
        let id = Uuid::new_v4();
        
        // Empty name should fail
        let result = ResearchWorkflowAggregate::create_workflow(
            id,
            "".to_string(),
            "Valid Query".to_string(),
            ResearchMethodology {
                name: "Test".to_string(),
                steps: vec![],
                ai_agents: vec![],
                estimated_duration_minutes: 10,
            },
        );
        assert!(result.is_err());

        // Empty query should fail
        let result = ResearchWorkflowAggregate::create_workflow(
            id,
            "Valid Name".to_string(),
            "".to_string(),
            ResearchMethodology {
                name: "Test".to_string(),
                steps: vec![],
                ai_agents: vec![],
                estimated_duration_minutes: 10,
            },
        );
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_event_validation() {
        let methodology = ResearchMethodology {
            name: "Test Method".to_string(),
            steps: vec![],
            ai_agents: vec![],
            estimated_duration_minutes: 10,
        };

        // Valid event
        let valid_event = ResearchWorkflowEvent::WorkflowCreated {
            workflow_id: Uuid::new_v4(),
            name: "Valid Name".to_string(),
            query: "Valid Query".to_string(),
            methodology: methodology.clone(),
            created_at: Utc::now(),
            correlation_id: None,
        };
        assert!(valid_event.validate().is_ok());

        // Invalid event - empty name
        let invalid_event = ResearchWorkflowEvent::WorkflowCreated {
            workflow_id: Uuid::new_v4(),
            name: "".to_string(),
            query: "Valid Query".to_string(),
            methodology,
            created_at: Utc::now(),
            correlation_id: None,
        };
        assert!(invalid_event.validate().is_err());
    }

    #[tokio::test]
    async fn test_integration_workflow() {
        let pool = setup_test_db().await;
        let config = EventStoreConfig::default();
        let serializer = Arc::new(JsonEventSerializer::new());
        let event_store = Arc::new(EventStore::new(pool, config, serializer));

        // Create and save aggregate
        let workflow_id = Uuid::new_v4();
        let methodology = ResearchMethodology {
            name: "Integration Test Method".to_string(),
            steps: vec!["Search".to_string(), "Analyze".to_string()],
            ai_agents: vec!["researcher".to_string()],
            estimated_duration_minutes: 30,
        };

        let mut aggregate = ResearchWorkflowAggregate::create_workflow(
            workflow_id,
            "Integration Test Workflow".to_string(),
            "Integration test query".to_string(),
            methodology,
        ).expect("Failed to create workflow");

        // Start execution and create tasks
        aggregate.start_execution().expect("Failed to start execution");
        
        let task_id = Uuid::new_v4();
        aggregate.create_task(
            task_id,
            "search".to_string(),
            Some("researcher".to_string()),
        ).expect("Failed to create task");

        // Save events to event store
        let events: Vec<Box<dyn DomainEvent>> = aggregate
            .get_uncommitted_events()
            .iter()
            .map(|e| Box::new(e.clone()) as Box<dyn DomainEvent>)
            .collect();

        let version = event_store
            .append_events(workflow_id, events, None)
            .await
            .expect("Failed to append events");

        assert_eq!(version, 3); // WorkflowCreated + ExecutionStarted + TaskCreated

        // Read events back and rebuild aggregate
        let read_events = event_store
            .read_events(workflow_id, None, None)
            .await
            .expect("Failed to read events");

        assert_eq!(read_events.len(), 3);

        // Verify we can rebuild the aggregate state from events
        let mut rebuilt_aggregate = ResearchWorkflowAggregate::restore_from_state(
            workflow_id,
            crate::event_store::aggregates::ResearchWorkflowState::default(),
            0,
        );

        for event in read_events {
            if let Ok(workflow_event) = event.serialize() {
                if let Ok(deserialized_event) = serde_json::from_value::<ResearchWorkflowEvent>(workflow_event) {
                    rebuilt_aggregate.apply_event(&deserialized_event);
                }
            }
        }

        assert_eq!(rebuilt_aggregate.get_version(), 3);
        assert_eq!(rebuilt_aggregate.state.name, "Integration Test Workflow");
        assert_eq!(rebuilt_aggregate.state.tasks.len(), 1);
    }

    // Performance tests
    #[tokio::test]
    async fn test_event_store_performance() {
        let pool = setup_test_db().await;
        let config = EventStoreConfig::default();
        let serializer = Arc::new(JsonEventSerializer::new());
        let event_store = EventStore::new(pool, config, serializer);

        let start_time = std::time::Instant::now();
        let num_events = 100;

        for i in 0..num_events {
            let stream_id = Uuid::new_v4();
            let methodology = ResearchMethodology {
                name: format!("Performance Test {}", i),
                steps: vec![],
                ai_agents: vec![],
                estimated_duration_minutes: 10,
            };

            let events: Vec<Box<dyn DomainEvent>> = vec![
                Box::new(ResearchWorkflowEvent::WorkflowCreated {
                    workflow_id: stream_id,
                    name: format!("Performance Test Workflow {}", i),
                    query: format!("Performance test query {}", i),
                    methodology,
                    created_at: Utc::now(),
                    correlation_id: None,
                }),
            ];

            event_store
                .append_events(stream_id, events, None)
                .await
                .expect("Failed to append events");
        }

        let duration = start_time.elapsed();
        let events_per_second = num_events as f64 / duration.as_secs_f64();

        println!("Performance: {} events/second", events_per_second);
        
        // Assert reasonable performance (adjust threshold as needed)
        assert!(events_per_second > 10.0, "Event store performance too slow");
    }
}
