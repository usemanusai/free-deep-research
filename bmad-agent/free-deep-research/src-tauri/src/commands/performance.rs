use crate::services::ServiceManager;
use crate::services::performance::{ComprehensivePerformanceMetrics, OptimizationRecommendation};
use tauri::State;
use tracing::{info, debug, error};

/// Get comprehensive performance metrics
#[tauri::command]
pub async fn get_performance_metrics(
    service_manager: State<'_, ServiceManager>,
) -> Result<ComprehensivePerformanceMetrics, String> {
    info!("Getting comprehensive performance metrics");
    
    let performance_service = service_manager.inner().performance.read().await;
    match performance_service.get_performance_metrics().await {
        Ok(metrics) => {
            debug!("Retrieved performance metrics successfully");
            Ok(metrics)
        }
        Err(e) => {
            error!("Failed to get performance metrics: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get performance optimization recommendations
#[tauri::command]
pub async fn get_optimization_recommendations(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<OptimizationRecommendation>, String> {
    info!("Getting performance optimization recommendations");
    
    let performance_service = service_manager.inner().performance.read().await;
    match performance_service.optimize_performance().await {
        Ok(recommendations) => {
            info!("Generated {} optimization recommendations", recommendations.len());
            Ok(recommendations)
        }
        Err(e) => {
            error!("Failed to get optimization recommendations: {}", e);
            Err(e.to_string())
        }
    }
}

/// Clear all performance caches
#[tauri::command]
pub async fn clear_performance_caches(
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Clearing all performance caches");
    
    let performance_service = service_manager.inner().performance.read().await;
    match performance_service.clear_all_caches().await {
        Ok(()) => {
            info!("Performance caches cleared successfully");
            Ok(())
        }
        Err(e) => {
            error!("Failed to clear performance caches: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get cache statistics
#[tauri::command]
pub async fn get_cache_statistics(
    service_manager: State<'_, ServiceManager>,
) -> Result<crate::services::performance::CacheStatistics, String> {
    debug!("Getting cache statistics");
    
    let performance_service = service_manager.inner().performance.read().await;
    let caching_service = performance_service.caching_service();
    let statistics = caching_service.get_statistics().await;
    
    debug!("Retrieved cache statistics successfully");
    Ok(statistics)
}

/// Get request deduplication statistics
#[tauri::command]
pub async fn get_deduplication_statistics(
    service_manager: State<'_, ServiceManager>,
) -> Result<crate::services::performance::DeduplicationStatistics, String> {
    debug!("Getting request deduplication statistics");
    
    let performance_service = service_manager.inner().performance.read().await;
    let deduplication_service = performance_service.deduplication_service();
    let statistics = deduplication_service.get_statistics().await;
    
    debug!("Retrieved deduplication statistics successfully");
    Ok(statistics)
}

/// Get background processing statistics
#[tauri::command]
pub async fn get_background_processing_statistics(
    service_manager: State<'_, ServiceManager>,
) -> Result<crate::services::performance::BackgroundProcessingStatistics, String> {
    debug!("Getting background processing statistics");
    
    let performance_service = service_manager.inner().performance.read().await;
    let background_processor = performance_service.background_processor();
    let statistics = background_processor.get_statistics().await;
    
    debug!("Retrieved background processing statistics successfully");
    Ok(statistics)
}

/// Submit a background task
#[tauri::command]
pub async fn submit_background_task(
    task_name: String,
    task_type: String,
    task_data: serde_json::Value,
    priority: Option<String>,
    service_manager: State<'_, ServiceManager>,
) -> Result<String, String> {
    info!("Submitting background task: {} ({})", task_name, task_type);
    
    let task_priority = match priority.as_deref() {
        Some("low") => crate::services::performance::TaskPriority::Low,
        Some("normal") => crate::services::performance::TaskPriority::Normal,
        Some("high") => crate::services::performance::TaskPriority::High,
        Some("critical") => crate::services::performance::TaskPriority::Critical,
        _ => crate::services::performance::TaskPriority::Normal,
    };
    
    let task = crate::services::performance::BackgroundTask {
        id: uuid::Uuid::new_v4(),
        name: task_name.clone(),
        priority: task_priority,
        created_at: chrono::Utc::now(),
        scheduled_for: None,
        max_retries: 3,
        retry_count: 0,
        timeout_seconds: Some(300),
        task_data,
        task_type,
    };
    
    let performance_service = service_manager.inner().performance.read().await;
    let background_processor = performance_service.background_processor();
    
    match background_processor.submit_task(task).await {
        Ok(task_id) => {
            info!("Background task submitted successfully: {}", task_id);
            Ok(task_id.to_string())
        }
        Err(e) => {
            error!("Failed to submit background task: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get background task status
#[tauri::command]
pub async fn get_background_task_status(
    task_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Option<String>, String> {
    debug!("Getting background task status: {}", task_id);
    
    let task_uuid = uuid::Uuid::parse_str(&task_id)
        .map_err(|e| format!("Invalid task ID: {}", e))?;
    
    let performance_service = service_manager.inner().performance.read().await;
    let background_processor = performance_service.background_processor();
    
    match background_processor.get_task_status(task_uuid).await {
        Ok(Some(status)) => {
            let status_str = match status {
                crate::services::performance::background_processor::TaskStatus::Queued(_) => "queued".to_string(),
                crate::services::performance::background_processor::TaskStatus::Running(_) => "running".to_string(),
                crate::services::performance::background_processor::TaskStatus::Completed(result) => {
                    if result.success {
                        "completed".to_string()
                    } else {
                        format!("failed: {}", result.error_message.unwrap_or_else(|| "Unknown error".to_string()))
                    }
                }
            };
            Ok(Some(status_str))
        }
        Ok(None) => {
            debug!("Background task not found: {}", task_id);
            Ok(None)
        }
        Err(e) => {
            error!("Failed to get background task status: {}", e);
            Err(e.to_string())
        }
    }
}

/// Cancel a background task
#[tauri::command]
pub async fn cancel_background_task(
    task_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<bool, String> {
    info!("Cancelling background task: {}", task_id);
    
    let task_uuid = uuid::Uuid::parse_str(&task_id)
        .map_err(|e| format!("Invalid task ID: {}", e))?;
    
    let performance_service = service_manager.inner().performance.read().await;
    let background_processor = performance_service.background_processor();
    
    match background_processor.cancel_task(task_uuid).await {
        Ok(cancelled) => {
            if cancelled {
                info!("Background task cancelled successfully: {}", task_id);
            } else {
                info!("Background task could not be cancelled (may be running): {}", task_id);
            }
            Ok(cancelled)
        }
        Err(e) => {
            error!("Failed to cancel background task: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get connection pool statistics
#[tauri::command]
pub async fn get_connection_pool_statistics(
    service_manager: State<'_, ServiceManager>,
) -> Result<crate::services::performance::PoolStatistics, String> {
    debug!("Getting connection pool statistics");
    
    let performance_service = service_manager.inner().performance.read().await;
    let connection_pool = performance_service.connection_pool();
    let statistics = connection_pool.get_statistics().await;
    
    debug!("Retrieved connection pool statistics successfully");
    Ok(statistics)
}

/// Perform performance health check
#[tauri::command]
pub async fn performance_health_check(
    service_manager: State<'_, ServiceManager>,
) -> Result<bool, String> {
    debug!("Performing performance health check");
    
    let performance_service = service_manager.inner().performance.read().await;
    match performance_service.health_check().await {
        Ok(()) => {
            debug!("Performance health check passed");
            Ok(true)
        }
        Err(e) => {
            error!("Performance health check failed: {}", e);
            Ok(false) // Return false instead of error for health checks
        }
    }
}
