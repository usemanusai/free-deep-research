use std::collections::HashMap;
use tracing::{info, debug, warn, error};
use chrono::Utc;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

use crate::error::{AppResult, ApiError};
use crate::models::api_key::{ServiceProvider, ApiKey};
use crate::services::api_manager::service_integration::{
    ServiceIntegration, ServiceRequest, ServiceResponse, ServiceHealth, ServiceConfig
};

/// SerpApi search parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerpApiSearchParams {
    pub q: String,                    // Search query
    pub engine: Option<String>,       // Search engine (google, bing, etc.)
    pub location: Option<String>,     // Location for search
    pub hl: Option<String>,          // Language
    pub gl: Option<String>,          // Country
    pub num: Option<u32>,            // Number of results
    pub start: Option<u32>,          // Starting result number
    pub safe: Option<String>,        // Safe search
    pub tbm: Option<String>,         // Search type (nws for news, isch for images)
    pub tbs: Option<String>,         // Time-based search
}

/// SerpApi search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerpApiResult {
    pub position: Option<u32>,
    pub title: Option<String>,
    pub link: Option<String>,
    pub snippet: Option<String>,
    pub displayed_link: Option<String>,
    pub date: Option<String>,
    pub thumbnail: Option<String>,
}

/// SerpApi response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerpApiResponse {
    pub search_metadata: Option<serde_json::Value>,
    pub search_parameters: Option<serde_json::Value>,
    pub search_information: Option<serde_json::Value>,
    pub organic_results: Option<Vec<SerpApiResult>>,
    pub news_results: Option<Vec<SerpApiResult>>,
    pub images_results: Option<Vec<SerpApiResult>>,
    pub related_searches: Option<Vec<serde_json::Value>>,
    pub pagination: Option<serde_json::Value>,
    pub error: Option<String>,
}

/// SerpApi integration
pub struct SerpApiIntegration {
    config: ServiceConfig,
    http_client: reqwest::Client,
}

impl SerpApiIntegration {
    /// Create a new SerpApi integration
    pub fn new() -> Self {
        let config = ServiceConfig::default_for_service(ServiceProvider::SerpApi);
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(config.default_timeout_ms as u64))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            config,
            http_client,
        }
    }

    /// Perform a Google search
    pub async fn google_search(&self, api_key: &ApiKey, params: SerpApiSearchParams) -> AppResult<SerpApiResponse> {
        debug!("Performing Google search via SerpApi: {}", params.q);

        let mut query_params = vec![
            ("api_key", api_key.encrypted_key.clone()),
            ("q", params.q),
            ("engine", params.engine.unwrap_or_else(|| "google".to_string())),
        ];

        // Add optional parameters
        if let Some(location) = params.location {
            query_params.push(("location", location));
        }
        if let Some(hl) = params.hl {
            query_params.push(("hl", hl));
        }
        if let Some(gl) = params.gl {
            query_params.push(("gl", gl));
        }
        if let Some(num) = params.num {
            query_params.push(("num", num.to_string()));
        }
        if let Some(start) = params.start {
            query_params.push(("start", start.to_string()));
        }
        if let Some(safe) = params.safe {
            query_params.push(("safe", safe));
        }
        if let Some(tbm) = params.tbm {
            query_params.push(("tbm", tbm));
        }
        if let Some(tbs) = params.tbs {
            query_params.push(("tbs", tbs));
        }

        let url = format!("{}/search", self.config.base_url);
        let response = self.http_client
            .get(&url)
            .query(&query_params)
            .send()
            .await
            .map_err(|e| ApiError::external_service_error("SerpApi".to_string(), e.to_string()))?;

        if response.status().is_success() {
            let serpapi_response: SerpApiResponse = response.json().await
                .map_err(|e| ApiError::external_service_error("SerpApi".to_string(), e.to_string()))?;

            if let Some(error) = &serpapi_response.error {
                error!("SerpApi returned error: {}", error);
                return Err(ApiError::external_service_error("SerpApi".to_string(), error.clone()));
            }

            debug!("Google search successful via SerpApi");
            Ok(serpapi_response)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            error!("SerpApi search failed: {}", error_text);
            Err(ApiError::external_service_error("SerpApi".to_string(), error_text))
        }
    }

    /// Get account information
    pub async fn get_account_info(&self, api_key: &ApiKey) -> AppResult<serde_json::Value> {
        debug!("Getting SerpApi account information");

        let url = format!("{}/account", self.config.base_url);
        let response = self.http_client
            .get(&url)
            .query(&[("api_key", &api_key.encrypted_key)])
            .send()
            .await
            .map_err(|e| ApiError::external_service_error("SerpApi".to_string(), e.to_string()))?;

        if response.status().is_success() {
            let account_info: serde_json::Value = response.json().await
                .map_err(|e| ApiError::external_service_error("SerpApi".to_string(), e.to_string()))?;

            debug!("Retrieved SerpApi account information");
            Ok(account_info)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(ApiError::external_service_error("SerpApi".to_string(), error_text))
        }
    }

    /// Test the API key with account info request
    async fn test_api_key_internal(&self, api_key: &ApiKey) -> AppResult<String> {
        debug!("Testing SerpApi API key");

        match self.get_account_info(api_key).await {
            Ok(account_info) => {
                if let Some(searches_left) = account_info.get("searches_left").and_then(|v| v.as_u64()) {
                    Ok(format!("API key valid, {} searches remaining", searches_left))
                } else {
                    Ok("API key valid".to_string())
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Extract search results from response
    pub fn extract_search_results(&self, response: &SerpApiResponse) -> Vec<SerpApiResult> {
        let mut results = Vec::new();

        if let Some(organic_results) = &response.organic_results {
            results.extend(organic_results.clone());
        }

        if let Some(news_results) = &response.news_results {
            results.extend(news_results.clone());
        }

        results
    }
}

#[async_trait]
impl ServiceIntegration for SerpApiIntegration {
    fn service_provider(&self) -> ServiceProvider {
        ServiceProvider::SerpApi
    }

    async fn make_request(&self, mut request: ServiceRequest, api_key: &ApiKey) -> AppResult<ServiceResponse> {
        debug!("Making SerpApi request: {}", request.endpoint);

        let start_time = std::time::Instant::now();
        let mut response = ServiceResponse {
            request_id: request.request_id,
            service: ServiceProvider::SerpApi,
            status_code: 0,
            headers: HashMap::new(),
            body: String::new(),
            response_time_ms: 0,
            success: false,
            error_message: None,
            metadata: HashMap::new(),
            timestamp: Utc::now(),
        };

        // Transform request for SerpApi specifics
        self.transform_request(&mut request).await?;

        let url = format!("{}{}", self.config.base_url, request.endpoint);
        let mut req_builder = match request.method.as_str() {
            "GET" => self.http_client.get(&url),
            "POST" => self.http_client.post(&url),
            _ => return Err(ApiError::invalid_configuration("method".to_string(), "Unsupported HTTP method".to_string())),
        };

        // Add API key to query parameters
        let mut query_params = vec![("api_key", api_key.encrypted_key.clone())];
        
        // Parse existing query parameters from request metadata
        if let Some(query_string) = request.metadata.get("query_params") {
            for param in query_string.split('&') {
                if let Some((key, value)) = param.split_once('=') {
                    query_params.push((key, value.to_string()));
                }
            }
        }

        req_builder = req_builder.query(&query_params);

        // Add headers
        for (key, value) in &request.headers {
            req_builder = req_builder.header(key, value);
        }

        // Add body if present
        if let Some(body) = &request.body {
            req_builder = req_builder.body(body.clone());
        }

        // Make the request
        match req_builder.send().await {
            Ok(http_response) => {
                response.status_code = http_response.status().as_u16();
                response.success = http_response.status().is_success();

                // Extract headers
                for (key, value) in http_response.headers() {
                    if let Ok(value_str) = value.to_str() {
                        response.headers.insert(key.to_string(), value_str.to_string());
                    }
                }

                // Extract body
                match http_response.text().await {
                    Ok(body) => response.body = body,
                    Err(e) => {
                        response.error_message = Some(format!("Failed to read response body: {}", e));
                        response.success = false;
                    }
                }
            }
            Err(e) => {
                response.status_code = 500;
                response.error_message = Some(e.to_string());
                response.success = false;
            }
        }

        response.response_time_ms = start_time.elapsed().as_millis() as u32;
        self.transform_response(&mut response).await?;

        Ok(response)
    }

    async fn health_check(&self, api_key: &ApiKey) -> AppResult<ServiceHealth> {
        debug!("Performing SerpApi health check");

        match self.test_api_key_internal(api_key).await {
            Ok(_) => Ok(ServiceHealth::Healthy),
            Err(e) => {
                warn!("SerpApi health check failed: {}", e);
                Ok(ServiceHealth::Unhealthy)
            }
        }
    }

    fn get_config(&self) -> &ServiceConfig {
        &self.config
    }

    async fn update_config(&mut self, config: ServiceConfig) -> AppResult<()> {
        info!("Updating SerpApi configuration");
        self.config = config;
        
        // Recreate HTTP client with new timeout
        self.http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(self.config.default_timeout_ms as u64))
            .build()
            .map_err(|e| ApiError::invalid_configuration("timeout".to_string(), e.to_string()))?;

        Ok(())
    }

    async fn validate_api_key(&self, api_key: &ApiKey) -> AppResult<bool> {
        match self.test_api_key_internal(api_key).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn get_endpoints(&self) -> Vec<String> {
        vec![
            "/search".to_string(),
            "/account".to_string(),
            "/locations".to_string(),
        ]
    }

    async fn transform_request(&self, _request: &mut ServiceRequest) -> AppResult<()> {
        // SerpApi doesn't need special request transformations
        Ok(())
    }

    async fn transform_response(&self, response: &mut ServiceResponse) -> AppResult<()> {
        // Add SerpApi-specific metadata
        if response.success {
            response.metadata.insert("provider".to_string(), "SerpApi".to_string());
            
            // Try to extract search metadata from response
            if let Ok(parsed_response) = serde_json::from_str::<serde_json::Value>(&response.body) {
                if let Some(search_metadata) = parsed_response.get("search_metadata") {
                    if let Some(total_time) = search_metadata.get("total_time").and_then(|v| v.as_f64()) {
                        response.metadata.insert("search_time".to_string(), total_time.to_string());
                    }
                    if let Some(engine_used) = search_metadata.get("engine").and_then(|v| v.as_str()) {
                        response.metadata.insert("engine".to_string(), engine_used.to_string());
                    }
                }
                
                // Count results
                let mut result_count = 0;
                if let Some(organic_results) = parsed_response.get("organic_results").and_then(|v| v.as_array()) {
                    result_count += organic_results.len();
                }
                if let Some(news_results) = parsed_response.get("news_results").and_then(|v| v.as_array()) {
                    result_count += news_results.len();
                }
                response.metadata.insert("result_count".to_string(), result_count.to_string());
            }
        }

        Ok(())
    }
}
