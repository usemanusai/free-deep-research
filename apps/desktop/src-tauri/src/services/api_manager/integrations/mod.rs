pub mod openrouter;
pub mod serpapi;
pub mod jina;
pub mod firecrawl;
pub mod tavily;
pub mod exa;

pub use openrouter::OpenRouterIntegration;
pub use serpapi::SerpApiIntegration;
pub use jina::JinaIntegration;
pub use firecrawl::FirecrawlIntegration;
pub use tavily::TavilyIntegration;
pub use exa::ExaIntegration;

use crate::error::AppResult;
use crate::models::api_key::ServiceProvider;
use crate::services::api_manager::service_integration::{ServiceIntegration, ServiceIntegrationManager};

/// Create all service integrations and register them
pub async fn create_all_integrations() -> AppResult<ServiceIntegrationManager> {
    let mut manager = ServiceIntegrationManager::new().await?;

    // Register OpenRouter integration
    let openrouter = Box::new(OpenRouterIntegration::new());
    manager.register_integration(openrouter).await?;

    // Register SerpApi integration
    let serpapi = Box::new(SerpApiIntegration::new());
    manager.register_integration(serpapi).await?;

    // Register Jina integration
    let jina = Box::new(JinaIntegration::new());
    manager.register_integration(jina).await?;

    // Register Firecrawl integration
    let firecrawl = Box::new(FirecrawlIntegration::new());
    manager.register_integration(firecrawl).await?;

    // Register Tavily integration
    let tavily = Box::new(TavilyIntegration::new());
    manager.register_integration(tavily).await?;

    // Register Exa integration
    let exa = Box::new(ExaIntegration::new());
    manager.register_integration(exa).await?;

    Ok(manager)
}

/// Get integration factory for a service
pub fn create_integration_for_service(service: ServiceProvider) -> Box<dyn ServiceIntegration> {
    match service {
        ServiceProvider::OpenRouter => Box::new(OpenRouterIntegration::new()),
        ServiceProvider::SerpApi => Box::new(SerpApiIntegration::new()),
        ServiceProvider::Jina => Box::new(JinaIntegration::new()),
        ServiceProvider::Firecrawl => Box::new(FirecrawlIntegration::new()),
        ServiceProvider::Tavily => Box::new(TavilyIntegration::new()),
        ServiceProvider::Exa => Box::new(ExaIntegration::new()),
    }
}
