// Agent Enhancement Service - Enhances BMAD agents with research capabilities
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, warn};
use uuid::Uuid;
use chrono::Utc;
use serde::{Serialize, Deserialize};

use crate::error::AppResult;
use super::research_bridge::{
    ResearchBridgeService, BMadAgentTask, EnhancedBMadTask, ResearchPhase,
    BMadResearchRequest, ResearchType, ResearchMethodology, ResearchDepth,
    AgentResearchConfig, AutoResearchTrigger, CostLimits, QualityRequirements,
};
use super::IntegrationConfig;

/// Agent Enhancement Service
pub struct AgentEnhancementService {
    research_bridge: Arc<RwLock<ResearchBridgeService>>,
    agent_configs: Arc<RwLock<HashMap<String, AgentResearchConfig>>>,
    enhancement_templates: Arc<RwLock<HashMap<String, EnhancementTemplate>>>,
    integration_config: IntegrationConfig,
}

/// Enhancement template for specific agent types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancementTemplate {
    pub agent_type: String,
    pub default_research_types: Vec<ResearchType>,
    pub task_enhancements: HashMap<String, TaskEnhancement>,
    pub quality_requirements: QualityRequirements,
    pub cost_optimization: CostOptimization,
}

/// Task-specific enhancement configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEnhancement {
    pub research_required: bool,
    pub research_types: Vec<ResearchType>,
    pub methodology: ResearchMethodology,
    pub depth: ResearchDepth,
    pub focus_areas: Vec<String>,
    pub max_duration_minutes: u32,
    pub quality_gates: Vec<String>,
}

/// Cost optimization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostOptimization {
    pub enable_caching: bool,
    pub research_deduplication: bool,
    pub smart_api_selection: bool,
    pub cost_threshold_warnings: bool,
    pub auto_cost_optimization: bool,
}

impl AgentEnhancementService {
    /// Create new agent enhancement service
    pub async fn new(
        research_bridge: Arc<RwLock<ResearchBridgeService>>,
        integration_config: IntegrationConfig,
    ) -> AppResult<Self> {
        info!("Initializing Agent Enhancement Service");

        let service = Self {
            research_bridge,
            agent_configs: Arc::new(RwLock::new(HashMap::new())),
            enhancement_templates: Arc::new(RwLock::new(HashMap::new())),
            integration_config,
        };

        // Initialize default enhancement templates
        service.initialize_default_templates().await?;

        Ok(service)
    }

    /// Enhance a BMAD agent task with research capabilities
    pub async fn enhance_task(
        &self,
        task: BMadAgentTask,
        research_config: Option<ResearchPhase>,
    ) -> AppResult<EnhancedBMadTask> {
        info!("Enhancing task for agent: {} - {}", task.agent_id, task.task_type);

        // Get agent configuration
        let agent_config = self.get_or_create_agent_config(&task.agent_id).await?;

        // Get enhancement template
        let template = self.get_enhancement_template(&task.agent_id).await?;

        // Determine research phase
        let research_phase = if let Some(config) = research_config {
            Some(config)
        } else {
            self.create_auto_research_phase(&task, &agent_config, &template).await?
        };

        // Create enhanced task
        let enhanced_task = EnhancedBMadTask {
            base_task: task.clone(),
            research_phase: research_phase.clone(),
            execution_phase: super::research_bridge::ExecutionPhase {
                research_integration: research_phase.is_some(),
                evidence_requirements: self.generate_evidence_requirements(&task, &template).await?,
                validation_criteria: self.generate_validation_criteria(&task, &template).await?,
                quality_gates: self.generate_quality_gates(&task, &template).await?,
            },
        };

        debug!("Task enhanced successfully: {}", task.task_id);
        Ok(enhanced_task)
    }

    /// Register agent research configuration
    pub async fn register_agent_config(&self, config: AgentResearchConfig) -> AppResult<()> {
        info!("Registering agent config: {}", config.agent_id);
        
        let mut agent_configs = self.agent_configs.write().await;
        agent_configs.insert(config.agent_id.clone(), config);
        
        Ok(())
    }

    /// Get agent configuration, creating default if not exists
    async fn get_or_create_agent_config(&self, agent_id: &str) -> AppResult<AgentResearchConfig> {
        let agent_configs = self.agent_configs.read().await;
        
        if let Some(config) = agent_configs.get(agent_id) {
            Ok(config.clone())
        } else {
            drop(agent_configs);
            self.create_default_agent_config(agent_id).await
        }
    }

    /// Create default agent configuration
    async fn create_default_agent_config(&self, agent_id: &str) -> AppResult<AgentResearchConfig> {
        info!("Creating default config for agent: {}", agent_id);

        let config = match agent_id {
            "product-manager" => AgentResearchConfig {
                agent_id: agent_id.to_string(),
                default_methodology: ResearchMethodology::Hybrid,
                research_capabilities: vec![
                    ResearchType::MarketAnalysis,
                    ResearchType::CompetitiveResearch,
                    ResearchType::UserResearch,
                ],
                auto_research_triggers: vec![
                    AutoResearchTrigger {
                        task_type: "create-prd".to_string(),
                        research_type: ResearchType::MarketAnalysis,
                        depth: ResearchDepth::Comprehensive,
                        conditions: vec!["new_product".to_string()],
                    },
                ],
                cost_limits: CostLimits {
                    max_per_research: 3.0,
                    max_per_task: 8.0,
                    max_daily: 25.0,
                },
                quality_requirements: QualityRequirements {
                    min_confidence_score: 0.75,
                    min_sources: 5,
                    min_evidence_items: 8,
                    required_source_diversity: 0.6,
                },
            },

            "architect" => AgentResearchConfig {
                agent_id: agent_id.to_string(),
                default_methodology: ResearchMethodology::NickScamara,
                research_capabilities: vec![
                    ResearchType::TechnologyEvaluation,
                    ResearchType::ArchitecturePatterns,
                    ResearchType::SecurityResearch,
                ],
                auto_research_triggers: vec![
                    AutoResearchTrigger {
                        task_type: "create-architecture".to_string(),
                        research_type: ResearchType::TechnologyEvaluation,
                        depth: ResearchDepth::Comprehensive,
                        conditions: vec!["new_technology".to_string()],
                    },
                ],
                cost_limits: CostLimits {
                    max_per_research: 4.0,
                    max_per_task: 12.0,
                    max_daily: 35.0,
                },
                quality_requirements: QualityRequirements {
                    min_confidence_score: 0.80,
                    min_sources: 6,
                    min_evidence_items: 10,
                    required_source_diversity: 0.7,
                },
            },

            "platform-engineer" => AgentResearchConfig {
                agent_id: agent_id.to_string(),
                default_methodology: ResearchMethodology::Comprehensive,
                research_capabilities: vec![
                    ResearchType::InfrastructureResearch,
                    ResearchType::SecurityResearch,
                    ResearchType::TechnologyEvaluation,
                ],
                auto_research_triggers: vec![
                    AutoResearchTrigger {
                        task_type: "infrastructure-design".to_string(),
                        research_type: ResearchType::InfrastructureResearch,
                        depth: ResearchDepth::Comprehensive,
                        conditions: vec!["new_infrastructure".to_string()],
                    },
                ],
                cost_limits: CostLimits {
                    max_per_research: 3.5,
                    max_per_task: 10.0,
                    max_daily: 30.0,
                },
                quality_requirements: QualityRequirements {
                    min_confidence_score: 0.75,
                    min_sources: 5,
                    min_evidence_items: 8,
                    required_source_diversity: 0.65,
                },
            },

            _ => {
                // Generic agent configuration
                AgentResearchConfig {
                    agent_id: agent_id.to_string(),
                    default_methodology: ResearchMethodology::Hybrid,
                    research_capabilities: vec![ResearchType::MarketAnalysis],
                    auto_research_triggers: vec![],
                    cost_limits: CostLimits {
                        max_per_research: 2.0,
                        max_per_task: 5.0,
                        max_daily: 15.0,
                    },
                    quality_requirements: QualityRequirements {
                        min_confidence_score: 0.70,
                        min_sources: 3,
                        min_evidence_items: 5,
                        required_source_diversity: 0.5,
                    },
                }
            }
        };

        // Register the config
        self.register_agent_config(config.clone()).await?;

        Ok(config)
    }

    /// Create automatic research phase if applicable
    async fn create_auto_research_phase(
        &self,
        task: &BMadAgentTask,
        agent_config: &AgentResearchConfig,
        template: &EnhancementTemplate,
    ) -> AppResult<Option<ResearchPhase>> {
        if !self.integration_config.auto_research_enabled {
            return Ok(None);
        }

        // Check if task should trigger automatic research
        for trigger in &agent_config.auto_research_triggers {
            if trigger.task_type == task.task_type {
                let research_request = BMadResearchRequest {
                    agent_id: task.agent_id.clone(),
                    agent_name: agent_config.agent_id.clone(),
                    research_type: trigger.research_type.clone(),
                    query: task.description.clone(),
                    methodology: agent_config.default_methodology.clone(),
                    focus_areas: self.extract_focus_areas(task, template).await?,
                    depth: trigger.depth.clone(),
                    max_duration_minutes: 30,
                    cost_limit: Some(agent_config.cost_limits.max_per_research),
                };

                return Ok(Some(ResearchPhase {
                    enabled: true,
                    research_requests: vec![research_request],
                    synthesis_prompt: format!("Synthesize research findings for {} task", task.task_type),
                    max_total_duration_minutes: 45,
                }));
            }
        }

        // Check template-based enhancements
        if let Some(task_enhancement) = template.task_enhancements.get(&task.task_type) {
            if task_enhancement.research_required {
                let research_requests: Vec<BMadResearchRequest> = task_enhancement.research_types.iter()
                    .map(|research_type| BMadResearchRequest {
                        agent_id: task.agent_id.clone(),
                        agent_name: agent_config.agent_id.clone(),
                        research_type: research_type.clone(),
                        query: task.description.clone(),
                        methodology: task_enhancement.methodology.clone(),
                        focus_areas: task_enhancement.focus_areas.clone(),
                        depth: task_enhancement.depth.clone(),
                        max_duration_minutes: task_enhancement.max_duration_minutes,
                        cost_limit: Some(agent_config.cost_limits.max_per_research),
                    })
                    .collect();

                return Ok(Some(ResearchPhase {
                    enabled: true,
                    research_requests,
                    synthesis_prompt: format!("Synthesize research findings for {} task", task.task_type),
                    max_total_duration_minutes: task_enhancement.max_duration_minutes + 15,
                }));
            }
        }

        Ok(None)
    }

    /// Extract focus areas from task parameters
    async fn extract_focus_areas(&self, task: &BMadAgentTask, template: &EnhancementTemplate) -> AppResult<Vec<String>> {
        let mut focus_areas = Vec::new();

        // Extract from task parameters
        if let Some(areas) = task.parameters.get("focus_areas") {
            if let Some(areas_array) = areas.as_array() {
                for area in areas_array {
                    if let Some(area_str) = area.as_str() {
                        focus_areas.push(area_str.to_string());
                    }
                }
            }
        }

        // Use template defaults if no focus areas specified
        if focus_areas.is_empty() {
            if let Some(task_enhancement) = template.task_enhancements.get(&task.task_type) {
                focus_areas = task_enhancement.focus_areas.clone();
            }
        }

        // Use agent-type defaults if still empty
        if focus_areas.is_empty() {
            focus_areas = match task.agent_id.as_str() {
                "product-manager" => vec!["market_size".to_string(), "competitors".to_string(), "user_needs".to_string()],
                "architect" => vec!["performance".to_string(), "scalability".to_string(), "security".to_string()],
                "platform-engineer" => vec!["cost".to_string(), "reliability".to_string(), "scalability".to_string()],
                _ => vec!["general".to_string()],
            };
        }

        Ok(focus_areas)
    }

    /// Generate evidence requirements
    async fn generate_evidence_requirements(&self, task: &BMadAgentTask, template: &EnhancementTemplate) -> AppResult<Vec<String>> {
        let requirements = vec![
            format!("Minimum {} sources per major claim", template.quality_requirements.min_sources),
            format!("Minimum confidence score of {}", template.quality_requirements.min_confidence_score),
            format!("Source diversity of at least {}", template.quality_requirements.required_source_diversity),
            "All major recommendations must be evidence-backed".to_string(),
        ];

        Ok(requirements)
    }

    /// Generate validation criteria
    async fn generate_validation_criteria(&self, task: &BMadAgentTask, template: &EnhancementTemplate) -> AppResult<Vec<String>> {
        let criteria = vec![
            "Research findings align with task objectives".to_string(),
            "Evidence quality meets minimum standards".to_string(),
            "Source credibility has been validated".to_string(),
            "Research conclusions are actionable".to_string(),
        ];

        Ok(criteria)
    }

    /// Generate quality gates
    async fn generate_quality_gates(&self, task: &BMadAgentTask, template: &EnhancementTemplate) -> AppResult<Vec<super::research_bridge::QualityGate>> {
        let gates = vec![
            super::research_bridge::QualityGate {
                name: "Research Confidence".to_string(),
                criteria: "Overall research confidence score".to_string(),
                threshold: template.quality_requirements.min_confidence_score,
                required: true,
            },
            super::research_bridge::QualityGate {
                name: "Source Count".to_string(),
                criteria: "Number of research sources".to_string(),
                threshold: template.quality_requirements.min_sources as f64,
                required: true,
            },
            super::research_bridge::QualityGate {
                name: "Evidence Completeness".to_string(),
                criteria: "Number of evidence items".to_string(),
                threshold: template.quality_requirements.min_evidence_items as f64,
                required: true,
            },
        ];

        Ok(gates)
    }

    /// Get enhancement template for agent
    async fn get_enhancement_template(&self, agent_id: &str) -> AppResult<EnhancementTemplate> {
        let templates = self.enhancement_templates.read().await;
        
        templates.get(agent_id)
            .cloned()
            .or_else(|| templates.get("default").cloned())
            .ok_or_else(|| crate::error::ResearchError::not_found(
                format!("Enhancement template not found for agent: {}", agent_id)
            ).into())
    }

    /// Initialize default enhancement templates
    async fn initialize_default_templates(&self) -> AppResult<()> {
        info!("Initializing default enhancement templates");

        let mut templates = self.enhancement_templates.write().await;

        // Product Manager template
        templates.insert("product-manager".to_string(), EnhancementTemplate {
            agent_type: "product-manager".to_string(),
            default_research_types: vec![
                ResearchType::MarketAnalysis,
                ResearchType::CompetitiveResearch,
                ResearchType::UserResearch,
            ],
            task_enhancements: {
                let mut enhancements = HashMap::new();
                enhancements.insert("create-prd".to_string(), TaskEnhancement {
                    research_required: true,
                    research_types: vec![ResearchType::MarketAnalysis, ResearchType::CompetitiveResearch],
                    methodology: ResearchMethodology::Hybrid,
                    depth: ResearchDepth::Comprehensive,
                    focus_areas: vec!["market_size".to_string(), "competitors".to_string(), "user_needs".to_string()],
                    max_duration_minutes: 25,
                    quality_gates: vec!["market_validation".to_string(), "competitive_analysis".to_string()],
                });
                enhancements
            },
            quality_requirements: QualityRequirements {
                min_confidence_score: 0.75,
                min_sources: 5,
                min_evidence_items: 8,
                required_source_diversity: 0.6,
            },
            cost_optimization: CostOptimization {
                enable_caching: true,
                research_deduplication: true,
                smart_api_selection: true,
                cost_threshold_warnings: true,
                auto_cost_optimization: true,
            },
        });

        // Technical Architect template
        templates.insert("architect".to_string(), EnhancementTemplate {
            agent_type: "architect".to_string(),
            default_research_types: vec![
                ResearchType::TechnologyEvaluation,
                ResearchType::ArchitecturePatterns,
                ResearchType::SecurityResearch,
            ],
            task_enhancements: {
                let mut enhancements = HashMap::new();
                enhancements.insert("create-architecture".to_string(), TaskEnhancement {
                    research_required: true,
                    research_types: vec![ResearchType::TechnologyEvaluation, ResearchType::ArchitecturePatterns],
                    methodology: ResearchMethodology::NickScamara,
                    depth: ResearchDepth::Comprehensive,
                    focus_areas: vec!["performance".to_string(), "scalability".to_string(), "security".to_string()],
                    max_duration_minutes: 30,
                    quality_gates: vec!["technology_validation".to_string(), "architecture_review".to_string()],
                });
                enhancements
            },
            quality_requirements: QualityRequirements {
                min_confidence_score: 0.80,
                min_sources: 6,
                min_evidence_items: 10,
                required_source_diversity: 0.7,
            },
            cost_optimization: CostOptimization {
                enable_caching: true,
                research_deduplication: true,
                smart_api_selection: true,
                cost_threshold_warnings: true,
                auto_cost_optimization: true,
            },
        });

        // Platform Engineer template
        templates.insert("platform-engineer".to_string(), EnhancementTemplate {
            agent_type: "platform-engineer".to_string(),
            default_research_types: vec![
                ResearchType::InfrastructureResearch,
                ResearchType::SecurityResearch,
                ResearchType::TechnologyEvaluation,
            ],
            task_enhancements: {
                let mut enhancements = HashMap::new();
                enhancements.insert("infrastructure-design".to_string(), TaskEnhancement {
                    research_required: true,
                    research_types: vec![ResearchType::InfrastructureResearch],
                    methodology: ResearchMethodology::Comprehensive,
                    depth: ResearchDepth::Standard,
                    focus_areas: vec!["cost".to_string(), "performance".to_string(), "reliability".to_string()],
                    max_duration_minutes: 20,
                    quality_gates: vec!["cost_validation".to_string(), "performance_review".to_string()],
                });
                enhancements
            },
            quality_requirements: QualityRequirements {
                min_confidence_score: 0.75,
                min_sources: 5,
                min_evidence_items: 8,
                required_source_diversity: 0.65,
            },
            cost_optimization: CostOptimization {
                enable_caching: true,
                research_deduplication: true,
                smart_api_selection: true,
                cost_threshold_warnings: true,
                auto_cost_optimization: true,
            },
        });

        Ok(())
    }

    /// Health check for agent enhancer
    pub async fn health_check(&self) -> AppResult<()> {
        debug!("Performing agent enhancer health check");
        
        // Check if templates are loaded
        let templates = self.enhancement_templates.read().await;
        if templates.is_empty() {
            return Err(crate::error::ResearchError::service_unavailable(
                "No enhancement templates loaded".to_string()
            ).into());
        }

        Ok(())
    }
}
