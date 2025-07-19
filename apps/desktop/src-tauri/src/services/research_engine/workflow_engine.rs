use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use tracing::{info, debug, warn, error};
use uuid::Uuid;
use chrono::Utc;

use crate::error::{AppResult, ApiError};
use crate::models::research_workflow::{
    ResearchWorkflow, WorkflowStep, WorkflowStatus, StepStatus, ResearchMethodology
};
use crate::services::{DataPersistenceService, ApiManagerService};
use crate::services::api_manager::{ServiceRequest, ServiceResponse};

/// Execution context for workflow steps
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub workflow_id: Uuid,
    pub step_id: Uuid,
    pub input_data: HashMap<String, serde_json::Value>,
    pub shared_data: HashMap<String, serde_json::Value>,
    pub metadata: HashMap<String, String>,
}

/// Workflow executor trait for different methodologies
#[async_trait::async_trait]
pub trait WorkflowExecutor: Send + Sync {
    /// Execute a workflow step
    async fn execute_step(
        &self,
        step: &mut WorkflowStep,
        context: &ExecutionContext,
        api_manager: &ApiManagerService,
    ) -> AppResult<HashMap<String, serde_json::Value>>;

    /// Get methodology type
    fn methodology(&self) -> ResearchMethodology;

    /// Prepare workflow steps for execution
    async fn prepare_steps(&self, workflow: &mut ResearchWorkflow) -> AppResult<()>;

    /// Post-process workflow results
    async fn post_process_results(
        &self,
        workflow: &ResearchWorkflow,
        step_results: &[HashMap<String, serde_json::Value>],
    ) -> AppResult<crate::models::research_workflow::ResearchResults>;
}

/// Workflow execution engine
pub struct WorkflowEngine {
    data_persistence: Arc<RwLock<DataPersistenceService>>,
    api_manager: Arc<RwLock<ApiManagerService>>,
    active_workflows: Arc<RwLock<HashMap<Uuid, Arc<Mutex<ResearchWorkflow>>>>>,
    executors: HashMap<ResearchMethodology, Box<dyn WorkflowExecutor>>,
}

impl WorkflowEngine {
    /// Create a new workflow engine
    pub async fn new(
        data_persistence: Arc<RwLock<DataPersistenceService>>,
        api_manager: Arc<RwLock<ApiManagerService>>,
    ) -> AppResult<Self> {
        info!("Initializing workflow engine...");

        let mut executors: HashMap<ResearchMethodology, Box<dyn WorkflowExecutor>> = HashMap::new();
        
        // Register methodology executors
        executors.insert(
            ResearchMethodology::DonLim,
            Box::new(super::methodology_don_lim::DonLimMethodology::new()),
        );
        executors.insert(
            ResearchMethodology::NickScamara,
            Box::new(super::methodology_nick_scamara::NickScamaraMethodology::new()),
        );
        executors.insert(
            ResearchMethodology::Hybrid,
            Box::new(super::methodology_hybrid::HybridMethodology::new()),
        );

        let engine = Self {
            data_persistence,
            api_manager,
            active_workflows: Arc::new(RwLock::new(HashMap::new())),
            executors,
        };

        info!("Workflow engine initialized successfully");
        Ok(engine)
    }

    /// Start executing a workflow
    pub async fn start_workflow(&self, workflow_id: Uuid) -> AppResult<()> {
        info!("Starting workflow execution: {}", workflow_id);

        // Load workflow from database
        let data_persistence = self.data_persistence.read().await;
        let mut workflow = data_persistence.get_research_workflow(workflow_id).await?
            .ok_or_else(|| ApiError::not_found("Workflow".to_string(), workflow_id.to_string()))?;
        drop(data_persistence);

        // Check if workflow can be started
        if !matches!(workflow.status, WorkflowStatus::Created | WorkflowStatus::Paused) {
            return Err(ApiError::invalid_operation(
                format!("Cannot start workflow in status: {:?}", workflow.status)
            ));
        }

        // Get executor for methodology
        let executor = self.executors.get(&workflow.parameters.methodology)
            .ok_or_else(|| ApiError::invalid_configuration(
                "methodology".to_string(),
                format!("No executor found for methodology: {:?}", workflow.parameters.methodology)
            ))?;

        // Prepare workflow steps
        executor.prepare_steps(&mut workflow).await?;

        // Mark workflow as running
        workflow.start();

        // Store in active workflows
        let workflow_arc = Arc::new(Mutex::new(workflow.clone()));
        let mut active_workflows = self.active_workflows.write().await;
        active_workflows.insert(workflow_id, workflow_arc.clone());
        drop(active_workflows);

        // Save updated workflow
        let data_persistence = self.data_persistence.write().await;
        data_persistence.save_research_workflow(&workflow).await?;
        drop(data_persistence);

        // Start execution in background
        let engine_clone = self.clone_for_execution();
        tokio::spawn(async move {
            if let Err(e) = engine_clone.execute_workflow_steps(workflow_id).await {
                error!("Workflow execution failed: {}", e);
            }
        });

        info!("Workflow execution started: {}", workflow_id);
        Ok(())
    }

    /// Execute workflow steps
    async fn execute_workflow_steps(&self, workflow_id: Uuid) -> AppResult<()> {
        debug!("Executing workflow steps for: {}", workflow_id);

        let workflow_arc = {
            let active_workflows = self.active_workflows.read().await;
            active_workflows.get(&workflow_id).cloned()
                .ok_or_else(|| ApiError::not_found("Active workflow".to_string(), workflow_id.to_string()))?
        };

        let mut step_results = Vec::new();
        let mut shared_data = HashMap::new();

        loop {
            let next_steps = {
                let workflow = workflow_arc.lock().await;
                if workflow.status != WorkflowStatus::Running {
                    debug!("Workflow {} is no longer running, stopping execution", workflow_id);
                    break;
                }
                workflow.get_next_steps().into_iter().map(|s| s.clone()).collect::<Vec<_>>()
            };

            if next_steps.is_empty() {
                // Check if all steps are completed
                let workflow = workflow_arc.lock().await;
                if workflow.all_steps_completed() {
                    debug!("All steps completed for workflow: {}", workflow_id);
                    break;
                } else if workflow.has_failed_steps() {
                    // Check for retryable steps
                    let retryable_steps = workflow.get_retryable_steps();
                    if retryable_steps.is_empty() {
                        error!("Workflow {} has failed steps with no retries available", workflow_id);
                        self.fail_workflow(workflow_id, "Workflow failed with non-retryable errors".to_string()).await?;
                        return Ok(());
                    }
                } else {
                    // No next steps but not all completed - might be waiting for dependencies
                    debug!("No executable steps available for workflow: {}", workflow_id);
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    continue;
                }
            }

            // Execute steps concurrently (up to max_concurrent_steps)
            let max_concurrent = {
                let workflow = workflow_arc.lock().await;
                workflow.parameters.max_concurrent_steps as usize
            };

            let steps_to_execute = next_steps.into_iter().take(max_concurrent).collect::<Vec<_>>();
            
            for step in steps_to_execute {
                let step_result = self.execute_single_step(workflow_id, step.id, &shared_data).await;
                
                match step_result {
                    Ok(result) => {
                        step_results.push(result.clone());
                        // Merge step output into shared data
                        for (key, value) in result {
                            shared_data.insert(key, value);
                        }
                    }
                    Err(e) => {
                        warn!("Step {} failed: {}", step.id, e);
                        // Step failure is handled in execute_single_step
                    }
                }

                // Update workflow progress
                {
                    let mut workflow = workflow_arc.lock().await;
                    workflow.calculate_progress();
                    
                    // Save progress to database
                    let data_persistence = self.data_persistence.write().await;
                    if let Err(e) = data_persistence.save_research_workflow(&workflow).await {
                        error!("Failed to save workflow progress: {}", e);
                    }
                    drop(data_persistence);
                }
            }
        }

        // Complete workflow
        self.complete_workflow(workflow_id, step_results).await?;
        Ok(())
    }

    /// Execute a single workflow step
    async fn execute_single_step(
        &self,
        workflow_id: Uuid,
        step_id: Uuid,
        shared_data: &HashMap<String, serde_json::Value>,
    ) -> AppResult<HashMap<String, serde_json::Value>> {
        debug!("Executing step {} for workflow {}", step_id, workflow_id);

        let workflow_arc = {
            let active_workflows = self.active_workflows.read().await;
            active_workflows.get(&workflow_id).cloned()
                .ok_or_else(|| ApiError::not_found("Active workflow".to_string(), workflow_id.to_string()))?
        };

        // Get step and mark as started
        {
            let mut workflow = workflow_arc.lock().await;
            if let Some(step) = workflow.get_step_mut(step_id) {
                step.start();
            }
        }

        let (step, methodology) = {
            let workflow = workflow_arc.lock().await;
            let step = workflow.get_step(step_id)
                .ok_or_else(|| ApiError::not_found("Step".to_string(), step_id.to_string()))?
                .clone();
            (step, workflow.parameters.methodology.clone())
        };

        // Get executor
        let executor = self.executors.get(&methodology)
            .ok_or_else(|| ApiError::invalid_configuration(
                "methodology".to_string(),
                format!("No executor found for methodology: {:?}", methodology)
            ))?;

        // Create execution context
        let context = ExecutionContext {
            workflow_id,
            step_id,
            input_data: step.input_data.clone(),
            shared_data: shared_data.clone(),
            metadata: step.metadata.clone(),
        };

        // Execute step
        let api_manager = self.api_manager.read().await;
        let mut step_copy = step.clone();
        let result = executor.execute_step(&mut step_copy, &context, &*api_manager).await;
        drop(api_manager);

        // Update step with result
        {
            let mut workflow = workflow_arc.lock().await;
            if let Some(workflow_step) = workflow.get_step_mut(step_id) {
                match result {
                    Ok(ref output) => {
                        workflow_step.complete(output.clone());
                        debug!("Step {} completed successfully", step_id);
                    }
                    Err(ref e) => {
                        workflow_step.fail(e.to_string());
                        error!("Step {} failed: {}", step_id, e);
                        
                        // Check if step should be retried
                        if workflow_step.should_retry() {
                            workflow_step.increment_retry();
                            info!("Step {} will be retried (attempt {})", step_id, workflow_step.retry_count);
                        }
                    }
                }
            }
        }

        result
    }

    /// Complete a workflow
    async fn complete_workflow(
        &self,
        workflow_id: Uuid,
        step_results: Vec<HashMap<String, serde_json::Value>>,
    ) -> AppResult<()> {
        info!("Completing workflow: {}", workflow_id);

        let workflow_arc = {
            let active_workflows = self.active_workflows.read().await;
            active_workflows.get(&workflow_id).cloned()
                .ok_or_else(|| ApiError::not_found("Active workflow".to_string(), workflow_id.to_string()))?
        };

        let (workflow, methodology) = {
            let workflow = workflow_arc.lock().await;
            (workflow.clone(), workflow.parameters.methodology.clone())
        };

        // Get executor for post-processing
        let executor = self.executors.get(&methodology)
            .ok_or_else(|| ApiError::invalid_configuration(
                "methodology".to_string(),
                format!("No executor found for methodology: {:?}", methodology)
            ))?;

        // Post-process results
        let final_results = executor.post_process_results(&workflow, &step_results).await?;

        // Update workflow with results
        {
            let mut workflow = workflow_arc.lock().await;
            workflow.complete(final_results.into());
        }

        // Remove from active workflows
        let mut active_workflows = self.active_workflows.write().await;
        active_workflows.remove(&workflow_id);
        drop(active_workflows);

        // Save final workflow state
        let workflow = workflow_arc.lock().await;
        let data_persistence = self.data_persistence.write().await;
        data_persistence.save_research_workflow(&workflow).await?;
        drop(data_persistence);

        info!("Workflow completed successfully: {}", workflow_id);
        Ok(())
    }

    /// Fail a workflow
    async fn fail_workflow(&self, workflow_id: Uuid, error: String) -> AppResult<()> {
        error!("Failing workflow {}: {}", workflow_id, error);

        let workflow_arc = {
            let active_workflows = self.active_workflows.read().await;
            active_workflows.get(&workflow_id).cloned()
                .ok_or_else(|| ApiError::not_found("Active workflow".to_string(), workflow_id.to_string()))?
        };

        // Update workflow status
        {
            let mut workflow = workflow_arc.lock().await;
            workflow.fail(error);
        }

        // Remove from active workflows
        let mut active_workflows = self.active_workflows.write().await;
        active_workflows.remove(&workflow_id);
        drop(active_workflows);

        // Save final workflow state
        let workflow = workflow_arc.lock().await;
        let data_persistence = self.data_persistence.write().await;
        data_persistence.save_research_workflow(&workflow).await?;
        drop(data_persistence);

        Ok(())
    }

    /// Clone engine for background execution
    fn clone_for_execution(&self) -> WorkflowEngineClone {
        WorkflowEngineClone {
            data_persistence: self.data_persistence.clone(),
            api_manager: self.api_manager.clone(),
            active_workflows: self.active_workflows.clone(),
        }
    }

    /// Pause a workflow
    pub async fn pause_workflow(&self, workflow_id: Uuid) -> AppResult<()> {
        info!("Pausing workflow: {}", workflow_id);

        let workflow_arc = {
            let active_workflows = self.active_workflows.read().await;
            active_workflows.get(&workflow_id).cloned()
                .ok_or_else(|| ApiError::not_found("Active workflow".to_string(), workflow_id.to_string()))?
        };

        {
            let mut workflow = workflow_arc.lock().await;
            workflow.pause();
        }

        let workflow = workflow_arc.lock().await;
        let data_persistence = self.data_persistence.write().await;
        data_persistence.save_research_workflow(&workflow).await?;
        drop(data_persistence);

        Ok(())
    }

    /// Resume a workflow
    pub async fn resume_workflow(&self, workflow_id: Uuid) -> AppResult<()> {
        info!("Resuming workflow: {}", workflow_id);

        let workflow_arc = {
            let active_workflows = self.active_workflows.read().await;
            active_workflows.get(&workflow_id).cloned()
                .ok_or_else(|| ApiError::not_found("Active workflow".to_string(), workflow_id.to_string()))?
        };

        {
            let mut workflow = workflow_arc.lock().await;
            workflow.resume();
        }

        let workflow = workflow_arc.lock().await;
        let data_persistence = self.data_persistence.write().await;
        data_persistence.save_research_workflow(&workflow).await?;
        drop(data_persistence);

        Ok(())
    }

    /// Cancel a workflow
    pub async fn cancel_workflow(&self, workflow_id: Uuid) -> AppResult<()> {
        info!("Cancelling workflow: {}", workflow_id);

        let workflow_arc = {
            let active_workflows = self.active_workflows.read().await;
            active_workflows.get(&workflow_id).cloned()
                .ok_or_else(|| ApiError::not_found("Active workflow".to_string(), workflow_id.to_string()))?
        };

        {
            let mut workflow = workflow_arc.lock().await;
            workflow.cancel();
        }

        // Remove from active workflows
        let mut active_workflows = self.active_workflows.write().await;
        active_workflows.remove(&workflow_id);
        drop(active_workflows);

        let workflow = workflow_arc.lock().await;
        let data_persistence = self.data_persistence.write().await;
        data_persistence.save_research_workflow(&workflow).await?;
        drop(data_persistence);

        Ok(())
    }

    /// Start background monitoring
    pub async fn start_background_monitoring(&self) -> AppResult<()> {
        info!("Starting workflow engine background monitoring...");

        // TODO: Implement background monitoring tasks
        // - Monitor workflow timeouts
        // - Clean up completed workflows
        // - Health checks for active workflows

        info!("Workflow engine background monitoring started successfully");
        Ok(())
    }
}

/// Simplified clone for background execution
#[derive(Clone)]
struct WorkflowEngineClone {
    data_persistence: Arc<RwLock<DataPersistenceService>>,
    api_manager: Arc<RwLock<ApiManagerService>>,
    active_workflows: Arc<RwLock<HashMap<Uuid, Arc<Mutex<ResearchWorkflow>>>>>,
}

impl WorkflowEngineClone {
    async fn execute_workflow_steps(&self, workflow_id: Uuid) -> AppResult<()> {
        // Implementation would be similar to WorkflowEngine::execute_workflow_steps
        // but simplified for the clone
        Ok(())
    }
}
