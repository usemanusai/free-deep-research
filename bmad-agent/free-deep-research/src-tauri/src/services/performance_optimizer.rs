//! Free Deep Research System - Performance Optimization Service
//! Comprehensive performance monitoring, caching, and optimization

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::error::{AppResult, AppError};

/// Performance metrics data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub timestamp: DateTime<Utc>,
    pub operation: String,
    pub duration_ms: u64,
    pub memory_usage: u64,
    pub cpu_usage: f64,
    pub success: bool,
    pub error_message: Option<String>,
}

/// Cache entry with TTL
#[derive(Debug, Clone)]
struct CacheEntry<T> {
    data: T,
    created_at: Instant,
    ttl: Duration,
    access_count: u64,
    last_accessed: Instant,
}

impl<T> CacheEntry<T> {
    fn new(data: T, ttl: Duration) -> Self {
        let now = Instant::now();
        Self {
            data,
            created_at: now,
            ttl,
            access_count: 0,
            last_accessed: now,
        }
    }
    
    fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }
    
    fn access(&mut self) -> &T {
        self.access_count += 1;
        self.last_accessed = Instant::now();
        &self.data
    }
}

/// Performance optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub enable_caching: bool,
    pub cache_ttl_seconds: u64,
    pub max_cache_size: usize,
    pub enable_metrics: bool,
    pub metrics_retention_hours: u64,
    pub enable_compression: bool,
    pub compression_threshold_bytes: usize,
    pub enable_request_batching: bool,
    pub batch_size: usize,
    pub batch_timeout_ms: u64,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_caching: true,
            cache_ttl_seconds: 3600, // 1 hour
            max_cache_size: 1000,
            enable_metrics: true,
            metrics_retention_hours: 24,
            enable_compression: true,
            compression_threshold_bytes: 1024, // 1KB
            enable_request_batching: true,
            batch_size: 10,
            batch_timeout_ms: 100,
        }
    }
}

/// Performance optimization service
pub struct PerformanceOptimizer {
    config: PerformanceConfig,
    cache: Arc<RwLock<HashMap<String, CacheEntry<Vec<u8>>>>>,
    metrics: Arc<RwLock<Vec<PerformanceMetrics>>>,
    operation_stats: Arc<RwLock<HashMap<String, OperationStats>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OperationStats {
    total_calls: u64,
    total_duration_ms: u64,
    average_duration_ms: f64,
    min_duration_ms: u64,
    max_duration_ms: u64,
    success_rate: f64,
    last_updated: DateTime<Utc>,
}

impl PerformanceOptimizer {
    /// Create new performance optimizer
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            config,
            cache: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(Vec::new())),
            operation_stats: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Start performance monitoring background task
    pub async fn start_monitoring(&self) -> AppResult<()> {
        if !self.config.enable_metrics {
            return Ok(());
        }
        
        let metrics = Arc::clone(&self.metrics);
        let retention_hours = self.config.metrics_retention_hours;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(300)); // 5 minutes
            
            loop {
                interval.tick().await;
                
                // Clean up old metrics
                let cutoff = Utc::now() - chrono::Duration::hours(retention_hours as i64);
                let mut metrics_guard = metrics.write().await;
                metrics_guard.retain(|metric| metric.timestamp > cutoff);
                
                // Log metrics summary
                if !metrics_guard.is_empty() {
                    let total_operations = metrics_guard.len();
                    let success_rate = metrics_guard.iter()
                        .filter(|m| m.success)
                        .count() as f64 / total_operations as f64 * 100.0;
                    
                    log::info!(
                        "Performance metrics summary: {} operations, {:.2}% success rate",
                        total_operations,
                        success_rate
                    );
                }
            }
        });
        
        Ok(())
    }
    
    /// Record performance metrics for an operation
    pub async fn record_operation<F, T>(&self, operation: &str, func: F) -> AppResult<T>
    where
        F: std::future::Future<Output = AppResult<T>>,
    {
        let start_time = Instant::now();
        let start_memory = self.get_memory_usage();
        let start_cpu = self.get_cpu_usage();
        
        let result = func.await;
        
        let duration = start_time.elapsed();
        let end_memory = self.get_memory_usage();
        let end_cpu = self.get_cpu_usage();
        
        let success = result.is_ok();
        let error_message = if let Err(ref e) = result {
            Some(e.to_string())
        } else {
            None
        };
        
        // Record metrics
        if self.config.enable_metrics {
            let metric = PerformanceMetrics {
                timestamp: Utc::now(),
                operation: operation.to_string(),
                duration_ms: duration.as_millis() as u64,
                memory_usage: end_memory.saturating_sub(start_memory),
                cpu_usage: end_cpu - start_cpu,
                success,
                error_message,
            };
            
            let mut metrics_guard = self.metrics.write().await;
            metrics_guard.push(metric);
        }
        
        // Update operation statistics
        self.update_operation_stats(operation, duration, success).await;
        
        result
    }
    
    /// Get data from cache or compute and cache it
    pub async fn get_or_compute<F, T>(&self, key: &str, compute_fn: F) -> AppResult<T>
    where
        F: std::future::Future<Output = AppResult<T>>,
        T: Serialize + for<'de> Deserialize<'de>,
    {
        if !self.config.enable_caching {
            return compute_fn.await;
        }
        
        // Try to get from cache first
        {
            let mut cache_guard = self.cache.write().await;
            if let Some(entry) = cache_guard.get_mut(key) {
                if !entry.is_expired() {
                    let data = entry.access();
                    if let Ok(result) = bincode::deserialize::<T>(data) {
                        return Ok(result);
                    }
                }
                // Remove expired or corrupted entry
                cache_guard.remove(key);
            }
        }
        
        // Compute new value
        let result = compute_fn.await?;
        
        // Cache the result
        if let Ok(serialized) = bincode::serialize(&result) {
            let compressed = if self.config.enable_compression && 
                serialized.len() > self.config.compression_threshold_bytes {
                self.compress_data(&serialized)?
            } else {
                serialized
            };
            
            let entry = CacheEntry::new(
                compressed,
                Duration::from_secs(self.config.cache_ttl_seconds),
            );
            
            let mut cache_guard = self.cache.write().await;
            
            // Evict old entries if cache is full
            if cache_guard.len() >= self.config.max_cache_size {
                self.evict_cache_entries(&mut cache_guard).await;
            }
            
            cache_guard.insert(key.to_string(), entry);
        }
        
        Ok(result)
    }
    
    /// Clear cache
    pub async fn clear_cache(&self) -> AppResult<()> {
        let mut cache_guard = self.cache.write().await;
        cache_guard.clear();
        Ok(())
    }
    
    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> AppResult<serde_json::Value> {
        let cache_guard = self.cache.read().await;
        let total_entries = cache_guard.len();
        let expired_entries = cache_guard.values()
            .filter(|entry| entry.is_expired())
            .count();
        
        let total_access_count: u64 = cache_guard.values()
            .map(|entry| entry.access_count)
            .sum();
        
        Ok(serde_json::json!({
            "total_entries": total_entries,
            "expired_entries": expired_entries,
            "active_entries": total_entries - expired_entries,
            "total_access_count": total_access_count,
            "hit_rate": if total_access_count > 0 { 
                (total_access_count as f64 / (total_access_count + expired_entries as u64) as f64) * 100.0 
            } else { 
                0.0 
            }
        }))
    }
    
    /// Get performance metrics
    pub async fn get_metrics(&self, operation: Option<String>) -> AppResult<Vec<PerformanceMetrics>> {
        let metrics_guard = self.metrics.read().await;
        
        let filtered_metrics = if let Some(op) = operation {
            metrics_guard.iter()
                .filter(|metric| metric.operation == op)
                .cloned()
                .collect()
        } else {
            metrics_guard.clone()
        };
        
        Ok(filtered_metrics)
    }
    
    /// Get operation statistics
    pub async fn get_operation_stats(&self) -> AppResult<HashMap<String, OperationStats>> {
        let stats_guard = self.operation_stats.read().await;
        Ok(stats_guard.clone())
    }
    
    /// Optimize system performance
    pub async fn optimize_performance(&self) -> AppResult<serde_json::Value> {
        let mut optimizations = Vec::new();
        
        // Analyze cache performance
        let cache_stats = self.get_cache_stats().await?;
        let hit_rate = cache_stats["hit_rate"].as_f64().unwrap_or(0.0);
        
        if hit_rate < 50.0 {
            optimizations.push("Consider increasing cache TTL or size");
        }
        
        // Analyze operation performance
        let operation_stats = self.get_operation_stats().await?;
        for (operation, stats) in operation_stats.iter() {
            if stats.average_duration_ms > 5000.0 {
                optimizations.push(format!("Operation '{}' is slow (avg: {:.2}ms)", operation, stats.average_duration_ms));
            }
            
            if stats.success_rate < 95.0 {
                optimizations.push(format!("Operation '{}' has low success rate ({:.2}%)", operation, stats.success_rate));
            }
        }
        
        // Memory usage analysis
        let current_memory = self.get_memory_usage();
        if current_memory > 1_000_000_000 { // 1GB
            optimizations.push("High memory usage detected - consider reducing cache size");
        }
        
        // Clean up expired cache entries
        {
            let mut cache_guard = self.cache.write().await;
            let initial_size = cache_guard.len();
            cache_guard.retain(|_, entry| !entry.is_expired());
            let cleaned_entries = initial_size - cache_guard.len();
            
            if cleaned_entries > 0 {
                optimizations.push(format!("Cleaned {} expired cache entries", cleaned_entries));
            }
        }
        
        Ok(serde_json::json!({
            "optimizations_applied": optimizations,
            "cache_stats": cache_stats,
            "timestamp": Utc::now()
        }))
    }
    
    /// Private helper methods
    async fn update_operation_stats(&self, operation: &str, duration: Duration, success: bool) {
        let mut stats_guard = self.operation_stats.write().await;
        let duration_ms = duration.as_millis() as u64;
        
        let stats = stats_guard.entry(operation.to_string()).or_insert(OperationStats {
            total_calls: 0,
            total_duration_ms: 0,
            average_duration_ms: 0.0,
            min_duration_ms: u64::MAX,
            max_duration_ms: 0,
            success_rate: 100.0,
            last_updated: Utc::now(),
        });
        
        stats.total_calls += 1;
        stats.total_duration_ms += duration_ms;
        stats.average_duration_ms = stats.total_duration_ms as f64 / stats.total_calls as f64;
        stats.min_duration_ms = stats.min_duration_ms.min(duration_ms);
        stats.max_duration_ms = stats.max_duration_ms.max(duration_ms);
        
        // Update success rate
        let success_count = if success { 1.0 } else { 0.0 };
        stats.success_rate = ((stats.success_rate * (stats.total_calls - 1) as f64) + success_count) / stats.total_calls as f64 * 100.0;
        
        stats.last_updated = Utc::now();
    }
    
    async fn evict_cache_entries(&self, cache: &mut HashMap<String, CacheEntry<Vec<u8>>>) {
        // Simple LRU eviction - remove least recently accessed entries
        let mut entries: Vec<_> = cache.iter().collect();
        entries.sort_by_key(|(_, entry)| entry.last_accessed);
        
        // Remove oldest 25% of entries
        let remove_count = cache.len() / 4;
        for (key, _) in entries.iter().take(remove_count) {
            cache.remove(*key);
        }
    }
    
    fn compress_data(&self, data: &[u8]) -> AppResult<Vec<u8>> {
        use flate2::Compression;
        use flate2::write::GzEncoder;
        use std::io::Write;
        
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data)
            .map_err(|e| AppError::Custom(format!("Compression failed: {}", e)))?;
        
        encoder.finish()
            .map_err(|e| AppError::Custom(format!("Compression finalization failed: {}", e)))
    }
    
    fn get_memory_usage(&self) -> u64 {
        // Simplified memory usage - in production, use proper system metrics
        std::process::id() as u64 * 1024 // Placeholder
    }
    
    fn get_cpu_usage(&self) -> f64 {
        // Simplified CPU usage - in production, use proper system metrics
        0.0 // Placeholder
    }
}

/// Performance monitoring middleware
pub struct PerformanceMiddleware {
    optimizer: Arc<PerformanceOptimizer>,
}

impl PerformanceMiddleware {
    pub fn new(optimizer: Arc<PerformanceOptimizer>) -> Self {
        Self { optimizer }
    }
    
    pub async fn measure_request<F, T>(&self, operation: &str, handler: F) -> AppResult<T>
    where
        F: std::future::Future<Output = AppResult<T>>,
    {
        self.optimizer.record_operation(operation, handler).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;
    
    #[tokio::test]
    async fn test_performance_optimizer() {
        let config = PerformanceConfig::default();
        let optimizer = PerformanceOptimizer::new(config);
        
        // Test operation recording
        let result = optimizer.record_operation("test_operation", async {
            sleep(Duration::from_millis(10)).await;
            Ok("test_result")
        }).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_result");
        
        // Check metrics were recorded
        let metrics = optimizer.get_metrics(Some("test_operation".to_string())).await.unwrap();
        assert_eq!(metrics.len(), 1);
        assert_eq!(metrics[0].operation, "test_operation");
        assert!(metrics[0].success);
    }
    
    #[tokio::test]
    async fn test_caching() {
        let config = PerformanceConfig::default();
        let optimizer = PerformanceOptimizer::new(config);
        
        let mut call_count = 0;
        
        // First call should compute
        let result1 = optimizer.get_or_compute("test_key", async {
            call_count += 1;
            Ok("computed_value")
        }).await;
        
        assert!(result1.is_ok());
        assert_eq!(result1.unwrap(), "computed_value");
        assert_eq!(call_count, 1);
        
        // Second call should use cache
        let result2 = optimizer.get_or_compute("test_key", async {
            call_count += 1;
            Ok("computed_value")
        }).await;
        
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), "computed_value");
        assert_eq!(call_count, 1); // Should not increment
    }
}
