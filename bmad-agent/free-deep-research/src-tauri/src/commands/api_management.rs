use tauri::State;
use uuid::Uuid;
use tracing::{info, error};

use crate::error::AppResult;
use crate::models::{ApiKey, CreateApiKeyRequest, UpdateApiKeyRequest, ApiKeyTestResult, ApiKeyImport};
use crate::services::ServiceManager;

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

/// Result of API key import operation
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ImportResult {
    pub successful_count: u32,
    pub failed_count: u32,
    pub errors: Vec<ImportError>,
}

/// Import error details
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ImportError {
    pub index: usize,
    pub service: String,
    pub name: String,
    pub error: String,
}
