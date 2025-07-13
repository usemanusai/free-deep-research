use tauri::State;
use tracing::{info, error};
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::research_workflow::{ResearchWorkflow, ResearchMethodology, WorkflowStatus, WorkflowParameters};
use crate::services::ServiceManager;
use crate::services::research_engine::{
    QueuedWorkflow, WorkflowPriority, QueueStats, ConcurrencyConfig,
    WorkflowProgress, QueueProgress, StepProgress, QueueManagementResult, QueueManagementStatus,
    ResourceLimits, ResourceUsage, ResourceStatus, ResourceMetrics
};

/// Create a new research workflow
#[tauri::command]
pub async fn create_research_workflow(
    name: String,
    query: String,
    methodology: String,
    created_by: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<ResearchWorkflow, String> {
    info!("Creating research workflow: {}", name);

    let methodology_enum = match methodology.to_lowercase().as_str() {
        "don_lim" => ResearchMethodology::DonLim,
        "nick_scamara" => ResearchMethodology::NickScamara,
        "hybrid" => ResearchMethodology::Hybrid,
        _ => return Err(format!("Invalid methodology: {}", methodology)),
    };

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.create_workflow(name, query, methodology_enum, created_by).await {
        Ok(workflow) => {
            info!("Created research workflow with ID: {}", workflow.id);
            Ok(workflow)
        }
        Err(e) => {
            error!("Failed to create research workflow: {}", e);
            Err(e.to_string())
        }
    }
}

/// Start executing a research workflow
#[tauri::command]
pub async fn start_research_workflow(
    workflow_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Starting research workflow: {}", workflow_id);

    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.start_workflow_execution(workflow_uuid).await {
        Ok(()) => {
            info!("Started research workflow: {}", workflow_id);
            Ok(())
        }
        Err(e) => {
            error!("Failed to start research workflow {}: {}", workflow_id, e);
            Err(e.to_string())
        }
    }
}

/// Pause a running research workflow
#[tauri::command]
pub async fn pause_research_workflow(
    workflow_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Pausing research workflow: {}", workflow_id);

    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.pause_workflow_execution(workflow_uuid).await {
        Ok(()) => {
            info!("Paused research workflow: {}", workflow_id);
            Ok(())
        }
        Err(e) => {
            error!("Failed to pause research workflow {}: {}", workflow_id, e);
            Err(e.to_string())
        }
    }
}

/// Resume a paused research workflow
#[tauri::command]
pub async fn resume_research_workflow(
    workflow_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Resuming research workflow: {}", workflow_id);

    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.resume_workflow_execution(workflow_uuid).await {
        Ok(()) => {
            info!("Resumed research workflow: {}", workflow_id);
            Ok(())
        }
        Err(e) => {
            error!("Failed to resume research workflow {}: {}", workflow_id, e);
            Err(e.to_string())
        }
    }
}

/// Cancel a research workflow
#[tauri::command]
pub async fn cancel_research_workflow(
    workflow_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Cancelling research workflow: {}", workflow_id);

    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.cancel_workflow_execution(workflow_uuid).await {
        Ok(()) => {
            info!("Cancelled research workflow: {}", workflow_id);
            Ok(())
        }
        Err(e) => {
            error!("Failed to cancel research workflow {}: {}", workflow_id, e);
            Err(e.to_string())
        }
    }
}

/// Get a research workflow by ID
#[tauri::command]
pub async fn get_research_workflow(
    workflow_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Option<ResearchWorkflow>, String> {
    info!("Getting research workflow: {}", workflow_id);

    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.get_workflow(workflow_uuid).await {
        Ok(workflow) => Ok(workflow),
        Err(e) => {
            error!("Failed to get research workflow {}: {}", workflow_id, e);
            Err(e.to_string())
        }
    }
}

/// Get all research workflows
#[tauri::command]
pub async fn get_all_research_workflows(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<ResearchWorkflow>, String> {
    info!("Getting all research workflows");

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.get_all_workflows().await {
        Ok(workflows) => {
            info!("Retrieved {} research workflows", workflows.len());
            Ok(workflows)
        }
        Err(e) => {
            error!("Failed to get research workflows: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get research workflows by status
#[tauri::command]
pub async fn get_research_workflows_by_status(
    status: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<ResearchWorkflow>, String> {
    info!("Getting research workflows by status: {}", status);

    let workflow_status = match status.to_lowercase().as_str() {
        "created" => WorkflowStatus::Created,
        "pending" => WorkflowStatus::Pending,
        "running" => WorkflowStatus::Running,
        "paused" => WorkflowStatus::Paused,
        "completed" => WorkflowStatus::Completed,
        "failed" => WorkflowStatus::Failed,
        "cancelled" => WorkflowStatus::Cancelled,
        _ => return Err(format!("Invalid workflow status: {}", status)),
    };

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.get_workflows_by_status(workflow_status).await {
        Ok(workflows) => {
            info!("Retrieved {} research workflows with status: {}", workflows.len(), status);
            Ok(workflows)
        }
        Err(e) => {
            error!("Failed to get research workflows by status {}: {}", status, e);
            Err(e.to_string())
        }
    }
}

/// Delete a research workflow
#[tauri::command]
pub async fn delete_research_workflow(
    workflow_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Deleting research workflow: {}", workflow_id);

    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.delete_workflow(workflow_uuid).await {
        Ok(()) => {
            info!("Deleted research workflow: {}", workflow_id);
            Ok(())
        }
        Err(e) => {
            error!("Failed to delete research workflow {}: {}", workflow_id, e);
            Err(e.to_string())
        }
    }
}

/// Get workflow execution status
#[tauri::command]
pub async fn get_workflow_status(
    workflow_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Option<String>, String> {
    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.get_workflow_status(workflow_uuid).await {
        Ok(Some(status)) => Ok(Some(format!("{:?}", status).to_lowercase())),
        Ok(None) => Ok(None),
        Err(e) => {
            error!("Failed to get workflow status {}: {}", workflow_id, e);
            Err(e.to_string())
        }
    }
}

/// Get workflow progress
#[tauri::command]
pub async fn get_workflow_progress(
    workflow_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Option<f64>, String> {
    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.get_workflow_progress(workflow_uuid).await {
        Ok(progress) => Ok(progress),
        Err(e) => {
            error!("Failed to get workflow progress {}: {}", workflow_id, e);
            Err(e.to_string())
        }
    }
}

/// Get workflow results
#[tauri::command]
pub async fn get_workflow_results(
    workflow_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Option<crate::models::research_workflow::ResearchResults>, String> {
    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.get_workflow_results(workflow_uuid).await {
        Ok(results) => Ok(results),
        Err(e) => {
            error!("Failed to get workflow results {}: {}", workflow_id, e);
            Err(e.to_string())
        }
    }
}

/// Get workflow statistics
#[tauri::command]
pub async fn get_workflow_statistics(
    service_manager: State<'_, ServiceManager>,
) -> Result<crate::services::research_engine::WorkflowStatistics, String> {
    info!("Getting workflow statistics");

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.get_workflow_statistics().await {
        Ok(stats) => {
            info!("Retrieved workflow statistics");
            Ok(stats)
        }
        Err(e) => {
            error!("Failed to get workflow statistics: {}", e);
            Err(e.to_string())
        }
    }
}

// ============================================================================
// QUEUE MANAGEMENT COMMANDS
// ============================================================================

/// Enqueue a workflow for execution with priority
#[tauri::command]
pub async fn enqueue_research_workflow(
    workflow_id: String,
    priority: String,
    estimated_duration_minutes: Option<u32>,
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Enqueuing research workflow: {} with priority: {}", workflow_id, priority);

    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    let workflow_priority = match priority.to_lowercase().as_str() {
        "low" => WorkflowPriority::Low,
        "normal" => WorkflowPriority::Normal,
        "high" => WorkflowPriority::High,
        "critical" => WorkflowPriority::Critical,
        _ => return Err(format!("Invalid priority: {}", priority)),
    };

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.enqueue_workflow(workflow_uuid, workflow_priority, estimated_duration_minutes).await {
        Ok(()) => {
            info!("Enqueued research workflow: {}", workflow_id);
            Ok(())
        }
        Err(e) => {
            error!("Failed to enqueue research workflow {}: {}", workflow_id, e);
            Err(e.to_string())
        }
    }
}

/// Get queue statistics
#[tauri::command]
pub async fn get_queue_statistics(
    service_manager: State<'_, ServiceManager>,
) -> Result<QueueStats, String> {
    info!("Getting queue statistics");

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.get_queue_statistics().await {
        Ok(stats) => {
            info!("Retrieved queue statistics");
            Ok(stats)
        }
        Err(e) => {
            error!("Failed to get queue statistics: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get active workflows from queue
#[tauri::command]
pub async fn get_active_queue_workflows(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<QueuedWorkflow>, String> {
    info!("Getting active queue workflows");

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.get_active_queue_workflows().await {
        Ok(workflows) => {
            info!("Retrieved {} active queue workflows", workflows.len());
            Ok(workflows)
        }
        Err(e) => {
            error!("Failed to get active queue workflows: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get queued workflows
#[tauri::command]
pub async fn get_queued_workflows(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<QueuedWorkflow>, String> {
    info!("Getting queued workflows");

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.get_queued_workflows().await {
        Ok(workflows) => {
            info!("Retrieved {} queued workflows", workflows.len());
            Ok(workflows)
        }
        Err(e) => {
            error!("Failed to get queued workflows: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get workflow history from queue
#[tauri::command]
pub async fn get_workflow_queue_history(
    limit: Option<usize>,
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<QueuedWorkflow>, String> {
    info!("Getting workflow queue history");

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.get_workflow_history(limit).await {
        Ok(workflows) => {
            info!("Retrieved {} workflow history entries", workflows.len());
            Ok(workflows)
        }
        Err(e) => {
            error!("Failed to get workflow queue history: {}", e);
            Err(e.to_string())
        }
    }
}

/// Cancel a queued workflow
#[tauri::command]
pub async fn cancel_queued_workflow(
    workflow_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<bool, String> {
    info!("Cancelling queued workflow: {}", workflow_id);

    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.cancel_queued_workflow(workflow_uuid).await {
        Ok(cancelled) => {
            if cancelled {
                info!("Cancelled queued workflow: {}", workflow_id);
            } else {
                warn!("Workflow not found in queue: {}", workflow_id);
            }
            Ok(cancelled)
        }
        Err(e) => {
            error!("Failed to cancel queued workflow {}: {}", workflow_id, e);
            Err(e.to_string())
        }
    }
}

/// Update queue concurrency configuration
#[tauri::command]
pub async fn update_queue_concurrency(
    max_concurrent: usize,
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Updating queue concurrency to: {}", max_concurrent);

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.update_queue_concurrency(max_concurrent).await {
        Ok(()) => {
            info!("Updated queue concurrency to: {}", max_concurrent);
            Ok(())
        }
        Err(e) => {
            error!("Failed to update queue concurrency: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get queue concurrency configuration
#[tauri::command]
pub async fn get_queue_concurrency_config(
    service_manager: State<'_, ServiceManager>,
) -> Result<ConcurrencyConfig, String> {
    info!("Getting queue concurrency configuration");

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.get_queue_concurrency_config().await {
        Ok(config) => {
            info!("Retrieved queue concurrency configuration");
            Ok(config)
        }
        Err(e) => {
            error!("Failed to get queue concurrency configuration: {}", e);
            Err(e.to_string())
        }
    }
}

/// Start queue processing
#[tauri::command]
pub async fn start_queue_processing(
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Starting queue processing");

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.start_queue_processing().await {
        Ok(()) => {
            info!("Queue processing started");
            Ok(())
        }
        Err(e) => {
            error!("Failed to start queue processing: {}", e);
            Err(e.to_string())
        }
    }
}

/// Stop queue processing
#[tauri::command]
pub async fn stop_queue_processing(
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Stopping queue processing");

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.stop_queue_processing().await {
        Ok(()) => {
            info!("Queue processing stopped");
            Ok(())
        }
        Err(e) => {
            error!("Failed to stop queue processing: {}", e);
            Err(e.to_string())
        }
    }
}

// ============================================================================
// PROGRESS MONITORING COMMANDS
// ============================================================================

/// Get detailed workflow progress
#[tauri::command]
pub async fn get_workflow_progress_detailed(
    workflow_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Option<WorkflowProgress>, String> {
    info!("Getting detailed workflow progress: {}", workflow_id);

    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.get_workflow_progress_detailed(workflow_uuid).await {
        Ok(progress) => {
            if progress.is_some() {
                info!("Retrieved detailed progress for workflow: {}", workflow_id);
            } else {
                warn!("No progress found for workflow: {}", workflow_id);
            }
            Ok(progress)
        }
        Err(e) => {
            error!("Failed to get workflow progress {}: {}", workflow_id, e);
            Err(e.to_string())
        }
    }
}

/// Get queue-wide progress overview
#[tauri::command]
pub async fn get_queue_progress_overview(
    service_manager: State<'_, ServiceManager>,
) -> Result<QueueProgress, String> {
    info!("Getting queue progress overview");

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.get_queue_progress_overview().await {
        Ok(progress) => {
            info!("Retrieved queue progress overview");
            Ok(progress)
        }
        Err(e) => {
            error!("Failed to get queue progress overview: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get progress history for analytics
#[tauri::command]
pub async fn get_progress_history(
    hours: Option<u32>,
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<WorkflowProgress>, String> {
    let hours_str = hours.map(|h| h.to_string()).unwrap_or_else(|| "24".to_string());
    info!("Getting progress history for last {} hours", hours_str);

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.get_progress_history(hours).await {
        Ok(history) => {
            info!("Retrieved {} progress history entries", history.len());
            Ok(history)
        }
        Err(e) => {
            error!("Failed to get progress history: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get real-time monitoring data (combines multiple metrics)
#[tauri::command]
pub async fn get_real_time_monitoring_data(
    service_manager: State<'_, ServiceManager>,
) -> Result<serde_json::Value, String> {
    info!("Getting real-time monitoring data");

    let research_engine = service_manager.inner().research_engine.read().await;

    // Get all monitoring data in parallel
    let queue_stats_result = research_engine.get_queue_statistics().await;
    let queue_progress_result = research_engine.get_queue_progress_overview().await;
    let concurrency_config_result = research_engine.get_queue_concurrency_config().await;
    let workflow_stats_result = research_engine.get_workflow_statistics().await;

    match (queue_stats_result, queue_progress_result, concurrency_config_result, workflow_stats_result) {
        (Ok(queue_stats), Ok(queue_progress), Ok(concurrency_config), Ok(workflow_stats)) => {
            let monitoring_data = serde_json::json!({
                "queue_stats": queue_stats,
                "queue_progress": queue_progress,
                "concurrency_config": concurrency_config,
                "workflow_stats": workflow_stats,
                "timestamp": chrono::Utc::now(),
                "system_status": "healthy"
            });

            info!("Retrieved comprehensive real-time monitoring data");
            Ok(monitoring_data)
        }
        _ => {
            error!("Failed to retrieve some monitoring data components");
            Err("Failed to retrieve complete monitoring data".to_string())
        }
    }
}

// ============================================================================
// QUEUE MANAGEMENT COMMANDS
// ============================================================================

/// Pause queue gracefully
#[tauri::command]
pub async fn pause_queue_gracefully(
    reason: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<QueueManagementResult, String> {
    info!("Pausing queue gracefully: {}", reason);

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.pause_queue_gracefully(reason).await {
        Ok(result) => {
            info!("Queue pause operation completed: {}", result.message);
            Ok(result)
        }
        Err(e) => {
            error!("Failed to pause queue: {}", e);
            Err(e.to_string())
        }
    }
}

/// Resume queue processing
#[tauri::command]
pub async fn resume_queue(
    reason: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<QueueManagementResult, String> {
    info!("Resuming queue: {}", reason);

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.resume_queue(reason).await {
        Ok(result) => {
            info!("Queue resume operation completed: {}", result.message);
            Ok(result)
        }
        Err(e) => {
            error!("Failed to resume queue: {}", e);
            Err(e.to_string())
        }
    }
}

/// Emergency stop queue
#[tauri::command]
pub async fn emergency_stop_queue(
    reason: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<QueueManagementResult, String> {
    warn!("Emergency stopping queue: {}", reason);

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.emergency_stop_queue(reason).await {
        Ok(result) => {
            warn!("Emergency stop completed: {}", result.message);
            Ok(result)
        }
        Err(e) => {
            error!("Failed to emergency stop queue: {}", e);
            Err(e.to_string())
        }
    }
}

/// Clear entire queue
#[tauri::command]
pub async fn clear_queue(
    reason: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<QueueManagementResult, String> {
    warn!("Clearing queue: {}", reason);

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.clear_queue(reason).await {
        Ok(result) => {
            warn!("Queue clear completed: {}", result.message);
            Ok(result)
        }
        Err(e) => {
            error!("Failed to clear queue: {}", e);
            Err(e.to_string())
        }
    }
}

/// Cancel multiple workflows
#[tauri::command]
pub async fn cancel_multiple_workflows(
    workflow_ids: Vec<String>,
    reason: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<QueueManagementResult, String> {
    info!("Cancelling {} workflows: {}", workflow_ids.len(), reason);

    // Parse workflow IDs
    let mut parsed_ids = Vec::new();
    for id_str in workflow_ids {
        match Uuid::parse_str(&id_str) {
            Ok(id) => parsed_ids.push(id),
            Err(e) => return Err(format!("Invalid workflow ID {}: {}", id_str, e)),
        }
    }

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.cancel_multiple_workflows(parsed_ids, reason).await {
        Ok(result) => {
            info!("Bulk cancel completed: {}", result.message);
            Ok(result)
        }
        Err(e) => {
            error!("Failed to cancel multiple workflows: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get queue management status
#[tauri::command]
pub async fn get_queue_management_status(
    service_manager: State<'_, ServiceManager>,
) -> Result<QueueManagementStatus, String> {
    info!("Getting queue management status");

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.get_queue_management_status().await {
        Ok(status) => {
            info!("Retrieved queue management status");
            Ok(status)
        }
        Err(e) => {
            error!("Failed to get queue management status: {}", e);
            Err(e.to_string())
        }
    }
}

// ============================================================================
// RESOURCE MANAGEMENT COMMANDS
// ============================================================================

/// Get current resource status
#[tauri::command]
pub async fn get_resource_status(
    service_manager: State<'_, ServiceManager>,
) -> Result<ResourceStatus, String> {
    info!("Getting resource status");

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.get_resource_status().await {
        Ok(status) => {
            info!("Retrieved resource status");
            Ok(status)
        }
        Err(e) => {
            error!("Failed to get resource status: {}", e);
            Err(e.to_string())
        }
    }
}

/// Update resource limits
#[tauri::command]
pub async fn update_resource_limits(
    limits: ResourceLimits,
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Updating resource limits");

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.update_resource_limits(limits).await {
        Ok(()) => {
            info!("Resource limits updated successfully");
            Ok(())
        }
        Err(e) => {
            error!("Failed to update resource limits: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get resource metrics for analytics
#[tauri::command]
pub async fn get_resource_metrics(
    hours: Option<u32>,
    service_manager: State<'_, ServiceManager>,
) -> Result<ResourceMetrics, String> {
    let hours_str = hours.map(|h| h.to_string()).unwrap_or_else(|| "24".to_string());
    info!("Getting resource metrics for last {} hours", hours_str);

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.get_resource_metrics(hours).await {
        Ok(metrics) => {
            info!("Retrieved resource metrics");
            Ok(metrics)
        }
        Err(e) => {
            error!("Failed to get resource metrics: {}", e);
            Err(e.to_string())
        }
    }
}

/// Check if resources are available for a workflow
#[tauri::command]
pub async fn can_allocate_workflow_resources(
    requirements: ResourceLimits,
    service_manager: State<'_, ServiceManager>,
) -> Result<bool, String> {
    info!("Checking resource availability");

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.can_allocate_workflow_resources(&requirements).await {
        Ok(can_allocate) => {
            info!("Resource availability check completed: {}", can_allocate);
            Ok(can_allocate)
        }
        Err(e) => {
            error!("Failed to check resource availability: {}", e);
            Err(e.to_string())
        }
    }
}

/// Record current resource usage
#[tauri::command]
pub async fn record_resource_usage(
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    debug!("Recording resource usage");

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.record_resource_usage().await {
        Ok(()) => {
            debug!("Resource usage recorded");
            Ok(())
        }
        Err(e) => {
            error!("Failed to record resource usage: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get comprehensive resource dashboard data
#[tauri::command]
pub async fn get_resource_dashboard_data(
    service_manager: State<'_, ServiceManager>,
) -> Result<serde_json::Value, String> {
    info!("Getting comprehensive resource dashboard data");

    let research_engine = service_manager.inner().research_engine.read().await;

    // Get all resource data in parallel
    let resource_status_result = research_engine.get_resource_status().await;
    let resource_metrics_result = research_engine.get_resource_metrics(Some(24)).await;
    let queue_stats_result = research_engine.get_queue_statistics().await;

    match (resource_status_result, resource_metrics_result, queue_stats_result) {
        (Ok(resource_status), Ok(resource_metrics), Ok(queue_stats)) => {
            let dashboard_data = serde_json::json!({
                "resource_status": resource_status,
                "resource_metrics": resource_metrics,
                "queue_stats": queue_stats,
                "timestamp": chrono::Utc::now(),
                "system_health": if resource_status.is_over_limit { "warning" } else { "healthy" }
            });

            info!("Retrieved comprehensive resource dashboard data");
            Ok(dashboard_data)
        }
        _ => {
            error!("Failed to retrieve some resource dashboard data components");
            Err("Failed to retrieve complete resource dashboard data".to_string())
        }
    }
}
