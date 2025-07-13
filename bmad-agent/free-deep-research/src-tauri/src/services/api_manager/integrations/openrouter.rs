use std::collections::HashMap;
use tracing::{info, debug, warn, error};
use chrono::Utc;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

use crate::error::{AppResult, ApiError};
use crate::models::api_key::{ServiceProvider, ApiKey};
use crate::services::api_manager::service_integration::{
    ServiceIntegration, ServiceRequest, ServiceResponse, ServiceHealth, ServiceConfig
};

/// OpenRouter.ai specific request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRouterRequest {
    pub model: String,
    pub messages: Vec<OpenRouterMessage>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub stream: Option<bool>,
}

/// OpenRouter.ai message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRouterMessage {
    pub role: String,
    pub content: String,
}

/// OpenRouter.ai response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRouterResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<OpenRouterChoice>,
    pub usage: OpenRouterUsage,
}

/// OpenRouter.ai choice structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRouterChoice {
    pub index: u32,
    pub message: OpenRouterMessage,
    pub finish_reason: String,
}

/// OpenRouter.ai usage structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRouterUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// OpenRouter.ai integration
pub struct OpenRouterIntegration {
    config: ServiceConfig,
    http_client: reqwest::Client,
}

impl OpenRouterIntegration {
    /// Create a new OpenRouter integration
    pub fn new() -> Self {
        let config = ServiceConfig::default_for_service(ServiceProvider::OpenRouter);
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(config.default_timeout_ms as u64))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            config,
            http_client,
        }
    }

    /// Create a chat completion request
    pub fn create_chat_request(
        &self,
        model: &str,
        messages: Vec<OpenRouterMessage>,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
    ) -> OpenRouterRequest {
        OpenRouterRequest {
            model: model.to_string(),
            messages,
            max_tokens,
            temperature,
            top_p: None,
            stream: Some(false),
        }
    }

    /// Get available models
    pub async fn get_models(&self, api_key: &ApiKey) -> AppResult<Vec<String>> {
        debug!("Getting available models from OpenRouter");

        let url = format!("{}/models", self.config.base_url);
        let response = self.http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", api_key.encrypted_key))
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| ApiError::external_service_error("OpenRouter".to_string(), e.to_string()))?;

        if response.status().is_success() {
            let models_response: serde_json::Value = response.json().await
                .map_err(|e| ApiError::external_service_error("OpenRouter".to_string(), e.to_string()))?;

            let models = models_response["data"]
                .as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .filter_map(|model| model["id"].as_str().map(|s| s.to_string()))
                .collect();

            debug!("Retrieved {} models from OpenRouter", models.len());
            Ok(models)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(ApiError::external_service_error("OpenRouter".to_string(), error_text))
        }
    }

    /// Make a chat completion request
    pub async fn chat_completion(&self, api_key: &ApiKey, request: OpenRouterRequest) -> AppResult<OpenRouterResponse> {
        debug!("Making chat completion request to OpenRouter");

        let url = format!("{}/chat/completions", self.config.base_url);
        let response = self.http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key.encrypted_key))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", "https://github.com/your-repo") // Required by OpenRouter
            .header("X-Title", "Free Deep Research System") // Required by OpenRouter
            .json(&request)
            .send()
            .await
            .map_err(|e| ApiError::external_service_error("OpenRouter".to_string(), e.to_string()))?;

        if response.status().is_success() {
            let openrouter_response: OpenRouterResponse = response.json().await
                .map_err(|e| ApiError::external_service_error("OpenRouter".to_string(), e.to_string()))?;

            debug!("Chat completion successful, used {} tokens", openrouter_response.usage.total_tokens);
            Ok(openrouter_response)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            error!("OpenRouter chat completion failed: {}", error_text);
            Err(ApiError::external_service_error("OpenRouter".to_string(), error_text))
        }
    }

    /// Test the API key with a simple request
    async fn test_api_key_internal(&self, api_key: &ApiKey) -> AppResult<String> {
        debug!("Testing OpenRouter API key");

        // Try to get models list as a simple test
        match self.get_models(api_key).await {
            Ok(models) => {
                if models.is_empty() {
                    Ok("API key valid but no models available".to_string())
                } else {
                    Ok(format!("API key valid, {} models available", models.len()))
                }
            }
            Err(e) => Err(e),
        }
    }
}

#[async_trait]
impl ServiceIntegration for OpenRouterIntegration {
    fn service_provider(&self) -> ServiceProvider {
        ServiceProvider::OpenRouter
    }

    async fn make_request(&self, mut request: ServiceRequest, api_key: &ApiKey) -> AppResult<ServiceResponse> {
        debug!("Making OpenRouter request: {}", request.endpoint);

        let start_time = std::time::Instant::now();
        let mut response = ServiceResponse {
            request_id: request.request_id,
            service: ServiceProvider::OpenRouter,
            status_code: 0,
            headers: HashMap::new(),
            body: String::new(),
            response_time_ms: 0,
            success: false,
            error_message: None,
            metadata: HashMap::new(),
            timestamp: Utc::now(),
        };

        // Transform request for OpenRouter specifics
        self.transform_request(&mut request).await?;

        let url = format!("{}{}", self.config.base_url, request.endpoint);
        let mut req_builder = match request.method.as_str() {
            "GET" => self.http_client.get(&url),
            "POST" => self.http_client.post(&url),
            "PUT" => self.http_client.put(&url),
            "DELETE" => self.http_client.delete(&url),
            _ => return Err(ApiError::invalid_configuration("method".to_string(), "Unsupported HTTP method".to_string())),
        };

        // Add headers
        req_builder = req_builder.header("Authorization", format!("Bearer {}", api_key.encrypted_key));
        req_builder = req_builder.header("Content-Type", "application/json");
        req_builder = req_builder.header("HTTP-Referer", "https://github.com/your-repo");
        req_builder = req_builder.header("X-Title", "Free Deep Research System");

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
        debug!("Performing OpenRouter health check");

        match self.test_api_key_internal(api_key).await {
            Ok(_) => Ok(ServiceHealth::Healthy),
            Err(e) => {
                warn!("OpenRouter health check failed: {}", e);
                Ok(ServiceHealth::Unhealthy)
            }
        }
    }

    fn get_config(&self) -> &ServiceConfig {
        &self.config
    }

    async fn update_config(&mut self, config: ServiceConfig) -> AppResult<()> {
        info!("Updating OpenRouter configuration");
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
            "/models".to_string(),
            "/chat/completions".to_string(),
            "/completions".to_string(),
        ]
    }

    async fn transform_request(&self, request: &mut ServiceRequest) -> AppResult<()> {
        // Add OpenRouter-specific headers if not present
        if !request.headers.contains_key("HTTP-Referer") {
            request.headers.insert("HTTP-Referer".to_string(), "https://github.com/your-repo".to_string());
        }
        if !request.headers.contains_key("X-Title") {
            request.headers.insert("X-Title".to_string(), "Free Deep Research System".to_string());
        }

        Ok(())
    }

    async fn transform_response(&self, response: &mut ServiceResponse) -> AppResult<()> {
        // Add OpenRouter-specific metadata
        if response.success {
            response.metadata.insert("provider".to_string(), "OpenRouter".to_string());
            
            // Try to extract token usage from response
            if let Ok(parsed_response) = serde_json::from_str::<serde_json::Value>(&response.body) {
                if let Some(usage) = parsed_response.get("usage") {
                    if let Some(total_tokens) = usage.get("total_tokens").and_then(|v| v.as_u64()) {
                        response.metadata.insert("total_tokens".to_string(), total_tokens.to_string());
                    }
                }
            }
        }

        Ok(())
    }
}
