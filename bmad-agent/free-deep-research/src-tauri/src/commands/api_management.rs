use tauri::State;
use uuid::Uuid;
use tracing::{info, error};

use crate::error::AppResult;
use crate::models::{ApiKey, CreateApiKeyRequest, UpdateApiKeyRequest, ApiKeyTestResult, ApiKeyImport};
use crate::services::{ServiceManager, api_manager::{ImportResult, UsageStatus, RateLimitAlert, UsageForecast, RateLimitConfig, KeyPerformanceMetrics, KeyHealth, RotationAnalytics, RotationConfig, ServiceRequest, ServiceResponse, ServiceHealth, ServiceConfig, ServiceMetrics}};

/// Get all API keys
#[tauri::command]
pub async fn get_api_keys(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<ApiKey>, String> {
    info!("Getting all API keys");
    
    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.get_all_keys().await {
        Ok(keys) => Ok(keys),
        Err(e) => {
            error!("Failed to get API keys: {}", e);
            Err(e.to_string())
        }
    }
}

/// Add a new API key
#[tauri::command]
pub async fn add_api_key(
    request: CreateApiKeyRequest,
    service_manager: State<'_, ServiceManager>,
) -> Result<ApiKey, String> {
    info!("Adding new API key for service: {:?}", request.service);
    
    let mut api_manager = service_manager.inner().api_manager.write().await;
    match api_manager.add_key(request).await {
        Ok(key) => {
            info!("API key added successfully: {}", key.id);
            Ok(key)
        }
        Err(e) => {
            error!("Failed to add API key: {}", e);
            Err(e.to_string())
        }
    }
}

/// Update an existing API key
#[tauri::command]
pub async fn update_api_key(
    key_id: String,
    request: UpdateApiKeyRequest,
    service_manager: State<'_, ServiceManager>,
) -> Result<ApiKey, String> {
    info!("Updating API key: {}", key_id);
    
    let key_uuid = Uuid::parse_str(&key_id)
        .map_err(|e| format!("Invalid key ID: {}", e))?;
    
    let mut api_manager = service_manager.inner().api_manager.write().await;
    match api_manager.update_key(key_uuid, request).await {
        Ok(key) => {
            info!("API key updated successfully: {}", key.id);
            Ok(key)
        }
        Err(e) => {
            error!("Failed to update API key: {}", e);
            Err(e.to_string())
        }
    }
}

/// Delete an API key
#[tauri::command]
pub async fn delete_api_key(
    key_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Deleting API key: {}", key_id);
    
    let key_uuid = Uuid::parse_str(&key_id)
        .map_err(|e| format!("Invalid key ID: {}", e))?;
    
    let mut api_manager = service_manager.inner().api_manager.write().await;
    match api_manager.delete_key(key_uuid).await {
        Ok(_) => {
            info!("API key deleted successfully: {}", key_id);
            Ok(())
        }
        Err(e) => {
            error!("Failed to delete API key: {}", e);
            Err(e.to_string())
        }
    }
}

/// Test an API key connection
#[tauri::command]
pub async fn test_api_key(
    key_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<ApiKeyTestResult, String> {
    info!("Testing API key: {}", key_id);
    
    let key_uuid = Uuid::parse_str(&key_id)
        .map_err(|e| format!("Invalid key ID: {}", e))?;
    
    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.test_key(key_uuid).await {
        Ok(result) => {
            info!("API key test completed: {} - {}", key_id, if result.success { "SUCCESS" } else { "FAILED" });
            Ok(result)
        }
        Err(e) => {
            error!("Failed to test API key: {}", e);
            Err(e.to_string())
        }
    }
}

/// Import API keys from file
#[tauri::command]
pub async fn import_api_keys(
    keys: Vec<ApiKeyImport>,
    service_manager: State<'_, ServiceManager>,
) -> Result<ImportResult, String> {
    info!("Importing {} API keys", keys.len());
    
    let mut api_manager = service_manager.inner().api_manager.write().await;
    match api_manager.import_keys(keys).await {
        Ok(result) => {
            info!("API keys import completed: {} successful, {} failed", 
                  result.successful_count, result.failed_count);
            Ok(result)
        }
        Err(e) => {
            error!("Failed to import API keys: {}", e);
            Err(e.to_string())
        }
    }
}

/// Export API keys to file
#[tauri::command]
pub async fn export_api_keys(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<crate::models::ApiKeyExport>, String> {
    info!("Exporting API keys");
    
    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.export_keys().await {
        Ok(exported_keys) => {
            info!("API keys exported successfully: {} keys", exported_keys.len());
            Ok(exported_keys)
        }
        Err(e) => {
            error!("Failed to export API keys: {}", e);
            Err(e.to_string())
        }
    }
}

/// Import API keys from CSV content
#[tauri::command]
pub async fn import_api_keys_csv(
    csv_content: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<ImportResult, String> {
    info!("Importing API keys from CSV content");

    let mut api_manager = service_manager.inner().api_manager.write().await;
    match api_manager.import_keys_from_csv(&csv_content).await {
        Ok(result) => {
            info!("CSV import completed: {} successful, {} failed",
                  result.successful_count, result.failed_count);
            Ok(result)
        }
        Err(e) => {
            error!("Failed to import API keys from CSV: {}", e);
            Err(e.to_string())
        }
    }
}

/// Import API keys from JSON content
#[tauri::command]
pub async fn import_api_keys_json(
    json_content: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<ImportResult, String> {
    info!("Importing API keys from JSON content");

    let mut api_manager = service_manager.inner().api_manager.write().await;
    match api_manager.import_keys_from_json(&json_content).await {
        Ok(result) => {
            info!("JSON import completed: {} successful, {} failed",
                  result.successful_count, result.failed_count);
            Ok(result)
        }
        Err(e) => {
            error!("Failed to import API keys from JSON: {}", e);
            Err(e.to_string())
        }
    }
}

/// Export API keys to CSV format
#[tauri::command]
pub async fn export_api_keys_csv(
    service_manager: State<'_, ServiceManager>,
) -> Result<String, String> {
    info!("Exporting API keys to CSV format");

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.export_keys_to_csv().await {
        Ok(csv_content) => {
            info!("API keys exported to CSV successfully");
            Ok(csv_content)
        }
        Err(e) => {
            error!("Failed to export API keys to CSV: {}", e);
            Err(e.to_string())
        }
    }
}

/// Export API keys to JSON format
#[tauri::command]
pub async fn export_api_keys_json(
    service_manager: State<'_, ServiceManager>,
) -> Result<String, String> {
    info!("Exporting API keys to JSON format");

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.export_keys_to_json().await {
        Ok(json_content) => {
            info!("API keys exported to JSON successfully");
            Ok(json_content)
        }
        Err(e) => {
            error!("Failed to export API keys to JSON: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get usage statistics for an API key
#[tauri::command]
pub async fn get_api_key_usage_stats(
    key_id: String,
    days: u32,
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<(String, u32, u32, u32, f64)>, String> {
    info!("Getting usage statistics for API key: {}", key_id);

    let key_uuid = Uuid::parse_str(&key_id)
        .map_err(|e| format!("Invalid key ID: {}", e))?;

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.get_key_usage_stats(key_uuid, days).await {
        Ok(stats) => {
            info!("Retrieved {} usage statistics records", stats.len());
            Ok(stats)
        }
        Err(e) => {
            error!("Failed to get usage statistics: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get all API keys with their current status
#[tauri::command]
pub async fn get_api_keys_with_status(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<(ApiKey, bool)>, String> {
    info!("Getting all API keys with status");

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.get_keys_with_status().await {
        Ok(keys_with_status) => {
            info!("Retrieved {} API keys with status", keys_with_status.len());
            Ok(keys_with_status)
        }
        Err(e) => {
            error!("Failed to get API keys with status: {}", e);
            Err(e.to_string())
        }
    }
}

/// Check if a request can be made for a specific API key (rate limiting)
#[tauri::command]
pub async fn can_make_request(
    key_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<bool, String> {
    info!("Checking if request can be made for API key: {}", key_id);

    let key_uuid = Uuid::parse_str(&key_id)
        .map_err(|e| format!("Invalid key ID: {}", e))?;

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.can_make_request(key_uuid).await {
        Ok(can_make) => {
            info!("Request check result for key {}: {}", key_id, can_make);
            Ok(can_make)
        }
        Err(e) => {
            error!("Failed to check request permission: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get usage status for an API key
#[tauri::command]
pub async fn get_key_usage_status(
    key_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<UsageStatus, String> {
    info!("Getting usage status for API key: {}", key_id);

    let key_uuid = Uuid::parse_str(&key_id)
        .map_err(|e| format!("Invalid key ID: {}", e))?;

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.get_key_usage_status(key_uuid).await {
        Ok(status) => {
            info!("Retrieved usage status for key {}: {:.1}% used", key_id, status.usage_percentage);
            Ok(status)
        }
        Err(e) => {
            error!("Failed to get usage status: {}", e);
            Err(e.to_string())
        }
    }
}

/// Record an API request and check for rate limit violations
#[tauri::command]
pub async fn record_api_request(
    key_id: String,
    success: bool,
    service_manager: State<'_, ServiceManager>,
) -> Result<Option<RateLimitAlert>, String> {
    info!("Recording API request for key: {} (success: {})", key_id, success);

    let key_uuid = Uuid::parse_str(&key_id)
        .map_err(|e| format!("Invalid key ID: {}", e))?;

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.record_api_request(key_uuid, success).await {
        Ok(alert) => {
            if let Some(ref alert) = alert {
                warn!("Rate limit alert generated: {:?}", alert.alert_type);
            }
            Ok(alert)
        }
        Err(e) => {
            error!("Failed to record API request: {}", e);
            Err(e.to_string())
        }
    }
}

/// Enable or disable emergency stop
#[tauri::command]
pub async fn set_emergency_stop(
    enabled: bool,
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Setting emergency stop: {}", enabled);

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.set_emergency_stop(enabled).await {
        Ok(()) => {
            info!("Emergency stop set to: {}", enabled);
            Ok(())
        }
        Err(e) => {
            error!("Failed to set emergency stop: {}", e);
            Err(e.to_string())
        }
    }
}

/// Check if emergency stop is enabled
#[tauri::command]
pub async fn is_emergency_stop_enabled(
    service_manager: State<'_, ServiceManager>,
) -> Result<bool, String> {
    let api_manager = service_manager.inner().api_manager.read().await;
    let enabled = api_manager.is_emergency_stop_enabled().await;
    Ok(enabled)
}

/// Get recent rate limit alerts
#[tauri::command]
pub async fn get_recent_alerts(
    limit: usize,
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<RateLimitAlert>, String> {
    info!("Getting {} recent rate limit alerts", limit);

    let api_manager = service_manager.inner().api_manager.read().await;
    let alerts = api_manager.get_recent_alerts(limit).await;

    info!("Retrieved {} recent alerts", alerts.len());
    Ok(alerts)
}

/// Generate usage forecast for an API key
#[tauri::command]
pub async fn generate_usage_forecast(
    key_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<UsageForecast, String> {
    info!("Generating usage forecast for API key: {}", key_id);

    let key_uuid = Uuid::parse_str(&key_id)
        .map_err(|e| format!("Invalid key ID: {}", e))?;

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.generate_usage_forecast(key_uuid).await {
        Ok(forecast) => {
            info!("Generated forecast for key {}: {:.1}% confidence", key_id, forecast.confidence_level * 100.0);
            Ok(forecast)
        }
        Err(e) => {
            error!("Failed to generate usage forecast: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get usage analytics for all API keys
#[tauri::command]
pub async fn get_usage_analytics(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<(String, UsageStatus, UsageForecast)>, String> {
    info!("Getting usage analytics for all API keys");

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.get_usage_analytics().await {
        Ok(analytics) => {
            let result: Vec<_> = analytics.into_iter()
                .map(|(uuid, status, forecast)| (uuid.to_string(), status, forecast))
                .collect();

            info!("Retrieved analytics for {} API keys", result.len());
            Ok(result)
        }
        Err(e) => {
            error!("Failed to get usage analytics: {}", e);
            Err(e.to_string())
        }
    }
}

/// Check all API keys for threshold violations
#[tauri::command]
pub async fn check_all_thresholds(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<RateLimitAlert>, String> {
    info!("Checking all API keys for threshold violations");

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.check_all_thresholds().await {
        Ok(alerts) => {
            info!("Generated {} threshold violation alerts", alerts.len());
            Ok(alerts)
        }
        Err(e) => {
            error!("Failed to check thresholds: {}", e);
            Err(e.to_string())
        }
    }
}

/// Generate automated usage report
#[tauri::command]
pub async fn generate_usage_report(
    service_manager: State<'_, ServiceManager>,
) -> Result<String, String> {
    info!("Generating automated usage report");

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.generate_usage_report().await {
        Ok(report) => {
            info!("Generated usage report ({} characters)", report.len());
            Ok(report)
        }
        Err(e) => {
            error!("Failed to generate usage report: {}", e);
            Err(e.to_string())
        }
    }
}

/// Select the best available API key for a service using intelligent rotation
#[tauri::command]
pub async fn select_best_key_for_service(
    service: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Option<ApiKey>, String> {
    info!("Selecting best API key for service: {}", service);

    let service_provider = crate::models::api_key::ServiceProvider::from_str(&service)
        .ok_or_else(|| format!("Invalid service: {}", service))?;

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.select_best_key_for_service(service_provider).await {
        Ok(key) => {
            if let Some(ref key) = key {
                info!("Selected key {} for service {}", key.id, service);
            } else {
                warn!("No available keys found for service {}", service);
            }
            Ok(key)
        }
        Err(e) => {
            error!("Failed to select best key for service {}: {}", service, e);
            Err(e.to_string())
        }
    }
}

/// Record request performance for key rotation optimization
#[tauri::command]
pub async fn record_key_performance(
    key_id: String,
    success: bool,
    response_time_ms: u32,
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    debug!("Recording performance for key: {} (success: {}, time: {}ms)", key_id, success, response_time_ms);

    let key_uuid = Uuid::parse_str(&key_id)
        .map_err(|e| format!("Invalid key ID: {}", e))?;

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.record_key_performance(key_uuid, success, response_time_ms).await {
        Ok(()) => {
            debug!("Recorded performance for key: {}", key_id);
            Ok(())
        }
        Err(e) => {
            error!("Failed to record key performance: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get performance metrics for an API key
#[tauri::command]
pub async fn get_key_performance_metrics(
    key_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Option<KeyPerformanceMetrics>, String> {
    info!("Getting performance metrics for API key: {}", key_id);

    let key_uuid = Uuid::parse_str(&key_id)
        .map_err(|e| format!("Invalid key ID: {}", e))?;

    let api_manager = service_manager.inner().api_manager.read().await;
    let metrics = api_manager.get_key_performance_metrics(key_uuid).await;

    Ok(metrics)
}

/// Get performance metrics for all keys of a service
#[tauri::command]
pub async fn get_service_performance_metrics(
    service: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<KeyPerformanceMetrics>, String> {
    info!("Getting performance metrics for service: {}", service);

    let service_provider = crate::models::api_key::ServiceProvider::from_str(&service)
        .ok_or_else(|| format!("Invalid service: {}", service))?;

    let api_manager = service_manager.inner().api_manager.read().await;
    let metrics = api_manager.get_service_performance_metrics(service_provider).await;

    info!("Retrieved performance metrics for {} keys in service {}", metrics.len(), service);
    Ok(metrics)
}

/// Get all performance metrics
#[tauri::command]
pub async fn get_all_performance_metrics(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<(String, KeyPerformanceMetrics)>, String> {
    info!("Getting all performance metrics");

    let api_manager = service_manager.inner().api_manager.read().await;
    let metrics = api_manager.get_all_performance_metrics().await;

    let result: Vec<_> = metrics.into_iter()
        .map(|(uuid, metrics)| (uuid.to_string(), metrics))
        .collect();

    info!("Retrieved performance metrics for {} keys", result.len());
    Ok(result)
}

/// Get rotation analytics
#[tauri::command]
pub async fn get_rotation_analytics(
    service_manager: State<'_, ServiceManager>,
) -> Result<RotationAnalytics, String> {
    info!("Getting rotation analytics");

    let api_manager = service_manager.inner().api_manager.read().await;
    let analytics = api_manager.get_rotation_analytics().await;

    info!("Retrieved rotation analytics: {} total rotations", analytics.total_rotations);
    Ok(analytics)
}

/// Perform health check on all API keys
#[tauri::command]
pub async fn perform_health_check(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<(String, KeyHealth)>, String> {
    info!("Performing health check on all API keys");

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.perform_health_check().await {
        Ok(health_status) => {
            let result: Vec<_> = health_status.into_iter()
                .map(|(uuid, health)| (uuid.to_string(), health))
                .collect();

            info!("Health check completed for {} keys", result.len());
            Ok(result)
        }
        Err(e) => {
            error!("Failed to perform health check: {}", e);
            Err(e.to_string())
        }
    }
}

/// Reactivate keys that have completed their cooldown
#[tauri::command]
pub async fn reactivate_cooled_down_keys(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<String>, String> {
    info!("Reactivating cooled down keys");

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.reactivate_cooled_down_keys().await {
        Ok(reactivated_keys) => {
            let result: Vec<_> = reactivated_keys.into_iter()
                .map(|uuid| uuid.to_string())
                .collect();

            info!("Reactivated {} keys from cooldown", result.len());
            Ok(result)
        }
        Err(e) => {
            error!("Failed to reactivate cooled down keys: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get keys that need attention (unhealthy, failed, or in cooldown)
#[tauri::command]
pub async fn get_keys_needing_attention(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<(String, KeyPerformanceMetrics)>, String> {
    info!("Getting keys that need attention");

    let api_manager = service_manager.inner().api_manager.read().await;
    let keys_needing_attention = api_manager.get_keys_needing_attention().await;

    let result: Vec<_> = keys_needing_attention.into_iter()
        .map(|(uuid, metrics)| (uuid.to_string(), metrics))
        .collect();

    info!("Found {} keys needing attention", result.len());
    Ok(result)
}

/// Generate rotation report
#[tauri::command]
pub async fn generate_rotation_report(
    service_manager: State<'_, ServiceManager>,
) -> Result<String, String> {
    info!("Generating rotation report");

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.generate_rotation_report().await {
        Ok(report) => {
            info!("Generated rotation report ({} characters)", report.len());
            Ok(report)
        }
        Err(e) => {
            error!("Failed to generate rotation report: {}", e);
            Err(e.to_string())
        }
    }
}

/// Make a service request through the integration framework
#[tauri::command]
pub async fn make_service_request(
    service: String,
    request: ServiceRequest,
    service_manager: State<'_, ServiceManager>,
) -> Result<ServiceResponse, String> {
    info!("Making service request to: {}", service);

    let service_provider = crate::models::api_key::ServiceProvider::from_str(&service)
        .ok_or_else(|| format!("Invalid service: {}", service))?;

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.make_service_request(service_provider, request).await {
        Ok(response) => {
            info!("Service request completed successfully for: {}", service);
            Ok(response)
        }
        Err(e) => {
            error!("Service request failed for {}: {}", service, e);
            Err(e.to_string())
        }
    }
}

/// Check service health
#[tauri::command]
pub async fn check_service_health(
    service: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<ServiceHealth, String> {
    info!("Checking health for service: {}", service);

    let service_provider = crate::models::api_key::ServiceProvider::from_str(&service)
        .ok_or_else(|| format!("Invalid service: {}", service))?;

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.check_service_health(service_provider).await {
        Ok(health) => {
            info!("Health check completed for {}: {:?}", service, health);
            Ok(health)
        }
        Err(e) => {
            error!("Health check failed for {}: {}", service, e);
            Err(e.to_string())
        }
    }
}

/// Get service metrics
#[tauri::command]
pub async fn get_service_metrics(
    service: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Option<ServiceMetrics>, String> {
    info!("Getting metrics for service: {}", service);

    let service_provider = crate::models::api_key::ServiceProvider::from_str(&service)
        .ok_or_else(|| format!("Invalid service: {}", service))?;

    let api_manager = service_manager.inner().api_manager.read().await;
    let metrics = api_manager.get_service_metrics(service_provider).await;

    Ok(metrics)
}

/// Get all service metrics
#[tauri::command]
pub async fn get_all_service_metrics(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<(String, ServiceMetrics)>, String> {
    info!("Getting all service metrics");

    let api_manager = service_manager.inner().api_manager.read().await;
    let metrics = api_manager.get_all_service_metrics().await;

    let result: Vec<_> = metrics.into_iter()
        .map(|(service, metrics)| (format!("{:?}", service).to_lowercase(), metrics))
        .collect();

    info!("Retrieved metrics for {} services", result.len());
    Ok(result)
}

/// Get service configuration
#[tauri::command]
pub async fn get_service_config(
    service: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Option<ServiceConfig>, String> {
    info!("Getting configuration for service: {}", service);

    let service_provider = crate::models::api_key::ServiceProvider::from_str(&service)
        .ok_or_else(|| format!("Invalid service: {}", service))?;

    let api_manager = service_manager.inner().api_manager.read().await;
    let config = api_manager.get_service_config(service_provider).await;

    Ok(config)
}

/// Get all service configurations
#[tauri::command]
pub async fn get_all_service_configs(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<(String, ServiceConfig)>, String> {
    info!("Getting all service configurations");

    let api_manager = service_manager.inner().api_manager.read().await;
    let configs = api_manager.get_all_service_configs().await;

    let result: Vec<_> = configs.into_iter()
        .map(|(service, config)| (format!("{:?}", service).to_lowercase(), config))
        .collect();

    info!("Retrieved configurations for {} services", result.len());
    Ok(result)
}

/// Update service configuration
#[tauri::command]
pub async fn update_service_config(
    service: String,
    config: ServiceConfig,
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Updating configuration for service: {}", service);

    let service_provider = crate::models::api_key::ServiceProvider::from_str(&service)
        .ok_or_else(|| format!("Invalid service: {}", service))?;

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.update_service_config(service_provider, config).await {
        Ok(()) => {
            info!("Configuration updated successfully for service: {}", service);
            Ok(())
        }
        Err(e) => {
            error!("Failed to update configuration for {}: {}", service, e);
            Err(e.to_string())
        }
    }
}

/// Get available endpoints for a service
#[tauri::command]
pub async fn get_service_endpoints(
    service: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<String>, String> {
    info!("Getting endpoints for service: {}", service);

    let service_provider = crate::models::api_key::ServiceProvider::from_str(&service)
        .ok_or_else(|| format!("Invalid service: {}", service))?;

    let api_manager = service_manager.inner().api_manager.read().await;
    let endpoints = api_manager.get_service_endpoints(service_provider).await;

    info!("Retrieved {} endpoints for service: {}", endpoints.len(), service);
    Ok(endpoints)
}

/// Get list of registered services
#[tauri::command]
pub async fn get_registered_services(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<String>, String> {
    info!("Getting list of registered services");

    let api_manager = service_manager.inner().api_manager.read().await;
    let services = api_manager.get_registered_services().await;

    let result: Vec<_> = services.into_iter()
        .map(|service| format!("{:?}", service).to_lowercase())
        .collect();

    info!("Retrieved {} registered services", result.len());
    Ok(result)
}

/// Generate service status report
#[tauri::command]
pub async fn generate_service_status_report(
    service_manager: State<'_, ServiceManager>,
) -> Result<String, String> {
    info!("Generating service status report");

    let api_manager = service_manager.inner().api_manager.read().await;
    match api_manager.generate_service_status_report().await {
        Ok(report) => {
            info!("Generated service status report ({} characters)", report.len());
            Ok(report)
        }
        Err(e) => {
            error!("Failed to generate service status report: {}", e);
            Err(e.to_string())
        }
    }
}


