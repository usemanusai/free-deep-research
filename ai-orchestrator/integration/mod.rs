// BMAD-Free Deep Research Integration Module
// Provides the integration layer between BMAD AI Agent Orchestrator and Free Deep Research

pub mod research_bridge;
pub mod agent_enhancer;
pub mod workflow_coordinator;

pub use research_bridge::{
    ResearchBridgeService,
    BMadResearchRequest,
    BMadResearchResponse,
    ResearchType,
    ResearchMethodology,
    ResearchDepth,
    ResearchResults,
    EnhancedBMadTask,
    AgentResearchConfig,
};

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};
use uuid::Uuid;

use crate::error::AppResult;
use crate::services::{
    ApiManagerService,
    ResearchEngineService,
    AIOrchestrationService,
    DataPersistenceService,
};

/// Main integration service that coordinates BMAD and Free Deep Research
pub struct BMadResearchIntegrationService {
    research_bridge: Arc<RwLock<ResearchBridgeService>>,
    agent_enhancer: Arc<RwLock<agent_enhancer::AgentEnhancementService>>,
    workflow_coordinator: Arc<RwLock<workflow_coordinator::WorkflowCoordinationService>>,
    integration_config: IntegrationConfig,
}

/// Integration configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntegrationConfig {
    pub enabled: bool,
    pub research_timeout_minutes: u32,
    pub max_concurrent_research: u32,
    pub cost_limit_per_session: f64,
    pub quality_threshold: f64,
    pub auto_research_enabled: bool,
    pub cache_research_results: bool,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            research_timeout_minutes: 45,
            max_concurrent_research: 3,
            cost_limit_per_session: 25.0,
            quality_threshold: 0.75,
            auto_research_enabled: true,
            cache_research_results: true,
        }
    }
}

impl BMadResearchIntegrationService {
    /// Create new integration service
    pub async fn new(
        research_engine: Arc<RwLock<ResearchEngineService>>,
        ai_orchestration: Arc<RwLock<AIOrchestrationService>>,
        api_manager: Arc<RwLock<ApiManagerService>>,
        data_persistence: Arc<RwLock<DataPersistenceService>>,
        config: Option<IntegrationConfig>,
    ) -> AppResult<Self> {
        info!("Initializing BMAD-Research Integration Service");

        let integration_config = config.unwrap_or_default();

        // Initialize research bridge
        let research_bridge = Arc::new(RwLock::new(
            ResearchBridgeService::new(
                research_engine.clone(),
                ai_orchestration.clone(),
                api_manager.clone(),
                data_persistence.clone(),
            ).await?
        ));

        // Initialize agent enhancer
        let agent_enhancer = Arc::new(RwLock::new(
            agent_enhancer::AgentEnhancementService::new(
                research_bridge.clone(),
                integration_config.clone(),
            ).await?
        ));

        // Initialize workflow coordinator
        let workflow_coordinator = Arc::new(RwLock::new(
            workflow_coordinator::WorkflowCoordinationService::new(
                research_bridge.clone(),
                agent_enhancer.clone(),
                integration_config.clone(),
            ).await?
        ));

        Ok(Self {
            research_bridge,
            agent_enhancer,
            workflow_coordinator,
            integration_config,
        })
    }

    /// Execute research-enhanced documentation mode
    pub async fn execute_research_enhanced_documentation_mode(
        &self,
        request: DocumentationModeRequest,
    ) -> AppResult<DocumentationModeResponse> {
        info!("Executing research-enhanced documentation mode");

        let workflow_coordinator = self.workflow_coordinator.read().await;
        workflow_coordinator.execute_documentation_mode(request).await
    }

    /// Execute research-enhanced development mode
    pub async fn execute_research_enhanced_development_mode(
        &self,
        request: DevelopmentModeRequest,
    ) -> AppResult<DevelopmentModeResponse> {
        info!("Executing research-enhanced development mode");

        let workflow_coordinator = self.workflow_coordinator.read().await;
        workflow_coordinator.execute_development_mode(request).await
    }

    /// Conduct agent research
    pub async fn conduct_agent_research(
        &self,
        request: BMadResearchRequest,
    ) -> AppResult<BMadResearchResponse> {
        let research_bridge = self.research_bridge.read().await;
        research_bridge.conduct_agent_research(request).await
    }

    /// Enhance agent task with research
    pub async fn enhance_agent_task(
        &self,
        task: research_bridge::BMadAgentTask,
        research_config: Option<research_bridge::ResearchPhase>,
    ) -> AppResult<EnhancedBMadTask> {
        let agent_enhancer = self.agent_enhancer.read().await;
        agent_enhancer.enhance_task(task, research_config).await
    }

    /// Register agent research configuration
    pub async fn register_agent_config(&self, config: AgentResearchConfig) -> AppResult<()> {
        let research_bridge = self.research_bridge.read().await;
        research_bridge.register_agent_config(config).await
    }

    /// Get research status
    pub async fn get_research_status(&self, research_id: Uuid) -> AppResult<Option<BMadResearchResponse>> {
        let research_bridge = self.research_bridge.read().await;
        research_bridge.get_research_status(research_id).await
    }

    /// Health check for integration service
    pub async fn health_check(&self) -> AppResult<IntegrationHealthStatus> {
        let mut status = IntegrationHealthStatus {
            overall_status: "healthy".to_string(),
            research_bridge_status: "unknown".to_string(),
            agent_enhancer_status: "unknown".to_string(),
            workflow_coordinator_status: "unknown".to_string(),
            integration_enabled: self.integration_config.enabled,
            active_research_count: 0,
            error_messages: Vec::new(),
        };

        // Check research bridge health
        match self.research_bridge.read().await.health_check().await {
            Ok(_) => status.research_bridge_status = "healthy".to_string(),
            Err(e) => {
                status.research_bridge_status = "unhealthy".to_string();
                status.error_messages.push(format!("Research bridge error: {}", e));
            }
        }

        // Check agent enhancer health
        match self.agent_enhancer.read().await.health_check().await {
            Ok(_) => status.agent_enhancer_status = "healthy".to_string(),
            Err(e) => {
                status.agent_enhancer_status = "unhealthy".to_string();
                status.error_messages.push(format!("Agent enhancer error: {}", e));
            }
        }

        // Check workflow coordinator health
        match self.workflow_coordinator.read().await.health_check().await {
            Ok(_) => status.workflow_coordinator_status = "healthy".to_string(),
            Err(e) => {
                status.workflow_coordinator_status = "unhealthy".to_string();
                status.error_messages.push(format!("Workflow coordinator error: {}", e));
            }
        }

        // Determine overall status
        if !status.error_messages.is_empty() {
            status.overall_status = "degraded".to_string();
        }

        if status.research_bridge_status == "unhealthy" || 
           status.agent_enhancer_status == "unhealthy" || 
           status.workflow_coordinator_status == "unhealthy" {
            status.overall_status = "unhealthy".to_string();
        }

        Ok(status)
    }
}

/// Documentation mode request
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DocumentationModeRequest {
    pub project_description: String,
    pub requirements: Vec<String>,
    pub target_audience: String,
    pub research_depth: ResearchDepth,
    pub cost_limit: Option<f64>,
    pub timeline_minutes: Option<u32>,
}

/// Documentation mode response
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DocumentationModeResponse {
    pub session_id: Uuid,
    pub status: String,
    pub deliverables: DocumentationDeliverables,
    pub research_summary: ResearchSummary,
    pub quality_metrics: QualityMetrics,
    pub cost_breakdown: CostBreakdown,
}

/// Documentation deliverables
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DocumentationDeliverables {
    pub prd: String,
    pub architecture: String,
    pub checklist: String,
    pub research_appendix: String,
}

/// Development mode request
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DevelopmentModeRequest {
    pub project_description: String,
    pub development_goals: Vec<String>,
    pub technology_preferences: Vec<String>,
    pub research_enabled: bool,
    pub cost_limit: Option<f64>,
}

/// Development mode response
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DevelopmentModeResponse {
    pub session_id: Uuid,
    pub status: String,
    pub active_agents: Vec<String>,
    pub research_capabilities: Vec<String>,
    pub estimated_timeline: String,
}

/// Research summary
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResearchSummary {
    pub total_research_conducted: u32,
    pub research_confidence_average: f64,
    pub sources_analyzed: u32,
    pub evidence_items_collected: u32,
    pub research_duration_minutes: u32,
}

/// Quality metrics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QualityMetrics {
    pub overall_confidence_score: f64,
    pub source_diversity_score: f64,
    pub evidence_completeness_score: f64,
    pub research_coverage_score: f64,
    pub quality_gates_passed: u32,
    pub quality_gates_total: u32,
}

/// Cost breakdown
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CostBreakdown {
    pub total_cost: f64,
    pub research_cost: f64,
    pub api_cost: f64,
    pub processing_cost: f64,
    pub cost_per_deliverable: f64,
}

/// Integration health status
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntegrationHealthStatus {
    pub overall_status: String,
    pub research_bridge_status: String,
    pub agent_enhancer_status: String,
    pub workflow_coordinator_status: String,
    pub integration_enabled: bool,
    pub active_research_count: u32,
    pub error_messages: Vec<String>,
}

/// Tauri commands for frontend integration
#[tauri::command]
pub async fn execute_research_enhanced_documentation_mode(
    request: DocumentationModeRequest,
    state: tauri::State<'_, Arc<RwLock<BMadResearchIntegrationService>>>,
) -> Result<DocumentationModeResponse, String> {
    let integration_service = state.read().await;
    integration_service.execute_research_enhanced_documentation_mode(request).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn execute_research_enhanced_development_mode(
    request: DevelopmentModeRequest,
    state: tauri::State<'_, Arc<RwLock<BMadResearchIntegrationService>>>,
) -> Result<DevelopmentModeResponse, String> {
    let integration_service = state.read().await;
    integration_service.execute_research_enhanced_development_mode(request).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_integration_health_status(
    state: tauri::State<'_, Arc<RwLock<BMadResearchIntegrationService>>>,
) -> Result<IntegrationHealthStatus, String> {
    let integration_service = state.read().await;
    integration_service.health_check().await
        .map_err(|e| e.to_string())
}
