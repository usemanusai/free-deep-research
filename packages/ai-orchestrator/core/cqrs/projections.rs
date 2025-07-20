// Projection System for CQRS Read Models
// Phase 4.2: CQRS Pattern Implementation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::error::{CQRSError, CQRSResult};
use super::read_models::{
    ReadModelStore, ResearchWorkflowReadModel, TaskReadModel,
    WorkflowStatus, TaskStatus, WorkflowMetrics,
};
use super::CQRSConfig;
use crate::event_store::{
    DomainEvent, ResearchWorkflowEvent, AIAgentEvent,
    EventStore, EventReplayService, ReplayHandler,
};

/// Projection checkpoint for tracking progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectionCheckpoint {
    pub projection_name: String,
    pub last_processed_event_id: Option<Uuid>,
    pub last_processed_sequence: u64,
    pub last_processed_timestamp: Option<DateTime<Utc>>,
    pub status: ProjectionStatus,
    pub error_count: u32,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectionStatus {
    Active,
    Paused,
    Error,
    Rebuilding,
    Stopped,
}

/// Projection builder trait for creating read models from events
#[async_trait]
pub trait ProjectionBuilder: Send + Sync {
    /// Get projection name
    fn projection_name(&self) -> &str;
    
    /// Get supported event types
    fn supported_event_types(&self) -> Vec<&'static str>;
    
    /// Handle an event and update read models
    async fn handle_event(
        &self,
        stream_id: Uuid,
        event: &dyn DomainEvent,
        read_model_store: Arc<RwLock<dyn ReadModelStore>>,
    ) -> CQRSResult<()>;
    
    /// Initialize projection (create tables, indexes, etc.)
    async fn initialize(&self, read_model_store: Arc<RwLock<dyn ReadModelStore>>) -> CQRSResult<()> {
        Ok(())
    }
    
    /// Reset projection (clear all read models)
    async fn reset(&self, read_model_store: Arc<RwLock<dyn ReadModelStore>>) -> CQRSResult<()>;
    
    /// Clone the projection builder
    fn clone_box(&self) -> Box<dyn ProjectionBuilder>;
}

/// Research workflow projection builder
pub struct ResearchWorkflowProjectionBuilder {
    name: String,
}

impl ResearchWorkflowProjectionBuilder {
    pub fn new() -> Self {
        Self {
            name: "research_workflow_projection".to_string(),
        }
    }
}

#[async_trait]
impl ProjectionBuilder for ResearchWorkflowProjectionBuilder {
    fn projection_name(&self) -> &str {
        &self.name
    }
    
    fn supported_event_types(&self) -> Vec<&'static str> {
        vec![
            "research.workflow.created",
            "research.workflow.started",
            "research.task.created",
            "research.task.completed",
            "research.workflow.completed",
            "research.workflow.failed",
            "research.workflow.updated",
        ]
    }
    
    async fn handle_event(
        &self,
        stream_id: Uuid,
        event: &dyn DomainEvent,
        read_model_store: Arc<RwLock<dyn ReadModelStore>>,
    ) -> CQRSResult<()> {
        // Deserialize event data
        let event_data = event.serialize()
            .map_err(|e| CQRSError::serialization_error(e.to_string()))?;
        
        match event.event_type() {
            "research.workflow.created" => {
                if let Ok(workflow_event) = serde_json::from_value::<ResearchWorkflowEvent>(event_data) {
                    if let ResearchWorkflowEvent::WorkflowCreated {
                        workflow_id, name, query, methodology, created_at, ..
                    } = workflow_event {
                        let read_model = ResearchWorkflowReadModel {
                            id: workflow_id,
                            name,
                            query,
                            methodology: Some(serde_json::to_value(methodology)?),
                            status: WorkflowStatus::Created,
                            created_at,
                            started_at: None,
                            completed_at: None,
                            updated_at: created_at,
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
                        
                        let store = read_model_store.write().await;
                        store.update_workflow(read_model).await?;
                    }
                }
            }
            
            "research.workflow.started" => {
                if let Ok(workflow_event) = serde_json::from_value::<ResearchWorkflowEvent>(event_data) {
                    if let ResearchWorkflowEvent::ExecutionStarted { workflow_id, started_at, .. } = workflow_event {
                        let store = read_model_store.read().await;
                        if let Some(mut workflow) = store.get_workflow(workflow_id).await? {
                            workflow.status = WorkflowStatus::Running;
                            workflow.started_at = Some(started_at);
                            workflow.updated_at = started_at;
                            
                            drop(store);
                            let store = read_model_store.write().await;
                            store.update_workflow(workflow).await?;
                        }
                    }
                }
            }
            
            "research.task.created" => {
                if let Ok(workflow_event) = serde_json::from_value::<ResearchWorkflowEvent>(event_data) {
                    if let ResearchWorkflowEvent::TaskCreated {
                        workflow_id, task_id, task_type, agent_type, created_at, ..
                    } = workflow_event {
                        let task_read_model = TaskReadModel {
                            id: task_id,
                            workflow_id,
                            task_type,
                            agent_type,
                            status: TaskStatus::Created,
                            created_at,
                            started_at: None,
                            completed_at: None,
                            results: None,
                            error_message: None,
                            duration_seconds: None,
                            retry_count: 0,
                        };
                        
                        let store = read_model_store.write().await;
                        store.update_task(task_read_model).await?;
                        
                        // Update workflow metrics
                        if let Some(mut workflow) = store.get_workflow(workflow_id).await? {
                            workflow.metrics.total_tasks += 1;
                            workflow.updated_at = created_at;
                            store.update_workflow(workflow).await?;
                        }
                    }
                }
            }
            
            "research.task.completed" => {
                if let Ok(workflow_event) = serde_json::from_value::<ResearchWorkflowEvent>(event_data) {
                    if let ResearchWorkflowEvent::TaskCompleted {
                        workflow_id, task_id, results, completed_at, ..
                    } = workflow_event {
                        let store = read_model_store.read().await;
                        let tasks = store.get_tasks_by_workflow(workflow_id, None).await?;
                        
                        if let Some(mut task) = tasks.into_iter().find(|t| t.id == task_id) {
                            task.status = TaskStatus::Completed;
                            task.completed_at = Some(completed_at);
                            task.results = Some(results);
                            
                            if let Some(started_at) = task.started_at {
                                let duration = completed_at.signed_duration_since(started_at);
                                task.duration_seconds = Some(duration.num_seconds() as u32);
                            }
                            
                            drop(store);
                            let store = read_model_store.write().await;
                            store.update_task(task).await?;
                            
                            // Update workflow metrics
                            if let Some(mut workflow) = store.get_workflow(workflow_id).await? {
                                workflow.metrics.completed_tasks += 1;
                                workflow.metrics.progress_percentage = 
                                    (workflow.metrics.completed_tasks as f64 / workflow.metrics.total_tasks as f64) * 100.0;
                                workflow.updated_at = completed_at;
                                store.update_workflow(workflow).await?;
                            }
                        }
                    }
                }
            }
            
            "research.workflow.completed" => {
                if let Ok(workflow_event) = serde_json::from_value::<ResearchWorkflowEvent>(event_data) {
                    if let ResearchWorkflowEvent::ExecutionCompleted {
                        workflow_id, results, completed_at, ..
                    } = workflow_event {
                        let store = read_model_store.read().await;
                        if let Some(mut workflow) = store.get_workflow(workflow_id).await? {
                            workflow.status = WorkflowStatus::Completed;
                            workflow.completed_at = Some(completed_at);
                            workflow.results = Some(serde_json::to_value(results)?);
                            workflow.updated_at = completed_at;
                            
                            // Calculate actual duration
                            if let Some(started_at) = workflow.started_at {
                                let duration = completed_at.signed_duration_since(started_at);
                                workflow.metrics.actual_duration_minutes = Some(duration.num_minutes() as u32);
                            }
                            
                            drop(store);
                            let store = read_model_store.write().await;
                            store.update_workflow(workflow).await?;
                        }
                    }
                }
            }
            
            "research.workflow.failed" => {
                if let Ok(workflow_event) = serde_json::from_value::<ResearchWorkflowEvent>(event_data) {
                    if let ResearchWorkflowEvent::ExecutionFailed {
                        workflow_id, error, failed_at, ..
                    } = workflow_event {
                        let store = read_model_store.read().await;
                        if let Some(mut workflow) = store.get_workflow(workflow_id).await? {
                            workflow.status = WorkflowStatus::Failed;
                            workflow.completed_at = Some(failed_at);
                            workflow.error_message = Some(error);
                            workflow.updated_at = failed_at;
                            
                            drop(store);
                            let store = read_model_store.write().await;
                            store.update_workflow(workflow).await?;
                        }
                    }
                }
            }
            
            _ => {
                // Ignore unsupported event types
            }
        }
        
        Ok(())
    }
    
    async fn reset(&self, read_model_store: Arc<RwLock<dyn ReadModelStore>>) -> CQRSResult<()> {
        // In a real implementation, you'd clear all workflow read models
        // For now, this is a placeholder
        Ok(())
    }
    
    fn clone_box(&self) -> Box<dyn ProjectionBuilder> {
        Box::new(ResearchWorkflowProjectionBuilder {
            name: self.name.clone(),
        })
    }
}

/// Projection manager for coordinating multiple projections
pub struct ProjectionManager {
    projections: HashMap<String, Box<dyn ProjectionBuilder>>,
    checkpoints: Arc<RwLock<HashMap<String, ProjectionCheckpoint>>>,
    config: CQRSConfig,
    is_running: Arc<RwLock<bool>>,
    metrics: Arc<RwLock<ProjectionMetrics>>,
}

#[derive(Debug, Clone, Default)]
pub struct ProjectionMetrics {
    pub events_processed: u64,
    pub events_failed: u64,
    pub read_models_updated: u64,
    pub projections_running: u32,
    pub average_processing_time_ms: f64,
}

impl ProjectionManager {
    pub fn new(config: CQRSConfig) -> Self {
        Self {
            projections: HashMap::new(),
            checkpoints: Arc::new(RwLock::new(HashMap::new())),
            config,
            is_running: Arc::new(RwLock::new(false)),
            metrics: Arc::new(RwLock::new(ProjectionMetrics::default())),
        }
    }
    
    /// Register a projection
    pub async fn register_projection(&mut self, projection: Box<dyn ProjectionBuilder>) {
        let name = projection.projection_name().to_string();
        
        // Initialize checkpoint
        let checkpoint = ProjectionCheckpoint {
            projection_name: name.clone(),
            last_processed_event_id: None,
            last_processed_sequence: 0,
            last_processed_timestamp: None,
            status: ProjectionStatus::Active,
            error_count: 0,
            last_error: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let mut checkpoints = self.checkpoints.write().await;
        checkpoints.insert(name.clone(), checkpoint);
        
        self.projections.insert(name, projection);
    }
    
    /// Start projection processing
    pub async fn start_processing(
        &mut self,
        read_model_store: Arc<RwLock<dyn ReadModelStore>>,
    ) -> CQRSResult<()> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            return Err(CQRSError::configuration_error("Projection manager already running"));
        }
        *is_running = true;
        
        // Initialize all projections
        for projection in self.projections.values() {
            projection.initialize(Arc::clone(&read_model_store)).await?;
        }
        
        // Start processing loop
        let projections = self.projections.clone();
        let checkpoints = Arc::clone(&self.checkpoints);
        let metrics = Arc::clone(&self.metrics);
        let is_running_flag = Arc::clone(&self.is_running);
        
        tokio::spawn(async move {
            while *is_running_flag.read().await {
                // Process events for each projection
                for (name, projection) in &projections {
                    if let Err(e) = Self::process_projection_events(
                        name,
                        projection.as_ref(),
                        Arc::clone(&read_model_store),
                        Arc::clone(&checkpoints),
                        Arc::clone(&metrics),
                    ).await {
                        eprintln!("Error processing projection {}: {:?}", name, e);
                        
                        // Update checkpoint with error
                        let mut checkpoints_guard = checkpoints.write().await;
                        if let Some(checkpoint) = checkpoints_guard.get_mut(name) {
                            checkpoint.status = ProjectionStatus::Error;
                            checkpoint.error_count += 1;
                            checkpoint.last_error = Some(e.to_string());
                            checkpoint.updated_at = Utc::now();
                        }
                    }
                }
                
                // Sleep before next iteration
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            }
        });
        
        Ok(())
    }
    
    /// Stop projection processing
    pub async fn stop_processing(&mut self) -> CQRSResult<()> {
        let mut is_running = self.is_running.write().await;
        *is_running = false;
        Ok(())
    }
    
    /// Get projection status
    pub async fn get_status(&self) -> HashMap<String, super::ProjectionStatus> {
        let checkpoints = self.checkpoints.read().await;
        let mut status_map = HashMap::new();
        
        for (name, checkpoint) in checkpoints.iter() {
            let status = super::ProjectionStatus {
                name: name.clone(),
                is_running: checkpoint.status == ProjectionStatus::Active,
                last_processed_event: checkpoint.last_processed_event_id,
                last_checkpoint: Some(checkpoint.clone()),
                events_processed: 0, // Would track this in real implementation
                events_failed: checkpoint.error_count as u64,
                last_error: checkpoint.last_error.clone(),
                processing_rate_per_second: 0.0, // Would calculate this
            };
            status_map.insert(name.clone(), status);
        }
        
        status_map
    }
    
    /// Check if projection manager is healthy
    pub async fn is_healthy(&self) -> bool {
        let checkpoints = self.checkpoints.read().await;
        checkpoints.values().all(|c| c.status != ProjectionStatus::Error)
    }
    
    /// Get projection metrics
    pub async fn get_metrics(&self) -> ProjectionMetrics {
        self.metrics.read().await.clone()
    }
    
    /// Process events for a specific projection
    async fn process_projection_events(
        projection_name: &str,
        projection: &dyn ProjectionBuilder,
        read_model_store: Arc<RwLock<dyn ReadModelStore>>,
        checkpoints: Arc<RwLock<HashMap<String, ProjectionCheckpoint>>>,
        metrics: Arc<RwLock<ProjectionMetrics>>,
    ) -> CQRSResult<()> {
        // In a real implementation, you'd:
        // 1. Get events from event store since last checkpoint
        // 2. Process each event through the projection
        // 3. Update checkpoint after successful processing
        // 4. Update metrics
        
        // For now, this is a placeholder
        Ok(())
    }
}

/// Projection replay handler for event store integration
pub struct ProjectionReplayHandler {
    projection: Box<dyn ProjectionBuilder>,
    read_model_store: Arc<RwLock<dyn ReadModelStore>>,
}

impl ProjectionReplayHandler {
    pub fn new(
        projection: Box<dyn ProjectionBuilder>,
        read_model_store: Arc<RwLock<dyn ReadModelStore>>,
    ) -> Self {
        Self {
            projection,
            read_model_store,
        }
    }
}

#[async_trait]
impl ReplayHandler for ProjectionReplayHandler {
    fn supported_event_types(&self) -> Vec<&'static str> {
        self.projection.supported_event_types()
    }
    
    async fn handle_event(&self, stream_id: Uuid, event: &dyn DomainEvent) -> crate::event_store::EventStoreResult<()> {
        self.projection
            .handle_event(stream_id, event, Arc::clone(&self.read_model_store))
            .await
            .map_err(|e| crate::event_store::EventStoreError::internal_error(e.to_string()))
    }
    
    fn clone_box(&self) -> Box<dyn ReplayHandler> {
        Box::new(ProjectionReplayHandler {
            projection: self.projection.clone_box(),
            read_model_store: Arc::clone(&self.read_model_store),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cqrs::read_models::MockReadModelStore;
    use crate::event_store::events::{ResearchWorkflowEvent, ResearchMethodology};

    #[tokio::test]
    async fn test_research_workflow_projection() {
        let projection = ResearchWorkflowProjectionBuilder::new();
        let read_model_store = Arc::new(RwLock::new(MockReadModelStore::new()));
        
        assert_eq!(projection.projection_name(), "research_workflow_projection");
        assert!(projection.supported_event_types().contains(&"research.workflow.created"));
        
        // Test initialization
        let result = projection.initialize(Arc::clone(&read_model_store)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_projection_manager() {
        let config = CQRSConfig::default();
        let mut manager = ProjectionManager::new(config);
        
        let projection = Box::new(ResearchWorkflowProjectionBuilder::new());
        manager.register_projection(projection).await;
        
        let status = manager.get_status().await;
        assert!(status.contains_key("research_workflow_projection"));
        
        assert!(manager.is_healthy().await);
    }

    #[tokio::test]
    async fn test_projection_checkpoint() {
        let checkpoint = ProjectionCheckpoint {
            projection_name: "test_projection".to_string(),
            last_processed_event_id: Some(Uuid::new_v4()),
            last_processed_sequence: 100,
            last_processed_timestamp: Some(Utc::now()),
            status: ProjectionStatus::Active,
            error_count: 0,
            last_error: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        assert_eq!(checkpoint.projection_name, "test_projection");
        assert_eq!(checkpoint.status, ProjectionStatus::Active);
        assert_eq!(checkpoint.error_count, 0);
    }
}
