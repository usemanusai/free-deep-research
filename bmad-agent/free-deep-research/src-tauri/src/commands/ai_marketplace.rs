use tauri::State;
use uuid::Uuid;
use tracing::{info, debug, error};

use crate::error::{AppResult, ResearchError};
use crate::services::ServiceManager;
use crate::models::ai_marketplace::*;

/// Register a new marketplace user
#[tauri::command]
pub async fn register_marketplace_user(
    service_manager: State<'_, ServiceManager>,
    username: String,
    email: String,
    display_name: String,
) -> Result<MarketplaceUser, String> {
    info!("API: Registering marketplace user: {}", username);
    
    match service_manager.ai_marketplace_service.register_user(username, email, display_name).await {
        Ok(user) => {
            info!("Successfully registered user: {}", user.id);
            Ok(user)
        }
        Err(e) => {
            error!("Failed to register user: {}", e);
            Err(e.to_string())
        }
    }
}

/// Publish an AI agent to the marketplace
#[tauri::command]
pub async fn publish_ai_agent(
    service_manager: State<'_, ServiceManager>,
    creator_id: String,
    agent: AIAgentMarketplace,
) -> Result<AIAgentMarketplace, String> {
    info!("API: Publishing AI agent: {}", agent.name);
    
    let user_id = Uuid::parse_str(&creator_id)
        .map_err(|e| format!("Invalid creator ID: {}", e))?;
    
    match service_manager.ai_marketplace_service.publish_agent(user_id, agent).await {
        Ok(published_agent) => {
            info!("Successfully published agent: {}", published_agent.id);
            Ok(published_agent)
        }
        Err(e) => {
            error!("Failed to publish agent: {}", e);
            Err(e.to_string())
        }
    }
}

/// Publish a research methodology to the marketplace
#[tauri::command]
pub async fn publish_research_methodology(
    service_manager: State<'_, ServiceManager>,
    creator_id: String,
    methodology: ResearchMethodologyMarketplace,
) -> Result<ResearchMethodologyMarketplace, String> {
    info!("API: Publishing research methodology: {}", methodology.name);
    
    let user_id = Uuid::parse_str(&creator_id)
        .map_err(|e| format!("Invalid creator ID: {}", e))?;
    
    match service_manager.ai_marketplace_service.publish_methodology(user_id, methodology).await {
        Ok(published_methodology) => {
            info!("Successfully published methodology: {}", published_methodology.id);
            Ok(published_methodology)
        }
        Err(e) => {
            error!("Failed to publish methodology: {}", e);
            Err(e.to_string())
        }
    }
}

/// Search the marketplace
#[tauri::command]
pub async fn search_marketplace(
    service_manager: State<'_, ServiceManager>,
    query: MarketplaceSearchQuery,
) -> Result<MarketplaceSearchResult, String> {
    debug!("API: Searching marketplace with query: {}", query.query);
    
    match service_manager.ai_marketplace_service.search_marketplace(query).await {
        Ok(results) => {
            debug!("Found {} results", results.total_count);
            Ok(results)
        }
        Err(e) => {
            error!("Failed to search marketplace: {}", e);
            Err(e.to_string())
        }
    }
}

/// Install an AI agent
#[tauri::command]
pub async fn install_ai_agent(
    service_manager: State<'_, ServiceManager>,
    user_id: String,
    request: AgentInstallationRequest,
) -> Result<AgentInstallationResult, String> {
    info!("API: Installing agent: {} for user: {}", request.agent_id, user_id);
    
    let uid = Uuid::parse_str(&user_id)
        .map_err(|e| format!("Invalid user ID: {}", e))?;
    
    match service_manager.ai_marketplace_service.install_agent(uid, request).await {
        Ok(result) => {
            info!("Agent installation completed with success: {}", result.success);
            Ok(result)
        }
        Err(e) => {
            error!("Failed to install agent: {}", e);
            Err(e.to_string())
        }
    }
}

/// Submit a rating/review
#[tauri::command]
pub async fn submit_community_rating(
    service_manager: State<'_, ServiceManager>,
    user_id: String,
    rating: CommunityRating,
) -> Result<CommunityRating, String> {
    info!("API: Submitting rating for {:?}: {} by user: {}", 
           rating.target_type, rating.target_id, user_id);
    
    let uid = Uuid::parse_str(&user_id)
        .map_err(|e| format!("Invalid user ID: {}", e))?;
    
    match service_manager.ai_marketplace_service.submit_rating(uid, rating).await {
        Ok(submitted_rating) => {
            info!("Successfully submitted rating: {}", submitted_rating.id);
            Ok(submitted_rating)
        }
        Err(e) => {
            error!("Failed to submit rating: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get user analytics
#[tauri::command]
pub async fn get_marketplace_user_analytics(
    service_manager: State<'_, ServiceManager>,
    user_id: String,
) -> Result<MarketplaceAnalytics, String> {
    debug!("API: Getting analytics for user: {}", user_id);
    
    let uid = Uuid::parse_str(&user_id)
        .map_err(|e| format!("Invalid user ID: {}", e))?;
    
    match service_manager.ai_marketplace_service.get_user_analytics(uid).await {
        Ok(analytics) => Ok(analytics),
        Err(e) => {
            error!("Failed to get user analytics: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get featured agents
#[tauri::command]
pub async fn get_featured_agents(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<AIAgentMarketplace>, String> {
    debug!("API: Getting featured agents");
    
    match service_manager.ai_marketplace_service.get_featured_agents().await {
        Ok(featured_agents) => Ok(featured_agents),
        Err(e) => {
            error!("Failed to get featured agents: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get trending methodologies
#[tauri::command]
pub async fn get_trending_methodologies(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<ResearchMethodologyMarketplace>, String> {
    debug!("API: Getting trending methodologies");
    
    match service_manager.ai_marketplace_service.get_trending_methodologies().await {
        Ok(trending_methodologies) => Ok(trending_methodologies),
        Err(e) => {
            error!("Failed to get trending methodologies: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get user's published content
#[tauri::command]
pub async fn get_user_marketplace_content(
    service_manager: State<'_, ServiceManager>,
    user_id: String,
) -> Result<UserMarketplaceContent, String> {
    debug!("API: Getting content for user: {}", user_id);
    
    let uid = Uuid::parse_str(&user_id)
        .map_err(|e| format!("Invalid user ID: {}", e))?;
    
    match service_manager.ai_marketplace_service.get_user_content(uid).await {
        Ok((agents, methodologies)) => Ok(UserMarketplaceContent {
            agents,
            methodologies,
        }),
        Err(e) => {
            error!("Failed to get user content: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get marketplace statistics
#[tauri::command]
pub async fn get_marketplace_statistics(
    service_manager: State<'_, ServiceManager>,
) -> Result<MarketplaceStatistics, String> {
    debug!("API: Getting marketplace statistics");
    
    // This would aggregate statistics across the marketplace
    // For now, return a mock response
    Ok(MarketplaceStatistics {
        total_users: 0,
        total_agents: 0,
        total_methodologies: 0,
        total_downloads: 0,
        average_rating: 0.0,
        active_users_24h: 0,
        new_content_24h: 0,
        last_updated: chrono::Utc::now(),
    })
}

/// User marketplace content
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserMarketplaceContent {
    pub agents: Vec<AIAgentMarketplace>,
    pub methodologies: Vec<ResearchMethodologyMarketplace>,
}

/// Marketplace statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MarketplaceStatistics {
    pub total_users: u32,
    pub total_agents: u32,
    pub total_methodologies: u32,
    pub total_downloads: u32,
    pub average_rating: f64,
    pub active_users_24h: u32,
    pub new_content_24h: u32,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Get agent categories
#[tauri::command]
pub async fn get_agent_categories(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<AgentCategoryInfo>, String> {
    debug!("API: Getting agent categories");
    
    // Return available agent categories with counts
    Ok(vec![
        AgentCategoryInfo { category: AgentCategory::Research, count: 0, description: "Research-focused AI agents".to_string() },
        AgentCategoryInfo { category: AgentCategory::Analysis, count: 0, description: "Data analysis and processing agents".to_string() },
        AgentCategoryInfo { category: AgentCategory::Automation, count: 0, description: "Workflow automation agents".to_string() },
        AgentCategoryInfo { category: AgentCategory::DataProcessing, count: 0, description: "Data processing and transformation agents".to_string() },
        AgentCategoryInfo { category: AgentCategory::Visualization, count: 0, description: "Data visualization and reporting agents".to_string() },
        AgentCategoryInfo { category: AgentCategory::NaturalLanguage, count: 0, description: "Natural language processing agents".to_string() },
        AgentCategoryInfo { category: AgentCategory::MachineLearning, count: 0, description: "Machine learning and AI model agents".to_string() },
        AgentCategoryInfo { category: AgentCategory::Integration, count: 0, description: "System integration and API agents".to_string() },
        AgentCategoryInfo { category: AgentCategory::Security, count: 0, description: "Security and privacy-focused agents".to_string() },
        AgentCategoryInfo { category: AgentCategory::Custom, count: 0, description: "Custom and specialized agents".to_string() },
    ])
}

/// Agent category information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AgentCategoryInfo {
    pub category: AgentCategory,
    pub count: u32,
    pub description: String,
}
