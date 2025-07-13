use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, warn, error};
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, ApiError};
use crate::models::api_key::{ServiceProvider, ApiKey};
use crate::services::DataPersistenceService;

/// Health status for an individual API key
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KeyHealth {
    Healthy,
    Degraded,
    Unhealthy,
    Failed,
    Cooldown,
}

/// Performance metrics for an API key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPerformanceMetrics {
    pub api_key_id: Uuid,
    pub service: ServiceProvider,
    pub total_requests: u32,
    pub successful_requests: u32,
    pub failed_requests: u32,
    pub success_rate: f64,
    pub average_response_time_ms: f64,
    pub last_success: Option<DateTime<Utc>>,
    pub last_failure: Option<DateTime<Utc>>,
    pub consecutive_failures: u32,
    pub health_status: KeyHealth,
    pub priority_score: f64,
    pub last_used: Option<DateTime<Utc>>,
    pub cooldown_until: Option<DateTime<Utc>>,
}

impl KeyPerformanceMetrics {
    /// Create new performance metrics for an API key
    pub fn new(api_key: &ApiKey) -> Self {
        Self {
            api_key_id: api_key.id,
            service: api_key.service.clone(),
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            success_rate: 100.0,
            average_response_time_ms: 0.0,
            last_success: None,
            last_failure: None,
            consecutive_failures: 0,
            health_status: KeyHealth::Healthy,
            priority_score: 100.0,
            last_used: api_key.last_used,
            cooldown_until: None,
        }
    }

    /// Update metrics after a request
    pub fn update_after_request(&mut self, success: bool, response_time_ms: u32) {
        self.total_requests += 1;
        self.last_used = Some(Utc::now());

        if success {
            self.successful_requests += 1;
            self.last_success = Some(Utc::now());
            self.consecutive_failures = 0;
        } else {
            self.failed_requests += 1;
            self.last_failure = Some(Utc::now());
            self.consecutive_failures += 1;
        }

        // Update success rate
        self.success_rate = if self.total_requests > 0 {
            (self.successful_requests as f64 / self.total_requests as f64) * 100.0
        } else {
            100.0
        };

        // Update average response time
        if self.total_requests == 1 {
            self.average_response_time_ms = response_time_ms as f64;
        } else {
            self.average_response_time_ms = 
                (self.average_response_time_ms * (self.total_requests - 1) as f64 + response_time_ms as f64) 
                / self.total_requests as f64;
        }

        // Update health status
        self.update_health_status();
        
        // Update priority score
        self.update_priority_score();
    }

    /// Update health status based on performance metrics
    fn update_health_status(&mut self) {
        // Check if in cooldown
        if let Some(cooldown_until) = self.cooldown_until {
            if Utc::now() < cooldown_until {
                self.health_status = KeyHealth::Cooldown;
                return;
            } else {
                self.cooldown_until = None;
            }
        }

        // Determine health based on metrics
        if self.consecutive_failures >= 5 {
            self.health_status = KeyHealth::Failed;
            // Set cooldown for 30 minutes
            self.cooldown_until = Some(Utc::now() + Duration::minutes(30));
        } else if self.consecutive_failures >= 3 || self.success_rate < 50.0 {
            self.health_status = KeyHealth::Unhealthy;
        } else if self.success_rate < 80.0 || self.average_response_time_ms > 5000.0 {
            self.health_status = KeyHealth::Degraded;
        } else {
            self.health_status = KeyHealth::Healthy;
        }
    }

    /// Update priority score for rotation selection
    fn update_priority_score(&mut self) {
        let mut score = 100.0;

        // Success rate factor (0-40 points)
        score += (self.success_rate - 50.0) * 0.4;

        // Response time factor (0-20 points)
        let response_time_score = if self.average_response_time_ms > 0.0 {
            (5000.0 - self.average_response_time_ms.min(5000.0)) / 5000.0 * 20.0
        } else {
            20.0
        };
        score += response_time_score;

        // Recency factor (0-20 points) - prefer less recently used keys
        if let Some(last_used) = self.last_used {
            let hours_since_use = (Utc::now() - last_used).num_hours() as f64;
            let recency_score = (hours_since_use / 24.0).min(1.0) * 20.0;
            score += recency_score;
        } else {
            score += 20.0; // Never used gets full points
        }

        // Health penalty
        match self.health_status {
            KeyHealth::Healthy => {},
            KeyHealth::Degraded => score -= 20.0,
            KeyHealth::Unhealthy => score -= 40.0,
            KeyHealth::Failed => score -= 80.0,
            KeyHealth::Cooldown => score -= 100.0,
        }

        self.priority_score = score.max(0.0);
    }

    /// Check if key is available for use
    pub fn is_available(&self) -> bool {
        match self.health_status {
            KeyHealth::Healthy | KeyHealth::Degraded => true,
            KeyHealth::Unhealthy => self.consecutive_failures < 3,
            KeyHealth::Failed | KeyHealth::Cooldown => false,
        }
    }

    /// Check if key needs cooldown
    pub fn needs_cooldown(&self) -> bool {
        self.consecutive_failures >= 5
    }
}

/// Rotation strategy for key selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RotationStrategy {
    RoundRobin,
    PriorityBased,
    LeastRecentlyUsed,
    HealthAware,
    LoadBalanced,
}

/// Configuration for key rotation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationConfig {
    pub strategy: RotationStrategy,
    pub health_check_interval_minutes: u32,
    pub cooldown_duration_minutes: u32,
    pub max_consecutive_failures: u32,
    pub min_success_rate_threshold: f64,
    pub max_response_time_threshold_ms: u32,
    pub enable_automatic_reactivation: bool,
    pub load_balancing_weight_factor: f64,
}

impl Default for RotationConfig {
    fn default() -> Self {
        Self {
            strategy: RotationStrategy::HealthAware,
            health_check_interval_minutes: 5,
            cooldown_duration_minutes: 30,
            max_consecutive_failures: 5,
            min_success_rate_threshold: 80.0,
            max_response_time_threshold_ms: 5000,
            enable_automatic_reactivation: true,
            load_balancing_weight_factor: 1.0,
        }
    }
}

/// Key rotation analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationAnalytics {
    pub total_rotations: u32,
    pub successful_rotations: u32,
    pub failed_rotations: u32,
    pub average_rotation_time_ms: f64,
    pub keys_in_cooldown: u32,
    pub keys_healthy: u32,
    pub keys_degraded: u32,
    pub keys_unhealthy: u32,
    pub keys_failed: u32,
    pub last_rotation: Option<DateTime<Utc>>,
    pub rotation_frequency_per_hour: f64,
}

/// Intelligent key rotator for managing API key selection and health
pub struct KeyRotator {
    data_persistence: Arc<RwLock<DataPersistenceService>>,
    performance_metrics: Arc<RwLock<HashMap<Uuid, KeyPerformanceMetrics>>>,
    rotation_config: Arc<RwLock<HashMap<ServiceProvider, RotationConfig>>>,
    analytics: Arc<RwLock<RotationAnalytics>>,
    last_selected_key: Arc<RwLock<HashMap<ServiceProvider, Uuid>>>,
}

impl KeyRotator {
    /// Create a new key rotator
    pub async fn new(data_persistence: Arc<RwLock<DataPersistenceService>>) -> AppResult<Self> {
        info!("Initializing key rotator...");

        let mut rotation_config = HashMap::new();
        
        // Initialize default configurations for all services
        for service in [
            ServiceProvider::OpenRouter,
            ServiceProvider::SerpApi,
            ServiceProvider::Jina,
            ServiceProvider::Firecrawl,
            ServiceProvider::Tavily,
            ServiceProvider::Exa,
        ] {
            rotation_config.insert(service, RotationConfig::default());
        }

        let analytics = RotationAnalytics {
            total_rotations: 0,
            successful_rotations: 0,
            failed_rotations: 0,
            average_rotation_time_ms: 0.0,
            keys_in_cooldown: 0,
            keys_healthy: 0,
            keys_degraded: 0,
            keys_unhealthy: 0,
            keys_failed: 0,
            last_rotation: None,
            rotation_frequency_per_hour: 0.0,
        };

        let rotator = Self {
            data_persistence,
            performance_metrics: Arc::new(RwLock::new(HashMap::new())),
            rotation_config: Arc::new(RwLock::new(rotation_config)),
            analytics: Arc::new(RwLock::new(analytics)),
            last_selected_key: Arc::new(RwLock::new(HashMap::new())),
        };

        // Initialize performance metrics for existing keys
        rotator.initialize_performance_metrics().await?;

        info!("Key rotator initialized successfully");
        Ok(rotator)
    }

    /// Initialize performance metrics for existing API keys
    async fn initialize_performance_metrics(&self) -> AppResult<()> {
        debug!("Initializing performance metrics for existing API keys");

        let data_persistence = self.data_persistence.read().await;
        let api_keys = data_persistence.get_all_api_keys().await?;
        drop(data_persistence);

        let mut metrics = self.performance_metrics.write().await;
        
        for api_key in api_keys {
            if !metrics.contains_key(&api_key.id) {
                metrics.insert(api_key.id, KeyPerformanceMetrics::new(&api_key));
            }
        }

        debug!("Initialized performance metrics for {} API keys", metrics.len());
        Ok(())
    }

    /// Select the best available API key for a service using intelligent rotation
    pub async fn select_best_key(&self, service: ServiceProvider) -> AppResult<Option<ApiKey>> {
        debug!("Selecting best API key for service: {:?}", service);

        let start_time = std::time::Instant::now();

        // Get all keys for the service
        let data_persistence = self.data_persistence.read().await;
        let all_keys = data_persistence.get_all_api_keys().await?;
        drop(data_persistence);

        let service_keys: Vec<_> = all_keys.into_iter()
            .filter(|key| key.service == service && key.is_available())
            .collect();

        if service_keys.is_empty() {
            debug!("No available keys found for service: {:?}", service);
            return Ok(None);
        }

        // Get performance metrics and rotation config
        let metrics = self.performance_metrics.read().await;
        let config = self.rotation_config.read().await;
        let rotation_config = config.get(&service).unwrap_or(&RotationConfig::default());

        // Filter keys based on health and availability
        let mut available_keys: Vec<_> = service_keys.into_iter()
            .filter_map(|key| {
                if let Some(key_metrics) = metrics.get(&key.id) {
                    if key_metrics.is_available() {
                        Some((key, key_metrics.clone()))
                    } else {
                        None
                    }
                } else {
                    // No metrics yet, consider it healthy
                    Some((key, KeyPerformanceMetrics::new(&key)))
                }
            })
            .collect();

        drop(metrics);
        drop(config);

        if available_keys.is_empty() {
            debug!("No healthy keys available for service: {:?}", service);
            return Ok(None);
        }

        // Select key based on rotation strategy
        let selected_key = match rotation_config.strategy {
            RotationStrategy::RoundRobin => self.select_round_robin(&mut available_keys, service).await,
            RotationStrategy::PriorityBased => self.select_priority_based(&mut available_keys),
            RotationStrategy::LeastRecentlyUsed => self.select_least_recently_used(&mut available_keys),
            RotationStrategy::HealthAware => self.select_health_aware(&mut available_keys),
            RotationStrategy::LoadBalanced => self.select_load_balanced(&mut available_keys),
        };

        // Update analytics
        let rotation_time = start_time.elapsed().as_millis() as f64;
        self.update_rotation_analytics(selected_key.is_some(), rotation_time).await;

        // Update last selected key
        if let Some(ref key) = selected_key {
            let mut last_selected = self.last_selected_key.write().await;
            last_selected.insert(service, key.id);
        }

        debug!("Selected key for service {:?}: {:?}", service,
               selected_key.as_ref().map(|k| k.id));

        Ok(selected_key)
    }

    /// Round-robin selection
    async fn select_round_robin(&self, keys: &mut [(ApiKey, KeyPerformanceMetrics)], service: ServiceProvider) -> Option<ApiKey> {
        let last_selected = self.last_selected_key.read().await;

        if let Some(last_key_id) = last_selected.get(&service) {
            // Find the next key after the last selected one
            if let Some(current_index) = keys.iter().position(|(key, _)| key.id == *last_key_id) {
                let next_index = (current_index + 1) % keys.len();
                return Some(keys[next_index].0.clone());
            }
        }

        // If no last selected key or key not found, return the first one
        keys.first().map(|(key, _)| key.clone())
    }

    /// Priority-based selection (highest priority score)
    fn select_priority_based(&self, keys: &mut [(ApiKey, KeyPerformanceMetrics)]) -> Option<ApiKey> {
        keys.sort_by(|a, b| b.1.priority_score.partial_cmp(&a.1.priority_score).unwrap());
        keys.first().map(|(key, _)| key.clone())
    }

    /// Least recently used selection
    fn select_least_recently_used(&self, keys: &mut [(ApiKey, KeyPerformanceMetrics)]) -> Option<ApiKey> {
        keys.sort_by(|a, b| {
            match (a.1.last_used, b.1.last_used) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Less, // Never used comes first
                (Some(_), None) => std::cmp::Ordering::Greater,
                (Some(a_time), Some(b_time)) => a_time.cmp(&b_time), // Earlier time comes first
            }
        });
        keys.first().map(|(key, _)| key.clone())
    }

    /// Health-aware selection (best health + priority)
    fn select_health_aware(&self, keys: &mut [(ApiKey, KeyPerformanceMetrics)]) -> Option<ApiKey> {
        // Sort by health status first, then by priority score
        keys.sort_by(|a, b| {
            let health_order = |health: &KeyHealth| match health {
                KeyHealth::Healthy => 0,
                KeyHealth::Degraded => 1,
                KeyHealth::Unhealthy => 2,
                KeyHealth::Failed => 3,
                KeyHealth::Cooldown => 4,
            };

            let health_cmp = health_order(&a.1.health_status).cmp(&health_order(&b.1.health_status));
            if health_cmp == std::cmp::Ordering::Equal {
                b.1.priority_score.partial_cmp(&a.1.priority_score).unwrap()
            } else {
                health_cmp
            }
        });

        keys.first().map(|(key, _)| key.clone())
    }

    /// Load-balanced selection (weighted by inverse usage)
    fn select_load_balanced(&self, keys: &mut [(ApiKey, KeyPerformanceMetrics)]) -> Option<ApiKey> {
        if keys.is_empty() {
            return None;
        }

        // Calculate weights based on inverse usage
        let total_requests: u32 = keys.iter().map(|(_, metrics)| metrics.total_requests).sum();

        if total_requests == 0 {
            // No usage data, use round-robin
            return keys.first().map(|(key, _)| key.clone());
        }

        // Calculate selection weights (higher weight for less used keys)
        let weights: Vec<f64> = keys.iter().map(|(_, metrics)| {
            let usage_ratio = metrics.total_requests as f64 / total_requests as f64;
            let weight = 1.0 - usage_ratio + 0.1; // Add small base weight
            weight * metrics.priority_score / 100.0 // Factor in priority
        }).collect();

        // Weighted random selection
        let total_weight: f64 = weights.iter().sum();
        let mut random_value = rand::random::<f64>() * total_weight;

        for (i, weight) in weights.iter().enumerate() {
            random_value -= weight;
            if random_value <= 0.0 {
                return Some(keys[i].0.clone());
            }
        }

        // Fallback to first key
        keys.first().map(|(key, _)| key.clone())
    }

    /// Record request performance and update metrics
    pub async fn record_request_performance(&self, api_key_id: Uuid, success: bool, response_time_ms: u32) -> AppResult<()> {
        debug!("Recording request performance for key: {} (success: {}, time: {}ms)",
               api_key_id, success, response_time_ms);

        let mut metrics = self.performance_metrics.write().await;

        if let Some(key_metrics) = metrics.get_mut(&api_key_id) {
            key_metrics.update_after_request(success, response_time_ms);
        } else {
            // Create new metrics if not found
            let data_persistence = self.data_persistence.read().await;
            if let Some(api_key) = data_persistence.get_api_key_by_id(api_key_id).await? {
                let mut new_metrics = KeyPerformanceMetrics::new(&api_key);
                new_metrics.update_after_request(success, response_time_ms);
                metrics.insert(api_key_id, new_metrics);
            }
            drop(data_persistence);
        }

        debug!("Updated performance metrics for key: {}", api_key_id);
        Ok(())
    }

    /// Update rotation analytics
    async fn update_rotation_analytics(&self, success: bool, rotation_time_ms: f64) {
        let mut analytics = self.analytics.write().await;

        analytics.total_rotations += 1;
        if success {
            analytics.successful_rotations += 1;
        } else {
            analytics.failed_rotations += 1;
        }

        // Update average rotation time
        if analytics.total_rotations == 1 {
            analytics.average_rotation_time_ms = rotation_time_ms;
        } else {
            analytics.average_rotation_time_ms =
                (analytics.average_rotation_time_ms * (analytics.total_rotations - 1) as f64 + rotation_time_ms)
                / analytics.total_rotations as f64;
        }

        analytics.last_rotation = Some(Utc::now());

        // Update health statistics
        let metrics = self.performance_metrics.read().await;
        analytics.keys_healthy = metrics.values().filter(|m| m.health_status == KeyHealth::Healthy).count() as u32;
        analytics.keys_degraded = metrics.values().filter(|m| m.health_status == KeyHealth::Degraded).count() as u32;
        analytics.keys_unhealthy = metrics.values().filter(|m| m.health_status == KeyHealth::Unhealthy).count() as u32;
        analytics.keys_failed = metrics.values().filter(|m| m.health_status == KeyHealth::Failed).count() as u32;
        analytics.keys_in_cooldown = metrics.values().filter(|m| m.health_status == KeyHealth::Cooldown).count() as u32;
        drop(metrics);

        // Calculate rotation frequency (rotations per hour)
        if let Some(last_rotation) = analytics.last_rotation {
            let hours_since_start = (last_rotation - (last_rotation - Duration::hours(1))).num_minutes() as f64 / 60.0;
            if hours_since_start > 0.0 {
                analytics.rotation_frequency_per_hour = analytics.total_rotations as f64 / hours_since_start;
            }
        }
    }

    /// Get performance metrics for an API key
    pub async fn get_key_performance(&self, api_key_id: Uuid) -> Option<KeyPerformanceMetrics> {
        let metrics = self.performance_metrics.read().await;
        metrics.get(&api_key_id).cloned()
    }

    /// Get performance metrics for all keys of a service
    pub async fn get_service_performance(&self, service: ServiceProvider) -> Vec<KeyPerformanceMetrics> {
        let metrics = self.performance_metrics.read().await;
        metrics.values()
            .filter(|m| m.service == service)
            .cloned()
            .collect()
    }

    /// Get all performance metrics
    pub async fn get_all_performance_metrics(&self) -> HashMap<Uuid, KeyPerformanceMetrics> {
        self.performance_metrics.read().await.clone()
    }

    /// Get rotation analytics
    pub async fn get_rotation_analytics(&self) -> RotationAnalytics {
        self.analytics.read().await.clone()
    }

    /// Update rotation configuration for a service
    pub async fn update_rotation_config(&self, service: ServiceProvider, config: RotationConfig) -> AppResult<()> {
        let mut configs = self.rotation_config.write().await;
        configs.insert(service, config);
        info!("Updated rotation configuration for service: {:?}", service);
        Ok(())
    }

    /// Get rotation configuration for a service
    pub async fn get_rotation_config(&self, service: ServiceProvider) -> RotationConfig {
        let configs = self.rotation_config.read().await;
        configs.get(&service).cloned().unwrap_or_default()
    }

    /// Perform health check on all API keys
    pub async fn perform_health_check(&self) -> AppResult<HashMap<Uuid, KeyHealth>> {
        debug!("Performing health check on all API keys");

        let mut health_status = HashMap::new();
        let mut metrics = self.performance_metrics.write().await;

        for (key_id, key_metrics) in metrics.iter_mut() {
            // Update health status based on current metrics
            key_metrics.update_health_status();
            health_status.insert(*key_id, key_metrics.health_status.clone());

            // Log health changes
            match key_metrics.health_status {
                KeyHealth::Failed => {
                    warn!("API key {} marked as failed (consecutive failures: {})",
                          key_id, key_metrics.consecutive_failures);
                }
                KeyHealth::Unhealthy => {
                    warn!("API key {} marked as unhealthy (success rate: {:.1}%)",
                          key_id, key_metrics.success_rate);
                }
                KeyHealth::Degraded => {
                    info!("API key {} marked as degraded (avg response time: {:.1}ms)",
                          key_id, key_metrics.average_response_time_ms);
                }
                _ => {}
            }
        }

        debug!("Health check completed for {} API keys", health_status.len());
        Ok(health_status)
    }

    /// Reactivate keys that have completed their cooldown
    pub async fn reactivate_cooled_down_keys(&self) -> AppResult<Vec<Uuid>> {
        debug!("Checking for keys to reactivate from cooldown");

        let mut reactivated_keys = Vec::new();
        let mut metrics = self.performance_metrics.write().await;

        for (key_id, key_metrics) in metrics.iter_mut() {
            if key_metrics.health_status == KeyHealth::Cooldown {
                if let Some(cooldown_until) = key_metrics.cooldown_until {
                    if Utc::now() >= cooldown_until {
                        key_metrics.cooldown_until = None;
                        key_metrics.consecutive_failures = 0;
                        key_metrics.health_status = KeyHealth::Healthy;
                        key_metrics.update_priority_score();
                        reactivated_keys.push(*key_id);

                        info!("Reactivated API key {} from cooldown", key_id);
                    }
                }
            }
        }

        if !reactivated_keys.is_empty() {
            info!("Reactivated {} API keys from cooldown", reactivated_keys.len());
        }

        Ok(reactivated_keys)
    }

    /// Get keys that need attention (unhealthy, failed, or in cooldown)
    pub async fn get_keys_needing_attention(&self) -> Vec<(Uuid, KeyPerformanceMetrics)> {
        let metrics = self.performance_metrics.read().await;
        metrics.iter()
            .filter(|(_, m)| matches!(m.health_status, KeyHealth::Unhealthy | KeyHealth::Failed | KeyHealth::Cooldown))
            .map(|(id, m)| (*id, m.clone()))
            .collect()
    }

    /// Generate rotation report
    pub async fn generate_rotation_report(&self) -> AppResult<String> {
        debug!("Generating rotation report");

        let analytics = self.get_rotation_analytics().await;
        let all_metrics = self.get_all_performance_metrics().await;

        let mut report = String::new();
        report.push_str("# Key Rotation Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

        // Rotation statistics
        report.push_str("## Rotation Statistics\n\n");
        report.push_str(&format!("- Total Rotations: {}\n", analytics.total_rotations));
        report.push_str(&format!("- Successful Rotations: {} ({:.1}%)\n",
            analytics.successful_rotations,
            if analytics.total_rotations > 0 {
                (analytics.successful_rotations as f64 / analytics.total_rotations as f64) * 100.0
            } else { 0.0 }));
        report.push_str(&format!("- Average Rotation Time: {:.2}ms\n", analytics.average_rotation_time_ms));
        report.push_str(&format!("- Rotation Frequency: {:.1} per hour\n", analytics.rotation_frequency_per_hour));
        report.push_str("\n");

        // Health summary
        report.push_str("## Health Summary\n\n");
        report.push_str(&format!("- Healthy Keys: {}\n", analytics.keys_healthy));
        report.push_str(&format!("- Degraded Keys: {}\n", analytics.keys_degraded));
        report.push_str(&format!("- Unhealthy Keys: {}\n", analytics.keys_unhealthy));
        report.push_str(&format!("- Failed Keys: {}\n", analytics.keys_failed));
        report.push_str(&format!("- Keys in Cooldown: {}\n", analytics.keys_in_cooldown));
        report.push_str("\n");

        // Performance details
        report.push_str("## Key Performance Details\n\n");
        let mut sorted_metrics: Vec<_> = all_metrics.values().collect();
        sorted_metrics.sort_by(|a, b| b.priority_score.partial_cmp(&a.priority_score).unwrap());

        for metrics in sorted_metrics.iter().take(20) {
            report.push_str(&format!("### Key {} ({:?})\n",
                metrics.api_key_id.to_string().chars().take(8).collect::<String>(),
                metrics.service));
            report.push_str(&format!("- Health: {:?}\n", metrics.health_status));
            report.push_str(&format!("- Success Rate: {:.1}%\n", metrics.success_rate));
            report.push_str(&format!("- Average Response Time: {:.1}ms\n", metrics.average_response_time_ms));
            report.push_str(&format!("- Total Requests: {}\n", metrics.total_requests));
            report.push_str(&format!("- Priority Score: {:.1}\n", metrics.priority_score));
            if let Some(last_used) = metrics.last_used {
                report.push_str(&format!("- Last Used: {}\n", last_used.format("%Y-%m-%d %H:%M:%S UTC")));
            }
            report.push_str("\n");
        }

        info!("Generated rotation report with {} keys", all_metrics.len());
        Ok(report)
    }

    /// Start background health monitoring tasks
    pub async fn start_background_tasks(&self) -> AppResult<()> {
        info!("Starting key rotator background tasks...");

        // Health check task
        let rotator_health = Arc::new(self.clone());
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // Every 5 minutes

            loop {
                interval.tick().await;

                if let Err(e) = rotator_health.perform_health_check().await {
                    error!("Failed to perform health check: {}", e);
                }

                if let Err(e) = rotator_health.reactivate_cooled_down_keys().await {
                    error!("Failed to reactivate cooled down keys: {}", e);
                }
            }
        });

        info!("Key rotator background tasks started successfully");
        Ok(())
    }
}

// Implement Clone for KeyRotator to enable Arc sharing
impl Clone for KeyRotator {
    fn clone(&self) -> Self {
        Self {
            data_persistence: self.data_persistence.clone(),
            performance_metrics: self.performance_metrics.clone(),
            rotation_config: self.rotation_config.clone(),
            analytics: self.analytics.clone(),
            last_selected_key: self.last_selected_key.clone(),
        }
    }
}
