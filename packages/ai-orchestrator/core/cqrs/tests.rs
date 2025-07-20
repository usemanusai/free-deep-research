// Comprehensive Tests for CQRS Implementation
// Phase 4.2: CQRS Pattern Implementation

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cqrs::{
        CQRSService, CQRSServiceBuilder, CQRSConfig,
        commands::{CommandFactory, CreateResearchWorkflowCommand},
        queries::{QueryFactory, GetResearchWorkflowQuery, GetWorkflowListQuery},
        handlers::{
            CreateResearchWorkflowHandler, GetResearchWorkflowHandler, GetWorkflowListHandler,
        },
        read_models::{MockReadModelStore, ResearchWorkflowReadModel, WorkflowStatus, WorkflowMetrics},
        projections::{ResearchWorkflowProjectionBuilder, ProjectionManager},
        error::CQRSError,
    };
    use crate::event_store::{EventStore, EventStoreConfig, JsonEventSerializer};
    use crate::event_store::events::ResearchMethodology;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use uuid::Uuid;
    use chrono::Utc;

    async fn setup_test_event_store() -> Arc<EventStore> {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://test:test@localhost/test_fdr".to_string());
        
        let pool = sqlx::PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to test database");
        
        let config = EventStoreConfig::default();
        let serializer = Arc::new(JsonEventSerializer::new());
        
        Arc::new(EventStore::new(pool, config, serializer))
    }

    async fn setup_cqrs_service() -> CQRSService {
        let read_model_store = Arc::new(RwLock::new(MockReadModelStore::new()));
        let config = CQRSConfig::default();
        
        CQRSService::new(read_model_store, config)
    }

    #[tokio::test]
    async fn test_cqrs_service_creation() {
        let read_model_store = Arc::new(RwLock::new(MockReadModelStore::new()));
        
        let cqrs_service = CQRSServiceBuilder::new()
            .with_command_timeout(30)
            .with_query_timeout(10)
            .with_projection_batch_size(100)
            .enable_caching(true)
            .with_read_model_store(read_model_store)
            .build()
            .expect("Failed to build CQRS service");

        assert_eq!(cqrs_service.config.command_timeout_seconds, 30);
        assert_eq!(cqrs_service.config.query_timeout_seconds, 10);
        assert_eq!(cqrs_service.config.projection_batch_size, 100);
        assert!(cqrs_service.config.enable_query_caching);
    }

    #[tokio::test]
    async fn test_command_execution_flow() {
        let event_store = setup_test_event_store().await;
        let cqrs_service = setup_cqrs_service().await;
        
        // Register command handler
        let create_handler = CreateResearchWorkflowHandler::new(event_store);
        cqrs_service.register_command_handler::<CreateResearchWorkflowCommand, _>(create_handler).await;
        
        // Create command
        let command_factory = CommandFactory::new();
        let methodology = ResearchMethodology {
            name: "Test Method".to_string(),
            steps: vec!["Step 1".to_string(), "Step 2".to_string()],
            ai_agents: vec!["agent1".to_string()],
            estimated_duration_minutes: 30,
        };
        
        let command = command_factory.create_research_workflow(
            Uuid::new_v4(),
            "Test Workflow".to_string(),
            "Test Query".to_string(),
            methodology,
        );
        
        // Execute command
        let result = cqrs_service.execute_command(command).await;
        assert!(result.is_ok());
        
        let command_result = result.unwrap();
        assert!(command_result.success);
        assert!(command_result.aggregate_id.is_some());
        assert_eq!(command_result.version, Some(1));
    }

    #[tokio::test]
    async fn test_query_execution_flow() {
        let cqrs_service = setup_cqrs_service().await;
        
        // Register query handler
        let read_model_store = Arc::new(RwLock::new(MockReadModelStore::new()));
        let get_handler = GetResearchWorkflowHandler::new(read_model_store);
        cqrs_service.register_query_handler::<GetResearchWorkflowQuery, _>(get_handler).await;
        
        // Create query
        let query_factory = QueryFactory::new();
        let query = query_factory.get_research_workflow(Uuid::new_v4(), true);
        
        // Execute query
        let result = cqrs_service.execute_query(query).await;
        assert!(result.is_ok());
        
        // Should return None for non-existent workflow
        let workflow = result.unwrap();
        assert!(workflow.is_none());
    }

    #[tokio::test]
    async fn test_workflow_list_query() {
        let cqrs_service = setup_cqrs_service().await;
        
        // Register query handler
        let read_model_store = Arc::new(RwLock::new(MockReadModelStore::new()));
        let list_handler = GetWorkflowListHandler::new(read_model_store);
        cqrs_service.register_query_handler::<GetWorkflowListQuery, _>(list_handler).await;
        
        // Create query
        let query_factory = QueryFactory::new();
        let query = query_factory.get_workflow_list(1, 10, None, None, None, None);
        
        // Execute query
        let result = cqrs_service.execute_query(query).await;
        assert!(result.is_ok());
        
        let workflow_list = result.unwrap();
        assert_eq!(workflow_list.workflows.len(), 0);
        assert_eq!(workflow_list.total_count, 0);
        assert_eq!(workflow_list.page, 1);
        assert_eq!(workflow_list.page_size, 10);
    }

    #[tokio::test]
    async fn test_projection_system() {
        let read_model_store = Arc::new(RwLock::new(MockReadModelStore::new()));
        let config = CQRSConfig::default();
        let mut projection_manager = ProjectionManager::new(config);
        
        // Register projection
        let projection = Box::new(ResearchWorkflowProjectionBuilder::new());
        projection_manager.register_projection(projection).await;
        
        // Check status
        let status = projection_manager.get_status().await;
        assert!(status.contains_key("research_workflow_projection"));
        
        let projection_status = &status["research_workflow_projection"];
        assert_eq!(projection_status.name, "research_workflow_projection");
        assert!(projection_status.is_running);
        
        // Check health
        assert!(projection_manager.is_healthy().await);
    }

    #[tokio::test]
    async fn test_command_validation() {
        let command_factory = CommandFactory::new();
        let methodology = ResearchMethodology {
            name: "Test Method".to_string(),
            steps: vec![],
            ai_agents: vec![],
            estimated_duration_minutes: 10,
        };
        
        // Valid command
        let valid_command = command_factory.create_research_workflow(
            Uuid::new_v4(),
            "Valid Name".to_string(),
            "Valid Query".to_string(),
            methodology.clone(),
        );
        assert!(valid_command.validate().is_ok());
        
        // Invalid command - empty name
        let invalid_command = command_factory.create_research_workflow(
            Uuid::new_v4(),
            "".to_string(),
            "Valid Query".to_string(),
            methodology,
        );
        assert!(invalid_command.validate().is_err());
    }

    #[tokio::test]
    async fn test_query_validation() {
        let query_factory = QueryFactory::new();
        
        // Valid query
        let valid_query = query_factory.get_workflow_list(1, 50, None, None, None, None);
        assert!(valid_query.validate().is_ok());
        
        // Invalid query - page size too large
        let invalid_query = query_factory.get_workflow_list(1, 2000, None, None, None, None);
        assert!(invalid_query.validate().is_err());
        
        // Invalid search query - empty search term
        let invalid_search = query_factory.search_workflows(
            "".to_string(),
            1,
            10,
            std::collections::HashMap::new(),
        );
        assert!(invalid_search.validate().is_err());
    }

    #[tokio::test]
    async fn test_read_model_operations() {
        let mut store = MockReadModelStore::new();
        
        // Create test workflow read model
        let workflow = ResearchWorkflowReadModel {
            id: Uuid::new_v4(),
            name: "Test Workflow".to_string(),
            query: "Test Query".to_string(),
            methodology: None,
            status: WorkflowStatus::Created,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            updated_at: Utc::now(),
            results: None,
            error_message: None,
            tasks: Vec::new(),
            metrics: WorkflowMetrics {
                total_tasks: 0,
                completed_tasks: 0,
                failed_tasks: 0,
                progress_percentage: 0.0,
                estimated_completion_time: None,
                actual_duration_minutes: None,
            },
            tags: Vec::new(),
        };
        
        let workflow_id = workflow.id;
        
        // Update workflow
        let result = store.update_workflow(workflow).await;
        assert!(result.is_ok());
        
        // Get workflow
        let retrieved = store.get_workflow(workflow_id).await;
        assert!(retrieved.is_ok());
        
        let retrieved_workflow = retrieved.unwrap();
        assert!(retrieved_workflow.is_some());
        
        let workflow = retrieved_workflow.unwrap();
        assert_eq!(workflow.id, workflow_id);
        assert_eq!(workflow.name, "Test Workflow");
        assert_eq!(workflow.status, WorkflowStatus::Created);
    }

    #[tokio::test]
    async fn test_cqrs_metrics() {
        let cqrs_service = setup_cqrs_service().await;
        
        let metrics = cqrs_service.get_metrics().await;
        
        // Initial metrics should be zero
        assert_eq!(metrics.commands_executed, 0);
        assert_eq!(metrics.queries_executed, 0);
        assert_eq!(metrics.projections_processed, 0);
        assert_eq!(metrics.commands_failed, 0);
        assert_eq!(metrics.queries_failed, 0);
    }

    #[tokio::test]
    async fn test_cqrs_health_check() {
        let cqrs_service = setup_cqrs_service().await;
        
        let health = cqrs_service.health_check().await;
        
        assert!(health.overall_healthy);
        assert!(health.command_bus_healthy);
        assert!(health.query_bus_healthy);
        assert!(health.projections_healthy);
        assert!(health.read_models_healthy);
    }

    #[tokio::test]
    async fn test_projection_processing() {
        let read_model_store = Arc::new(RwLock::new(MockReadModelStore::new()));
        let cqrs_service = setup_cqrs_service().await;
        
        // Register projection
        let projection = ResearchWorkflowProjectionBuilder::new();
        cqrs_service.register_projection(projection).await;
        
        // Start projections
        let result = cqrs_service.start_projections().await;
        assert!(result.is_ok());
        
        // Check projection status
        let status = cqrs_service.get_projection_status().await;
        assert!(status.contains_key("research_workflow_projection"));
        
        // Stop projections
        let result = cqrs_service.stop_projections().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_error_handling() {
        let cqrs_service = setup_cqrs_service().await;
        
        // Try to execute command without registered handler
        let command_factory = CommandFactory::new();
        let methodology = ResearchMethodology {
            name: "Test".to_string(),
            steps: vec![],
            ai_agents: vec![],
            estimated_duration_minutes: 10,
        };
        
        let command = command_factory.create_research_workflow(
            Uuid::new_v4(),
            "Test".to_string(),
            "Test".to_string(),
            methodology,
        );
        
        let result = cqrs_service.execute_command(command).await;
        assert!(result.is_err());
        
        // Should be handler not found error
        let error = result.unwrap_err();
        assert!(matches!(error, CQRSError::HandlerNotFound(_)));
    }

    #[tokio::test]
    async fn test_query_caching() {
        use crate::cqrs::queries::{QueryBus, QueryCache};
        
        let mut cache = QueryCache::new(100);
        let data = serde_json::json!({"test": "value"});
        
        // Set and get
        cache.set("test_key".to_string(), data.clone(), 60);
        let retrieved = cache.get("test_key");
        assert_eq!(retrieved, Some(data));
        
        // Test cache stats
        let (size, max_size) = cache.stats();
        assert_eq!(size, 1);
        assert_eq!(max_size, 100);
        
        // Test cache clear
        cache.clear();
        let (size, _) = cache.stats();
        assert_eq!(size, 0);
    }

    #[tokio::test]
    async fn test_integration_workflow() {
        let event_store = setup_test_event_store().await;
        let read_model_store = Arc::new(RwLock::new(MockReadModelStore::new()));
        
        let cqrs_service = CQRSServiceBuilder::new()
            .with_read_model_store(Arc::clone(&read_model_store))
            .build()
            .expect("Failed to build CQRS service");
        
        // Register handlers
        let create_handler = CreateResearchWorkflowHandler::new(event_store);
        cqrs_service.register_command_handler::<CreateResearchWorkflowCommand, _>(create_handler).await;
        
        let get_handler = GetResearchWorkflowHandler::new(read_model_store);
        cqrs_service.register_query_handler::<GetResearchWorkflowQuery, _>(get_handler).await;
        
        // Execute command
        let command_factory = CommandFactory::new();
        let workflow_id = Uuid::new_v4();
        let methodology = ResearchMethodology {
            name: "Integration Test".to_string(),
            steps: vec!["Step 1".to_string()],
            ai_agents: vec!["agent1".to_string()],
            estimated_duration_minutes: 20,
        };
        
        let command = command_factory.create_research_workflow(
            workflow_id,
            "Integration Test Workflow".to_string(),
            "Integration test query".to_string(),
            methodology,
        );
        
        let command_result = cqrs_service.execute_command(command).await;
        assert!(command_result.is_ok());
        
        // Execute query
        let query_factory = QueryFactory::new();
        let query = query_factory.get_research_workflow(workflow_id, true);
        
        let query_result = cqrs_service.execute_query(query).await;
        assert!(query_result.is_ok());
        
        // In a real integration test, the projection would have updated the read model
        // For now, we just verify the query executes successfully
    }

    #[tokio::test]
    async fn test_concurrent_operations() {
        let cqrs_service = Arc::new(setup_cqrs_service().await);
        
        // Register handlers
        let read_model_store = Arc::new(RwLock::new(MockReadModelStore::new()));
        let get_handler = GetResearchWorkflowHandler::new(read_model_store);
        cqrs_service.register_query_handler::<GetResearchWorkflowQuery, _>(get_handler).await;
        
        // Execute multiple concurrent queries
        let mut handles = Vec::new();
        
        for _ in 0..10 {
            let service = Arc::clone(&cqrs_service);
            let handle = tokio::spawn(async move {
                let query_factory = QueryFactory::new();
                let query = query_factory.get_research_workflow(Uuid::new_v4(), false);
                service.execute_query(query).await
            });
            handles.push(handle);
        }
        
        // Wait for all queries to complete
        for handle in handles {
            let result = handle.await.expect("Task panicked");
            assert!(result.is_ok());
        }
    }
}
