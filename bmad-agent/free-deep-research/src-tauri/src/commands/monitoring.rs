use tauri::State;
use tracing::{info, error};

use crate::error::AppResult;
use crate::models::MonitoringMetrics;
use crate::services::{ServiceManager, ServiceHealthStatus};

/// Get system metrics
#[tauri::command]
pub async fn get_system_metrics(
    service_manager: State<'_, ServiceManager>,
) -> Result<MonitoringMetrics, String> {
    info!("Getting system metrics");
    
    // TODO: Implement actual metrics retrieval
    Err("Not implemented".to_string())
}

/// Get API usage statistics
#[tauri::command]
pub async fn get_api_usage_stats(
    service_manager: State<'_, ServiceManager>,
) -> Result<serde_json::Value, String> {
    info!("Getting API usage statistics");
    
    // TODO: Implement actual usage stats retrieval
    Err("Not implemented".to_string())
}

/// Get service health status
#[tauri::command]
pub async fn get_service_health(
    service_manager: State<'_, ServiceManager>,
) -> Result<ServiceHealthStatus, String> {
    info!("Getting service health status");
    
    match service_manager.health_check().await {
        Ok(status) => Ok(status),
        Err(e) => {
            error!("Failed to get service health: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get audit logs
#[tauri::command]
pub async fn get_audit_logs(
    limit: Option<u32>,
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<crate::services::security::AuditEvent>, String> {
    info!("Getting audit logs with limit: {:?}", limit);
    
    let security = service_manager.security.read().await;
    match security.get_audit_logs(limit).await {
        Ok(logs) => Ok(logs),
        Err(e) => {
            error!("Failed to get audit logs: {}", e);
            Err(e.to_string())
        }
    }
}
