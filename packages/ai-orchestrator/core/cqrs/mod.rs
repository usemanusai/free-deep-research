// CQRS Implementation for Free Deep Research System
// Phase 4.2: CQRS Pattern Implementation

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod commands;
pub mod queries;
pub mod handlers;
pub mod projections;
pub mod read_models;
pub mod error;

#[cfg(test)]
pub mod tests;

use error::{CQRSError, CQRSResult};

// Re-export key types
pub use commands::{Command, CommandBus, CommandHandler, CommandResult};
pub use queries::{Query, QueryBus, QueryHandler, QueryResult};
pub use handlers::{
    CreateResearchWorkflowHandler, StartWorkflowExecutionHandler,
    CreateTaskHandler, CompleteTaskHandler, CompleteWorkflowHandler,
    GetResearchWorkflowHandler, GetWorkflowListHandler, GetWorkflowStatsHandler
};
pub use projections::{ProjectionManager, ProjectionBuilder, ProjectionCheckpoint};
pub use read_models::{
    ResearchWorkflowReadModel, WorkflowListReadModel, WorkflowStatsReadModel,
    TaskReadModel, ReadModelStore
};

/// CQRS Service - Main orchestrator for command/query separation
pub struct CQRSService {
    command_bus: Arc<RwLock<CommandBus>>,
    query_bus: Arc<RwLock<QueryBus>>,
    projection_manager: Arc<RwLock<ProjectionManager>>,
    read_model_store: Arc<RwLock<dyn ReadModelStore>>,
    config: CQRSConfig,
}

/// CQRS Configuration
#[derive(Debug, Clone)]
pub struct CQRSConfig {
    pub command_timeout_seconds: u64,
    pub query_timeout_seconds: u64,
    pub projection_batch_size: usize,
    pub projection_checkpoint_frequency: usize,
    pub read_model_cache_size: usize,
    pub read_model_cache_ttl_seconds: u64,
    pub enable_command_validation: bool,
    pub enable_query_caching: bool,
    pub max_concurrent_projections: usize,
}

impl Default for CQRSConfig {
    fn default() -> Self {
        Self {
            command_timeout_seconds: 30,
            query_timeout_seconds: 10,
            projection_batch_size: 100,
            projection_checkpoint_frequency: 50,
            read_model_cache_size: 10000,
            read_model_cache_ttl_seconds: 300, // 5 minutes
            enable_command_validation: true,
            enable_query_caching: true,
            max_concurrent_projections: 5,
        }
    }
}

impl CQRSService {
    pub fn new(
        read_model_store: Arc<RwLock<dyn ReadModelStore>>,
        config: CQRSConfig,
    ) -> Self {
        Self {
            command_bus: Arc::new(RwLock::new(CommandBus::new())),
            query_bus: Arc::new(RwLock::new(QueryBus::new())),
            projection_manager: Arc::new(RwLock::new(ProjectionManager::new(config.clone()))),
            read_model_store,
            config,
        }
    }

    /// Register a command handler
    pub async fn register_command_handler<C, H>(&self, handler: H)
    where
        C: Command + 'static,
        H: CommandHandler<C> + 'static,
    {
        let mut command_bus = self.command_bus.write().await;
        command_bus.register_handler::<C, H>(handler).await;
    }

    /// Register a query handler
    pub async fn register_query_handler<Q, H>(&self, handler: H)
    where
        Q: Query + 'static,
        H: QueryHandler<Q> + 'static,
    {
        let mut query_bus = self.query_bus.write().await;
        query_bus.register_handler::<Q, H>(handler).await;
    }

    /// Register a projection builder
    pub async fn register_projection<P>(&self, projection: P)
    where
        P: ProjectionBuilder + 'static,
    {
        let mut projection_manager = self.projection_manager.write().await;
        projection_manager.register_projection(Box::new(projection)).await;
    }

    /// Execute a command
    pub async fn execute_command<C>(&self, command: C) -> CQRSResult<CommandResult>
    where
        C: Command + 'static,
    {
        // Validate command if enabled
        if self.config.enable_command_validation {
            command.validate()?;
        }

        // Execute with timeout
        let timeout = tokio::time::Duration::from_secs(self.config.command_timeout_seconds);
        
        tokio::time::timeout(timeout, async {
            let command_bus = self.command_bus.read().await;
            command_bus.execute(command).await
        })
        .await
        .map_err(|_| CQRSError::CommandTimeout)?
    }

    /// Execute a query
    pub async fn execute_query<Q>(&self, query: Q) -> CQRSResult<Q::Result>
    where
        Q: Query + 'static,
    {
        // Validate query
        query.validate()?;

        // Execute with timeout
        let timeout = tokio::time::Duration::from_secs(self.config.query_timeout_seconds);
        
        tokio::time::timeout(timeout, async {
            let query_bus = self.query_bus.read().await;
            query_bus.execute(query).await
        })
        .await
        .map_err(|_| CQRSError::QueryTimeout)?
    }

    /// Start projection processing
    pub async fn start_projections(&self) -> CQRSResult<()> {
        let projection_manager = Arc::clone(&self.projection_manager);
        let read_model_store = Arc::clone(&self.read_model_store);
        
        tokio::spawn(async move {
            let mut manager = projection_manager.write().await;
            if let Err(e) = manager.start_processing(read_model_store).await {
                eprintln!("Projection processing error: {:?}", e);
            }
        });

        Ok(())
    }

    /// Stop projection processing
    pub async fn stop_projections(&self) -> CQRSResult<()> {
        let mut projection_manager = self.projection_manager.write().await;
        projection_manager.stop_processing().await
    }

    /// Get projection status
    pub async fn get_projection_status(&self) -> HashMap<String, ProjectionStatus> {
        let projection_manager = self.projection_manager.read().await;
        projection_manager.get_status().await
    }

    /// Get CQRS metrics
    pub async fn get_metrics(&self) -> CQRSMetrics {
        let command_bus = self.command_bus.read().await;
        let query_bus = self.query_bus.read().await;
        let projection_manager = self.projection_manager.read().await;

        CQRSMetrics {
            commands_executed: command_bus.get_metrics().commands_executed,
            commands_failed: command_bus.get_metrics().commands_failed,
            queries_executed: query_bus.get_metrics().queries_executed,
            queries_failed: query_bus.get_metrics().queries_failed,
            projections_processed: projection_manager.get_metrics().events_processed,
            projections_failed: projection_manager.get_metrics().events_failed,
            read_models_updated: projection_manager.get_metrics().read_models_updated,
            average_command_duration_ms: command_bus.get_metrics().average_duration_ms,
            average_query_duration_ms: query_bus.get_metrics().average_duration_ms,
        }
    }

    /// Health check
    pub async fn health_check(&self) -> CQRSHealthStatus {
        let command_bus_healthy = {
            let command_bus = self.command_bus.read().await;
            command_bus.is_healthy().await
        };

        let query_bus_healthy = {
            let query_bus = self.query_bus.read().await;
            query_bus.is_healthy().await
        };

        let projections_healthy = {
            let projection_manager = self.projection_manager.read().await;
            projection_manager.is_healthy().await
        };

        let read_models_healthy = {
            let read_model_store = self.read_model_store.read().await;
            read_model_store.health_check().await.is_ok()
        };

        CQRSHealthStatus {
            overall_healthy: command_bus_healthy && query_bus_healthy && projections_healthy && read_models_healthy,
            command_bus_healthy,
            query_bus_healthy,
            projections_healthy,
            read_models_healthy,
            last_check: chrono::Utc::now(),
        }
    }
}

/// Projection status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectionStatus {
    pub name: String,
    pub is_running: bool,
    pub last_processed_event: Option<Uuid>,
    pub last_checkpoint: Option<ProjectionCheckpoint>,
    pub events_processed: u64,
    pub events_failed: u64,
    pub last_error: Option<String>,
    pub processing_rate_per_second: f64,
}

/// CQRS metrics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQRSMetrics {
    pub commands_executed: u64,
    pub commands_failed: u64,
    pub queries_executed: u64,
    pub queries_failed: u64,
    pub projections_processed: u64,
    pub projections_failed: u64,
    pub read_models_updated: u64,
    pub average_command_duration_ms: f64,
    pub average_query_duration_ms: f64,
}

/// CQRS health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQRSHealthStatus {
    pub overall_healthy: bool,
    pub command_bus_healthy: bool,
    pub query_bus_healthy: bool,
    pub projections_healthy: bool,
    pub read_models_healthy: bool,
    pub last_check: chrono::DateTime<chrono::Utc>,
}

/// CQRS service builder for easier configuration
pub struct CQRSServiceBuilder {
    config: CQRSConfig,
    read_model_store: Option<Arc<RwLock<dyn ReadModelStore>>>,
}

impl CQRSServiceBuilder {
    pub fn new() -> Self {
        Self {
            config: CQRSConfig::default(),
            read_model_store: None,
        }
    }

    pub fn with_config(mut self, config: CQRSConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_read_model_store(mut self, store: Arc<RwLock<dyn ReadModelStore>>) -> Self {
        self.read_model_store = Some(store);
        self
    }

    pub fn with_command_timeout(mut self, seconds: u64) -> Self {
        self.config.command_timeout_seconds = seconds;
        self
    }

    pub fn with_query_timeout(mut self, seconds: u64) -> Self {
        self.config.query_timeout_seconds = seconds;
        self
    }

    pub fn with_projection_batch_size(mut self, size: usize) -> Self {
        self.config.projection_batch_size = size;
        self
    }

    pub fn enable_caching(mut self, enabled: bool) -> Self {
        self.config.enable_query_caching = enabled;
        self
    }

    pub fn build(self) -> CQRSResult<CQRSService> {
        let read_model_store = self.read_model_store
            .ok_or_else(|| CQRSError::Configuration("Read model store is required".to_string()))?;

        Ok(CQRSService::new(read_model_store, self.config))
    }
}

impl Default for CQRSServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cqrs::read_models::MockReadModelStore;

    #[tokio::test]
    async fn test_cqrs_service_creation() {
        let read_model_store = Arc::new(RwLock::new(MockReadModelStore::new()));
        let config = CQRSConfig::default();
        
        let cqrs_service = CQRSService::new(read_model_store, config);
        
        // Test health check
        let health = cqrs_service.health_check().await;
        assert!(health.overall_healthy);
    }

    #[tokio::test]
    async fn test_cqrs_service_builder() {
        let read_model_store = Arc::new(RwLock::new(MockReadModelStore::new()));
        
        let cqrs_service = CQRSServiceBuilder::new()
            .with_command_timeout(60)
            .with_query_timeout(30)
            .with_projection_batch_size(200)
            .enable_caching(true)
            .with_read_model_store(read_model_store)
            .build()
            .expect("Failed to build CQRS service");

        assert_eq!(cqrs_service.config.command_timeout_seconds, 60);
        assert_eq!(cqrs_service.config.query_timeout_seconds, 30);
        assert_eq!(cqrs_service.config.projection_batch_size, 200);
        assert!(cqrs_service.config.enable_query_caching);
    }

    #[tokio::test]
    async fn test_cqrs_metrics() {
        let read_model_store = Arc::new(RwLock::new(MockReadModelStore::new()));
        let cqrs_service = CQRSService::new(read_model_store, CQRSConfig::default());
        
        let metrics = cqrs_service.get_metrics().await;
        
        // Initial metrics should be zero
        assert_eq!(metrics.commands_executed, 0);
        assert_eq!(metrics.queries_executed, 0);
        assert_eq!(metrics.projections_processed, 0);
    }
}
