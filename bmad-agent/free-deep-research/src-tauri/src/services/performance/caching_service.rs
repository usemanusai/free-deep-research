use std::collections::HashMap;
use std::sync::Arc;
use std::hash::{Hash, Hasher};
use tokio::sync::RwLock;
use tracing::{info, debug, warn, error};
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::error::{AppResult, CacheError};
use crate::services::Service;

/// Cache entry with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    pub data: T,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub access_count: u64,
    pub last_accessed: DateTime<Utc>,
    pub size_bytes: usize,
    pub cache_key: String,
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStatistics {
    pub total_entries: usize,
    pub total_size_bytes: usize,
    pub hit_count: u64,
    pub miss_count: u64,
    pub hit_rate: f64,
    pub eviction_count: u64,
    pub average_access_time_ms: f64,
    pub most_accessed_keys: Vec<String>,
    pub cache_efficiency_score: f64,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub max_entries: usize,
    pub max_size_bytes: usize,
    pub default_ttl_minutes: i64,
    pub cleanup_interval_minutes: i64,
    pub enable_compression: bool,
    pub enable_statistics: bool,
    pub eviction_strategy: EvictionStrategy,
}

/// Cache eviction strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvictionStrategy {
    LRU,  // Least Recently Used
    LFU,  // Least Frequently Used
    TTL,  // Time To Live based
    Size, // Size based
}

/// Intelligent caching service with multiple cache types
pub struct CachingService {
    // API response cache
    api_cache: Arc<RwLock<HashMap<String, CacheEntry<String>>>>,
    
    // Research result cache
    research_cache: Arc<RwLock<HashMap<String, CacheEntry<serde_json::Value>>>>,
    
    // Database query cache
    db_cache: Arc<RwLock<HashMap<String, CacheEntry<serde_json::Value>>>>,
    
    // Template cache
    template_cache: Arc<RwLock<HashMap<String, CacheEntry<String>>>>,
    
    // Configuration
    config: Arc<RwLock<CacheConfig>>,
    
    // Statistics
    statistics: Arc<RwLock<CacheStatistics>>,
    
    // Background cleanup task handle
    cleanup_task: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

impl CachingService {
    /// Create a new caching service
    pub async fn new() -> AppResult<Self> {
        info!("Initializing intelligent caching service...");

        let config = CacheConfig {
            max_entries: 10000,
            max_size_bytes: 100 * 1024 * 1024, // 100MB
            default_ttl_minutes: 60,
            cleanup_interval_minutes: 10,
            enable_compression: true,
            enable_statistics: true,
            eviction_strategy: EvictionStrategy::LRU,
        };

        let statistics = CacheStatistics {
            total_entries: 0,
            total_size_bytes: 0,
            hit_count: 0,
            miss_count: 0,
            hit_rate: 0.0,
            eviction_count: 0,
            average_access_time_ms: 0.0,
            most_accessed_keys: Vec::new(),
            cache_efficiency_score: 0.0,
        };

        let service = Self {
            api_cache: Arc::new(RwLock::new(HashMap::new())),
            research_cache: Arc::new(RwLock::new(HashMap::new())),
            db_cache: Arc::new(RwLock::new(HashMap::new())),
            template_cache: Arc::new(RwLock::new(HashMap::new())),
            config: Arc::new(RwLock::new(config)),
            statistics: Arc::new(RwLock::new(statistics)),
            cleanup_task: Arc::new(RwLock::new(None)),
        };

        // Start background cleanup task
        service.start_cleanup_task().await?;

        info!("Intelligent caching service initialized successfully");
        Ok(service)
    }

    /// Generate cache key from request parameters
    pub fn generate_cache_key(&self, prefix: &str, params: &[&str]) -> String {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        prefix.hash(&mut hasher);
        for param in params {
            param.hash(&mut hasher);
        }
        format!("{}:{:x}", prefix, hasher.finish())
    }

    /// Cache API response
    pub async fn cache_api_response(&self, key: String, response: String, ttl_minutes: Option<i64>) -> AppResult<()> {
        let config = self.config.read().await;
        let ttl = ttl_minutes.unwrap_or(config.default_ttl_minutes);
        drop(config);

        let entry = CacheEntry {
            data: response.clone(),
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::minutes(ttl),
            access_count: 0,
            last_accessed: Utc::now(),
            size_bytes: response.len(),
            cache_key: key.clone(),
        };

        let mut cache = self.api_cache.write().await;
        cache.insert(key.clone(), entry);
        drop(cache);

        self.update_statistics_on_insert().await;
        debug!("Cached API response with key: {}", key);
        Ok(())
    }

    /// Get cached API response
    pub async fn get_cached_api_response(&self, key: &str) -> AppResult<Option<String>> {
        let mut cache = self.api_cache.write().await;
        
        if let Some(entry) = cache.get_mut(key) {
            // Check if expired
            if entry.expires_at < Utc::now() {
                cache.remove(key);
                self.update_statistics_on_miss().await;
                return Ok(None);
            }

            // Update access statistics
            entry.access_count += 1;
            entry.last_accessed = Utc::now();
            
            let data = entry.data.clone();
            drop(cache);
            
            self.update_statistics_on_hit().await;
            debug!("Cache hit for API response key: {}", key);
            Ok(Some(data))
        } else {
            drop(cache);
            self.update_statistics_on_miss().await;
            debug!("Cache miss for API response key: {}", key);
            Ok(None)
        }
    }

    /// Cache research result
    pub async fn cache_research_result(&self, workflow_id: Uuid, result: serde_json::Value, ttl_minutes: Option<i64>) -> AppResult<()> {
        let key = format!("research:{}", workflow_id);
        let config = self.config.read().await;
        let ttl = ttl_minutes.unwrap_or(config.default_ttl_minutes * 2); // Research results cached longer
        drop(config);

        let result_str = serde_json::to_string(&result)?;
        let entry = CacheEntry {
            data: result.clone(),
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::minutes(ttl),
            access_count: 0,
            last_accessed: Utc::now(),
            size_bytes: result_str.len(),
            cache_key: key.clone(),
        };

        let mut cache = self.research_cache.write().await;
        cache.insert(key.clone(), entry);
        drop(cache);

        self.update_statistics_on_insert().await;
        debug!("Cached research result for workflow: {}", workflow_id);
        Ok(())
    }

    /// Get cached research result
    pub async fn get_cached_research_result(&self, workflow_id: Uuid) -> AppResult<Option<serde_json::Value>> {
        let key = format!("research:{}", workflow_id);
        let mut cache = self.research_cache.write().await;
        
        if let Some(entry) = cache.get_mut(&key) {
            if entry.expires_at < Utc::now() {
                cache.remove(&key);
                self.update_statistics_on_miss().await;
                return Ok(None);
            }

            entry.access_count += 1;
            entry.last_accessed = Utc::now();
            
            let data = entry.data.clone();
            drop(cache);
            
            self.update_statistics_on_hit().await;
            debug!("Cache hit for research result: {}", workflow_id);
            Ok(Some(data))
        } else {
            drop(cache);
            self.update_statistics_on_miss().await;
            debug!("Cache miss for research result: {}", workflow_id);
            Ok(None)
        }
    }

    /// Cache database query result
    pub async fn cache_db_query(&self, query_hash: String, result: serde_json::Value, ttl_minutes: Option<i64>) -> AppResult<()> {
        let key = format!("db:{}", query_hash);
        let config = self.config.read().await;
        let ttl = ttl_minutes.unwrap_or(config.default_ttl_minutes / 2); // DB cache shorter TTL
        drop(config);

        let result_str = serde_json::to_string(&result)?;
        let entry = CacheEntry {
            data: result.clone(),
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::minutes(ttl),
            access_count: 0,
            last_accessed: Utc::now(),
            size_bytes: result_str.len(),
            cache_key: key.clone(),
        };

        let mut cache = self.db_cache.write().await;
        cache.insert(key.clone(), entry);
        drop(cache);

        self.update_statistics_on_insert().await;
        debug!("Cached database query result: {}", query_hash);
        Ok(())
    }

    /// Get cached database query result
    pub async fn get_cached_db_query(&self, query_hash: &str) -> AppResult<Option<serde_json::Value>> {
        let key = format!("db:{}", query_hash);
        let mut cache = self.db_cache.write().await;
        
        if let Some(entry) = cache.get_mut(&key) {
            if entry.expires_at < Utc::now() {
                cache.remove(&key);
                self.update_statistics_on_miss().await;
                return Ok(None);
            }

            entry.access_count += 1;
            entry.last_accessed = Utc::now();
            
            let data = entry.data.clone();
            drop(cache);
            
            self.update_statistics_on_hit().await;
            debug!("Cache hit for database query: {}", query_hash);
            Ok(Some(data))
        } else {
            drop(cache);
            self.update_statistics_on_miss().await;
            debug!("Cache miss for database query: {}", query_hash);
            Ok(None)
        }
    }

    /// Get cache statistics
    pub async fn get_statistics(&self) -> CacheStatistics {
        let statistics = self.statistics.read().await;
        statistics.clone()
    }

    /// Clear all caches
    pub async fn clear_all_caches(&self) -> AppResult<()> {
        info!("Clearing all caches...");
        
        let mut api_cache = self.api_cache.write().await;
        api_cache.clear();
        drop(api_cache);

        let mut research_cache = self.research_cache.write().await;
        research_cache.clear();
        drop(research_cache);

        let mut db_cache = self.db_cache.write().await;
        db_cache.clear();
        drop(db_cache);

        let mut template_cache = self.template_cache.write().await;
        template_cache.clear();
        drop(template_cache);

        // Reset statistics
        let mut statistics = self.statistics.write().await;
        *statistics = CacheStatistics {
            total_entries: 0,
            total_size_bytes: 0,
            hit_count: 0,
            miss_count: 0,
            hit_rate: 0.0,
            eviction_count: 0,
            average_access_time_ms: 0.0,
            most_accessed_keys: Vec::new(),
            cache_efficiency_score: 0.0,
        };
        drop(statistics);

        info!("All caches cleared successfully");
        Ok(())
    }

    /// Start background cleanup task
    async fn start_cleanup_task(&self) -> AppResult<()> {
        let api_cache = self.api_cache.clone();
        let research_cache = self.research_cache.clone();
        let db_cache = self.db_cache.clone();
        let template_cache = self.template_cache.clone();
        let config = self.config.clone();
        let statistics = self.statistics.clone();

        let task = tokio::spawn(async move {
            loop {
                let cleanup_interval = {
                    let config = config.read().await;
                    config.cleanup_interval_minutes
                };

                tokio::time::sleep(tokio::time::Duration::from_secs((cleanup_interval * 60) as u64)).await;

                // Cleanup expired entries
                Self::cleanup_expired_entries(&api_cache, &statistics).await;
                Self::cleanup_expired_entries(&research_cache, &statistics).await;
                Self::cleanup_expired_entries(&db_cache, &statistics).await;
                Self::cleanup_expired_entries(&template_cache, &statistics).await;

                debug!("Cache cleanup completed");
            }
        });

        let mut cleanup_task = self.cleanup_task.write().await;
        *cleanup_task = Some(task);
        drop(cleanup_task);

        Ok(())
    }

    /// Cleanup expired entries from a cache
    async fn cleanup_expired_entries<T: Clone>(
        cache: &Arc<RwLock<HashMap<String, CacheEntry<T>>>>,
        statistics: &Arc<RwLock<CacheStatistics>>,
    ) {
        let mut cache_guard = cache.write().await;
        let now = Utc::now();
        let mut expired_keys = Vec::new();

        for (key, entry) in cache_guard.iter() {
            if entry.expires_at < now {
                expired_keys.push(key.clone());
            }
        }

        for key in expired_keys {
            cache_guard.remove(&key);
            
            // Update statistics
            let mut stats = statistics.write().await;
            stats.eviction_count += 1;
            drop(stats);
        }

        drop(cache_guard);
    }

    /// Update statistics on cache hit
    async fn update_statistics_on_hit(&self) {
        let mut statistics = self.statistics.write().await;
        statistics.hit_count += 1;
        statistics.hit_rate = statistics.hit_count as f64 / (statistics.hit_count + statistics.miss_count) as f64;
        drop(statistics);
    }

    /// Update statistics on cache miss
    async fn update_statistics_on_miss(&self) {
        let mut statistics = self.statistics.write().await;
        statistics.miss_count += 1;
        statistics.hit_rate = statistics.hit_count as f64 / (statistics.hit_count + statistics.miss_count) as f64;
        drop(statistics);
    }

    /// Update statistics on cache insert
    async fn update_statistics_on_insert(&self) {
        let mut statistics = self.statistics.write().await;
        statistics.total_entries += 1;
        drop(statistics);
    }
}

#[async_trait::async_trait]
impl Service for CachingService {
    async fn health_check(&self) -> AppResult<()> {
        debug!("Performing caching service health check");
        
        // Check if cleanup task is running
        let cleanup_task = self.cleanup_task.read().await;
        if let Some(task) = cleanup_task.as_ref() {
            if task.is_finished() {
                return Err(CacheError::service_error("Cleanup task has stopped".to_string()).into());
            }
        }
        drop(cleanup_task);

        // Check cache sizes
        let config = self.config.read().await;
        let api_cache = self.api_cache.read().await;
        let research_cache = self.research_cache.read().await;
        let db_cache = self.db_cache.read().await;
        
        let total_entries = api_cache.len() + research_cache.len() + db_cache.len();
        if total_entries > config.max_entries {
            warn!("Cache entries ({}) exceed maximum ({})", total_entries, config.max_entries);
        }

        drop(api_cache);
        drop(research_cache);
        drop(db_cache);
        drop(config);

        debug!("Caching service health check completed successfully");
        Ok(())
    }

    async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down caching service...");
        
        // Stop cleanup task
        let mut cleanup_task = self.cleanup_task.write().await;
        if let Some(task) = cleanup_task.take() {
            task.abort();
        }
        drop(cleanup_task);

        // Clear all caches
        self.clear_all_caches().await?;

        info!("Caching service shutdown completed");
        Ok(())
    }
}
