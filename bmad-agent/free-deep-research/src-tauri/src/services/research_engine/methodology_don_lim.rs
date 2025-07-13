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

/// Don Lim methodology implementation
/// Uses OpenRouter.ai + SerpApi + Jina AI for cost-optimized comprehensive research
pub struct DonLimMethodology;

impl DonLimMethodology {
    pub fn new() -> Self {
        Self
    }

    /// Create search step using SerpApi
    fn create_search_step(workflow_id: Uuid, query: &str) -> WorkflowStep {
        let mut step = WorkflowStep::new(
            workflow_id,
            1,
            "Web Search".to_string(),
            "Search for relevant information using SerpApi".to_string(),
        );
        
        step.service_provider = Some("serpapi".to_string());
        step.endpoint = Some("/search".to_string());
        step.input_data.insert("query".to_string(), serde_json::Value::String(query.to_string()));
        step.input_data.insert("num_results".to_string(), serde_json::Value::Number(serde_json::Number::from(20)));
        step.input_data.insert("engine".to_string(), serde_json::Value::String("google".to_string()));
        
        step
    }

    /// Create content extraction step using Jina AI
    fn create_extraction_step(workflow_id: Uuid, depends_on: Uuid) -> WorkflowStep {
        let mut step = WorkflowStep::new(
            workflow_id,
            2,
            "Content Extraction".to_string(),
            "Extract and process content from search results using Jina AI".to_string(),
        );
        
        step.service_provider = Some("jina".to_string());
        step.endpoint = Some("/embeddings".to_string());
        step.depends_on.push(depends_on);
        step.input_data.insert("model".to_string(), serde_json::Value::String("jina-embeddings-v2-base-en".to_string()));
        
        step
    }

    /// Create AI analysis step using OpenRouter
    fn create_analysis_step(workflow_id: Uuid, depends_on: Vec<Uuid>) -> WorkflowStep {
        let mut step = WorkflowStep::new(
            workflow_id,
            3,
            "AI Analysis".to_string(),
            "Analyze and synthesize research findings using OpenRouter AI".to_string(),
        );
        
        step.service_provider = Some("openrouter".to_string());
        step.endpoint = Some("/chat/completions".to_string());
        step.depends_on = depends_on;
        step.input_data.insert("model".to_string(), serde_json::Value::String("anthropic/claude-3-sonnet".to_string()));
        step.input_data.insert("temperature".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.3).unwrap()));
        step.input_data.insert("max_tokens".to_string(), serde_json::Value::Number(serde_json::Number::from(4000)));
        
        step
    }

    /// Execute SerpApi search step
    async fn execute_search_step(
        &self,
        step: &mut WorkflowStep,
        context: &ExecutionContext,
        api_manager: &ApiManagerService,
    ) -> AppResult<HashMap<String, serde_json::Value>> {
        debug!("Executing SerpApi search step");

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

        // Add query parameters to metadata
        let query_params = format!("q={}&engine=google&num=20", urlencoding::encode(query));
        request.metadata.insert("query_params".to_string(), query_params);

        // Make request
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

        // Parse search results
        let search_results: serde_json::Value = serde_json::from_str(&response.body)?;
        
        let mut results = HashMap::new();
        results.insert("search_results".to_string(), search_results);
        results.insert("result_count".to_string(), serde_json::Value::Number(serde_json::Number::from(20)));
        results.insert("search_query".to_string(), serde_json::Value::String(query.to_string()));

        debug!("SerpApi search completed successfully");
        Ok(results)
    }

    /// Execute Jina AI content extraction step
    async fn execute_extraction_step(
        &self,
        step: &mut WorkflowStep,
        context: &ExecutionContext,
        api_manager: &ApiManagerService,
    ) -> AppResult<HashMap<String, serde_json::Value>> {
        debug!("Executing Jina AI content extraction step");

        // Get search results from previous step
        let search_results = context.shared_data.get("search_results")
            .ok_or_else(|| crate::error::ApiError::invalid_operation(
                "No search results available for content extraction".to_string()
            ))?;

        // Extract URLs from search results
        let urls = self.extract_urls_from_search_results(search_results)?;
        
        if urls.is_empty() {
            return Err(crate::error::ApiError::invalid_operation(
                "No URLs found in search results".to_string()
            ));
        }

        // Take first 10 URLs for processing
        let urls_to_process: Vec<String> = urls.into_iter().take(10).collect();
        
        // Create embeddings request
        let request_body = serde_json::json!({
            "input": urls_to_process,
            "model": "jina-embeddings-v2-base-en"
        });

        let mut request = ServiceRequest {
            request_id: Uuid::new_v4(),
            service: crate::models::api_key::ServiceProvider::Jina,
            endpoint: "/embeddings".to_string(),
            method: "POST".to_string(),
            headers: HashMap::new(),
            body: Some(request_body.to_string()),
            timeout_ms: 20000,
            retry_count: 0,
            metadata: HashMap::new(),
        };

        // Make request
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

        // Parse embeddings response
        let embeddings_response: serde_json::Value = serde_json::from_str(&response.body)?;
        
        let mut results = HashMap::new();
        results.insert("embeddings".to_string(), embeddings_response);
        results.insert("processed_urls".to_string(), serde_json::Value::Array(
            urls_to_process.into_iter().map(serde_json::Value::String).collect()
        ));
        results.insert("extraction_method".to_string(), serde_json::Value::String("jina_embeddings".to_string()));

        debug!("Jina AI content extraction completed successfully");
        Ok(results)
    }

    /// Execute OpenRouter AI analysis step
    async fn execute_analysis_step(
        &self,
        step: &mut WorkflowStep,
        context: &ExecutionContext,
        api_manager: &ApiManagerService,
    ) -> AppResult<HashMap<String, serde_json::Value>> {
        debug!("Executing OpenRouter AI analysis step");

        // Get search results and embeddings from previous steps
        let search_results = context.shared_data.get("search_results")
            .ok_or_else(|| crate::error::ApiError::invalid_operation(
                "No search results available for analysis".to_string()
            ))?;

        let embeddings = context.shared_data.get("embeddings")
            .ok_or_else(|| crate::error::ApiError::invalid_operation(
                "No embeddings available for analysis".to_string()
            ))?;

        // Create analysis prompt
        let analysis_prompt = self.create_analysis_prompt(search_results, embeddings)?;

        // Create chat completion request
        let request_body = serde_json::json!({
            "model": "anthropic/claude-3-sonnet",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a research analyst. Analyze the provided search results and embeddings to create a comprehensive research report. Focus on accuracy, relevance, and actionable insights."
                },
                {
                    "role": "user",
                    "content": analysis_prompt
                }
            ],
            "temperature": 0.3,
            "max_tokens": 4000
        });

        let mut request = ServiceRequest {
            request_id: Uuid::new_v4(),
            service: crate::models::api_key::ServiceProvider::OpenRouter,
            endpoint: "/chat/completions".to_string(),
            method: "POST".to_string(),
            headers: HashMap::new(),
            body: Some(request_body.to_string()),
            timeout_ms: 30000,
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
        
        let analysis_content = ai_response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("Analysis failed")
            .to_string();

        let mut results = HashMap::new();
        results.insert("analysis".to_string(), serde_json::Value::String(analysis_content));
        results.insert("ai_response".to_string(), ai_response);
        results.insert("methodology".to_string(), serde_json::Value::String("don_lim".to_string()));

        debug!("OpenRouter AI analysis completed successfully");
        Ok(results)
    }

    /// Extract URLs from SerpApi search results
    fn extract_urls_from_search_results(&self, search_results: &serde_json::Value) -> AppResult<Vec<String>> {
        let mut urls = Vec::new();

        // Extract from organic results
        if let Some(organic_results) = search_results.get("organic_results").and_then(|v| v.as_array()) {
            for result in organic_results {
                if let Some(link) = result.get("link").and_then(|v| v.as_str()) {
                    urls.push(link.to_string());
                }
            }
        }

        // Extract from news results
        if let Some(news_results) = search_results.get("news_results").and_then(|v| v.as_array()) {
            for result in news_results {
                if let Some(link) = result.get("link").and_then(|v| v.as_str()) {
                    urls.push(link.to_string());
                }
            }
        }

        Ok(urls)
    }

    /// Create analysis prompt for AI
    fn create_analysis_prompt(
        &self,
        search_results: &serde_json::Value,
        embeddings: &serde_json::Value,
    ) -> AppResult<String> {
        let prompt = format!(
            r#"Please analyze the following research data and provide a comprehensive report:

SEARCH RESULTS:
{}

CONTENT EMBEDDINGS:
{}

Please provide:
1. Executive Summary
2. Key Findings
3. Detailed Analysis
4. Sources and References
5. Conclusions and Recommendations

Format the response in clear, well-structured markdown."#,
            serde_json::to_string_pretty(search_results)?,
            serde_json::to_string_pretty(embeddings)?
        );

        Ok(prompt)
    }
}

#[async_trait::async_trait]
impl WorkflowExecutor for DonLimMethodology {
    fn methodology(&self) -> ResearchMethodology {
        ResearchMethodology::DonLim
    }

    async fn prepare_steps(&self, workflow: &mut ResearchWorkflow) -> AppResult<()> {
        info!("Preparing Don Lim methodology steps for workflow: {}", workflow.id);

        // Clear existing steps
        workflow.steps.clear();

        // Create workflow steps
        let search_step = Self::create_search_step(workflow.id, &workflow.query);
        let extraction_step = Self::create_extraction_step(workflow.id, search_step.id);
        let analysis_step = Self::create_analysis_step(workflow.id, vec![search_step.id, extraction_step.id]);

        // Add steps to workflow
        workflow.add_step(search_step);
        workflow.add_step(extraction_step);
        workflow.add_step(analysis_step);

        info!("Prepared {} steps for Don Lim methodology", workflow.steps.len());
        Ok(())
    }

    async fn execute_step(
        &self,
        step: &mut WorkflowStep,
        context: &ExecutionContext,
        api_manager: &ApiManagerService,
    ) -> AppResult<HashMap<String, serde_json::Value>> {
        match step.name.as_str() {
            "Web Search" => self.execute_search_step(step, context, api_manager).await,
            "Content Extraction" => self.execute_extraction_step(step, context, api_manager).await,
            "AI Analysis" => self.execute_analysis_step(step, context, api_manager).await,
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
        info!("Post-processing Don Lim methodology results");

        // Find analysis result
        let analysis_result = step_results.iter()
            .find(|result| result.contains_key("analysis"))
            .ok_or_else(|| crate::error::ApiError::invalid_operation(
                "No analysis result found".to_string()
            ))?;

        let content = analysis_result.get("analysis")
            .and_then(|v| v.as_str())
            .unwrap_or("Analysis not available")
            .to_string();

        // Extract sources from search results
        let sources = step_results.iter()
            .find(|result| result.contains_key("processed_urls"))
            .and_then(|result| result.get("processed_urls"))
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_else(Vec::new);

        // Create metadata
        let mut metadata = HashMap::new();
        metadata.insert("methodology".to_string(), serde_json::Value::String("don_lim".to_string()));
        metadata.insert("services_used".to_string(), serde_json::Value::Array(vec![
            serde_json::Value::String("serpapi".to_string()),
            serde_json::Value::String("jina".to_string()),
            serde_json::Value::String("openrouter".to_string()),
        ]));

        let results = ResearchResults {
            content,
            sources,
            metadata,
            word_count: 0, // TODO: Calculate actual word count
            source_count: sources.len() as u32,
            methodology_used: ResearchMethodology::DonLim,
            execution_time_ms: workflow.execution_duration_ms().unwrap_or(0),
        };

        info!("Don Lim methodology results processed successfully");
        Ok(results)
    }
}
