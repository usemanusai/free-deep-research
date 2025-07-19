use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{info, debug, error};
use uuid::Uuid;
use chrono::Utc;
use serde_json;

use crate::error::{AppResult, ResearchError};
use crate::services::{Service, ApiManagerService, DataPersistenceService, MonitoringService};
use crate::models::research_workflow::{
    ResearchWorkflow, WorkflowStatus, ResearchMethodology, WorkflowParameters,
    CreateWorkflowRequest, ResearchResult, ResearchStep, StepStatus
};

use self::queue_manager::{
    QueueManager, QueuedWorkflow, WorkflowPriority, QueueStats, ConcurrencyConfig,
    WorkflowProgress, QueueProgress, StepProgress, ProgressUpdate, ProgressUpdateType,
    QueueState, QueueManagementResult, QueueManagementStatus, BulkOperationRequest, BulkOperationType,
    ResourceLimits, ResourceUsage, ResourceAllocation, ResourceMetrics, ResourceStatus,
    ResourceRecommendation, RecommendationType, RecommendationPriority, ImplementationEffort
};

pub mod workflow_orchestrator;
pub mod queue_manager;
pub mod result_processor;
pub mod workflow_engine;
pub mod methodology_don_lim;
pub mod methodology_nick_scamara;
pub mod methodology_hybrid;

// Re-export queue types for external use
pub use queue_manager::{
    QueuedWorkflow, WorkflowPriority, QueueStats, ConcurrencyConfig,
    WorkflowProgress, QueueProgress, StepProgress, ProgressUpdate, ProgressUpdateType,
    QueueState, QueueManagementResult, QueueManagementStatus, BulkOperationRequest, BulkOperationType,
    ResourceLimits, ResourceUsage, ResourceAllocation, ResourceMetrics, ResourceStatus,
    ResourceRecommendation, RecommendationType, RecommendationPriority, ImplementationEffort
};

/// Workflow statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkflowStatistics {
    pub total_workflows: usize,
    pub active_workflows: usize,
    pub completed_workflows: usize,
    pub failed_workflows: usize,
    pub cancelled_workflows: usize,
    pub average_duration_minutes: f64,
    pub success_rate: f64,
}

impl Default for WorkflowStatistics {
    fn default() -> Self {
        Self {
            total_workflows: 0,
            active_workflows: 0,
            completed_workflows: 0,
            failed_workflows: 0,
            cancelled_workflows: 0,
            average_duration_minutes: 0.0,
            success_rate: 100.0,
        }
    }
}

/// Research Engine Service that orchestrates research workflows
pub struct ResearchEngineService {
    api_manager: Arc<RwLock<ApiManagerService>>,
    data_persistence: Arc<RwLock<DataPersistenceService>>,
    monitoring: Arc<RwLock<MonitoringService>>,
    active_workflows: Arc<RwLock<HashMap<Uuid, ResearchWorkflow>>>,
    methodologies: Arc<RwLock<HashMap<String, ResearchMethodology>>>,
    workflow_engine: Arc<workflow_engine::WorkflowEngine>,
    queue_manager: Arc<QueueManager>,
}

impl ResearchEngineService {
    /// Create a new research engine service
    pub async fn new(
        api_manager: Arc<RwLock<ApiManagerService>>,
        data_persistence: Arc<RwLock<DataPersistenceService>>,
        monitoring: Arc<RwLock<MonitoringService>>,
    ) -> AppResult<Self> {
        info!("Initializing research engine service...");

        // Create workflow engine
        let workflow_engine = Arc::new(workflow_engine::WorkflowEngine::new(
            data_persistence.clone(),
            api_manager.clone(),
        ).await?);

        // Create queue manager with default max concurrent workflows
        let queue_manager = Arc::new(QueueManager::new(5).await?);

        let service = Self {
            api_manager,
            data_persistence,
            monitoring,
            active_workflows: Arc::new(RwLock::new(HashMap::new())),
            methodologies: Arc::new(RwLock::new(HashMap::new())),
            workflow_engine,
            queue_manager,
        };

        // Initialize default methodologies
        service.initialize_default_methodologies().await?;

        info!("Research engine service initialized successfully");
        Ok(service)
    }

    /// Initialize default research methodologies
    async fn initialize_default_methodologies(&self) -> AppResult<()> {
        debug!("Initializing default research methodologies");

        let mut methodologies = self.methodologies.write().await;

        // Comprehensive methodology
        methodologies.insert("comprehensive".to_string(), ResearchMethodology {
            id: Uuid::new_v4(),
            name: "Comprehensive Research".to_string(),
            description: "Deep, multi-source research with AI analysis".to_string(),
            steps: vec![
                ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "web_search".to_string(),
                    provider: "serpapi".to_string(),
                    parameters: serde_json::json!({
                        "num_results": 20,
                        "time_range": "last_year"
                    }),
                    status: StepStatus::Pending,
                    result: None,
                    error: None,
                    started_at: None,
                    completed_at: None,
                },
                ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "content_extraction".to_string(),
                    provider: "firecrawl".to_string(),
                    parameters: serde_json::json!({
                        "extract_depth": "advanced",
                        "include_images": false
                    }),
                    status: StepStatus::Pending,
                    result: None,
                    error: None,
                    started_at: None,
                    completed_at: None,
                },
                ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "ai_analysis".to_string(),
                    provider: "openrouter".to_string(),
                    parameters: serde_json::json!({
                        "model": "anthropic/claude-3-sonnet",
                        "temperature": 0.3,
                        "max_tokens": 4000
                    }),
                    status: StepStatus::Pending,
                    result: None,
                    error: None,
                    started_at: None,
                    completed_at: None,
                },
            ],
            estimated_duration_minutes: 15,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        // Quick methodology
        methodologies.insert("quick".to_string(), ResearchMethodology {
            id: Uuid::new_v4(),
            name: "Quick Research".to_string(),
            description: "Fast research with basic AI analysis".to_string(),
            steps: vec![
                ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "web_search".to_string(),
                    provider: "tavily".to_string(),
                    parameters: serde_json::json!({
                        "num_results": 10,
                        "search_depth": "basic"
                    }),
                    status: StepStatus::Pending,
                    result: None,
                    error: None,
                    started_at: None,
                    completed_at: None,
                },
                ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "ai_summary".to_string(),
                    provider: "openrouter".to_string(),
                    parameters: serde_json::json!({
                        "model": "anthropic/claude-3-haiku",
                        "temperature": 0.2,
                        "max_tokens": 2000
                    }),
                    status: StepStatus::Pending,
                    result: None,
                    error: None,
                    started_at: None,
                    completed_at: None,
                },
            ],
            estimated_duration_minutes: 5,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        // Academic methodology
        methodologies.insert("academic".to_string(), ResearchMethodology {
            id: Uuid::new_v4(),
            name: "Academic Research".to_string(),
            description: "Scholarly research focused on academic sources".to_string(),
            steps: vec![
                ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "academic_search".to_string(),
                    provider: "exa".to_string(),
                    parameters: serde_json::json!({
                        "search_type": "academic",
                        "num_results": 15,
                        "include_domains": ["arxiv.org", "scholar.google.com", "pubmed.ncbi.nlm.nih.gov"]
                    }),
                    status: StepStatus::Pending,
                    result: None,
                    error: None,
                    started_at: None,
                    completed_at: None,
                },
                ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "content_extraction".to_string(),
                    provider: "jina".to_string(),
                    parameters: serde_json::json!({
                        "extract_depth": "advanced",
                        "focus": "academic_content"
                    }),
                    status: StepStatus::Pending,
                    result: None,
                    error: None,
                    started_at: None,
                    completed_at: None,
                },
                ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "academic_analysis".to_string(),
                    provider: "openrouter".to_string(),
                    parameters: serde_json::json!({
                        "model": "anthropic/claude-3-sonnet",
                        "temperature": 0.1,
                        "max_tokens": 6000,
                        "system_prompt": "You are an academic researcher. Analyze the provided sources with scholarly rigor."
                    }),
                    status: StepStatus::Pending,
                    result: None,
                    error: None,
                    started_at: None,
                    completed_at: None,
                },
            ],
            estimated_duration_minutes: 20,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        debug!("Initialized {} default methodologies", methodologies.len());
        Ok(())
    }

    /// Create a new research workflow from request
    pub async fn create_workflow_from_request(&self, request: CreateWorkflowRequest) -> AppResult<ResearchWorkflow> {
        info!("Creating new research workflow: {}", request.name);

        // Validate request
        if request.name.trim().is_empty() {
            return Err(ResearchError::invalid_request("Workflow name cannot be empty".to_string()).into());
        }

        if request.query.trim().is_empty() {
            return Err(ResearchError::invalid_request("Research query cannot be empty".to_string()).into());
        }

        // Get methodology
        let methodologies = self.methodologies.read().await;
        let methodology_name = request.template_id
            .and_then(|id| methodologies.values().find(|m| m.id == id).map(|m| m.name.clone()))
            .unwrap_or_else(|| "comprehensive".to_string());

        let methodology = methodologies.get(&methodology_name)
            .ok_or_else(|| ResearchError::methodology_not_found(methodology_name.clone()))?
            .clone();
        drop(methodologies);

        // Create workflow
        let workflow = ResearchWorkflow {
            id: Uuid::new_v4(),
            name: request.name,
            query: request.query,
            status: WorkflowStatus::Created,
            methodology: methodology_name,
            parameters: request.parameters.unwrap_or_default(),
            steps: methodology.steps,
            results: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: None,
            completed_at: None,
        };

        // Store in active workflows
        let mut active_workflows = self.active_workflows.write().await;
        active_workflows.insert(workflow.id, workflow.clone());
        drop(active_workflows);

        // TODO: Store in database

        info!("Research workflow created: {} ({})", workflow.name, workflow.id);
        Ok(workflow)
    }

    /// Create a new research workflow (legacy method for compatibility)
    pub async fn create_workflow(
        &self,
        name: String,
        query: String,
        methodology: ResearchMethodology,
        created_by: String,
    ) -> AppResult<ResearchWorkflow> {
        let request = CreateWorkflowRequest {
            name,
            query,
            template_id: None,
            parameters: Some(WorkflowParameters {
                methodology: Some(methodology),
                created_by: Some(created_by),
                ..Default::default()
            }),
        };
        self.create_workflow_from_request(request).await
    }

    /// Create workflow from request (called by Tauri command)
    pub async fn create_workflow(&self, request: CreateWorkflowRequest) -> AppResult<ResearchWorkflow> {
        self.create_workflow_from_request(request).await
    }

    /// Start workflow execution (called by Tauri command)
    pub async fn start_workflow_execution(&self, workflow_id: Uuid) -> AppResult<()> {
        info!("Starting workflow execution: {}", workflow_id);

        // Use the workflow engine to start execution
        self.workflow_engine.start_workflow(workflow_id).await
    }

    /// Get workflow by ID (called by Tauri command)
    pub async fn get_workflow(&self, workflow_id: Uuid) -> AppResult<Option<ResearchWorkflow>> {
        debug!("Getting workflow: {}", workflow_id);

        // First check active workflows
        let active_workflows = self.active_workflows.read().await;
        if let Some(workflow) = active_workflows.get(&workflow_id) {
            return Ok(Some(workflow.clone()));
        }
        drop(active_workflows);

        // If not in active workflows, check database
        let data_persistence = self.data_persistence.read().await;
        let workflow = data_persistence.get_research_workflow(workflow_id).await?;
        drop(data_persistence);

        Ok(workflow)
    }

    /// Cancel workflow (called by Tauri command)
    pub async fn cancel_workflow(&self, workflow_id: Uuid) -> AppResult<()> {
        info!("Cancelling workflow: {}", workflow_id);

        // Use the workflow engine to cancel execution
        self.workflow_engine.cancel_workflow(workflow_id).await
    }

    /// Get workflow results (called by Tauri command)
    pub async fn get_workflow_results(&self, workflow_id: Uuid) -> AppResult<Option<crate::models::ResearchResults>> {
        debug!("Getting workflow results: {}", workflow_id);

        // Get workflow first
        let workflow = self.get_workflow(workflow_id).await?;

        match workflow {
            Some(wf) => {
                // Check if workflow has results
                if let Some(results) = wf.results {
                    Ok(Some(results))
                } else {
                    // No results yet
                    Ok(None)
                }
            }
            None => {
                // Workflow not found
                Ok(None)
            }
        }
    }

    /// Enqueue a workflow for execution with priority
    pub async fn enqueue_workflow(
        &self,
        workflow_id: Uuid,
        priority: WorkflowPriority,
        estimated_duration_minutes: Option<u32>,
    ) -> AppResult<()> {
        info!("Enqueuing workflow: {} with priority: {:?}", workflow_id, priority);

        // Get workflow from active workflows
        let active_workflows = self.active_workflows.read().await;
        let workflow = active_workflows.get(&workflow_id)
            .ok_or_else(|| ResearchError::workflow_not_found(workflow_id.to_string()))?
            .clone();
        drop(active_workflows);

        // Enqueue the workflow
        self.queue_manager.enqueue_workflow(workflow, priority, estimated_duration_minutes).await?;

        info!("Workflow enqueued successfully: {}", workflow_id);
        Ok(())
    }

    /// Get queue statistics
    pub async fn get_queue_statistics(&self) -> AppResult<QueueStats> {
        self.queue_manager.get_queue_stats().await
    }

    /// Get active workflows from queue
    pub async fn get_active_queue_workflows(&self) -> AppResult<Vec<QueuedWorkflow>> {
        self.queue_manager.get_active_workflows().await
    }

    /// Get queued workflows
    pub async fn get_queued_workflows(&self) -> AppResult<Vec<QueuedWorkflow>> {
        self.queue_manager.get_queued_workflows().await
    }

    /// Get workflow history from queue
    pub async fn get_workflow_history(&self, limit: Option<usize>) -> AppResult<Vec<QueuedWorkflow>> {
        self.queue_manager.get_workflow_history(limit).await
    }

    /// Cancel a workflow in the queue
    pub async fn cancel_queued_workflow(&self, workflow_id: Uuid) -> AppResult<bool> {
        info!("Cancelling queued workflow: {}", workflow_id);
        self.queue_manager.cancel_workflow(workflow_id).await
    }

    /// Update queue concurrency configuration
    pub async fn update_queue_concurrency(&self, max_concurrent: usize) -> AppResult<()> {
        info!("Updating queue concurrency to: {}", max_concurrent);
        self.queue_manager.update_max_concurrent(max_concurrent).await
    }

    /// Get queue concurrency configuration
    pub async fn get_queue_concurrency_config(&self) -> AppResult<ConcurrencyConfig> {
        self.queue_manager.get_concurrency_config().await
    }

    /// Start queue processing
    pub async fn start_queue_processing(&self) -> AppResult<()> {
        info!("Starting queue processing");
        self.queue_manager.start_processing().await
    }

    /// Stop queue processing
    pub async fn stop_queue_processing(&self) -> AppResult<()> {
        info!("Stopping queue processing");
        self.queue_manager.stop_processing().await
    }

    /// Get detailed workflow progress
    pub async fn get_workflow_progress_detailed(&self, workflow_id: Uuid) -> AppResult<Option<WorkflowProgress>> {
        self.queue_manager.get_workflow_progress(workflow_id).await
    }

    /// Get queue-wide progress overview
    pub async fn get_queue_progress_overview(&self) -> AppResult<QueueProgress> {
        self.queue_manager.get_queue_progress().await
    }

    /// Get progress history for analytics
    pub async fn get_progress_history(&self, hours: Option<u32>) -> AppResult<Vec<WorkflowProgress>> {
        self.queue_manager.get_progress_history(hours).await
    }

    /// Pause queue gracefully
    pub async fn pause_queue_gracefully(&self, reason: String) -> AppResult<QueueManagementResult> {
        info!("Pausing queue gracefully: {}", reason);
        self.queue_manager.pause_queue_gracefully(reason).await
    }

    /// Resume queue processing
    pub async fn resume_queue(&self, reason: String) -> AppResult<QueueManagementResult> {
        info!("Resuming queue: {}", reason);
        self.queue_manager.resume_queue(reason).await
    }

    /// Emergency stop queue
    pub async fn emergency_stop_queue(&self, reason: String) -> AppResult<QueueManagementResult> {
        warn!("Emergency stopping queue: {}", reason);
        self.queue_manager.emergency_stop(reason).await
    }

    /// Drain queue
    pub async fn drain_queue(&self, reason: String) -> AppResult<QueueManagementResult> {
        info!("Draining queue: {}", reason);
        self.queue_manager.drain_queue(reason).await
    }

    /// Cancel multiple workflows
    pub async fn cancel_multiple_workflows(&self, workflow_ids: Vec<Uuid>, reason: String) -> AppResult<QueueManagementResult> {
        info!("Cancelling {} workflows: {}", workflow_ids.len(), reason);
        self.queue_manager.cancel_multiple_workflows(workflow_ids, reason).await
    }

    /// Clear entire queue
    pub async fn clear_queue(&self, reason: String) -> AppResult<QueueManagementResult> {
        warn!("Clearing queue: {}", reason);
        self.queue_manager.clear_queue(reason).await
    }

    /// Get queue management status
    pub async fn get_queue_management_status(&self) -> AppResult<QueueManagementStatus> {
        self.queue_manager.get_queue_management_status().await
    }

    /// Get current resource status
    pub async fn get_resource_status(&self) -> AppResult<ResourceStatus> {
        self.queue_manager.get_resource_status().await
    }

    /// Update resource limits
    pub async fn update_resource_limits(&self, new_limits: ResourceLimits) -> AppResult<()> {
        info!("Updating resource limits");
        self.queue_manager.update_resource_limits(new_limits).await
    }

    /// Get resource metrics for analytics
    pub async fn get_resource_metrics(&self, hours: Option<u32>) -> AppResult<ResourceMetrics> {
        self.queue_manager.get_resource_metrics(hours).await
    }

    /// Allocate resources for a workflow
    pub async fn allocate_workflow_resources(&self, workflow_id: Uuid, requirements: ResourceLimits) -> AppResult<ResourceAllocation> {
        info!("Allocating resources for workflow: {}", workflow_id);
        self.queue_manager.allocate_resources(workflow_id, requirements).await
    }

    /// Deallocate resources when workflow completes
    pub async fn deallocate_workflow_resources(&self, workflow_id: Uuid) -> AppResult<()> {
        info!("Deallocating resources for workflow: {}", workflow_id);
        self.queue_manager.deallocate_resources(workflow_id).await
    }

    /// Check if resources are available for a new workflow
    pub async fn can_allocate_workflow_resources(&self, requirements: &ResourceLimits) -> AppResult<bool> {
        self.queue_manager.can_allocate_resources(requirements).await
    }

    /// Record current resource usage for monitoring
    pub async fn record_resource_usage(&self) -> AppResult<()> {
        self.queue_manager.record_resource_usage().await
    }

    /// Get a workflow by ID
    pub async fn get_workflow(&self, workflow_id: Uuid) -> AppResult<Option<ResearchWorkflow>> {
        let active_workflows = self.active_workflows.read().await;
        Ok(active_workflows.get(&workflow_id).cloned())
    }

    /// Get all workflows
    pub async fn get_all_workflows(&self) -> AppResult<Vec<ResearchWorkflow>> {
        let active_workflows = self.active_workflows.read().await;
        Ok(active_workflows.values().cloned().collect())
    }

    /// Get workflows by status
    pub async fn get_workflows_by_status(&self, status: WorkflowStatus) -> AppResult<Vec<ResearchWorkflow>> {
        let active_workflows = self.active_workflows.read().await;
        Ok(active_workflows.values()
            .filter(|w| w.status == status)
            .cloned()
            .collect())
    }

    /// Delete a workflow
    pub async fn delete_workflow(&self, workflow_id: Uuid) -> AppResult<()> {
        let mut active_workflows = self.active_workflows.write().await;
        active_workflows.remove(&workflow_id);
        Ok(())
    }

    /// Get workflow status
    pub async fn get_workflow_status(&self, workflow_id: Uuid) -> AppResult<Option<WorkflowStatus>> {
        let active_workflows = self.active_workflows.read().await;
        Ok(active_workflows.get(&workflow_id).map(|w| w.status))
    }

    /// Get workflow progress
    pub async fn get_workflow_progress(&self, workflow_id: Uuid) -> AppResult<Option<f64>> {
        let active_workflows = self.active_workflows.read().await;
        if let Some(workflow) = active_workflows.get(&workflow_id) {
            let total_steps = workflow.steps.len() as f64;
            if total_steps == 0.0 {
                return Ok(Some(0.0));
            }
            let completed_steps = workflow.steps.iter()
                .filter(|s| s.status == StepStatus::Completed)
                .count() as f64;
            Ok(Some(completed_steps / total_steps * 100.0))
        } else {
            Ok(None)
        }
    }

    /// Get workflow results
    pub async fn get_workflow_results(&self, workflow_id: Uuid) -> AppResult<Option<crate::models::research_workflow::ResearchResults>> {
        let active_workflows = self.active_workflows.read().await;
        Ok(active_workflows.get(&workflow_id).and_then(|w| w.results.clone()))
    }

    /// Get workflow statistics
    pub async fn get_workflow_statistics(&self) -> AppResult<WorkflowStatistics> {
        let active_workflows = self.active_workflows.read().await;
        let workflows: Vec<&ResearchWorkflow> = active_workflows.values().collect();

        let total_workflows = workflows.len();
        let active_workflows_count = workflows.iter().filter(|w| w.status == WorkflowStatus::Running).count();
        let completed_workflows = workflows.iter().filter(|w| w.status == WorkflowStatus::Completed).count();
        let failed_workflows = workflows.iter().filter(|w| w.status == WorkflowStatus::Failed).count();
        let cancelled_workflows = workflows.iter().filter(|w| w.status == WorkflowStatus::Cancelled).count();

        let average_duration_minutes = if completed_workflows > 0 {
            let total_duration: i64 = workflows.iter()
                .filter(|w| w.status == WorkflowStatus::Completed)
                .filter_map(|w| {
                    if let (Some(started), Some(completed)) = (w.started_at, w.completed_at) {
                        Some((completed - started).num_minutes())
                    } else {
                        None
                    }
                })
                .sum();
            total_duration as f64 / completed_workflows as f64
        } else {
            0.0
        };

        let success_rate = if total_workflows > 0 {
            completed_workflows as f64 / total_workflows as f64 * 100.0
        } else {
            0.0
        };

        Ok(WorkflowStatistics {
            total_workflows,
            active_workflows: active_workflows_count,
            completed_workflows,
            failed_workflows,
            cancelled_workflows,
            average_duration_minutes,
            success_rate,
        })
    }

    /// Start a workflow using the workflow engine
    pub async fn start_workflow_execution(&self, workflow_id: Uuid) -> AppResult<()> {
        info!("Starting workflow execution via workflow engine: {}", workflow_id);
        self.workflow_engine.start_workflow(workflow_id).await
    }

    /// Pause a workflow
    pub async fn pause_workflow_execution(&self, workflow_id: Uuid) -> AppResult<()> {
        info!("Pausing workflow execution: {}", workflow_id);
        self.workflow_engine.pause_workflow(workflow_id).await
    }

    /// Resume a workflow
    pub async fn resume_workflow_execution(&self, workflow_id: Uuid) -> AppResult<()> {
        info!("Resuming workflow execution: {}", workflow_id);
        self.workflow_engine.resume_workflow(workflow_id).await
    }

    /// Cancel a workflow
    pub async fn cancel_workflow_execution(&self, workflow_id: Uuid) -> AppResult<()> {
        info!("Cancelling workflow execution: {}", workflow_id);
        self.workflow_engine.cancel_workflow(workflow_id).await
    }

    /// Start background monitoring
    pub async fn start_background_monitoring(&self) -> AppResult<()> {
        info!("Starting research engine background monitoring...");

        // Start workflow engine monitoring
        self.workflow_engine.start_background_monitoring().await?;

        // Start queue processing
        self.start_queue_processing().await?;

        // Start background queue processor
        self.start_queue_processor().await?;

        info!("Research engine background monitoring started successfully");
        Ok(())
    }

    /// Start the background queue processor
    async fn start_queue_processor(&self) -> AppResult<()> {
        info!("Starting background queue processor...");

        let queue_manager = self.queue_manager.clone();
        let workflow_engine = self.workflow_engine.clone();

        tokio::spawn(async move {
            loop {
                // Check if processing is enabled
                if !queue_manager.is_processing().await {
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    continue;
                }

                // Try to dequeue and start a workflow
                match queue_manager.dequeue_workflow().await {
                    Ok(Some(queued_workflow)) => {
                        info!("Processing queued workflow: {}", queued_workflow.workflow.name);

                        // Start workflow execution
                        let workflow_id = queued_workflow.workflow.id;
                        match workflow_engine.start_workflow(workflow_id).await {
                            Ok(()) => {
                                info!("Successfully started workflow: {}", workflow_id);
                            }
                            Err(e) => {
                                error!("Failed to start workflow {}: {}", workflow_id, e);
                                // Mark workflow as failed in queue
                                if let Err(queue_err) = queue_manager.fail_workflow(workflow_id, e.to_string()).await {
                                    error!("Failed to mark workflow as failed in queue: {}", queue_err);
                                }
                            }
                        }
                    }
                    Ok(None) => {
                        // No workflows to process or max concurrent reached
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    }
                    Err(e) => {
                        error!("Error dequeuing workflow: {}", e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }
            }
        });

        info!("Background queue processor started");
        Ok(())
    }
}

#[async_trait::async_trait]
impl Service for ResearchEngineService {
    async fn health_check(&self) -> AppResult<()> {
        debug!("Performing research engine health check");

        // Check workflow orchestrator
        if let Some(orchestrator) = &self.workflow_orchestrator {
            // Test orchestrator connectivity
            debug!("Checking workflow orchestrator health");
        }

        // Check queue manager
        let queue_manager = self.queue_manager.read().await;
        let queue_stats = queue_manager.get_queue_stats().await?;
        debug!("Queue manager health - {} items in queue, {} active",
               queue_stats.queue_length, queue_stats.active_count);
        drop(queue_manager);

        // Check template manager
        let template_manager = self.template_manager.read().await;
        match template_manager.get_all_templates().await {
            Ok(templates) => {
                debug!("Template manager health - {} templates available", templates.len());
            }
            Err(e) => {
                error!("Template manager health check failed: {}", e);
                return Err(e);
            }
        }
        drop(template_manager);

        // Check workflow statistics
        match self.get_workflow_statistics().await {
            Ok(stats) => {
                debug!("Research engine health check passed - {} total workflows, {} active",
                       stats.total_workflows, stats.active_workflows);
            }
            Err(e) => {
                error!("Failed to get workflow statistics during health check: {}", e);
                return Err(e);
            }
        }

        debug!("Research engine health check completed successfully");
        Ok(())
    }

    async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down research engine service...");
        
        // TODO: Implement graceful shutdown
        
        Ok(())
    }
}
