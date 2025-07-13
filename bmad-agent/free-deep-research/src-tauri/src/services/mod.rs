use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};

pub mod api_manager;
pub mod research_engine;
pub mod template_manager;
pub mod data_persistence;
pub mod monitoring;
pub mod security;
pub mod output_processor;
pub mod analytics;

use crate::error::{AppError, AppResult};
use api_manager::ApiManagerService;
use research_engine::ResearchEngineService;
use template_manager::TemplateManagerService;
use data_persistence::DataPersistenceService;
use monitoring::MonitoringService;
use security::SecurityService;
use output_processor::OutputProcessorService;
use analytics::AnalyticsService;

/// Central service manager that coordinates all application services
#[derive(Clone)]
pub struct ServiceManager {
    pub api_manager: Arc<RwLock<ApiManagerService>>,
    pub research_engine: Arc<RwLock<ResearchEngineService>>,
    pub template_manager: Arc<RwLock<TemplateManagerService>>,
    pub data_persistence: Arc<RwLock<DataPersistenceService>>,
    pub monitoring: Arc<RwLock<MonitoringService>>,
    pub security: Arc<RwLock<SecurityService>>,
    pub output_processor: Arc<RwLock<OutputProcessorService>>,
    pub analytics: Arc<RwLock<AnalyticsService>>,
}

impl ServiceManager {
    /// Create a new service manager and initialize all services
    pub async fn new() -> AppResult<Self> {
        info!("Initializing service manager...");
        
        // Initialize security service first (required by others)
        let security = SecurityService::new().await?;
        let security = Arc::new(RwLock::new(security));
        
        // Initialize data persistence service
        let data_persistence = DataPersistenceService::new(security.clone()).await?;
        let data_persistence = Arc::new(RwLock::new(data_persistence));
        
        // Initialize monitoring service
        let monitoring = MonitoringService::new().await?;
        let monitoring = Arc::new(RwLock::new(monitoring));
        
        // Initialize API manager service
        let api_manager = ApiManagerService::new(
            data_persistence.clone(),
            security.clone(),
            monitoring.clone(),
        ).await?;
        let api_manager = Arc::new(RwLock::new(api_manager));
        
        // Initialize research engine service
        let research_engine = ResearchEngineService::new(
            api_manager.clone(),
            data_persistence.clone(),
            monitoring.clone(),
        ).await?;
        let research_engine = Arc::new(RwLock::new(research_engine));

        // Initialize template manager service
        let template_manager = TemplateManagerService::new(
            data_persistence.clone(),
            research_engine.clone(),
        ).await?;
        let template_manager = Arc::new(RwLock::new(template_manager));

        // Initialize output processor service
        let output_processor = OutputProcessorService::new().await?;
        let output_processor = Arc::new(RwLock::new(output_processor));

        // Initialize analytics service
        let analytics = AnalyticsService::new(
            data_persistence.clone(),
            monitoring.clone(),
        ).await?;
        let analytics = Arc::new(RwLock::new(analytics));

        let service_manager = Self {
            api_manager,
            research_engine,
            template_manager,
            data_persistence,
            monitoring,
            security,
            output_processor,
            analytics,
        };
        
        // Start background services
        service_manager.start_background_services().await?;
        
        info!("Service manager initialized successfully");
        Ok(service_manager)
    }
    
    /// Start background services and monitoring
    async fn start_background_services(&self) -> AppResult<()> {
        info!("Starting background services...");
        
        // Start monitoring service
        {
            let monitoring = self.monitoring.read().await;
            monitoring.start_monitoring().await?;
        }
        
        // Start API manager background tasks
        {
            let api_manager = self.api_manager.read().await;
            api_manager.start_background_tasks().await?;
        }
        
        // Start data persistence background tasks (backups, cleanup)
        {
            let data_persistence = self.data_persistence.read().await;
            data_persistence.start_background_tasks().await?;
        }

        // Start template manager background monitoring
        {
            let template_manager = self.template_manager.read().await;
            template_manager.start_background_monitoring().await?;
        }

        // Start analytics service processing
        {
            let analytics = self.analytics.read().await;
            analytics.start_analytics_processing().await?;
        }

        info!("Background services started successfully");
        Ok(())
    }
    
    /// Perform health check on all services
    pub async fn health_check(&self) -> AppResult<ServiceHealthStatus> {
        let mut status = ServiceHealthStatus::default();
        
        // Check security service
        match self.security.read().await.health_check().await {
            Ok(_) => status.security = ServiceStatus::Healthy,
            Err(e) => {
                error!("Security service health check failed: {}", e);
                status.security = ServiceStatus::Unhealthy;
            }
        }
        
        // Check data persistence service
        match self.data_persistence.read().await.health_check().await {
            Ok(_) => status.data_persistence = ServiceStatus::Healthy,
            Err(e) => {
                error!("Data persistence service health check failed: {}", e);
                status.data_persistence = ServiceStatus::Unhealthy;
            }
        }
        
        // Check monitoring service
        match self.monitoring.read().await.health_check().await {
            Ok(_) => status.monitoring = ServiceStatus::Healthy,
            Err(e) => {
                error!("Monitoring service health check failed: {}", e);
                status.monitoring = ServiceStatus::Unhealthy;
            }
        }
        
        // Check API manager service
        match self.api_manager.read().await.health_check().await {
            Ok(_) => status.api_manager = ServiceStatus::Healthy,
            Err(e) => {
                error!("API manager service health check failed: {}", e);
                status.api_manager = ServiceStatus::Unhealthy;
            }
        }
        
        // Check research engine service
        match self.research_engine.read().await.health_check().await {
            Ok(_) => status.research_engine = ServiceStatus::Healthy,
            Err(e) => {
                error!("Research engine service health check failed: {}", e);
                status.research_engine = ServiceStatus::Unhealthy;
            }
        }

        // Check output processor service
        match self.output_processor.read().await.health_check().await {
            Ok(_) => status.output_processor = ServiceStatus::Healthy,
            Err(e) => {
                error!("Output processor service health check failed: {}", e);
                status.output_processor = ServiceStatus::Unhealthy;
            }
        }

        // Check analytics service
        match self.analytics.read().await.health_check().await {
            Ok(_) => status.analytics = ServiceStatus::Healthy,
            Err(e) => {
                error!("Analytics service health check failed: {}", e);
                status.analytics = ServiceStatus::Unhealthy;
            }
        }

        Ok(status)
    }
    
    /// Gracefully shutdown all services
    pub async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down service manager...");
        
        // Stop research engine first
        {
            let research_engine = self.research_engine.write().await;
            research_engine.shutdown().await?;
        }

        // Stop output processor
        {
            let output_processor = self.output_processor.write().await;
            output_processor.shutdown().await?;
        }

        // Stop analytics service
        {
            let analytics = self.analytics.write().await;
            analytics.shutdown().await?;
        }

        // Stop API manager
        {
            let api_manager = self.api_manager.write().await;
            api_manager.shutdown().await?;
        }
        
        // Stop monitoring
        {
            let monitoring = self.monitoring.write().await;
            monitoring.shutdown().await?;
        }
        
        // Stop data persistence (ensure all data is saved)
        {
            let data_persistence = self.data_persistence.write().await;
            data_persistence.shutdown().await?;
        }
        
        // Stop security service last
        {
            let security = self.security.write().await;
            security.shutdown().await?;
        }
        
        info!("Service manager shutdown complete");
        Ok(())
    }
}

/// Service health status
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceHealthStatus {
    pub security: ServiceStatus,
    pub data_persistence: ServiceStatus,
    pub monitoring: ServiceStatus,
    pub api_manager: ServiceStatus,
    pub research_engine: ServiceStatus,
    pub output_processor: ServiceStatus,
    pub analytics: ServiceStatus,
}

impl Default for ServiceHealthStatus {
    fn default() -> Self {
        Self {
            security: ServiceStatus::Unknown,
            data_persistence: ServiceStatus::Unknown,
            monitoring: ServiceStatus::Unknown,
            api_manager: ServiceStatus::Unknown,
            research_engine: ServiceStatus::Unknown,
            output_processor: ServiceStatus::Unknown,
            analytics: ServiceStatus::Unknown,
        }
    }
}

impl ServiceHealthStatus {
    /// Check if all services are healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self.security, ServiceStatus::Healthy)
            && matches!(self.data_persistence, ServiceStatus::Healthy)
            && matches!(self.monitoring, ServiceStatus::Healthy)
            && matches!(self.api_manager, ServiceStatus::Healthy)
            && matches!(self.research_engine, ServiceStatus::Healthy)
            && matches!(self.output_processor, ServiceStatus::Healthy)
            && matches!(self.analytics, ServiceStatus::Healthy)
    }
}

/// Individual service status
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ServiceStatus {
    Healthy,
    Unhealthy,
    Unknown,
}

/// Trait that all services must implement
#[async_trait::async_trait]
pub trait Service {
    /// Perform a health check on the service
    async fn health_check(&self) -> AppResult<()>;
    
    /// Gracefully shutdown the service
    async fn shutdown(&self) -> AppResult<()>;
}
