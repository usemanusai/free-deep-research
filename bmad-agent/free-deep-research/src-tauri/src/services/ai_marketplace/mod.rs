use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{info, debug, warn, error};
use uuid::Uuid;
use chrono::Utc;

use crate::error::{AppResult, ResearchError};
use crate::services::{Service, DataPersistenceService, SecurityService};
use crate::models::ai_marketplace::*;

pub mod user_manager;
pub mod agent_manager;
pub mod methodology_manager;
pub mod rating_system;
pub mod search_engine;
pub mod installation_manager;
pub mod analytics_tracker;

use user_manager::UserManager;
use agent_manager::AgentManager;
use methodology_manager::MethodologyManager;
use rating_system::RatingSystem;
use search_engine::MarketplaceSearchEngine;
use installation_manager::InstallationManager;
use analytics_tracker::AnalyticsTracker;

/// AI Marketplace Service for community platform and agent sharing
pub struct AIMarketplaceService {
    data_persistence: Arc<RwLock<DataPersistenceService>>,
    security: Arc<RwLock<SecurityService>>,
    user_manager: Arc<RwLock<UserManager>>,
    agent_manager: Arc<RwLock<AgentManager>>,
    methodology_manager: Arc<RwLock<MethodologyManager>>,
    rating_system: Arc<RwLock<RatingSystem>>,
    search_engine: Arc<RwLock<MarketplaceSearchEngine>>,
    installation_manager: Arc<RwLock<InstallationManager>>,
    analytics_tracker: Arc<RwLock<AnalyticsTracker>>,
}

impl AIMarketplaceService {
    /// Create a new AI marketplace service
    pub async fn new(
        data_persistence: Arc<RwLock<DataPersistenceService>>,
        security: Arc<RwLock<SecurityService>>,
    ) -> AppResult<Self> {
        info!("Initializing AI Marketplace Service");

        let user_manager = Arc::new(RwLock::new(
            UserManager::new(data_persistence.clone(), security.clone()).await?
        ));

        let agent_manager = Arc::new(RwLock::new(
            AgentManager::new(data_persistence.clone(), user_manager.clone()).await?
        ));

        let methodology_manager = Arc::new(RwLock::new(
            MethodologyManager::new(data_persistence.clone(), user_manager.clone()).await?
        ));

        let rating_system = Arc::new(RwLock::new(
            RatingSystem::new(data_persistence.clone()).await?
        ));

        let search_engine = Arc::new(RwLock::new(
            MarketplaceSearchEngine::new(
                agent_manager.clone(),
                methodology_manager.clone(),
                rating_system.clone(),
            ).await?
        ));

        let installation_manager = Arc::new(RwLock::new(
            InstallationManager::new(security.clone()).await?
        ));

        let analytics_tracker = Arc::new(RwLock::new(
            AnalyticsTracker::new(data_persistence.clone()).await?
        ));

        Ok(Self {
            data_persistence,
            security,
            user_manager,
            agent_manager,
            methodology_manager,
            rating_system,
            search_engine,
            installation_manager,
            analytics_tracker,
        })
    }

    /// Register a new marketplace user
    pub async fn register_user(
        &self,
        username: String,
        email: String,
        display_name: String,
    ) -> AppResult<MarketplaceUser> {
        info!("Registering new marketplace user: {}", username);

        let user_manager = self.user_manager.write().await;
        let user = user_manager.create_user(username, email, display_name).await?;
        
        // Track registration analytics
        let analytics_tracker = self.analytics_tracker.write().await;
        analytics_tracker.track_user_registration(user.id).await?;
        
        info!("Successfully registered user: {}", user.id);
        Ok(user)
    }

    /// Publish an AI agent to the marketplace
    pub async fn publish_agent(
        &self,
        creator_id: Uuid,
        agent: AIAgentMarketplace,
    ) -> AppResult<AIAgentMarketplace> {
        info!("Publishing AI agent: {} by user: {}", agent.name, creator_id);

        // Validate user permissions
        let user_manager = self.user_manager.read().await;
        let user = user_manager.get_user(creator_id).await?;
        
        if !user.verified {
            return Err(ResearchError::Unauthorized {
                message: "Only verified users can publish agents".to_string(),
            }.into());
        }

        let agent_manager = self.agent_manager.write().await;
        let published_agent = agent_manager.publish_agent(creator_id, agent).await?;
        
        // Track publication analytics
        let analytics_tracker = self.analytics_tracker.write().await;
        analytics_tracker.track_agent_publication(published_agent.id, creator_id).await?;
        
        info!("Successfully published agent: {}", published_agent.id);
        Ok(published_agent)
    }

    /// Publish a research methodology to the marketplace
    pub async fn publish_methodology(
        &self,
        creator_id: Uuid,
        methodology: ResearchMethodologyMarketplace,
    ) -> AppResult<ResearchMethodologyMarketplace> {
        info!("Publishing research methodology: {} by user: {}", methodology.name, creator_id);

        let methodology_manager = self.methodology_manager.write().await;
        let published_methodology = methodology_manager
            .publish_methodology(creator_id, methodology)
            .await?;
        
        // Track publication analytics
        let analytics_tracker = self.analytics_tracker.write().await;
        analytics_tracker
            .track_methodology_publication(published_methodology.id, creator_id)
            .await?;
        
        info!("Successfully published methodology: {}", published_methodology.id);
        Ok(published_methodology)
    }

    /// Search the marketplace
    pub async fn search_marketplace(
        &self,
        query: MarketplaceSearchQuery,
    ) -> AppResult<MarketplaceSearchResult> {
        debug!("Searching marketplace with query: {}", query.query);

        let search_engine = self.search_engine.read().await;
        let results = search_engine.search(query).await?;
        
        // Track search analytics
        let analytics_tracker = self.analytics_tracker.write().await;
        analytics_tracker.track_search_query(&results).await?;
        
        debug!("Found {} results", results.total_count);
        Ok(results)
    }

    /// Install an AI agent
    pub async fn install_agent(
        &self,
        user_id: Uuid,
        request: AgentInstallationRequest,
    ) -> AppResult<AgentInstallationResult> {
        info!("Installing agent: {} for user: {}", request.agent_id, user_id);

        // Get agent details
        let agent_manager = self.agent_manager.read().await;
        let agent = agent_manager.get_agent(request.agent_id).await?;
        
        if agent.status != AgentStatus::Published {
            return Err(ResearchError::InvalidInput {
                message: "Agent is not available for installation".to_string(),
            }.into());
        }

        // Perform installation
        let installation_manager = self.installation_manager.write().await;
        let result = installation_manager.install_agent(user_id, agent, request).await?;
        
        if result.success {
            // Update download count
            let mut agent_manager = self.agent_manager.write().await;
            agent_manager.increment_download_count(agent.id).await?;
            
            // Track installation analytics
            let analytics_tracker = self.analytics_tracker.write().await;
            analytics_tracker.track_agent_installation(agent.id, user_id).await?;
        }
        
        info!("Agent installation completed with success: {}", result.success);
        Ok(result)
    }

    /// Submit a rating/review
    pub async fn submit_rating(
        &self,
        user_id: Uuid,
        rating: CommunityRating,
    ) -> AppResult<CommunityRating> {
        info!("Submitting rating for {:?}: {} by user: {}", 
               rating.target_type, rating.target_id, user_id);

        let rating_system = self.rating_system.write().await;
        let submitted_rating = rating_system.submit_rating(user_id, rating).await?;
        
        // Update aggregate ratings
        match submitted_rating.target_type {
            RatingTargetType::Agent => {
                let agent_manager = self.agent_manager.write().await;
                agent_manager.update_agent_rating(submitted_rating.target_id).await?;
            }
            RatingTargetType::Methodology => {
                let methodology_manager = self.methodology_manager.write().await;
                methodology_manager.update_methodology_rating(submitted_rating.target_id).await?;
            }
            RatingTargetType::User => {
                let user_manager = self.user_manager.write().await;
                user_manager.update_user_reputation(submitted_rating.target_id).await?;
            }
        }
        
        // Track rating analytics
        let analytics_tracker = self.analytics_tracker.write().await;
        analytics_tracker.track_rating_submission(submitted_rating.id, user_id).await?;
        
        info!("Successfully submitted rating: {}", submitted_rating.id);
        Ok(submitted_rating)
    }

    /// Get user analytics
    pub async fn get_user_analytics(
        &self,
        user_id: Uuid,
    ) -> AppResult<MarketplaceAnalytics> {
        debug!("Getting analytics for user: {}", user_id);

        let analytics_tracker = self.analytics_tracker.read().await;
        let analytics = analytics_tracker.get_user_analytics(user_id).await?;
        
        Ok(analytics)
    }

    /// Get featured agents
    pub async fn get_featured_agents(&self) -> AppResult<Vec<AIAgentMarketplace>> {
        debug!("Getting featured agents");

        let agent_manager = self.agent_manager.read().await;
        let featured_agents = agent_manager.get_featured_agents().await?;
        
        Ok(featured_agents)
    }

    /// Get trending methodologies
    pub async fn get_trending_methodologies(&self) -> AppResult<Vec<ResearchMethodologyMarketplace>> {
        debug!("Getting trending methodologies");

        let methodology_manager = self.methodology_manager.read().await;
        let trending_methodologies = methodology_manager.get_trending_methodologies().await?;
        
        Ok(trending_methodologies)
    }

    /// Get user's published content
    pub async fn get_user_content(
        &self,
        user_id: Uuid,
    ) -> AppResult<(Vec<AIAgentMarketplace>, Vec<ResearchMethodologyMarketplace>)> {
        debug!("Getting content for user: {}", user_id);

        let agent_manager = self.agent_manager.read().await;
        let user_agents = agent_manager.get_user_agents(user_id).await?;

        let methodology_manager = self.methodology_manager.read().await;
        let user_methodologies = methodology_manager.get_user_methodologies(user_id).await?;
        
        Ok((user_agents, user_methodologies))
    }

    /// Moderate content (admin function)
    pub async fn moderate_content(
        &self,
        moderator_id: Uuid,
        target_type: RatingTargetType,
        target_id: Uuid,
        action: ModerationAction,
    ) -> AppResult<()> {
        info!("Moderating {:?}: {} by moderator: {}", target_type, target_id, moderator_id);

        // Verify moderator permissions
        let user_manager = self.user_manager.read().await;
        let moderator = user_manager.get_user(moderator_id).await?;
        
        if !moderator.verified {
            return Err(ResearchError::Unauthorized {
                message: "Only verified users can moderate content".to_string(),
            }.into());
        }

        match target_type {
            RatingTargetType::Agent => {
                let agent_manager = self.agent_manager.write().await;
                agent_manager.moderate_agent(target_id, action).await?;
            }
            RatingTargetType::Methodology => {
                let methodology_manager = self.methodology_manager.write().await;
                methodology_manager.moderate_methodology(target_id, action).await?;
            }
            RatingTargetType::User => {
                let user_manager = self.user_manager.write().await;
                user_manager.moderate_user(target_id, action).await?;
            }
        }
        
        info!("Content moderation completed");
        Ok(())
    }

    /// Start background tasks
    pub async fn start_background_tasks(&self) -> AppResult<()> {
        info!("Starting AI marketplace background tasks...");

        // Start analytics aggregation
        let analytics_tracker = self.analytics_tracker.read().await;
        analytics_tracker.start_background_aggregation().await?;

        // Start content quality monitoring
        let agent_manager = self.agent_manager.read().await;
        agent_manager.start_quality_monitoring().await?;

        let methodology_manager = self.methodology_manager.read().await;
        methodology_manager.start_quality_monitoring().await?;

        info!("AI marketplace background tasks started successfully");
        Ok(())
    }
}

/// Moderation action
#[derive(Debug, Clone)]
pub enum ModerationAction {
    Approve,
    Reject,
    Suspend,
    Remove,
    Flag,
    Unflag,
}

#[async_trait::async_trait]
impl Service for AIMarketplaceService {
    async fn health_check(&self) -> AppResult<()> {
        debug!("Performing AI marketplace service health check");
        
        // Check all sub-managers
        let user_manager = self.user_manager.read().await;
        user_manager.health_check().await?;

        let agent_manager = self.agent_manager.read().await;
        agent_manager.health_check().await?;

        let methodology_manager = self.methodology_manager.read().await;
        methodology_manager.health_check().await?;

        let rating_system = self.rating_system.read().await;
        rating_system.health_check().await?;

        Ok(())
    }
    
    async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down AI marketplace service...");
        
        // Shutdown all sub-managers
        let user_manager = self.user_manager.read().await;
        user_manager.shutdown().await?;

        let agent_manager = self.agent_manager.read().await;
        agent_manager.shutdown().await?;

        let methodology_manager = self.methodology_manager.read().await;
        methodology_manager.shutdown().await?;

        let rating_system = self.rating_system.read().await;
        rating_system.shutdown().await?;

        let search_engine = self.search_engine.read().await;
        search_engine.shutdown().await?;

        let installation_manager = self.installation_manager.read().await;
        installation_manager.shutdown().await?;

        let analytics_tracker = self.analytics_tracker.read().await;
        analytics_tracker.shutdown().await?;

        info!("AI marketplace service shutdown complete");
        Ok(())
    }
}
