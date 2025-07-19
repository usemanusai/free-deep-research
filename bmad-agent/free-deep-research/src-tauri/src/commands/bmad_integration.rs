use tauri::State;
use tracing::{info, error};

use crate::services::ServiceManager;
use crate::services::bmad_integration::{
    BMadResearchRequest, BMadResearchResponse, DocumentationModeRequest, 
    DocumentationModeResponse, IntegrationHealthStatus
};

/// Execute research-enhanced documentation mode
#[tauri::command]
pub async fn execute_research_enhanced_documentation_mode(
    request: DocumentationModeRequest,
    service_manager: State<'_, ServiceManager>,
) -> Result<DocumentationModeResponse, String> {
    info!("API: Executing research-enhanced documentation mode");
    
    let bmad_integration = service_manager.bmad_integration.read().await;
    match bmad_integration.execute_research_enhanced_documentation_mode(request).await {
        Ok(response) => {
            info!("Research-enhanced documentation mode completed successfully");
            Ok(response)
        }
        Err(e) => {
            error!("Research-enhanced documentation mode failed: {}", e);
            Err(e.to_string())
        }
    }
}

/// Conduct agent research
#[tauri::command]
pub async fn conduct_agent_research(
    request: BMadResearchRequest,
    service_manager: State<'_, ServiceManager>,
) -> Result<BMadResearchResponse, String> {
    info!("API: Conducting agent research for agent: {}", request.agent_id);
    
    let bmad_integration = service_manager.bmad_integration.read().await;
    match bmad_integration.conduct_agent_research(request).await {
        Ok(response) => {
            info!("Agent research completed successfully: {}", response.research_id);
            Ok(response)
        }
        Err(e) => {
            error!("Agent research failed: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get integration health status
#[tauri::command]
pub async fn get_integration_health_status(
    service_manager: State<'_, ServiceManager>,
) -> Result<IntegrationHealthStatus, String> {
    info!("API: Getting BMAD integration health status");
    
    let bmad_integration = service_manager.bmad_integration.read().await;
    match bmad_integration.health_check().await {
        Ok(status) => {
            info!("Integration health check completed: {}", status.overall_status);
            Ok(status)
        }
        Err(e) => {
            error!("Integration health check failed: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get research methodologies
#[tauri::command]
pub async fn get_research_methodologies() -> Result<Vec<String>, String> {
    info!("API: Getting available research methodologies");
    
    Ok(vec![
        "DonLim".to_string(),
        "NickScamara".to_string(),
        "Hybrid".to_string(),
        "Comprehensive".to_string(),
    ])
}

/// Get research types
#[tauri::command]
pub async fn get_research_types() -> Result<Vec<String>, String> {
    info!("API: Getting available research types");
    
    Ok(vec![
        "MarketAnalysis".to_string(),
        "CompetitiveResearch".to_string(),
        "TechnologyEvaluation".to_string(),
        "ArchitecturePatterns".to_string(),
        "SecurityResearch".to_string(),
        "InfrastructureResearch".to_string(),
        "UserResearch".to_string(),
        "ComplianceResearch".to_string(),
    ])
}

/// Get research depth levels
#[tauri::command]
pub async fn get_research_depth_levels() -> Result<Vec<String>, String> {
    info!("API: Getting available research depth levels");
    
    Ok(vec![
        "Basic".to_string(),
        "Standard".to_string(),
        "Comprehensive".to_string(),
        "Expert".to_string(),
    ])
}

/// Get integration configuration
#[tauri::command]
pub async fn get_integration_config(
    service_manager: State<'_, ServiceManager>,
) -> Result<serde_json::Value, String> {
    info!("API: Getting BMAD integration configuration");
    
    let bmad_integration = service_manager.bmad_integration.read().await;
    let health_status = bmad_integration.health_check().await
        .map_err(|e| e.to_string())?;
    
    Ok(serde_json::json!({
        "version": "2.1.0",
        "integration_enabled": health_status.integration_enabled,
        "research_timeout_minutes": 45,
        "max_concurrent_research": 3,
        "cost_limit_per_session": 25.0,
        "quality_threshold": 0.75,
        "auto_research_enabled": true,
        "supported_methodologies": [
            "DonLim",
            "NickScamara", 
            "Hybrid",
            "Comprehensive"
        ],
        "supported_research_types": [
            "MarketAnalysis",
            "CompetitiveResearch",
            "TechnologyEvaluation",
            "ArchitecturePatterns",
            "SecurityResearch",
            "InfrastructureResearch",
            "UserResearch",
            "ComplianceResearch"
        ],
        "supported_depth_levels": [
            "Basic",
            "Standard",
            "Comprehensive",
            "Expert"
        ]
    }))
}

/// Test BMAD integration connectivity
#[tauri::command]
pub async fn test_bmad_integration(
    service_manager: State<'_, ServiceManager>,
) -> Result<serde_json::Value, String> {
    info!("API: Testing BMAD integration connectivity");
    
    let bmad_integration = service_manager.bmad_integration.read().await;
    let health_status = bmad_integration.health_check().await
        .map_err(|e| e.to_string())?;
    
    let test_successful = health_status.overall_status == "healthy";
    
    Ok(serde_json::json!({
        "test_successful": test_successful,
        "overall_status": health_status.overall_status,
        "research_engine_status": health_status.research_engine_status,
        "ai_orchestration_status": health_status.ai_orchestration_status,
        "api_manager_status": health_status.api_manager_status,
        "integration_enabled": health_status.integration_enabled,
        "active_research_count": health_status.active_research_count,
        "error_messages": health_status.error_messages,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Get BMAD agent information
#[tauri::command]
pub async fn get_bmad_agents() -> Result<Vec<serde_json::Value>, String> {
    info!("API: Getting BMAD agent information");
    
    Ok(vec![
        serde_json::json!({
            "id": "product-manager",
            "name": "Product Manager",
            "title": "John - Strategic Product Manager",
            "description": "Expert in market analysis, product strategy, and user research",
            "research_capabilities": [
                "MarketAnalysis",
                "CompetitiveResearch", 
                "UserResearch"
            ],
            "available_tasks": [
                "Create PRD",
                "Market Research",
                "Competitive Analysis",
                "User Story Creation"
            ]
        }),
        serde_json::json!({
            "id": "architect",
            "name": "Technical Architect", 
            "title": "Fred - Senior Technical Architect",
            "description": "Expert in system architecture, technology evaluation, and security",
            "research_capabilities": [
                "TechnologyEvaluation",
                "ArchitecturePatterns",
                "SecurityResearch",
                "InfrastructureResearch"
            ],
            "available_tasks": [
                "Architecture Design",
                "Technology Evaluation",
                "Security Assessment",
                "Infrastructure Planning"
            ]
        }),
        serde_json::json!({
            "id": "platform-engineer",
            "name": "Platform Engineer",
            "title": "Tyler - DevOps Platform Engineer", 
            "description": "Expert in infrastructure, deployment, and operational excellence",
            "research_capabilities": [
                "InfrastructureResearch",
                "TechnologyEvaluation",
                "ComplianceResearch"
            ],
            "available_tasks": [
                "Infrastructure Setup",
                "CI/CD Pipeline",
                "Monitoring Setup",
                "Compliance Validation"
            ]
        })
    ])
}

/// Get integration statistics
#[tauri::command]
pub async fn get_integration_statistics(
    service_manager: State<'_, ServiceManager>,
) -> Result<serde_json::Value, String> {
    info!("API: Getting BMAD integration statistics");
    
    let bmad_integration = service_manager.bmad_integration.read().await;
    let health_status = bmad_integration.health_check().await
        .map_err(|e| e.to_string())?;
    
    Ok(serde_json::json!({
        "service_status": health_status.overall_status,
        "active_research_count": health_status.active_research_count,
        "integration_enabled": health_status.integration_enabled,
        "error_count": health_status.error_messages.len(),
        "uptime_seconds": 0, // TODO: Implement uptime tracking
        "total_research_conducted": 0, // TODO: Implement research tracking
        "average_research_duration_minutes": 0.0, // TODO: Implement duration tracking
        "success_rate": 100.0, // TODO: Implement success rate tracking
        "last_health_check": chrono::Utc::now().to_rfc3339()
    }))
}
