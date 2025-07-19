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

/// Nick Scamara methodology implementation
/// Uses Firecrawl + AI SDK for professional interface approach with advanced web scraping
pub struct NickScamaraMethodology;

impl NickScamaraMethodology {
    pub fn new() -> Self {
        Self
    }

    /// Create web scraping step using Firecrawl
    fn create_scraping_step(workflow_id: Uuid, query: &str) -> WorkflowStep {
        let mut step = WorkflowStep::new(
            workflow_id,
            1,
            "Web Scraping".to_string(),
            "Scrape and extract content from relevant websites using Firecrawl".to_string(),
        );
        
        step.service_provider = Some("firecrawl".to_string());
        step.endpoint = Some("/scrape".to_string());
        step.input_data.insert("query".to_string(), serde_json::Value::String(query.to_string()));
        step.input_data.insert("formats".to_string(), serde_json::Value::Array(vec![
            serde_json::Value::String("markdown".to_string()),
            serde_json::Value::String("html".to_string()),
        ]));
        step.input_data.insert("only_main_content".to_string(), serde_json::Value::Bool(true));
        
        step
    }

    /// Create content mapping step using Firecrawl
    fn create_mapping_step(workflow_id: Uuid, depends_on: Uuid) -> WorkflowStep {
        let mut step = WorkflowStep::new(
            workflow_id,
            2,
            "Content Mapping".to_string(),
            "Map and discover additional relevant content using Firecrawl".to_string(),
        );
        
        step.service_provider = Some("firecrawl".to_string());
        step.endpoint = Some("/map".to_string());
        step.depends_on.push(depends_on);
        step.input_data.insert("max_depth".to_string(), serde_json::Value::Number(serde_json::Number::from(2)));
        step.input_data.insert("limit".to_string(), serde_json::Value::Number(serde_json::Number::from(50)));
        
        step
    }

    /// Create AI synthesis step using OpenRouter
    fn create_synthesis_step(workflow_id: Uuid, depends_on: Vec<Uuid>) -> WorkflowStep {
        let mut step = WorkflowStep::new(
            workflow_id,
            3,
            "AI Synthesis".to_string(),
            "Synthesize and analyze scraped content using advanced AI models".to_string(),
        );
        
        step.service_provider = Some("openrouter".to_string());
        step.endpoint = Some("/chat/completions".to_string());
        step.depends_on = depends_on;
        step.input_data.insert("model".to_string(), serde_json::Value::String("anthropic/claude-3-sonnet".to_string()));
        step.input_data.insert("temperature".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.2).unwrap()));
        step.input_data.insert("max_tokens".to_string(), serde_json::Value::Number(serde_json::Number::from(6000)));
        
        step
    }

    /// Execute Firecrawl web scraping step
    async fn execute_scraping_step(
        &self,
        step: &mut WorkflowStep,
        context: &ExecutionContext,
        api_manager: &ApiManagerService,
    ) -> AppResult<HashMap<String, serde_json::Value>> {
        debug!("Executing Firecrawl web scraping step");

        let query = context.input_data.get("query")
            .and_then(|v| v.as_str())
            .unwrap_or("research query");

        // First, we need to find relevant URLs to scrape
        // For this example, we'll use a predefined list of authoritative sources
        let urls_to_scrape = self.get_authoritative_urls_for_query(query);

        let mut all_scraped_content = Vec::new();
        let mut successful_scrapes = 0;

        // Scrape each URL
        for url in urls_to_scrape.iter().take(10) { // Limit to 10 URLs
            match self.scrape_single_url(url, api_manager).await {
                Ok(content) => {
                    all_scraped_content.push(content);
                    successful_scrapes += 1;
                }
                Err(e) => {
                    debug!("Failed to scrape {}: {}", url, e);
                }
            }
        }

        if successful_scrapes == 0 {
            return Err(crate::error::ApiError::external_service_error(
                "Firecrawl".to_string(),
                "No URLs could be successfully scraped".to_string(),
            ));
        }

        let mut results = HashMap::new();
        results.insert("scraped_content".to_string(), serde_json::Value::Array(all_scraped_content));
        results.insert("successful_scrapes".to_string(), serde_json::Value::Number(serde_json::Number::from(successful_scrapes)));
        results.insert("total_urls".to_string(), serde_json::Value::Number(serde_json::Number::from(urls_to_scrape.len())));
        results.insert("scraping_method".to_string(), serde_json::Value::String("firecrawl_advanced".to_string()));

        debug!("Firecrawl web scraping completed successfully");
        Ok(results)
    }

    /// Execute Firecrawl content mapping step
    async fn execute_mapping_step(
        &self,
        step: &mut WorkflowStep,
        context: &ExecutionContext,
        api_manager: &ApiManagerService,
    ) -> AppResult<HashMap<String, serde_json::Value>> {
        debug!("Executing Firecrawl content mapping step");

        // Get scraped content from previous step
        let scraped_content = context.shared_data.get("scraped_content")
            .ok_or_else(|| crate::error::ApiError::invalid_operation(
                "No scraped content available for mapping".to_string()
            ))?;

        // Extract base URLs from scraped content for mapping
        let base_urls = self.extract_base_urls_from_content(scraped_content)?;
        
        let mut all_mapped_urls = Vec::new();

        // Map each base URL to discover additional content
        for base_url in base_urls.iter().take(5) { // Limit to 5 base URLs
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
        results.insert("mapping_method".to_string(), serde_json::Value::String("firecrawl_mapping".to_string()));

        debug!("Firecrawl content mapping completed successfully");
        Ok(results)
    }

    /// Execute AI synthesis step
    async fn execute_synthesis_step(
        &self,
        step: &mut WorkflowStep,
        context: &ExecutionContext,
        api_manager: &ApiManagerService,
    ) -> AppResult<HashMap<String, serde_json::Value>> {
        debug!("Executing AI synthesis step");

        // Get scraped content and mapped URLs from previous steps
        let scraped_content = context.shared_data.get("scraped_content")
            .ok_or_else(|| crate::error::ApiError::invalid_operation(
                "No scraped content available for synthesis".to_string()
            ))?;

        let mapped_urls = context.shared_data.get("mapped_urls")
            .ok_or_else(|| crate::error::ApiError::invalid_operation(
                "No mapped URLs available for synthesis".to_string()
            ))?;

        // Create synthesis prompt
        let synthesis_prompt = self.create_synthesis_prompt(scraped_content, mapped_urls)?;

        // Create chat completion request
        let request_body = serde_json::json!({
            "model": "anthropic/claude-3-sonnet",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a professional research analyst specializing in comprehensive content synthesis. Create detailed, well-structured research reports with professional formatting and actionable insights."
                },
                {
                    "role": "user",
                    "content": synthesis_prompt
                }
            ],
            "temperature": 0.2,
            "max_tokens": 6000
        });

        let mut request = ServiceRequest {
            request_id: Uuid::new_v4(),
            service: crate::models::api_key::ServiceProvider::OpenRouter,
            endpoint: "/chat/completions".to_string(),
            method: "POST".to_string(),
            headers: HashMap::new(),
            body: Some(request_body.to_string()),
            timeout_ms: 45000,
            retry_count: 0,
            metadata: HashMap::new(),
        };

        // Make request
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

        // Parse AI response
        let ai_response: serde_json::Value = serde_json::from_str(&response.body)?;
        
        let synthesis_content = ai_response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("Synthesis failed")
            .to_string();

        let mut results = HashMap::new();
        results.insert("synthesis".to_string(), serde_json::Value::String(synthesis_content));
        results.insert("ai_response".to_string(), ai_response);
        results.insert("methodology".to_string(), serde_json::Value::String("nick_scamara".to_string()));

        debug!("AI synthesis completed successfully");
        Ok(results)
    }

    /// Get authoritative URLs for a given query
    fn get_authoritative_urls_for_query(&self, query: &str) -> Vec<String> {
        // This is a simplified implementation
        // In a real system, this would use search APIs or predefined databases
        vec![
            format!("https://en.wikipedia.org/wiki/{}", query.replace(" ", "_")),
            format!("https://scholar.google.com/scholar?q={}", urlencoding::encode(query)),
            format!("https://www.britannica.com/search?query={}", urlencoding::encode(query)),
            format!("https://www.nature.com/search?q={}", urlencoding::encode(query)),
            format!("https://arxiv.org/search/?query={}", urlencoding::encode(query)),
        ]
    }

    /// Scrape a single URL using Firecrawl
    async fn scrape_single_url(
        &self,
        url: &str,
        api_manager: &ApiManagerService,
    ) -> AppResult<serde_json::Value> {
        let request_body = serde_json::json!({
            "url": url,
            "formats": ["markdown", "html"],
            "only_main_content": true,
            "wait_for": 2000
        });

        let mut request = ServiceRequest {
            request_id: Uuid::new_v4(),
            service: crate::models::api_key::ServiceProvider::Firecrawl,
            endpoint: "/scrape".to_string(),
            method: "POST".to_string(),
            headers: HashMap::new(),
            body: Some(request_body.to_string()),
            timeout_ms: 60000,
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
    async fn map_single_url(
        &self,
        url: &str,
        api_manager: &ApiManagerService,
    ) -> AppResult<Vec<String>> {
        let request_body = serde_json::json!({
            "url": url,
            "max_depth": 2,
            "limit": 20
        });

        let mut request = ServiceRequest {
            request_id: Uuid::new_v4(),
            service: crate::models::api_key::ServiceProvider::Firecrawl,
            endpoint: "/map".to_string(),
            method: "POST".to_string(),
            headers: HashMap::new(),
            body: Some(request_body.to_string()),
            timeout_ms: 30000,
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
        
        // Extract URLs from mapping response
        let urls = mapped_data.get("links")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_else(Vec::new);

        Ok(urls)
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

    /// Create synthesis prompt for AI
    fn create_synthesis_prompt(
        &self,
        scraped_content: &serde_json::Value,
        mapped_urls: &serde_json::Value,
    ) -> AppResult<String> {
        let prompt = format!(
            r#"Please synthesize the following research data into a comprehensive, professional report:

SCRAPED CONTENT:
{}

DISCOVERED URLS:
{}

Please provide a detailed research report with:
1. Executive Summary
2. Methodology Overview
3. Key Findings and Insights
4. Detailed Analysis by Topic
5. Source Evaluation and Credibility Assessment
6. Conclusions and Recommendations
7. Future Research Directions
8. Complete Bibliography

Format the response in professional markdown with proper headings, bullet points, and citations."#,
            serde_json::to_string_pretty(scraped_content)?,
            serde_json::to_string_pretty(mapped_urls)?
        );

        Ok(prompt)
    }
}

#[async_trait::async_trait]
impl WorkflowExecutor for NickScamaraMethodology {
    fn methodology(&self) -> ResearchMethodology {
        ResearchMethodology::NickScamara
    }

    async fn prepare_steps(&self, workflow: &mut ResearchWorkflow) -> AppResult<()> {
        info!("Preparing Nick Scamara methodology steps for workflow: {}", workflow.id);

        // Clear existing steps
        workflow.steps.clear();

        // Create workflow steps
        let scraping_step = Self::create_scraping_step(workflow.id, &workflow.query);
        let mapping_step = Self::create_mapping_step(workflow.id, scraping_step.id);
        let synthesis_step = Self::create_synthesis_step(workflow.id, vec![scraping_step.id, mapping_step.id]);

        // Add steps to workflow
        workflow.add_step(scraping_step);
        workflow.add_step(mapping_step);
        workflow.add_step(synthesis_step);

        info!("Prepared {} steps for Nick Scamara methodology", workflow.steps.len());
        Ok(())
    }

    async fn execute_step(
        &self,
        step: &mut WorkflowStep,
        context: &ExecutionContext,
        api_manager: &ApiManagerService,
    ) -> AppResult<HashMap<String, serde_json::Value>> {
        match step.name.as_str() {
            "Web Scraping" => self.execute_scraping_step(step, context, api_manager).await,
            "Content Mapping" => self.execute_mapping_step(step, context, api_manager).await,
            "AI Synthesis" => self.execute_synthesis_step(step, context, api_manager).await,
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
        info!("Post-processing Nick Scamara methodology results");

        // Find synthesis result
        let synthesis_result = step_results.iter()
            .find(|result| result.contains_key("synthesis"))
            .ok_or_else(|| crate::error::ApiError::invalid_operation(
                "No synthesis result found".to_string()
            ))?;

        let content = synthesis_result.get("synthesis")
            .and_then(|v| v.as_str())
            .unwrap_or("Synthesis not available")
            .to_string();

        // Extract sources from mapped URLs
        let sources = step_results.iter()
            .find(|result| result.contains_key("mapped_urls"))
            .and_then(|result| result.get("mapped_urls"))
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_else(Vec::new);

        // Create metadata
        let mut metadata = HashMap::new();
        metadata.insert("methodology".to_string(), serde_json::Value::String("nick_scamara".to_string()));
        metadata.insert("services_used".to_string(), serde_json::Value::Array(vec![
            serde_json::Value::String("firecrawl".to_string()),
            serde_json::Value::String("openrouter".to_string()),
        ]));

        // Calculate actual word count from content
        let word_count = content.split_whitespace().count() as u32;

        let results = ResearchResults {
            content,
            sources,
            metadata,
            word_count,
            source_count: sources.len() as u32,
            methodology_used: ResearchMethodology::NickScamara,
            execution_time_ms: workflow.execution_duration_ms().unwrap_or(0),
        };

        info!("Nick Scamara methodology results processed successfully");
        Ok(results)
    }
}
