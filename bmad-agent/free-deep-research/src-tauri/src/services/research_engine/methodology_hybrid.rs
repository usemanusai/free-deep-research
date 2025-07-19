use std::collections::HashMap;
use tracing::{info, debug, error};
use uuid::Uuid;
use serde_json;

use crate::error::AppResult;
use crate::models::research_workflow::{
    ResearchWorkflow, WorkflowStep, ResearchMethodology, ResearchResults
};
use crate::services::api_manager::{ServiceRequest, ServiceResponse, ApiManagerService};
use crate::services::research_engine::workflow_engine::{WorkflowExecutor, ExecutionContext};

/// Hybrid methodology implementation
/// Combines Don Lim (OpenRouter + SerpApi + Jina AI) and Nick Scamara (Firecrawl + AI SDK) approaches
/// for maximum research coverage and accuracy
pub struct HybridMethodology;

impl HybridMethodology {
    pub fn new() -> Self {
        Self
    }

    /// Create initial search step using SerpApi (Don Lim approach)
    fn create_search_step(workflow_id: Uuid, query: &str) -> WorkflowStep {
        let mut step = WorkflowStep::new(
            workflow_id,
            1,
            "Initial Web Search".to_string(),
            "Perform comprehensive web search using SerpApi".to_string(),
        );
        
        step.service_provider = Some("serpapi".to_string());
        step.endpoint = Some("/search".to_string());
        step.input_data.insert("query".to_string(), serde_json::Value::String(query.to_string()));
        step.input_data.insert("num_results".to_string(), serde_json::Value::Number(serde_json::Number::from(30)));
        step.input_data.insert("engine".to_string(), serde_json::Value::String("google".to_string()));
        
        step
    }

    /// Create content scraping step using Firecrawl (Nick Scamara approach)
    fn create_scraping_step(workflow_id: Uuid, depends_on: Uuid) -> WorkflowStep {
        let mut step = WorkflowStep::new(
            workflow_id,
            2,
            "Content Scraping".to_string(),
            "Scrape detailed content from search results using Firecrawl".to_string(),
        );
        
        step.service_provider = Some("firecrawl".to_string());
        step.endpoint = Some("/scrape".to_string());
        step.depends_on.push(depends_on);
        step.input_data.insert("formats".to_string(), serde_json::Value::Array(vec![
            serde_json::Value::String("markdown".to_string()),
        ]));
        step.input_data.insert("only_main_content".to_string(), serde_json::Value::Bool(true));
        
        step
    }

    /// Create content analysis step using Jina AI (Don Lim approach)
    fn create_analysis_step(workflow_id: Uuid, depends_on: Uuid) -> WorkflowStep {
        let mut step = WorkflowStep::new(
            workflow_id,
            3,
            "Content Analysis".to_string(),
            "Analyze and embed content using Jina AI".to_string(),
        );
        
        step.service_provider = Some("jina".to_string());
        step.endpoint = Some("/embeddings".to_string());
        step.depends_on.push(depends_on);
        step.input_data.insert("model".to_string(), serde_json::Value::String("jina-embeddings-v2-base-en".to_string()));
        
        step
    }

    /// Create content mapping step using Firecrawl (Nick Scamara approach)
    fn create_mapping_step(workflow_id: Uuid, depends_on: Uuid) -> WorkflowStep {
        let mut step = WorkflowStep::new(
            workflow_id,
            4,
            "Content Mapping".to_string(),
            "Discover additional relevant content using Firecrawl mapping".to_string(),
        );
        
        step.service_provider = Some("firecrawl".to_string());
        step.endpoint = Some("/map".to_string());
        step.depends_on.push(depends_on);
        step.input_data.insert("max_depth".to_string(), serde_json::Value::Number(serde_json::Number::from(2)));
        step.input_data.insert("limit".to_string(), serde_json::Value::Number(serde_json::Number::from(30)));
        
        step
    }

    /// Create final synthesis step using OpenRouter (Combined approach)
    fn create_synthesis_step(workflow_id: Uuid, depends_on: Vec<Uuid>) -> WorkflowStep {
        let mut step = WorkflowStep::new(
            workflow_id,
            5,
            "Hybrid Synthesis".to_string(),
            "Synthesize all research data using advanced AI analysis".to_string(),
        );
        
        step.service_provider = Some("openrouter".to_string());
        step.endpoint = Some("/chat/completions".to_string());
        step.depends_on = depends_on;
        step.input_data.insert("model".to_string(), serde_json::Value::String("anthropic/claude-3-sonnet".to_string()));
        step.input_data.insert("temperature".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.25).unwrap()));
        step.input_data.insert("max_tokens".to_string(), serde_json::Value::Number(serde_json::Number::from(8000)));
        
        step
    }

    /// Execute SerpApi search step
    async fn execute_search_step(
        &self,
        step: &mut WorkflowStep,
        context: &ExecutionContext,
        api_manager: &ApiManagerService,
    ) -> AppResult<HashMap<String, serde_json::Value>> {
        debug!("Executing hybrid SerpApi search step");

        let query = context.input_data.get("query")
            .and_then(|v| v.as_str())
            .unwrap_or("research query");

        // Create service request
        let mut request = ServiceRequest {
            request_id: Uuid::new_v4(),
            service: crate::models::api_key::ServiceProvider::SerpApi,
            endpoint: "/search".to_string(),
            method: "GET".to_string(),
            headers: HashMap::new(),
            body: None,
            timeout_ms: 15000,
            retry_count: 0,
            metadata: HashMap::new(),
        };

        // Add query parameters
        let query_params = format!("q={}&engine=google&num=30", urlencoding::encode(query));
        request.metadata.insert("query_params".to_string(), query_params);

        let response = api_manager.make_service_request(
            crate::models::api_key::ServiceProvider::SerpApi,
            request,
        ).await?;

        if !response.success {
            return Err(crate::error::ApiError::external_service_error(
                "SerpApi".to_string(),
                response.error_message.unwrap_or_default(),
            ));
        }

        let search_results: serde_json::Value = serde_json::from_str(&response.body)?;
        
        let mut results = HashMap::new();
        results.insert("search_results".to_string(), search_results);
        results.insert("search_query".to_string(), serde_json::Value::String(query.to_string()));
        results.insert("methodology_step".to_string(), serde_json::Value::String("hybrid_search".to_string()));

        debug!("Hybrid SerpApi search completed successfully");
        Ok(results)
    }

    /// Execute Firecrawl scraping step
    async fn execute_scraping_step(
        &self,
        step: &mut WorkflowStep,
        context: &ExecutionContext,
        api_manager: &ApiManagerService,
    ) -> AppResult<HashMap<String, serde_json::Value>> {
        debug!("Executing hybrid Firecrawl scraping step");

        // Get search results from previous step
        let search_results = context.shared_data.get("search_results")
            .ok_or_else(|| crate::error::ApiError::invalid_operation(
                "No search results available for scraping".to_string()
            ))?;

        // Extract top URLs from search results
        let urls = self.extract_top_urls_from_search_results(search_results, 15)?;
        
        let mut all_scraped_content = Vec::new();
        let mut successful_scrapes = 0;

        // Scrape each URL
        for url in urls {
            match self.scrape_single_url(&url, api_manager).await {
                Ok(content) => {
                    all_scraped_content.push(content);
                    successful_scrapes += 1;
                }
                Err(e) => {
                    debug!("Failed to scrape {}: {}", url, e);
                }
            }
        }

        let mut results = HashMap::new();
        results.insert("scraped_content".to_string(), serde_json::Value::Array(all_scraped_content));
        results.insert("successful_scrapes".to_string(), serde_json::Value::Number(serde_json::Number::from(successful_scrapes)));
        results.insert("methodology_step".to_string(), serde_json::Value::String("hybrid_scraping".to_string()));

        debug!("Hybrid Firecrawl scraping completed successfully");
        Ok(results)
    }

    /// Execute Jina AI analysis step
    async fn execute_analysis_step(
        &self,
        step: &mut WorkflowStep,
        context: &ExecutionContext,
        api_manager: &ApiManagerService,
    ) -> AppResult<HashMap<String, serde_json::Value>> {
        debug!("Executing hybrid Jina AI analysis step");

        // Get scraped content from previous step
        let scraped_content = context.shared_data.get("scraped_content")
            .ok_or_else(|| crate::error::ApiError::invalid_operation(
                "No scraped content available for analysis".to_string()
            ))?;

        // Extract text content for embedding
        let text_content = self.extract_text_from_scraped_content(scraped_content)?;
        
        if text_content.is_empty() {
            return Err(crate::error::ApiError::invalid_operation(
                "No text content available for embedding".to_string()
            ));
        }

        // Create embeddings request
        let request_body = serde_json::json!({
            "input": text_content,
            "model": "jina-embeddings-v2-base-en"
        });

        let mut request = ServiceRequest {
            request_id: Uuid::new_v4(),
            service: crate::models::api_key::ServiceProvider::Jina,
            endpoint: "/embeddings".to_string(),
            method: "POST".to_string(),
            headers: HashMap::new(),
            body: Some(request_body.to_string()),
            timeout_ms: 25000,
            retry_count: 0,
            metadata: HashMap::new(),
        };

        let response = api_manager.make_service_request(
            crate::models::api_key::ServiceProvider::Jina,
            request,
        ).await?;

        if !response.success {
            return Err(crate::error::ApiError::external_service_error(
                "Jina".to_string(),
                response.error_message.unwrap_or_default(),
            ));
        }

        let embeddings_response: serde_json::Value = serde_json::from_str(&response.body)?;
        
        let mut results = HashMap::new();
        results.insert("embeddings".to_string(), embeddings_response);
        results.insert("analyzed_content_count".to_string(), serde_json::Value::Number(serde_json::Number::from(text_content.len())));
        results.insert("methodology_step".to_string(), serde_json::Value::String("hybrid_analysis".to_string()));

        debug!("Hybrid Jina AI analysis completed successfully");
        Ok(results)
    }

    /// Execute Firecrawl mapping step
    async fn execute_mapping_step(
        &self,
        step: &mut WorkflowStep,
        context: &ExecutionContext,
        api_manager: &ApiManagerService,
    ) -> AppResult<HashMap<String, serde_json::Value>> {
        debug!("Executing hybrid Firecrawl mapping step");

        // Get scraped content to extract base URLs
        let scraped_content = context.shared_data.get("scraped_content")
            .ok_or_else(|| crate::error::ApiError::invalid_operation(
                "No scraped content available for mapping".to_string()
            ))?;

        let base_urls = self.extract_base_urls_from_content(scraped_content)?;
        let mut all_mapped_urls = Vec::new();

        // Map each base URL
        for base_url in base_urls.iter().take(5) {
            match self.map_single_url(base_url, api_manager).await {
                Ok(mapped_urls) => {
                    all_mapped_urls.extend(mapped_urls);
                }
                Err(e) => {
                    debug!("Failed to map {}: {}", base_url, e);
                }
            }
        }

        let mut results = HashMap::new();
        results.insert("mapped_urls".to_string(), serde_json::Value::Array(
            all_mapped_urls.into_iter().map(serde_json::Value::String).collect()
        ));
        results.insert("base_urls_mapped".to_string(), serde_json::Value::Number(serde_json::Number::from(base_urls.len())));
        results.insert("methodology_step".to_string(), serde_json::Value::String("hybrid_mapping".to_string()));

        debug!("Hybrid Firecrawl mapping completed successfully");
        Ok(results)
    }

    /// Execute hybrid synthesis step
    async fn execute_synthesis_step(
        &self,
        step: &mut WorkflowStep,
        context: &ExecutionContext,
        api_manager: &ApiManagerService,
    ) -> AppResult<HashMap<String, serde_json::Value>> {
        debug!("Executing hybrid synthesis step");

        // Gather all data from previous steps
        let search_results = context.shared_data.get("search_results");
        let scraped_content = context.shared_data.get("scraped_content");
        let embeddings = context.shared_data.get("embeddings");
        let mapped_urls = context.shared_data.get("mapped_urls");

        // Create comprehensive synthesis prompt
        let synthesis_prompt = self.create_hybrid_synthesis_prompt(
            search_results,
            scraped_content,
            embeddings,
            mapped_urls,
        )?;

        // Create chat completion request
        let request_body = serde_json::json!({
            "model": "anthropic/claude-3-sonnet",
            "messages": [
                {
                    "role": "system",
                    "content": "You are an expert research analyst specializing in comprehensive multi-source research synthesis. You excel at combining data from multiple methodologies (search engines, web scraping, content analysis, and content mapping) to create authoritative, well-structured research reports."
                },
                {
                    "role": "user",
                    "content": synthesis_prompt
                }
            ],
            "temperature": 0.25,
            "max_tokens": 8000
        });

        let mut request = ServiceRequest {
            request_id: Uuid::new_v4(),
            service: crate::models::api_key::ServiceProvider::OpenRouter,
            endpoint: "/chat/completions".to_string(),
            method: "POST".to_string(),
            headers: HashMap::new(),
            body: Some(request_body.to_string()),
            timeout_ms: 60000,
            retry_count: 0,
            metadata: HashMap::new(),
        };

        let response = api_manager.make_service_request(
            crate::models::api_key::ServiceProvider::OpenRouter,
            request,
        ).await?;

        if !response.success {
            return Err(crate::error::ApiError::external_service_error(
                "OpenRouter".to_string(),
                response.error_message.unwrap_or_default(),
            ));
        }

        let ai_response: serde_json::Value = serde_json::from_str(&response.body)?;
        
        let synthesis_content = ai_response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("Hybrid synthesis failed")
            .to_string();

        let mut results = HashMap::new();
        results.insert("hybrid_synthesis".to_string(), serde_json::Value::String(synthesis_content));
        results.insert("ai_response".to_string(), ai_response);
        results.insert("methodology".to_string(), serde_json::Value::String("hybrid".to_string()));

        debug!("Hybrid synthesis completed successfully");
        Ok(results)
    }

    /// Extract top URLs from search results
    fn extract_top_urls_from_search_results(&self, search_results: &serde_json::Value, limit: usize) -> AppResult<Vec<String>> {
        let mut urls = Vec::new();

        // Extract from organic results
        if let Some(organic_results) = search_results.get("organic_results").and_then(|v| v.as_array()) {
            for result in organic_results.iter().take(limit) {
                if let Some(link) = result.get("link").and_then(|v| v.as_str()) {
                    urls.push(link.to_string());
                }
            }
        }

        Ok(urls)
    }

    /// Extract text content from scraped content
    fn extract_text_from_scraped_content(&self, scraped_content: &serde_json::Value) -> AppResult<Vec<String>> {
        let mut text_content = Vec::new();

        if let Some(content_array) = scraped_content.as_array() {
            for item in content_array {
                if let Some(markdown) = item.get("markdown").and_then(|v| v.as_str()) {
                    // Take first 1000 characters to avoid token limits
                    let truncated = if markdown.len() > 1000 {
                        &markdown[..1000]
                    } else {
                        markdown
                    };
                    text_content.push(truncated.to_string());
                }
            }
        }

        Ok(text_content)
    }

    /// Extract base URLs from scraped content
    fn extract_base_urls_from_content(&self, content: &serde_json::Value) -> AppResult<Vec<String>> {
        let mut base_urls = Vec::new();

        if let Some(content_array) = content.as_array() {
            for item in content_array {
                if let Some(url) = item.get("url").and_then(|v| v.as_str()) {
                    if let Ok(parsed_url) = url::Url::parse(url) {
                        let base_url = format!("{}://{}", parsed_url.scheme(), parsed_url.host_str().unwrap_or(""));
                        if !base_urls.contains(&base_url) {
                            base_urls.push(base_url);
                        }
                    }
                }
            }
        }

        Ok(base_urls)
    }

    /// Scrape a single URL using Firecrawl
    async fn scrape_single_url(&self, url: &str, api_manager: &ApiManagerService) -> AppResult<serde_json::Value> {
        let request_body = serde_json::json!({
            "url": url,
            "formats": ["markdown"],
            "only_main_content": true,
            "wait_for": 1000
        });

        let mut request = ServiceRequest {
            request_id: Uuid::new_v4(),
            service: crate::models::api_key::ServiceProvider::Firecrawl,
            endpoint: "/scrape".to_string(),
            method: "POST".to_string(),
            headers: HashMap::new(),
            body: Some(request_body.to_string()),
            timeout_ms: 45000,
            retry_count: 0,
            metadata: HashMap::new(),
        };

        let response = api_manager.make_service_request(
            crate::models::api_key::ServiceProvider::Firecrawl,
            request,
        ).await?;

        if !response.success {
            return Err(crate::error::ApiError::external_service_error(
                "Firecrawl".to_string(),
                response.error_message.unwrap_or_default(),
            ));
        }

        let scraped_data: serde_json::Value = serde_json::from_str(&response.body)?;
        Ok(scraped_data)
    }

    /// Map a single URL using Firecrawl
    async fn map_single_url(&self, url: &str, api_manager: &ApiManagerService) -> AppResult<Vec<String>> {
        let request_body = serde_json::json!({
            "url": url,
            "max_depth": 2,
            "limit": 15
        });

        let mut request = ServiceRequest {
            request_id: Uuid::new_v4(),
            service: crate::models::api_key::ServiceProvider::Firecrawl,
            endpoint: "/map".to_string(),
            method: "POST".to_string(),
            headers: HashMap::new(),
            body: Some(request_body.to_string()),
            timeout_ms: 25000,
            retry_count: 0,
            metadata: HashMap::new(),
        };

        let response = api_manager.make_service_request(
            crate::models::api_key::ServiceProvider::Firecrawl,
            request,
        ).await?;

        if !response.success {
            return Err(crate::error::ApiError::external_service_error(
                "Firecrawl".to_string(),
                response.error_message.unwrap_or_default(),
            ));
        }

        let mapped_data: serde_json::Value = serde_json::from_str(&response.body)?;
        
        let urls = mapped_data.get("links")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_else(Vec::new);

        Ok(urls)
    }

    /// Create comprehensive hybrid synthesis prompt
    fn create_hybrid_synthesis_prompt(
        &self,
        search_results: Option<&serde_json::Value>,
        scraped_content: Option<&serde_json::Value>,
        embeddings: Option<&serde_json::Value>,
        mapped_urls: Option<&serde_json::Value>,
    ) -> AppResult<String> {
        let mut prompt = String::from(
            r#"Please create a comprehensive research report by synthesizing data from multiple research methodologies:

METHODOLOGY OVERVIEW:
This research combines the Don Lim approach (cost-optimized using SerpApi + Jina AI + OpenRouter) with the Nick Scamara approach (professional interface using Firecrawl + AI SDK) for maximum coverage and accuracy.

"#
        );

        if let Some(search_data) = search_results {
            prompt.push_str(&format!(
                "SEARCH RESULTS (SerpApi):\n{}\n\n",
                serde_json::to_string_pretty(search_data)?
            ));
        }

        if let Some(scraped_data) = scraped_content {
            prompt.push_str(&format!(
                "SCRAPED CONTENT (Firecrawl):\n{}\n\n",
                serde_json::to_string_pretty(scraped_data)?
            ));
        }

        if let Some(embeddings_data) = embeddings {
            prompt.push_str(&format!(
                "CONTENT ANALYSIS (Jina AI):\n{}\n\n",
                serde_json::to_string_pretty(embeddings_data)?
            ));
        }

        if let Some(mapped_data) = mapped_urls {
            prompt.push_str(&format!(
                "MAPPED CONTENT (Firecrawl):\n{}\n\n",
                serde_json::to_string_pretty(mapped_data)?
            ));
        }

        prompt.push_str(
            r#"Please provide a comprehensive research report with:

1. **Executive Summary** - Key findings and insights
2. **Methodology Analysis** - How the hybrid approach enhanced research quality
3. **Comprehensive Findings** - Detailed analysis of all data sources
4. **Cross-Validation** - How different sources confirm or contradict each other
5. **Source Quality Assessment** - Evaluation of source credibility and relevance
6. **Detailed Analysis by Topic** - Organized thematic analysis
7. **Conclusions and Recommendations** - Actionable insights and next steps
8. **Future Research Directions** - Suggested areas for deeper investigation
9. **Complete Bibliography** - All sources with quality ratings

Format in professional markdown with proper headings, citations, and visual elements."#
        );

        Ok(prompt)
    }
}

#[async_trait::async_trait]
impl WorkflowExecutor for HybridMethodology {
    fn methodology(&self) -> ResearchMethodology {
        ResearchMethodology::Hybrid
    }

    async fn prepare_steps(&self, workflow: &mut ResearchWorkflow) -> AppResult<()> {
        info!("Preparing Hybrid methodology steps for workflow: {}", workflow.id);

        // Clear existing steps
        workflow.steps.clear();

        // Create workflow steps
        let search_step = Self::create_search_step(workflow.id, &workflow.query);
        let scraping_step = Self::create_scraping_step(workflow.id, search_step.id);
        let analysis_step = Self::create_analysis_step(workflow.id, scraping_step.id);
        let mapping_step = Self::create_mapping_step(workflow.id, scraping_step.id);
        let synthesis_step = Self::create_synthesis_step(workflow.id, vec![
            search_step.id,
            scraping_step.id,
            analysis_step.id,
            mapping_step.id,
        ]);

        // Add steps to workflow
        workflow.add_step(search_step);
        workflow.add_step(scraping_step);
        workflow.add_step(analysis_step);
        workflow.add_step(mapping_step);
        workflow.add_step(synthesis_step);

        info!("Prepared {} steps for Hybrid methodology", workflow.steps.len());
        Ok(())
    }

    async fn execute_step(
        &self,
        step: &mut WorkflowStep,
        context: &ExecutionContext,
        api_manager: &ApiManagerService,
    ) -> AppResult<HashMap<String, serde_json::Value>> {
        match step.name.as_str() {
            "Initial Web Search" => self.execute_search_step(step, context, api_manager).await,
            "Content Scraping" => self.execute_scraping_step(step, context, api_manager).await,
            "Content Analysis" => self.execute_analysis_step(step, context, api_manager).await,
            "Content Mapping" => self.execute_mapping_step(step, context, api_manager).await,
            "Hybrid Synthesis" => self.execute_synthesis_step(step, context, api_manager).await,
            _ => Err(crate::error::ApiError::invalid_operation(
                format!("Unknown step type: {}", step.name)
            )),
        }
    }

    async fn post_process_results(
        &self,
        workflow: &ResearchWorkflow,
        step_results: &[HashMap<String, serde_json::Value>],
    ) -> AppResult<ResearchResults> {
        info!("Post-processing Hybrid methodology results");

        // Find synthesis result
        let synthesis_result = step_results.iter()
            .find(|result| result.contains_key("hybrid_synthesis"))
            .ok_or_else(|| crate::error::ApiError::invalid_operation(
                "No hybrid synthesis result found".to_string()
            ))?;

        let content = synthesis_result.get("hybrid_synthesis")
            .and_then(|v| v.as_str())
            .unwrap_or("Hybrid synthesis not available")
            .to_string();

        // Collect sources from all steps
        let mut sources = Vec::new();
        
        // Add scraped URLs
        if let Some(scraped_result) = step_results.iter().find(|r| r.contains_key("scraped_content")) {
            if let Some(scraped_content) = scraped_result.get("scraped_content").and_then(|v| v.as_array()) {
                for item in scraped_content {
                    if let Some(url) = item.get("url").and_then(|v| v.as_str()) {
                        sources.push(url.to_string());
                    }
                }
            }
        }

        // Add mapped URLs
        if let Some(mapped_result) = step_results.iter().find(|r| r.contains_key("mapped_urls")) {
            if let Some(mapped_urls) = mapped_result.get("mapped_urls").and_then(|v| v.as_array()) {
                for url in mapped_urls {
                    if let Some(url_str) = url.as_str() {
                        sources.push(url_str.to_string());
                    }
                }
            }
        }

        // Create metadata
        let mut metadata = HashMap::new();
        metadata.insert("methodology".to_string(), serde_json::Value::String("hybrid".to_string()));
        metadata.insert("services_used".to_string(), serde_json::Value::Array(vec![
            serde_json::Value::String("serpapi".to_string()),
            serde_json::Value::String("firecrawl".to_string()),
            serde_json::Value::String("jina".to_string()),
            serde_json::Value::String("openrouter".to_string()),
        ]));
        metadata.insert("approach".to_string(), serde_json::Value::String("don_lim_nick_scamara_hybrid".to_string()));

        // Calculate actual word count from content
        let word_count = content.split_whitespace().count() as u32;

        let results = ResearchResults {
            content,
            sources,
            metadata,
            word_count,
            source_count: sources.len() as u32,
            methodology_used: ResearchMethodology::Hybrid,
            execution_time_ms: workflow.execution_duration_ms().unwrap_or(0),
        };

        info!("Hybrid methodology results processed successfully");
        Ok(results)
    }
}
