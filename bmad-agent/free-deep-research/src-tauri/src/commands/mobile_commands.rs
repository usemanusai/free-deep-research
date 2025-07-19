use tauri::State;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use tracing::{info, debug, error};

use crate::error::AppResult;
use crate::services::mobile_platform::{
    MobilePlatformService,
    DeviceRegistration,
    MobilePlatform,
    DeviceCapabilities,
    MobileResearchRequest,
    MobileSession
};

/// Register a mobile device
#[tauri::command]
pub async fn register_mobile_device(
    device_info: DeviceRegistrationRequest,
    mobile_service: State<'_, MobilePlatformService>
) -> AppResult<DeviceRegistrationResponse> {
    info!("Registering mobile device: {}", device_info.device_name);

    let platform = match device_info.platform.as_str() {
        "ios" => MobilePlatform::iOS,
        "android" => MobilePlatform::Android,
        "web" => MobilePlatform::Web,
        _ => return Err(crate::error::ResearchError::invalid_request(
            format!("Unsupported platform: {}", device_info.platform)
        ).into()),
    };

    let registration = DeviceRegistration {
        device_name: device_info.device_name,
        platform,
        os_version: device_info.os_version,
        app_version: device_info.app_version,
        push_token: device_info.push_token,
        device_capabilities: DeviceCapabilities {
            offline_storage_mb: device_info.offline_storage_mb,
            supports_push_notifications: device_info.supports_push_notifications,
            supports_background_sync: device_info.supports_background_sync,
            max_concurrent_requests: device_info.max_concurrent_requests,
        },
    };

    let device_token = mobile_service.register_device(registration).await?;

    Ok(DeviceRegistrationResponse {
        device_id: device_token.device_id.to_string(),
        token: device_token.token,
        expires_at: device_token.expires_at.to_rfc3339(),
        registration_successful: true,
    })
}

/// Start a mobile research session
#[tauri::command]
pub async fn start_mobile_session(
    device_id: String,
    user_id: String,
    mobile_service: State<'_, MobilePlatformService>
) -> AppResult<MobileSessionResponse> {
    info!("Starting mobile session for device: {}", device_id);

    let device_uuid = Uuid::parse_str(&device_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid device ID".to_string()))?;
    
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid user ID".to_string()))?;

    let session = mobile_service.start_mobile_session(device_uuid, user_uuid).await?;

    Ok(MobileSessionResponse {
        session_id: session.session_id.to_string(),
        device_id: session.device_id.to_string(),
        user_id: session.user_id.to_string(),
        started_at: session.started_at.to_rfc3339(),
        is_offline: session.is_offline,
        sync_status: format!("{:?}", session.sync_status),
    })
}

/// Execute mobile-optimized research
#[tauri::command]
pub async fn execute_mobile_research(
    request: MobileResearchRequestData,
    mobile_service: State<'_, MobilePlatformService>
) -> AppResult<MobileResearchResponseData> {
    info!("Executing mobile research: {}", request.query);

    let session_uuid = Uuid::parse_str(&request.session_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid session ID".to_string()))?;

    let mobile_request = MobileResearchRequest {
        request_id: Uuid::new_v4(),
        session_id: session_uuid,
        query: request.query,
        methodology: request.methodology,
        mobile_optimized: request.mobile_optimized,
        offline_fallback: request.offline_fallback,
    };

    let response = mobile_service.execute_mobile_research(mobile_request).await?;

    Ok(MobileResearchResponseData {
        request_id: response.request_id.to_string(),
        session_id: response.session_id.to_string(),
        status: format!("{:?}", response.status),
        results: response.results,
        cached_results: response.cached_results,
        estimated_completion: response.estimated_completion.map(|dt| dt.to_rfc3339()),
        offline_mode: response.offline_mode,
        sync_required: response.sync_required,
    })
}

/// Sync offline data
#[tauri::command]
pub async fn sync_offline_data(
    device_id: String,
    mobile_service: State<'_, MobilePlatformService>
) -> AppResult<SyncResultResponse> {
    info!("Syncing offline data for device: {}", device_id);

    let device_uuid = Uuid::parse_str(&device_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid device ID".to_string()))?;

    let sync_result = mobile_service.sync_offline_data(device_uuid).await?;

    Ok(SyncResultResponse {
        synced_items: sync_result.synced_items,
        failed_items: sync_result.failed_items,
        sync_duration_ms: sync_result.sync_duration_ms,
        last_sync: sync_result.last_sync.to_rfc3339(),
        sync_successful: sync_result.failed_items == 0,
    })
}

/// Get mobile dashboard data
#[tauri::command]
pub async fn get_mobile_dashboard(
    device_id: String,
    user_id: String,
    mobile_service: State<'_, MobilePlatformService>
) -> AppResult<MobileDashboardResponse> {
    debug!("Getting mobile dashboard for device: {}", device_id);

    let device_uuid = Uuid::parse_str(&device_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid device ID".to_string()))?;
    
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid user ID".to_string()))?;

    let dashboard = mobile_service.get_mobile_dashboard(device_uuid, user_uuid).await?;

    let recent_sessions: Vec<MobileSessionSummaryResponse> = dashboard.recent_sessions.into_iter().map(|session| {
        MobileSessionSummaryResponse {
            session_id: session.session_id.to_string(),
            started_at: session.started_at.to_rfc3339(),
            duration_minutes: session.duration_minutes,
            research_count: session.research_count,
            success_rate: session.success_rate,
        }
    }).collect();

    let quick_actions: Vec<QuickActionResponse> = dashboard.quick_actions.into_iter().map(|action| {
        QuickActionResponse {
            id: action.id,
            title: action.title,
            description: action.description,
            icon: action.icon,
            action_type: format!("{:?}", action.action_type),
        }
    }).collect();

    let recommendations: Vec<MobileRecommendationResponse> = dashboard.recommendations.into_iter().map(|rec| {
        MobileRecommendationResponse {
            id: rec.id.to_string(),
            title: rec.title,
            description: rec.description,
            priority: format!("{:?}", rec.priority),
            action_required: rec.action_required,
        }
    }).collect();

    Ok(MobileDashboardResponse {
        device_id: dashboard.device_id.to_string(),
        user_id: dashboard.user_id.to_string(),
        recent_sessions,
        sync_status: format!("{:?}", dashboard.sync_status),
        cached_data_summary: CachedDataSummaryResponse {
            total_cached_items: dashboard.cached_data_summary.total_cached_items,
            cache_size_mb: dashboard.cached_data_summary.cache_size_mb,
            last_cache_update: dashboard.cached_data_summary.last_cache_update.to_rfc3339(),
        },
        performance_metrics: DevicePerformanceMetricsResponse {
            average_response_time_ms: dashboard.performance_metrics.average_response_time_ms,
            success_rate: dashboard.performance_metrics.success_rate,
            offline_usage_percentage: dashboard.performance_metrics.offline_usage_percentage,
            battery_efficiency_score: dashboard.performance_metrics.battery_efficiency_score,
        },
        quick_actions,
        recommendations,
        generated_at: dashboard.generated_at.to_rfc3339(),
    })
}

/// Get mobile platform metrics
#[tauri::command]
pub async fn get_mobile_platform_metrics(
    mobile_service: State<'_, MobilePlatformService>
) -> AppResult<MobilePlatformMetricsResponse> {
    debug!("Getting mobile platform metrics");

    let metrics = mobile_service.get_platform_metrics().await?;

    Ok(MobilePlatformMetricsResponse {
        total_registered_devices: metrics.total_registered_devices,
        active_sessions: metrics.active_sessions,
        total_mobile_research_sessions: metrics.total_mobile_research_sessions,
        offline_sync_operations: metrics.offline_sync_operations,
        average_mobile_response_time: metrics.average_mobile_response_time,
        mobile_user_satisfaction: metrics.mobile_user_satisfaction,
    })
}

// Request/Response structures

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceRegistrationRequest {
    pub device_name: String,
    pub platform: String,
    pub os_version: String,
    pub app_version: String,
    pub push_token: Option<String>,
    pub offline_storage_mb: u64,
    pub supports_push_notifications: bool,
    pub supports_background_sync: bool,
    pub max_concurrent_requests: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceRegistrationResponse {
    pub device_id: String,
    pub token: String,
    pub expires_at: String,
    pub registration_successful: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MobileSessionResponse {
    pub session_id: String,
    pub device_id: String,
    pub user_id: String,
    pub started_at: String,
    pub is_offline: bool,
    pub sync_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MobileResearchRequestData {
    pub session_id: String,
    pub query: String,
    pub methodology: String,
    pub mobile_optimized: bool,
    pub offline_fallback: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MobileResearchResponseData {
    pub request_id: String,
    pub session_id: String,
    pub status: String,
    pub results: Option<serde_json::Value>,
    pub cached_results: Option<Vec<serde_json::Value>>,
    pub estimated_completion: Option<String>,
    pub offline_mode: bool,
    pub sync_required: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncResultResponse {
    pub synced_items: u64,
    pub failed_items: u64,
    pub sync_duration_ms: u64,
    pub last_sync: String,
    pub sync_successful: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MobileDashboardResponse {
    pub device_id: String,
    pub user_id: String,
    pub recent_sessions: Vec<MobileSessionSummaryResponse>,
    pub sync_status: String,
    pub cached_data_summary: CachedDataSummaryResponse,
    pub performance_metrics: DevicePerformanceMetricsResponse,
    pub quick_actions: Vec<QuickActionResponse>,
    pub recommendations: Vec<MobileRecommendationResponse>,
    pub generated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MobileSessionSummaryResponse {
    pub session_id: String,
    pub started_at: String,
    pub duration_minutes: u32,
    pub research_count: u32,
    pub success_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedDataSummaryResponse {
    pub total_cached_items: u64,
    pub cache_size_mb: f64,
    pub last_cache_update: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevicePerformanceMetricsResponse {
    pub average_response_time_ms: f64,
    pub success_rate: f64,
    pub offline_usage_percentage: f64,
    pub battery_efficiency_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuickActionResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub action_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MobileRecommendationResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub priority: String,
    pub action_required: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MobilePlatformMetricsResponse {
    pub total_registered_devices: u64,
    pub active_sessions: u64,
    pub total_mobile_research_sessions: u64,
    pub offline_sync_operations: u64,
    pub average_mobile_response_time: f64,
    pub mobile_user_satisfaction: f64,
}
