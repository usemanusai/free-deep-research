use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error, warn};
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, VecDeque};
use chrono::{DateTime, Utc, Duration};

use crate::error::{AppError, AppResult};
use super::{PerformanceMetrics, OptimizationRecommendation, OptimizationCategory, Priority, ImpactEstimate, EffortLevel};

/// Performance monitor for tracking system performance and identifying optimization opportunities
#[derive(Clone)]
pub struct PerformanceMonitor {
    metrics_collector: Arc<RwLock<super::metrics_collector::MetricsCollector>>,
    monitoring_service: Arc<RwLock<crate::services::monitoring::MonitoringService>>,
    config: PerformanceMonitoringConfig,
    performance_history: Arc<RwLock<VecDeque<PerformanceSnapshot>>>,
    bottleneck_detector: Arc<RwLock<BottleneckDetector>>,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub async fn new(
        metrics_collector: Arc<RwLock<super::metrics_collector::MetricsCollector>>,
        monitoring_service: Arc<RwLock<crate::services::monitoring::MonitoringService>>,
    ) -> AppResult<Self> {
        info!("Initializing performance monitor...");

        let config = PerformanceMonitoringConfig::default();
        let performance_history = Arc::new(RwLock::new(VecDeque::new()));
        let bottleneck_detector = Arc::new(RwLock::new(BottleneckDetector::new()));

        let monitor = Self {
            metrics_collector,
            monitoring_service,
            config,
            performance_history,
            bottleneck_detector,
        };

        info!("Performance monitor initialized successfully");
        Ok(monitor)
    }

    /// Start performance monitoring
    pub async fn start_monitoring(&self) -> AppResult<()> {
        info!("Starting performance monitoring...");

        // Start background monitoring tasks
        let monitor_clone = self.clone();
        tokio::spawn(async move {
            monitor_clone.collect_performance_metrics().await;
        });

        let monitor_clone = self.clone();
        tokio::spawn(async move {
            monitor_clone.detect_bottlenecks().await;
        });

        let monitor_clone = self.clone();
        tokio::spawn(async move {
            monitor_clone.generate_optimization_recommendations().await;
        });

        let monitor_clone = self.clone();
        tokio::spawn(async move {
            monitor_clone.performance_alerting().await;
        });

        info!("Performance monitoring started successfully");
        Ok(())
    }

    /// Get current performance metrics
    pub async fn get_current_metrics(&self) -> AppResult<PerformanceMetrics> {
        let snapshot = self.collect_current_snapshot().await?;
        
        let response_times = ResponseTimeMetrics {
            api_average_ms: snapshot.api_response_time,
            ui_average_ms: snapshot.ui_response_time,
            database_average_ms: snapshot.database_response_time,
            p95_response_time: snapshot.p95_response_time,
            p99_response_time: snapshot.p99_response_time,
        };

        let throughput = ThroughputMetrics {
            requests_per_second: snapshot.requests_per_second,
            research_sessions_per_hour: snapshot.research_sessions_per_hour,
            api_calls_per_minute: snapshot.api_calls_per_minute,
            concurrent_users: snapshot.concurrent_users,
        };

        let resource_usage = ResourceUsageMetrics {
            cpu_usage_percent: snapshot.cpu_usage,
            memory_usage_percent: snapshot.memory_usage,
            disk_usage_percent: snapshot.disk_usage,
            network_usage_mbps: snapshot.network_usage,
        };

        let bottlenecks = self.get_current_bottlenecks().await?;
        let optimization_opportunities = self.get_optimization_opportunities().await?;

        Ok(PerformanceMetrics {
            response_times,
            throughput,
            resource_usage,
            bottlenecks,
            optimization_opportunities,
        })
    }

    /// Get performance history for analysis
    pub async fn get_performance_history(&self, duration: Duration) -> AppResult<Vec<PerformanceSnapshot>> {
        let history = self.performance_history.read().await;
        let cutoff_time = Utc::now() - duration;
        
        let filtered_history: Vec<PerformanceSnapshot> = history
            .iter()
            .filter(|snapshot| snapshot.timestamp >= cutoff_time)
            .cloned()
            .collect();

        Ok(filtered_history)
    }

    /// Get performance trends analysis
    pub async fn get_performance_trends(&self) -> AppResult<PerformanceTrends> {
        let history = self.get_performance_history(Duration::hours(24)).await?;
        
        if history.is_empty() {
            return Ok(PerformanceTrends::default());
        }

        let mut response_time_trend = Vec::new();
        let mut throughput_trend = Vec::new();
        let mut resource_usage_trend = Vec::new();

        for snapshot in &history {
            response_time_trend.push(TrendPoint {
                timestamp: snapshot.timestamp,
                value: snapshot.api_response_time,
            });

            throughput_trend.push(TrendPoint {
                timestamp: snapshot.timestamp,
                value: snapshot.requests_per_second,
            });

            resource_usage_trend.push(TrendPoint {
                timestamp: snapshot.timestamp,
                value: snapshot.cpu_usage,
            });
        }

        Ok(PerformanceTrends {
            response_time_trend,
            throughput_trend,
            resource_usage_trend,
            trend_analysis: self.analyze_trends(&history).await?,
        })
    }

    /// Collect current performance snapshot
    async fn collect_current_snapshot(&self) -> AppResult<PerformanceSnapshot> {
        let monitoring_service = self.monitoring_service.read().await;
        
        // Get system metrics from monitoring service
        let system_metrics = monitoring_service.get_system_metrics().await?;
        
        // Calculate performance metrics
        let api_response_time = self.calculate_average_api_response_time().await?;
        let ui_response_time = self.calculate_ui_response_time().await?;
        let database_response_time = self.calculate_database_response_time().await?;
        
        let snapshot = PerformanceSnapshot {
            timestamp: Utc::now(),
            api_response_time,
            ui_response_time,
            database_response_time,
            p95_response_time: self.calculate_percentile_response_time(95.0).await?,
            p99_response_time: self.calculate_percentile_response_time(99.0).await?,
            requests_per_second: self.calculate_requests_per_second().await?,
            research_sessions_per_hour: self.calculate_research_sessions_per_hour().await?,
            api_calls_per_minute: self.calculate_api_calls_per_minute().await?,
            concurrent_users: self.calculate_concurrent_users().await?,
            cpu_usage: system_metrics.cpu_usage,
            memory_usage: system_metrics.memory_usage,
            disk_usage: system_metrics.disk_usage,
            network_usage: system_metrics.network_usage,
        };

        Ok(snapshot)
    }

    /// Calculate average API response time
    async fn calculate_average_api_response_time(&self) -> AppResult<f64> {
        let metrics_collector = self.metrics_collector.read().await;
        let end_time = Utc::now();
        let start_time = end_time - Duration::minutes(5);
        
        let api_events = metrics_collector.get_events_by_type_and_time(
            super::EventType::ApiCallMade,
            start_time,
            end_time,
        ).await?;

        if api_events.is_empty() {
            return Ok(0.0);
        }

        let total_response_time: f64 = api_events
            .iter()
            .filter_map(|event| {
                event.metadata.get("response_time")
                    .and_then(|v| v.as_f64())
            })
            .sum();

        Ok(total_response_time / api_events.len() as f64)
    }

    /// Calculate UI response time
    async fn calculate_ui_response_time(&self) -> AppResult<f64> {
        // This would measure UI interaction response times
        // For now, return a placeholder value
        Ok(150.0) // 150ms average UI response time
    }

    /// Calculate database response time
    async fn calculate_database_response_time(&self) -> AppResult<f64> {
        // This would measure database query response times
        // For now, return a placeholder value
        Ok(25.0) // 25ms average database response time
    }

    /// Calculate percentile response time
    async fn calculate_percentile_response_time(&self, percentile: f64) -> AppResult<f64> {
        let metrics_collector = self.metrics_collector.read().await;
        let end_time = Utc::now();
        let start_time = end_time - Duration::minutes(5);
        
        let api_events = metrics_collector.get_events_by_type_and_time(
            super::EventType::ApiCallMade,
            start_time,
            end_time,
        ).await?;

        let mut response_times: Vec<f64> = api_events
            .iter()
            .filter_map(|event| {
                event.metadata.get("response_time")
                    .and_then(|v| v.as_f64())
            })
            .collect();

        if response_times.is_empty() {
            return Ok(0.0);
        }

        response_times.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let index = ((percentile / 100.0) * response_times.len() as f64) as usize;
        let index = index.min(response_times.len() - 1);

        Ok(response_times[index])
    }

    /// Calculate requests per second
    async fn calculate_requests_per_second(&self) -> AppResult<f64> {
        let metrics_collector = self.metrics_collector.read().await;
        let end_time = Utc::now();
        let start_time = end_time - Duration::seconds(60);
        
        let events = metrics_collector.get_events_by_time(start_time, end_time).await?;
        Ok(events.len() as f64 / 60.0)
    }

    /// Calculate research sessions per hour
    async fn calculate_research_sessions_per_hour(&self) -> AppResult<f64> {
        let metrics_collector = self.metrics_collector.read().await;
        let end_time = Utc::now();
        let start_time = end_time - Duration::hours(1);
        
        let research_events = metrics_collector.get_events_by_type_and_time(
            super::EventType::ResearchStarted,
            start_time,
            end_time,
        ).await?;

        Ok(research_events.len() as f64)
    }

    /// Calculate API calls per minute
    async fn calculate_api_calls_per_minute(&self) -> AppResult<f64> {
        let metrics_collector = self.metrics_collector.read().await;
        let end_time = Utc::now();
        let start_time = end_time - Duration::minutes(1);
        
        let api_events = metrics_collector.get_events_by_type_and_time(
            super::EventType::ApiCallMade,
            start_time,
            end_time,
        ).await?;

        Ok(api_events.len() as f64)
    }

    /// Calculate concurrent users
    async fn calculate_concurrent_users(&self) -> AppResult<f64> {
        let metrics_collector = self.metrics_collector.read().await;
        let end_time = Utc::now();
        let start_time = end_time - Duration::minutes(5);
        
        let events = metrics_collector.get_events_by_time(start_time, end_time).await?;
        
        let mut unique_sessions = std::collections::HashSet::new();
        for event in events {
            unique_sessions.insert(event.session_id);
        }

        Ok(unique_sessions.len() as f64)
    }

    /// Get current bottlenecks
    async fn get_current_bottlenecks(&self) -> AppResult<Vec<PerformanceBottleneck>> {
        let bottleneck_detector = self.bottleneck_detector.read().await;
        Ok(bottleneck_detector.get_current_bottlenecks())
    }

    /// Get optimization opportunities
    async fn get_optimization_opportunities(&self) -> AppResult<Vec<OptimizationOpportunity>> {
        let snapshot = self.collect_current_snapshot().await?;
        let mut opportunities = Vec::new();

        // Check for high response times
        if snapshot.api_response_time > self.config.response_time_threshold_ms {
            opportunities.push(OptimizationOpportunity {
                id: "high_api_response_time".to_string(),
                title: "High API Response Time".to_string(),
                description: format!("API response time ({:.2}ms) exceeds threshold", snapshot.api_response_time),
                category: OptimizationCategory::Performance,
                potential_improvement: format!("Reduce response time by up to 30%"),
                implementation_suggestions: vec![
                    "Implement request caching".to_string(),
                    "Optimize API call batching".to_string(),
                    "Add connection pooling".to_string(),
                ],
            });
        }

        // Check for high resource usage
        if snapshot.cpu_usage > self.config.cpu_usage_threshold {
            opportunities.push(OptimizationOpportunity {
                id: "high_cpu_usage".to_string(),
                title: "High CPU Usage".to_string(),
                description: format!("CPU usage ({:.1}%) is high", snapshot.cpu_usage),
                category: OptimizationCategory::Performance,
                potential_improvement: format!("Reduce CPU usage by up to 25%"),
                implementation_suggestions: vec![
                    "Optimize background processing".to_string(),
                    "Implement task scheduling".to_string(),
                    "Add processing throttling".to_string(),
                ],
            });
        }

        Ok(opportunities)
    }

    /// Analyze performance trends
    async fn analyze_trends(&self, history: &[PerformanceSnapshot]) -> AppResult<TrendAnalysis> {
        if history.len() < 2 {
            return Ok(TrendAnalysis::default());
        }

        let first = &history[0];
        let last = &history[history.len() - 1];

        let response_time_change = ((last.api_response_time - first.api_response_time) / first.api_response_time) * 100.0;
        let throughput_change = ((last.requests_per_second - first.requests_per_second) / first.requests_per_second) * 100.0;
        let cpu_usage_change = ((last.cpu_usage - first.cpu_usage) / first.cpu_usage) * 100.0;

        Ok(TrendAnalysis {
            response_time_trend: if response_time_change > 5.0 { TrendDirection::Increasing } 
                               else if response_time_change < -5.0 { TrendDirection::Decreasing } 
                               else { TrendDirection::Stable },
            throughput_trend: if throughput_change > 5.0 { TrendDirection::Increasing } 
                             else if throughput_change < -5.0 { TrendDirection::Decreasing } 
                             else { TrendDirection::Stable },
            resource_usage_trend: if cpu_usage_change > 5.0 { TrendDirection::Increasing } 
                                 else if cpu_usage_change < -5.0 { TrendDirection::Decreasing } 
                                 else { TrendDirection::Stable },
            overall_health: self.calculate_overall_health(last).await?,
        })
    }

    /// Calculate overall system health score
    async fn calculate_overall_health(&self, snapshot: &PerformanceSnapshot) -> AppResult<HealthScore> {
        let mut score = 100.0;

        // Deduct points for high response times
        if snapshot.api_response_time > self.config.response_time_threshold_ms {
            score -= 20.0;
        }

        // Deduct points for high resource usage
        if snapshot.cpu_usage > self.config.cpu_usage_threshold {
            score -= 15.0;
        }

        if snapshot.memory_usage > self.config.memory_usage_threshold {
            score -= 15.0;
        }

        // Deduct points for low throughput
        if snapshot.requests_per_second < self.config.min_throughput_rps {
            score -= 10.0;
        }

        let health_status = if score >= 80.0 { HealthStatus::Excellent }
                           else if score >= 60.0 { HealthStatus::Good }
                           else if score >= 40.0 { HealthStatus::Fair }
                           else { HealthStatus::Poor };

        Ok(HealthScore {
            score: score.max(0.0),
            status: health_status,
        })
    }

    /// Background task for collecting performance metrics
    async fn collect_performance_metrics(&self) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(self.config.collection_interval_seconds));
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.collect_and_store_snapshot().await {
                error!("Failed to collect performance snapshot: {}", e);
            }
        }
    }

    /// Collect and store performance snapshot
    async fn collect_and_store_snapshot(&self) -> AppResult<()> {
        let snapshot = self.collect_current_snapshot().await?;
        
        {
            let mut history = self.performance_history.write().await;
            history.push_back(snapshot);
            
            // Keep only recent history
            while history.len() > self.config.max_history_size {
                history.pop_front();
            }
        }

        Ok(())
    }

    /// Background task for detecting bottlenecks
    async fn detect_bottlenecks(&self) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60)); // 1 minute
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.run_bottleneck_detection().await {
                error!("Failed to run bottleneck detection: {}", e);
            }
        }
    }

    /// Run bottleneck detection
    async fn run_bottleneck_detection(&self) -> AppResult<()> {
        let snapshot = self.collect_current_snapshot().await?;
        let mut bottleneck_detector = self.bottleneck_detector.write().await;
        
        bottleneck_detector.analyze_snapshot(&snapshot, &self.config);
        
        Ok(())
    }

    /// Background task for generating optimization recommendations
    async fn generate_optimization_recommendations(&self) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // 5 minutes
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.update_optimization_recommendations().await {
                error!("Failed to update optimization recommendations: {}", e);
            }
        }
    }

    /// Update optimization recommendations
    async fn update_optimization_recommendations(&self) -> AppResult<()> {
        let _opportunities = self.get_optimization_opportunities().await?;
        // Store or update recommendations
        Ok(())
    }

    /// Background task for performance alerting
    async fn performance_alerting(&self) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30)); // 30 seconds
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.check_performance_alerts().await {
                error!("Failed to check performance alerts: {}", e);
            }
        }
    }

    /// Check for performance alerts
    async fn check_performance_alerts(&self) -> AppResult<()> {
        let snapshot = self.collect_current_snapshot().await?;
        
        // Check for critical performance issues
        if snapshot.api_response_time > self.config.critical_response_time_ms {
            warn!("Critical: API response time ({:.2}ms) exceeds critical threshold", snapshot.api_response_time);
        }

        if snapshot.cpu_usage > self.config.critical_cpu_usage {
            warn!("Critical: CPU usage ({:.1}%) exceeds critical threshold", snapshot.cpu_usage);
        }

        Ok(())
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: &PerformanceMonitoringConfig) -> AppResult<()> {
        info!("Updating performance monitoring configuration");
        self.config = config.clone();
        Ok(())
    }

    /// Health check
    pub async fn health_check(&self) -> AppResult<()> {
        let history = self.performance_history.read().await;
        
        if history.is_empty() {
            return Err(AppError::Analytics("No performance data available".to_string()));
        }

        let latest = history.back().unwrap();
        let age = Utc::now() - latest.timestamp;
        
        if age > Duration::minutes(5) {
            return Err(AppError::Analytics("Performance data is stale".to_string()));
        }

        Ok(())
    }

    /// Shutdown
    pub async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down performance monitor...");
        info!("Performance monitor shutdown complete");
        Ok(())
    }
}

/// Performance monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMonitoringConfig {
    pub collection_interval_seconds: u64,
    pub max_history_size: usize,
    pub response_time_threshold_ms: f64,
    pub critical_response_time_ms: f64,
    pub cpu_usage_threshold: f64,
    pub critical_cpu_usage: f64,
    pub memory_usage_threshold: f64,
    pub min_throughput_rps: f64,
    pub enable_bottleneck_detection: bool,
    pub enable_optimization_recommendations: bool,
}

impl Default for PerformanceMonitoringConfig {
    fn default() -> Self {
        Self {
            collection_interval_seconds: 30,
            max_history_size: 2880, // 24 hours at 30-second intervals
            response_time_threshold_ms: 1000.0,
            critical_response_time_ms: 5000.0,
            cpu_usage_threshold: 80.0,
            critical_cpu_usage: 95.0,
            memory_usage_threshold: 85.0,
            min_throughput_rps: 1.0,
            enable_bottleneck_detection: true,
            enable_optimization_recommendations: true,
        }
    }
}

/// Performance snapshot at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    pub timestamp: DateTime<Utc>,
    pub api_response_time: f64,
    pub ui_response_time: f64,
    pub database_response_time: f64,
    pub p95_response_time: f64,
    pub p99_response_time: f64,
    pub requests_per_second: f64,
    pub research_sessions_per_hour: f64,
    pub api_calls_per_minute: f64,
    pub concurrent_users: f64,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_usage: f64,
}

/// Response time metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTimeMetrics {
    pub api_average_ms: f64,
    pub ui_average_ms: f64,
    pub database_average_ms: f64,
    pub p95_response_time: f64,
    pub p99_response_time: f64,
}

/// Throughput metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputMetrics {
    pub requests_per_second: f64,
    pub research_sessions_per_hour: f64,
    pub api_calls_per_minute: f64,
    pub concurrent_users: f64,
}

/// Resource usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageMetrics {
    pub cpu_usage_percent: f64,
    pub memory_usage_percent: f64,
    pub disk_usage_percent: f64,
    pub network_usage_mbps: f64,
}

/// Performance bottleneck identification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBottleneck {
    pub id: String,
    pub bottleneck_type: BottleneckType,
    pub severity: BottleneckSeverity,
    pub description: String,
    pub affected_components: Vec<String>,
    pub detected_at: DateTime<Utc>,
    pub estimated_impact: String,
    pub suggested_solutions: Vec<String>,
}

/// Types of performance bottlenecks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BottleneckType {
    CpuBound,
    MemoryBound,
    IoBound,
    NetworkBound,
    DatabaseBound,
    ApiBound,
}

/// Bottleneck severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BottleneckSeverity {
    Critical,
    High,
    Medium,
    Low,
}

/// Optimization opportunity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationOpportunity {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: OptimizationCategory,
    pub potential_improvement: String,
    pub implementation_suggestions: Vec<String>,
}

/// Performance trends analysis
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceTrends {
    pub response_time_trend: Vec<TrendPoint>,
    pub throughput_trend: Vec<TrendPoint>,
    pub resource_usage_trend: Vec<TrendPoint>,
    pub trend_analysis: TrendAnalysis,
}

/// Trend data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendPoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
}

/// Trend analysis
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrendAnalysis {
    pub response_time_trend: TrendDirection,
    pub throughput_trend: TrendDirection,
    pub resource_usage_trend: TrendDirection,
    pub overall_health: HealthScore,
}

/// Trend direction
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum TrendDirection {
    #[default]
    Stable,
    Increasing,
    Decreasing,
}

/// Health score
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthScore {
    pub score: f64,
    pub status: HealthStatus,
}

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum HealthStatus {
    #[default]
    Unknown,
    Excellent,
    Good,
    Fair,
    Poor,
}

/// Bottleneck detector for identifying performance issues
#[derive(Debug, Clone)]
struct BottleneckDetector {
    detected_bottlenecks: Vec<PerformanceBottleneck>,
}

impl BottleneckDetector {
    fn new() -> Self {
        Self {
            detected_bottlenecks: Vec::new(),
        }
    }

    fn analyze_snapshot(&mut self, snapshot: &PerformanceSnapshot, config: &PerformanceMonitoringConfig) {
        self.detected_bottlenecks.clear();

        // Check for CPU bottlenecks
        if snapshot.cpu_usage > config.cpu_usage_threshold {
            self.detected_bottlenecks.push(PerformanceBottleneck {
                id: "cpu_bottleneck".to_string(),
                bottleneck_type: BottleneckType::CpuBound,
                severity: if snapshot.cpu_usage > config.critical_cpu_usage {
                    BottleneckSeverity::Critical
                } else {
                    BottleneckSeverity::High
                },
                description: format!("High CPU usage: {:.1}%", snapshot.cpu_usage),
                affected_components: vec!["System".to_string(), "Background Tasks".to_string()],
                detected_at: Utc::now(),
                estimated_impact: "Reduced system responsiveness and throughput".to_string(),
                suggested_solutions: vec![
                    "Optimize background processing".to_string(),
                    "Implement task throttling".to_string(),
                    "Review CPU-intensive operations".to_string(),
                ],
            });
        }

        // Check for memory bottlenecks
        if snapshot.memory_usage > config.memory_usage_threshold {
            self.detected_bottlenecks.push(PerformanceBottleneck {
                id: "memory_bottleneck".to_string(),
                bottleneck_type: BottleneckType::MemoryBound,
                severity: BottleneckSeverity::High,
                description: format!("High memory usage: {:.1}%", snapshot.memory_usage),
                affected_components: vec!["System".to_string(), "Data Processing".to_string()],
                detected_at: Utc::now(),
                estimated_impact: "Potential system instability and performance degradation".to_string(),
                suggested_solutions: vec![
                    "Implement memory cleanup routines".to_string(),
                    "Optimize data structures".to_string(),
                    "Add memory usage monitoring".to_string(),
                ],
            });
        }

        // Check for API response time bottlenecks
        if snapshot.api_response_time > config.response_time_threshold_ms {
            self.detected_bottlenecks.push(PerformanceBottleneck {
                id: "api_response_bottleneck".to_string(),
                bottleneck_type: BottleneckType::ApiBound,
                severity: if snapshot.api_response_time > config.critical_response_time_ms {
                    BottleneckSeverity::Critical
                } else {
                    BottleneckSeverity::Medium
                },
                description: format!("Slow API response time: {:.2}ms", snapshot.api_response_time),
                affected_components: vec!["API Layer".to_string(), "External Services".to_string()],
                detected_at: Utc::now(),
                estimated_impact: "Poor user experience and reduced productivity".to_string(),
                suggested_solutions: vec![
                    "Implement request caching".to_string(),
                    "Optimize API call patterns".to_string(),
                    "Add connection pooling".to_string(),
                ],
            });
        }
    }

    fn get_current_bottlenecks(&self) -> Vec<PerformanceBottleneck> {
        self.detected_bottlenecks.clone()
    }
}
