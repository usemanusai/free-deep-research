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
                self.execute_serpapi_search(query, num_results).await
            }
            "tavily" => {
                self.execute_tavily_search(query, num_results).await
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
                self.execute_exa_search(query, 10).await
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
                self.execute_firecrawl_extraction(previous_results).await
            }
            "jina" => {
                self.execute_jina_processing(previous_results).await
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

    /// Execute SerpAPI search
    async fn execute_serpapi_search(&self, query: &str, num_results: u32) -> AppResult<serde_json::Value> {
        debug!("Executing SerpAPI search for query: {}", query);

        let api_manager = self.api_manager.read().await;
        let service_request = crate::services::api_manager::service_integration::ServiceRequest {
            endpoint: "/search".to_string(),
            method: "GET".to_string(),
            headers: std::collections::HashMap::new(),
            body: None,
            parameters: {
                let mut params = std::collections::HashMap::new();
                params.insert("q".to_string(), query.to_string());
                params.insert("engine".to_string(), "google".to_string());
                params.insert("num".to_string(), num_results.to_string());
                params
            },
        };

        match api_manager.make_service_request(crate::models::api_key::ServiceProvider::SerpApi, service_request).await {
            Ok(response) => {
                debug!("SerpAPI search completed successfully");
                Ok(serde_json::json!({
                    "provider": "serpapi",
                    "query": query,
                    "results": response.data,
                    "success": response.success,
                    "response_time_ms": response.response_time_ms
                }))
            }
            Err(e) => {
                error!("SerpAPI search failed: {}", e);
                Err(e)
            }
        }
    }

    /// Execute Tavily search
    async fn execute_tavily_search(&self, query: &str, num_results: u32) -> AppResult<serde_json::Value> {
        debug!("Executing Tavily search for query: {}", query);

        let api_manager = self.api_manager.read().await;
        let request_body = serde_json::json!({
            "query": query,
            "search_depth": "basic",
            "max_results": num_results,
            "include_images": false,
            "include_answer": true
        });

        let service_request = crate::services::api_manager::service_integration::ServiceRequest {
            endpoint: "/search".to_string(),
            method: "POST".to_string(),
            headers: {
                let mut headers = std::collections::HashMap::new();
                headers.insert("Content-Type".to_string(), "application/json".to_string());
                headers
            },
            body: Some(request_body.to_string()),
            parameters: std::collections::HashMap::new(),
        };

        match api_manager.make_service_request(crate::models::api_key::ServiceProvider::Tavily, service_request).await {
            Ok(response) => {
                debug!("Tavily search completed successfully");
                Ok(serde_json::json!({
                    "provider": "tavily",
                    "query": query,
                    "results": response.data,
                    "success": response.success,
                    "response_time_ms": response.response_time_ms
                }))
            }
            Err(e) => {
                error!("Tavily search failed: {}", e);
                Err(e)
            }
        }
    }

    /// Execute Exa search
    async fn execute_exa_search(&self, query: &str, num_results: u32) -> AppResult<serde_json::Value> {
        debug!("Executing Exa search for query: {}", query);

        let api_manager = self.api_manager.read().await;
        let request_body = serde_json::json!({
            "query": query,
            "numResults": num_results,
            "useAutoprompt": false
        });

        let service_request = crate::services::api_manager::service_integration::ServiceRequest {
            endpoint: "/search".to_string(),
            method: "POST".to_string(),
            headers: {
                let mut headers = std::collections::HashMap::new();
                headers.insert("Content-Type".to_string(), "application/json".to_string());
                headers
            },
            body: Some(request_body.to_string()),
            parameters: std::collections::HashMap::new(),
        };

        match api_manager.make_service_request(crate::models::api_key::ServiceProvider::Exa, service_request).await {
            Ok(response) => {
                debug!("Exa search completed successfully");
                Ok(serde_json::json!({
                    "provider": "exa",
                    "query": query,
                    "results": response.data,
                    "success": response.success,
                    "response_time_ms": response.response_time_ms
                }))
            }
            Err(e) => {
                error!("Exa search failed: {}", e);
                Err(e)
            }
        }
    }

    /// Execute Firecrawl content extraction
    async fn execute_firecrawl_extraction(&self, previous_results: &[serde_json::Value]) -> AppResult<serde_json::Value> {
        debug!("Executing Firecrawl content extraction");

        // Extract URLs from previous search results
        let urls = self.extract_urls_from_results(previous_results);
        if urls.is_empty() {
            return Ok(serde_json::json!({
                "provider": "firecrawl",
                "extracted_content": [],
                "message": "No URLs found in previous results"
            }));
        }

        let api_manager = self.api_manager.read().await;
        let mut extracted_content = Vec::new();

        // Process up to 5 URLs to avoid rate limits
        for url in urls.iter().take(5) {
            let request_body = serde_json::json!({
                "url": url,
                "formats": ["markdown", "html"],
                "onlyMainContent": true
            });

            let service_request = crate::services::api_manager::service_integration::ServiceRequest {
                endpoint: "/scrape".to_string(),
                method: "POST".to_string(),
                headers: {
                    let mut headers = std::collections::HashMap::new();
                    headers.insert("Content-Type".to_string(), "application/json".to_string());
                    headers
                },
                body: Some(request_body.to_string()),
                parameters: std::collections::HashMap::new(),
            };

            match api_manager.make_service_request(crate::models::api_key::ServiceProvider::Firecrawl, service_request).await {
                Ok(response) => {
                    extracted_content.push(serde_json::json!({
                        "url": url,
                        "content": response.data,
                        "success": response.success
                    }));
                }
                Err(e) => {
                    warn!("Failed to extract content from {}: {}", url, e);
                    extracted_content.push(serde_json::json!({
                        "url": url,
                        "error": e.to_string(),
                        "success": false
                    }));
                }
            }
        }

        Ok(serde_json::json!({
            "provider": "firecrawl",
            "extracted_content": extracted_content,
            "urls_processed": urls.len().min(5),
            "total_urls_found": urls.len()
        }))
    }

    /// Execute Jina AI processing
    async fn execute_jina_processing(&self, previous_results: &[serde_json::Value]) -> AppResult<serde_json::Value> {
        debug!("Executing Jina AI processing");

        // Extract text content from previous results
        let text_content = self.extract_text_from_results(previous_results);
        if text_content.is_empty() {
            return Ok(serde_json::json!({
                "provider": "jina",
                "processed_content": [],
                "message": "No text content found in previous results"
            }));
        }

        let api_manager = self.api_manager.read().await;
        let request_body = serde_json::json!({
            "input": text_content,
            "model": "jina-embeddings-v2-base-en"
        });

        let service_request = crate::services::api_manager::service_integration::ServiceRequest {
            endpoint: "/embeddings".to_string(),
            method: "POST".to_string(),
            headers: {
                let mut headers = std::collections::HashMap::new();
                headers.insert("Content-Type".to_string(), "application/json".to_string());
                headers
            },
            body: Some(request_body.to_string()),
            parameters: std::collections::HashMap::new(),
        };

        match api_manager.make_service_request(crate::models::api_key::ServiceProvider::Jina, service_request).await {
            Ok(response) => {
                debug!("Jina AI processing completed successfully");
                Ok(serde_json::json!({
                    "provider": "jina",
                    "processed_content": response.data,
                    "success": response.success,
                    "response_time_ms": response.response_time_ms,
                    "input_texts": text_content.len()
                }))
            }
            Err(e) => {
                error!("Jina AI processing failed: {}", e);
                Err(e)
            }
        }
    }

    /// Extract URLs from search results
    fn extract_urls_from_results(&self, results: &[serde_json::Value]) -> Vec<String> {
        let mut urls = Vec::new();

        for result in results {
            if let Some(results_array) = result.get("results").and_then(|r| r.as_array()) {
                for item in results_array {
                    if let Some(url) = item.get("url").and_then(|u| u.as_str()) {
                        urls.push(url.to_string());
                    } else if let Some(link) = item.get("link").and_then(|l| l.as_str()) {
                        urls.push(link.to_string());
                    }
                }
            }
        }

        urls
    }

    /// Extract text content from results
    fn extract_text_from_results(&self, results: &[serde_json::Value]) -> Vec<String> {
        let mut text_content = Vec::new();

        for result in results {
            if let Some(content) = result.get("extracted_content").and_then(|c| c.as_array()) {
                for item in content {
                    if let Some(text) = item.get("content").and_then(|t| t.as_str()) {
                        text_content.push(text.to_string());
                    }
                }
            } else if let Some(results_array) = result.get("results").and_then(|r| r.as_array()) {
                for item in results_array {
                    if let Some(snippet) = item.get("snippet").and_then(|s| s.as_str()) {
                        text_content.push(snippet.to_string());
                    } else if let Some(content) = item.get("content").and_then(|c| c.as_str()) {
                        text_content.push(content.to_string());
                    }
                }
            }
        }

        text_content
    }
}
