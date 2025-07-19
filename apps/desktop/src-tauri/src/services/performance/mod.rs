use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, error};

use crate::error::AppResult;
use crate::services::Service;

pub mod caching_service;
pub mod request_deduplication;
pub mod background_processor;
pub mod connection_pool;
pub mod performance_optimizer;

pub use caching_service::{CachingService, CacheEntry, CacheStatistics, CacheConfig, EvictionStrategy};
pub use request_deduplication::{RequestDeduplicationService, DuplicateRequestInfo, DeduplicationStatistics};
pub use background_processor::{BackgroundProcessor, BackgroundTask, TaskPriority, TaskResult, BackgroundProcessingStatistics};
pub use connection_pool::{ConnectionPool, PoolConfig, PoolStatistics};
pub use performance_optimizer::{PerformanceOptimizer, OptimizationRecommendation, PerformanceMetrics};

/// Comprehensive performance service that orchestrates all performance optimizations
pub struct PerformanceService {
    caching_service: Arc<CachingService>,
    deduplication_service: Arc<RequestDeduplicationService>,
    background_processor: Arc<BackgroundProcessor>,
    connection_pool: Arc<ConnectionPool>,
    performance_optimizer: Arc<PerformanceOptimizer>,
}

impl PerformanceService {
    /// Create a new performance service
    pub async fn new() -> AppResult<Self> {
        info!("Initializing comprehensive performance service...");

        let caching_service = Arc::new(CachingService::new().await?);
        let deduplication_service = Arc::new(RequestDeduplicationService::new().await?);
        let background_processor = Arc::new(BackgroundProcessor::new().await?);
        let connection_pool = Arc::new(ConnectionPool::new().await?);
        let performance_optimizer = Arc::new(PerformanceOptimizer::new(
            caching_service.clone(),
            deduplication_service.clone(),
            background_processor.clone(),
            connection_pool.clone(),
        ).await?);

        let service = Self {
            caching_service,
            deduplication_service,
            background_processor,
            connection_pool,
            performance_optimizer,
        };

        info!("Comprehensive performance service initialized successfully");
        Ok(service)
    }

    /// Get caching service
    pub fn caching_service(&self) -> Arc<CachingService> {
        self.caching_service.clone()
    }

    /// Get request deduplication service
    pub fn deduplication_service(&self) -> Arc<RequestDeduplicationService> {
        self.deduplication_service.clone()
    }

    /// Get background processor
    pub fn background_processor(&self) -> Arc<BackgroundProcessor> {
        self.background_processor.clone()
    }

    /// Get connection pool
    pub fn connection_pool(&self) -> Arc<ConnectionPool> {
        self.connection_pool.clone()
    }

    /// Get performance optimizer
    pub fn performance_optimizer(&self) -> Arc<PerformanceOptimizer> {
        self.performance_optimizer.clone()
    }

    /// Get comprehensive performance metrics
    pub async fn get_performance_metrics(&self) -> AppResult<ComprehensivePerformanceMetrics> {
        debug!("Collecting comprehensive performance metrics");

        let cache_stats = self.caching_service.get_statistics().await;
        let dedup_stats = self.deduplication_service.get_statistics().await;
        let background_stats = self.background_processor.get_statistics().await;
        let pool_stats = self.connection_pool.get_statistics().await;
        let optimizer_metrics = self.performance_optimizer.get_current_metrics().await?;

        let metrics = ComprehensivePerformanceMetrics {
            cache_statistics: cache_stats,
            deduplication_statistics: dedup_stats,
            background_processing_statistics: background_stats,
            connection_pool_statistics: pool_stats,
            optimization_metrics: optimizer_metrics,
            overall_performance_score: self.calculate_overall_performance_score().await?,
        };

        Ok(metrics)
    }

    /// Calculate overall performance score
    async fn calculate_overall_performance_score(&self) -> AppResult<f64> {
        let cache_stats = self.caching_service.get_statistics().await;
        let dedup_stats = self.deduplication_service.get_statistics().await;
        let pool_stats = self.connection_pool.get_statistics().await;

        // Calculate weighted performance score
        let cache_score = cache_stats.hit_rate * 100.0;
        let dedup_score = dedup_stats.deduplication_rate * 100.0;
        let pool_score = (pool_stats.active_connections as f64 / pool_stats.max_connections as f64) * 100.0;

        let overall_score = (cache_score * 0.4) + (dedup_score * 0.3) + (pool_score * 0.3);
        Ok(overall_score.min(100.0).max(0.0))
    }

    /// Optimize system performance
    pub async fn optimize_performance(&self) -> AppResult<Vec<OptimizationRecommendation>> {
        info!("Running performance optimization analysis");
        self.performance_optimizer.analyze_and_optimize().await
    }

    /// Clear all performance caches
    pub async fn clear_all_caches(&self) -> AppResult<()> {
        info!("Clearing all performance caches");
        self.caching_service.clear_all_caches().await?;
        self.deduplication_service.clear_request_cache().await?;
        Ok(())
    }
}

/// Comprehensive performance metrics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComprehensivePerformanceMetrics {
    pub cache_statistics: CacheStatistics,
    pub deduplication_statistics: DeduplicationStatistics,
    pub background_processing_statistics: BackgroundProcessingStatistics,
    pub connection_pool_statistics: PoolStatistics,
    pub optimization_metrics: PerformanceMetrics,
    pub overall_performance_score: f64,
}



#[async_trait::async_trait]
impl Service for PerformanceService {
    async fn health_check(&self) -> AppResult<()> {
        debug!("Performing comprehensive performance service health check");

        // Check all sub-services
        self.caching_service.health_check().await?;
        self.deduplication_service.health_check().await?;
        self.background_processor.health_check().await?;
        self.connection_pool.health_check().await?;
        self.performance_optimizer.health_check().await?;

        // Check overall performance score
        let performance_score = self.calculate_overall_performance_score().await?;
        if performance_score < 50.0 {
            error!("Overall performance score is low: {:.2}%", performance_score);
            return Err(crate::error::PerformanceError::poor_performance(
                format!("Performance score below threshold: {:.2}%", performance_score)
            ).into());
        }

        debug!("Performance service health check completed successfully (score: {:.2}%)", performance_score);
        Ok(())
    }

    async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down comprehensive performance service...");

        // Shutdown all sub-services
        self.performance_optimizer.shutdown().await?;
        self.connection_pool.shutdown().await?;
        self.background_processor.shutdown().await?;
        self.deduplication_service.shutdown().await?;
        self.caching_service.shutdown().await?;

        info!("Performance service shutdown completed");
        Ok(())
    }
}
