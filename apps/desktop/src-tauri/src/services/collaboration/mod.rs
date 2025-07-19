use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use tracing::{info, debug, warn, error};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, ResearchError};
use crate::models::research_workflow::ResearchWorkflow;
use crate::services::Service;

pub mod user_management;
pub mod sharing_service;
pub mod real_time_sync;
pub mod team_management;
pub mod permissions;

use user_management::{UserManager, User, UserRole};
use sharing_service::{SharingService, ShareRequest, SharePermission};
use real_time_sync::{RealTimeSyncService, SyncEvent, SyncEventType};
use team_management::{TeamManager, Team, TeamMember};
use permissions::{PermissionManager, Permission, ResourceType};

/// Collaboration service for team sharing and real-time collaboration (V1.1.0)
pub struct CollaborationService {
    user_manager: Arc<RwLock<UserManager>>,
    sharing_service: Arc<RwLock<SharingService>>,
    real_time_sync: Arc<RwLock<RealTimeSyncService>>,
    team_manager: Arc<RwLock<TeamManager>>,
    permission_manager: Arc<RwLock<PermissionManager>>,
    active_sessions: Arc<RwLock<HashMap<Uuid, CollaborationSession>>>,
    event_broadcaster: broadcast::Sender<CollaborationEvent>,
}

/// Collaboration session tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationSession {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub workflow_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub session_type: SessionType,
    pub permissions: Vec<Permission>,
}

/// Types of collaboration sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionType {
    ReadOnly,
    Edit,
    Comment,
    Review,
    Admin,
}

/// Collaboration events for real-time updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationEvent {
    pub event_id: Uuid,
    pub event_type: CollaborationEventType,
    pub user_id: Uuid,
    pub workflow_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
    pub data: serde_json::Value,
}

/// Types of collaboration events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollaborationEventType {
    WorkflowShared,
    WorkflowUnshared,
    UserJoinedSession,
    UserLeftSession,
    WorkflowModified,
    CommentAdded,
    PermissionChanged,
    TeamCreated,
    TeamMemberAdded,
    TeamMemberRemoved,
}

/// Collaboration statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationStats {
    pub total_users: u64,
    pub active_sessions: u64,
    pub shared_workflows: u64,
    pub teams_count: u64,
    pub total_collaborations: u64,
    pub average_session_duration_minutes: f64,
    pub most_active_users: Vec<UserActivity>,
    pub collaboration_trends: Vec<CollaborationTrend>,
}

/// User activity tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserActivity {
    pub user_id: Uuid,
    pub username: String,
    pub total_sessions: u64,
    pub total_duration_minutes: u64,
    pub workflows_shared: u64,
    pub workflows_accessed: u64,
    pub last_active: DateTime<Utc>,
}

/// Collaboration trend data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationTrend {
    pub date: DateTime<Utc>,
    pub active_users: u32,
    pub new_shares: u32,
    pub total_sessions: u32,
    pub average_session_duration: f64,
}

impl CollaborationService {
    /// Create a new collaboration service
    pub async fn new() -> AppResult<Self> {
        info!("Initializing collaboration service...");

        let user_manager = Arc::new(RwLock::new(UserManager::new().await?));
        let sharing_service = Arc::new(RwLock::new(SharingService::new().await?));
        let real_time_sync = Arc::new(RwLock::new(RealTimeSyncService::new().await?));
        let team_manager = Arc::new(RwLock::new(TeamManager::new().await?));
        let permission_manager = Arc::new(RwLock::new(PermissionManager::new().await?));
        let active_sessions = Arc::new(RwLock::new(HashMap::new()));

        let (event_broadcaster, _) = broadcast::channel(1000);

        let service = Self {
            user_manager,
            sharing_service,
            real_time_sync,
            team_manager,
            permission_manager,
            active_sessions,
            event_broadcaster,
        };

        info!("Collaboration service initialized successfully");
        Ok(service)
    }

    /// Start a collaboration session
    pub async fn start_session(
        &self,
        user_id: Uuid,
        workflow_id: Uuid,
        session_type: SessionType,
    ) -> AppResult<CollaborationSession> {
        info!("Starting collaboration session for user {} on workflow {}", user_id, workflow_id);

        // Check permissions
        let permission_manager = self.permission_manager.read().await;
        let required_permission = match session_type {
            SessionType::ReadOnly => Permission::Read,
            SessionType::Edit => Permission::Write,
            SessionType::Comment => Permission::Comment,
            SessionType::Review => Permission::Review,
            SessionType::Admin => Permission::Admin,
        };

        if !permission_manager.check_permission(user_id, ResourceType::Workflow(workflow_id), required_permission).await? {
            return Err(ResearchError::permission_denied("Insufficient permissions for this session type".to_string()).into());
        }

        drop(permission_manager);

        // Create session
        let session = CollaborationSession {
            session_id: Uuid::new_v4(),
            user_id,
            workflow_id,
            started_at: Utc::now(),
            last_activity: Utc::now(),
            session_type,
            permissions: vec![required_permission],
        };

        // Store session
        {
            let mut sessions = self.active_sessions.write().await;
            sessions.insert(session.session_id, session.clone());
        }

        // Broadcast event
        let event = CollaborationEvent {
            event_id: Uuid::new_v4(),
            event_type: CollaborationEventType::UserJoinedSession,
            user_id,
            workflow_id: Some(workflow_id),
            team_id: None,
            timestamp: Utc::now(),
            data: serde_json::to_value(&session)?,
        };

        let _ = self.event_broadcaster.send(event);

        info!("Collaboration session started: {}", session.session_id);
        Ok(session)
    }

    /// End a collaboration session
    pub async fn end_session(&self, session_id: Uuid) -> AppResult<()> {
        info!("Ending collaboration session: {}", session_id);

        let session = {
            let mut sessions = self.active_sessions.write().await;
            sessions.remove(&session_id)
        };

        if let Some(session) = session {
            // Broadcast event
            let event = CollaborationEvent {
                event_id: Uuid::new_v4(),
                event_type: CollaborationEventType::UserLeftSession,
                user_id: session.user_id,
                workflow_id: Some(session.workflow_id),
                team_id: None,
                timestamp: Utc::now(),
                data: serde_json::to_value(&session)?,
            };

            let _ = self.event_broadcaster.send(event);
            info!("Collaboration session ended: {}", session_id);
        }

        Ok(())
    }

    /// Share a workflow with users or teams
    pub async fn share_workflow(
        &self,
        owner_id: Uuid,
        workflow_id: Uuid,
        share_request: ShareRequest,
    ) -> AppResult<()> {
        info!("Sharing workflow {} by user {}", workflow_id, owner_id);

        // Check if user owns the workflow or has admin permissions
        let permission_manager = self.permission_manager.read().await;
        if !permission_manager.check_permission(owner_id, ResourceType::Workflow(workflow_id), Permission::Admin).await? {
            return Err(ResearchError::permission_denied("Only workflow owners can share workflows".to_string()).into());
        }
        drop(permission_manager);

        // Process sharing
        let sharing_service = self.sharing_service.write().await;
        sharing_service.share_workflow(workflow_id, share_request.clone()).await?;
        drop(sharing_service);

        // Broadcast event
        let event = CollaborationEvent {
            event_id: Uuid::new_v4(),
            event_type: CollaborationEventType::WorkflowShared,
            user_id: owner_id,
            workflow_id: Some(workflow_id),
            team_id: None,
            timestamp: Utc::now(),
            data: serde_json::to_value(&share_request)?,
        };

        let _ = self.event_broadcaster.send(event);

        info!("Workflow {} shared successfully", workflow_id);
        Ok(())
    }

    /// Unshare a workflow
    pub async fn unshare_workflow(
        &self,
        owner_id: Uuid,
        workflow_id: Uuid,
        target_user_id: Option<Uuid>,
        target_team_id: Option<Uuid>,
    ) -> AppResult<()> {
        info!("Unsharing workflow {} by user {}", workflow_id, owner_id);

        // Check permissions
        let permission_manager = self.permission_manager.read().await;
        if !permission_manager.check_permission(owner_id, ResourceType::Workflow(workflow_id), Permission::Admin).await? {
            return Err(ResearchError::permission_denied("Only workflow owners can unshare workflows".to_string()).into());
        }
        drop(permission_manager);

        // Process unsharing
        let sharing_service = self.sharing_service.write().await;
        sharing_service.unshare_workflow(workflow_id, target_user_id, target_team_id).await?;
        drop(sharing_service);

        // Broadcast event
        let event = CollaborationEvent {
            event_id: Uuid::new_v4(),
            event_type: CollaborationEventType::WorkflowUnshared,
            user_id: owner_id,
            workflow_id: Some(workflow_id),
            team_id: target_team_id,
            timestamp: Utc::now(),
            data: serde_json::json!({
                "target_user_id": target_user_id,
                "target_team_id": target_team_id
            }),
        };

        let _ = self.event_broadcaster.send(event);

        info!("Workflow {} unshared successfully", workflow_id);
        Ok(())
    }

    /// Get shared workflows for a user
    pub async fn get_shared_workflows(&self, user_id: Uuid) -> AppResult<Vec<Uuid>> {
        let sharing_service = self.sharing_service.read().await;
        sharing_service.get_shared_workflows(user_id).await
    }

    /// Create a team
    pub async fn create_team(&self, creator_id: Uuid, team_name: String, description: Option<String>) -> AppResult<Team> {
        info!("Creating team '{}' by user {}", team_name, creator_id);

        let team_manager = self.team_manager.write().await;
        let team = team_manager.create_team(creator_id, team_name, description).await?;
        drop(team_manager);

        // Broadcast event
        let event = CollaborationEvent {
            event_id: Uuid::new_v4(),
            event_type: CollaborationEventType::TeamCreated,
            user_id: creator_id,
            workflow_id: None,
            team_id: Some(team.id),
            timestamp: Utc::now(),
            data: serde_json::to_value(&team)?,
        };

        let _ = self.event_broadcaster.send(event);

        info!("Team created: {} ({})", team.name, team.id);
        Ok(team)
    }

    /// Add member to team
    pub async fn add_team_member(
        &self,
        admin_id: Uuid,
        team_id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<()> {
        info!("Adding user {} to team {} by admin {}", user_id, team_id, admin_id);

        let team_manager = self.team_manager.write().await;
        team_manager.add_member(admin_id, team_id, user_id, role).await?;
        drop(team_manager);

        // Broadcast event
        let event = CollaborationEvent {
            event_id: Uuid::new_v4(),
            event_type: CollaborationEventType::TeamMemberAdded,
            user_id: admin_id,
            workflow_id: None,
            team_id: Some(team_id),
            timestamp: Utc::now(),
            data: serde_json::json!({
                "added_user_id": user_id,
                "role": role
            }),
        };

        let _ = self.event_broadcaster.send(event);

        info!("User {} added to team {}", user_id, team_id);
        Ok(())
    }

    /// Get active sessions
    pub async fn get_active_sessions(&self) -> AppResult<Vec<CollaborationSession>> {
        let sessions = self.active_sessions.read().await;
        Ok(sessions.values().cloned().collect())
    }

    /// Get collaboration statistics
    pub async fn get_collaboration_stats(&self) -> AppResult<CollaborationStats> {
        let user_manager = self.user_manager.read().await;
        let team_manager = self.team_manager.read().await;
        let sharing_service = self.sharing_service.read().await;
        let sessions = self.active_sessions.read().await;

        let total_users = user_manager.get_user_count().await?;
        let active_sessions = sessions.len() as u64;
        let shared_workflows = sharing_service.get_shared_workflow_count().await?;
        let teams_count = team_manager.get_team_count().await?;

        Ok(CollaborationStats {
            total_users,
            active_sessions,
            shared_workflows,
            teams_count,
            total_collaborations: 0, // TODO: Implement
            average_session_duration_minutes: 0.0, // TODO: Implement
            most_active_users: Vec::new(), // TODO: Implement
            collaboration_trends: Vec::new(), // TODO: Implement
        })
    }

    /// Subscribe to collaboration events
    pub fn subscribe_to_events(&self) -> broadcast::Receiver<CollaborationEvent> {
        self.event_broadcaster.subscribe()
    }

    /// Update session activity
    pub async fn update_session_activity(&self, session_id: Uuid) -> AppResult<()> {
        let mut sessions = self.active_sessions.write().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            session.last_activity = Utc::now();
        }
        Ok(())
    }

    /// Clean up inactive sessions
    pub async fn cleanup_inactive_sessions(&self, timeout_minutes: u32) -> AppResult<u32> {
        let cutoff_time = Utc::now() - chrono::Duration::minutes(timeout_minutes as i64);
        let mut sessions = self.active_sessions.write().await;
        
        let initial_count = sessions.len();
        sessions.retain(|_, session| session.last_activity > cutoff_time);
        let removed_count = initial_count - sessions.len();

        if removed_count > 0 {
            info!("Cleaned up {} inactive collaboration sessions", removed_count);
        }

        Ok(removed_count as u32)
    }
}

#[async_trait::async_trait]
impl Service for CollaborationService {
    async fn health_check(&self) -> AppResult<()> {
        debug!("Performing collaboration service health check");
        
        // Check all sub-services
        {
            let user_manager = self.user_manager.read().await;
            user_manager.health_check().await?;
        }
        
        {
            let sharing_service = self.sharing_service.read().await;
            sharing_service.health_check().await?;
        }
        
        {
            let real_time_sync = self.real_time_sync.read().await;
            real_time_sync.health_check().await?;
        }
        
        {
            let team_manager = self.team_manager.read().await;
            team_manager.health_check().await?;
        }
        
        {
            let permission_manager = self.permission_manager.read().await;
            permission_manager.health_check().await?;
        }

        Ok(())
    }

    async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down collaboration service...");

        // End all active sessions
        {
            let sessions = self.active_sessions.read().await;
            for session_id in sessions.keys() {
                let _ = self.end_session(*session_id).await;
            }
        }

        // Shutdown sub-services
        {
            let real_time_sync = self.real_time_sync.write().await;
            real_time_sync.shutdown().await?;
        }

        info!("Collaboration service shutdown complete");
        Ok(())
    }
}
