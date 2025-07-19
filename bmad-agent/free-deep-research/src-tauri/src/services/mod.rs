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
pub mod collaboration;
pub mod mobile_api;
pub mod performance;

// V1.2.0 Services
pub mod plugin_system;
pub mod workflow_engine;
pub mod ml_engine;
pub mod cloud_sync;
pub mod enterprise;

// V2.0.0 Services
pub mod distributed;
pub mod ai_orchestration;
pub mod realtime_collaboration;
pub mod bmad_integration;

// V3.0.0 Services - Global Intelligence Network
pub mod federated_research;
pub mod ai_marketplace;
pub mod quantum_ready;
pub mod nlp_engine;
pub mod blockchain;
pub mod knowledge_graph;

use crate::error::{AppError, AppResult};
use api_manager::ApiManagerService;
use research_engine::ResearchEngineService;
use template_manager::TemplateManagerService;
use data_persistence::DataPersistenceService;
use monitoring::MonitoringService;
use security::SecurityService;
use output_processor::OutputProcessorService;
use analytics::AnalyticsService;
use collaboration::CollaborationService;
use mobile_api::MobileApiService;
use performance::PerformanceService;

// V1.2.0 Services
use plugin_system::PluginSystemService;
use workflow_engine::WorkflowEngineService;
use ml_engine::MLEngineService;
use cloud_sync::CloudSyncService;
use enterprise::EnterpriseService;

// V2.0.0 Services
use distributed::DistributedService;
use ai_orchestration::AIOrchestrationService;
use realtime_collaboration::RealtimeCollaborationService;
use bmad_integration::BMadIntegrationService;

// V3.0.0 Services
use federated_research::FederatedResearchService;
use ai_marketplace::AIMarketplaceService;
use quantum_ready::QuantumReadyService;
use nlp_engine::NLPEngineService;
use blockchain::BlockchainService;
use knowledge_graph::KnowledgeGraphService;

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
    pub collaboration: Arc<RwLock<CollaborationService>>,
    pub mobile_api: Arc<RwLock<MobileApiService>>,
    pub performance: Arc<RwLock<PerformanceService>>,

    // V1.2.0 Services
    pub plugin_system: Arc<RwLock<PluginSystemService>>,
    pub workflow_engine: Arc<RwLock<WorkflowEngineService>>,
    pub ml_engine: Arc<RwLock<MLEngineService>>,
    pub cloud_sync: Arc<RwLock<CloudSyncService>>,
    pub enterprise: Arc<RwLock<EnterpriseService>>,

    // V2.0.0 Services
    pub distributed: Arc<RwLock<DistributedService>>,
    pub ai_orchestration: Arc<RwLock<AIOrchestrationService>>,
    pub realtime_collaboration: Arc<RwLock<RealtimeCollaborationService>>,
    pub bmad_integration: Arc<RwLock<BMadIntegrationService>>,

    // V3.0.0 Services - Global Intelligence Network
    pub federated_research_service: Arc<RwLock<FederatedResearchService>>,
    pub ai_marketplace_service: Arc<RwLock<AIMarketplaceService>>,
    pub quantum_ready_service: Arc<RwLock<QuantumReadyService>>,
    pub nlp_engine_service: Arc<RwLock<NLPEngineService>>,
    pub blockchain_service: Arc<RwLock<BlockchainService>>,
    pub knowledge_graph_service: Arc<RwLock<KnowledgeGraphService>>,
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

        // Initialize V3.0.0 Services - Global Intelligence Network
        let federated_research_service = FederatedResearchService::new(
            data_persistence.clone(),
            security.clone(),
        ).await?;
        let federated_research_service = Arc::new(RwLock::new(federated_research_service));

        let ai_marketplace_service = AIMarketplaceService::new(
            data_persistence.clone(),
            security.clone(),
        ).await?;
        let ai_marketplace_service = Arc::new(RwLock::new(ai_marketplace_service));

        let quantum_ready_service = QuantumReadyService::new(
            data_persistence.clone(),
            security.clone(),
        ).await?;
        let quantum_ready_service = Arc::new(RwLock::new(quantum_ready_service));

        let nlp_engine_service = NLPEngineService::new(
            data_persistence.clone(),
        ).await?;
        let nlp_engine_service = Arc::new(RwLock::new(nlp_engine_service));

        let blockchain_service = BlockchainService::new(
            data_persistence.clone(),
        ).await?;
        let blockchain_service = Arc::new(RwLock::new(blockchain_service));

        let knowledge_graph_service = KnowledgeGraphService::new(
            data_persistence.clone(),
        ).await?;
        let knowledge_graph_service = Arc::new(RwLock::new(knowledge_graph_service));

        // Initialize V2.0.0 services first
        let ai_orchestration = Arc::new(RwLock::new(AIOrchestrationService::new().await?));

        // Initialize BMAD integration service
        let bmad_integration = BMadIntegrationService::new(
            research_engine.clone(),
            ai_orchestration.clone(),
            api_manager.clone(),
            data_persistence.clone(),
            None, // Use default config
        ).await?;
        let bmad_integration = Arc::new(RwLock::new(bmad_integration));

        let service_manager = Self {
            api_manager,
            research_engine,
            template_manager,
            data_persistence,
            monitoring,
            security,
            output_processor,
            analytics,
            // TODO: Initialize missing services (collaboration, mobile_api, plugin_system, etc.)
            collaboration: Arc::new(RwLock::new(CollaborationService::new().await?)),
            mobile_api: Arc::new(RwLock::new(MobileApiService::new().await?)),
            performance: Arc::new(RwLock::new(PerformanceService::new().await?)),
            plugin_system: Arc::new(RwLock::new(PluginSystemService::new().await?)),
            workflow_engine: Arc::new(RwLock::new(WorkflowEngineService::new().await?)),
            ml_engine: Arc::new(RwLock::new(MLEngineService::new().await?)),
            cloud_sync: Arc::new(RwLock::new(CloudSyncService::new().await?)),
            enterprise: Arc::new(RwLock::new(EnterpriseService::new().await?)),
            distributed: Arc::new(RwLock::new(DistributedService::new().await?)),
            ai_orchestration,
            realtime_collaboration: Arc::new(RwLock::new(RealtimeCollaborationService::new().await?)),
            bmad_integration,
            // V3.0.0 Services
            federated_research_service,
            ai_marketplace_service,
            quantum_ready_service,
            nlp_engine_service,
            blockchain_service,
            knowledge_graph_service,
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

        // Start V3.0.0 services background tasks
        {
            let federated_research = self.federated_research_service.read().await;
            federated_research.start_background_tasks().await?;
        }

        {
            let ai_marketplace = self.ai_marketplace_service.read().await;
            ai_marketplace.start_background_tasks().await?;
        }

        {
            let quantum_ready = self.quantum_ready_service.read().await;
            quantum_ready.start_background_tasks().await?;
        }

        {
            let nlp_engine = self.nlp_engine_service.read().await;
            nlp_engine.start_background_tasks().await?;
        }

        {
            let blockchain = self.blockchain_service.read().await;
            blockchain.start_background_tasks().await?;
        }

        {
            let knowledge_graph = self.knowledge_graph_service.read().await;
            knowledge_graph.start_background_tasks().await?;
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

        // Check performance service
        match self.performance.read().await.health_check().await {
            Ok(_) => status.performance = ServiceStatus::Healthy,
            Err(e) => {
                error!("Performance service health check failed: {}", e);
                status.performance = ServiceStatus::Unhealthy;
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
    pub performance: ServiceStatus,
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
            performance: ServiceStatus::Unknown,
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
            && matches!(self.performance, ServiceStatus::Healthy)
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
