// Service Registration for BMAD-Research Integration
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};

use crate::error::AppResult;
use crate::services::{
    ApiManagerService,
    ResearchEngineService,
    AIOrchestrationService,
    DataPersistenceService,
    MonitoringService,
};

use super::{
    BMadResearchIntegrationService,
    IntegrationConfig,
    research_bridge::ResearchBridgeService,
    agent_enhancer::AgentEnhancementService,
    workflow_coordinator::WorkflowCoordinationService,
};

/// Service registration and initialization for BMAD-Research integration
pub struct IntegrationServiceRegistry {
    integration_service: Option<Arc<RwLock<BMadResearchIntegrationService>>>,
    config: IntegrationConfig,
}

impl IntegrationServiceRegistry {
    /// Create new service registry
    pub fn new(config: Option<IntegrationConfig>) -> Self {
        Self {
            integration_service: None,
            config: config.unwrap_or_default(),
        }
    }

    /// Initialize and register all integration services
    pub async fn initialize_services(
        &mut self,
        api_manager: Arc<RwLock<ApiManagerService>>,
        research_engine: Arc<RwLock<ResearchEngineService>>,
        ai_orchestration: Arc<RwLock<AIOrchestrationService>>,
        data_persistence: Arc<RwLock<DataPersistenceService>>,
        monitoring: Arc<RwLock<MonitoringService>>,
    ) -> AppResult<()> {
        info!("Initializing BMAD-Research integration services");

        // Validate that all required services are available
        self.validate_service_dependencies(&api_manager, &research_engine, &ai_orchestration).await?;

        // Initialize the main integration service
        let integration_service = BMadResearchIntegrationService::new(
            research_engine,
            ai_orchestration,
            api_manager,
            data_persistence,
            Some(self.config.clone()),
        ).await?;

        self.integration_service = Some(Arc::new(RwLock::new(integration_service)));

        // Register default agent configurations
        self.register_default_agent_configurations().await?;

        // Perform initial health check
        self.perform_initial_health_check().await?;

        info!("BMAD-Research integration services initialized successfully");
        Ok(())
    }

    /// Get the integration service instance
    pub fn get_integration_service(&self) -> Option<Arc<RwLock<BMadResearchIntegrationService>>> {
        self.integration_service.clone()
    }

    /// Register Tauri commands for the integration
    pub fn register_tauri_commands() -> Vec<Box<dyn Fn(tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry>>> {
        vec![
            Box::new(|builder| {
                builder.invoke_handler(tauri::generate_handler![
                    super::tauri_commands::execute_research_enhanced_documentation_mode,
                    super::tauri_commands::execute_research_enhanced_development_mode,
                    super::tauri_commands::get_integration_health_status,
                    super::tauri_commands::bmad_conduct_agent_research,
                    super::tauri_commands::bmad_get_research_status,
                    super::tauri_commands::bmad_enhance_agent_task,
                    super::tauri_commands::bmad_register_agent_config,
                    super::tauri_commands::bmad_get_research_methodologies,
                    super::tauri_commands::bmad_get_research_types,
                    super::tauri_commands::bmad_get_research_enabled_agents,
                    super::tauri_commands::bmad_get_integration_config,
                    super::tauri_commands::bmad_validate_research_request,
                    super::tauri_commands::bmad_get_research_cost_estimate,
                ])
            }),
        ]
    }

    /// Validate service dependencies
    async fn validate_service_dependencies(
        &self,
        api_manager: &Arc<RwLock<ApiManagerService>>,
        research_engine: &Arc<RwLock<ResearchEngineService>>,
        ai_orchestration: &Arc<RwLock<AIOrchestrationService>>,
    ) -> AppResult<()> {
        info!("Validating service dependencies for integration");

        // Check API Manager health
        {
            let api_manager_service = api_manager.read().await;
            api_manager_service.health_check().await
                .map_err(|e| {
                    error!("API Manager service health check failed: {}", e);
                    e
                })?;
        }

        // Check Research Engine health
        {
            let research_engine_service = research_engine.read().await;
            research_engine_service.health_check().await
                .map_err(|e| {
                    error!("Research Engine service health check failed: {}", e);
                    e
                })?;
        }

        // Check AI Orchestration health
        {
            let ai_orchestration_service = ai_orchestration.read().await;
            ai_orchestration_service.health_check().await
                .map_err(|e| {
                    error!("AI Orchestration service health check failed: {}", e);
                    e
                })?;
        }

        info!("All service dependencies validated successfully");
        Ok(())
    }

    /// Register default agent configurations
    async fn register_default_agent_configurations(&self) -> AppResult<()> {
        info!("Registering default agent configurations");

        if let Some(integration_service) = &self.integration_service {
            let service = integration_service.read().await;

            // Product Manager configuration
            let pm_config = super::research_bridge::AgentResearchConfig {
                agent_id: "product-manager".to_string(),
                default_methodology: super::research_bridge::ResearchMethodology::Hybrid,
                research_capabilities: vec![
                    super::research_bridge::ResearchType::MarketAnalysis,
                    super::research_bridge::ResearchType::CompetitiveResearch,
                    super::research_bridge::ResearchType::UserResearch,
                ],
                auto_research_triggers: vec![
                    super::research_bridge::AutoResearchTrigger {
                        task_type: "create-prd".to_string(),
                        research_type: super::research_bridge::ResearchType::MarketAnalysis,
                        depth: super::research_bridge::ResearchDepth::Comprehensive,
                        conditions: vec!["new_product".to_string()],
                    },
                ],
                cost_limits: super::research_bridge::CostLimits {
                    max_per_research: 3.0,
                    max_per_task: 8.0,
                    max_daily: 25.0,
                },
                quality_requirements: super::research_bridge::QualityRequirements {
                    min_confidence_score: 0.75,
                    min_sources: 5,
                    min_evidence_items: 8,
                    required_source_diversity: 0.6,
                },
            };

            service.register_agent_config(pm_config).await?;

            // Technical Architect configuration
            let arch_config = super::research_bridge::AgentResearchConfig {
                agent_id: "architect".to_string(),
                default_methodology: super::research_bridge::ResearchMethodology::NickScamara,
                research_capabilities: vec![
                    super::research_bridge::ResearchType::TechnologyEvaluation,
                    super::research_bridge::ResearchType::ArchitecturePatterns,
                    super::research_bridge::ResearchType::SecurityResearch,
                ],
                auto_research_triggers: vec![
                    super::research_bridge::AutoResearchTrigger {
                        task_type: "create-architecture".to_string(),
                        research_type: super::research_bridge::ResearchType::TechnologyEvaluation,
                        depth: super::research_bridge::ResearchDepth::Comprehensive,
                        conditions: vec!["new_technology".to_string()],
                    },
                ],
                cost_limits: super::research_bridge::CostLimits {
                    max_per_research: 4.0,
                    max_per_task: 12.0,
                    max_daily: 35.0,
                },
                quality_requirements: super::research_bridge::QualityRequirements {
                    min_confidence_score: 0.80,
                    min_sources: 6,
                    min_evidence_items: 10,
                    required_source_diversity: 0.7,
                },
            };

            service.register_agent_config(arch_config).await?;

            // Platform Engineer configuration
            let pe_config = super::research_bridge::AgentResearchConfig {
                agent_id: "platform-engineer".to_string(),
                default_methodology: super::research_bridge::ResearchMethodology::Comprehensive,
                research_capabilities: vec![
                    super::research_bridge::ResearchType::InfrastructureResearch,
                    super::research_bridge::ResearchType::SecurityResearch,
                    super::research_bridge::ResearchType::TechnologyEvaluation,
                ],
                auto_research_triggers: vec![
                    super::research_bridge::AutoResearchTrigger {
                        task_type: "infrastructure-design".to_string(),
                        research_type: super::research_bridge::ResearchType::InfrastructureResearch,
                        depth: super::research_bridge::ResearchDepth::Standard,
                        conditions: vec!["new_infrastructure".to_string()],
                    },
                ],
                cost_limits: super::research_bridge::CostLimits {
                    max_per_research: 3.5,
                    max_per_task: 10.0,
                    max_daily: 30.0,
                },
                quality_requirements: super::research_bridge::QualityRequirements {
                    min_confidence_score: 0.75,
                    min_sources: 5,
                    min_evidence_items: 8,
                    required_source_diversity: 0.65,
                },
            };

            service.register_agent_config(pe_config).await?;

            info!("Default agent configurations registered successfully");
        }

        Ok(())
    }

    /// Perform initial health check
    async fn perform_initial_health_check(&self) -> AppResult<()> {
        info!("Performing initial integration health check");

        if let Some(integration_service) = &self.integration_service {
            let service = integration_service.read().await;
            let health_status = service.health_check().await?;

            if health_status.overall_status != "healthy" {
                error!("Integration health check failed: {}", health_status.overall_status);
                for error_msg in &health_status.error_messages {
                    error!("Health check error: {}", error_msg);
                }
                return Err(crate::error::ResearchError::service_unavailable(
                    format!("Integration service is not healthy: {}", health_status.overall_status)
                ).into());
            }

            info!("Integration health check passed: {}", health_status.overall_status);
        }

        Ok(())
    }

    /// Shutdown integration services
    pub async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down BMAD-Research integration services");

        if let Some(integration_service) = &self.integration_service {
            // Perform any necessary cleanup
            let service = integration_service.read().await;
            let health_status = service.health_check().await?;
            
            info!("Integration service shutdown completed. Final status: {}", health_status.overall_status);
        }

        Ok(())
    }

    /// Get integration statistics
    pub async fn get_integration_statistics(&self) -> AppResult<IntegrationStatistics> {
        if let Some(integration_service) = &self.integration_service {
            let service = integration_service.read().await;
            let health_status = service.health_check().await?;

            Ok(IntegrationStatistics {
                service_status: health_status.overall_status,
                active_research_count: health_status.active_research_count,
                integration_enabled: health_status.integration_enabled,
                error_count: health_status.error_messages.len() as u32,
                uptime_seconds: 0, // TODO: Implement uptime tracking
            })
        } else {
            Ok(IntegrationStatistics {
                service_status: "not_initialized".to_string(),
                active_research_count: 0,
                integration_enabled: false,
                error_count: 0,
                uptime_seconds: 0,
            })
        }
    }
}

/// Integration statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntegrationStatistics {
    pub service_status: String,
    pub active_research_count: u32,
    pub integration_enabled: bool,
    pub error_count: u32,
    pub uptime_seconds: u64,
}

/// Integration service builder for easy configuration
pub struct IntegrationServiceBuilder {
    config: IntegrationConfig,
}

impl IntegrationServiceBuilder {
    /// Create new builder with default configuration
    pub fn new() -> Self {
        Self {
            config: IntegrationConfig::default(),
        }
    }

    /// Set integration enabled/disabled
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.config.enabled = enabled;
        self
    }

    /// Set research timeout
    pub fn research_timeout_minutes(mut self, timeout: u32) -> Self {
        self.config.research_timeout_minutes = timeout;
        self
    }

    /// Set maximum concurrent research sessions
    pub fn max_concurrent_research(mut self, max_concurrent: u32) -> Self {
        self.config.max_concurrent_research = max_concurrent;
        self
    }

    /// Set cost limit per session
    pub fn cost_limit_per_session(mut self, cost_limit: f64) -> Self {
        self.config.cost_limit_per_session = cost_limit;
        self
    }

    /// Set quality threshold
    pub fn quality_threshold(mut self, threshold: f64) -> Self {
        self.config.quality_threshold = threshold;
        self
    }

    /// Enable/disable automatic research
    pub fn auto_research_enabled(mut self, enabled: bool) -> Self {
        self.config.auto_research_enabled = enabled;
        self
    }

    /// Enable/disable research result caching
    pub fn cache_research_results(mut self, cache: bool) -> Self {
        self.config.cache_research_results = cache;
        self
    }

    /// Build the service registry
    pub fn build(self) -> IntegrationServiceRegistry {
        IntegrationServiceRegistry::new(Some(self.config))
    }
}

impl Default for IntegrationServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility function to create a production-ready integration service registry
pub fn create_production_integration_registry() -> IntegrationServiceRegistry {
    IntegrationServiceBuilder::new()
        .enabled(true)
        .research_timeout_minutes(45)
        .max_concurrent_research(3)
        .cost_limit_per_session(25.0)
        .quality_threshold(0.75)
        .auto_research_enabled(true)
        .cache_research_results(true)
        .build()
}

/// Utility function to create a development integration service registry
pub fn create_development_integration_registry() -> IntegrationServiceRegistry {
    IntegrationServiceBuilder::new()
        .enabled(true)
        .research_timeout_minutes(30)
        .max_concurrent_research(2)
        .cost_limit_per_session(10.0)
        .quality_threshold(0.70)
        .auto_research_enabled(true)
        .cache_research_results(true)
        .build()
}

/// Utility function to create a testing integration service registry
pub fn create_testing_integration_registry() -> IntegrationServiceRegistry {
    IntegrationServiceBuilder::new()
        .enabled(false) // Disabled for testing to avoid API costs
        .research_timeout_minutes(15)
        .max_concurrent_research(1)
        .cost_limit_per_session(5.0)
        .quality_threshold(0.60)
        .auto_research_enabled(false)
        .cache_research_results(false)
        .build()
}
