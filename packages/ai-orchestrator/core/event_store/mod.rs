// Event Store Implementation for Free Deep Research System
// Phase 4.1: Event Sourcing Foundation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod error;
pub mod events;
pub mod serialization;
pub mod snapshots;
pub mod aggregates;
pub mod replay;

#[cfg(test)]
pub mod tests;

use error::EventStoreError;
use events::{DomainEvent, EventMetadata};
use serialization::EventSerializer;
use snapshots::SnapshotStore;

// Re-export key types for easier access
pub use error::{EventStoreError, EventStoreResult, ErrorCategory};
pub use events::{ResearchWorkflowEvent, AIAgentEvent, EventFactory};
pub use aggregates::{AggregateRoot, ResearchWorkflowAggregate, AggregateId};
pub use serialization::{JsonEventSerializer, EventTypeHandler};
pub use snapshots::{SnapshotStore as SnapshotStoreImpl, PostgresSnapshotStorage, SnapshotConfig};
pub use replay::{EventReplayService, ReplayConfig, ReplayHandler};

// Type aliases
pub type StreamId = Uuid;
pub type EventId = Uuid;
pub type AggregateId = Uuid;

/// Serialized event for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedEvent {
    pub metadata: EventMetadata,
    pub data: serde_json::Value,
}

/// Event store configuration
#[derive(Debug, Clone)]
pub struct EventStoreConfig {
    pub snapshot_frequency: u64,
    pub max_events_per_read: usize,
    pub enable_projections: bool,
    pub projection_batch_size: usize,
}

impl Default for EventStoreConfig {
    fn default() -> Self {
        Self {
            snapshot_frequency: 100,
            max_events_per_read: 1000,
            enable_projections: true,
            projection_batch_size: 50,
        }
    }
}

/// Main event store service
pub struct EventStore {
    pool: PgPool,
    config: EventStoreConfig,
    event_serializer: Arc<dyn EventSerializer>,
    snapshot_store: Arc<RwLock<SnapshotStore>>,
    event_bus: Arc<RwLock<EventBus>>,
}

impl EventStore {
    pub fn new(
        pool: PgPool,
        config: EventStoreConfig,
        event_serializer: Arc<dyn EventSerializer>,
    ) -> Self {
        let snapshot_store = Arc::new(RwLock::new(SnapshotStore::new(pool.clone())));
        let event_bus = Arc::new(RwLock::new(EventBus::new()));

        Self {
            pool,
            config,
            event_serializer,
            snapshot_store,
            event_bus,
        }
    }

    /// Append events to a stream with optimistic concurrency control
    pub async fn append_events(
        &self,
        stream_id: StreamId,
        events: Vec<Box<dyn DomainEvent>>,
        expected_version: Option<u64>,
    ) -> Result<u64, EventStoreError> {
        let mut tx = self.pool.begin().await?;

        // Validate expected version for optimistic concurrency
        if let Some(expected) = expected_version {
            let current_version = self.get_stream_version_tx(&mut tx, &stream_id).await?;
            if current_version != expected {
                return Err(EventStoreError::ConcurrencyConflict {
                    expected,
                    actual: current_version,
                });
            }
        }

        // Serialize events
        let mut serialized_events = Vec::new();
        let base_sequence = expected_version.unwrap_or(0);

        for (index, event) in events.iter().enumerate() {
            let event_metadata = EventMetadata {
                event_id: Uuid::new_v4(),
                stream_id,
                event_type: event.event_type().to_string(),
                event_version: 1,
                sequence_number: base_sequence + index as u64 + 1,
                timestamp: Utc::now(),
                correlation_id: event.correlation_id(),
                causation_id: event.causation_id(),
            };

            let serialized_event = SerializedEvent {
                metadata: event_metadata,
                data: self.event_serializer.serialize(event.as_ref())?,
            };

            serialized_events.push(serialized_event);
        }

        // Insert events using the stored procedure
        let events_json: Vec<serde_json::Value> = serialized_events
            .iter()
            .map(|e| {
                serde_json::json!({
                    "event_id": e.metadata.event_id,
                    "event_type": e.metadata.event_type,
                    "event_version": e.metadata.event_version,
                    "data": e.data,
                    "metadata": {
                        "correlation_id": e.metadata.correlation_id,
                        "causation_id": e.metadata.causation_id,
                        "timestamp": e.metadata.timestamp
                    },
                    "correlation_id": e.metadata.correlation_id,
                    "causation_id": e.metadata.causation_id
                })
            })
            .collect();

        let new_version: i64 = sqlx::query_scalar(
            "SELECT append_events($1, $2, $3)"
        )
        .bind(stream_id)
        .bind(&events_json)
        .bind(expected_version.map(|v| v as i64))
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        // Publish events to event bus
        for serialized_event in serialized_events {
            self.event_bus
                .write()
                .await
                .publish(serialized_event)
                .await?;
        }

        Ok(new_version as u64)
    }

    /// Read events from a stream
    pub async fn read_events(
        &self,
        stream_id: StreamId,
        from_version: Option<u64>,
        max_count: Option<usize>,
    ) -> Result<Vec<Box<dyn DomainEvent>>, EventStoreError> {
        let limit = max_count.unwrap_or(self.config.max_events_per_read);
        let from_seq = from_version.unwrap_or(0) as i64;

        let rows = sqlx::query(
            r#"
            SELECT event_id, event_type, event_version, sequence_number, 
                   event_data, metadata, timestamp, correlation_id, causation_id
            FROM event_store 
            WHERE stream_id = $1 AND sequence_number > $2
            ORDER BY sequence_number ASC
            LIMIT $3
            "#,
        )
        .bind(stream_id)
        .bind(from_seq)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        let mut events = Vec::new();
        for row in rows {
            let event_type: String = row.get("event_type");
            let event_data: serde_json::Value = row.get("event_data");
            let metadata = EventMetadata {
                event_id: row.get("event_id"),
                stream_id,
                event_type: event_type.clone(),
                event_version: row.get::<i32, _>("event_version") as u32,
                sequence_number: row.get::<i64, _>("sequence_number") as u64,
                timestamp: row.get("timestamp"),
                correlation_id: row.get("correlation_id"),
                causation_id: row.get("causation_id"),
            };

            let event = self
                .event_serializer
                .deserialize(&event_data, &event_type, metadata)?;
            events.push(event);
        }

        Ok(events)
    }

    /// Get current version of a stream
    pub async fn get_stream_version(&self, stream_id: &StreamId) -> Result<u64, EventStoreError> {
        let version: Option<i64> = sqlx::query_scalar("SELECT get_stream_version($1)")
            .bind(stream_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(version.unwrap_or(0) as u64)
    }

    /// Get stream version within a transaction
    async fn get_stream_version_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        stream_id: &StreamId,
    ) -> Result<u64, EventStoreError> {
        let version: Option<i64> = sqlx::query_scalar("SELECT get_stream_version($1)")
            .bind(stream_id)
            .fetch_optional(&mut **tx)
            .await?;

        Ok(version.unwrap_or(0) as u64)
    }

    /// Create a snapshot for a stream
    pub async fn create_snapshot(
        &self,
        stream_id: StreamId,
        aggregate_state: serde_json::Value,
        version: u64,
    ) -> Result<(), EventStoreError> {
        self.snapshot_store
            .write()
            .await
            .save_snapshot(stream_id, aggregate_state, version)
            .await?;

        Ok(())
    }

    /// Load the latest snapshot for a stream
    pub async fn load_snapshot(
        &self,
        stream_id: StreamId,
    ) -> Result<Option<(serde_json::Value, u64)>, EventStoreError> {
        self.snapshot_store
            .read()
            .await
            .load_latest_snapshot(stream_id)
            .await
    }

    /// Check if snapshot should be created
    pub fn should_create_snapshot(&self, version: u64) -> bool {
        version > 0 && version % self.config.snapshot_frequency == 0
    }

    /// Subscribe to events
    pub async fn subscribe<F>(&self, handler: F) -> Result<(), EventStoreError>
    where
        F: Fn(SerializedEvent) -> Result<(), EventStoreError> + Send + Sync + 'static,
    {
        self.event_bus.write().await.subscribe(Box::new(handler)).await
    }
}

/// Event bus for publishing events to subscribers
pub struct EventBus {
    subscribers: Vec<Box<dyn Fn(SerializedEvent) -> Result<(), EventStoreError> + Send + Sync>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }

    pub async fn publish(&mut self, event: SerializedEvent) -> Result<(), EventStoreError> {
        for subscriber in &self.subscribers {
            if let Err(e) = subscriber(event.clone()) {
                eprintln!("Error in event subscriber: {:?}", e);
                // Continue with other subscribers
            }
        }
        Ok(())
    }

    pub async fn subscribe(
        &mut self,
        handler: Box<dyn Fn(SerializedEvent) -> Result<(), EventStoreError> + Send + Sync>,
    ) -> Result<(), EventStoreError> {
        self.subscribers.push(handler);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[tokio::test]
    async fn test_append_and_read_events() {
        let pool = setup_test_db().await;
        let config = EventStoreConfig::default();
        let serializer = Arc::new(JsonEventSerializer::new());
        let event_store = EventStore::new(pool, config, serializer);

        let stream_id = Uuid::new_v4();
        let events = vec![
            Box::new(TestEvent::new("test_event_1")) as Box<dyn DomainEvent>,
            Box::new(TestEvent::new("test_event_2")) as Box<dyn DomainEvent>,
        ];

        // Append events
        let version = event_store
            .append_events(stream_id, events, None)
            .await
            .unwrap();

        assert_eq!(version, 2);

        // Read events
        let read_events = event_store
            .read_events(stream_id, None, None)
            .await
            .unwrap();

        assert_eq!(read_events.len(), 2);
    }

    #[tokio::test]
    async fn test_optimistic_concurrency_control() {
        let pool = setup_test_db().await;
        let config = EventStoreConfig::default();
        let serializer = Arc::new(JsonEventSerializer::new());
        let event_store = EventStore::new(pool, config, serializer);

        let stream_id = Uuid::new_v4();
        let events = vec![Box::new(TestEvent::new("test_event")) as Box<dyn DomainEvent>];

        // First append should succeed
        let version1 = event_store
            .append_events(stream_id, events.clone(), Some(0))
            .await
            .unwrap();

        assert_eq!(version1, 1);

        // Second append with wrong expected version should fail
        let result = event_store
            .append_events(stream_id, events, Some(0))
            .await;

        assert!(matches!(result, Err(EventStoreError::ConcurrencyConflict { .. })));
    }
}
