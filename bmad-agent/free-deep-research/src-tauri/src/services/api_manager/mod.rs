use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};

use crate::error::{AppResult, ApiError};
use crate::models::{ApiKey, CreateApiKeyRequest, UpdateApiKeyRequest, ApiKeyTestResult, ApiKeyImport, ApiKeyExport};
use crate::services::{Service, DataPersistenceService, SecurityService, MonitoringService};
use uuid::Uuid;

pub mod rate_limiter;
pub use rate_limiter::{RateLimiter, RateLimitConfig, UsageStatus, LimitStatus, RateLimitAlert, AlertType, UsageForecast};

pub mod key_rotator;
pub use key_rotator::{KeyRotator, KeyPerformanceMetrics, KeyHealth, RotationStrategy, RotationConfig, RotationAnalytics};

pub mod service_integration;
pub use service_integration::{ServiceIntegrationManager, ServiceRequest, ServiceResponse, ServiceHealth, ServiceConfig, ServiceMetrics};

pub mod integrations;
pub use integrations::create_all_integrations;

/// Result of API key import operation
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ImportResult {
    pub successful_count: u32,
    pub failed_count: u32,
    pub errors: Vec<String>,
}

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
    rate_limiter: Arc<RateLimiter>,
    key_rotator: Arc<KeyRotator>,
    service_integration: Arc<RwLock<ServiceIntegrationManager>>,
}

impl ApiManagerService {
    /// Create a new API manager service
    pub async fn new(
        data_persistence: Arc<RwLock<DataPersistenceService>>,
        security: Arc<RwLock<SecurityService>>,
        monitoring: Arc<RwLock<MonitoringService>>,
    ) -> AppResult<Self> {
        info!("Initializing API manager service...");

        // Initialize rate limiter
        let rate_limiter = Arc::new(RateLimiter::new(data_persistence.clone()).await?);

        // Initialize key rotator
        let key_rotator = Arc::new(KeyRotator::new(data_persistence.clone()).await?);

        // Initialize service integration manager
        let service_integration = Arc::new(RwLock::new(create_all_integrations().await?));

        let service = Self {
            data_persistence,
            security,
            monitoring,
            rate_limiter,
            key_rotator,
            service_integration,
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

        // Get the API key first to get its name for logging
        let mut data_persistence = self.data_persistence.write().await;
        let api_key = data_persistence.get_api_key_by_id(key_id).await?
            .ok_or_else(|| ApiError::key_not_found(key_id.to_string()))?;

        let key_name = api_key.name.clone();

        // Delete from database
        data_persistence.delete_api_key(key_id).await?;
        drop(data_persistence);

        // Log audit event
        let monitoring = self.monitoring.read().await;
        // TODO: Log audit event through monitoring service
        drop(monitoring);

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

        // Record usage statistics
        let mut data_persistence = self.data_persistence.write().await;
        let service_name = format!("{:?}", api_key.service).to_lowercase();
        if let Err(e) = data_persistence.record_api_usage(
            key_id,
            &service_name,
            Some("test"),
            result.success,
            response_time
        ).await {
            error!("Failed to record API usage statistics: {}", e);
        }
        drop(data_persistence);

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

    /// Import API keys from CSV content
    pub async fn import_keys_from_csv(&mut self, csv_content: &str) -> AppResult<ImportResult> {
        debug!("Importing API keys from CSV content");

        let mut successful_count = 0;
        let mut failed_count = 0;
        let mut errors = Vec::new();

        // Parse CSV content
        let mut reader = csv::Reader::from_reader(csv_content.as_bytes());

        for (line_num, result) in reader.records().enumerate() {
            let line_num = line_num + 2; // +2 because enumerate starts at 0 and we have a header

            match result {
                Ok(record) => {
                    if record.len() < 3 {
                        failed_count += 1;
                        let error_msg = format!("Line {}: Insufficient columns (expected at least 3: service, name, key)", line_num);
                        errors.push(error_msg);
                        continue;
                    }

                    let service_str = record.get(0).unwrap_or("").trim();
                    let name = record.get(1).unwrap_or("").trim();
                    let api_key = record.get(2).unwrap_or("").trim();
                    let rate_limit_str = record.get(3).unwrap_or("").trim();

                    // Parse service
                    let service = match crate::models::api_key::ServiceProvider::from_str(service_str) {
                        Some(s) => s,
                        None => {
                            failed_count += 1;
                            let error_msg = format!("Line {}: Invalid service '{}'", line_num, service_str);
                            errors.push(error_msg);
                            continue;
                        }
                    };

                    // Parse rate limit (optional)
                    let rate_limit = if rate_limit_str.is_empty() {
                        None
                    } else {
                        match rate_limit_str.parse::<u32>() {
                            Ok(limit) => Some(limit),
                            Err(_) => {
                                failed_count += 1;
                                let error_msg = format!("Line {}: Invalid rate limit '{}'", line_num, rate_limit_str);
                                errors.push(error_msg);
                                continue;
                            }
                        }
                    };

                    let create_request = CreateApiKeyRequest {
                        service,
                        name: name.to_string(),
                        api_key: api_key.to_string(),
                        rate_limit,
                    };

                    match self.add_key(create_request).await {
                        Ok(_) => {
                            successful_count += 1;
                            info!("Successfully imported API key: {}", name);
                        }
                        Err(e) => {
                            failed_count += 1;
                            let error_msg = format!("Line {}: Failed to import '{}': {}", line_num, name, e);
                            errors.push(error_msg);
                        }
                    }
                }
                Err(e) => {
                    failed_count += 1;
                    let error_msg = format!("Line {}: CSV parsing error: {}", line_num, e);
                    errors.push(error_msg);
                }
            }
        }

        info!("CSV import completed: {} successful, {} failed", successful_count, failed_count);

        Ok(ImportResult {
            successful_count,
            failed_count,
            errors,
        })
    }

    /// Import API keys from JSON content
    pub async fn import_keys_from_json(&mut self, json_content: &str) -> AppResult<ImportResult> {
        debug!("Importing API keys from JSON content");

        let keys: Vec<ApiKeyImport> = serde_json::from_str(json_content)
            .map_err(|e| ApiError::invalid_configuration("json".to_string(), format!("JSON parsing error: {}", e)))?;

        self.import_keys(keys).await
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

    /// Export API keys to CSV format
    pub async fn export_keys_to_csv(&self) -> AppResult<String> {
        debug!("Exporting API keys to CSV format");

        let api_keys = self.get_all_keys().await?;
        let mut csv_content = String::new();

        // Add CSV header
        csv_content.push_str("service,name,rate_limit,usage_count,status,created_at\n");

        for api_key in api_keys {
            csv_content.push_str(&format!(
                "{},{},{},{},{},{}\n",
                format!("{:?}", api_key.service).to_lowercase(),
                api_key.name,
                api_key.rate_limit,
                api_key.usage_count,
                format!("{:?}", api_key.status).to_lowercase(),
                api_key.created_at.to_rfc3339()
            ));
        }

        debug!("Exported {} API keys to CSV", api_keys.len());
        Ok(csv_content)
    }

    /// Export API keys to JSON format
    pub async fn export_keys_to_json(&self) -> AppResult<String> {
        debug!("Exporting API keys to JSON format");

        let exports = self.export_keys().await?;
        let json_content = serde_json::to_string_pretty(&exports)
            .map_err(|e| ApiError::invalid_configuration("json".to_string(), format!("JSON serialization error: {}", e)))?;

        debug!("Exported {} API keys to JSON", exports.len());
        Ok(json_content)
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

    /// Get usage statistics for an API key
    pub async fn get_key_usage_stats(&self, key_id: Uuid, days: u32) -> AppResult<Vec<(String, u32, u32, u32, f64)>> {
        debug!("Getting usage statistics for API key: {}", key_id);

        let data_persistence = self.data_persistence.read().await;
        let stats = data_persistence.get_api_key_usage_stats(key_id, days).await?;
        drop(data_persistence);

        debug!("Retrieved {} usage statistics records", stats.len());
        Ok(stats)
    }

    /// Get all API keys with their current status
    pub async fn get_keys_with_status(&self) -> AppResult<Vec<(ApiKey, bool)>> {
        debug!("Getting all API keys with status");

        let api_keys = self.get_all_keys().await?;
        let mut keys_with_status = Vec::new();

        for api_key in api_keys {
            // Check if key needs reset
            let mut key = api_key.clone();
            if key.needs_reset() {
                // Note: In a real implementation, we'd update the key in the database
                // For now, we'll just mark it as needing reset
            }

            let is_available = key.is_available();
            keys_with_status.push((key, is_available));
        }

        debug!("Retrieved {} API keys with status", keys_with_status.len());
        Ok(keys_with_status)
    }

    /// Reset API key usage if needed
    pub async fn reset_key_usage_if_needed(&mut self, key_id: Uuid) -> AppResult<bool> {
        debug!("Checking if API key needs usage reset: {}", key_id);

        let mut data_persistence = self.data_persistence.write().await;
        let api_key = data_persistence.get_api_key_by_id(key_id).await?
            .ok_or_else(|| ApiError::key_not_found(key_id.to_string()))?;

        if api_key.needs_reset() {
            let mut updated_key = api_key.clone();
            updated_key.reset_usage();

            data_persistence.store_api_key(&updated_key).await?;
            drop(data_persistence);

            info!("API key usage reset: {} ({})", updated_key.name, key_id);
            return Ok(true);
        }

        drop(data_persistence);
        Ok(false)
    }

    /// Reset all API keys that need reset
    pub async fn reset_all_keys_if_needed(&mut self) -> AppResult<u32> {
        debug!("Checking all API keys for usage reset");

        let api_keys = self.get_all_keys().await?;
        let mut reset_count = 0;

        for api_key in api_keys {
            if self.reset_key_usage_if_needed(api_key.id).await? {
                reset_count += 1;
            }
        }

        if reset_count > 0 {
            info!("Reset usage for {} API keys", reset_count);
        }

        Ok(reset_count)
    }

    /// Get next available API key for a service
    pub async fn get_next_available_key(&mut self, service: &str) -> AppResult<Option<ApiKey>> {
        debug!("Getting next available key for service: {}", service);

        // First, reset any keys that need reset
        self.reset_all_keys_if_needed().await?;

        let api_keys = self.get_all_keys().await?;

        // Filter keys for the requested service and find available ones
        let available_keys: Vec<_> = api_keys
            .into_iter()
            .filter(|key| {
                let key_service = format!("{:?}", key.service).to_lowercase();
                key_service == service.to_lowercase() && key.is_available()
            })
            .collect();

        if available_keys.is_empty() {
            debug!("No available keys found for service: {}", service);
            return Ok(None);
        }

        // Simple round-robin: find the key with the lowest usage count
        let best_key = available_keys
            .into_iter()
            .min_by_key(|key| key.usage_count)
            .unwrap();

        debug!("Selected key for service {}: {} (usage: {}/{})",
               service, best_key.name, best_key.usage_count, best_key.rate_limit);

        Ok(Some(best_key))
    }

    /// Record API key usage
    pub async fn record_key_usage(&mut self, key_id: Uuid, success: bool, response_time_ms: u32) -> AppResult<()> {
        debug!("Recording usage for API key: {}", key_id);

        let mut data_persistence = self.data_persistence.write().await;

        // Get and update the API key
        let mut api_key = data_persistence.get_api_key_by_id(key_id).await?
            .ok_or_else(|| ApiError::key_not_found(key_id.to_string()))?;

        api_key.increment_usage();
        data_persistence.store_api_key(&api_key).await?;

        // Record usage statistics
        let service_name = format!("{:?}", api_key.service).to_lowercase();
        data_persistence.record_api_usage(
            key_id,
            &service_name,
            None,
            success,
            response_time_ms
        ).await?;

        drop(data_persistence);

        debug!("Recorded usage for API key: {} (new usage: {}/{})",
               key_id, api_key.usage_count, api_key.rate_limit);

        Ok(())
    }

    /// Check if a request can be made for a specific API key (with rate limiting)
    pub async fn can_make_request(&self, api_key_id: Uuid) -> AppResult<bool> {
        self.rate_limiter.can_make_request(api_key_id).await
    }

    /// Get usage status for an API key
    pub async fn get_key_usage_status(&self, api_key_id: Uuid) -> AppResult<UsageStatus> {
        self.rate_limiter.get_usage_status(api_key_id).await
    }

    /// Record a request and check for rate limit violations
    pub async fn record_api_request(&self, api_key_id: Uuid, success: bool) -> AppResult<Option<RateLimitAlert>> {
        self.rate_limiter.record_request(api_key_id, success).await
    }

    /// Enable or disable emergency stop
    pub async fn set_emergency_stop(&self, enabled: bool) -> AppResult<()> {
        self.rate_limiter.set_emergency_stop(enabled).await
    }

    /// Check if emergency stop is enabled
    pub async fn is_emergency_stop_enabled(&self) -> bool {
        self.rate_limiter.is_emergency_stop_enabled().await
    }

    /// Get recent rate limit alerts
    pub async fn get_recent_alerts(&self, limit: usize) -> Vec<RateLimitAlert> {
        self.rate_limiter.get_recent_alerts(limit).await
    }

    /// Generate usage forecast for an API key
    pub async fn generate_usage_forecast(&self, api_key_id: Uuid) -> AppResult<UsageForecast> {
        self.rate_limiter.generate_usage_forecast(api_key_id).await
    }

    /// Get usage analytics for all API keys
    pub async fn get_usage_analytics(&self) -> AppResult<Vec<(Uuid, UsageStatus, UsageForecast)>> {
        self.rate_limiter.get_usage_analytics().await
    }

    /// Check all API keys for threshold violations
    pub async fn check_all_thresholds(&self) -> AppResult<Vec<RateLimitAlert>> {
        self.rate_limiter.check_all_thresholds().await
    }

    /// Generate automated usage report
    pub async fn generate_usage_report(&self) -> AppResult<String> {
        self.rate_limiter.generate_usage_report().await
    }

    /// Update rate limit configuration for a service
    pub async fn update_rate_limit_config(&self, service: crate::models::api_key::ServiceProvider, config: RateLimitConfig) -> AppResult<()> {
        self.rate_limiter.update_config(service, config).await
    }

    /// Get rate limit configuration for a service
    pub async fn get_rate_limit_config(&self, service: crate::models::api_key::ServiceProvider) -> AppResult<RateLimitConfig> {
        self.rate_limiter.get_config(service).await
    }

    /// Get all rate limit configurations
    pub async fn get_all_rate_limit_configs(&self) -> std::collections::HashMap<crate::models::api_key::ServiceProvider, RateLimitConfig> {
        self.rate_limiter.get_all_configs().await
    }

    /// Select the best available API key for a service using intelligent rotation
    pub async fn select_best_key_for_service(&self, service: crate::models::api_key::ServiceProvider) -> AppResult<Option<ApiKey>> {
        self.key_rotator.select_best_key(service).await
    }

    /// Record request performance for key rotation optimization
    pub async fn record_key_performance(&self, api_key_id: Uuid, success: bool, response_time_ms: u32) -> AppResult<()> {
        self.key_rotator.record_request_performance(api_key_id, success, response_time_ms).await
    }

    /// Get performance metrics for an API key
    pub async fn get_key_performance_metrics(&self, api_key_id: Uuid) -> Option<KeyPerformanceMetrics> {
        self.key_rotator.get_key_performance(api_key_id).await
    }

    /// Get performance metrics for all keys of a service
    pub async fn get_service_performance_metrics(&self, service: crate::models::api_key::ServiceProvider) -> Vec<KeyPerformanceMetrics> {
        self.key_rotator.get_service_performance(service).await
    }

    /// Get all performance metrics
    pub async fn get_all_performance_metrics(&self) -> std::collections::HashMap<Uuid, KeyPerformanceMetrics> {
        self.key_rotator.get_all_performance_metrics().await
    }

    /// Get rotation analytics
    pub async fn get_rotation_analytics(&self) -> RotationAnalytics {
        self.key_rotator.get_rotation_analytics().await
    }

    /// Update rotation configuration for a service
    pub async fn update_rotation_config(&self, service: crate::models::api_key::ServiceProvider, config: RotationConfig) -> AppResult<()> {
        self.key_rotator.update_rotation_config(service, config).await
    }

    /// Get rotation configuration for a service
    pub async fn get_rotation_config(&self, service: crate::models::api_key::ServiceProvider) -> RotationConfig {
        self.key_rotator.get_rotation_config(service).await
    }

    /// Perform health check on all API keys
    pub async fn perform_health_check(&self) -> AppResult<std::collections::HashMap<Uuid, KeyHealth>> {
        self.key_rotator.perform_health_check().await
    }

    /// Reactivate keys that have completed their cooldown
    pub async fn reactivate_cooled_down_keys(&self) -> AppResult<Vec<Uuid>> {
        self.key_rotator.reactivate_cooled_down_keys().await
    }

    /// Get keys that need attention (unhealthy, failed, or in cooldown)
    pub async fn get_keys_needing_attention(&self) -> Vec<(Uuid, KeyPerformanceMetrics)> {
        self.key_rotator.get_keys_needing_attention().await
    }

    /// Generate rotation report
    pub async fn generate_rotation_report(&self) -> AppResult<String> {
        self.key_rotator.generate_rotation_report().await
    }

    /// Make a service request through the integration framework
    pub async fn make_service_request(&self, service: crate::models::api_key::ServiceProvider, request: ServiceRequest) -> AppResult<ServiceResponse> {
        // Get the best available key for the service
        let api_key = self.select_best_key_for_service(service).await?
            .ok_or_else(|| ApiError::key_not_found(format!("No available keys for service: {:?}", service)))?;

        // Check rate limits
        if !self.can_make_request(api_key.id).await? {
            return Err(ApiError::rate_limit_exceeded(format!("Rate limit exceeded for service: {:?}", service)));
        }

        let start_time = std::time::Instant::now();

        // Make the request through service integration
        let service_integration = self.service_integration.read().await;
        let result = service_integration.make_service_request(service, request, &api_key).await;
        drop(service_integration);

        let response_time = start_time.elapsed().as_millis() as u32;
        let success = result.is_ok();

        // Record performance metrics
        if let Err(e) = self.record_key_performance(api_key.id, success, response_time).await {
            error!("Failed to record key performance: {}", e);
        }

        // Record rate limiting
        if let Err(e) = self.record_api_request(api_key.id, success).await {
            error!("Failed to record API request: {}", e);
        }

        result
    }

    /// Check service health
    pub async fn check_service_health(&self, service: crate::models::api_key::ServiceProvider) -> AppResult<ServiceHealth> {
        // Get the best available key for the service
        let api_key = self.select_best_key_for_service(service).await?
            .ok_or_else(|| ApiError::key_not_found(format!("No available keys for service: {:?}", service)))?;

        let service_integration = self.service_integration.read().await;
        let health = service_integration.check_service_health(service, &api_key).await?;
        drop(service_integration);

        Ok(health)
    }

    /// Get service metrics
    pub async fn get_service_metrics(&self, service: crate::models::api_key::ServiceProvider) -> Option<ServiceMetrics> {
        let service_integration = self.service_integration.read().await;
        service_integration.get_service_metrics(service).await
    }

    /// Get all service metrics
    pub async fn get_all_service_metrics(&self) -> std::collections::HashMap<crate::models::api_key::ServiceProvider, ServiceMetrics> {
        let service_integration = self.service_integration.read().await;
        service_integration.get_all_service_metrics().await
    }

    /// Update service configuration
    pub async fn update_service_config(&self, service: crate::models::api_key::ServiceProvider, config: ServiceConfig) -> AppResult<()> {
        let mut service_integration = self.service_integration.write().await;
        service_integration.update_service_config(service, config).await
    }

    /// Get service configuration
    pub async fn get_service_config(&self, service: crate::models::api_key::ServiceProvider) -> Option<ServiceConfig> {
        let service_integration = self.service_integration.read().await;
        service_integration.get_service_config(service).await
    }

    /// Get all service configurations
    pub async fn get_all_service_configs(&self) -> std::collections::HashMap<crate::models::api_key::ServiceProvider, ServiceConfig> {
        let service_integration = self.service_integration.read().await;
        service_integration.get_all_service_configs().await
    }

    /// Validate API key for a service
    pub async fn validate_service_api_key(&self, service: crate::models::api_key::ServiceProvider, api_key: &ApiKey) -> AppResult<bool> {
        let service_integration = self.service_integration.read().await;
        service_integration.validate_service_api_key(service, api_key).await
    }

    /// Get available endpoints for a service
    pub async fn get_service_endpoints(&self, service: crate::models::api_key::ServiceProvider) -> Vec<String> {
        let service_integration = self.service_integration.read().await;
        service_integration.get_service_endpoints(service)
    }

    /// Get list of registered services
    pub async fn get_registered_services(&self) -> Vec<crate::models::api_key::ServiceProvider> {
        let service_integration = self.service_integration.read().await;
        service_integration.get_registered_services()
    }

    /// Generate service status report
    pub async fn generate_service_status_report(&self) -> AppResult<String> {
        let service_integration = self.service_integration.read().await;
        service_integration.generate_service_status_report().await
    }

    /// Start background tasks
    pub async fn start_background_tasks(&self) -> AppResult<()> {
        info!("Starting API manager background tasks...");

        // Start rate limit monitoring task
        let rate_limiter = self.rate_limiter.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // Check every 5 minutes

            loop {
                interval.tick().await;

                // Check all thresholds
                if let Err(e) = rate_limiter.check_all_thresholds().await {
                    error!("Failed to check rate limit thresholds: {}", e);
                }

                // Clear old alerts (older than 24 hours)
                if let Err(e) = rate_limiter.clear_old_alerts(24).await {
                    error!("Failed to clear old alerts: {}", e);
                }
            }
        });

        // Start daily report generation task
        let rate_limiter_report = self.rate_limiter.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(86400)); // Daily

            loop {
                interval.tick().await;

                // Generate daily usage report
                match rate_limiter_report.generate_usage_report().await {
                    Ok(report) => {
                        info!("Generated daily usage report ({} characters)", report.len());
                        // TODO: Save report to file or send via notification
                    }
                    Err(e) => {
                        error!("Failed to generate daily usage report: {}", e);
                    }
                }
            }
        });

        // Start key rotator background tasks
        self.key_rotator.start_background_tasks().await?;

        // Start service integration background monitoring
        let service_integration = self.service_integration.read().await;
        service_integration.start_background_monitoring().await?;
        drop(service_integration);

        info!("API manager background tasks started successfully");
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
