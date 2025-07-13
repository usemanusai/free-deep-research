use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{info, debug, error};
use uuid::Uuid;
use chrono::Utc;
use serde_json;

use crate::error::{AppResult, ResearchError};
use crate::services::{Service, ApiManagerService, DataPersistenceService, MonitoringService};
use crate::models::research_workflow::{
    ResearchWorkflow, WorkflowStatus, ResearchMethodology, WorkflowParameters,
    CreateWorkflowRequest, ResearchResult, ResearchStep, StepStatus
};

pub mod workflow_orchestrator;
pub mod queue_manager;
pub mod result_processor;
pub mod workflow_engine;
pub mod methodology_don_lim;
pub mod methodology_nick_scamara;
pub mod methodology_hybrid;

/// Research Engine Service that orchestrates research workflows
pub struct ResearchEngineService {
    api_manager: Arc<RwLock<ApiManagerService>>,
    data_persistence: Arc<RwLock<DataPersistenceService>>,
    monitoring: Arc<RwLock<MonitoringService>>,
    active_workflows: Arc<RwLock<HashMap<Uuid, ResearchWorkflow>>>,
    methodologies: Arc<RwLock<HashMap<String, ResearchMethodology>>>,
    workflow_engine: Arc<workflow_engine::WorkflowEngine>,
}

impl ResearchEngineService {
    /// Create a new research engine service
    pub async fn new(
        api_manager: Arc<RwLock<ApiManagerService>>,
        data_persistence: Arc<RwLock<DataPersistenceService>>,
        monitoring: Arc<RwLock<MonitoringService>>,
    ) -> AppResult<Self> {
        info!("Initializing research engine service...");

        // Create workflow engine
        let workflow_engine = Arc::new(workflow_engine::WorkflowEngine::new(
            data_persistence.clone(),
            api_manager.clone(),
        ).await?);

        let service = Self {
            api_manager,
            data_persistence,
            monitoring,
            active_workflows: Arc::new(RwLock::new(HashMap::new())),
            methodologies: Arc::new(RwLock::new(HashMap::new())),
            workflow_engine,
        };

        // Initialize default methodologies
        service.initialize_default_methodologies().await?;

        info!("Research engine service initialized successfully");
        Ok(service)
    }

    /// Initialize default research methodologies
    async fn initialize_default_methodologies(&self) -> AppResult<()> {
        debug!("Initializing default research methodologies");

        let mut methodologies = self.methodologies.write().await;

        // Comprehensive methodology
        methodologies.insert("comprehensive".to_string(), ResearchMethodology {
            id: Uuid::new_v4(),
            name: "Comprehensive Research".to_string(),
            description: "Deep, multi-source research with AI analysis".to_string(),
            steps: vec![
                ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "web_search".to_string(),
                    provider: "serpapi".to_string(),
                    parameters: serde_json::json!({
                        "num_results": 20,
                        "time_range": "last_year"
                    }),
                    status: StepStatus::Pending,
                    result: None,
                    error: None,
                    started_at: None,
                    completed_at: None,
                },
                ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "content_extraction".to_string(),
                    provider: "firecrawl".to_string(),
                    parameters: serde_json::json!({
                        "extract_depth": "advanced",
                        "include_images": false
                    }),
                    status: StepStatus::Pending,
                    result: None,
                    error: None,
                    started_at: None,
                    completed_at: None,
                },
                ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "ai_analysis".to_string(),
                    provider: "openrouter".to_string(),
                    parameters: serde_json::json!({
                        "model": "anthropic/claude-3-sonnet",
                        "temperature": 0.3,
                        "max_tokens": 4000
                    }),
                    status: StepStatus::Pending,
                    result: None,
                    error: None,
                    started_at: None,
                    completed_at: None,
                },
            ],
            estimated_duration_minutes: 15,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        // Quick methodology
        methodologies.insert("quick".to_string(), ResearchMethodology {
            id: Uuid::new_v4(),
            name: "Quick Research".to_string(),
            description: "Fast research with basic AI analysis".to_string(),
            steps: vec![
                ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "web_search".to_string(),
                    provider: "tavily".to_string(),
                    parameters: serde_json::json!({
                        "num_results": 10,
                        "search_depth": "basic"
                    }),
                    status: StepStatus::Pending,
                    result: None,
                    error: None,
                    started_at: None,
                    completed_at: None,
                },
                ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "ai_summary".to_string(),
                    provider: "openrouter".to_string(),
                    parameters: serde_json::json!({
                        "model": "anthropic/claude-3-haiku",
                        "temperature": 0.2,
                        "max_tokens": 2000
                    }),
                    status: StepStatus::Pending,
                    result: None,
                    error: None,
                    started_at: None,
                    completed_at: None,
                },
            ],
            estimated_duration_minutes: 5,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        // Academic methodology
        methodologies.insert("academic".to_string(), ResearchMethodology {
            id: Uuid::new_v4(),
            name: "Academic Research".to_string(),
            description: "Scholarly research focused on academic sources".to_string(),
            steps: vec![
                ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "academic_search".to_string(),
                    provider: "exa".to_string(),
                    parameters: serde_json::json!({
                        "search_type": "academic",
                        "num_results": 15,
                        "include_domains": ["arxiv.org", "scholar.google.com", "pubmed.ncbi.nlm.nih.gov"]
                    }),
                    status: StepStatus::Pending,
                    result: None,
                    error: None,
                    started_at: None,
                    completed_at: None,
                },
                ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "content_extraction".to_string(),
                    provider: "jina".to_string(),
                    parameters: serde_json::json!({
                        "extract_depth": "advanced",
                        "focus": "academic_content"
                    }),
                    status: StepStatus::Pending,
                    result: None,
                    error: None,
                    started_at: None,
                    completed_at: None,
                },
                ResearchStep {
                    id: Uuid::new_v4(),
                    step_type: "academic_analysis".to_string(),
                    provider: "openrouter".to_string(),
                    parameters: serde_json::json!({
                        "model": "anthropic/claude-3-sonnet",
                        "temperature": 0.1,
                        "max_tokens": 6000,
                        "system_prompt": "You are an academic researcher. Analyze the provided sources with scholarly rigor."
                    }),
                    status: StepStatus::Pending,
                    result: None,
                    error: None,
                    started_at: None,
                    completed_at: None,
                },
            ],
            estimated_duration_minutes: 20,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        debug!("Initialized {} default methodologies", methodologies.len());
        Ok(())
    }

    /// Create a new research workflow
    pub async fn create_workflow(&self, request: CreateWorkflowRequest) -> AppResult<ResearchWorkflow> {
        info!("Creating new research workflow: {}", request.name);

        // Validate request
        if request.name.trim().is_empty() {
            return Err(ResearchError::invalid_request("Workflow name cannot be empty".to_string()).into());
        }

        if request.query.trim().is_empty() {
            return Err(ResearchError::invalid_request("Research query cannot be empty".to_string()).into());
        }

        // Get methodology
        let methodologies = self.methodologies.read().await;
        let methodology_name = request.template_id
            .and_then(|id| methodologies.values().find(|m| m.id == id).map(|m| m.name.clone()))
            .unwrap_or_else(|| "comprehensive".to_string());

        let methodology = methodologies.get(&methodology_name)
            .ok_or_else(|| ResearchError::methodology_not_found(methodology_name.clone()))?
            .clone();
        drop(methodologies);

        // Create workflow
        let workflow = ResearchWorkflow {
            id: Uuid::new_v4(),
            name: request.name,
            query: request.query,
            status: WorkflowStatus::Created,
            methodology: methodology_name,
            parameters: request.parameters.unwrap_or_default(),
            steps: methodology.steps,
            results: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: None,
            completed_at: None,
        };

        // Store in active workflows
        let mut active_workflows = self.active_workflows.write().await;
        active_workflows.insert(workflow.id, workflow.clone());
        drop(active_workflows);

        // TODO: Store in database

        info!("Research workflow created: {} ({})", workflow.name, workflow.id);
        Ok(workflow)
    }

#[async_trait::async_trait]
impl Service for ResearchEngineService {
    async fn health_check(&self) -> AppResult<()> {
        debug!("Performing research engine health check");
        
        // TODO: Implement actual health check

        Ok(())
    }

    /// Start a workflow using the workflow engine
    pub async fn start_workflow_execution(&self, workflow_id: Uuid) -> AppResult<()> {
        info!("Starting workflow execution via workflow engine: {}", workflow_id);
        self.workflow_engine.start_workflow(workflow_id).await
    }

    /// Pause a workflow
    pub async fn pause_workflow_execution(&self, workflow_id: Uuid) -> AppResult<()> {
        info!("Pausing workflow execution: {}", workflow_id);
        self.workflow_engine.pause_workflow(workflow_id).await
    }

    /// Resume a workflow
    pub async fn resume_workflow_execution(&self, workflow_id: Uuid) -> AppResult<()> {
        info!("Resuming workflow execution: {}", workflow_id);
        self.workflow_engine.resume_workflow(workflow_id).await
    }

    /// Cancel a workflow
    pub async fn cancel_workflow_execution(&self, workflow_id: Uuid) -> AppResult<()> {
        info!("Cancelling workflow execution: {}", workflow_id);
        self.workflow_engine.cancel_workflow(workflow_id).await
    }

    /// Start background monitoring
    pub async fn start_background_monitoring(&self) -> AppResult<()> {
        info!("Starting research engine background monitoring...");

        // Start workflow engine monitoring
        self.workflow_engine.start_background_monitoring().await?;

        info!("Research engine background monitoring started successfully");
        Ok(())
    }

    async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down research engine service...");
        
        // TODO: Implement graceful shutdown
        
        Ok(())
    }
}
