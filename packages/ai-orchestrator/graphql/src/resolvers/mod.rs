// GraphQL Resolvers for Free Deep Research System
// Phase 4.4: API Gateway & GraphQL

use async_graphql::{
    Context, Object, Result, Subscription, ID, dataloader::DataLoader,
    subscription::SimpleBroker,
};
use chrono::{DateTime, Utc};
use futures_util::Stream;
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    types::*,
    dataloaders::*,
    AppContext, GraphQLError,
};

pub mod query;
pub mod mutation;
pub mod subscription;
pub mod auth;
pub mod research;
pub mod api_keys;
pub mod monitoring;
pub mod v3_features;

pub use query::QueryRoot;
pub use mutation::MutationRoot;
pub use subscription::SubscriptionRoot;

// Query Root Implementation
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // Authentication & User Management
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let auth_token = ctx.data_opt::<crate::AuthToken>();
        if let Some(token) = auth_token {
            let app_ctx = ctx.data::<AppContext>()?;
            match app_ctx.auth_service.authenticate(&token.0).await {
                Ok(user) => Ok(Some(user)),
                Err(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    async fn users(
        &self,
        ctx: &Context<'_>,
        filter: Option<UserFilter>,
        pagination: Option<PaginationInput>,
    ) -> Result<UserConnection> {
        let app_ctx = ctx.data::<AppContext>()?;
        
        // Check authorization
        if let Some(current_user) = self.get_current_user(ctx).await? {
            app_ctx.auth_service.authorize(&current_user, "users", "read").await?;
        } else {
            return Err(GraphQLError::Auth("Authentication required".to_string()).into());
        }

        let filter = filter.unwrap_or_default();
        let users = app_ctx.database.get_users(filter).await?;
        
        // Apply pagination
        let connection = self.paginate_users(users, pagination).await?;
        Ok(connection)
    }

    async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<Option<User>> {
        let user_id = Uuid::parse_str(&id)?;
        let loader = ctx.data::<DataLoader<UserLoader>>()?;
        loader.load_one(user_id).await.map_err(Into::into)
    }

    // API Key Management
    async fn api_keys(
        &self,
        ctx: &Context<'_>,
        filter: Option<ApiKeyFilter>,
        pagination: Option<PaginationInput>,
    ) -> Result<ApiKeyConnection> {
        let current_user = self.require_auth(ctx).await?;
        let app_ctx = ctx.data::<AppContext>()?;
        
        let filter = filter.unwrap_or_default();
        let api_keys = app_ctx.api_manager.get_user_api_keys(current_user.id, filter).await?;
        
        let connection = self.paginate_api_keys(api_keys, pagination).await?;
        Ok(connection)
    }

    async fn api_key(&self, ctx: &Context<'_>, id: ID) -> Result<Option<ApiKey>> {
        let key_id = Uuid::parse_str(&id)?;
        let current_user = self.require_auth(ctx).await?;
        let loader = ctx.data::<DataLoader<ApiKeyLoader>>()?;
        
        if let Some(api_key) = loader.load_one(key_id).await? {
            // Check ownership
            if api_key.user_id == current_user.id {
                Ok(Some(api_key))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    async fn api_key_usage_stats(
        &self,
        ctx: &Context<'_>,
        key_id: ID,
        time_range: TimeRange,
    ) -> Result<UsageStats> {
        let key_uuid = Uuid::parse_str(&key_id)?;
        let current_user = self.require_auth(ctx).await?;
        let app_ctx = ctx.data::<AppContext>()?;
        
        // Verify ownership
        let api_key = app_ctx.api_manager.get_api_key(key_uuid).await?
            .ok_or_else(|| GraphQLError::Validation("API key not found".to_string()))?;
        
        if api_key.user_id != current_user.id {
            return Err(GraphQLError::Auth("Access denied".to_string()).into());
        }
        
        app_ctx.api_manager.get_usage_stats(key_uuid, time_range).await.map_err(Into::into)
    }

    // Research Workflows
    async fn research_workflows(
        &self,
        ctx: &Context<'_>,
        filter: Option<WorkflowFilter>,
        pagination: Option<PaginationInput>,
    ) -> Result<WorkflowConnection> {
        let current_user = self.require_auth(ctx).await?;
        let app_ctx = ctx.data::<AppContext>()?;
        
        let mut filter = filter.unwrap_or_default();
        // Filter to user's workflows unless admin
        if !current_user.is_admin() {
            filter.created_by = Some(current_user.id);
        }
        
        let workflows = app_ctx.research_engine.get_workflows(filter).await?;
        let connection = self.paginate_workflows(workflows, pagination).await?;
        Ok(connection)
    }

    async fn research_workflow(&self, ctx: &Context<'_>, id: ID) -> Result<Option<ResearchWorkflow>> {
        let workflow_id = Uuid::parse_str(&id)?;
        let current_user = self.require_auth(ctx).await?;
        let loader = ctx.data::<DataLoader<WorkflowLoader>>()?;
        
        if let Some(workflow) = loader.load_one(workflow_id).await? {
            // Check access permissions
            if workflow.creator_id == current_user.id || 
               workflow.collaborators.contains(&current_user.id) ||
               current_user.is_admin() {
                Ok(Some(workflow))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    async fn research_templates(
        &self,
        ctx: &Context<'_>,
        filter: Option<TemplateFilter>,
    ) -> Result<Vec<ResearchTemplate>> {
        let app_ctx = ctx.data::<AppContext>()?;
        let filter = filter.unwrap_or_default();
        app_ctx.research_engine.get_templates(filter).await.map_err(Into::into)
    }

    // System Configuration
    async fn system_config(&self, ctx: &Context<'_>) -> Result<SystemConfiguration> {
        let current_user = self.require_auth(ctx).await?;
        let app_ctx = ctx.data::<AppContext>()?;
        
        // Only admins can view system config
        if !current_user.is_admin() {
            return Err(GraphQLError::Auth("Admin access required".to_string()).into());
        }
        
        app_ctx.database.get_system_config().await.map_err(Into::into)
    }

    async fn user_config(&self, ctx: &Context<'_>, user_id: Option<ID>) -> Result<UserConfiguration> {
        let current_user = self.require_auth(ctx).await?;
        let app_ctx = ctx.data::<AppContext>()?;
        
        let target_user_id = if let Some(id) = user_id {
            let uuid = Uuid::parse_str(&id)?;
            // Check if user can access other user's config
            if uuid != current_user.id && !current_user.is_admin() {
                return Err(GraphQLError::Auth("Access denied".to_string()).into());
            }
            uuid
        } else {
            current_user.id
        };
        
        app_ctx.database.get_user_config(target_user_id).await.map_err(Into::into)
    }

    // Monitoring & Analytics
    async fn system_metrics(&self, ctx: &Context<'_>, time_range: Option<TimeRange>) -> Result<SystemMetrics> {
        let current_user = self.require_auth(ctx).await?;
        let app_ctx = ctx.data::<AppContext>()?;
        
        // Check monitoring permissions
        app_ctx.auth_service.authorize(&current_user, "monitoring", "read").await?;
        
        let range = time_range.unwrap_or_else(|| TimeRange::last_24_hours());
        app_ctx.metrics.get_system_metrics(range).await.map_err(Into::into)
    }

    async fn performance_metrics(
        &self,
        ctx: &Context<'_>,
        service: Option<String>,
        time_range: Option<TimeRange>,
    ) -> Result<PerformanceMetrics> {
        let current_user = self.require_auth(ctx).await?;
        let app_ctx = ctx.data::<AppContext>()?;
        
        app_ctx.auth_service.authorize(&current_user, "monitoring", "read").await?;
        
        let range = time_range.unwrap_or_else(|| TimeRange::last_24_hours());
        app_ctx.metrics.get_performance_metrics(service, range).await.map_err(Into::into)
    }

    // V3.0.0 Features
    async fn federated_research(
        &self,
        ctx: &Context<'_>,
        filter: Option<FederatedFilter>,
    ) -> Result<Vec<FederatedResearchNode>> {
        let current_user = self.require_auth(ctx).await?;
        let app_ctx = ctx.data::<AppContext>()?;
        
        let filter = filter.unwrap_or_default();
        app_ctx.federated_service.get_research_nodes(current_user.id, filter).await.map_err(Into::into)
    }

    async fn ai_marketplace(
        &self,
        ctx: &Context<'_>,
        category: Option<MarketplaceCategory>,
    ) -> Result<Vec<MarketplaceItem>> {
        let app_ctx = ctx.data::<AppContext>()?;
        app_ctx.marketplace_service.get_items(category).await.map_err(Into::into)
    }

    async fn knowledge_graph(&self, ctx: &Context<'_>, graph_id: ID) -> Result<Option<KnowledgeGraph>> {
        let graph_uuid = Uuid::parse_str(&graph_id)?;
        let current_user = self.require_auth(ctx).await?;
        let app_ctx = ctx.data::<AppContext>()?;
        
        app_ctx.knowledge_service.get_graph(graph_uuid, current_user.id).await.map_err(Into::into)
    }

    // BMAD Integration
    async fn bmad_agents(&self, ctx: &Context<'_>) -> Result<Vec<BMadAgent>> {
        let app_ctx = ctx.data::<AppContext>()?;
        app_ctx.bmad_service.get_agents().await.map_err(Into::into)
    }

    async fn bmad_workflows(&self, ctx: &Context<'_>, agent_id: Option<String>) -> Result<Vec<BMadWorkflow>> {
        let app_ctx = ctx.data::<AppContext>()?;
        app_ctx.bmad_service.get_workflows(agent_id).await.map_err(Into::into)
    }

    // Real-time Data
    async fn live_metrics(&self, ctx: &Context<'_>) -> Result<LiveMetrics> {
        let current_user = self.require_auth(ctx).await?;
        let app_ctx = ctx.data::<AppContext>()?;
        
        app_ctx.auth_service.authorize(&current_user, "monitoring", "read").await?;
        app_ctx.metrics.get_live_metrics().await.map_err(Into::into)
    }
}

// Helper methods for QueryRoot
impl QueryRoot {
    async fn get_current_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        if let Some(auth_token) = ctx.data_opt::<crate::AuthToken>() {
            let app_ctx = ctx.data::<AppContext>()?;
            match app_ctx.auth_service.authenticate(&auth_token.0).await {
                Ok(user) => Ok(Some(user)),
                Err(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    async fn require_auth(&self, ctx: &Context<'_>) -> Result<User> {
        self.get_current_user(ctx).await?
            .ok_or_else(|| GraphQLError::Auth("Authentication required".to_string()).into())
    }

    async fn paginate_users(&self, users: Vec<User>, pagination: Option<PaginationInput>) -> Result<UserConnection> {
        // Implement pagination logic
        let total_count = users.len();
        let edges: Vec<UserEdge> = users.into_iter()
            .enumerate()
            .map(|(i, user)| UserEdge {
                node: user,
                cursor: base64::encode(i.to_string()),
            })
            .collect();

        Ok(UserConnection {
            edges,
            page_info: PageInfo {
                has_next_page: false, // TODO: Implement proper pagination
                has_previous_page: false,
                start_cursor: None,
                end_cursor: None,
            },
            total_count: total_count as i32,
        })
    }

    async fn paginate_api_keys(&self, api_keys: Vec<ApiKey>, pagination: Option<PaginationInput>) -> Result<ApiKeyConnection> {
        // Similar pagination implementation for API keys
        let total_count = api_keys.len();
        let edges: Vec<ApiKeyEdge> = api_keys.into_iter()
            .enumerate()
            .map(|(i, api_key)| ApiKeyEdge {
                node: api_key,
                cursor: base64::encode(i.to_string()),
            })
            .collect();

        Ok(ApiKeyConnection {
            edges,
            page_info: PageInfo {
                has_next_page: false,
                has_previous_page: false,
                start_cursor: None,
                end_cursor: None,
            },
            total_count: total_count as i32,
        })
    }

    async fn paginate_workflows(&self, workflows: Vec<ResearchWorkflow>, pagination: Option<PaginationInput>) -> Result<WorkflowConnection> {
        // Similar pagination implementation for workflows
        let total_count = workflows.len();
        let edges: Vec<WorkflowEdge> = workflows.into_iter()
            .enumerate()
            .map(|(i, workflow)| WorkflowEdge {
                node: workflow,
                cursor: base64::encode(i.to_string()),
            })
            .collect();

        Ok(WorkflowConnection {
            edges,
            page_info: PageInfo {
                has_next_page: false,
                has_previous_page: false,
                start_cursor: None,
                end_cursor: None,
            },
            total_count: total_count as i32,
        })
    }
}
