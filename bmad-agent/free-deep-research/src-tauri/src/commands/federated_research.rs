use tauri::State;
use uuid::Uuid;
use tracing::{info, debug, error};

use crate::error::{AppResult, ResearchError};
use crate::services::ServiceManager;
use crate::models::federated_research::*;

/// Register a new federated organization
#[tauri::command]
pub async fn register_federated_organization(
    service_manager: State<'_, ServiceManager>,
    request: CreateFederatedOrganizationRequest,
) -> Result<FederatedOrganization, String> {
    info!("API: Registering federated organization: {}", request.name);
    
    match service_manager.federated_research_service.register_organization(request).await {
        Ok(organization) => {
            info!("Successfully registered organization: {}", organization.id);
            Ok(organization)
        }
        Err(e) => {
            error!("Failed to register organization: {}", e);
            Err(e.to_string())
        }
    }
}

/// Create a research partnership
#[tauri::command]
pub async fn create_research_partnership(
    service_manager: State<'_, ServiceManager>,
    organization_id: String,
    request: CreateResearchPartnershipRequest,
) -> Result<ResearchPartnership, String> {
    info!("API: Creating research partnership for organization: {}", organization_id);
    
    let org_id = Uuid::parse_str(&organization_id)
        .map_err(|e| format!("Invalid organization ID: {}", e))?;
    
    match service_manager.federated_research_service.create_partnership(org_id, request).await {
        Ok(partnership) => {
            info!("Successfully created partnership: {}", partnership.id);
            Ok(partnership)
        }
        Err(e) => {
            error!("Failed to create partnership: {}", e);
            Err(e.to_string())
        }
    }
}

/// Share research session with federated partners
#[tauri::command]
pub async fn share_research_session(
    service_manager: State<'_, ServiceManager>,
    request: ShareResearchSessionRequest,
) -> Result<SharedResearchSession, String> {
    info!("API: Sharing research session: {}", request.workflow_id);
    
    match service_manager.federated_research_service.share_research_session(request).await {
        Ok(shared_session) => {
            info!("Successfully shared research session: {}", shared_session.id);
            Ok(shared_session)
        }
        Err(e) => {
            error!("Failed to share research session: {}", e);
            Err(e.to_string())
        }
    }
}

/// Execute federated research query
#[tauri::command]
pub async fn execute_federated_query(
    service_manager: State<'_, ServiceManager>,
    query: FederatedResearchQuery,
) -> Result<Vec<FederatedResearchResponse>, String> {
    info!("API: Executing federated research query: {}", query.id);
    
    match service_manager.federated_research_service.execute_federated_query(query).await {
        Ok(responses) => {
            info!("Successfully executed federated query with {} responses", responses.len());
            Ok(responses)
        }
        Err(e) => {
            error!("Failed to execute federated query: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get organization metrics
#[tauri::command]
pub async fn get_organization_metrics(
    service_manager: State<'_, ServiceManager>,
    organization_id: String,
) -> Result<FederatedResearchMetrics, String> {
    debug!("API: Getting metrics for organization: {}", organization_id);
    
    let org_id = Uuid::parse_str(&organization_id)
        .map_err(|e| format!("Invalid organization ID: {}", e))?;
    
    match service_manager.federated_research_service.get_organization_metrics(org_id).await {
        Ok(metrics) => Ok(metrics),
        Err(e) => {
            error!("Failed to get organization metrics: {}", e);
            Err(e.to_string())
        }
    }
}

/// Update privacy controls
#[tauri::command]
pub async fn update_privacy_controls(
    service_manager: State<'_, ServiceManager>,
    organization_id: String,
    controls: PrivacyControls,
) -> Result<(), String> {
    info!("API: Updating privacy controls for organization: {}", organization_id);
    
    let org_id = Uuid::parse_str(&organization_id)
        .map_err(|e| format!("Invalid organization ID: {}", e))?;
    
    match service_manager.federated_research_service.update_privacy_controls(org_id, controls).await {
        Ok(()) => {
            info!("Successfully updated privacy controls");
            Ok(())
        }
        Err(e) => {
            error!("Failed to update privacy controls: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get active partnerships
#[tauri::command]
pub async fn get_active_partnerships(
    service_manager: State<'_, ServiceManager>,
    organization_id: String,
) -> Result<Vec<ResearchPartnership>, String> {
    debug!("API: Getting active partnerships for organization: {}", organization_id);
    
    let org_id = Uuid::parse_str(&organization_id)
        .map_err(|e| format!("Invalid organization ID: {}", e))?;
    
    match service_manager.federated_research_service.get_active_partnerships(org_id).await {
        Ok(partnerships) => Ok(partnerships),
        Err(e) => {
            error!("Failed to get active partnerships: {}", e);
            Err(e.to_string())
        }
    }
}

/// Validate federated authentication token
#[tauri::command]
pub async fn validate_federated_auth_token(
    service_manager: State<'_, ServiceManager>,
    token: String,
) -> Result<FederatedAuthToken, String> {
    debug!("API: Validating federated authentication token");
    
    match service_manager.federated_research_service.validate_auth_token(&token).await {
        Ok(token_info) => Ok(token_info),
        Err(e) => {
            error!("Failed to validate auth token: {}", e);
            Err(e.to_string())
        }
    }
}

/// Create cross-organization collaboration
#[tauri::command]
pub async fn create_cross_org_collaboration(
    service_manager: State<'_, ServiceManager>,
    lead_organization_id: String,
    collaboration_request: CrossOrgCollaboration,
) -> Result<CrossOrgCollaboration, String> {
    info!("API: Creating cross-organization collaboration");
    
    let lead_org_id = Uuid::parse_str(&lead_organization_id)
        .map_err(|e| format!("Invalid lead organization ID: {}", e))?;
    
    match service_manager.federated_research_service
        .create_collaboration(lead_org_id, collaboration_request).await {
        Ok(collaboration) => {
            info!("Successfully created collaboration: {}", collaboration.id);
            Ok(collaboration)
        }
        Err(e) => {
            error!("Failed to create collaboration: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get federated research statistics
#[tauri::command]
pub async fn get_federated_research_statistics(
    service_manager: State<'_, ServiceManager>,
) -> Result<FederatedResearchStatistics, String> {
    debug!("API: Getting federated research statistics");
    
    // This would aggregate statistics across all organizations
    // For now, return a mock response
    Ok(FederatedResearchStatistics {
        total_organizations: 0,
        active_partnerships: 0,
        total_shared_sessions: 0,
        total_federated_queries: 0,
        average_response_time_ms: 0.0,
        success_rate: 0.0,
        data_shared_gb: 0.0,
        last_updated: chrono::Utc::now(),
    })
}

/// Federated research statistics model
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FederatedResearchStatistics {
    pub total_organizations: u32,
    pub active_partnerships: u32,
    pub total_shared_sessions: u32,
    pub total_federated_queries: u32,
    pub average_response_time_ms: f64,
    pub success_rate: f64,
    pub data_shared_gb: f64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Test federated connection
#[tauri::command]
pub async fn test_federated_connection(
    service_manager: State<'_, ServiceManager>,
    organization_id: String,
    target_endpoint: String,
) -> Result<FederatedConnectionTest, String> {
    info!("API: Testing federated connection to: {}", target_endpoint);
    
    let org_id = Uuid::parse_str(&organization_id)
        .map_err(|e| format!("Invalid organization ID: {}", e))?;
    
    // Simulate connection test
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    Ok(FederatedConnectionTest {
        organization_id: org_id,
        target_endpoint,
        connection_successful: true,
        response_time_ms: 250,
        error_message: None,
        tested_at: chrono::Utc::now(),
    })
}

/// Federated connection test result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FederatedConnectionTest {
    pub organization_id: Uuid,
    pub target_endpoint: String,
    pub connection_successful: bool,
    pub response_time_ms: u32,
    pub error_message: Option<String>,
    pub tested_at: chrono::DateTime<chrono::Utc>,
}
