use tauri::State;
use uuid::Uuid;
use tracing::{info, error};

use crate::error::AppResult;
use crate::models::{ResearchWorkflow, CreateWorkflowRequest};
use crate::services::ServiceManager;

/// Create a new research workflow
#[tauri::command]
pub async fn create_research_workflow(
    request: CreateWorkflowRequest,
    service_manager: State<'_, ServiceManager>,
) -> Result<ResearchWorkflow, String> {
    info!("Creating research workflow: {}", request.name);

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.create_workflow(request).await {
        Ok(workflow) => {
            info!("Research workflow created successfully: {}", workflow.id);
            Ok(workflow)
        }
        Err(e) => {
            error!("Failed to create research workflow: {}", e);
            Err(e.to_string())
        }
    }
}

/// Execute a research workflow
#[tauri::command]
pub async fn execute_research(
    workflow_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Executing research workflow: {}", workflow_id);

    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.start_workflow_execution(workflow_uuid).await {
        Ok(()) => {
            info!("Research workflow execution started: {}", workflow_id);
            Ok(())
        }
        Err(e) => {
            error!("Failed to execute research workflow {}: {}", workflow_id, e);
            Err(e.to_string())
        }
    }
}

/// Get research workflow status
#[tauri::command]
pub async fn get_research_status(
    workflow_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<ResearchWorkflow, String> {
    info!("Getting research status: {}", workflow_id);

    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.get_workflow(workflow_uuid).await {
        Ok(Some(workflow)) => {
            info!("Retrieved research workflow status: {}", workflow_id);
            Ok(workflow)
        }
        Ok(None) => {
            error!("Research workflow not found: {}", workflow_id);
            Err(format!("Workflow not found: {}", workflow_id))
        }
        Err(e) => {
            error!("Failed to get research status {}: {}", workflow_id, e);
            Err(e.to_string())
        }
    }
}

/// Cancel a research workflow
#[tauri::command]
pub async fn cancel_research(
    workflow_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Cancelling research workflow: {}", workflow_id);

    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.cancel_workflow(workflow_uuid).await {
        Ok(()) => {
            info!("Research workflow cancelled: {}", workflow_id);
            Ok(())
        }
        Err(e) => {
            error!("Failed to cancel research workflow {}: {}", workflow_id, e);
            Err(e.to_string())
        }
    }
}

/// Get research workflow results
#[tauri::command]
pub async fn get_research_results(
    workflow_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Option<crate::models::ResearchResults>, String> {
    info!("Getting research results: {}", workflow_id);

    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    let research_engine = service_manager.inner().research_engine.read().await;
    match research_engine.get_workflow_results(workflow_uuid).await {
        Ok(Some(results)) => {
            info!("Retrieved research results for workflow: {}", workflow_id);
            Ok(Some(results))
        }
        Ok(None) => {
            info!("No results found for workflow: {}", workflow_id);
            Ok(None)
        }
        Err(e) => {
            error!("Failed to get research results {}: {}", workflow_id, e);
            Err(e.to_string())
        }
    }
}
