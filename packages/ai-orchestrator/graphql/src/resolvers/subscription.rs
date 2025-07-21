// GraphQL Subscription Resolvers for Free Deep Research System
// Phase 4.4: API Gateway & GraphQL

use async_graphql::{Context, Result, Subscription, ID, subscription::SimpleBroker};
use futures_util::{Stream, StreamExt};
use std::time::Duration;
use uuid::Uuid;

use crate::{
    types::*,
    AppContext, GraphQLError,
};

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    // Workflow execution updates
    async fn workflow_execution_updates(
        &self,
        ctx: &Context<'_>,
        workflow_id: ID,
    ) -> Result<impl Stream<Item = WorkflowExecutionUpdate>> {
        let workflow_uuid = Uuid::parse_str(&workflow_id)?;
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;
        
        // Verify access to workflow
        let workflow = app_ctx.research_engine.get_workflow(workflow_uuid).await?
            .ok_or_else(|| GraphQLError::Validation("Workflow not found".to_string()))?;
        
        if workflow.creator_id != current_user.id && 
           !workflow.collaborators.contains(&current_user.id) &&
           !current_user.is_admin() {
            return Err(GraphQLError::Auth("Access denied".to_string()).into());
        }

        // Subscribe to workflow execution updates
        Ok(SimpleBroker::<WorkflowExecutionUpdate>::subscribe()
            .filter(move |update| {
                let matches = update.workflow_id == workflow_uuid;
                async move { matches }
            }))
    }

    // Workflow status changes for user
    async fn workflow_status_changed(
        &self,
        ctx: &Context<'_>,
        user_id: Option<ID>,
    ) -> Result<impl Stream<Item = WorkflowStatusUpdate>> {
        let current_user = self.require_auth(ctx).await?;
        
        let target_user_id = if let Some(id) = user_id {
            let uuid = Uuid::parse_str(&id)?;
            // Check if user can subscribe to other user's updates
            if uuid != current_user.id && !current_user.is_admin() {
                return Err(GraphQLError::Auth("Access denied".to_string()).into());
            }
            uuid
        } else {
            current_user.id
        };

        Ok(SimpleBroker::<WorkflowStatusUpdate>::subscribe()
            .filter(move |update| {
                let matches = update.user_id == target_user_id;
                async move { matches }
            }))
    }

    // System metrics updates
    async fn system_metrics_updates(
        &self,
        ctx: &Context<'_>,
    ) -> Result<impl Stream<Item = SystemMetricsUpdate>> {
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;
        
        // Check monitoring permissions
        app_ctx.auth_service.authorize(&current_user, "monitoring", "read").await?;

        // Create a stream that emits system metrics every 30 seconds
        Ok(async_stream::stream! {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                if let Ok(metrics) = app_ctx.metrics.get_live_metrics().await {
                    yield SystemMetricsUpdate {
                        timestamp: chrono::Utc::now(),
                        metrics,
                        alert_level: determine_alert_level(&metrics),
                    };
                }
            }
        })
    }

    // Performance alerts
    async fn performance_alerts(
        &self,
        ctx: &Context<'_>,
    ) -> Result<impl Stream<Item = PerformanceAlert>> {
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;
        
        app_ctx.auth_service.authorize(&current_user, "monitoring", "read").await?;

        Ok(SimpleBroker::<PerformanceAlert>::subscribe())
    }

    // API key usage updates
    async fn api_key_usage_updates(
        &self,
        ctx: &Context<'_>,
        key_id: ID,
    ) -> Result<impl Stream<Item = ApiKeyUsageUpdate>> {
        let key_uuid = Uuid::parse_str(&key_id)?;
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;
        
        // Verify ownership of API key
        let api_key = app_ctx.api_manager.get_api_key(key_uuid).await?
            .ok_or_else(|| GraphQLError::Validation("API key not found".to_string()))?;
        
        if api_key.user_id != current_user.id {
            return Err(GraphQLError::Auth("Access denied".to_string()).into());
        }

        Ok(SimpleBroker::<ApiKeyUsageUpdate>::subscribe()
            .filter(move |update| {
                let matches = update.key_id == key_uuid;
                async move { matches }
            }))
    }

    // Rate limit alerts for user
    async fn rate_limit_alerts(
        &self,
        ctx: &Context<'_>,
        user_id: Option<ID>,
    ) -> Result<impl Stream<Item = RateLimitAlert>> {
        let current_user = self.require_auth(ctx).await?;
        
        let target_user_id = if let Some(id) = user_id {
            let uuid = Uuid::parse_str(&id)?;
            if uuid != current_user.id && !current_user.is_admin() {
                return Err(GraphQLError::Auth("Access denied".to_string()).into());
            }
            uuid
        } else {
            current_user.id
        };

        Ok(SimpleBroker::<RateLimitAlert>::subscribe()
            .filter(move |alert| {
                let matches = alert.user_id == target_user_id;
                async move { matches }
            }))
    }

    // Federated research updates
    async fn federated_research_updates(
        &self,
        ctx: &Context<'_>,
        network_id: ID,
    ) -> Result<impl Stream<Item = FederatedResearchUpdate>> {
        let network_uuid = Uuid::parse_str(&network_id)?;
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;
        
        // Verify access to federated network
        let has_access = app_ctx.federated_service
            .check_network_access(network_uuid, current_user.id).await?;
        
        if !has_access {
            return Err(GraphQLError::Auth("Access denied to federated network".to_string()).into());
        }

        Ok(SimpleBroker::<FederatedResearchUpdate>::subscribe()
            .filter(move |update| {
                let matches = update.network_id == network_uuid;
                async move { matches }
            }))
    }

    // Collaboration invites
    async fn collaboration_invites(
        &self,
        ctx: &Context<'_>,
        user_id: ID,
    ) -> Result<impl Stream<Item = CollaborationInvite>> {
        let target_user_id = Uuid::parse_str(&user_id)?;
        let current_user = self.require_auth(ctx).await?;
        
        // Users can only subscribe to their own invites
        if target_user_id != current_user.id && !current_user.is_admin() {
            return Err(GraphQLError::Auth("Access denied".to_string()).into());
        }

        Ok(SimpleBroker::<CollaborationInvite>::subscribe()
            .filter(move |invite| {
                let matches = invite.invited_user_id == target_user_id;
                async move { matches }
            }))
    }

    // Knowledge graph updates
    async fn knowledge_graph_updates(
        &self,
        ctx: &Context<'_>,
        graph_id: ID,
    ) -> Result<impl Stream<Item = KnowledgeGraphUpdate>> {
        let graph_uuid = Uuid::parse_str(&graph_id)?;
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;
        
        // Verify access to knowledge graph
        let graph = app_ctx.knowledge_service.get_graph(graph_uuid, current_user.id).await?
            .ok_or_else(|| GraphQLError::Validation("Knowledge graph not found".to_string()))?;
        
        // Check if user has read access to the graph
        if !graph.has_read_access(current_user.id) {
            return Err(GraphQLError::Auth("Access denied".to_string()).into());
        }

        Ok(SimpleBroker::<KnowledgeGraphUpdate>::subscribe()
            .filter(move |update| {
                let matches = update.graph_id == graph_uuid;
                async move { matches }
            }))
    }

    // BMAD execution updates
    async fn bmad_execution_updates(
        &self,
        ctx: &Context<'_>,
        execution_id: ID,
    ) -> Result<impl Stream<Item = BMadExecutionUpdate>> {
        let execution_uuid = Uuid::parse_str(&execution_id)?;
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;
        
        // Verify access to BMAD execution
        let execution = app_ctx.bmad_service.get_execution(execution_uuid).await?
            .ok_or_else(|| GraphQLError::Validation("BMAD execution not found".to_string()))?;
        
        if execution.executor_id != current_user.id && !current_user.is_admin() {
            return Err(GraphQLError::Auth("Access denied".to_string()).into());
        }

        Ok(SimpleBroker::<BMadExecutionUpdate>::subscribe()
            .filter(move |update| {
                let matches = update.execution_id == execution_uuid;
                async move { matches }
            }))
    }

    // Live connection monitoring (for admins)
    async fn live_connections(
        &self,
        ctx: &Context<'_>,
    ) -> Result<impl Stream<Item = ConnectionUpdate>> {
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;
        
        // Only admins can monitor live connections
        if !current_user.is_admin() {
            return Err(GraphQLError::Auth("Admin access required".to_string()).into());
        }

        Ok(async_stream::stream! {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            
            loop {
                interval.tick().await;
                
                if let Ok(connections) = app_ctx.metrics.get_active_connections().await {
                    yield ConnectionUpdate {
                        timestamp: chrono::Utc::now(),
                        active_connections: connections.len() as i32,
                        connections,
                    };
                }
            }
        })
    }

    // Real-time query performance monitoring
    async fn query_performance_updates(
        &self,
        ctx: &Context<'_>,
    ) -> Result<impl Stream<Item = QueryPerformanceUpdate>> {
        let app_ctx = ctx.data::<AppContext>()?;
        let current_user = self.require_auth(ctx).await?;
        
        app_ctx.auth_service.authorize(&current_user, "monitoring", "read").await?;

        Ok(SimpleBroker::<QueryPerformanceUpdate>::subscribe())
    }
}

// Helper methods for SubscriptionRoot
impl SubscriptionRoot {
    async fn require_auth(&self, ctx: &Context<'_>) -> Result<User> {
        if let Some(auth_token) = ctx.data_opt::<crate::AuthToken>() {
            let app_ctx = ctx.data::<AppContext>()?;
            app_ctx.auth_service.authenticate(&auth_token.0).await.map_err(Into::into)
        } else {
            Err(GraphQLError::Auth("Authentication required".to_string()).into())
        }
    }
}

// Helper function to determine alert level from metrics
fn determine_alert_level(metrics: &LiveMetrics) -> AlertLevel {
    if metrics.cpu_usage > 90.0 || metrics.memory_usage > 95.0 {
        AlertLevel::Critical
    } else if metrics.cpu_usage > 80.0 || metrics.memory_usage > 85.0 {
        AlertLevel::Warning
    } else if metrics.error_rate > 0.05 {
        AlertLevel::Warning
    } else {
        AlertLevel::Info
    }
}

// Additional subscription update types
#[derive(Clone)]
pub struct ConnectionUpdate {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub active_connections: i32,
    pub connections: Vec<Connection>,
}

#[derive(Clone)]
pub struct QueryPerformanceUpdate {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub query_hash: String,
    pub execution_time: f64,
    pub complexity_score: i32,
    pub cache_hit: bool,
    pub error: Option<String>,
}

#[derive(Clone, PartialEq)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
}
