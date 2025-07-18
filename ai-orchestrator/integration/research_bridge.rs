// Research Bridge Service - Integration layer between BMAD and Free Deep Research
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, error, warn};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

// Import types from free-deep-research system
use crate::error::{AppResult, ResearchError};
use crate::services::{
    ApiManagerService, 
    ResearchEngineService, 
    AIOrchestrationService,
    DataPersistenceService
};
use crate::models::research_workflow::{ResearchWorkflow, WorkflowStatus, ResearchStep};

/// BMAD Agent Research Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BMadResearchRequest {
    pub agent_id: String,
    pub agent_name: String,
    pub research_type: ResearchType,
    pub query: String,
    pub methodology: ResearchMethodology,
    pub focus_areas: Vec<String>,
    pub depth: ResearchDepth,
    pub max_duration_minutes: u32,
    pub cost_limit: Option<f64>,
}

/// Research types specific to BMAD agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchType {
    MarketAnalysis,
    CompetitiveResearch,
    TechnologyEvaluation,
    ArchitecturePatterns,
    SecurityResearch,
    InfrastructureResearch,
    UserResearch,
    ComplianceResearch,
}

/// Research methodologies available
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchMethodology {
    DonLim,
    NickScamara,
    Hybrid,
    Comprehensive,
}

/// Research depth levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchDepth {
    Basic,
    Standard,
    Comprehensive,
    Expert,
}

/// BMAD Agent Research Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BMadResearchResponse {
    pub research_id: Uuid,
    pub agent_id: String,
    pub status: ResearchStatus,
    pub results: Option<ResearchResults>,
    pub metadata: ResearchMetadata,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Research execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchStatus {
    Queued,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

/// Research results structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchResults {
    pub summary: String,
    pub key_findings: Vec<String>,
    pub evidence: Vec<ResearchEvidence>,
    pub recommendations: Vec<String>,
    pub confidence_score: f64,
    pub sources: Vec<ResearchSource>,
}

/// Research evidence item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchEvidence {
    pub claim: String,
    pub evidence: String,
    pub source: String,
    pub confidence: f64,
    pub relevance: f64,
}

/// Research source information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchSource {
    pub url: String,
    pub title: String,
    pub provider: String,
    pub accessed_at: DateTime<Utc>,
    pub relevance_score: f64,
}

/// Research execution metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchMetadata {
    pub methodology_used: String,
    pub apis_accessed: Vec<String>,
    pub duration_seconds: u64,
    pub cost_estimate: f64,
    pub steps_executed: u32,
    pub sources_analyzed: u32,
}

/// Enhanced BMAD Agent Task with research capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedBMadTask {
    pub base_task: BMadAgentTask,
    pub research_phase: Option<ResearchPhase>,
    pub execution_phase: ExecutionPhase,
}

/// Base BMAD agent task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BMadAgentTask {
    pub task_id: Uuid,
    pub agent_id: String,
    pub task_type: String,
    pub description: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Research phase configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchPhase {
    pub enabled: bool,
    pub research_requests: Vec<BMadResearchRequest>,
    pub synthesis_prompt: String,
    pub max_total_duration_minutes: u32,
}

/// Task execution phase configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPhase {
    pub research_integration: bool,
    pub evidence_requirements: Vec<String>,
    pub validation_criteria: Vec<String>,
    pub quality_gates: Vec<QualityGate>,
}

/// Quality gate for research validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityGate {
    pub name: String,
    pub criteria: String,
    pub threshold: f64,
    pub required: bool,
}

/// Research Bridge Service - Main integration component
pub struct ResearchBridgeService {
    research_engine: Arc<RwLock<ResearchEngineService>>,
    ai_orchestration: Arc<RwLock<AIOrchestrationService>>,
    api_manager: Arc<RwLock<ApiManagerService>>,
    data_persistence: Arc<RwLock<DataPersistenceService>>,
    active_research: Arc<RwLock<HashMap<Uuid, BMadResearchResponse>>>,
    agent_configs: Arc<RwLock<HashMap<String, AgentResearchConfig>>>,
}

/// Agent-specific research configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResearchConfig {
    pub agent_id: String,
    pub default_methodology: ResearchMethodology,
    pub research_capabilities: Vec<ResearchType>,
    pub auto_research_triggers: Vec<AutoResearchTrigger>,
    pub cost_limits: CostLimits,
    pub quality_requirements: QualityRequirements,
}

/// Automatic research trigger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoResearchTrigger {
    pub task_type: String,
    pub research_type: ResearchType,
    pub depth: ResearchDepth,
    pub conditions: Vec<String>,
}

/// Cost limits for research operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostLimits {
    pub max_per_research: f64,
    pub max_per_task: f64,
    pub max_daily: f64,
}

/// Quality requirements for research
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityRequirements {
    pub min_confidence_score: f64,
    pub min_sources: u32,
    pub min_evidence_items: u32,
    pub required_source_diversity: f64,
}

impl ResearchBridgeService {
    /// Create new Research Bridge Service
    pub async fn new(
        research_engine: Arc<RwLock<ResearchEngineService>>,
        ai_orchestration: Arc<RwLock<AIOrchestrationService>>,
        api_manager: Arc<RwLock<ApiManagerService>>,
        data_persistence: Arc<RwLock<DataPersistenceService>>,
    ) -> AppResult<Self> {
        info!("Initializing Research Bridge Service");

        Ok(Self {
            research_engine,
            ai_orchestration,
            api_manager,
            data_persistence,
            active_research: Arc::new(RwLock::new(HashMap::new())),
            agent_configs: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Conduct research for a BMAD agent
    pub async fn conduct_agent_research(
        &self,
        request: BMadResearchRequest,
    ) -> AppResult<BMadResearchResponse> {
        info!("Conducting research for agent: {} - {}", request.agent_id, request.research_type);

        let research_id = Uuid::new_v4();
        let start_time = Utc::now();

        // Create initial response
        let mut response = BMadResearchResponse {
            research_id,
            agent_id: request.agent_id.clone(),
            status: ResearchStatus::Queued,
            results: None,
            metadata: ResearchMetadata {
                methodology_used: format!("{:?}", request.methodology),
                apis_accessed: Vec::new(),
                duration_seconds: 0,
                cost_estimate: 0.0,
                steps_executed: 0,
                sources_analyzed: 0,
            },
            created_at: start_time,
            completed_at: None,
        };

        // Store in active research
        {
            let mut active_research = self.active_research.write().await;
            active_research.insert(research_id, response.clone());
        }

        // Convert BMAD request to research workflow
        let workflow = self.create_research_workflow(&request).await?;

        // Execute research workflow
        response.status = ResearchStatus::InProgress;
        self.update_research_status(research_id, response.clone()).await?;

        match self.execute_research_workflow(workflow).await {
            Ok(completed_workflow) => {
                // Process results
                let results = self.process_research_results(&request, &completed_workflow).await?;
                
                response.status = ResearchStatus::Completed;
                response.results = Some(results);
                response.completed_at = Some(Utc::now());
                response.metadata.duration_seconds = 
                    (Utc::now() - start_time).num_seconds() as u64;

                info!("Research completed successfully for agent: {}", request.agent_id);
            }
            Err(e) => {
                error!("Research failed for agent {}: {}", request.agent_id, e);
                response.status = ResearchStatus::Failed;
                response.completed_at = Some(Utc::now());
            }
        }

        // Update final status
        self.update_research_status(research_id, response.clone()).await?;

        Ok(response)
    }

    /// Enhance a BMAD agent task with research capabilities
    pub async fn enhance_agent_task(
        &self,
        task: BMadAgentTask,
        research_config: Option<ResearchPhase>,
    ) -> AppResult<EnhancedBMadTask> {
        info!("Enhancing task for agent: {} - {}", task.agent_id, task.task_type);

        // Get agent configuration
        let agent_config = self.get_agent_config(&task.agent_id).await?;

        // Determine if research should be triggered automatically
        let research_phase = if let Some(config) = research_config {
            Some(config)
        } else {
            self.check_auto_research_triggers(&task, &agent_config).await?
        };

        let enhanced_task = EnhancedBMadTask {
            base_task: task,
            research_phase,
            execution_phase: ExecutionPhase {
                research_integration: research_phase.is_some(),
                evidence_requirements: agent_config.quality_requirements.min_evidence_items.to_string().into(),
                validation_criteria: vec![
                    format!("Minimum confidence score: {}", agent_config.quality_requirements.min_confidence_score),
                    format!("Minimum sources: {}", agent_config.quality_requirements.min_sources),
                ],
                quality_gates: vec![
                    QualityGate {
                        name: "Confidence Threshold".to_string(),
                        criteria: "Research confidence score".to_string(),
                        threshold: agent_config.quality_requirements.min_confidence_score,
                        required: true,
                    },
                ],
            },
        };

        Ok(enhanced_task)
    }

    /// Get research status
    pub async fn get_research_status(&self, research_id: Uuid) -> AppResult<Option<BMadResearchResponse>> {
        let active_research = self.active_research.read().await;
        Ok(active_research.get(&research_id).cloned())
    }

    /// Register agent research configuration
    pub async fn register_agent_config(&self, config: AgentResearchConfig) -> AppResult<()> {
        info!("Registering research config for agent: {}", config.agent_id);
        
        let mut agent_configs = self.agent_configs.write().await;
        agent_configs.insert(config.agent_id.clone(), config);
        
        Ok(())
    }

    // Private helper methods
    async fn create_research_workflow(&self, request: &BMadResearchRequest) -> AppResult<ResearchWorkflow> {
        info!("Creating research workflow for agent: {} - {:?}", request.agent_id, request.research_type);

        let workflow_id = Uuid::new_v4();
        let steps = self.create_research_steps(request).await?;

        let workflow = ResearchWorkflow {
            id: workflow_id,
            name: format!("{} - {}", request.agent_name, self.research_type_to_string(&request.research_type)),
            description: format!("Research workflow for {} agent: {}", request.agent_name, request.query),
            methodology: self.methodology_to_string(&request.methodology),
            query: request.query.clone(),
            steps,
            status: WorkflowStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: None,
            completed_at: None,
            results: Vec::new(),
            metadata: serde_json::json!({
                "agent_id": request.agent_id,
                "research_type": request.research_type,
                "depth": request.depth,
                "focus_areas": request.focus_areas,
                "max_duration_minutes": request.max_duration_minutes,
                "cost_limit": request.cost_limit
            }),
        };

        Ok(workflow)
    }

    async fn create_research_steps(&self, request: &BMadResearchRequest) -> AppResult<Vec<ResearchStep>> {
        let mut steps = Vec::new();
        let step_id_base = Uuid::new_v4();

        match request.research_type {
            ResearchType::MarketAnalysis => {
                // Market size research
                steps.push(ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "web_search".to_string(),
                    provider: "serpapi".to_string(),
                    query: format!("{} market size analysis", request.query),
                    parameters: serde_json::json!({
                        "num_results": 10,
                        "focus": "market_size"
                    }),
                    status: StepStatus::Pending,
                    created_at: Utc::now(),
                    started_at: None,
                    completed_at: None,
                    result: None,
                    error: None,
                });

                // Competitive analysis
                steps.push(ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "web_search".to_string(),
                    provider: "exa".to_string(),
                    query: format!("{} competitors analysis", request.query),
                    parameters: serde_json::json!({
                        "num_results": 8,
                        "focus": "competitors"
                    }),
                    status: StepStatus::Pending,
                    created_at: Utc::now(),
                    started_at: None,
                    completed_at: None,
                    result: None,
                    error: None,
                });

                // AI synthesis
                steps.push(ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "ai_analysis".to_string(),
                    provider: "openrouter".to_string(),
                    query: format!("Analyze market data for {}", request.query),
                    parameters: serde_json::json!({
                        "analysis_type": "market_analysis",
                        "focus_areas": request.focus_areas
                    }),
                    status: StepStatus::Pending,
                    created_at: Utc::now(),
                    started_at: None,
                    completed_at: None,
                    result: None,
                    error: None,
                });
            }

            ResearchType::TechnologyEvaluation => {
                // Technology documentation research
                steps.push(ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "content_extraction".to_string(),
                    provider: "firecrawl".to_string(),
                    query: format!("{} documentation analysis", request.query),
                    parameters: serde_json::json!({
                        "extract_type": "documentation",
                        "focus": "technical_specs"
                    }),
                    status: StepStatus::Pending,
                    created_at: Utc::now(),
                    started_at: None,
                    completed_at: None,
                    result: None,
                    error: None,
                });

                // Performance benchmarks
                steps.push(ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "web_search".to_string(),
                    provider: "tavily".to_string(),
                    query: format!("{} performance benchmarks comparison", request.query),
                    parameters: serde_json::json!({
                        "num_results": 6,
                        "focus": "performance"
                    }),
                    status: StepStatus::Pending,
                    created_at: Utc::now(),
                    started_at: None,
                    completed_at: None,
                    result: None,
                    error: None,
                });

                // Technical analysis
                steps.push(ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "ai_analysis".to_string(),
                    provider: "openrouter".to_string(),
                    query: format!("Technical evaluation of {}", request.query),
                    parameters: serde_json::json!({
                        "analysis_type": "technology_evaluation",
                        "focus_areas": request.focus_areas
                    }),
                    status: StepStatus::Pending,
                    created_at: Utc::now(),
                    started_at: None,
                    completed_at: None,
                    result: None,
                    error: None,
                });
            }

            ResearchType::InfrastructureResearch => {
                // Cloud pricing research
                steps.push(ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "web_search".to_string(),
                    provider: "serpapi".to_string(),
                    query: format!("{} cloud infrastructure pricing", request.query),
                    parameters: serde_json::json!({
                        "num_results": 8,
                        "focus": "pricing"
                    }),
                    status: StepStatus::Pending,
                    created_at: Utc::now(),
                    started_at: None,
                    completed_at: None,
                    result: None,
                    error: None,
                });

                // Best practices research
                steps.push(ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "content_extraction".to_string(),
                    provider: "jina".to_string(),
                    query: format!("{} infrastructure best practices", request.query),
                    parameters: serde_json::json!({
                        "extract_type": "best_practices",
                        "focus": "infrastructure"
                    }),
                    status: StepStatus::Pending,
                    created_at: Utc::now(),
                    started_at: None,
                    completed_at: None,
                    result: None,
                    error: None,
                });

                // Cost optimization analysis
                steps.push(ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "ai_analysis".to_string(),
                    provider: "openrouter".to_string(),
                    query: format!("Infrastructure cost optimization for {}", request.query),
                    parameters: serde_json::json!({
                        "analysis_type": "cost_optimization",
                        "focus_areas": request.focus_areas
                    }),
                    status: StepStatus::Pending,
                    created_at: Utc::now(),
                    started_at: None,
                    completed_at: None,
                    result: None,
                    error: None,
                });
            }

            // Add other research types as needed
            _ => {
                // Generic research workflow
                steps.push(ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "web_search".to_string(),
                    provider: "tavily".to_string(),
                    query: request.query.clone(),
                    parameters: serde_json::json!({
                        "num_results": 10,
                        "focus": "general"
                    }),
                    status: StepStatus::Pending,
                    created_at: Utc::now(),
                    started_at: None,
                    completed_at: None,
                    result: None,
                    error: None,
                });

                steps.push(ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "ai_analysis".to_string(),
                    provider: "openrouter".to_string(),
                    query: format!("Analyze research data for {}", request.query),
                    parameters: serde_json::json!({
                        "analysis_type": "general_analysis",
                        "focus_areas": request.focus_areas
                    }),
                    status: StepStatus::Pending,
                    created_at: Utc::now(),
                    started_at: None,
                    completed_at: None,
                    result: None,
                    error: None,
                });
            }
        }

        Ok(steps)
    }

    async fn execute_research_workflow(&self, workflow: ResearchWorkflow) -> AppResult<ResearchWorkflow> {
        let research_engine = self.research_engine.read().await;
        research_engine.execute_workflow(workflow).await
    }

    async fn process_research_results(
        &self,
        request: &BMadResearchRequest,
        workflow: &ResearchWorkflow,
    ) -> AppResult<ResearchResults> {
        info!("Processing research results for agent: {}", request.agent_id);

        let mut key_findings = Vec::new();
        let mut evidence = Vec::new();
        let mut sources = Vec::new();
        let mut recommendations = Vec::new();
        let mut total_confidence = 0.0;
        let mut confidence_count = 0;

        // Process each step's results
        for step in &workflow.steps {
            if let Some(result) = &step.result {
                // Extract findings from step result
                if let Some(findings) = result.get("findings").and_then(|f| f.as_array()) {
                    for finding in findings {
                        if let Some(text) = finding.get("text").and_then(|t| t.as_str()) {
                            key_findings.push(text.to_string());
                        }
                    }
                }

                // Extract evidence
                if let Some(evidence_items) = result.get("evidence").and_then(|e| e.as_array()) {
                    for item in evidence_items {
                        if let (Some(claim), Some(evidence_text), Some(source)) = (
                            item.get("claim").and_then(|c| c.as_str()),
                            item.get("evidence").and_then(|e| e.as_str()),
                            item.get("source").and_then(|s| s.as_str()),
                        ) {
                            evidence.push(ResearchEvidence {
                                claim: claim.to_string(),
                                evidence: evidence_text.to_string(),
                                source: source.to_string(),
                                confidence: item.get("confidence").and_then(|c| c.as_f64()).unwrap_or(0.7),
                                relevance: item.get("relevance").and_then(|r| r.as_f64()).unwrap_or(0.8),
                            });
                        }
                    }
                }

                // Extract sources
                if let Some(source_items) = result.get("sources").and_then(|s| s.as_array()) {
                    for source in source_items {
                        if let (Some(url), Some(title)) = (
                            source.get("url").and_then(|u| u.as_str()),
                            source.get("title").and_then(|t| t.as_str()),
                        ) {
                            sources.push(ResearchSource {
                                url: url.to_string(),
                                title: title.to_string(),
                                provider: step.provider.clone(),
                                accessed_at: step.completed_at.unwrap_or(Utc::now()),
                                relevance_score: source.get("relevance").and_then(|r| r.as_f64()).unwrap_or(0.8),
                            });
                        }
                    }
                }

                // Extract confidence scores
                if let Some(confidence) = result.get("confidence").and_then(|c| c.as_f64()) {
                    total_confidence += confidence;
                    confidence_count += 1;
                }

                // Extract recommendations
                if let Some(recs) = result.get("recommendations").and_then(|r| r.as_array()) {
                    for rec in recs {
                        if let Some(text) = rec.as_str() {
                            recommendations.push(text.to_string());
                        }
                    }
                }
            }
        }

        // Calculate overall confidence
        let overall_confidence = if confidence_count > 0 {
            total_confidence / confidence_count as f64
        } else {
            0.7 // Default confidence
        };

        // Generate summary based on research type
        let summary = self.generate_research_summary(request, &key_findings, &evidence).await?;

        // Ensure minimum quality requirements
        if key_findings.is_empty() {
            key_findings.push("Research completed but no specific findings extracted".to_string());
        }

        if recommendations.is_empty() {
            recommendations = self.generate_default_recommendations(request).await?;
        }

        Ok(ResearchResults {
            summary,
            key_findings,
            evidence,
            recommendations,
            confidence_score: overall_confidence,
            sources,
        })
    }

    async fn generate_research_summary(
        &self,
        request: &BMadResearchRequest,
        findings: &[String],
        evidence: &[ResearchEvidence],
    ) -> AppResult<String> {
        let summary = match request.research_type {
            ResearchType::MarketAnalysis => {
                format!(
                    "Market analysis for '{}' completed. Found {} key insights with {} pieces of supporting evidence. Research focused on {}.",
                    request.query,
                    findings.len(),
                    evidence.len(),
                    request.focus_areas.join(", ")
                )
            }
            ResearchType::TechnologyEvaluation => {
                format!(
                    "Technology evaluation for '{}' completed. Analyzed {} technical aspects with {} evidence points. Evaluation covered {}.",
                    request.query,
                    findings.len(),
                    evidence.len(),
                    request.focus_areas.join(", ")
                )
            }
            ResearchType::InfrastructureResearch => {
                format!(
                    "Infrastructure research for '{}' completed. Identified {} optimization opportunities with {} supporting data points. Analysis focused on {}.",
                    request.query,
                    findings.len(),
                    evidence.len(),
                    request.focus_areas.join(", ")
                )
            }
            _ => {
                format!(
                    "Research for '{}' completed. Gathered {} insights with {} evidence items.",
                    request.query,
                    findings.len(),
                    evidence.len()
                )
            }
        };

        Ok(summary)
    }

    async fn generate_default_recommendations(&self, request: &BMadResearchRequest) -> AppResult<Vec<String>> {
        let recommendations = match request.research_type {
            ResearchType::MarketAnalysis => vec![
                "Conduct additional market validation with target customers".to_string(),
                "Analyze competitor pricing strategies in detail".to_string(),
                "Identify key market entry barriers and mitigation strategies".to_string(),
            ],
            ResearchType::TechnologyEvaluation => vec![
                "Conduct proof-of-concept implementation".to_string(),
                "Evaluate long-term maintenance and support requirements".to_string(),
                "Assess team training and skill development needs".to_string(),
            ],
            ResearchType::InfrastructureResearch => vec![
                "Implement cost monitoring and optimization tools".to_string(),
                "Establish performance benchmarks and SLAs".to_string(),
                "Plan for scalability and disaster recovery".to_string(),
            ],
            _ => vec![
                "Review findings with relevant stakeholders".to_string(),
                "Validate research conclusions with additional sources".to_string(),
                "Develop implementation plan based on research insights".to_string(),
            ],
        };

        Ok(recommendations)
    }

    fn research_type_to_string(&self, research_type: &ResearchType) -> String {
        match research_type {
            ResearchType::MarketAnalysis => "Market Analysis".to_string(),
            ResearchType::CompetitiveResearch => "Competitive Research".to_string(),
            ResearchType::TechnologyEvaluation => "Technology Evaluation".to_string(),
            ResearchType::ArchitecturePatterns => "Architecture Patterns".to_string(),
            ResearchType::SecurityResearch => "Security Research".to_string(),
            ResearchType::InfrastructureResearch => "Infrastructure Research".to_string(),
            ResearchType::UserResearch => "User Research".to_string(),
            ResearchType::ComplianceResearch => "Compliance Research".to_string(),
        }
    }

    fn methodology_to_string(&self, methodology: &ResearchMethodology) -> String {
        match methodology {
            ResearchMethodology::DonLim => "don_lim".to_string(),
            ResearchMethodology::NickScamara => "nick_scamara".to_string(),
            ResearchMethodology::Hybrid => "hybrid".to_string(),
            ResearchMethodology::Comprehensive => "comprehensive".to_string(),
        }
    }

    async fn update_research_status(&self, research_id: Uuid, response: BMadResearchResponse) -> AppResult<()> {
        let mut active_research = self.active_research.write().await;
        active_research.insert(research_id, response);
        Ok(())
    }

    async fn get_agent_config(&self, agent_id: &str) -> AppResult<AgentResearchConfig> {
        let agent_configs = self.agent_configs.read().await;
        agent_configs.get(agent_id)
            .cloned()
            .ok_or_else(|| ResearchError::not_found(format!("Agent config not found: {}", agent_id)).into())
    }

    async fn check_auto_research_triggers(
        &self,
        task: &BMadAgentTask,
        agent_config: &AgentResearchConfig,
    ) -> AppResult<Option<ResearchPhase>> {
        // Check if task should trigger automatic research
        for trigger in &agent_config.auto_research_triggers {
            if trigger.task_type == task.task_type {
                // Create research phase
                let research_request = BMadResearchRequest {
                    agent_id: task.agent_id.clone(),
                    agent_name: agent_config.agent_id.clone(),
                    research_type: trigger.research_type.clone(),
                    query: task.description.clone(),
                    methodology: agent_config.default_methodology.clone(),
                    focus_areas: vec![], // Will be populated based on task parameters
                    depth: trigger.depth.clone(),
                    max_duration_minutes: 30, // Default
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

        Ok(None)
    }
}

/// Tauri commands for frontend integration
#[tauri::command]
pub async fn bmad_conduct_research(
    request: BMadResearchRequest,
    state: tauri::State<'_, Arc<RwLock<ResearchBridgeService>>>,
) -> Result<BMadResearchResponse, String> {
    let bridge = state.read().await;
    bridge.conduct_agent_research(request).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn bmad_get_research_status(
    research_id: String,
    state: tauri::State<'_, Arc<RwLock<ResearchBridgeService>>>,
) -> Result<Option<BMadResearchResponse>, String> {
    let research_uuid = Uuid::parse_str(&research_id)
        .map_err(|e| format!("Invalid research ID: {}", e))?;
    
    let bridge = state.read().await;
    bridge.get_research_status(research_uuid).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn bmad_enhance_agent_task(
    task: BMadAgentTask,
    research_config: Option<ResearchPhase>,
    state: tauri::State<'_, Arc<RwLock<ResearchBridgeService>>>,
) -> Result<EnhancedBMadTask, String> {
    let bridge = state.read().await;
    bridge.enhance_agent_task(task, research_config).await
        .map_err(|e| e.to_string())
}
