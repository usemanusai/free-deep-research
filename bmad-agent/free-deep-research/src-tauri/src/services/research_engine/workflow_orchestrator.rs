use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, error, warn};
use uuid::Uuid;
use chrono::Utc;
use serde_json;

use crate::error::{AppResult, ResearchError};
use crate::services::{ApiManagerService, DataPersistenceService, MonitoringService};
use crate::models::research_workflow::{ResearchWorkflow, WorkflowStatus, ResearchStep, StepStatus, ResearchResult};

/// Workflow orchestrator for managing research execution
pub struct WorkflowOrchestrator {
    api_manager: Arc<RwLock<ApiManagerService>>,
    data_persistence: Arc<RwLock<DataPersistenceService>>,
    monitoring: Arc<RwLock<MonitoringService>>,
}

impl WorkflowOrchestrator {
    /// Create a new workflow orchestrator
    pub async fn new(
        api_manager: Arc<RwLock<ApiManagerService>>,
        data_persistence: Arc<RwLock<DataPersistenceService>>,
        monitoring: Arc<RwLock<MonitoringService>>,
    ) -> AppResult<Self> {
        info!("Initializing workflow orchestrator");
        
        let orchestrator = Self {
            api_manager,
            data_persistence,
            monitoring,
        };
        
        info!("Workflow orchestrator initialized successfully");
        Ok(orchestrator)
    }
    
    /// Execute a research workflow
    pub async fn execute_workflow(&self, mut workflow: ResearchWorkflow) -> AppResult<ResearchWorkflow> {
        info!("Executing research workflow: {} ({})", workflow.name, workflow.id);
        
        workflow.status = WorkflowStatus::Running;
        workflow.started_at = Some(Utc::now());
        workflow.updated_at = Utc::now();
        
        let mut all_results = Vec::new();
        let mut execution_successful = true;
        
        // Execute each step in sequence
        for (step_index, step) in workflow.steps.iter_mut().enumerate() {
            info!("Executing step {}/{}: {} ({})", 
                step_index + 1, workflow.steps.len(), step.step_type, step.provider);
            
            step.status = StepStatus::Running;
            step.started_at = Some(Utc::now());
            
            match self.execute_step(step, &workflow.query, &all_results).await {
                Ok(result) => {
                    step.status = StepStatus::Completed;
                    step.result = Some(result.clone());
                    step.completed_at = Some(Utc::now());
                    all_results.push(result);
                    
                    debug!("Step completed successfully: {}", step.step_type);
                }
                Err(e) => {
                    step.status = StepStatus::Failed;
                    step.error = Some(e.to_string());
                    step.completed_at = Some(Utc::now());
                    execution_successful = false;
                    
                    error!("Step failed: {} - {}", step.step_type, e);
                    
                    // Decide whether to continue or abort
                    if self.is_critical_step(&step.step_type) {
                        error!("Critical step failed, aborting workflow");
                        break;
                    } else {
                        warn!("Non-critical step failed, continuing workflow");
                    }
                }
            }
        }
        
        // Update workflow status and results
        if execution_successful {
            workflow.status = WorkflowStatus::Completed;
            workflow.results = Some(self.compile_results(&all_results, &workflow.query).await?);
        } else {
            workflow.status = WorkflowStatus::Failed;
        }
        
        workflow.completed_at = Some(Utc::now());
        workflow.updated_at = Utc::now();
        
        info!("Workflow execution completed: {} (status: {:?})", workflow.name, workflow.status);
        Ok(workflow)
    }
    
    /// Execute a single research step
    async fn execute_step(
        &self,
        step: &ResearchStep,
        query: &str,
        previous_results: &[serde_json::Value],
    ) -> AppResult<serde_json::Value> {
        debug!("Executing step: {} with provider: {}", step.step_type, step.provider);
        
        match step.step_type.as_str() {
            "web_search" => self.execute_web_search(step, query).await,
            "academic_search" => self.execute_academic_search(step, query).await,
            "content_extraction" => self.execute_content_extraction(step, previous_results).await,
            "ai_analysis" => self.execute_ai_analysis(step, query, previous_results).await,
            "ai_summary" => self.execute_ai_summary(step, query, previous_results).await,
            "academic_analysis" => self.execute_academic_analysis(step, query, previous_results).await,
            _ => {
                error!("Unknown step type: {}", step.step_type);
                Err(ResearchError::invalid_step_type(step.step_type.clone()).into())
            }
        }
    }
    
    /// Execute web search step
    async fn execute_web_search(&self, step: &ResearchStep, query: &str) -> AppResult<serde_json::Value> {
        debug!("Executing web search with provider: {}", step.provider);
        
        // Get parameters
        let num_results = step.parameters.get("num_results")
            .and_then(|v| v.as_u64())
            .unwrap_or(10) as u32;
        
        match step.provider.as_str() {
            "serpapi" => {
                // TODO: Implement SerpAPI search
                Ok(serde_json::json!({
                    "provider": "serpapi",
                    "query": query,
                    "results": [],
                    "message": "SerpAPI search not yet implemented"
                }))
            }
            "tavily" => {
                // TODO: Implement Tavily search
                Ok(serde_json::json!({
                    "provider": "tavily",
                    "query": query,
                    "results": [],
                    "message": "Tavily search not yet implemented"
                }))
            }
            _ => {
                Err(ResearchError::unsupported_provider(step.provider.clone()).into())
            }
        }
    }
    
    /// Execute academic search step
    async fn execute_academic_search(&self, step: &ResearchStep, query: &str) -> AppResult<serde_json::Value> {
        debug!("Executing academic search with provider: {}", step.provider);
        
        match step.provider.as_str() {
            "exa" => {
                // TODO: Implement Exa academic search
                Ok(serde_json::json!({
                    "provider": "exa",
                    "query": query,
                    "results": [],
                    "message": "Exa academic search not yet implemented"
                }))
            }
            _ => {
                Err(ResearchError::unsupported_provider(step.provider.clone()).into())
            }
        }
    }
    
    /// Execute content extraction step
    async fn execute_content_extraction(
        &self,
        step: &ResearchStep,
        previous_results: &[serde_json::Value],
    ) -> AppResult<serde_json::Value> {
        debug!("Executing content extraction with provider: {}", step.provider);
        
        match step.provider.as_str() {
            "firecrawl" => {
                // TODO: Implement Firecrawl content extraction
                Ok(serde_json::json!({
                    "provider": "firecrawl",
                    "extracted_content": [],
                    "message": "Firecrawl content extraction not yet implemented"
                }))
            }
            "jina" => {
                // TODO: Implement Jina content extraction
                Ok(serde_json::json!({
                    "provider": "jina",
                    "extracted_content": [],
                    "message": "Jina content extraction not yet implemented"
                }))
            }
            _ => {
                Err(ResearchError::unsupported_provider(step.provider.clone()).into())
            }
        }
    }
    
    /// Execute AI analysis step
    async fn execute_ai_analysis(
        &self,
        step: &ResearchStep,
        query: &str,
        previous_results: &[serde_json::Value],
    ) -> AppResult<serde_json::Value> {
        debug!("Executing AI analysis with provider: {}", step.provider);
        
        match step.provider.as_str() {
            "openrouter" => {
                // TODO: Implement OpenRouter AI analysis
                Ok(serde_json::json!({
                    "provider": "openrouter",
                    "analysis": "AI analysis not yet implemented",
                    "confidence": 0.0,
                    "message": "OpenRouter AI analysis not yet implemented"
                }))
            }
            _ => {
                Err(ResearchError::unsupported_provider(step.provider.clone()).into())
            }
        }
    }
    
    /// Execute AI summary step
    async fn execute_ai_summary(
        &self,
        step: &ResearchStep,
        query: &str,
        previous_results: &[serde_json::Value],
    ) -> AppResult<serde_json::Value> {
        debug!("Executing AI summary with provider: {}", step.provider);
        
        match step.provider.as_str() {
            "openrouter" => {
                // TODO: Implement OpenRouter AI summary
                Ok(serde_json::json!({
                    "provider": "openrouter",
                    "summary": "AI summary not yet implemented",
                    "key_points": [],
                    "message": "OpenRouter AI summary not yet implemented"
                }))
            }
            _ => {
                Err(ResearchError::unsupported_provider(step.provider.clone()).into())
            }
        }
    }
    
    /// Execute academic analysis step
    async fn execute_academic_analysis(
        &self,
        step: &ResearchStep,
        query: &str,
        previous_results: &[serde_json::Value],
    ) -> AppResult<serde_json::Value> {
        debug!("Executing academic analysis with provider: {}", step.provider);
        
        match step.provider.as_str() {
            "openrouter" => {
                // TODO: Implement OpenRouter academic analysis
                Ok(serde_json::json!({
                    "provider": "openrouter",
                    "academic_analysis": "Academic analysis not yet implemented",
                    "citations": [],
                    "methodology": "Not specified",
                    "message": "OpenRouter academic analysis not yet implemented"
                }))
            }
            _ => {
                Err(ResearchError::unsupported_provider(step.provider.clone()).into())
            }
        }
    }
    
    /// Check if a step type is critical for workflow success
    fn is_critical_step(&self, step_type: &str) -> bool {
        matches!(step_type, "web_search" | "academic_search")
    }
    
    /// Compile all step results into final research result
    async fn compile_results(
        &self,
        step_results: &[serde_json::Value],
        query: &str,
    ) -> AppResult<ResearchResult> {
        debug!("Compiling research results from {} steps", step_results.len());
        
        // TODO: Implement intelligent result compilation
        let summary = format!("Research completed for query: '{}'. {} steps executed.", query, step_results.len());
        
        Ok(ResearchResult {
            summary,
            detailed_findings: "Detailed findings compilation not yet implemented".to_string(),
            sources: vec![],
            confidence: 0.5,
            metadata: serde_json::json!({
                "steps_executed": step_results.len(),
                "compilation_method": "basic",
                "timestamp": Utc::now().to_rfc3339()
            }),
        })
    }
}
