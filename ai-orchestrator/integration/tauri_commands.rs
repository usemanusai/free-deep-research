// Tauri Commands for BMAD-Research Integration
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};
use uuid::Uuid;

use crate::error::AppResult;
use super::{
    BMadResearchIntegrationService,
    DocumentationModeRequest,
    DocumentationModeResponse,
    DevelopmentModeRequest,
    DevelopmentModeResponse,
    IntegrationHealthStatus,
};
use super::research_bridge::{
    BMadResearchRequest,
    BMadResearchResponse,
    BMadAgentTask,
    EnhancedBMadTask,
    ResearchPhase,
    AgentResearchConfig,
};

/// Execute research-enhanced documentation mode
#[tauri::command]
pub async fn execute_research_enhanced_documentation_mode(
    request: DocumentationModeRequest,
    state: tauri::State<'_, Arc<RwLock<BMadResearchIntegrationService>>>,
) -> Result<DocumentationModeResponse, String> {
    info!("Tauri command: execute_research_enhanced_documentation_mode");
    
    let integration_service = state.read().await;
    match integration_service.execute_research_enhanced_documentation_mode(request).await {
        Ok(response) => {
            info!("Documentation mode completed successfully");
            Ok(response)
        }
        Err(e) => {
            error!("Documentation mode failed: {}", e);
            Err(format!("Documentation mode execution failed: {}", e))
        }
    }
}

/// Execute research-enhanced development mode
#[tauri::command]
pub async fn execute_research_enhanced_development_mode(
    request: DevelopmentModeRequest,
    state: tauri::State<'_, Arc<RwLock<BMadResearchIntegrationService>>>,
) -> Result<DevelopmentModeResponse, String> {
    info!("Tauri command: execute_research_enhanced_development_mode");
    
    let integration_service = state.read().await;
    match integration_service.execute_research_enhanced_development_mode(request).await {
        Ok(response) => {
            info!("Development mode initialized successfully");
            Ok(response)
        }
        Err(e) => {
            error!("Development mode failed: {}", e);
            Err(format!("Development mode execution failed: {}", e))
        }
    }
}

/// Get integration health status
#[tauri::command]
pub async fn get_integration_health_status(
    state: tauri::State<'_, Arc<RwLock<BMadResearchIntegrationService>>>,
) -> Result<IntegrationHealthStatus, String> {
    info!("Tauri command: get_integration_health_status");
    
    let integration_service = state.read().await;
    match integration_service.health_check().await {
        Ok(status) => {
            info!("Health check completed: {}", status.overall_status);
            Ok(status)
        }
        Err(e) => {
            error!("Health check failed: {}", e);
            Err(format!("Health check failed: {}", e))
        }
    }
}

/// Conduct agent research
#[tauri::command]
pub async fn bmad_conduct_agent_research(
    request: BMadResearchRequest,
    state: tauri::State<'_, Arc<RwLock<BMadResearchIntegrationService>>>,
) -> Result<BMadResearchResponse, String> {
    info!("Tauri command: bmad_conduct_agent_research for agent: {}", request.agent_id);
    
    let integration_service = state.read().await;
    match integration_service.conduct_agent_research(request).await {
        Ok(response) => {
            info!("Agent research completed: {}", response.research_id);
            Ok(response)
        }
        Err(e) => {
            error!("Agent research failed: {}", e);
            Err(format!("Agent research failed: {}", e))
        }
    }
}

/// Get research status
#[tauri::command]
pub async fn bmad_get_research_status(
    research_id: String,
    state: tauri::State<'_, Arc<RwLock<BMadResearchIntegrationService>>>,
) -> Result<Option<BMadResearchResponse>, String> {
    info!("Tauri command: bmad_get_research_status for ID: {}", research_id);
    
    let research_uuid = Uuid::parse_str(&research_id)
        .map_err(|e| format!("Invalid research ID format: {}", e))?;
    
    let integration_service = state.read().await;
    match integration_service.get_research_status(research_uuid).await {
        Ok(status) => {
            info!("Research status retrieved for ID: {}", research_id);
            Ok(status)
        }
        Err(e) => {
            error!("Failed to get research status: {}", e);
            Err(format!("Failed to get research status: {}", e))
        }
    }
}

/// Enhance agent task with research
#[tauri::command]
pub async fn bmad_enhance_agent_task(
    task: BMadAgentTask,
    research_config: Option<ResearchPhase>,
    state: tauri::State<'_, Arc<RwLock<BMadResearchIntegrationService>>>,
) -> Result<EnhancedBMadTask, String> {
    info!("Tauri command: bmad_enhance_agent_task for agent: {}", task.agent_id);
    
    let integration_service = state.read().await;
    match integration_service.enhance_agent_task(task, research_config).await {
        Ok(enhanced_task) => {
            info!("Agent task enhanced successfully");
            Ok(enhanced_task)
        }
        Err(e) => {
            error!("Agent task enhancement failed: {}", e);
            Err(format!("Agent task enhancement failed: {}", e))
        }
    }
}

/// Register agent research configuration
#[tauri::command]
pub async fn bmad_register_agent_config(
    config: AgentResearchConfig,
    state: tauri::State<'_, Arc<RwLock<BMadResearchIntegrationService>>>,
) -> Result<(), String> {
    info!("Tauri command: bmad_register_agent_config for agent: {}", config.agent_id);
    
    let integration_service = state.read().await;
    match integration_service.register_agent_config(config).await {
        Ok(_) => {
            info!("Agent configuration registered successfully");
            Ok(())
        }
        Err(e) => {
            error!("Agent configuration registration failed: {}", e);
            Err(format!("Agent configuration registration failed: {}", e))
        }
    }
}

/// Get available research methodologies
#[tauri::command]
pub async fn bmad_get_research_methodologies() -> Result<Vec<String>, String> {
    info!("Tauri command: bmad_get_research_methodologies");
    
    Ok(vec![
        "don_lim".to_string(),
        "nick_scamara".to_string(),
        "hybrid".to_string(),
        "comprehensive".to_string(),
    ])
}

/// Get available research types
#[tauri::command]
pub async fn bmad_get_research_types() -> Result<Vec<String>, String> {
    info!("Tauri command: bmad_get_research_types");
    
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

/// Get available agents with research capabilities
#[tauri::command]
pub async fn bmad_get_research_enabled_agents() -> Result<Vec<AgentInfo>, String> {
    info!("Tauri command: bmad_get_research_enabled_agents");
    
    Ok(vec![
        AgentInfo {
            id: "product-manager".to_string(),
            name: "John".to_string(),
            title: "Product Manager".to_string(),
            description: "Research-powered product management specialist with market analysis and competitive intelligence capabilities".to_string(),
            research_capabilities: vec![
                "MarketAnalysis".to_string(),
                "CompetitiveResearch".to_string(),
                "UserResearch".to_string(),
            ],
            available_tasks: vec![
                "create-prd".to_string(),
                "analyze-requirements".to_string(),
                "stakeholder-interview".to_string(),
            ],
        },
        AgentInfo {
            id: "architect".to_string(),
            name: "Fred".to_string(),
            title: "Technical Architect".to_string(),
            description: "Research-enhanced technical architect with technology evaluation and pattern analysis capabilities".to_string(),
            research_capabilities: vec![
                "TechnologyEvaluation".to_string(),
                "ArchitecturePatterns".to_string(),
                "SecurityResearch".to_string(),
            ],
            available_tasks: vec![
                "create-architecture".to_string(),
                "technology-assessment".to_string(),
                "security-review".to_string(),
            ],
        },
        AgentInfo {
            id: "platform-engineer".to_string(),
            name: "Alex".to_string(),
            title: "Platform Engineer".to_string(),
            description: "Research-powered platform engineer with infrastructure analysis and DevOps best practices research".to_string(),
            research_capabilities: vec![
                "InfrastructureResearch".to_string(),
                "SecurityResearch".to_string(),
                "TechnologyEvaluation".to_string(),
            ],
            available_tasks: vec![
                "infrastructure-design".to_string(),
                "cicd-pipeline".to_string(),
                "security-compliance".to_string(),
            ],
        },
    ])
}

/// Agent information structure
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AgentInfo {
    pub id: String,
    pub name: String,
    pub title: String,
    pub description: String,
    pub research_capabilities: Vec<String>,
    pub available_tasks: Vec<String>,
}

/// Get integration configuration
#[tauri::command]
pub async fn bmad_get_integration_config() -> Result<IntegrationConfigInfo, String> {
    info!("Tauri command: bmad_get_integration_config");
    
    Ok(IntegrationConfigInfo {
        version: "2.1.0".to_string(),
        integration_enabled: true,
        research_timeout_minutes: 45,
        max_concurrent_research: 3,
        cost_limit_per_session: 25.0,
        quality_threshold: 0.75,
        auto_research_enabled: true,
        supported_methodologies: vec![
            "don_lim".to_string(),
            "nick_scamara".to_string(),
            "hybrid".to_string(),
            "comprehensive".to_string(),
        ],
        supported_research_types: vec![
            "MarketAnalysis".to_string(),
            "CompetitiveResearch".to_string(),
            "TechnologyEvaluation".to_string(),
            "ArchitecturePatterns".to_string(),
            "SecurityResearch".to_string(),
            "InfrastructureResearch".to_string(),
            "UserResearch".to_string(),
            "ComplianceResearch".to_string(),
        ],
    })
}

/// Integration configuration information
#[derive(serde::Serialize, serde::Deserialize)]
pub struct IntegrationConfigInfo {
    pub version: String,
    pub integration_enabled: bool,
    pub research_timeout_minutes: u32,
    pub max_concurrent_research: u32,
    pub cost_limit_per_session: f64,
    pub quality_threshold: f64,
    pub auto_research_enabled: bool,
    pub supported_methodologies: Vec<String>,
    pub supported_research_types: Vec<String>,
}

/// Validate research request
#[tauri::command]
pub async fn bmad_validate_research_request(
    request: BMadResearchRequest,
) -> Result<ValidationResult, String> {
    info!("Tauri command: bmad_validate_research_request");
    
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    
    // Validate required fields
    if request.agent_id.is_empty() {
        errors.push("Agent ID is required".to_string());
    }
    
    if request.query.is_empty() {
        errors.push("Research query is required".to_string());
    }
    
    if request.query.len() < 10 {
        warnings.push("Research query is very short, consider providing more detail".to_string());
    }
    
    if request.focus_areas.is_empty() {
        warnings.push("No focus areas specified, research may be too broad".to_string());
    }
    
    if request.max_duration_minutes > 60 {
        warnings.push("Research duration over 60 minutes may be expensive".to_string());
    }
    
    if let Some(cost_limit) = request.cost_limit {
        if cost_limit > 10.0 {
            warnings.push("Cost limit is high, consider reducing for cost optimization".to_string());
        }
    }
    
    let is_valid = errors.is_empty();
    
    Ok(ValidationResult {
        is_valid,
        errors,
        warnings,
        estimated_cost: estimate_research_cost(&request),
        estimated_duration_minutes: request.max_duration_minutes,
    })
}

/// Validation result structure
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub estimated_cost: f64,
    pub estimated_duration_minutes: u32,
}

/// Estimate research cost based on request parameters
fn estimate_research_cost(request: &BMadResearchRequest) -> f64 {
    let base_cost = 1.0;
    let duration_multiplier = request.max_duration_minutes as f64 / 30.0;
    let depth_multiplier = match request.depth {
        super::research_bridge::ResearchDepth::Basic => 0.5,
        super::research_bridge::ResearchDepth::Standard => 1.0,
        super::research_bridge::ResearchDepth::Comprehensive => 1.5,
        super::research_bridge::ResearchDepth::Expert => 2.0,
    };
    let focus_multiplier = 1.0 + (request.focus_areas.len() as f64 * 0.2);
    
    base_cost * duration_multiplier * depth_multiplier * focus_multiplier
}

/// Get research cost estimate
#[tauri::command]
pub async fn bmad_get_research_cost_estimate(
    request: BMadResearchRequest,
) -> Result<CostEstimate, String> {
    info!("Tauri command: bmad_get_research_cost_estimate");
    
    let estimated_cost = estimate_research_cost(&request);
    
    Ok(CostEstimate {
        estimated_total_cost: estimated_cost,
        cost_breakdown: CostBreakdownEstimate {
            base_cost: 1.0,
            duration_cost: (request.max_duration_minutes as f64 / 30.0) * 0.5,
            depth_cost: match request.depth {
                super::research_bridge::ResearchDepth::Basic => 0.0,
                super::research_bridge::ResearchDepth::Standard => 0.5,
                super::research_bridge::ResearchDepth::Comprehensive => 1.0,
                super::research_bridge::ResearchDepth::Expert => 2.0,
            },
            focus_cost: request.focus_areas.len() as f64 * 0.2,
        },
        cost_optimization_suggestions: generate_cost_optimization_suggestions(&request),
    })
}

/// Cost estimate structure
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CostEstimate {
    pub estimated_total_cost: f64,
    pub cost_breakdown: CostBreakdownEstimate,
    pub cost_optimization_suggestions: Vec<String>,
}

/// Cost breakdown estimate
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CostBreakdownEstimate {
    pub base_cost: f64,
    pub duration_cost: f64,
    pub depth_cost: f64,
    pub focus_cost: f64,
}

/// Generate cost optimization suggestions
fn generate_cost_optimization_suggestions(request: &BMadResearchRequest) -> Vec<String> {
    let mut suggestions = Vec::new();
    
    if request.max_duration_minutes > 45 {
        suggestions.push("Consider reducing research duration to 30-45 minutes for cost optimization".to_string());
    }
    
    if request.focus_areas.len() > 5 {
        suggestions.push("Consider reducing focus areas to 3-5 key areas for better cost efficiency".to_string());
    }
    
    match request.depth {
        super::research_bridge::ResearchDepth::Expert => {
            suggestions.push("Expert depth research is expensive - consider Comprehensive depth if sufficient".to_string());
        }
        _ => {}
    }
    
    if request.cost_limit.is_none() {
        suggestions.push("Set a cost limit to prevent unexpected charges".to_string());
    }
    
    suggestions
}
