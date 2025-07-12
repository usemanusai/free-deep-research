use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};

use crate::error::{AppResult, ApiError};
use crate::models::{ApiKey, CreateApiKeyRequest, UpdateApiKeyRequest, ApiKeyTestResult, ApiKeyImport, ApiKeyExport};
use crate::services::{Service, DataPersistenceService, SecurityService, MonitoringService};
use crate::commands::api_management::ImportResult;
use uuid::Uuid;

// TODO: Implement these modules
// pub mod key_manager;
// pub mod rate_limiter;
// pub mod health_monitor;
// pub mod fallback_router;

/// API Manager Service that handles all external API interactions
pub struct ApiManagerService {
    data_persistence: Arc<RwLock<DataPersistenceService>>,
    security: Arc<RwLock<SecurityService>>,
    monitoring: Arc<RwLock<MonitoringService>>,
}

impl ApiManagerService {
    /// Create a new API manager service
    pub async fn new(
        data_persistence: Arc<RwLock<DataPersistenceService>>,
        security: Arc<RwLock<SecurityService>>,
        monitoring: Arc<RwLock<MonitoringService>>,
    ) -> AppResult<Self> {
        info!("Initializing API manager service...");
        
        let service = Self {
            data_persistence,
            security,
            monitoring,
        };
        
        info!("API manager service initialized successfully");
        Ok(service)
    }
    
    /// Get all API keys
    pub async fn get_all_keys(&self) -> AppResult<Vec<ApiKey>> {
        debug!("Getting all API keys");

        let data_persistence = self.data_persistence.read().await;
        let api_keys = data_persistence.get_all_api_keys().await?;
        drop(data_persistence);

        debug!("Retrieved {} API keys", api_keys.len());
        Ok(api_keys)
    }

    /// Add a new API key
    pub async fn add_key(&mut self, request: CreateApiKeyRequest) -> AppResult<ApiKey> {
        debug!("Adding new API key for service: {:?}", request.service);

        // Validate the request
        if request.name.trim().is_empty() {
            return Err(ApiError::invalid_configuration(
                format!("{:?}", request.service),
                "API key name cannot be empty".to_string()
            ).into());
        }

        if request.api_key.trim().is_empty() {
            return Err(ApiError::invalid_configuration(
                format!("{:?}", request.service),
                "API key cannot be empty".to_string()
            ).into());
        }

        // Encrypt the API key
        let security = self.security.read().await;
        let encrypted_key = security.encrypt_string(&request.api_key).await?;
        drop(security);

        // Create the API key object
        let api_key = ApiKey {
            id: Uuid::new_v4(),
            service: request.service,
            name: request.name,
            encrypted_key,
            usage_count: 0,
            rate_limit: request.rate_limit.unwrap_or(1000),
            reset_period: crate::models::api_key::ResetPeriod::Daily,
            last_used: None,
            last_reset: chrono::Utc::now(),
            status: crate::models::api_key::ApiKeyStatus::Active,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // Store in database
        let mut data_persistence = self.data_persistence.write().await;
        data_persistence.store_api_key(&api_key).await?;
        drop(data_persistence);

        // Log audit event
        let monitoring = self.monitoring.read().await;
        // TODO: Log audit event through monitoring service
        drop(monitoring);

        info!("API key added successfully: {} ({})", api_key.name, api_key.id);
        Ok(api_key)
    }

    /// Update an existing API key
    pub async fn update_key(&mut self, key_id: Uuid, request: UpdateApiKeyRequest) -> AppResult<ApiKey> {
        debug!("Updating API key: {}", key_id);

        // Get existing key
        let mut api_keys = self.get_all_keys().await?;
        let api_key_index = api_keys.iter().position(|k| k.id == key_id)
            .ok_or_else(|| ApiError::key_not_found(key_id.to_string()))?;

        let mut api_key = api_keys.remove(api_key_index);

        // Update fields
        if let Some(name) = request.name {
            if name.trim().is_empty() {
                return Err(ApiError::invalid_configuration(
                    format!("{:?}", api_key.service),
                    "API key name cannot be empty".to_string()
                ).into());
            }
            api_key.name = name;
        }

        if let Some(new_api_key) = request.api_key {
            if new_api_key.trim().is_empty() {
                return Err(ApiError::invalid_configuration(
                    format!("{:?}", api_key.service),
                    "API key cannot be empty".to_string()
                ).into());
            }

            // Encrypt the new API key
            let security = self.security.read().await;
            api_key.encrypted_key = security.encrypt_string(&new_api_key).await?;
            drop(security);
        }

        if let Some(rate_limit) = request.rate_limit {
            api_key.rate_limit = rate_limit;
        }

        if let Some(status) = request.status {
            api_key.status = status;
        }

        api_key.updated_at = chrono::Utc::now();

        // Store updated key
        let mut data_persistence = self.data_persistence.write().await;
        data_persistence.store_api_key(&api_key).await?;
        drop(data_persistence);

        info!("API key updated successfully: {} ({})", api_key.name, api_key.id);
        Ok(api_key)
    }

    /// Delete an API key
    pub async fn delete_key(&mut self, key_id: Uuid) -> AppResult<()> {
        debug!("Deleting API key: {}", key_id);

        // Verify key exists
        let api_keys = self.get_all_keys().await?;
        let api_key = api_keys.iter().find(|k| k.id == key_id)
            .ok_or_else(|| ApiError::key_not_found(key_id.to_string()))?;

        let key_name = api_key.name.clone();

        // TODO: Implement actual deletion from database
        // For now, we'll mark it as inactive
        let update_request = UpdateApiKeyRequest {
            name: None,
            api_key: None,
            rate_limit: None,
            status: Some(crate::models::api_key::ApiKeyStatus::Inactive),
        };

        self.update_key(key_id, update_request).await?;

        info!("API key deleted successfully: {} ({})", key_name, key_id);
        Ok(())
    }
    
    /// Test an API key connection
    pub async fn test_key(&self, key_id: Uuid) -> AppResult<ApiKeyTestResult> {
        debug!("Testing API key: {}", key_id);

        // Get the API key
        let api_keys = self.get_all_keys().await?;
        let api_key = api_keys.iter().find(|k| k.id == key_id)
            .ok_or_else(|| ApiError::key_not_found(key_id.to_string()))?;

        // Decrypt the API key
        let security = self.security.read().await;
        let decrypted_key = security.decrypt_string(&api_key.encrypted_key).await?;
        drop(security);

        let start_time = std::time::Instant::now();

        // Test the key based on service type
        let test_result = match api_key.service {
            crate::models::api_key::ServiceProvider::OpenRouter => {
                self.test_openrouter_key(&decrypted_key).await
            }
            crate::models::api_key::ServiceProvider::SerpApi => {
                self.test_serpapi_key(&decrypted_key).await
            }
            crate::models::api_key::ServiceProvider::Jina => {
                self.test_jina_key(&decrypted_key).await
            }
            crate::models::api_key::ServiceProvider::Firecrawl => {
                self.test_firecrawl_key(&decrypted_key).await
            }
            crate::models::api_key::ServiceProvider::Tavily => {
                self.test_tavily_key(&decrypted_key).await
            }
            crate::models::api_key::ServiceProvider::Exa => {
                self.test_exa_key(&decrypted_key).await
            }
        };

        let response_time = start_time.elapsed().as_millis() as u32;

        let result = match test_result {
            Ok(message) => ApiKeyTestResult {
                key_id,
                success: true,
                message,
                response_time_ms: Some(response_time),
                tested_at: chrono::Utc::now(),
            },
            Err(e) => ApiKeyTestResult {
                key_id,
                success: false,
                message: e.to_string(),
                response_time_ms: Some(response_time),
                tested_at: chrono::Utc::now(),
            }
        };

        debug!("API key test completed: {} (success: {})", key_id, result.success);
        Ok(result)
    }

    /// Test OpenRouter API key
    async fn test_openrouter_key(&self, api_key: &str) -> AppResult<String> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://openrouter.ai/api/v1/models")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("HTTP-Referer", "https://github.com/usemanusai/free-deep-research")
            .header("X-Title", "Free Deep Research System")
            .send()
            .await
            .map_err(|e| ApiError::connection_failed("openrouter".to_string(), e.to_string()))?;

        if response.status().is_success() {
            Ok("OpenRouter API key is valid and working".to_string())
        } else {
            Err(ApiError::authentication_failed("openrouter".to_string(),
                format!("HTTP {}: {}", response.status(), response.text().await.unwrap_or_default())).into())
        }
    }

    /// Test SerpAPI key
    async fn test_serpapi_key(&self, api_key: &str) -> AppResult<String> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://serpapi.com/account")
            .query(&[("api_key", api_key)])
            .send()
            .await
            .map_err(|e| ApiError::connection_failed("serpapi".to_string(), e.to_string()))?;

        if response.status().is_success() {
            Ok("SerpAPI key is valid and working".to_string())
        } else {
            Err(ApiError::authentication_failed("serpapi".to_string(),
                format!("HTTP {}: {}", response.status(), response.text().await.unwrap_or_default())).into())
        }
    }

    /// Test Jina AI key
    async fn test_jina_key(&self, api_key: &str) -> AppResult<String> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://api.jina.ai/v1/models")
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await
            .map_err(|e| ApiError::connection_failed("jina".to_string(), e.to_string()))?;

        if response.status().is_success() {
            Ok("Jina AI API key is valid and working".to_string())
        } else {
            Err(ApiError::authentication_failed("jina".to_string(),
                format!("HTTP {}: {}", response.status(), response.text().await.unwrap_or_default())).into())
        }
    }

    /// Test Firecrawl key
    async fn test_firecrawl_key(&self, api_key: &str) -> AppResult<String> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://api.firecrawl.dev/v0/crawl/status/test")
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await
            .map_err(|e| ApiError::connection_failed("firecrawl".to_string(), e.to_string()))?;

        // Firecrawl returns 404 for non-existent jobs, but with valid auth
        if response.status().is_success() || response.status() == 404 {
            Ok("Firecrawl API key is valid and working".to_string())
        } else {
            Err(ApiError::authentication_failed("firecrawl".to_string(),
                format!("HTTP {}: {}", response.status(), response.text().await.unwrap_or_default())).into())
        }
    }

    /// Test Tavily key
    async fn test_tavily_key(&self, api_key: &str) -> AppResult<String> {
        let client = reqwest::Client::new();
        let test_payload = serde_json::json!({
            "api_key": api_key,
            "query": "test",
            "max_results": 1
        });

        let response = client
            .post("https://api.tavily.com/search")
            .json(&test_payload)
            .send()
            .await
            .map_err(|e| ApiError::connection_failed("tavily".to_string(), e.to_string()))?;

        if response.status().is_success() {
            Ok("Tavily API key is valid and working".to_string())
        } else {
            Err(ApiError::authentication_failed("tavily".to_string(),
                format!("HTTP {}: {}", response.status(), response.text().await.unwrap_or_default())).into())
        }
    }

    /// Test Exa key
    async fn test_exa_key(&self, api_key: &str) -> AppResult<String> {
        let client = reqwest::Client::new();
        let test_payload = serde_json::json!({
            "query": "test",
            "numResults": 1
        });

        let response = client
            .post("https://api.exa.ai/search")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&test_payload)
            .send()
            .await
            .map_err(|e| ApiError::connection_failed("exa".to_string(), e.to_string()))?;

        if response.status().is_success() {
            Ok("Exa API key is valid and working".to_string())
        } else {
            Err(ApiError::authentication_failed("exa".to_string(),
                format!("HTTP {}: {}", response.status(), response.text().await.unwrap_or_default())).into())
        }
    }

    /// Import API keys from file
    pub async fn import_keys(&mut self, keys: Vec<ApiKeyImport>) -> AppResult<ImportResult> {
        debug!("Importing {} API keys", keys.len());

        let mut successful_count = 0;
        let mut failed_count = 0;
        let mut errors = Vec::new();

        for key_import in keys {
            let create_request = CreateApiKeyRequest {
                service: key_import.service,
                name: key_import.name.clone(),
                api_key: key_import.api_key,
                rate_limit: key_import.rate_limit,
            };

            match self.add_key(create_request).await {
                Ok(_) => {
                    successful_count += 1;
                    info!("Successfully imported API key: {}", key_import.name);
                }
                Err(e) => {
                    failed_count += 1;
                    let error_msg = format!("Failed to import '{}': {}", key_import.name, e);
                    errors.push(error_msg.clone());
                    error!("{}", error_msg);
                }
            }
        }

        info!("Import completed: {} successful, {} failed", successful_count, failed_count);

        Ok(ImportResult {
            successful_count,
            failed_count,
            errors,
        })
    }

    /// Export API keys to file
    pub async fn export_keys(&self) -> AppResult<Vec<ApiKeyExport>> {
        debug!("Exporting API keys");

        let api_keys = self.get_all_keys().await?;
        let mut exports = Vec::new();

        for api_key in api_keys {
            exports.push(ApiKeyExport {
                service: api_key.service,
                name: api_key.name,
                rate_limit: api_key.rate_limit,
                usage_count: api_key.usage_count,
                status: api_key.status,
                created_at: api_key.created_at,
            });
        }

        debug!("Exported {} API keys", exports.len());
        Ok(exports)
    }
    
    /// Start background tasks
    pub async fn start_background_tasks(&self) -> AppResult<()> {
        info!("Starting API manager background tasks...");
        
        // TODO: Start rate limit monitoring, key rotation, health checks
        
        Ok(())
    }
}

#[async_trait::async_trait]
impl Service for ApiManagerService {
    async fn health_check(&self) -> AppResult<()> {
        debug!("Performing API manager health check");
        
        // TODO: Implement actual health check
        
        Ok(())
    }
    
    async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down API manager service...");
        
        // TODO: Implement graceful shutdown
        
        Ok(())
    }
}
