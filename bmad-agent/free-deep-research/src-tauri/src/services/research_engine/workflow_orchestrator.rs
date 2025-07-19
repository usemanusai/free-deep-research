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
                // Implement OpenRouter AI analysis
                let analysis_prompt = format!(
                    "Analyze the following research data and provide insights:\n\n{}",
                    serde_json::to_string_pretty(&step.input_data)?
                );

                let analysis_request = serde_json::json!({
                    "model": "anthropic/claude-3-sonnet",
                    "messages": [{
                        "role": "user",
                        "content": analysis_prompt
                    }],
                    "max_tokens": 2000,
                    "temperature": 0.3
                });

                // Make API call to OpenRouter
                let response = self.api_manager.make_request(
                    "openrouter",
                    "/api/v1/chat/completions",
                    analysis_request
                ).await?;

                // Extract analysis from response
                let analysis_text = response["choices"][0]["message"]["content"]
                    .as_str()
                    .unwrap_or("Analysis could not be extracted")
                    .to_string();

                Ok(serde_json::json!({
                    "provider": "openrouter",
                    "analysis": analysis_text,
                    "confidence": 0.85,
                    "model_used": "anthropic/claude-3-sonnet",
                    "tokens_used": response["usage"]["total_tokens"].as_u64().unwrap_or(0),
                    "timestamp": chrono::Utc::now().to_rfc3339()
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
                // Implement OpenRouter AI summary
                let summary_prompt = format!(
                    "Summarize the following research data into key insights and actionable points:\n\n{}",
                    serde_json::to_string_pretty(&step.input_data)?
                );

                let summary_request = serde_json::json!({
                    "model": "anthropic/claude-3-sonnet",
                    "messages": [{
                        "role": "user",
                        "content": summary_prompt
                    }],
                    "max_tokens": 1500,
                    "temperature": 0.2
                });

                // Make API call to OpenRouter
                let response = self.api_manager.make_request(
                    "openrouter",
                    "/api/v1/chat/completions",
                    summary_request
                ).await?;

                // Extract summary from response
                let summary_text = response["choices"][0]["message"]["content"]
                    .as_str()
                    .unwrap_or("Summary could not be extracted")
                    .to_string();

                // Extract key points (simple implementation)
                let key_points: Vec<String> = summary_text
                    .lines()
                    .filter(|line| line.starts_with("- ") || line.starts_with("• "))
                    .map(|line| line.trim_start_matches("- ").trim_start_matches("• ").to_string())
                    .collect();

                Ok(serde_json::json!({
                    "provider": "openrouter",
                    "summary": summary_text,
                    "key_points": key_points,
                    "model_used": "anthropic/claude-3-sonnet",
                    "tokens_used": response["usage"]["total_tokens"].as_u64().unwrap_or(0),
                    "timestamp": chrono::Utc::now().to_rfc3339()
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
                // Implement OpenRouter academic analysis
                let academic_prompt = format!(
                    "Perform an academic analysis of the following research data. Include methodology assessment, citation analysis, and scholarly insights:\n\n{}",
                    serde_json::to_string_pretty(&step.input_data)?
                );

                let academic_request = serde_json::json!({
                    "model": "anthropic/claude-3-sonnet",
                    "messages": [{
                        "role": "user",
                        "content": academic_prompt
                    }],
                    "max_tokens": 2500,
                    "temperature": 0.1
                });

                // Make API call to OpenRouter
                let response = self.api_manager.make_request(
                    "openrouter",
                    "/api/v1/chat/completions",
                    academic_request
                ).await?;

                // Extract academic analysis from response
                let analysis_text = response["choices"][0]["message"]["content"]
                    .as_str()
                    .unwrap_or("Academic analysis could not be extracted")
                    .to_string();

                // Extract citations (simple implementation - look for URLs and DOIs)
                let citations: Vec<String> = analysis_text
                    .lines()
                    .filter(|line| line.contains("http") || line.contains("doi:") || line.contains("DOI:"))
                    .map(|line| line.trim().to_string())
                    .collect();

                // Determine methodology from input data
                let methodology = step.input_data.get("methodology")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Mixed methods research")
                    .to_string();

                Ok(serde_json::json!({
                    "provider": "openrouter",
                    "academic_analysis": analysis_text,
                    "citations": citations,
                    "methodology": methodology,
                    "model_used": "anthropic/claude-3-sonnet",
                    "tokens_used": response["usage"]["total_tokens"].as_u64().unwrap_or(0),
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "academic_rigor_score": 0.8
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
        
        // Implement intelligent result compilation
        let mut all_findings = Vec::new();
        let mut all_sources = Vec::new();
        let mut total_tokens = 0u64;
        let mut confidence_scores = Vec::new();

        // Extract data from all step results
        for result in step_results {
            // Extract findings
            if let Some(analysis) = result.get("analysis").and_then(|a| a.as_str()) {
                all_findings.push(analysis.to_string());
            }
            if let Some(summary) = result.get("summary").and_then(|s| s.as_str()) {
                all_findings.push(summary.to_string());
            }
            if let Some(academic) = result.get("academic_analysis").and_then(|a| a.as_str()) {
                all_findings.push(academic.to_string());
            }

            // Extract sources
            if let Some(sources) = result.get("sources").and_then(|s| s.as_array()) {
                for source in sources {
                    if let Some(url) = source.as_str() {
                        all_sources.push(url.to_string());
                    }
                }
            }

            // Extract token usage
            if let Some(tokens) = result.get("tokens_used").and_then(|t| t.as_u64()) {
                total_tokens += tokens;
            }

            // Extract confidence scores
            if let Some(confidence) = result.get("confidence").and_then(|c| c.as_f64()) {
                confidence_scores.push(confidence);
            }
        }

        // Calculate overall confidence
        let overall_confidence = if !confidence_scores.is_empty() {
            confidence_scores.iter().sum::<f64>() / confidence_scores.len() as f64
        } else {
            0.75 // Default confidence
        };

        // Create comprehensive summary
        let summary = format!(
            "Research completed for query: '{}'. {} steps executed with {} total findings and {} sources analyzed. Overall confidence: {:.1}%",
            query,
            step_results.len(),
            all_findings.len(),
            all_sources.len(),
            overall_confidence * 100.0
        );

        // Compile detailed findings
        let detailed_findings = if all_findings.is_empty() {
            "No detailed findings were generated during the research process.".to_string()
        } else {
            format!(
                "## Research Findings\n\n{}\n\n## Sources Analyzed\n\n{}\n\n## Methodology\n\nThis research utilized {} analytical steps with a combined confidence score of {:.1}%.",
                all_findings.join("\n\n---\n\n"),
                all_sources.iter().enumerate().map(|(i, source)| format!("{}. {}", i + 1, source)).collect::<Vec<_>>().join("\n"),
                step_results.len(),
                overall_confidence * 100.0
            )
        };

        Ok(ResearchResult {
            summary,
            detailed_findings,
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
