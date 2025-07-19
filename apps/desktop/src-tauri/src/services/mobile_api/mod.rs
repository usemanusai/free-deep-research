use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, warn, error};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, ResearchError};
use crate::models::research_workflow::{ResearchWorkflow, WorkflowStatus};
use crate::services::Service;

pub mod device_management;
pub mod push_notifications;
pub mod sync_service;
pub mod mobile_auth;
pub mod offline_support;

use device_management::{DeviceManager, MobileDevice, DeviceType};
use push_notifications::{PushNotificationService, NotificationPayload, NotificationType};
use sync_service::{MobileSyncService, SyncRequest, SyncResponse};
use mobile_auth::{MobileAuthService, AuthToken, AuthRequest};
use offline_support::{OfflineSupportService, OfflineData, OfflineOperation};

/// Mobile API service for companion app support (V1.1.0)
pub struct MobileApiService {
    device_manager: Arc<RwLock<DeviceManager>>,
    push_notifications: Arc<RwLock<PushNotificationService>>,
    sync_service: Arc<RwLock<MobileSyncService>>,
    mobile_auth: Arc<RwLock<MobileAuthService>>,
    offline_support: Arc<RwLock<OfflineSupportService>>,
    active_connections: Arc<RwLock<HashMap<Uuid, MobileConnection>>>,
    api_endpoints: HashMap<String, MobileEndpoint>,
}

/// Mobile device connection tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileConnection {
    pub connection_id: Uuid,
    pub device_id: Uuid,
    pub user_id: Uuid,
    pub connected_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub app_version: String,
    pub platform: MobilePlatform,
    pub push_token: Option<String>,
    pub sync_enabled: bool,
}

/// Mobile platforms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MobilePlatform {
    iOS,
    Android,
    Web,
}

/// Mobile API endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileEndpoint {
    pub path: String,
    pub method: String,
    pub description: String,
    pub auth_required: bool,
    pub rate_limit: Option<u32>,
    pub supported_versions: Vec<String>,
}

/// Mobile API request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileApiRequest {
    pub request_id: Uuid,
    pub device_id: Uuid,
    pub user_id: Option<Uuid>,
    pub endpoint: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<serde_json::Value>,
    pub timestamp: DateTime<Utc>,
}

/// Mobile API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileApiResponse {
    pub request_id: Uuid,
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: serde_json::Value,
    pub processing_time_ms: u64,
    pub timestamp: DateTime<Utc>,
}

/// Mobile workflow summary for lightweight display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileWorkflowSummary {
    pub id: Uuid,
    pub name: String,
    pub status: WorkflowStatus,
    pub progress: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub result_summary: Option<String>,
}

/// Mobile dashboard data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileDashboard {
    pub active_workflows: Vec<MobileWorkflowSummary>,
    pub recent_completions: Vec<MobileWorkflowSummary>,
    pub system_status: MobileSystemStatus,
    pub quick_stats: MobileQuickStats,
    pub notifications: Vec<MobileNotification>,
}

/// Mobile system status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileSystemStatus {
    pub overall_health: String,
    pub api_services_online: u32,
    pub api_services_total: u32,
    pub queue_length: u32,
    pub last_updated: DateTime<Utc>,
}

/// Mobile quick statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileQuickStats {
    pub total_workflows: u64,
    pub completed_today: u32,
    pub average_completion_time_minutes: f64,
    pub success_rate_percentage: f64,
    pub cost_savings_today: f64,
}

/// Mobile notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileNotification {
    pub id: Uuid,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub workflow_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub read: bool,
    pub action_url: Option<String>,
}

impl MobileApiService {
    /// Create a new mobile API service
    pub async fn new() -> AppResult<Self> {
        info!("Initializing mobile API service...");

        let device_manager = Arc::new(RwLock::new(DeviceManager::new().await?));
        let push_notifications = Arc::new(RwLock::new(PushNotificationService::new().await?));
        let sync_service = Arc::new(RwLock::new(MobileSyncService::new().await?));
        let mobile_auth = Arc::new(RwLock::new(MobileAuthService::new().await?));
        let offline_support = Arc::new(RwLock::new(OfflineSupportService::new().await?));
        let active_connections = Arc::new(RwLock::new(HashMap::new()));

        let mut api_endpoints = HashMap::new();
        Self::initialize_api_endpoints(&mut api_endpoints);

        let service = Self {
            device_manager,
            push_notifications,
            sync_service,
            mobile_auth,
            offline_support,
            active_connections,
            api_endpoints,
        };

        info!("Mobile API service initialized successfully");
        Ok(service)
    }

    /// Initialize mobile API endpoints
    fn initialize_api_endpoints(endpoints: &mut HashMap<String, MobileEndpoint>) {
        // Authentication endpoints
        endpoints.insert("auth/login".to_string(), MobileEndpoint {
            path: "/api/mobile/auth/login".to_string(),
            method: "POST".to_string(),
            description: "Authenticate mobile device".to_string(),
            auth_required: false,
            rate_limit: Some(5), // 5 requests per minute
            supported_versions: vec!["1.0".to_string(), "1.1".to_string()],
        });

        // Dashboard endpoints
        endpoints.insert("dashboard".to_string(), MobileEndpoint {
            path: "/api/mobile/dashboard".to_string(),
            method: "GET".to_string(),
            description: "Get mobile dashboard data".to_string(),
            auth_required: true,
            rate_limit: Some(60), // 60 requests per minute
            supported_versions: vec!["1.0".to_string(), "1.1".to_string()],
        });

        // Workflow endpoints
        endpoints.insert("workflows".to_string(), MobileEndpoint {
            path: "/api/mobile/workflows".to_string(),
            method: "GET".to_string(),
            description: "Get user workflows".to_string(),
            auth_required: true,
            rate_limit: Some(30),
            supported_versions: vec!["1.0".to_string(), "1.1".to_string()],
        });

        endpoints.insert("workflows/start".to_string(), MobileEndpoint {
            path: "/api/mobile/workflows/start".to_string(),
            method: "POST".to_string(),
            description: "Start a new workflow".to_string(),
            auth_required: true,
            rate_limit: Some(10),
            supported_versions: vec!["1.1".to_string()],
        });

        endpoints.insert("workflows/stop".to_string(), MobileEndpoint {
            path: "/api/mobile/workflows/stop".to_string(),
            method: "POST".to_string(),
            description: "Stop a running workflow".to_string(),
            auth_required: true,
            rate_limit: Some(20),
            supported_versions: vec!["1.0".to_string(), "1.1".to_string()],
        });

        // Notification endpoints
        endpoints.insert("notifications".to_string(), MobileEndpoint {
            path: "/api/mobile/notifications".to_string(),
            method: "GET".to_string(),
            description: "Get user notifications".to_string(),
            auth_required: true,
            rate_limit: Some(30),
            supported_versions: vec!["1.0".to_string(), "1.1".to_string()],
        });

        // Sync endpoints
        endpoints.insert("sync".to_string(), MobileEndpoint {
            path: "/api/mobile/sync".to_string(),
            method: "POST".to_string(),
            description: "Sync data with mobile device".to_string(),
            auth_required: true,
            rate_limit: Some(10),
            supported_versions: vec!["1.0".to_string(), "1.1".to_string()],
        });
    }

    /// Register a mobile device
    pub async fn register_device(
        &self,
        user_id: Uuid,
        device_info: MobileDevice,
        app_version: String,
        push_token: Option<String>,
    ) -> AppResult<MobileConnection> {
        info!("Registering mobile device for user {}", user_id);

        // Register device
        let device_manager = self.device_manager.write().await;
        let device = device_manager.register_device(user_id, device_info).await?;
        drop(device_manager);

        // Create connection
        let connection = MobileConnection {
            connection_id: Uuid::new_v4(),
            device_id: device.id,
            user_id,
            connected_at: Utc::now(),
            last_activity: Utc::now(),
            app_version,
            platform: device.device_type.into(),
            push_token,
            sync_enabled: true,
        };

        // Store connection
        {
            let mut connections = self.active_connections.write().await;
            connections.insert(connection.connection_id, connection.clone());
        }

        // Setup push notifications if token provided
        if let Some(token) = &push_token {
            let push_service = self.push_notifications.write().await;
            push_service.register_device_token(device.id, token.clone()).await?;
        }

        info!("Mobile device registered: {} ({})", device.name, device.id);
        Ok(connection)
    }

    /// Get mobile dashboard data
    pub async fn get_dashboard(&self, user_id: Uuid) -> AppResult<MobileDashboard> {
        debug!("Getting mobile dashboard for user {}", user_id);

        // This would integrate with other services to get real data
        // For now, returning mock data structure
        Ok(MobileDashboard {
            active_workflows: Vec::new(),
            recent_completions: Vec::new(),
            system_status: MobileSystemStatus {
                overall_health: "Healthy".to_string(),
                api_services_online: 6,
                api_services_total: 6,
                queue_length: 0,
                last_updated: Utc::now(),
            },
            quick_stats: MobileQuickStats {
                total_workflows: 0,
                completed_today: 0,
                average_completion_time_minutes: 0.0,
                success_rate_percentage: 100.0,
                cost_savings_today: 0.0,
            },
            notifications: Vec::new(),
        })
    }

    /// Start a workflow from mobile
    pub async fn start_workflow_mobile(
        &self,
        user_id: Uuid,
        workflow_name: String,
        query: String,
        methodology: Option<String>,
    ) -> AppResult<Uuid> {
        info!("Starting workflow '{}' from mobile for user {}", workflow_name, user_id);

        // This would integrate with the research engine service
        // For now, returning a mock workflow ID
        let workflow_id = Uuid::new_v4();

        // Send push notification
        let push_service = self.push_notifications.read().await;
        let notification = NotificationPayload {
            title: "Workflow Started".to_string(),
            body: format!("Your research workflow '{}' has been started", workflow_name),
            notification_type: NotificationType::WorkflowStarted,
            workflow_id: Some(workflow_id),
            data: HashMap::new(),
        };

        push_service.send_to_user(user_id, notification).await?;

        Ok(workflow_id)
    }

    /// Stop a workflow from mobile
    pub async fn stop_workflow_mobile(&self, user_id: Uuid, workflow_id: Uuid) -> AppResult<()> {
        info!("Stopping workflow {} from mobile for user {}", workflow_id, user_id);

        // This would integrate with the research engine service
        // For now, just sending a notification

        let push_service = self.push_notifications.read().await;
        let notification = NotificationPayload {
            title: "Workflow Stopped".to_string(),
            body: "Your research workflow has been stopped".to_string(),
            notification_type: NotificationType::WorkflowStopped,
            workflow_id: Some(workflow_id),
            data: HashMap::new(),
        };

        push_service.send_to_user(user_id, notification).await?;

        Ok(())
    }

    /// Get user workflows for mobile display
    pub async fn get_user_workflows_mobile(&self, user_id: Uuid) -> AppResult<Vec<MobileWorkflowSummary>> {
        debug!("Getting workflows for mobile user {}", user_id);

        // This would integrate with the research engine service
        // For now, returning empty list
        Ok(Vec::new())
    }

    /// Sync data with mobile device
    pub async fn sync_with_device(
        &self,
        device_id: Uuid,
        sync_request: SyncRequest,
    ) -> AppResult<SyncResponse> {
        debug!("Syncing data with device {}", device_id);

        let sync_service = self.sync_service.read().await;
        sync_service.sync_device_data(device_id, sync_request).await
    }

    /// Send push notification to user
    pub async fn send_notification_to_user(
        &self,
        user_id: Uuid,
        notification: NotificationPayload,
    ) -> AppResult<()> {
        let push_service = self.push_notifications.read().await;
        push_service.send_to_user(user_id, notification).await
    }

    /// Get user notifications
    pub async fn get_user_notifications(&self, user_id: Uuid, limit: Option<u32>) -> AppResult<Vec<MobileNotification>> {
        debug!("Getting notifications for user {}", user_id);

        // This would integrate with the notification service
        // For now, returning empty list
        Ok(Vec::new())
    }

    /// Mark notification as read
    pub async fn mark_notification_read(&self, user_id: Uuid, notification_id: Uuid) -> AppResult<()> {
        debug!("Marking notification {} as read for user {}", notification_id, user_id);
        // Implementation would update notification status
        Ok(())
    }

    /// Get active mobile connections
    pub async fn get_active_connections(&self) -> AppResult<Vec<MobileConnection>> {
        let connections = self.active_connections.read().await;
        Ok(connections.values().cloned().collect())
    }

    /// Disconnect mobile device
    pub async fn disconnect_device(&self, connection_id: Uuid) -> AppResult<()> {
        info!("Disconnecting mobile device: {}", connection_id);

        let mut connections = self.active_connections.write().await;
        connections.remove(&connection_id);

        Ok(())
    }

    /// Update connection activity
    pub async fn update_connection_activity(&self, connection_id: Uuid) -> AppResult<()> {
        let mut connections = self.active_connections.write().await;
        if let Some(connection) = connections.get_mut(&connection_id) {
            connection.last_activity = Utc::now();
        }
        Ok(())
    }

    /// Get API endpoints
    pub fn get_api_endpoints(&self) -> &HashMap<String, MobileEndpoint> {
        &self.api_endpoints
    }

    /// Validate API request
    pub async fn validate_request(&self, request: &MobileApiRequest) -> AppResult<bool> {
        // Check if endpoint exists
        if !self.api_endpoints.contains_key(&request.endpoint) {
            return Ok(false);
        }

        // Check authentication if required
        let endpoint = &self.api_endpoints[&request.endpoint];
        if endpoint.auth_required && request.user_id.is_none() {
            return Ok(false);
        }

        // Check rate limits (would be implemented with actual rate limiting)
        // For now, always return true
        Ok(true)
    }

    /// Process mobile API request
    pub async fn process_request(&self, request: MobileApiRequest) -> AppResult<MobileApiResponse> {
        let start_time = std::time::Instant::now();

        // Validate request
        if !self.validate_request(&request).await? {
            return Ok(MobileApiResponse {
                request_id: request.request_id,
                status_code: 400,
                headers: HashMap::new(),
                body: serde_json::json!({"error": "Invalid request"}),
                processing_time_ms: start_time.elapsed().as_millis() as u64,
                timestamp: Utc::now(),
            });
        }

        // Route request to appropriate handler
        let response_body = match request.endpoint.as_str() {
            "dashboard" => {
                if let Some(user_id) = request.user_id {
                    let dashboard = self.get_dashboard(user_id).await?;
                    serde_json::to_value(dashboard)?
                } else {
                    serde_json::json!({"error": "User ID required"})
                }
            }
            "workflows" => {
                if let Some(user_id) = request.user_id {
                    let workflows = self.get_user_workflows_mobile(user_id).await?;
                    serde_json::to_value(workflows)?
                } else {
                    serde_json::json!({"error": "User ID required"})
                }
            }
            "notifications" => {
                if let Some(user_id) = request.user_id {
                    let notifications = self.get_user_notifications(user_id, Some(50)).await?;
                    serde_json::to_value(notifications)?
                } else {
                    serde_json::json!({"error": "User ID required"})
                }
            }
            _ => serde_json::json!({"error": "Endpoint not implemented"}),
        };

        Ok(MobileApiResponse {
            request_id: request.request_id,
            status_code: 200,
            headers: HashMap::new(),
            body: response_body,
            processing_time_ms: start_time.elapsed().as_millis() as u64,
            timestamp: Utc::now(),
        })
    }
}

#[async_trait::async_trait]
impl Service for MobileApiService {
    async fn health_check(&self) -> AppResult<()> {
        debug!("Performing mobile API service health check");

        // Check all sub-services
        {
            let device_manager = self.device_manager.read().await;
            device_manager.health_check().await?;
        }

        {
            let push_notifications = self.push_notifications.read().await;
            push_notifications.health_check().await?;
        }

        {
            let sync_service = self.sync_service.read().await;
            sync_service.health_check().await?;
        }

        {
            let mobile_auth = self.mobile_auth.read().await;
            mobile_auth.health_check().await?;
        }

        {
            let offline_support = self.offline_support.read().await;
            offline_support.health_check().await?;
        }

        Ok(())
    }

    async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down mobile API service...");

        // Disconnect all active connections
        {
            let connections = self.active_connections.read().await;
            for connection_id in connections.keys() {
                let _ = self.disconnect_device(*connection_id).await;
            }
        }

        // Shutdown sub-services
        {
            let push_notifications = self.push_notifications.write().await;
            push_notifications.shutdown().await?;
        }

        {
            let sync_service = self.sync_service.write().await;
            sync_service.shutdown().await?;
        }

        info!("Mobile API service shutdown complete");
        Ok(())
    }
}

impl From<DeviceType> for MobilePlatform {
    fn from(device_type: DeviceType) -> Self {
        match device_type {
            DeviceType::iOS => MobilePlatform::iOS,
            DeviceType::Android => MobilePlatform::Android,
            DeviceType::Web => MobilePlatform::Web,
            _ => MobilePlatform::Web,
        }
    }
}
