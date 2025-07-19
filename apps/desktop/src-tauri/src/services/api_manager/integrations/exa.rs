use std::collections::HashMap;
use tracing::{info, debug, warn};
use chrono::Utc;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

use crate::error::{AppResult, ApiError};
use crate::models::api_key::{ServiceProvider, ApiKey};
use crate::services::api_manager::service_integration::{
    ServiceIntegration, ServiceRequest, ServiceResponse, ServiceHealth, ServiceConfig
};

/// Exa search request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExaSearchRequest {
    pub query: String,
    pub num_results: Option<u32>,
    pub include_domains: Option<Vec<String>>,
    pub exclude_domains: Option<Vec<String>>,
    pub start_crawl_date: Option<String>,
    pub end_crawl_date: Option<String>,
    pub start_published_date: Option<String>,
    pub end_published_date: Option<String>,
    pub use_autoprompt: Option<bool>,
}

/// Exa integration
pub struct ExaIntegration {
    config: ServiceConfig,
    http_client: reqwest::Client,
}

impl ExaIntegration {
    pub fn new() -> Self {
        let config = ServiceConfig::default_for_service(ServiceProvider::Exa);
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(config.default_timeout_ms as u64))
            .build()
            .expect("Failed to create HTTP client");

        Self { config, http_client }
    }

    async fn test_api_key_internal(&self, api_key: &ApiKey) -> AppResult<String> {
        debug!("Testing Exa API key");

        let request = ExaSearchRequest {
            query: "test".to_string(),
            num_results: Some(1),
            include_domains: None,
            exclude_domains: None,
            start_crawl_date: None,
            end_crawl_date: None,
            start_published_date: None,
            end_published_date: None,
            use_autoprompt: Some(false),
        };

        let url = format!("{}/search", self.config.base_url);
        let response = self.http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key.encrypted_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| ApiError::external_service_error("Exa".to_string(), e.to_string()))?;

        if response.status().is_success() {
            Ok("API key valid".to_string())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(ApiError::external_service_error("Exa".to_string(), error_text))
        }
    }
}

#[async_trait]
impl ServiceIntegration for ExaIntegration {
    fn service_provider(&self) -> ServiceProvider {
        ServiceProvider::Exa
    }

    async fn make_request(&self, mut request: ServiceRequest, api_key: &ApiKey) -> AppResult<ServiceResponse> {
        debug!("Making Exa request: {}", request.endpoint);

        let start_time = std::time::Instant::now();
        let mut response = ServiceResponse {
            request_id: request.request_id,
            service: ServiceProvider::Exa,
            status_code: 0,
            headers: HashMap::new(),
            body: String::new(),
            response_time_ms: 0,
            success: false,
            error_message: None,
            metadata: HashMap::new(),
            timestamp: Utc::now(),
        };

        self.transform_request(&mut request).await?;

        let url = format!("{}{}", self.config.base_url, request.endpoint);
        let mut req_builder = match request.method.as_str() {
            "GET" => self.http_client.get(&url),
            "POST" => self.http_client.post(&url),
            _ => return Err(ApiError::invalid_configuration("method".to_string(), "Unsupported HTTP method".to_string())),
        };

        req_builder = req_builder.header("Authorization", format!("Bearer {}", api_key.encrypted_key));
        req_builder = req_builder.header("Content-Type", "application/json");

        for (key, value) in &request.headers {
            req_builder = req_builder.header(key, value);
        }

        if let Some(body) = &request.body {
            req_builder = req_builder.body(body.clone());
        }

        match req_builder.send().await {
            Ok(http_response) => {
                response.status_code = http_response.status().as_u16();
                response.success = http_response.status().is_success();

                for (key, value) in http_response.headers() {
                    if let Ok(value_str) = value.to_str() {
                        response.headers.insert(key.to_string(), value_str.to_string());
                    }
                }

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
        match self.test_api_key_internal(api_key).await {
            Ok(_) => Ok(ServiceHealth::Healthy),
            Err(_) => Ok(ServiceHealth::Unhealthy),
        }
    }

    fn get_config(&self) -> &ServiceConfig {
        &self.config
    }

    async fn update_config(&mut self, config: ServiceConfig) -> AppResult<()> {
        self.config = config;
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
        vec!["/search".to_string(), "/contents".to_string(), "/find_similar".to_string()]
    }

    async fn transform_request(&self, _request: &mut ServiceRequest) -> AppResult<()> {
        Ok(())
    }

    async fn transform_response(&self, response: &mut ServiceResponse) -> AppResult<()> {
        if response.success {
            response.metadata.insert("provider".to_string(), "Exa".to_string());
        }
        Ok(())
    }
}
