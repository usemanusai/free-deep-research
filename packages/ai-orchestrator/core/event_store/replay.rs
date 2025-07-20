// Event Replay Service for Event Sourcing
// Phase 4.1: Event Sourcing Foundation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::error::{EventStoreError, EventStoreResult};
use super::events::DomainEvent;
use super::aggregates::{AggregateRoot, AggregateId};
use super::EventStore;

/// Event replay configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayConfig {
    pub batch_size: usize,
    pub max_concurrent_streams: usize,
    pub checkpoint_frequency: usize,
    pub timeout_seconds: u64,
    pub include_snapshots: bool,
    pub validate_events: bool,
}

impl Default for ReplayConfig {
    fn default() -> Self {
        Self {
            batch_size: 100,
            max_concurrent_streams: 10,
            checkpoint_frequency: 1000,
            timeout_seconds: 300, // 5 minutes
            include_snapshots: true,
            validate_events: true,
        }
    }
}

/// Event replay service for rebuilding aggregates and projections
pub struct EventReplayService {
    event_store: Arc<EventStore>,
    config: ReplayConfig,
    replay_handlers: HashMap<String, Box<dyn ReplayHandler>>,
    progress_tracker: Arc<RwLock<ReplayProgress>>,
}

/// Replay progress tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayProgress {
    pub replay_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub total_streams: u64,
    pub processed_streams: u64,
    pub total_events: u64,
    pub processed_events: u64,
    pub failed_streams: u64,
    pub status: ReplayStatus,
    pub error_message: Option<String>,
    pub checkpoints: HashMap<Uuid, u64>, // stream_id -> last_processed_version
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReplayStatus {
    NotStarted,
    Running,
    Completed,
    Failed,
    Cancelled,
    Paused,
}

impl EventReplayService {
    pub fn new(event_store: Arc<EventStore>, config: ReplayConfig) -> Self {
        Self {
            event_store,
            config,
            replay_handlers: HashMap::new(),
            progress_tracker: Arc::new(RwLock::new(ReplayProgress {
                replay_id: Uuid::new_v4(),
                started_at: Utc::now(),
                updated_at: Utc::now(),
                total_streams: 0,
                processed_streams: 0,
                total_events: 0,
                processed_events: 0,
                failed_streams: 0,
                status: ReplayStatus::NotStarted,
                error_message: None,
                checkpoints: HashMap::new(),
            })),
        }
    }
    
    /// Register a replay handler for specific event types
    pub fn register_handler(&mut self, handler: Box<dyn ReplayHandler>) {
        for event_type in handler.supported_event_types() {
            self.replay_handlers.insert(event_type.to_string(), handler.clone_box());
        }
    }
    
    /// Start full event replay from the beginning
    pub async fn replay_all_events(&self) -> EventStoreResult<ReplayResult> {
        self.replay_events_from_timestamp(None, None).await
    }
    
    /// Replay events from a specific timestamp
    pub async fn replay_events_from_timestamp(
        &self,
        from_timestamp: Option<DateTime<Utc>>,
        to_timestamp: Option<DateTime<Utc>>,
    ) -> EventStoreResult<ReplayResult> {
        let replay_id = Uuid::new_v4();
        
        // Initialize progress
        {
            let mut progress = self.progress_tracker.write().await;
            progress.replay_id = replay_id;
            progress.started_at = Utc::now();
            progress.status = ReplayStatus::Running;
            progress.error_message = None;
            progress.checkpoints.clear();
        }
        
        let start_time = Utc::now();
        let mut total_events_processed = 0u64;
        let mut total_streams_processed = 0u64;
        let mut failed_streams = 0u64;
        
        // Get all streams to replay
        let stream_ids = self.get_all_stream_ids().await?;
        
        // Update total streams count
        {
            let mut progress = self.progress_tracker.write().await;
            progress.total_streams = stream_ids.len() as u64;
        }
        
        // Process streams in batches
        let mut stream_batches = stream_ids.chunks(self.config.max_concurrent_streams);
        
        for batch in stream_batches {
            let mut batch_tasks = Vec::new();
            
            for &stream_id in batch {
                let event_store = Arc::clone(&self.event_store);
                let handlers = self.clone_handlers();
                let config = self.config.clone();
                let progress_tracker = Arc::clone(&self.progress_tracker);
                
                let task = tokio::spawn(async move {
                    Self::replay_stream(
                        event_store,
                        stream_id,
                        handlers,
                        config,
                        progress_tracker,
                        from_timestamp,
                        to_timestamp,
                    ).await
                });
                
                batch_tasks.push(task);
            }
            
            // Wait for batch to complete
            for task in batch_tasks {
                match task.await {
                    Ok(Ok(stream_result)) => {
                        total_events_processed += stream_result.events_processed;
                        total_streams_processed += 1;
                    }
                    Ok(Err(_)) | Err(_) => {
                        failed_streams += 1;
                    }
                }
            }
            
            // Update progress
            {
                let mut progress = self.progress_tracker.write().await;
                progress.processed_streams = total_streams_processed;
                progress.processed_events = total_events_processed;
                progress.failed_streams = failed_streams;
                progress.updated_at = Utc::now();
            }
        }
        
        let end_time = Utc::now();
        let duration = end_time.signed_duration_since(start_time);
        
        // Update final status
        {
            let mut progress = self.progress_tracker.write().await;
            progress.status = if failed_streams == 0 {
                ReplayStatus::Completed
            } else {
                ReplayStatus::Failed
            };
            progress.updated_at = end_time;
        }
        
        Ok(ReplayResult {
            replay_id,
            started_at: start_time,
            completed_at: end_time,
            duration_seconds: duration.num_seconds() as u64,
            total_streams: stream_ids.len() as u64,
            processed_streams: total_streams_processed,
            failed_streams,
            total_events_processed,
            success: failed_streams == 0,
        })
    }
    
    /// Replay a specific stream
    async fn replay_stream(
        event_store: Arc<EventStore>,
        stream_id: Uuid,
        handlers: HashMap<String, Box<dyn ReplayHandler>>,
        config: ReplayConfig,
        progress_tracker: Arc<RwLock<ReplayProgress>>,
        from_timestamp: Option<DateTime<Utc>>,
        to_timestamp: Option<DateTime<Utc>>,
    ) -> EventStoreResult<StreamReplayResult> {
        let mut events_processed = 0u64;
        let mut from_version = 0u64;
        
        // Check for existing checkpoint
        {
            let progress = progress_tracker.read().await;
            if let Some(&checkpoint_version) = progress.checkpoints.get(&stream_id) {
                from_version = checkpoint_version;
            }
        }
        
        loop {
            // Read events in batches
            let events = event_store
                .read_events(stream_id, Some(from_version), Some(config.batch_size))
                .await?;
            
            if events.is_empty() {
                break;
            }
            
            // Process events
            for event in events {
                // Apply timestamp filters if specified
                if let Some(from_ts) = from_timestamp {
                    // Note: This would require event metadata to include timestamp
                    // For now, we'll skip timestamp filtering in this implementation
                }
                
                if let Some(to_ts) = to_timestamp {
                    // Similar timestamp filtering for end time
                }
                
                // Validate event if configured
                if config.validate_events {
                    event.validate().map_err(|e| EventStoreError::schema_validation(e))?;
                }
                
                // Find appropriate handler
                if let Some(handler) = handlers.get(event.event_type()) {
                    handler.handle_event(stream_id, event.as_ref()).await?;
                }
                
                events_processed += 1;
                from_version += 1;
                
                // Update checkpoint periodically
                if events_processed % config.checkpoint_frequency as u64 == 0 {
                    let mut progress = progress_tracker.write().await;
                    progress.checkpoints.insert(stream_id, from_version);
                    progress.updated_at = Utc::now();
                }
            }
            
            // If we got fewer events than batch size, we're done
            if events.len() < config.batch_size {
                break;
            }
        }
        
        // Final checkpoint update
        {
            let mut progress = progress_tracker.write().await;
            progress.checkpoints.insert(stream_id, from_version);
        }
        
        Ok(StreamReplayResult {
            stream_id,
            events_processed,
            success: true,
        })
    }
    
    /// Get replay progress
    pub async fn get_progress(&self) -> ReplayProgress {
        self.progress_tracker.read().await.clone()
    }
    
    /// Cancel ongoing replay
    pub async fn cancel_replay(&self) -> EventStoreResult<()> {
        let mut progress = self.progress_tracker.write().await;
        if progress.status == ReplayStatus::Running {
            progress.status = ReplayStatus::Cancelled;
            progress.updated_at = Utc::now();
        }
        Ok(())
    }
    
    /// Pause ongoing replay
    pub async fn pause_replay(&self) -> EventStoreResult<()> {
        let mut progress = self.progress_tracker.write().await;
        if progress.status == ReplayStatus::Running {
            progress.status = ReplayStatus::Paused;
            progress.updated_at = Utc::now();
        }
        Ok(())
    }
    
    /// Resume paused replay
    pub async fn resume_replay(&self) -> EventStoreResult<()> {
        let mut progress = self.progress_tracker.write().await;
        if progress.status == ReplayStatus::Paused {
            progress.status = ReplayStatus::Running;
            progress.updated_at = Utc::now();
        }
        Ok(())
    }
    
    /// Get all stream IDs (simplified implementation)
    async fn get_all_stream_ids(&self) -> EventStoreResult<Vec<Uuid>> {
        // In a real implementation, you'd query the database for all unique stream IDs
        // For now, return an empty vector as placeholder
        Ok(Vec::new())
    }
    
    /// Clone handlers for concurrent processing
    fn clone_handlers(&self) -> HashMap<String, Box<dyn ReplayHandler>> {
        self.replay_handlers
            .iter()
            .map(|(k, v)| (k.clone(), v.clone_box()))
            .collect()
    }
}

/// Replay handler trait for processing events during replay
#[async_trait]
pub trait ReplayHandler: Send + Sync {
    /// Get supported event types
    fn supported_event_types(&self) -> Vec<&'static str>;
    
    /// Handle an event during replay
    async fn handle_event(&self, stream_id: Uuid, event: &dyn DomainEvent) -> EventStoreResult<()>;
    
    /// Clone the handler
    fn clone_box(&self) -> Box<dyn ReplayHandler>;
    
    /// Initialize handler before replay starts
    async fn initialize(&self) -> EventStoreResult<()> {
        Ok(())
    }
    
    /// Finalize handler after replay completes
    async fn finalize(&self) -> EventStoreResult<()> {
        Ok(())
    }
}

/// Aggregate rebuilding handler
pub struct AggregateReplayHandler<T: AggregateRoot> {
    aggregate_type: std::marker::PhantomData<T>,
    rebuilt_aggregates: Arc<RwLock<HashMap<Uuid, T>>>,
}

impl<T: AggregateRoot> AggregateReplayHandler<T> {
    pub fn new() -> Self {
        Self {
            aggregate_type: std::marker::PhantomData,
            rebuilt_aggregates: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn get_rebuilt_aggregate(&self, id: Uuid) -> Option<T> {
        let aggregates = self.rebuilt_aggregates.read().await;
        aggregates.get(&id).cloned()
    }
    
    pub async fn get_all_rebuilt_aggregates(&self) -> HashMap<Uuid, T> {
        self.rebuilt_aggregates.read().await.clone()
    }
}

/// Replay results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayResult {
    pub replay_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub duration_seconds: u64,
    pub total_streams: u64,
    pub processed_streams: u64,
    pub failed_streams: u64,
    pub total_events_processed: u64,
    pub success: bool,
}

#[derive(Debug, Clone)]
struct StreamReplayResult {
    pub stream_id: Uuid,
    pub events_processed: u64,
    pub success: bool,
}

/// Projection rebuilding handler
pub struct ProjectionReplayHandler {
    projection_name: String,
    projection_builder: Box<dyn ProjectionBuilder>,
}

impl ProjectionReplayHandler {
    pub fn new(
        projection_name: String,
        projection_builder: Box<dyn ProjectionBuilder>,
    ) -> Self {
        Self {
            projection_name,
            projection_builder,
        }
    }
}

#[async_trait]
impl ReplayHandler for ProjectionReplayHandler {
    fn supported_event_types(&self) -> Vec<&'static str> {
        self.projection_builder.supported_event_types()
    }
    
    async fn handle_event(&self, stream_id: Uuid, event: &dyn DomainEvent) -> EventStoreResult<()> {
        self.projection_builder.handle_event(stream_id, event).await
    }
    
    fn clone_box(&self) -> Box<dyn ReplayHandler> {
        Box::new(ProjectionReplayHandler {
            projection_name: self.projection_name.clone(),
            projection_builder: self.projection_builder.clone_box(),
        })
    }
}

/// Projection builder trait
#[async_trait]
pub trait ProjectionBuilder: Send + Sync {
    fn supported_event_types(&self) -> Vec<&'static str>;
    async fn handle_event(&self, stream_id: Uuid, event: &dyn DomainEvent) -> EventStoreResult<()>;
    fn clone_box(&self) -> Box<dyn ProjectionBuilder>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_store::events::ResearchWorkflowEvent;
    use std::sync::atomic::{AtomicU64, Ordering};

    // Mock replay handler for testing
    struct MockReplayHandler {
        events_handled: Arc<AtomicU64>,
    }

    impl MockReplayHandler {
        fn new() -> Self {
            Self {
                events_handled: Arc::new(AtomicU64::new(0)),
            }
        }
        
        fn get_events_handled(&self) -> u64 {
            self.events_handled.load(Ordering::Relaxed)
        }
    }

    #[async_trait]
    impl ReplayHandler for MockReplayHandler {
        fn supported_event_types(&self) -> Vec<&'static str> {
            vec!["research.workflow.created", "research.workflow.started"]
        }
        
        async fn handle_event(&self, _stream_id: Uuid, _event: &dyn DomainEvent) -> EventStoreResult<()> {
            self.events_handled.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
        
        fn clone_box(&self) -> Box<dyn ReplayHandler> {
            Box::new(MockReplayHandler {
                events_handled: Arc::clone(&self.events_handled),
            })
        }
    }

    #[tokio::test]
    async fn test_replay_service_creation() {
        let event_store = Arc::new(EventStore::new(
            // Mock pool - in real tests you'd use a test database
            sqlx::PgPool::connect("postgresql://test").await.unwrap_or_else(|_| {
                // Return a mock pool for testing
                panic!("Test database not available")
            }),
            super::super::EventStoreConfig::default(),
            Arc::new(super::super::serialization::JsonEventSerializer::new()),
        ));
        
        let config = ReplayConfig::default();
        let mut replay_service = EventReplayService::new(event_store, config);
        
        let handler = Box::new(MockReplayHandler::new());
        replay_service.register_handler(handler);
        
        // Test that handler was registered
        assert!(replay_service.replay_handlers.contains_key("research.workflow.created"));
    }

    #[tokio::test]
    async fn test_replay_progress_tracking() {
        let event_store = Arc::new(EventStore::new(
            // Mock setup
            sqlx::PgPool::connect("postgresql://test").await.unwrap_or_else(|_| {
                panic!("Test database not available")
            }),
            super::super::EventStoreConfig::default(),
            Arc::new(super::super::serialization::JsonEventSerializer::new()),
        ));
        
        let replay_service = EventReplayService::new(event_store, ReplayConfig::default());
        
        let initial_progress = replay_service.get_progress().await;
        assert_eq!(initial_progress.status, ReplayStatus::NotStarted);
        assert_eq!(initial_progress.processed_events, 0);
        
        // Test pause/resume
        replay_service.pause_replay().await.unwrap();
        let paused_progress = replay_service.get_progress().await;
        assert_eq!(paused_progress.status, ReplayStatus::NotStarted); // Still not started
        
        replay_service.resume_replay().await.unwrap();
        let resumed_progress = replay_service.get_progress().await;
        assert_eq!(resumed_progress.status, ReplayStatus::NotStarted); // Still not started
    }
}
