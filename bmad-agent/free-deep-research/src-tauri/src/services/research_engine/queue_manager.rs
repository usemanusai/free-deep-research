use std::sync::Arc;
use std::collections::{VecDeque, HashMap};
use tokio::sync::{RwLock, Mutex};
use tracing::{info, debug, warn, error};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, ResearchError};
use crate::models::research_workflow::{ResearchWorkflow, WorkflowStatus, StepStatus};

/// Queue manager for research workflow execution
pub struct QueueManager {
    queue: Arc<Mutex<VecDeque<QueuedWorkflow>>>,
    active_workflows: Arc<RwLock<HashMap<Uuid, QueuedWorkflow>>>,
    max_concurrent: Arc<RwLock<usize>>,
    workflow_history: Arc<RwLock<Vec<QueuedWorkflow>>>,
    is_processing: Arc<RwLock<bool>>,
    queue_state: Arc<RwLock<QueueState>>,
    last_state_change: Arc<RwLock<DateTime<Utc>>>,
    state_change_reason: Arc<RwLock<String>>,
    resource_limits: Arc<RwLock<ResourceLimits>>,
    current_resource_usage: Arc<RwLock<ResourceUsage>>,
    resource_allocations: Arc<RwLock<HashMap<Uuid, ResourceAllocation>>>,
    resource_history: Arc<RwLock<Vec<ResourceUsage>>>,
}

/// Queued workflow with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedWorkflow {
    pub workflow: ResearchWorkflow,
    pub priority: WorkflowPriority,
    pub queued_at: DateTime<Utc>,
    pub estimated_duration_minutes: u32,
    pub retry_count: u32,
    pub max_retries: u32,
}

/// Workflow priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WorkflowPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

impl QueueManager {
    /// Create a new queue manager
    pub async fn new(max_concurrent: usize) -> AppResult<Self> {
        info!("Initializing queue manager with max concurrent workflows: {}", max_concurrent);
        
        let manager = Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            active_workflows: Arc::new(RwLock::new(HashMap::new())),
            max_concurrent: Arc::new(RwLock::new(max_concurrent)),
            workflow_history: Arc::new(RwLock::new(Vec::new())),
            is_processing: Arc::new(RwLock::new(false)),
            queue_state: Arc::new(RwLock::new(QueueState::Stopped)),
            last_state_change: Arc::new(RwLock::new(Utc::now())),
            state_change_reason: Arc::new(RwLock::new("Initial state".to_string())),
            resource_limits: Arc::new(RwLock::new(ResourceLimits::default())),
            current_resource_usage: Arc::new(RwLock::new(ResourceUsage {
                memory_mb: 0,
                cpu_percentage: 0.0,
                api_calls_current_hour: 0,
                concurrent_requests: 0,
                bandwidth_mbps: 0.0,
                storage_mb: 0,
                active_workflows: 0,
                timestamp: Utc::now(),
            })),
            resource_allocations: Arc::new(RwLock::new(HashMap::new())),
            resource_history: Arc::new(RwLock::new(Vec::new())),
        };
        
        info!("Queue manager initialized successfully");
        Ok(manager)
    }
    
    /// Add a workflow to the queue
    pub async fn enqueue_workflow(
        &self,
        workflow: ResearchWorkflow,
        priority: WorkflowPriority,
        estimated_duration_minutes: Option<u32>,
    ) -> AppResult<()> {
        debug!("Enqueuing workflow: {} (priority: {:?})", workflow.name, priority);
        
        let queued_workflow = QueuedWorkflow {
            workflow,
            priority,
            queued_at: Utc::now(),
            estimated_duration_minutes: estimated_duration_minutes.unwrap_or(10),
            retry_count: 0,
            max_retries: 3,
        };
        
        let mut queue = self.queue.lock().await;
        
        // Insert based on priority (higher priority first)
        let insert_position = queue
            .iter()
            .position(|item| item.priority < priority)
            .unwrap_or(queue.len());
        
        queue.insert(insert_position, queued_workflow.clone());
        drop(queue);
        
        info!("Workflow queued: {} (position: {}, priority: {:?})", 
            queued_workflow.workflow.name, insert_position, priority);
        
        Ok(())
    }
    
    /// Get the next workflow to execute
    pub async fn dequeue_workflow(&self) -> AppResult<Option<QueuedWorkflow>> {
        // Check if we can process workflows in current state
        if !self.can_process_workflows().await {
            debug!("Queue processing not allowed in current state");
            return Ok(None);
        }

        let mut queue = self.queue.lock().await;

        // Check if we can start a new workflow
        let active_count = {
            let active_workflows = self.active_workflows.read().await;
            active_workflows.len()
        };

        let max_concurrent = *self.max_concurrent.read().await;
        if active_count >= max_concurrent {
            debug!("Maximum concurrent workflows reached ({}), cannot dequeue", max_concurrent);
            return Ok(None);
        }
        
        // Get the highest priority workflow
        if let Some(queued_workflow) = queue.pop_front() {
            debug!("Dequeued workflow: {} (priority: {:?})", 
                queued_workflow.workflow.name, queued_workflow.priority);
            
            // Add to active workflows
            let mut active_workflows = self.active_workflows.write().await;
            active_workflows.insert(queued_workflow.workflow.id, queued_workflow.clone());
            drop(active_workflows);
            
            Ok(Some(queued_workflow))
        } else {
            debug!("No workflows in queue");
            Ok(None)
        }
    }
    
    /// Mark a workflow as completed
    pub async fn complete_workflow(&self, workflow_id: Uuid, final_workflow: ResearchWorkflow) -> AppResult<()> {
        debug!("Completing workflow: {}", workflow_id);
        
        let mut active_workflows = self.active_workflows.write().await;
        if let Some(mut queued_workflow) = active_workflows.remove(&workflow_id) {
            queued_workflow.workflow = final_workflow;
            drop(active_workflows);
            
            // Add to history
            let mut history = self.workflow_history.write().await;
            history.push(queued_workflow.clone());
            
            // Keep only last 100 workflows in history
            if history.len() > 100 {
                history.remove(0);
            }
            drop(history);
            
            info!("Workflow completed and moved to history: {}", workflow_id);
        } else {
            warn!("Attempted to complete workflow not in active list: {}", workflow_id);
        }
        
        Ok(())
    }
    
    /// Mark a workflow as failed and potentially retry
    pub async fn fail_workflow(&self, workflow_id: Uuid, error: String) -> AppResult<bool> {
        debug!("Failing workflow: {} - {}", workflow_id, error);
        
        let mut active_workflows = self.active_workflows.write().await;
        if let Some(mut queued_workflow) = active_workflows.remove(&workflow_id) {
            queued_workflow.retry_count += 1;
            
            if queued_workflow.retry_count <= queued_workflow.max_retries {
                // Retry the workflow
                info!("Retrying workflow: {} (attempt {}/{})", 
                    workflow_id, queued_workflow.retry_count, queued_workflow.max_retries);
                
                // Reset workflow status
                queued_workflow.workflow.status = WorkflowStatus::Created;
                queued_workflow.workflow.started_at = None;
                queued_workflow.workflow.completed_at = None;
                
                // Re-queue with lower priority
                let retry_priority = match queued_workflow.priority {
                    WorkflowPriority::Critical => WorkflowPriority::High,
                    WorkflowPriority::High => WorkflowPriority::Normal,
                    _ => WorkflowPriority::Low,
                };
                
                drop(active_workflows);
                
                let mut queue = self.queue.lock().await;
                queue.push_back(queued_workflow);
                drop(queue);
                
                Ok(true) // Workflow will be retried
            } else {
                // Max retries exceeded, move to history as failed
                queued_workflow.workflow.status = WorkflowStatus::Failed;
                drop(active_workflows);
                
                let mut history = self.workflow_history.write().await;
                history.push(queued_workflow);
                drop(history);
                
                error!("Workflow failed permanently after {} retries: {}", 
                    queued_workflow.max_retries, workflow_id);
                
                Ok(false) // Workflow failed permanently
            }
        } else {
            warn!("Attempted to fail workflow not in active list: {}", workflow_id);
            Ok(false)
        }
    }
    
    /// Cancel a workflow (remove from queue or active)
    pub async fn cancel_workflow(&self, workflow_id: Uuid) -> AppResult<bool> {
        debug!("Cancelling workflow: {}", workflow_id);
        
        // Check active workflows first
        {
            let mut active_workflows = self.active_workflows.write().await;
            if let Some(mut queued_workflow) = active_workflows.remove(&workflow_id) {
                queued_workflow.workflow.status = WorkflowStatus::Cancelled;
                drop(active_workflows);
                
                let mut history = self.workflow_history.write().await;
                history.push(queued_workflow);
                drop(history);
                
                info!("Active workflow cancelled: {}", workflow_id);
                return Ok(true);
            }
        }
        
        // Check queue
        {
            let mut queue = self.queue.lock().await;
            if let Some(pos) = queue.iter().position(|w| w.workflow.id == workflow_id) {
                let mut queued_workflow = queue.remove(pos).unwrap();
                queued_workflow.workflow.status = WorkflowStatus::Cancelled;
                drop(queue);
                
                let mut history = self.workflow_history.write().await;
                history.push(queued_workflow);
                drop(history);
                
                info!("Queued workflow cancelled: {}", workflow_id);
                return Ok(true);
            }
        }
        
        warn!("Workflow not found for cancellation: {}", workflow_id);
        Ok(false)
    }
    
    /// Get queue statistics
    pub async fn get_queue_stats(&self) -> AppResult<QueueStats> {
        let queue = self.queue.lock().await;
        let active_workflows = self.active_workflows.read().await;
        let history = self.workflow_history.read().await;
        
        let queue_length = queue.len();
        let active_count = active_workflows.len();
        let total_completed = history.iter().filter(|w| w.workflow.status == WorkflowStatus::Completed).count();
        let total_failed = history.iter().filter(|w| w.workflow.status == WorkflowStatus::Failed).count();
        let total_cancelled = history.iter().filter(|w| w.workflow.status == WorkflowStatus::Cancelled).count();
        
        // Calculate estimated wait time for next workflow
        let max_concurrent = *self.max_concurrent.read().await;
        let estimated_wait_minutes = if queue_length > 0 && active_count > 0 {
            let avg_duration = active_workflows.values()
                .map(|w| w.estimated_duration_minutes)
                .sum::<u32>() / active_count as u32;
            avg_duration * (queue_length as u32 / max_concurrent as u32)
        } else {
            0
        };

        Ok(QueueStats {
            queue_length,
            active_count,
            max_concurrent,
            total_completed,
            total_failed,
            total_cancelled,
            estimated_wait_minutes,
        })
    }
    
    /// Get active workflows
    pub async fn get_active_workflows(&self) -> AppResult<Vec<QueuedWorkflow>> {
        let active_workflows = self.active_workflows.read().await;
        Ok(active_workflows.values().cloned().collect())
    }
    
    /// Get queued workflows
    pub async fn get_queued_workflows(&self) -> AppResult<Vec<QueuedWorkflow>> {
        let queue = self.queue.lock().await;
        Ok(queue.iter().cloned().collect())
    }
    
    /// Get workflow history
    pub async fn get_workflow_history(&self, limit: Option<usize>) -> AppResult<Vec<QueuedWorkflow>> {
        let history = self.workflow_history.read().await;
        let limit = limit.unwrap_or(50);

        Ok(history.iter()
            .rev()
            .take(limit)
            .cloned()
            .collect())
    }

    /// Update maximum concurrent workflows (configurable parallelism)
    pub async fn update_max_concurrent(&self, new_max: usize) -> AppResult<()> {
        if new_max == 0 {
            return Err(crate::error::ResearchError::invalid_request(
                "Maximum concurrent workflows must be greater than 0".to_string()
            ).into());
        }

        let mut max_concurrent = self.max_concurrent.write().await;
        let old_max = *max_concurrent;
        *max_concurrent = new_max;
        drop(max_concurrent);

        info!("Updated maximum concurrent workflows from {} to {}", old_max, new_max);
        Ok(())
    }

    /// Get current concurrency configuration
    pub async fn get_concurrency_config(&self) -> AppResult<ConcurrencyConfig> {
        let max_concurrent = *self.max_concurrent.read().await;
        let active_count = {
            let active_workflows = self.active_workflows.read().await;
            active_workflows.len()
        };
        let queue_length = {
            let queue = self.queue.lock().await;
            queue.len()
        };
        let is_processing = *self.is_processing.read().await;

        Ok(ConcurrencyConfig {
            max_concurrent,
            current_active: active_count,
            queue_length,
            available_slots: max_concurrent.saturating_sub(active_count),
            is_processing,
            utilization_percentage: if max_concurrent > 0 {
                (active_count as f64 / max_concurrent as f64) * 100.0
            } else {
                0.0
            },
        })
    }

    /// Start queue processing
    pub async fn start_processing(&self) -> AppResult<()> {
        let mut is_processing = self.is_processing.write().await;
        *is_processing = true;
        drop(is_processing);

        info!("Queue processing started");
        Ok(())
    }

    /// Stop queue processing
    pub async fn stop_processing(&self) -> AppResult<()> {
        let mut is_processing = self.is_processing.write().await;
        *is_processing = false;
        drop(is_processing);

        info!("Queue processing stopped");
        Ok(())
    }

    /// Check if queue processing is active
    pub async fn is_processing(&self) -> bool {
        *self.is_processing.read().await
    }

    /// Get detailed progress for a specific workflow
    pub async fn get_workflow_progress(&self, workflow_id: Uuid) -> AppResult<Option<WorkflowProgress>> {
        // Check active workflows first
        let active_workflows = self.active_workflows.read().await;
        if let Some(queued_workflow) = active_workflows.get(&workflow_id) {
            let workflow = &queued_workflow.workflow;

            let total_steps = workflow.steps.len();
            let completed_steps = workflow.steps.iter()
                .filter(|s| s.status == StepStatus::Completed)
                .count();

            let progress_percentage = if total_steps > 0 {
                (completed_steps as f64 / total_steps as f64) * 100.0
            } else {
                0.0
            };

            let current_step = workflow.steps.iter()
                .find(|s| s.status == StepStatus::Running)
                .map(|s| s.step_type.clone());

            let current_step_index = workflow.steps.iter()
                .position(|s| s.status == StepStatus::Running)
                .unwrap_or(completed_steps);

            let elapsed_time_minutes = if let Some(started_at) = workflow.started_at {
                (Utc::now() - started_at).num_minutes() as f64
            } else {
                0.0
            };

            let remaining_time_minutes = if progress_percentage > 0.0 && elapsed_time_minutes > 0.0 {
                let estimated_total_time = elapsed_time_minutes / (progress_percentage / 100.0);
                Some(estimated_total_time - elapsed_time_minutes)
            } else {
                None
            };

            let estimated_completion_time = remaining_time_minutes.map(|remaining| {
                Utc::now() + chrono::Duration::minutes(remaining as i64)
            });

            let steps_progress: Vec<StepProgress> = workflow.steps.iter()
                .map(|step| StepProgress {
                    step_name: step.step_type.clone(),
                    status: step.status,
                    progress_percentage: match step.status {
                        StepStatus::Completed => 100.0,
                        StepStatus::Running => 50.0, // Assume 50% if running
                        StepStatus::Failed => 0.0,
                        _ => 0.0,
                    },
                    started_at: step.started_at,
                    completed_at: step.completed_at,
                    error_message: step.error.clone(),
                })
                .collect();

            return Ok(Some(WorkflowProgress {
                workflow_id,
                workflow_name: workflow.name.clone(),
                status: workflow.status,
                progress_percentage,
                current_step,
                current_step_index,
                total_steps,
                completed_steps,
                estimated_completion_time,
                elapsed_time_minutes,
                remaining_time_minutes,
                steps_progress,
            }));
        }
        drop(active_workflows);

        // Check workflow history
        let history = self.workflow_history.read().await;
        if let Some(queued_workflow) = history.iter().find(|w| w.workflow.id == workflow_id) {
            let workflow = &queued_workflow.workflow;
            let total_steps = workflow.steps.len();
            let completed_steps = workflow.steps.iter()
                .filter(|s| s.status == StepStatus::Completed)
                .count();

            let progress_percentage = match workflow.status {
                WorkflowStatus::Completed => 100.0,
                WorkflowStatus::Failed | WorkflowStatus::Cancelled => 0.0,
                _ => if total_steps > 0 {
                    (completed_steps as f64 / total_steps as f64) * 100.0
                } else {
                    0.0
                }
            };

            let elapsed_time_minutes = if let (Some(started_at), Some(completed_at)) = (workflow.started_at, workflow.completed_at) {
                (completed_at - started_at).num_minutes() as f64
            } else {
                0.0
            };

            let steps_progress: Vec<StepProgress> = workflow.steps.iter()
                .map(|step| StepProgress {
                    step_name: step.step_type.clone(),
                    status: step.status,
                    progress_percentage: match step.status {
                        StepStatus::Completed => 100.0,
                        StepStatus::Failed => 0.0,
                        _ => 0.0,
                    },
                    started_at: step.started_at,
                    completed_at: step.completed_at,
                    error_message: step.error.clone(),
                })
                .collect();

            return Ok(Some(WorkflowProgress {
                workflow_id,
                workflow_name: workflow.name.clone(),
                status: workflow.status,
                progress_percentage,
                current_step: None,
                current_step_index: completed_steps,
                total_steps,
                completed_steps,
                estimated_completion_time: None,
                elapsed_time_minutes,
                remaining_time_minutes: None,
                steps_progress,
            }));
        }

        Ok(None)
    }

    /// Get queue-wide progress overview
    pub async fn get_queue_progress(&self) -> AppResult<QueueProgress> {
        let queue = self.queue.lock().await;
        let active_workflows = self.active_workflows.read().await;
        let history = self.workflow_history.read().await;

        let total_workflows = active_workflows.len() + queue.len() + history.len();
        let active_workflows_count = active_workflows.len();
        let queued_workflows = queue.len();
        let completed_workflows = history.iter().filter(|w| w.workflow.status == WorkflowStatus::Completed).count();
        let failed_workflows = history.iter().filter(|w| w.workflow.status == WorkflowStatus::Failed).count();

        // Calculate overall progress
        let overall_progress_percentage = if total_workflows > 0 {
            (completed_workflows as f64 / total_workflows as f64) * 100.0
        } else {
            0.0
        };

        // Calculate average workflow duration
        let completed_durations: Vec<f64> = history.iter()
            .filter(|w| w.workflow.status == WorkflowStatus::Completed)
            .filter_map(|w| {
                if let (Some(started), Some(completed)) = (w.workflow.started_at, w.workflow.completed_at) {
                    Some((completed - started).num_minutes() as f64)
                } else {
                    None
                }
            })
            .collect();

        let average_workflow_duration_minutes = if !completed_durations.is_empty() {
            completed_durations.iter().sum::<f64>() / completed_durations.len() as f64
        } else {
            0.0
        };

        // Calculate throughput (workflows per hour)
        let throughput_workflows_per_hour = if average_workflow_duration_minutes > 0.0 {
            60.0 / average_workflow_duration_minutes
        } else {
            0.0
        };

        // Estimate completion time for remaining workflows
        let estimated_completion_time = if queued_workflows > 0 && average_workflow_duration_minutes > 0.0 {
            let max_concurrent = *self.max_concurrent.read().await;
            let estimated_minutes = if active_workflows_count < max_concurrent {
                // Can start immediately
                average_workflow_duration_minutes
            } else {
                // Need to wait for current workflows to complete
                let remaining_time_for_active = active_workflows.values()
                    .filter_map(|w| {
                        if let Some(started_at) = w.workflow.started_at {
                            let elapsed = (Utc::now() - started_at).num_minutes() as f64;
                            Some((w.estimated_duration_minutes as f64).max(elapsed) - elapsed)
                        } else {
                            Some(w.estimated_duration_minutes as f64)
                        }
                    })
                    .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap_or(0.0);

                remaining_time_for_active + (queued_workflows as f64 * average_workflow_duration_minutes / max_concurrent as f64)
            };
            Some(Utc::now() + chrono::Duration::minutes(estimated_minutes as i64))
        } else {
            None
        };

        // Get progress for all active workflows
        let mut active_workflow_progress = Vec::new();
        for workflow_id in active_workflows.keys() {
            if let Ok(Some(progress)) = self.get_workflow_progress(*workflow_id).await {
                active_workflow_progress.push(progress);
            }
        }

        drop(queue);
        drop(active_workflows);
        drop(history);

        Ok(QueueProgress {
            total_workflows,
            active_workflows: active_workflows_count,
            queued_workflows,
            completed_workflows,
            failed_workflows,
            overall_progress_percentage,
            estimated_completion_time,
            average_workflow_duration_minutes,
            throughput_workflows_per_hour,
            active_workflow_progress,
        })
    }

    /// Get progress history for analytics
    pub async fn get_progress_history(&self, hours: Option<u32>) -> AppResult<Vec<WorkflowProgress>> {
        let hours = hours.unwrap_or(24);
        let cutoff_time = Utc::now() - chrono::Duration::hours(hours as i64);

        let history = self.workflow_history.read().await;
        let mut progress_history = Vec::new();

        for queued_workflow in history.iter() {
            if let Some(completed_at) = queued_workflow.workflow.completed_at {
                if completed_at >= cutoff_time {
                    if let Ok(Some(progress)) = self.get_workflow_progress(queued_workflow.workflow.id).await {
                        progress_history.push(progress);
                    }
                }
            }
        }

        // Sort by completion time (most recent first)
        progress_history.sort_by(|a, b| {
            b.workflow_id.cmp(&a.workflow_id) // Simple sort by ID as proxy
        });

        Ok(progress_history)
    }

    /// Pause queue gracefully (finish current workflows, don't start new ones)
    pub async fn pause_queue_gracefully(&self, reason: String) -> AppResult<QueueManagementResult> {
        info!("Pausing queue gracefully: {}", reason);

        let current_state = *self.queue_state.read().await;
        if current_state == QueueState::Paused {
            return Ok(QueueManagementResult {
                success: false,
                message: "Queue is already paused".to_string(),
                affected_workflows: Vec::new(),
                new_state: current_state,
                timestamp: Utc::now(),
            });
        }

        // Update state
        self.set_queue_state(QueueState::Paused, reason).await?;

        Ok(QueueManagementResult {
            success: true,
            message: "Queue paused gracefully. Current workflows will continue, no new workflows will start.".to_string(),
            affected_workflows: Vec::new(),
            new_state: QueueState::Paused,
            timestamp: Utc::now(),
        })
    }

    /// Resume queue processing
    pub async fn resume_queue(&self, reason: String) -> AppResult<QueueManagementResult> {
        info!("Resuming queue: {}", reason);

        let current_state = *self.queue_state.read().await;
        if current_state == QueueState::Running {
            return Ok(QueueManagementResult {
                success: false,
                message: "Queue is already running".to_string(),
                affected_workflows: Vec::new(),
                new_state: current_state,
                timestamp: Utc::now(),
            });
        }

        // Update state
        self.set_queue_state(QueueState::Running, reason).await?;

        Ok(QueueManagementResult {
            success: true,
            message: "Queue resumed. Processing will continue normally.".to_string(),
            affected_workflows: Vec::new(),
            new_state: QueueState::Running,
            timestamp: Utc::now(),
        })
    }

    /// Emergency stop - immediately halt all processing
    pub async fn emergency_stop(&self, reason: String) -> AppResult<QueueManagementResult> {
        warn!("Emergency stop initiated: {}", reason);

        // Update state
        self.set_queue_state(QueueState::Emergency, reason).await?;

        // Get all active workflow IDs
        let active_workflows = self.active_workflows.read().await;
        let affected_workflows: Vec<Uuid> = active_workflows.keys().cloned().collect();
        drop(active_workflows);

        // Note: In a real implementation, you would also signal the workflow engine
        // to immediately stop all active workflows

        Ok(QueueManagementResult {
            success: true,
            message: format!("Emergency stop executed. {} active workflows affected.", affected_workflows.len()),
            affected_workflows,
            new_state: QueueState::Emergency,
            timestamp: Utc::now(),
        })
    }

    /// Drain queue - finish all workflows and stop processing
    pub async fn drain_queue(&self, reason: String) -> AppResult<QueueManagementResult> {
        info!("Draining queue: {}", reason);

        // Update state
        self.set_queue_state(QueueState::Draining, reason).await?;

        let queue_length = {
            let queue = self.queue.lock().await;
            queue.len()
        };

        Ok(QueueManagementResult {
            success: true,
            message: format!("Queue draining initiated. {} workflows in queue will be processed, then queue will stop.", queue_length),
            affected_workflows: Vec::new(),
            new_state: QueueState::Draining,
            timestamp: Utc::now(),
        })
    }

    /// Cancel multiple workflows
    pub async fn cancel_multiple_workflows(&self, workflow_ids: Vec<Uuid>, reason: String) -> AppResult<QueueManagementResult> {
        info!("Cancelling {} workflows: {}", workflow_ids.len(), reason);

        let mut affected_workflows = Vec::new();
        let mut success_count = 0;

        for workflow_id in workflow_ids {
            match self.cancel_workflow(workflow_id).await {
                Ok(true) => {
                    affected_workflows.push(workflow_id);
                    success_count += 1;
                }
                Ok(false) => {
                    debug!("Workflow not found for cancellation: {}", workflow_id);
                }
                Err(e) => {
                    error!("Failed to cancel workflow {}: {}", workflow_id, e);
                }
            }
        }

        let current_state = *self.queue_state.read().await;
        Ok(QueueManagementResult {
            success: success_count > 0,
            message: format!("Cancelled {} out of {} requested workflows", success_count, affected_workflows.len()),
            affected_workflows,
            new_state: current_state,
            timestamp: Utc::now(),
        })
    }

    /// Clear entire queue (remove all pending workflows)
    pub async fn clear_queue(&self, reason: String) -> AppResult<QueueManagementResult> {
        warn!("Clearing entire queue: {}", reason);

        let mut queue = self.queue.lock().await;
        let affected_workflows: Vec<Uuid> = queue.iter().map(|w| w.workflow.id).collect();
        queue.clear();
        drop(queue);

        Ok(QueueManagementResult {
            success: true,
            message: format!("Queue cleared. {} pending workflows removed.", affected_workflows.len()),
            affected_workflows,
            new_state: *self.queue_state.read().await,
            timestamp: Utc::now(),
        })
    }

    /// Set queue state with reason
    async fn set_queue_state(&self, new_state: QueueState, reason: String) -> AppResult<()> {
        let mut queue_state = self.queue_state.write().await;
        let mut last_change = self.last_state_change.write().await;
        let mut change_reason = self.state_change_reason.write().await;

        *queue_state = new_state;
        *last_change = Utc::now();
        *change_reason = reason;

        // Update processing flag based on state
        let mut is_processing = self.is_processing.write().await;
        *is_processing = matches!(new_state, QueueState::Running | QueueState::Draining);

        info!("Queue state changed to: {:?}", new_state);
        Ok(())
    }

    /// Get queue management status
    pub async fn get_queue_management_status(&self) -> AppResult<QueueManagementStatus> {
        let current_state = *self.queue_state.read().await;
        let is_processing = *self.is_processing.read().await;
        let last_state_change = *self.last_state_change.read().await;
        let state_change_reason = self.state_change_reason.read().await.clone();

        let active_count = {
            let active_workflows = self.active_workflows.read().await;
            active_workflows.len()
        };

        let queue_length = {
            let queue = self.queue.lock().await;
            queue.len()
        };

        // Determine what operations are allowed in current state
        let can_pause = matches!(current_state, QueueState::Running);
        let can_resume = matches!(current_state, QueueState::Paused | QueueState::Stopped);
        let can_stop = matches!(current_state, QueueState::Running | QueueState::Paused | QueueState::Draining);
        let can_emergency_stop = !matches!(current_state, QueueState::Emergency | QueueState::Stopped);

        let pending_operations = match current_state {
            QueueState::Draining => vec!["Draining queue - finishing active workflows".to_string()],
            QueueState::Emergency => vec!["Emergency stop active - all processing halted".to_string()],
            _ => Vec::new(),
        };

        Ok(QueueManagementStatus {
            current_state,
            is_processing,
            pending_operations,
            last_state_change,
            state_change_reason,
            can_pause,
            can_resume,
            can_stop,
            can_emergency_stop,
        })
    }

    /// Check if queue operation is allowed in current state
    pub async fn can_process_workflows(&self) -> bool {
        let state = *self.queue_state.read().await;
        matches!(state, QueueState::Running | QueueState::Draining)
    }

    /// Check if resources are available for a new workflow
    pub async fn can_allocate_resources(&self, estimated_requirements: &ResourceLimits) -> AppResult<bool> {
        let current_usage = self.current_resource_usage.read().await;
        let limits = self.resource_limits.read().await;

        // Check each resource constraint
        let memory_ok = current_usage.memory_mb + estimated_requirements.max_memory_mb <= limits.max_memory_mb;
        let cpu_ok = current_usage.cpu_percentage + estimated_requirements.max_cpu_percentage <= limits.max_cpu_percentage;
        let api_calls_ok = current_usage.api_calls_current_hour + estimated_requirements.max_api_calls_per_hour <= limits.max_api_calls_per_hour;
        let requests_ok = current_usage.concurrent_requests + estimated_requirements.max_concurrent_requests <= limits.max_concurrent_requests;
        let bandwidth_ok = current_usage.bandwidth_mbps + estimated_requirements.max_bandwidth_mbps <= limits.max_bandwidth_mbps;
        let storage_ok = current_usage.storage_mb + estimated_requirements.max_storage_mb <= limits.max_storage_mb;

        Ok(memory_ok && cpu_ok && api_calls_ok && requests_ok && bandwidth_ok && storage_ok)
    }

    /// Allocate resources for a workflow
    pub async fn allocate_resources(&self, workflow_id: Uuid, requirements: ResourceLimits) -> AppResult<ResourceAllocation> {
        // Check if resources are available
        if !self.can_allocate_resources(&requirements).await? {
            return Err(crate::error::ResearchError::resource_limit_exceeded(
                "Insufficient resources available for workflow".to_string()
            ).into());
        }

        // Create allocation
        let allocation = ResourceAllocation {
            workflow_id,
            allocated_memory_mb: requirements.max_memory_mb,
            allocated_cpu_percentage: requirements.max_cpu_percentage,
            allocated_api_calls_per_hour: requirements.max_api_calls_per_hour,
            allocated_concurrent_requests: requirements.max_concurrent_requests,
            allocated_bandwidth_mbps: requirements.max_bandwidth_mbps,
            allocated_storage_mb: requirements.max_storage_mb,
            allocation_timestamp: Utc::now(),
            estimated_duration_minutes: requirements.max_execution_time_minutes,
        };

        // Update current usage
        {
            let mut current_usage = self.current_resource_usage.write().await;
            current_usage.memory_mb += allocation.allocated_memory_mb;
            current_usage.cpu_percentage += allocation.allocated_cpu_percentage;
            current_usage.api_calls_current_hour += allocation.allocated_api_calls_per_hour;
            current_usage.concurrent_requests += allocation.allocated_concurrent_requests;
            current_usage.bandwidth_mbps += allocation.allocated_bandwidth_mbps;
            current_usage.storage_mb += allocation.allocated_storage_mb;
            current_usage.active_workflows += 1;
            current_usage.timestamp = Utc::now();
        }

        // Store allocation
        {
            let mut allocations = self.resource_allocations.write().await;
            allocations.insert(workflow_id, allocation.clone());
        }

        info!("Allocated resources for workflow {}: {}MB memory, {}% CPU",
            workflow_id, allocation.allocated_memory_mb, allocation.allocated_cpu_percentage);

        Ok(allocation)
    }

    /// Deallocate resources when workflow completes
    pub async fn deallocate_resources(&self, workflow_id: Uuid) -> AppResult<()> {
        let allocation = {
            let mut allocations = self.resource_allocations.write().await;
            allocations.remove(&workflow_id)
        };

        if let Some(allocation) = allocation {
            // Update current usage
            let mut current_usage = self.current_resource_usage.write().await;
            current_usage.memory_mb = current_usage.memory_mb.saturating_sub(allocation.allocated_memory_mb);
            current_usage.cpu_percentage = (current_usage.cpu_percentage - allocation.allocated_cpu_percentage).max(0.0);
            current_usage.api_calls_current_hour = current_usage.api_calls_current_hour.saturating_sub(allocation.allocated_api_calls_per_hour);
            current_usage.concurrent_requests = current_usage.concurrent_requests.saturating_sub(allocation.allocated_concurrent_requests);
            current_usage.bandwidth_mbps = (current_usage.bandwidth_mbps - allocation.allocated_bandwidth_mbps).max(0.0);
            current_usage.storage_mb = current_usage.storage_mb.saturating_sub(allocation.allocated_storage_mb);
            current_usage.active_workflows = current_usage.active_workflows.saturating_sub(1);
            current_usage.timestamp = Utc::now();

            info!("Deallocated resources for workflow {}: {}MB memory, {}% CPU",
                workflow_id, allocation.allocated_memory_mb, allocation.allocated_cpu_percentage);
        }

        Ok(())
    }

    /// Get current resource status
    pub async fn get_resource_status(&self) -> AppResult<ResourceStatus> {
        let current_usage = self.current_resource_usage.read().await.clone();
        let limits = self.resource_limits.read().await.clone();

        // Calculate utilization
        let memory_util = (current_usage.memory_mb as f64 / limits.max_memory_mb as f64) * 100.0;
        let cpu_util = (current_usage.cpu_percentage / limits.max_cpu_percentage) * 100.0;
        let api_util = (current_usage.api_calls_current_hour as f64 / limits.max_api_calls_per_hour as f64) * 100.0;
        let requests_util = (current_usage.concurrent_requests as f64 / limits.max_concurrent_requests as f64) * 100.0;
        let bandwidth_util = (current_usage.bandwidth_mbps / limits.max_bandwidth_mbps) * 100.0;
        let storage_util = (current_usage.storage_mb as f64 / limits.max_storage_mb as f64) * 100.0;

        let utilization_percentage = [memory_util, cpu_util, api_util, requests_util, bandwidth_util, storage_util]
            .iter()
            .fold(0.0, |acc, &x| acc.max(x));

        // Calculate available resources
        let available_resources = ResourceUsage {
            memory_mb: limits.max_memory_mb.saturating_sub(current_usage.memory_mb),
            cpu_percentage: (limits.max_cpu_percentage - current_usage.cpu_percentage).max(0.0),
            api_calls_current_hour: limits.max_api_calls_per_hour.saturating_sub(current_usage.api_calls_current_hour),
            concurrent_requests: limits.max_concurrent_requests.saturating_sub(current_usage.concurrent_requests),
            bandwidth_mbps: (limits.max_bandwidth_mbps - current_usage.bandwidth_mbps).max(0.0),
            storage_mb: limits.max_storage_mb.saturating_sub(current_usage.storage_mb),
            active_workflows: current_usage.active_workflows,
            timestamp: Utc::now(),
        };

        // Check for over-limit conditions
        let is_over_limit = memory_util > 100.0 || cpu_util > 100.0 || api_util > 100.0 ||
                           requests_util > 100.0 || bandwidth_util > 100.0 || storage_util > 100.0;

        // Generate warnings
        let mut warnings = Vec::new();
        if memory_util > 90.0 { warnings.push("Memory usage above 90%".to_string()); }
        if cpu_util > 90.0 { warnings.push("CPU usage above 90%".to_string()); }
        if api_util > 90.0 { warnings.push("API call rate above 90%".to_string()); }
        if requests_util > 90.0 { warnings.push("Concurrent requests above 90%".to_string()); }
        if bandwidth_util > 90.0 { warnings.push("Bandwidth usage above 90%".to_string()); }
        if storage_util > 90.0 { warnings.push("Storage usage above 90%".to_string()); }

        // Generate recommendations
        let mut recommendations = Vec::new();
        if memory_util > 80.0 { recommendations.push("Consider increasing memory limits".to_string()); }
        if cpu_util > 80.0 { recommendations.push("Consider reducing concurrent workflows".to_string()); }
        if api_util > 80.0 { recommendations.push("Consider optimizing API usage patterns".to_string()); }

        // Estimate capacity
        let estimated_capacity_workflows = if utilization_percentage > 0.0 {
            ((100.0 - utilization_percentage) / utilization_percentage * current_usage.active_workflows as f64) as u32
        } else {
            10 // Default estimate when no workflows are running
        };

        Ok(ResourceStatus {
            current_usage,
            limits,
            utilization_percentage,
            available_resources,
            is_over_limit,
            warnings,
            recommendations,
            can_accept_new_workflow: utilization_percentage < 90.0 && !is_over_limit,
            estimated_capacity_workflows,
        })
    }

    /// Update resource limits
    pub async fn update_resource_limits(&self, new_limits: ResourceLimits) -> AppResult<()> {
        let mut limits = self.resource_limits.write().await;
        *limits = new_limits;
        info!("Updated resource limits: {}MB memory, {}% CPU", limits.max_memory_mb, limits.max_cpu_percentage);
        Ok(())
    }

    /// Record current resource usage for history
    pub async fn record_resource_usage(&self) -> AppResult<()> {
        let current_usage = self.current_resource_usage.read().await.clone();

        let mut history = self.resource_history.write().await;
        history.push(current_usage);

        // Keep only last 1000 entries
        if history.len() > 1000 {
            history.remove(0);
        }

        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStats {
    pub queue_length: usize,
    pub active_count: usize,
    pub max_concurrent: usize,
    pub total_completed: usize,
    pub total_failed: usize,
    pub total_cancelled: usize,
    pub estimated_wait_minutes: u32,
}

impl Default for QueueStats {
    fn default() -> Self {
        Self {
            queue_length: 0,
            active_count: 0,
            max_concurrent: 5,
            total_completed: 0,
            total_failed: 0,
            total_cancelled: 0,
            estimated_wait_minutes: 0,
        }
    }
}

/// Concurrency configuration and status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyConfig {
    pub max_concurrent: usize,
    pub current_active: usize,
    pub queue_length: usize,
    pub available_slots: usize,
    pub is_processing: bool,
    pub utilization_percentage: f64,
}

/// Detailed workflow progress information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowProgress {
    pub workflow_id: Uuid,
    pub workflow_name: String,
    pub status: WorkflowStatus,
    pub progress_percentage: f64,
    pub current_step: Option<String>,
    pub current_step_index: usize,
    pub total_steps: usize,
    pub completed_steps: usize,
    pub estimated_completion_time: Option<DateTime<Utc>>,
    pub elapsed_time_minutes: f64,
    pub remaining_time_minutes: Option<f64>,
    pub steps_progress: Vec<StepProgress>,
}

/// Individual step progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepProgress {
    pub step_name: String,
    pub status: StepStatus,
    pub progress_percentage: f64,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
}

/// Queue-wide progress overview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueProgress {
    pub total_workflows: usize,
    pub active_workflows: usize,
    pub queued_workflows: usize,
    pub completed_workflows: usize,
    pub failed_workflows: usize,
    pub overall_progress_percentage: f64,
    pub estimated_completion_time: Option<DateTime<Utc>>,
    pub average_workflow_duration_minutes: f64,
    pub throughput_workflows_per_hour: f64,
    pub active_workflow_progress: Vec<WorkflowProgress>,
}

/// Progress update event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressUpdate {
    pub workflow_id: Uuid,
    pub update_type: ProgressUpdateType,
    pub timestamp: DateTime<Utc>,
    pub progress_percentage: f64,
    pub message: String,
}

/// Types of progress updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgressUpdateType {
    WorkflowStarted,
    StepStarted,
    StepProgress,
    StepCompleted,
    WorkflowCompleted,
    WorkflowFailed,
    WorkflowCancelled,
}

/// Queue state for management operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueueState {
    Running,    // Normal operation, processing workflows
    Paused,     // Paused, no new workflows started but active ones continue
    Stopped,    // Stopped, no processing at all
    Draining,   // Finishing current workflows, no new ones started
    Emergency,  // Emergency stop, all processing halted immediately
}

/// Queue management operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueManagementResult {
    pub success: bool,
    pub message: String,
    pub affected_workflows: Vec<Uuid>,
    pub new_state: QueueState,
    pub timestamp: DateTime<Utc>,
}

/// Queue management status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueManagementStatus {
    pub current_state: QueueState,
    pub is_processing: bool,
    pub pending_operations: Vec<String>,
    pub last_state_change: DateTime<Utc>,
    pub state_change_reason: String,
    pub can_pause: bool,
    pub can_resume: bool,
    pub can_stop: bool,
    pub can_emergency_stop: bool,
}

/// Bulk operation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOperationRequest {
    pub operation_type: BulkOperationType,
    pub workflow_ids: Vec<Uuid>,
    pub reason: Option<String>,
}

/// Types of bulk operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BulkOperationType {
    Cancel,
    Pause,
    Resume,
    ChangePriority(WorkflowPriority),
    Remove,
}

/// Resource limits for workflows and queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_memory_mb: u64,
    pub max_cpu_percentage: f64,
    pub max_api_calls_per_hour: u32,
    pub max_concurrent_requests: u32,
    pub max_bandwidth_mbps: f64,
    pub max_storage_mb: u64,
    pub max_execution_time_minutes: u32,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: 1024,           // 1GB default
            max_cpu_percentage: 50.0,      // 50% CPU default
            max_api_calls_per_hour: 1000,  // 1000 API calls per hour
            max_concurrent_requests: 10,   // 10 concurrent requests
            max_bandwidth_mbps: 10.0,      // 10 Mbps bandwidth
            max_storage_mb: 500,           // 500MB storage
            max_execution_time_minutes: 60, // 1 hour max execution
        }
    }
}

/// Current resource usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub memory_mb: u64,
    pub cpu_percentage: f64,
    pub api_calls_current_hour: u32,
    pub concurrent_requests: u32,
    pub bandwidth_mbps: f64,
    pub storage_mb: u64,
    pub active_workflows: u32,
    pub timestamp: DateTime<Utc>,
}

/// Resource allocation for a specific workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocation {
    pub workflow_id: Uuid,
    pub allocated_memory_mb: u64,
    pub allocated_cpu_percentage: f64,
    pub allocated_api_calls_per_hour: u32,
    pub allocated_concurrent_requests: u32,
    pub allocated_bandwidth_mbps: f64,
    pub allocated_storage_mb: u64,
    pub allocation_timestamp: DateTime<Utc>,
    pub estimated_duration_minutes: u32,
}

/// Resource metrics and analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    pub average_memory_usage_mb: f64,
    pub peak_memory_usage_mb: u64,
    pub average_cpu_usage: f64,
    pub peak_cpu_usage: f64,
    pub total_api_calls: u32,
    pub api_calls_per_hour_average: f64,
    pub bandwidth_usage_average_mbps: f64,
    pub peak_bandwidth_mbps: f64,
    pub resource_efficiency_percentage: f64,
    pub resource_waste_percentage: f64,
    pub time_period_hours: u32,
    pub workflows_completed: u32,
}

/// Resource status and recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceStatus {
    pub current_usage: ResourceUsage,
    pub limits: ResourceLimits,
    pub utilization_percentage: f64,
    pub available_resources: ResourceUsage,
    pub is_over_limit: bool,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
    pub can_accept_new_workflow: bool,
    pub estimated_capacity_workflows: u32,
}

/// Resource optimization recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRecommendation {
    pub recommendation_type: RecommendationType,
    pub priority: RecommendationPriority,
    pub description: String,
    pub expected_improvement: String,
    pub implementation_effort: ImplementationEffort,
}

/// Types of resource recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    IncreaseMemoryLimit,
    DecreaseConcurrency,
    OptimizeApiUsage,
    AdjustBandwidthLimits,
    ScheduleMaintenanceWindow,
    UpgradeResources,
    OptimizeWorkflowPriorities,
}

/// Priority levels for recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Critical,
    High,
    Medium,
    Low,
}

/// Implementation effort for recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationEffort {
    Immediate,  // Can be done right now
    Quick,      // Within minutes
    Moderate,   // Within hours
    Significant, // Within days
}
