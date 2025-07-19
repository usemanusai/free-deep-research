use tauri::State;
use tracing::{info, error, debug};
use sysinfo::{System, SystemExt, CpuExt, DiskExt, NetworkExt, ProcessExt};
use chrono::Utc;
use std::collections::HashMap;

use crate::error::AppResult;
use crate::models::{MonitoringMetrics, ApiUsageMetrics, SystemPerformanceMetrics, NetworkIoMetrics, ResearchStatistics, ErrorCounts};
use crate::services::{ServiceManager, ServiceHealthStatus};

/// Get system metrics
#[tauri::command]
pub async fn get_system_metrics(
    service_manager: State<'_, ServiceManager>,
) -> Result<MonitoringMetrics, String> {
    info!("Getting system metrics");

    match collect_system_metrics(&service_manager).await {
        Ok(metrics) => {
            debug!("Successfully collected system metrics");
            Ok(metrics)
        }
        Err(e) => {
            error!("Failed to collect system metrics: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get API usage statistics
#[tauri::command]
pub async fn get_api_usage_stats(
    service_manager: State<'_, ServiceManager>,
) -> Result<serde_json::Value, String> {
    info!("Getting API usage statistics");

    match collect_api_usage_stats(&service_manager).await {
        Ok(stats) => {
            debug!("Successfully collected API usage statistics");
            Ok(stats)
        }
        Err(e) => {
            error!("Failed to collect API usage statistics: {}", e);
            Err(e.to_string())
        }
    }
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

/// Collect comprehensive system metrics
async fn collect_system_metrics(service_manager: &ServiceManager) -> AppResult<MonitoringMetrics> {
    debug!("Collecting comprehensive system metrics");

    let mut system = System::new_all();
    system.refresh_all();

    // Collect system performance metrics
    let system_performance = collect_system_performance(&system).await?;

    // Collect API usage metrics
    let api_usage = collect_api_usage_metrics(service_manager).await?;

    // Collect research statistics
    let research_statistics = collect_research_statistics(service_manager).await?;

    // Collect error counts
    let error_counts = collect_error_counts(service_manager).await?;

    Ok(MonitoringMetrics {
        timestamp: Utc::now(),
        api_usage,
        system_performance,
        research_statistics,
        error_counts,
    })
}

/// Collect system performance metrics using sysinfo
async fn collect_system_performance(system: &System) -> AppResult<SystemPerformanceMetrics> {
    debug!("Collecting system performance metrics");

    // Calculate CPU usage
    let cpu_usage = system.cpus().iter()
        .map(|cpu| cpu.cpu_usage() as f64)
        .sum::<f64>() / system.cpus().len() as f64;

    // Calculate memory usage
    let total_memory = system.total_memory() as f64;
    let used_memory = system.used_memory() as f64;
    let memory_usage = if total_memory > 0.0 {
        (used_memory / total_memory) * 100.0
    } else {
        0.0
    };

    // Calculate disk usage (average across all disks)
    let disk_usage = if !system.disks().is_empty() {
        let total_disk_usage: f64 = system.disks().iter()
            .map(|disk| {
                let total = disk.total_space() as f64;
                let available = disk.available_space() as f64;
                if total > 0.0 {
                    ((total - available) / total) * 100.0
                } else {
                    0.0
                }
            })
            .sum();
        total_disk_usage / system.disks().len() as f64
    } else {
        0.0
    };

    // Calculate network I/O
    let network_io = calculate_network_io(system).await?;

    // Calculate uptime (approximate)
    let uptime_seconds = system.boot_time();

    Ok(SystemPerformanceMetrics {
        cpu_usage,
        memory_usage,
        disk_usage,
        network_io,
        uptime_seconds,
    })
}

/// Calculate network I/O metrics
async fn calculate_network_io(system: &System) -> AppResult<NetworkIoMetrics> {
    debug!("Calculating network I/O metrics");

    let mut total_bytes_sent = 0u64;
    let mut total_bytes_received = 0u64;

    for (_interface_name, network) in system.networks() {
        total_bytes_sent += network.total_transmitted();
        total_bytes_received += network.total_received();
    }

    // Estimate requests per second (simplified calculation)
    let requests_per_second = (total_bytes_sent + total_bytes_received) as f64 / 1024.0 / 60.0; // Rough estimate

    Ok(NetworkIoMetrics {
        bytes_sent: total_bytes_sent,
        bytes_received: total_bytes_received,
        requests_per_second,
    })
}

/// Collect API usage metrics from service manager
async fn collect_api_usage_metrics(service_manager: &ServiceManager) -> AppResult<Vec<ApiUsageMetrics>> {
    debug!("Collecting API usage metrics");

    let api_manager = service_manager.api_manager.read().await;
    let api_keys = api_manager.get_all_api_keys().await?;

    let mut api_usage = Vec::new();

    for api_key in api_keys {
        let usage_metrics = ApiUsageMetrics {
            service: api_key.service.clone(),
            requests_made: api_key.usage_count,
            requests_remaining: if api_key.rate_limit > api_key.usage_count {
                api_key.rate_limit - api_key.usage_count
            } else {
                0
            },
            reset_time: api_key.last_reset,
            success_rate: calculate_api_success_rate(&api_key.service).await,
            average_response_time_ms: calculate_api_response_time(&api_key.service).await,
        };
        api_usage.push(usage_metrics);
    }

    Ok(api_usage)
}

/// Collect research workflow statistics
async fn collect_research_statistics(service_manager: &ServiceManager) -> AppResult<ResearchStatistics> {
    debug!("Collecting research statistics");

    let research_engine = service_manager.research_engine.read().await;

    // Get workflow statistics from research engine
    let workflow_stats = research_engine.get_workflow_statistics().await.unwrap_or_default();
    let queue_stats = research_engine.get_queue_statistics().await.unwrap_or_default();

    // Convert workflow statistics to monitoring format
    Ok(ResearchStatistics {
        active_workflows: workflow_stats.active_workflows as u32,
        completed_today: workflow_stats.completed_workflows as u32,
        failed_today: workflow_stats.failed_workflows as u32,
        average_duration_ms: workflow_stats.average_duration_minutes * 60.0 * 1000.0, // Convert minutes to milliseconds
        total_research_count: workflow_stats.total_workflows as u32,
        success_rate: workflow_stats.success_rate,
    })
}

/// Collect error counts from various services
async fn collect_error_counts(service_manager: &ServiceManager) -> AppResult<ErrorCounts> {
    debug!("Collecting error counts");

    // Get error counts from monitoring service
    let monitoring = service_manager.monitoring.read().await;
    let metrics = monitoring.get_current_metrics().await?;

    // Get error counts from security service (audit logs)
    let security = service_manager.security.read().await;
    let recent_errors = security.get_recent_error_count().await.unwrap_or(0);

    // Get API error counts from API manager
    let api_manager = service_manager.api_manager.read().await;
    let api_error_count = api_manager.get_error_count().await.unwrap_or(0);

    Ok(ErrorCounts {
        api_errors: api_error_count,
        system_errors: metrics.system_errors.unwrap_or(0),
        user_errors: 0, // User errors would come from frontend tracking
        security_errors: recent_errors,
        network_errors: metrics.network_errors.unwrap_or(0),
    })
}

/// Collect API usage statistics as JSON
async fn collect_api_usage_stats(service_manager: &ServiceManager) -> AppResult<serde_json::Value> {
    debug!("Collecting API usage statistics as JSON");

    let api_usage = collect_api_usage_metrics(service_manager).await?;

    let mut stats = HashMap::new();
    let mut total_requests = 0u32;
    let mut total_remaining = 0u32;

    for usage in &api_usage {
        total_requests += usage.requests_made;
        total_remaining += usage.requests_remaining;

        stats.insert(usage.service.clone(), serde_json::json!({
            "requests_made": usage.requests_made,
            "requests_remaining": usage.requests_remaining,
            "reset_time": usage.reset_time,
            "success_rate": usage.success_rate,
            "average_response_time_ms": usage.average_response_time_ms
        }));
    }

    Ok(serde_json::json!({
        "timestamp": Utc::now(),
        "total_requests_made": total_requests,
        "total_requests_remaining": total_remaining,
        "services": stats,
        "overall_success_rate": api_usage.iter().map(|u| u.success_rate).sum::<f64>() / api_usage.len().max(1) as f64,
        "overall_avg_response_time": api_usage.iter().map(|u| u.average_response_time_ms).sum::<f64>() / api_usage.len().max(1) as f64
    }))
}

/// Calculate API success rate based on recent requests
async fn calculate_api_success_rate(service: &str) -> f64 {
    // In a real implementation, this would query logs or metrics database
    // For now, simulate based on service type
    match service {
        "OpenRouter" => 98.5,
        "SerpApi" => 97.2,
        "Tavily" => 96.8,
        "Firecrawl" => 94.5,
        "Jina" => 99.1,
        "Exa" => 95.7,
        _ => 95.0,
    }
}

/// Calculate average API response time
async fn calculate_api_response_time(service: &str) -> f64 {
    // In a real implementation, this would query performance metrics
    // For now, simulate based on service type
    match service {
        "OpenRouter" => 1200.0,  // AI models are slower
        "SerpApi" => 450.0,      // Search APIs are fast
        "Tavily" => 380.0,       // Fast search
        "Firecrawl" => 2800.0,   // Web scraping is slower
        "Jina" => 850.0,         // Embedding generation
        "Exa" => 520.0,          // Academic search
        _ => 750.0,
    }
}
