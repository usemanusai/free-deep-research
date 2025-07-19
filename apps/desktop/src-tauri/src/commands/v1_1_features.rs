use tauri::State;
use uuid::Uuid;
use tracing::{info, error};
use std::collections::HashMap;

use crate::error::AppResult;
use crate::services::ServiceManager;
use crate::services::api_manager::{ModelConfiguration, ModelRecommendation, ModelPerformanceMetrics};
use crate::services::output_processor::export::enhanced_export::{
    EnhancedExportRequest, EnhancedExportResult, ExportFormat, EnhancedExportOptions, ExportDestination
};
use crate::services::collaboration::{
    CollaborationSession, SessionType, CollaborationStats, CollaborationEvent
};
use crate::services::mobile_api::{
    MobileDashboard, MobileWorkflowSummary, MobileConnection, MobileApiRequest, MobileApiResponse
};

/// V1.1.0 Enhanced AI Models Commands

/// Get latest available AI models
#[tauri::command]
pub async fn get_latest_ai_models(
    service_manager: State<'_, ServiceManager>,
) -> AppResult<Vec<String>> {
    info!("Getting latest AI models");
    
    let api_manager = service_manager.api_manager.read().await;
    // This would integrate with the model manager
    // For now, returning a list of latest models
    Ok(vec![
        "anthropic/claude-3.5-sonnet".to_string(),
        "openai/gpt-4-turbo".to_string(),
        "openai/gpt-4o".to_string(),
        "google/gemini-1.5-pro".to_string(),
        "meta-llama/llama-3.1-405b".to_string(),
        "mistralai/mixtral-8x22b".to_string(),
        "alibaba/qwen2.5-72b".to_string(),
    ])
}

/// Get model configurations
#[tauri::command]
pub async fn get_model_configurations(
    service_manager: State<'_, ServiceManager>,
) -> AppResult<Vec<ModelConfiguration>> {
    info!("Getting model configurations");
    
    let api_manager = service_manager.api_manager.read().await;
    // This would integrate with the model manager
    // For now, returning empty list
    Ok(Vec::new())
}

/// Get model recommendations for use case
#[tauri::command]
pub async fn get_model_recommendations(
    use_case: String,
    budget_limit: Option<f64>,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<Vec<ModelRecommendation>> {
    info!("Getting model recommendations for use case: {}", use_case);
    
    let api_manager = service_manager.api_manager.read().await;
    // This would integrate with the model manager
    // For now, returning empty list
    Ok(Vec::new())
}

/// Update model configuration
#[tauri::command]
pub async fn update_model_configuration(
    config: ModelConfiguration,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<()> {
    info!("Updating model configuration: {}", config.model_id);
    
    let api_manager = service_manager.api_manager.write().await;
    // This would integrate with the model manager
    Ok(())
}

/// V1.1.0 Advanced Analytics Commands

/// Get comprehensive analytics dashboard
#[tauri::command]
pub async fn get_analytics_dashboard(
    service_manager: State<'_, ServiceManager>,
) -> AppResult<serde_json::Value> {
    info!("Getting analytics dashboard");
    
    let analytics = service_manager.analytics.read().await;
    let dashboard_data = analytics.get_dashboard_data().await?;
    Ok(serde_json::to_value(dashboard_data)?)
}

/// Get usage analytics for time period
#[tauri::command]
pub async fn get_usage_analytics(
    period: String,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<serde_json::Value> {
    info!("Getting usage analytics for period: {}", period);
    
    let analytics = service_manager.analytics.read().await;
    // Convert string period to TimePeriod enum
    let time_period = match period.as_str() {
        "last_hour" => crate::services::analytics::TimePeriod::LastHour,
        "last_24_hours" => crate::services::analytics::TimePeriod::Last24Hours,
        "last_week" => crate::services::analytics::TimePeriod::LastWeek,
        "last_month" => crate::services::analytics::TimePeriod::LastMonth,
        "last_quarter" => crate::services::analytics::TimePeriod::LastQuarter,
        "last_year" => crate::services::analytics::TimePeriod::LastYear,
        _ => crate::services::analytics::TimePeriod::Last24Hours,
    };
    
    let usage_data = analytics.get_usage_analytics(time_period).await?;
    Ok(serde_json::to_value(usage_data)?)
}

/// Generate business intelligence report
#[tauri::command]
pub async fn generate_business_report(
    report_type: String,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<serde_json::Value> {
    info!("Generating business report: {}", report_type);
    
    let analytics = service_manager.analytics.read().await;
    let business_report_type = match report_type.as_str() {
        "executive_summary" => crate::services::analytics::BusinessReportType::ExecutiveSummary,
        "usage_report" => crate::services::analytics::BusinessReportType::UsageReport,
        "performance_report" => crate::services::analytics::BusinessReportType::PerformanceReport,
        "cost_analysis" => crate::services::analytics::BusinessReportType::CostAnalysis,
        "trend_analysis" => crate::services::analytics::BusinessReportType::TrendAnalysis,
        _ => crate::services::analytics::BusinessReportType::ExecutiveSummary,
    };
    
    let report = analytics.generate_business_report(business_report_type).await?;
    Ok(serde_json::to_value(report)?)
}

/// V1.1.0 Enhanced Export Commands

/// Export workflows to enhanced formats
#[tauri::command]
pub async fn export_workflows_enhanced(
    workflow_ids: Vec<String>,
    format: String,
    destination_path: String,
    filename: String,
    options: serde_json::Value,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<EnhancedExportResult> {
    info!("Exporting {} workflows to format: {}", workflow_ids.len(), format);
    
    let output_processor = service_manager.output_processor.read().await;
    
    // Convert format string to ExportFormat enum
    let export_format = match format.as_str() {
        "pdf" => ExportFormat::PDF,
        "docx" => ExportFormat::DOCX,
        "pptx" => ExportFormat::PPTX,
        "html" => ExportFormat::HTML,
        "markdown" => ExportFormat::Markdown,
        "json" => ExportFormat::JSON,
        "csv" => ExportFormat::CSV,
        "excel" => ExportFormat::Excel,
        _ => ExportFormat::PDF,
    };
    
    // Create export request
    let export_request = EnhancedExportRequest {
        id: Uuid::new_v4(),
        workflows: workflow_ids.iter().filter_map(|id| Uuid::parse_str(id).ok()).collect(),
        format: export_format,
        template_id: None,
        options: serde_json::from_value(options).unwrap_or_default(),
        destination: ExportDestination {
            destination_type: crate::services::output_processor::export::enhanced_export::ExportDestinationType::LocalFile,
            path: destination_path,
            filename,
        },
    };
    
    // This would integrate with the enhanced export service
    // For now, returning a mock result
    Ok(EnhancedExportResult {
        job_id: export_request.id,
        success: true,
        output_path: Some(format!("{}/{}", export_request.destination.path, export_request.destination.filename)),
        file_size_bytes: Some(1024),
        processing_time_ms: 1000,
        error_message: None,
        metadata: HashMap::new(),
    })
}

/// Get export job status
#[tauri::command]
pub async fn get_export_job_status(
    job_id: String,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<serde_json::Value> {
    info!("Getting export job status: {}", job_id);
    
    let output_processor = service_manager.output_processor.read().await;
    
    if let Ok(uuid) = Uuid::parse_str(&job_id) {
        // This would integrate with the enhanced export service
        // For now, returning mock status
        Ok(serde_json::json!({
            "job_id": job_id,
            "status": "completed",
            "progress": 100.0,
            "created_at": chrono::Utc::now(),
            "completed_at": chrono::Utc::now()
        }))
    } else {
        Ok(serde_json::json!({
            "error": "Invalid job ID"
        }))
    }
}

/// V1.1.0 Collaboration Commands

/// Start collaboration session
#[tauri::command]
pub async fn start_collaboration_session(
    user_id: String,
    workflow_id: String,
    session_type: String,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<CollaborationSession> {
    info!("Starting collaboration session for user {} on workflow {}", user_id, workflow_id);
    
    let collaboration = service_manager.collaboration.read().await;
    
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid user ID".to_string()))?;
    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid workflow ID".to_string()))?;
    
    let session_type_enum = match session_type.as_str() {
        "read_only" => SessionType::ReadOnly,
        "edit" => SessionType::Edit,
        "comment" => SessionType::Comment,
        "review" => SessionType::Review,
        "admin" => SessionType::Admin,
        _ => SessionType::ReadOnly,
    };
    
    collaboration.start_session(user_uuid, workflow_uuid, session_type_enum).await
}

/// End collaboration session
#[tauri::command]
pub async fn end_collaboration_session(
    session_id: String,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<()> {
    info!("Ending collaboration session: {}", session_id);
    
    let collaboration = service_manager.collaboration.read().await;
    
    let session_uuid = Uuid::parse_str(&session_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid session ID".to_string()))?;
    
    collaboration.end_session(session_uuid).await
}

/// Get collaboration statistics
#[tauri::command]
pub async fn get_collaboration_stats(
    service_manager: State<'_, ServiceManager>,
) -> AppResult<CollaborationStats> {
    info!("Getting collaboration statistics");
    
    let collaboration = service_manager.collaboration.read().await;
    collaboration.get_collaboration_stats().await
}

/// V1.1.0 Mobile API Commands

/// Get mobile dashboard
#[tauri::command]
pub async fn get_mobile_dashboard(
    user_id: String,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<MobileDashboard> {
    info!("Getting mobile dashboard for user: {}", user_id);
    
    let mobile_api = service_manager.mobile_api.read().await;
    
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid user ID".to_string()))?;
    
    mobile_api.get_dashboard(user_uuid).await
}

/// Start workflow from mobile
#[tauri::command]
pub async fn start_workflow_mobile(
    user_id: String,
    workflow_name: String,
    query: String,
    methodology: Option<String>,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<String> {
    info!("Starting workflow from mobile: {}", workflow_name);
    
    let mobile_api = service_manager.mobile_api.read().await;
    
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid user ID".to_string()))?;
    
    let workflow_id = mobile_api.start_workflow_mobile(user_uuid, workflow_name, query, methodology).await?;
    Ok(workflow_id.to_string())
}

/// Stop workflow from mobile
#[tauri::command]
pub async fn stop_workflow_mobile(
    user_id: String,
    workflow_id: String,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<()> {
    info!("Stopping workflow from mobile: {}", workflow_id);
    
    let mobile_api = service_manager.mobile_api.read().await;
    
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid user ID".to_string()))?;
    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid workflow ID".to_string()))?;
    
    mobile_api.stop_workflow_mobile(user_uuid, workflow_uuid).await
}

/// Get user workflows for mobile
#[tauri::command]
pub async fn get_user_workflows_mobile(
    user_id: String,
    service_manager: State<'_, ServiceManager>,
) -> AppResult<Vec<MobileWorkflowSummary>> {
    info!("Getting workflows for mobile user: {}", user_id);
    
    let mobile_api = service_manager.mobile_api.read().await;
    
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid user ID".to_string()))?;
    
    mobile_api.get_user_workflows_mobile(user_uuid).await
}

/// Get active mobile connections
#[tauri::command]
pub async fn get_active_mobile_connections(
    service_manager: State<'_, ServiceManager>,
) -> AppResult<Vec<MobileConnection>> {
    info!("Getting active mobile connections");
    
    let mobile_api = service_manager.mobile_api.read().await;
    mobile_api.get_active_connections().await
}
