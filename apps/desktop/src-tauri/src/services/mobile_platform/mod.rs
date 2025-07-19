use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, warn, error};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, ResearchError};
use crate::services::Service;

pub mod mobile_api;
pub mod offline_sync;
pub mod push_notifications;
pub mod mobile_analytics;
pub mod device_management;

use mobile_api::{MobileApiService, MobileApiConfig};
use offline_sync::{OfflineSyncService, SyncConfig};
use push_notifications::{PushNotificationService, NotificationConfig};
use mobile_analytics::{MobileAnalyticsService, MobileAnalyticsConfig};
use device_management::{DeviceManagementService, DeviceConfig};

/// Mobile platform service for cross-platform mobile support
#[derive(Clone)]
pub struct MobilePlatformService {
    mobile_api: Arc<RwLock<MobileApiService>>,
    offline_sync: Arc<RwLock<OfflineSyncService>>,
    push_notifications: Arc<RwLock<PushNotificationService>>,
    mobile_analytics: Arc<RwLock<MobileAnalyticsService>>,
    device_management: Arc<RwLock<DeviceManagementService>>,
    active_sessions: Arc<RwLock<HashMap<Uuid, MobileSession>>>,
    config: MobilePlatformConfig,
}

impl MobilePlatformService {
    pub async fn new(config: MobilePlatformConfig) -> AppResult<Self> {
        info!("Initializing Mobile Platform Service");

        let mobile_api = MobileApiService::new(config.mobile_api_config.clone()).await?;
        let offline_sync = OfflineSyncService::new(config.sync_config.clone()).await?;
        let push_notifications = PushNotificationService::new(config.notification_config.clone()).await?;
        let mobile_analytics = MobileAnalyticsService::new(config.mobile_analytics_config.clone()).await?;
        let device_management = DeviceManagementService::new(config.device_config.clone()).await?;

        Ok(Self {
            mobile_api: Arc::new(RwLock::new(mobile_api)),
            offline_sync: Arc::new(RwLock::new(offline_sync)),
            push_notifications: Arc::new(RwLock::new(push_notifications)),
            mobile_analytics: Arc::new(RwLock::new(mobile_analytics)),
            device_management: Arc::new(RwLock::new(device_management)),
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            config,
        })
    }

    /// Register a new mobile device
    pub async fn register_device(&self, device_info: DeviceRegistration) -> AppResult<DeviceToken> {
        info!("Registering mobile device: {} ({})", device_info.device_name, device_info.platform);

        // Register with device management
        let device_manager = self.device_management.write().await;
        let device_token = device_manager.register_device(device_info.clone()).await?;

        // Initialize offline sync for the device
        let mut sync_service = self.offline_sync.write().await;
        sync_service.initialize_device_sync(device_token.device_id).await?;

        // Set up push notifications
        if let Some(push_token) = &device_info.push_token {
            let mut notification_service = self.push_notifications.write().await;
            notification_service.register_device(device_token.device_id, push_token.clone()).await?;
        }

        // Track device registration
        let mut analytics = self.mobile_analytics.write().await;
        analytics.track_device_registration(&device_info).await?;

        info!("Device registered successfully: {}", device_token.device_id);
        Ok(device_token)
    }

    /// Start a mobile research session
    pub async fn start_mobile_session(&self, device_id: Uuid, user_id: Uuid) -> AppResult<MobileSession> {
        info!("Starting mobile session for device: {} user: {}", device_id, user_id);

        let session = MobileSession {
            session_id: Uuid::new_v4(),
            device_id,
            user_id,
            started_at: Utc::now(),
            last_activity: Utc::now(),
            is_offline: false,
            sync_status: SyncStatus::Synchronized,
            cached_data: HashMap::new(),
        };

        // Store active session
        {
            let mut sessions = self.active_sessions.write().await;
            sessions.insert(session.session_id, session.clone());
        }

        // Initialize mobile API session
        let mut mobile_api = self.mobile_api.write().await;
        mobile_api.initialize_session(session.session_id, device_id, user_id).await?;

        // Track session start
        let mut analytics = self.mobile_analytics.write().await;
        analytics.track_session_start(&session).await?;

        info!("Mobile session started: {}", session.session_id);
        Ok(session)
    }

    /// Execute mobile-optimized research
    pub async fn execute_mobile_research(&self, request: MobileResearchRequest) -> AppResult<MobileResearchResponse> {
        info!("Executing mobile research: {}", request.session_id);

        // Get session
        let session = {
            let sessions = self.active_sessions.read().await;
            sessions.get(&request.session_id)
                .ok_or_else(|| ResearchError::invalid_request("Session not found".to_string()))?
                .clone()
        };

        // Check if device is offline
        if session.is_offline {
            return self.handle_offline_research(request).await;
        }

        // Execute research through mobile API
        let mobile_api = self.mobile_api.read().await;
        let response = mobile_api.execute_research(request.clone()).await?;

        // Cache results for offline access
        let mut sync_service = self.offline_sync.write().await;
        sync_service.cache_research_results(session.device_id, &response).await?;

        // Track research execution
        let mut analytics = self.mobile_analytics.write().await;
        analytics.track_research_execution(&request, &response).await?;

        // Update session activity
        self.update_session_activity(request.session_id).await?;

        info!("Mobile research completed: {}", request.session_id);
        Ok(response)
    }

    /// Handle offline research request
    async fn handle_offline_research(&self, request: MobileResearchRequest) -> AppResult<MobileResearchResponse> {
        info!("Handling offline research request");

        let sync_service = self.offline_sync.read().await;
        
        // Check if we have cached results for similar queries
        if let Some(cached_response) = sync_service.get_cached_results(&request.query).await? {
            info!("Returning cached results for offline query");
            return Ok(cached_response);
        }

        // Queue request for when online
        sync_service.queue_offline_request(request.clone()).await?;

        // Return offline response
        Ok(MobileResearchResponse {
            request_id: request.request_id,
            session_id: request.session_id,
            status: ResearchStatus::QueuedOffline,
            results: None,
            cached_results: sync_service.get_related_cached_results(&request.query).await?,
            estimated_completion: None,
            offline_mode: true,
            sync_required: true,
        })
    }

    /// Sync offline data when device comes online
    pub async fn sync_offline_data(&self, device_id: Uuid) -> AppResult<SyncResult> {
        info!("Syncing offline data for device: {}", device_id);

        let mut sync_service = self.offline_sync.write().await;
        let sync_result = sync_service.sync_device_data(device_id).await?;

        // Send push notification about sync completion
        if sync_result.synced_items > 0 {
            let notification_service = self.push_notifications.read().await;
            notification_service.send_sync_completion_notification(device_id, &sync_result).await?;
        }

        // Track sync operation
        let mut analytics = self.mobile_analytics.write().await;
        analytics.track_sync_operation(device_id, &sync_result).await?;

        info!("Offline data sync completed for device: {}", device_id);
        Ok(sync_result)
    }

    /// Get mobile dashboard data
    pub async fn get_mobile_dashboard(&self, device_id: Uuid, user_id: Uuid) -> AppResult<MobileDashboard> {
        debug!("Getting mobile dashboard for device: {}", device_id);

        // Get recent research sessions
        let analytics = self.mobile_analytics.read().await;
        let recent_sessions = analytics.get_recent_sessions(user_id, 10).await?;

        // Get sync status
        let sync_service = self.offline_sync.read().await;
        let sync_status = sync_service.get_device_sync_status(device_id).await?;

        // Get cached data summary
        let cached_data_summary = sync_service.get_cached_data_summary(device_id).await?;

        // Get device performance metrics
        let device_manager = self.device_management.read().await;
        let performance_metrics = device_manager.get_device_performance(device_id).await?;

        Ok(MobileDashboard {
            device_id,
            user_id,
            recent_sessions,
            sync_status,
            cached_data_summary,
            performance_metrics,
            quick_actions: self.generate_quick_actions(&recent_sessions).await?,
            recommendations: self.generate_mobile_recommendations(user_id).await?,
            generated_at: Utc::now(),
        })
    }

    /// Generate quick actions for mobile dashboard
    async fn generate_quick_actions(&self, recent_sessions: &[MobileSessionSummary]) -> AppResult<Vec<QuickAction>> {
        let mut actions = Vec::new();

        // Add common quick actions
        actions.push(QuickAction {
            id: "new_research".to_string(),
            title: "New Research".to_string(),
            description: "Start a new research session".to_string(),
            icon: "search".to_string(),
            action_type: ActionType::NewResearch,
        });

        actions.push(QuickAction {
            id: "recent_results".to_string(),
            title: "Recent Results".to_string(),
            description: "View your latest research results".to_string(),
            icon: "history".to_string(),
            action_type: ActionType::ViewResults,
        });

        // Add contextual actions based on recent activity
        if recent_sessions.len() > 3 {
            actions.push(QuickAction {
                id: "export_results".to_string(),
                title: "Export Results".to_string(),
                description: "Export recent research to PDF".to_string(),
                icon: "download".to_string(),
                action_type: ActionType::ExportResults,
            });
        }

        Ok(actions)
    }

    /// Generate mobile-specific recommendations
    async fn generate_mobile_recommendations(&self, user_id: Uuid) -> AppResult<Vec<MobileRecommendation>> {
        let mut recommendations = Vec::new();

        // Mobile-specific recommendations
        recommendations.push(MobileRecommendation {
            id: Uuid::new_v4(),
            title: "Enable Offline Mode".to_string(),
            description: "Download recent results for offline access".to_string(),
            priority: RecommendationPriority::Medium,
            action_required: "Enable offline sync in settings".to_string(),
        });

        recommendations.push(MobileRecommendation {
            id: Uuid::new_v4(),
            title: "Optimize for Mobile".to_string(),
            description: "Use shorter queries for better mobile performance".to_string(),
            priority: RecommendationPriority::Low,
            action_required: "Keep queries under 100 characters".to_string(),
        });

        Ok(recommendations)
    }

    /// Update session activity
    async fn update_session_activity(&self, session_id: Uuid) -> AppResult<()> {
        let mut sessions = self.active_sessions.write().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            session.last_activity = Utc::now();
        }
        Ok(())
    }

    /// Get mobile platform metrics
    pub async fn get_platform_metrics(&self) -> AppResult<MobilePlatformMetrics> {
        let analytics = self.mobile_analytics.read().await;
        let device_manager = self.device_management.read().await;
        let sync_service = self.offline_sync.read().await;

        Ok(MobilePlatformMetrics {
            total_registered_devices: device_manager.get_device_count().await?,
            active_sessions: self.active_sessions.read().await.len() as u64,
            total_mobile_research_sessions: analytics.get_total_sessions().await?,
            offline_sync_operations: sync_service.get_sync_operations_count().await?,
            average_mobile_response_time: analytics.get_average_response_time().await?,
            mobile_user_satisfaction: analytics.get_user_satisfaction_score().await?,
        })
    }
}

#[async_trait::async_trait]
impl Service for MobilePlatformService {
    async fn start(&self) -> AppResult<()> {
        info!("Starting Mobile Platform Service");

        // Start all sub-services
        let mobile_api = self.mobile_api.read().await;
        mobile_api.start().await?;

        let sync_service = self.offline_sync.read().await;
        sync_service.start().await?;

        let notification_service = self.push_notifications.read().await;
        notification_service.start().await?;

        let analytics = self.mobile_analytics.read().await;
        analytics.start().await?;

        let device_manager = self.device_management.read().await;
        device_manager.start().await?;

        info!("Mobile Platform Service started successfully");
        Ok(())
    }

    async fn stop(&self) -> AppResult<()> {
        info!("Stopping Mobile Platform Service");

        // Stop all sub-services
        let mobile_api = self.mobile_api.read().await;
        mobile_api.stop().await?;

        let sync_service = self.offline_sync.read().await;
        sync_service.stop().await?;

        let notification_service = self.push_notifications.read().await;
        notification_service.stop().await?;

        let analytics = self.mobile_analytics.read().await;
        analytics.stop().await?;

        let device_manager = self.device_management.read().await;
        device_manager.stop().await?;

        info!("Mobile Platform Service stopped successfully");
        Ok(())
    }

    async fn health_check(&self) -> AppResult<()> {
        debug!("Performing Mobile Platform Service health check");

        // Check all sub-services
        let mobile_api = self.mobile_api.read().await;
        mobile_api.health_check().await?;

        let sync_service = self.offline_sync.read().await;
        sync_service.health_check().await?;

        let notification_service = self.push_notifications.read().await;
        notification_service.health_check().await?;

        let analytics = self.mobile_analytics.read().await;
        analytics.health_check().await?;

        let device_manager = self.device_management.read().await;
        device_manager.health_check().await?;

        debug!("Mobile Platform Service health check completed successfully");
        Ok(())
    }

    async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down Mobile Platform Service");

        // Graceful shutdown of all sub-services
        let mobile_api = self.mobile_api.read().await;
        mobile_api.shutdown().await?;

        let sync_service = self.offline_sync.read().await;
        sync_service.shutdown().await?;

        let notification_service = self.push_notifications.read().await;
        notification_service.shutdown().await?;

        let analytics = self.mobile_analytics.read().await;
        analytics.shutdown().await?;

        let device_manager = self.device_management.read().await;
        device_manager.shutdown().await?;

        info!("Mobile Platform Service shutdown completed successfully");
        Ok(())
    }
}

// Supporting data structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobilePlatformConfig {
    pub mobile_api_config: MobileApiConfig,
    pub sync_config: SyncConfig,
    pub notification_config: NotificationConfig,
    pub mobile_analytics_config: MobileAnalyticsConfig,
    pub device_config: DeviceConfig,
}

impl Default for MobilePlatformConfig {
    fn default() -> Self {
        Self {
            mobile_api_config: MobileApiConfig::default(),
            sync_config: SyncConfig::default(),
            notification_config: NotificationConfig::default(),
            mobile_analytics_config: MobileAnalyticsConfig::default(),
            device_config: DeviceConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRegistration {
    pub device_name: String,
    pub platform: MobilePlatform,
    pub os_version: String,
    pub app_version: String,
    pub push_token: Option<String>,
    pub device_capabilities: DeviceCapabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MobilePlatform {
    iOS,
    Android,
    Web,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    pub offline_storage_mb: u64,
    pub supports_push_notifications: bool,
    pub supports_background_sync: bool,
    pub max_concurrent_requests: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceToken {
    pub device_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileSession {
    pub session_id: Uuid,
    pub device_id: Uuid,
    pub user_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub is_offline: bool,
    pub sync_status: SyncStatus,
    pub cached_data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncStatus {
    Synchronized,
    PendingSync,
    Syncing,
    SyncError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileResearchRequest {
    pub request_id: Uuid,
    pub session_id: Uuid,
    pub query: String,
    pub methodology: String,
    pub mobile_optimized: bool,
    pub offline_fallback: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileResearchResponse {
    pub request_id: Uuid,
    pub session_id: Uuid,
    pub status: ResearchStatus,
    pub results: Option<serde_json::Value>,
    pub cached_results: Option<Vec<serde_json::Value>>,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub offline_mode: bool,
    pub sync_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchStatus {
    Processing,
    Completed,
    Failed,
    QueuedOffline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub synced_items: u64,
    pub failed_items: u64,
    pub sync_duration_ms: u64,
    pub last_sync: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileDashboard {
    pub device_id: Uuid,
    pub user_id: Uuid,
    pub recent_sessions: Vec<MobileSessionSummary>,
    pub sync_status: SyncStatus,
    pub cached_data_summary: CachedDataSummary,
    pub performance_metrics: DevicePerformanceMetrics,
    pub quick_actions: Vec<QuickAction>,
    pub recommendations: Vec<MobileRecommendation>,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileSessionSummary {
    pub session_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub duration_minutes: u32,
    pub research_count: u32,
    pub success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedDataSummary {
    pub total_cached_items: u64,
    pub cache_size_mb: f64,
    pub last_cache_update: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevicePerformanceMetrics {
    pub average_response_time_ms: f64,
    pub success_rate: f64,
    pub offline_usage_percentage: f64,
    pub battery_efficiency_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickAction {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub action_type: ActionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    NewResearch,
    ViewResults,
    ExportResults,
    SyncData,
    Settings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileRecommendation {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub priority: RecommendationPriority,
    pub action_required: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationPriority {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobilePlatformMetrics {
    pub total_registered_devices: u64,
    pub active_sessions: u64,
    pub total_mobile_research_sessions: u64,
    pub offline_sync_operations: u64,
    pub average_mobile_response_time: f64,
    pub mobile_user_satisfaction: f64,
}
