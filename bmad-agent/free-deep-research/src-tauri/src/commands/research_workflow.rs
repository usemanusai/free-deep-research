use tauri::State;
use tracing::{info, error};
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::research_workflow::{ResearchWorkflow, ResearchMethodology, WorkflowStatus, WorkflowParameters};
use crate::services::ServiceManager;

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
