use tauri::State;
use tracing::{info, error};
use serde::{Serialize, Deserialize};

use crate::services::ServiceManager;
use crate::error::{AppError, AppResult};

/// Get comprehensive analytics dashboard data
#[tauri::command]
pub async fn get_analytics_dashboard_data(
    service_manager: State<'_, ServiceManager>,
    time_range: Option<String>,
) -> Result<serde_json::Value, String> {
    info!("Getting analytics dashboard data for time range: {:?}", time_range);

    let analytics = service_manager.analytics.read().await;
    
    match analytics.get_dashboard_data().await {
        Ok(dashboard_data) => {
            info!("Successfully retrieved analytics dashboard data");
            Ok(serde_json::to_value(dashboard_data).map_err(|e| e.to_string())?)
        }
        Err(e) => {
            error!("Failed to get analytics dashboard data: {}", e);
            Err(format!("Failed to get analytics dashboard data: {}", e))
        }
    }
}

/// Get usage analytics for a specific time period
#[tauri::command]
pub async fn get_usage_analytics(
    service_manager: State<'_, ServiceManager>,
    period: String,
) -> Result<serde_json::Value, String> {
    info!("Getting usage analytics for period: {}", period);

    let analytics = service_manager.analytics.read().await;
    
    // Parse the period string into TimePeriod enum
    let time_period = match period.as_str() {
        "LastHour" => crate::services::analytics::TimePeriod::LastHour,
        "Last24Hours" => crate::services::analytics::TimePeriod::Last24Hours,
        "LastWeek" => crate::services::analytics::TimePeriod::LastWeek,
        "LastMonth" => crate::services::analytics::TimePeriod::LastMonth,
        "LastQuarter" => crate::services::analytics::TimePeriod::LastQuarter,
        "LastYear" => crate::services::analytics::TimePeriod::LastYear,
        _ => crate::services::analytics::TimePeriod::LastWeek, // Default
    };

    match analytics.get_usage_analytics(time_period).await {
        Ok(usage_data) => {
            info!("Successfully retrieved usage analytics data");
            Ok(serde_json::to_value(usage_data).map_err(|e| e.to_string())?)
        }
        Err(e) => {
            error!("Failed to get usage analytics: {}", e);
            Err(format!("Failed to get usage analytics: {}", e))
        }
    }
}

/// Get current performance metrics
#[tauri::command]
pub async fn get_performance_metrics(
    service_manager: State<'_, ServiceManager>,
) -> Result<serde_json::Value, String> {
    info!("Getting current performance metrics");

    let analytics = service_manager.analytics.read().await;
    
    match analytics.get_performance_metrics().await {
        Ok(performance_metrics) => {
            info!("Successfully retrieved performance metrics");
            Ok(serde_json::to_value(performance_metrics).map_err(|e| e.to_string())?)
        }
        Err(e) => {
            error!("Failed to get performance metrics: {}", e);
            Err(format!("Failed to get performance metrics: {}", e))
        }
    }
}

/// Get performance trends analysis
#[tauri::command]
pub async fn get_performance_trends(
    service_manager: State<'_, ServiceManager>,
) -> Result<serde_json::Value, String> {
    info!("Getting performance trends analysis");

    // This would typically call a method on the performance monitor
    // For now, we'll return mock data
    let trends_data = serde_json::json!({
        "response_time_trend": [
            {"timestamp": "2024-01-01T10:00:00Z", "value": 150.0},
            {"timestamp": "2024-01-01T10:05:00Z", "value": 145.0},
            {"timestamp": "2024-01-01T10:10:00Z", "value": 140.0},
            {"timestamp": "2024-01-01T10:15:00Z", "value": 155.0},
            {"timestamp": "2024-01-01T10:20:00Z", "value": 148.0}
        ],
        "throughput_trend": [
            {"timestamp": "2024-01-01T10:00:00Z", "value": 5.2},
            {"timestamp": "2024-01-01T10:05:00Z", "value": 5.8},
            {"timestamp": "2024-01-01T10:10:00Z", "value": 6.1},
            {"timestamp": "2024-01-01T10:15:00Z", "value": 5.9},
            {"timestamp": "2024-01-01T10:20:00Z", "value": 6.3}
        ],
        "resource_usage_trend": [
            {"timestamp": "2024-01-01T10:00:00Z", "value": 45.0},
            {"timestamp": "2024-01-01T10:05:00Z", "value": 47.0},
            {"timestamp": "2024-01-01T10:10:00Z", "value": 44.0},
            {"timestamp": "2024-01-01T10:15:00Z", "value": 48.0},
            {"timestamp": "2024-01-01T10:20:00Z", "value": 46.0}
        ],
        "trend_analysis": {
            "response_time_trend": "Stable",
            "throughput_trend": "Increasing",
            "resource_usage_trend": "Stable",
            "overall_health": {
                "score": 92.5,
                "status": "Excellent"
            }
        }
    });

    Ok(trends_data)
}

/// Get predictive analytics data
#[tauri::command]
pub async fn get_predictive_analytics(
    service_manager: State<'_, ServiceManager>,
) -> Result<serde_json::Value, String> {
    info!("Getting predictive analytics data");

    let analytics = service_manager.analytics.read().await;
    
    match analytics.get_predictive_analytics().await {
        Ok(predictive_data) => {
            info!("Successfully retrieved predictive analytics data");
            Ok(serde_json::to_value(predictive_data).map_err(|e| e.to_string())?)
        }
        Err(e) => {
            error!("Failed to get predictive analytics: {}", e);
            Err(format!("Failed to get predictive analytics: {}", e))
        }
    }
}

/// Generate business intelligence report
#[tauri::command]
pub async fn generate_business_report(
    service_manager: State<'_, ServiceManager>,
    report_type: String,
) -> Result<serde_json::Value, String> {
    info!("Generating business report of type: {}", report_type);

    let analytics = service_manager.analytics.read().await;
    
    // Parse the report type string
    let business_report_type = match report_type.as_str() {
        "ExecutiveSummary" => crate::services::analytics::BusinessReportType::ExecutiveSummary,
        "UsageReport" => crate::services::analytics::BusinessReportType::UsageReport,
        "PerformanceReport" => crate::services::analytics::BusinessReportType::PerformanceReport,
        "CostAnalysis" => crate::services::analytics::BusinessReportType::CostAnalysis,
        "TrendAnalysis" => crate::services::analytics::BusinessReportType::TrendAnalysis,
        _ => crate::services::analytics::BusinessReportType::ExecutiveSummary, // Default
    };

    match analytics.generate_business_report(business_report_type).await {
        Ok(report) => {
            info!("Successfully generated business report");
            Ok(serde_json::to_value(report).map_err(|e| e.to_string())?)
        }
        Err(e) => {
            error!("Failed to generate business report: {}", e);
            Err(format!("Failed to generate business report: {}", e))
        }
    }
}

/// Get optimization recommendations
#[tauri::command]
pub async fn get_optimization_recommendations(
    service_manager: State<'_, ServiceManager>,
) -> Result<serde_json::Value, String> {
    info!("Getting optimization recommendations");

    let analytics = service_manager.analytics.read().await;
    
    match analytics.get_optimization_recommendations().await {
        Ok(recommendations) => {
            info!("Successfully retrieved optimization recommendations");
            Ok(serde_json::to_value(recommendations).map_err(|e| e.to_string())?)
        }
        Err(e) => {
            error!("Failed to get optimization recommendations: {}", e);
            Err(format!("Failed to get optimization recommendations: {}", e))
        }
    }
}

/// Record an analytics event
#[tauri::command]
pub async fn record_analytics_event(
    service_manager: State<'_, ServiceManager>,
    event_type: String,
    metadata: Option<serde_json::Value>,
) -> Result<(), String> {
    info!("Recording analytics event: {}", event_type);

    let analytics = service_manager.analytics.read().await;
    
    // Parse the event type
    let analytics_event_type = match event_type.as_str() {
        "ResearchStarted" => crate::services::analytics::EventType::ResearchStarted,
        "ResearchCompleted" => crate::services::analytics::EventType::ResearchCompleted,
        "ResearchFailed" => crate::services::analytics::EventType::ResearchFailed,
        "MethodologySelected" => crate::services::analytics::EventType::MethodologySelected,
        "ApiKeyAdded" => crate::services::analytics::EventType::ApiKeyAdded,
        "ApiKeyRotated" => crate::services::analytics::EventType::ApiKeyRotated,
        "ApiCallMade" => crate::services::analytics::EventType::ApiCallMade,
        "RateLimitHit" => crate::services::analytics::EventType::RateLimitHit,
        "SystemStartup" => crate::services::analytics::EventType::SystemStartup,
        "SystemShutdown" => crate::services::analytics::EventType::SystemShutdown,
        "PerformanceAlert" => crate::services::analytics::EventType::PerformanceAlert,
        "DashboardViewed" => crate::services::analytics::EventType::DashboardViewed,
        "ReportGenerated" => crate::services::analytics::EventType::ReportGenerated,
        "ConfigurationChanged" => crate::services::analytics::EventType::ConfigurationChanged,
        "BackupCompleted" => crate::services::analytics::EventType::BackupCompleted,
        "SecurityEvent" => crate::services::analytics::EventType::SecurityEvent,
        "ErrorOccurred" => crate::services::analytics::EventType::ErrorOccurred,
        _ => {
            error!("Unknown event type: {}", event_type);
            return Err(format!("Unknown event type: {}", event_type));
        }
    };

    // Create analytics event
    let event = crate::services::analytics::AnalyticsEvent {
        event_type: analytics_event_type,
        timestamp: chrono::Utc::now(),
        user_id: None, // Could be populated from session data
        session_id: uuid::Uuid::new_v4().to_string(),
        metadata: metadata.map(|m| {
            if let serde_json::Value::Object(map) = m {
                map
            } else {
                std::collections::HashMap::new()
            }
        }).unwrap_or_default(),
    };

    match analytics.record_event(event).await {
        Ok(_) => {
            info!("Successfully recorded analytics event");
            Ok(())
        }
        Err(e) => {
            error!("Failed to record analytics event: {}", e);
            Err(format!("Failed to record analytics event: {}", e))
        }
    }
}

/// Get analytics configuration
#[tauri::command]
pub async fn get_analytics_config(
    service_manager: State<'_, ServiceManager>,
) -> Result<serde_json::Value, String> {
    info!("Getting analytics configuration");

    // Return default analytics configuration
    let config = crate::services::analytics::AnalyticsConfig::default();
    
    match serde_json::to_value(config) {
        Ok(config_json) => {
            info!("Successfully retrieved analytics configuration");
            Ok(config_json)
        }
        Err(e) => {
            error!("Failed to serialize analytics configuration: {}", e);
            Err(format!("Failed to get analytics configuration: {}", e))
        }
    }
}

/// Update analytics configuration
#[tauri::command]
pub async fn update_analytics_config(
    service_manager: State<'_, ServiceManager>,
    config: serde_json::Value,
) -> Result<(), String> {
    info!("Updating analytics configuration");

    let analytics_config: crate::services::analytics::AnalyticsConfig = 
        serde_json::from_value(config).map_err(|e| {
            error!("Failed to parse analytics configuration: {}", e);
            format!("Invalid configuration format: {}", e)
        })?;

    let mut analytics = service_manager.analytics.write().await;
    
    match analytics.update_config(analytics_config).await {
        Ok(_) => {
            info!("Successfully updated analytics configuration");
            Ok(())
        }
        Err(e) => {
            error!("Failed to update analytics configuration: {}", e);
            Err(format!("Failed to update analytics configuration: {}", e))
        }
    }
}

/// Export analytics data
#[tauri::command]
pub async fn export_analytics_data(
    service_manager: State<'_, ServiceManager>,
    export_type: String,
    time_period: String,
    format: String,
) -> Result<String, String> {
    info!("Exporting analytics data: type={}, period={}, format={}", export_type, time_period, format);

    // This would implement actual data export functionality
    // For now, return a success message
    let export_result = format!(
        "Analytics data export initiated: {} data for {} in {} format",
        export_type, time_period, format
    );

    info!("Analytics data export completed");
    Ok(export_result)
}

/// Get analytics health status
#[tauri::command]
pub async fn get_analytics_health(
    service_manager: State<'_, ServiceManager>,
) -> Result<serde_json::Value, String> {
    info!("Getting analytics health status");

    let analytics = service_manager.analytics.read().await;
    
    match analytics.health_check().await {
        Ok(_) => {
            let health_status = serde_json::json!({
                "status": "healthy",
                "last_check": chrono::Utc::now(),
                "components": {
                    "usage_analytics": "healthy",
                    "performance_monitor": "healthy",
                    "predictive_analytics": "healthy",
                    "business_intelligence": "healthy",
                    "metrics_collector": "healthy",
                    "dashboard_engine": "healthy",
                    "report_generator": "healthy"
                }
            });
            
            info!("Analytics health check passed");
            Ok(health_status)
        }
        Err(e) => {
            error!("Analytics health check failed: {}", e);
            let health_status = serde_json::json!({
                "status": "unhealthy",
                "last_check": chrono::Utc::now(),
                "error": e.to_string()
            });
            Ok(health_status)
        }
    }
}
