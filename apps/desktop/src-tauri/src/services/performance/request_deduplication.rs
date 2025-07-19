use std::collections::HashMap;
use std::sync::Arc;
use std::hash::{Hash, Hasher};
use tokio::sync::{RwLock, Mutex};
use tracing::{info, debug, warn, error};
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::error::{AppResult, DeduplicationError};
use crate::services::Service;

/// Information about a duplicate request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateRequestInfo {
    pub request_id: Uuid,
    pub original_request_id: Uuid,
    pub request_hash: String,
    pub created_at: DateTime<Utc>,
    pub duplicate_count: u32,
    pub last_duplicate_at: DateTime<Utc>,
}

/// Pending request information
#[derive(Debug, Clone)]
pub struct PendingRequest {
    pub request_id: Uuid,
    pub request_hash: String,
    pub created_at: DateTime<Utc>,
    pub result_sender: tokio::sync::oneshot::Sender<Result<serde_json::Value, String>>,
    pub waiters: Vec<tokio::sync::oneshot::Sender<Result<serde_json::Value, String>>>,
}

/// Request deduplication statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeduplicationStatistics {
    pub total_requests: u64,
    pub duplicate_requests: u64,
    pub deduplication_rate: f64,
    pub active_pending_requests: usize,
    pub total_saved_requests: u64,
    pub average_wait_time_ms: f64,
    pub most_duplicated_requests: Vec<String>,
    pub efficiency_score: f64,
}

/// Request deduplication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeduplicationConfig {
    pub enable_deduplication: bool,
    pub max_pending_requests: usize,
    pub request_timeout_seconds: u64,
    pub cleanup_interval_minutes: i64,
    pub hash_algorithm: HashAlgorithm,
    pub track_statistics: bool,
}

/// Hash algorithms for request deduplication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashAlgorithm {
    DefaultHasher,
    Sha256,
    Blake3,
}

/// Request deduplication service
pub struct RequestDeduplicationService {
    // Pending requests (in-flight)
    pending_requests: Arc<Mutex<HashMap<String, PendingRequest>>>,
    
    // Request history for statistics
    request_history: Arc<RwLock<HashMap<String, DuplicateRequestInfo>>>,
    
    // Configuration
    config: Arc<RwLock<DeduplicationConfig>>,
    
    // Statistics
    statistics: Arc<RwLock<DeduplicationStatistics>>,
    
    // Background cleanup task
    cleanup_task: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

impl RequestDeduplicationService {
    /// Create a new request deduplication service
    pub async fn new() -> AppResult<Self> {
        info!("Initializing request deduplication service...");

        let config = DeduplicationConfig {
            enable_deduplication: true,
            max_pending_requests: 1000,
            request_timeout_seconds: 300, // 5 minutes
            cleanup_interval_minutes: 5,
            hash_algorithm: HashAlgorithm::DefaultHasher,
            track_statistics: true,
        };

        let statistics = DeduplicationStatistics {
            total_requests: 0,
            duplicate_requests: 0,
            deduplication_rate: 0.0,
            active_pending_requests: 0,
            total_saved_requests: 0,
            average_wait_time_ms: 0.0,
            most_duplicated_requests: Vec::new(),
            efficiency_score: 0.0,
        };

        let service = Self {
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
            request_history: Arc::new(RwLock::new(HashMap::new())),
            config: Arc::new(RwLock::new(config)),
            statistics: Arc::new(RwLock::new(statistics)),
            cleanup_task: Arc::new(RwLock::new(None)),
        };

        // Start background cleanup task
        service.start_cleanup_task().await?;

        info!("Request deduplication service initialized successfully");
        Ok(service)
    }

    /// Generate request hash
    pub fn generate_request_hash(&self, request_data: &str) -> String {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        request_data.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Check if request is duplicate and handle accordingly
    pub async fn handle_request<F, Fut>(
        &self,
        request_data: String,
        request_handler: F,
    ) -> AppResult<serde_json::Value>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = AppResult<serde_json::Value>> + Send,
    {
        let config = self.config.read().await;
        if !config.enable_deduplication {
            drop(config);
            return request_handler().await;
        }
        drop(config);

        let request_hash = self.generate_request_hash(&request_data);
        let request_id = Uuid::new_v4();

        // Update total requests statistics
        self.update_statistics_on_request().await;

        // Check if there's already a pending request with the same hash
        let mut pending_requests = self.pending_requests.lock().await;
        
        if let Some(existing_request) = pending_requests.get_mut(&request_hash) {
            // This is a duplicate request
            debug!("Duplicate request detected: {} (original: {})", request_id, existing_request.request_id);
            
            // Update statistics
            self.update_statistics_on_duplicate().await;
            self.update_request_history(&request_hash, request_id).await;

            // Create a oneshot channel to wait for the result
            let (tx, rx) = tokio::sync::oneshot::channel();
            existing_request.waiters.push(tx);
            
            drop(pending_requests);

            // Wait for the original request to complete
            match rx.await {
                Ok(result) => result.map_err(|e| DeduplicationError::request_failed(e).into()),
                Err(_) => Err(DeduplicationError::channel_closed("Original request channel closed".to_string()).into()),
            }
        } else {
            // This is a new request
            debug!("New request: {} (hash: {})", request_id, request_hash);

            let (tx, rx) = tokio::sync::oneshot::channel();
            let pending_request = PendingRequest {
                request_id,
                request_hash: request_hash.clone(),
                created_at: Utc::now(),
                result_sender: tx,
                waiters: Vec::new(),
            };

            pending_requests.insert(request_hash.clone(), pending_request);
            drop(pending_requests);

            // Execute the request in a background task
            let pending_requests_clone = self.pending_requests.clone();
            let request_hash_clone = request_hash.clone();
            
            tokio::spawn(async move {
                let start_time = std::time::Instant::now();
                let result = request_handler().await;
                let processing_time = start_time.elapsed();

                // Get the pending request and notify all waiters
                let mut pending_requests = pending_requests_clone.lock().await;
                if let Some(pending_request) = pending_requests.remove(&request_hash_clone) {
                    let result_json = match &result {
                        Ok(value) => Ok(value.clone()),
                        Err(e) => Err(e.to_string()),
                    };

                    // Send result to original requester
                    let _ = pending_request.result_sender.send(result_json.clone());

                    // Send result to all waiters
                    for waiter in pending_request.waiters {
                        let _ = waiter.send(result_json.clone());
                    }

                    debug!("Request completed: {} (processing time: {:?})", 
                           pending_request.request_id, processing_time);
                }
            });

            // Wait for the result
            match rx.await {
                Ok(result) => result.map_err(|e| DeduplicationError::request_failed(e).into()),
                Err(_) => Err(DeduplicationError::channel_closed("Request channel closed".to_string()).into()),
            }
        }
    }

    /// Get deduplication statistics
    pub async fn get_statistics(&self) -> DeduplicationStatistics {
        let mut statistics = self.statistics.read().await;
        
        // Update active pending requests count
        let pending_requests = self.pending_requests.lock().await;
        let active_count = pending_requests.len();
        drop(pending_requests);

        // Create updated statistics
        let mut updated_stats = statistics.clone();
        updated_stats.active_pending_requests = active_count;
        updated_stats.efficiency_score = self.calculate_efficiency_score(&updated_stats);
        
        updated_stats
    }

    /// Clear request cache
    pub async fn clear_request_cache(&self) -> AppResult<()> {
        info!("Clearing request deduplication cache...");
        
        let mut pending_requests = self.pending_requests.lock().await;
        pending_requests.clear();
        drop(pending_requests);

        let mut request_history = self.request_history.write().await;
        request_history.clear();
        drop(request_history);

        // Reset statistics
        let mut statistics = self.statistics.write().await;
        *statistics = DeduplicationStatistics {
            total_requests: 0,
            duplicate_requests: 0,
            deduplication_rate: 0.0,
            active_pending_requests: 0,
            total_saved_requests: 0,
            average_wait_time_ms: 0.0,
            most_duplicated_requests: Vec::new(),
            efficiency_score: 0.0,
        };
        drop(statistics);

        info!("Request deduplication cache cleared successfully");
        Ok(())
    }

    /// Update statistics on new request
    async fn update_statistics_on_request(&self) {
        let mut statistics = self.statistics.write().await;
        statistics.total_requests += 1;
        statistics.deduplication_rate = if statistics.total_requests > 0 {
            statistics.duplicate_requests as f64 / statistics.total_requests as f64
        } else {
            0.0
        };
        drop(statistics);
    }

    /// Update statistics on duplicate request
    async fn update_statistics_on_duplicate(&self) {
        let mut statistics = self.statistics.write().await;
        statistics.duplicate_requests += 1;
        statistics.total_saved_requests += 1;
        statistics.deduplication_rate = if statistics.total_requests > 0 {
            statistics.duplicate_requests as f64 / statistics.total_requests as f64
        } else {
            0.0
        };
        drop(statistics);
    }

    /// Update request history
    async fn update_request_history(&self, request_hash: &str, request_id: Uuid) {
        let mut request_history = self.request_history.write().await;
        
        if let Some(info) = request_history.get_mut(request_hash) {
            info.duplicate_count += 1;
            info.last_duplicate_at = Utc::now();
        } else {
            let info = DuplicateRequestInfo {
                request_id,
                original_request_id: request_id,
                request_hash: request_hash.to_string(),
                created_at: Utc::now(),
                duplicate_count: 1,
                last_duplicate_at: Utc::now(),
            };
            request_history.insert(request_hash.to_string(), info);
        }
        
        drop(request_history);
    }

    /// Calculate efficiency score
    fn calculate_efficiency_score(&self, stats: &DeduplicationStatistics) -> f64 {
        if stats.total_requests == 0 {
            return 0.0;
        }

        let dedup_score = stats.deduplication_rate * 100.0;
        let saved_ratio = stats.total_saved_requests as f64 / stats.total_requests as f64 * 100.0;
        
        (dedup_score + saved_ratio) / 2.0
    }

    /// Start background cleanup task
    async fn start_cleanup_task(&self) -> AppResult<()> {
        let pending_requests = self.pending_requests.clone();
        let config = self.config.clone();

        let task = tokio::spawn(async move {
            loop {
                let cleanup_interval = {
                    let config = config.read().await;
                    config.cleanup_interval_minutes
                };

                tokio::time::sleep(tokio::time::Duration::from_secs((cleanup_interval * 60) as u64)).await;

                // Cleanup expired pending requests
                let mut pending = pending_requests.lock().await;
                let now = Utc::now();
                let timeout_duration = Duration::seconds(300); // 5 minutes timeout
                
                let mut expired_keys = Vec::new();
                for (key, request) in pending.iter() {
                    if now - request.created_at > timeout_duration {
                        expired_keys.push(key.clone());
                    }
                }

                for key in expired_keys {
                    if let Some(expired_request) = pending.remove(&key) {
                        warn!("Cleaning up expired pending request: {}", expired_request.request_id);
                        
                        // Notify waiters about timeout
                        let timeout_error = Err("Request timeout".to_string());
                        let _ = expired_request.result_sender.send(timeout_error.clone());
                        for waiter in expired_request.waiters {
                            let _ = waiter.send(timeout_error.clone());
                        }
                    }
                }

                drop(pending);
                debug!("Request deduplication cleanup completed");
            }
        });

        let mut cleanup_task = self.cleanup_task.write().await;
        *cleanup_task = Some(task);
        drop(cleanup_task);

        Ok(())
    }
}

#[async_trait::async_trait]
impl Service for RequestDeduplicationService {
    async fn health_check(&self) -> AppResult<()> {
        debug!("Performing request deduplication service health check");
        
        // Check if cleanup task is running
        let cleanup_task = self.cleanup_task.read().await;
        if let Some(task) = cleanup_task.as_ref() {
            if task.is_finished() {
                return Err(DeduplicationError::service_error("Cleanup task has stopped".to_string()).into());
            }
        }
        drop(cleanup_task);

        // Check pending requests count
        let config = self.config.read().await;
        let pending_requests = self.pending_requests.lock().await;
        let pending_count = pending_requests.len();
        
        if pending_count > config.max_pending_requests {
            warn!("Pending requests ({}) exceed maximum ({})", pending_count, config.max_pending_requests);
        }

        drop(pending_requests);
        drop(config);

        debug!("Request deduplication service health check completed successfully");
        Ok(())
    }

    async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down request deduplication service...");
        
        // Stop cleanup task
        let mut cleanup_task = self.cleanup_task.write().await;
        if let Some(task) = cleanup_task.take() {
            task.abort();
        }
        drop(cleanup_task);

        // Clear all pending requests
        self.clear_request_cache().await?;

        info!("Request deduplication service shutdown completed");
        Ok(())
    }
}
