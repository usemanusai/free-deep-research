use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error, warn};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

use crate::error::{AppError, AppResult};
use super::{TimePeriod, AnalyticsEvent, EventType, UsageAnalyticsData};

/// Usage analytics engine for tracking and analyzing system usage patterns
#[derive(Clone)]
pub struct UsageAnalyticsEngine {
    metrics_collector: Arc<RwLock<super::metrics_collector::MetricsCollector>>,
    data_persistence: Arc<RwLock<crate::services::data_persistence::DataPersistenceService>>,
    config: UsageAnalyticsConfig,
    usage_cache: Arc<RwLock<UsageCache>>,
}

impl UsageAnalyticsEngine {
    /// Create a new usage analytics engine
    pub async fn new(
        metrics_collector: Arc<RwLock<super::metrics_collector::MetricsCollector>>,
        data_persistence: Arc<RwLock<crate::services::data_persistence::DataPersistenceService>>,
    ) -> AppResult<Self> {
        info!("Initializing usage analytics engine...");

        let config = UsageAnalyticsConfig::default();
        let usage_cache = Arc::new(RwLock::new(UsageCache::new()));

        let engine = Self {
            metrics_collector,
            data_persistence,
            config,
            usage_cache,
        };

        // Load historical data
        engine.load_historical_data().await?;

        info!("Usage analytics engine initialized successfully");
        Ok(engine)
    }

    /// Start usage analytics processing
    pub async fn start_processing(&self) -> AppResult<()> {
        info!("Starting usage analytics processing...");

        // Start background processing tasks
        let engine_clone = self.clone();
        tokio::spawn(async move {
            engine_clone.process_usage_analytics().await;
        });

        let engine_clone = self.clone();
        tokio::spawn(async move {
            engine_clone.generate_usage_reports().await;
        });

        let engine_clone = self.clone();
        tokio::spawn(async move {
            engine_clone.calculate_cost_savings().await;
        });

        info!("Usage analytics processing started successfully");
        Ok(())
    }

    /// Get usage analytics data for a specific time period
    pub async fn get_analytics_data(&self, period: TimePeriod) -> AppResult<UsageAnalyticsData> {
        info!("Retrieving usage analytics data for period: {:?}", period);

        let (start_time, end_time) = self.get_time_range(&period);
        
        // Get usage data from cache and database
        let usage_cache = self.usage_cache.read().await;
        let cached_data = usage_cache.get_cached_data(&period);

        if let Some(data) = cached_data {
            if data.last_updated > Utc::now() - Duration::minutes(5) {
                return Ok(data.analytics_data);
            }
        }

        // Generate fresh analytics data
        drop(usage_cache);
        let analytics_data = self.generate_analytics_data(start_time, end_time).await?;

        // Update cache
        let mut usage_cache = self.usage_cache.write().await;
        usage_cache.update_cache(period, analytics_data.clone());

        Ok(analytics_data)
    }

    /// Generate comprehensive usage analytics data
    async fn generate_analytics_data(&self, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> AppResult<UsageAnalyticsData> {
        // Get research session statistics
        let total_research_sessions = self.count_research_sessions(start_time, end_time).await?;

        // Get methodology usage statistics
        let methodology_usage = self.get_methodology_usage(start_time, end_time).await?;

        // Get API usage statistics
        let api_usage_stats = self.get_api_usage_stats(start_time, end_time).await?;

        // Calculate cost savings analysis
        let cost_savings = self.calculate_cost_savings_analysis(start_time, end_time).await?;

        // Generate usage trends
        let usage_trends = self.generate_usage_trends(start_time, end_time).await?;

        // Identify peak usage times
        let peak_usage_times = self.identify_peak_usage_times(start_time, end_time).await?;

        Ok(UsageAnalyticsData {
            total_research_sessions,
            methodology_usage,
            api_usage_stats,
            cost_savings,
            usage_trends,
            peak_usage_times,
        })
    }

    /// Count research sessions in time period
    async fn count_research_sessions(&self, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> AppResult<u64> {
        let metrics_collector = self.metrics_collector.read().await;
        let events = metrics_collector.get_events_by_type_and_time(
            EventType::ResearchStarted,
            start_time,
            end_time,
        ).await?;
        Ok(events.len() as u64)
    }

    /// Get methodology usage statistics
    async fn get_methodology_usage(&self, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> AppResult<HashMap<String, u64>> {
        let metrics_collector = self.metrics_collector.read().await;
        let events = metrics_collector.get_events_by_type_and_time(
            EventType::MethodologySelected,
            start_time,
            end_time,
        ).await?;

        let mut methodology_counts = HashMap::new();
        for event in events {
            if let Some(methodology) = event.metadata.get("methodology") {
                if let Some(methodology_str) = methodology.as_str() {
                    *methodology_counts.entry(methodology_str.to_string()).or_insert(0) += 1;
                }
            }
        }

        Ok(methodology_counts)
    }

    /// Get API usage statistics
    async fn get_api_usage_stats(&self, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> AppResult<HashMap<String, ApiUsageStats>> {
        let metrics_collector = self.metrics_collector.read().await;
        let api_events = metrics_collector.get_events_by_type_and_time(
            EventType::ApiCallMade,
            start_time,
            end_time,
        ).await?;

        let mut api_stats = HashMap::new();
        for event in api_events {
            if let Some(service) = event.metadata.get("service") {
                if let Some(service_str) = service.as_str() {
                    let stats = api_stats.entry(service_str.to_string()).or_insert(ApiUsageStats::default());
                    stats.total_calls += 1;
                    
                    if let Some(response_time) = event.metadata.get("response_time") {
                        if let Some(response_time_ms) = response_time.as_f64() {
                            stats.total_response_time += response_time_ms;
                            stats.average_response_time = stats.total_response_time / stats.total_calls as f64;
                        }
                    }

                    if let Some(success) = event.metadata.get("success") {
                        if let Some(success_bool) = success.as_bool() {
                            if success_bool {
                                stats.successful_calls += 1;
                            } else {
                                stats.failed_calls += 1;
                            }
                        }
                    }
                }
            }
        }

        // Calculate success rates
        for stats in api_stats.values_mut() {
            if stats.total_calls > 0 {
                stats.success_rate = (stats.successful_calls as f64 / stats.total_calls as f64) * 100.0;
            }
        }

        Ok(api_stats)
    }

    /// Calculate cost savings analysis
    async fn calculate_cost_savings_analysis(&self, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> AppResult<CostSavingsAnalysis> {
        let api_usage_stats = self.get_api_usage_stats(start_time, end_time).await?;
        
        let mut total_theoretical_cost = 0.0;
        let mut service_costs = HashMap::new();

        // Calculate theoretical costs based on commercial pricing
        for (service, stats) in &api_usage_stats {
            let cost_per_call = self.get_theoretical_cost_per_call(service);
            let service_cost = stats.total_calls as f64 * cost_per_call;
            total_theoretical_cost += service_cost;
            service_costs.insert(service.clone(), service_cost);
        }

        // Calculate savings compared to commercial alternatives
        let commercial_alternative_cost = self.calculate_commercial_alternative_cost(&api_usage_stats).await?;
        let savings_vs_commercial = commercial_alternative_cost - 0.0; // We operate at zero cost

        Ok(CostSavingsAnalysis {
            total_theoretical_cost,
            actual_cost: 0.0, // Free tier usage
            total_savings: total_theoretical_cost,
            service_costs,
            savings_vs_commercial,
            cost_per_research_session: if api_usage_stats.len() > 0 { 
                total_theoretical_cost / api_usage_stats.len() as f64 
            } else { 
                0.0 
            },
        })
    }

    /// Generate usage trends over time
    async fn generate_usage_trends(&self, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> AppResult<Vec<UsageTrend>> {
        let mut trends = Vec::new();
        let duration = end_time - start_time;
        let interval = duration / 24; // 24 data points

        for i in 0..24 {
            let period_start = start_time + (interval * i as i32);
            let period_end = start_time + (interval * (i + 1) as i32);
            
            let session_count = self.count_research_sessions(period_start, period_end).await?;
            let api_calls = self.count_api_calls(period_start, period_end).await?;

            trends.push(UsageTrend {
                timestamp: period_start,
                research_sessions: session_count,
                api_calls,
                active_users: self.count_active_users(period_start, period_end).await?,
            });
        }

        Ok(trends)
    }

    /// Identify peak usage times
    async fn identify_peak_usage_times(&self, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> AppResult<Vec<PeakUsageTime>> {
        let trends = self.generate_usage_trends(start_time, end_time).await?;
        let mut peak_times = Vec::new();

        // Find peaks in usage
        for (i, trend) in trends.iter().enumerate() {
            if i > 0 && i < trends.len() - 1 {
                let prev = &trends[i - 1];
                let next = &trends[i + 1];
                
                if trend.research_sessions > prev.research_sessions && trend.research_sessions > next.research_sessions {
                    peak_times.push(PeakUsageTime {
                        timestamp: trend.timestamp,
                        usage_level: trend.research_sessions,
                        peak_type: PeakType::ResearchSessions,
                    });
                }
            }
        }

        Ok(peak_times)
    }

    /// Count API calls in time period
    async fn count_api_calls(&self, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> AppResult<u64> {
        let metrics_collector = self.metrics_collector.read().await;
        let events = metrics_collector.get_events_by_type_and_time(
            EventType::ApiCallMade,
            start_time,
            end_time,
        ).await?;
        Ok(events.len() as u64)
    }

    /// Count active users in time period
    async fn count_active_users(&self, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> AppResult<u64> {
        let metrics_collector = self.metrics_collector.read().await;
        let events = metrics_collector.get_events_by_time(start_time, end_time).await?;
        
        let mut unique_sessions = std::collections::HashSet::new();
        for event in events {
            unique_sessions.insert(event.session_id.clone());
        }
        
        Ok(unique_sessions.len() as u64)
    }

    /// Get theoretical cost per API call for a service
    fn get_theoretical_cost_per_call(&self, service: &str) -> f64 {
        match service {
            "openai" => 0.002, // Approximate cost per API call
            "anthropic" => 0.003,
            "serpapi" => 0.001,
            "firecrawl" => 0.0005,
            "jina" => 0.0002,
            _ => 0.001, // Default cost
        }
    }

    /// Calculate commercial alternative cost
    async fn calculate_commercial_alternative_cost(&self, _api_usage_stats: &HashMap<String, ApiUsageStats>) -> AppResult<f64> {
        // Estimate cost of commercial alternatives like OpenAI's Deep Research
        // Based on typical pricing of $200/month for similar services
        Ok(200.0)
    }

    /// Load historical usage data
    async fn load_historical_data(&self) -> AppResult<()> {
        info!("Loading historical usage data...");
        // Implementation would load from persistent storage
        Ok(())
    }

    /// Background task for processing usage analytics
    async fn process_usage_analytics(&self) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // 5 minutes
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.update_usage_analytics().await {
                error!("Failed to update usage analytics: {}", e);
            }
        }
    }

    /// Update usage analytics
    async fn update_usage_analytics(&self) -> AppResult<()> {
        // Clear old cache entries
        let mut usage_cache = self.usage_cache.write().await;
        usage_cache.cleanup_old_entries();
        Ok(())
    }

    /// Background task for generating usage reports
    async fn generate_usage_reports(&self) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600)); // 1 hour
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.generate_hourly_report().await {
                error!("Failed to generate hourly usage report: {}", e);
            }
        }
    }

    /// Generate hourly usage report
    async fn generate_hourly_report(&self) -> AppResult<()> {
        info!("Generating hourly usage report...");
        // Implementation would generate and store hourly reports
        Ok(())
    }

    /// Background task for calculating cost savings
    async fn calculate_cost_savings(&self) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1800)); // 30 minutes
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.update_cost_savings().await {
                error!("Failed to update cost savings: {}", e);
            }
        }
    }

    /// Update cost savings calculations
    async fn update_cost_savings(&self) -> AppResult<()> {
        info!("Updating cost savings calculations...");
        // Implementation would update cost savings metrics
        Ok(())
    }

    /// Get time range for a time period
    fn get_time_range(&self, period: &TimePeriod) -> (DateTime<Utc>, DateTime<Utc>) {
        let end_time = Utc::now();
        let start_time = match period {
            TimePeriod::LastHour => end_time - Duration::hours(1),
            TimePeriod::Last24Hours => end_time - Duration::days(1),
            TimePeriod::LastWeek => end_time - Duration::weeks(1),
            TimePeriod::LastMonth => end_time - Duration::days(30),
            TimePeriod::LastQuarter => end_time - Duration::days(90),
            TimePeriod::LastYear => end_time - Duration::days(365),
            TimePeriod::Custom { start, end: _ } => *start,
        };
        (start_time, end_time)
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: &UsageAnalyticsConfig) -> AppResult<()> {
        info!("Updating usage analytics configuration");
        self.config = config.clone();
        Ok(())
    }

    /// Health check
    pub async fn health_check(&self) -> AppResult<()> {
        // Check if metrics collector is accessible
        let _metrics_collector = self.metrics_collector.read().await;
        Ok(())
    }

    /// Shutdown
    pub async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down usage analytics engine...");
        Ok(())
    }
}

/// Usage analytics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageAnalyticsConfig {
    pub collection_interval_seconds: u64,
    pub report_generation_interval_hours: u64,
    pub cache_duration_minutes: u64,
    pub detailed_tracking_enabled: bool,
    pub cost_analysis_enabled: bool,
}

impl Default for UsageAnalyticsConfig {
    fn default() -> Self {
        Self {
            collection_interval_seconds: 300, // 5 minutes
            report_generation_interval_hours: 1,
            cache_duration_minutes: 5,
            detailed_tracking_enabled: true,
            cost_analysis_enabled: true,
        }
    }
}

/// API usage statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiUsageStats {
    pub total_calls: u64,
    pub successful_calls: u64,
    pub failed_calls: u64,
    pub success_rate: f64,
    pub total_response_time: f64,
    pub average_response_time: f64,
    pub rate_limit_hits: u64,
}

/// Cost savings analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostSavingsAnalysis {
    pub total_theoretical_cost: f64,
    pub actual_cost: f64,
    pub total_savings: f64,
    pub service_costs: HashMap<String, f64>,
    pub savings_vs_commercial: f64,
    pub cost_per_research_session: f64,
}

/// Usage trend data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageTrend {
    pub timestamp: DateTime<Utc>,
    pub research_sessions: u64,
    pub api_calls: u64,
    pub active_users: u64,
}

/// Peak usage time identification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeakUsageTime {
    pub timestamp: DateTime<Utc>,
    pub usage_level: u64,
    pub peak_type: PeakType,
}

/// Types of usage peaks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PeakType {
    ResearchSessions,
    ApiCalls,
    ActiveUsers,
}

/// Usage cache for performance optimization
#[derive(Debug, Clone)]
struct UsageCache {
    cached_data: HashMap<String, CachedUsageData>,
}

impl UsageCache {
    fn new() -> Self {
        Self {
            cached_data: HashMap::new(),
        }
    }

    fn get_cached_data(&self, period: &TimePeriod) -> Option<&CachedUsageData> {
        let key = self.period_to_key(period);
        self.cached_data.get(&key)
    }

    fn update_cache(&mut self, period: TimePeriod, analytics_data: UsageAnalyticsData) {
        let key = self.period_to_key(&period);
        self.cached_data.insert(key, CachedUsageData {
            analytics_data,
            last_updated: Utc::now(),
        });
    }

    fn cleanup_old_entries(&mut self) {
        let cutoff = Utc::now() - Duration::minutes(10);
        self.cached_data.retain(|_, data| data.last_updated > cutoff);
    }

    fn period_to_key(&self, period: &TimePeriod) -> String {
        match period {
            TimePeriod::LastHour => "last_hour".to_string(),
            TimePeriod::Last24Hours => "last_24_hours".to_string(),
            TimePeriod::LastWeek => "last_week".to_string(),
            TimePeriod::LastMonth => "last_month".to_string(),
            TimePeriod::LastQuarter => "last_quarter".to_string(),
            TimePeriod::LastYear => "last_year".to_string(),
            TimePeriod::Custom { start, end } => format!("custom_{}_{}", start.timestamp(), end.timestamp()),
        }
    }
}

/// Cached usage data
#[derive(Debug, Clone)]
struct CachedUsageData {
    analytics_data: UsageAnalyticsData,
    last_updated: DateTime<Utc>,
}
