use std::sync::Arc;
use std::collections::{VecDeque, HashMap};
use tokio::sync::{RwLock, Mutex};
use tracing::{info, debug, warn, error};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, ResearchError};
use crate::models::research_workflow::{ResearchWorkflow, WorkflowStatus};

/// Queue manager for research workflow execution
pub struct QueueManager {
    queue: Arc<Mutex<VecDeque<QueuedWorkflow>>>,
    active_workflows: Arc<RwLock<HashMap<Uuid, QueuedWorkflow>>>,
    max_concurrent: usize,
    workflow_history: Arc<RwLock<Vec<QueuedWorkflow>>>,
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
            max_concurrent,
            workflow_history: Arc::new(RwLock::new(Vec::new())),
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
        let mut queue = self.queue.lock().await;
        
        // Check if we can start a new workflow
        let active_count = {
            let active_workflows = self.active_workflows.read().await;
            active_workflows.len()
        };
        
        if active_count >= self.max_concurrent {
            debug!("Maximum concurrent workflows reached ({}), cannot dequeue", self.max_concurrent);
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
        let estimated_wait_minutes = if queue_length > 0 && active_count > 0 {
            let avg_duration = active_workflows.values()
                .map(|w| w.estimated_duration_minutes)
                .sum::<u32>() / active_count as u32;
            avg_duration * (queue_length as u32 / self.max_concurrent as u32)
        } else {
            0
        };
        
        Ok(QueueStats {
            queue_length,
            active_count,
            max_concurrent: self.max_concurrent,
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
}

/// Queue statistics
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
