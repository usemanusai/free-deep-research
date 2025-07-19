use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Duration};

use crate::error::{AppError, AppResult};
use super::{AnalyticsDashboardData, TimePeriod, AnalyticsAlert, AlertType, AlertSeverity};

/// Dashboard engine for aggregating and presenting analytics data
#[derive(Clone)]
pub struct DashboardEngine {
    usage_analytics: Arc<RwLock<super::usage_analytics::UsageAnalyticsEngine>>,
    performance_monitor: Arc<RwLock<super::performance_monitor::PerformanceMonitor>>,
    predictive_analytics: Arc<RwLock<super::predictive_analytics::PredictiveAnalyticsEngine>>,
    business_intelligence: Arc<RwLock<super::business_intelligence::BusinessIntelligenceEngine>>,
    config: DashboardConfig,
    dashboard_cache: Arc<RwLock<DashboardCache>>,
}

impl DashboardEngine {
    /// Create a new dashboard engine
    pub async fn new(
        usage_analytics: Arc<RwLock<super::usage_analytics::UsageAnalyticsEngine>>,
        performance_monitor: Arc<RwLock<super::performance_monitor::PerformanceMonitor>>,
        predictive_analytics: Arc<RwLock<super::predictive_analytics::PredictiveAnalyticsEngine>>,
        business_intelligence: Arc<RwLock<super::business_intelligence::BusinessIntelligenceEngine>>,
    ) -> AppResult<Self> {
        info!("Initializing dashboard engine...");

        let config = DashboardConfig::default();
        let dashboard_cache = Arc::new(RwLock::new(DashboardCache::new()));

        let engine = Self {
            usage_analytics,
            performance_monitor,
            predictive_analytics,
            business_intelligence,
            config,
            dashboard_cache,
        };

        info!("Dashboard engine initialized successfully");
        Ok(engine)
    }

    /// Get comprehensive dashboard data
    pub async fn get_dashboard_data(&self) -> AppResult<AnalyticsDashboardData> {
        // Check cache first
        {
            let cache = self.dashboard_cache.read().await;
            if let Some(cached_data) = &cache.cached_dashboard_data {
                if cached_data.generated_at > Utc::now() - Duration::minutes(self.config.cache_duration_minutes as i64) {
                    return Ok(cached_data.data.clone());
                }
            }
        }

        // Generate fresh dashboard data
        let dashboard_data = self.generate_dashboard_data().await?;

        // Update cache
        {
            let mut cache = self.dashboard_cache.write().await;
            cache.cached_dashboard_data = Some(CachedDashboardData {
                data: dashboard_data.clone(),
                generated_at: Utc::now(),
            });
        }

        Ok(dashboard_data)
    }

    /// Generate comprehensive dashboard data
    async fn generate_dashboard_data(&self) -> AppResult<AnalyticsDashboardData> {
        info!("Generating dashboard data...");

        // Collect data from all analytics engines
        let usage_data = self.usage_analytics.read().await.get_analytics_data(TimePeriod::Last24Hours).await?;
        let performance_metrics = self.performance_monitor.read().await.get_current_metrics().await?;
        let predictions = self.predictive_analytics.read().await.get_predictions().await?;
        let recommendations = self.business_intelligence.read().await.get_optimization_recommendations().await?;

        // Generate usage summary
        let usage_summary = UsageSummary {
            total_research_sessions: usage_data.total_research_sessions,
            active_methodologies: usage_data.methodology_usage.len() as u64,
            total_api_calls: usage_data.api_usage_stats.values().map(|s| s.total_calls).sum(),
            average_success_rate: self.calculate_average_success_rate(&usage_data.api_usage_stats),
            cost_savings: usage_data.cost_savings.total_savings,
            peak_usage_hour: self.identify_peak_usage_hour(&usage_data.peak_usage_times),
        };

        // Generate performance summary
        let performance_summary = PerformanceSummary {
            average_response_time: performance_metrics.response_times.api_average_ms,
            current_throughput: performance_metrics.throughput.requests_per_second,
            cpu_utilization: performance_metrics.resource_usage.cpu_usage_percent,
            memory_utilization: performance_metrics.resource_usage.memory_usage_percent,
            active_bottlenecks: performance_metrics.bottlenecks.len() as u64,
            system_health_score: self.calculate_system_health_score(&performance_metrics).await?,
        };

        // Generate prediction summary
        let prediction_summary = PredictionSummary {
            predicted_growth_rate: self.calculate_predicted_growth_rate(&predictions),
            quota_risk_services: self.count_quota_risk_services(&predictions),
            capacity_warnings: predictions.capacity_planning.scaling_recommendations.len() as u64,
            early_warnings: predictions.early_warnings.len() as u64,
            model_accuracy: predictions.model_accuracy.overall_accuracy,
            next_prediction_update: Utc::now() + Duration::minutes(30),
        };

        // Generate alerts
        let alerts = self.generate_dashboard_alerts(&usage_data, &performance_metrics, &predictions).await?;

        Ok(AnalyticsDashboardData {
            usage_summary,
            performance_summary,
            predictions: prediction_summary,
            recommendations,
            alerts,
            last_updated: Utc::now(),
        })
    }

    /// Calculate average success rate across all services
    fn calculate_average_success_rate(&self, api_stats: &std::collections::HashMap<String, super::usage_analytics::ApiUsageStats>) -> f64 {
        if api_stats.is_empty() {
            return 100.0;
        }

        let total_success_rate: f64 = api_stats.values().map(|s| s.success_rate).sum();
        total_success_rate / api_stats.len() as f64
    }

    /// Identify peak usage hour
    fn identify_peak_usage_hour(&self, peak_times: &[super::usage_analytics::PeakUsageTime]) -> Option<u32> {
        peak_times.iter()
            .max_by_key(|p| p.usage_level)
            .map(|p| p.timestamp.hour())
    }

    /// Calculate system health score
    async fn calculate_system_health_score(&self, metrics: &super::PerformanceMetrics) -> AppResult<f64> {
        let mut score = 100.0;

        // Deduct points for high response times
        if metrics.response_times.api_average_ms > 1000.0 {
            score -= 20.0;
        }

        // Deduct points for high resource usage
        if metrics.resource_usage.cpu_usage_percent > 80.0 {
            score -= 15.0;
        }

        if metrics.resource_usage.memory_usage_percent > 80.0 {
            score -= 15.0;
        }

        // Deduct points for bottlenecks
        score -= metrics.bottlenecks.len() as f64 * 10.0;

        // Deduct points for low throughput
        if metrics.throughput.requests_per_second < 1.0 {
            score -= 10.0;
        }

        Ok(score.max(0.0))
    }

    /// Calculate predicted growth rate
    fn calculate_predicted_growth_rate(&self, predictions: &super::PredictiveAnalyticsData) -> f64 {
        // Find research sessions prediction
        for prediction in &predictions.usage_predictions {
            if matches!(prediction.prediction_type, super::predictive_analytics::PredictionType::ResearchSessions) {
                if let (Some(first), Some(last)) = (prediction.forecast_points.first(), prediction.forecast_points.last()) {
                    if first.predicted_value > 0.0 {
                        return ((last.predicted_value - first.predicted_value) / first.predicted_value) * 100.0;
                    }
                }
            }
        }
        0.0
    }

    /// Count services with quota risk
    fn count_quota_risk_services(&self, predictions: &super::PredictiveAnalyticsData) -> u64 {
        predictions.quota_forecasts.iter()
            .filter(|f| matches!(f.risk_level, super::predictive_analytics::QuotaRiskLevel::High | super::predictive_analytics::QuotaRiskLevel::Critical))
            .count() as u64
    }

    /// Generate dashboard alerts
    async fn generate_dashboard_alerts(
        &self,
        usage_data: &super::usage_analytics::UsageAnalyticsData,
        performance_metrics: &super::PerformanceMetrics,
        predictions: &super::PredictiveAnalyticsData,
    ) -> AppResult<Vec<AnalyticsAlert>> {
        let mut alerts = Vec::new();

        // Performance alerts
        if performance_metrics.response_times.api_average_ms > 3000.0 {
            alerts.push(AnalyticsAlert {
                id: "high_response_time".to_string(),
                alert_type: AlertType::PerformanceThreshold,
                severity: AlertSeverity::Critical,
                title: "High Response Time".to_string(),
                message: format!("API response time ({:.2}ms) is critically high", performance_metrics.response_times.api_average_ms),
                created_at: Utc::now(),
                acknowledged: false,
            });
        }

        if performance_metrics.resource_usage.cpu_usage_percent > 90.0 {
            alerts.push(AnalyticsAlert {
                id: "high_cpu_usage".to_string(),
                alert_type: AlertType::SystemHealth,
                severity: AlertSeverity::Critical,
                title: "High CPU Usage".to_string(),
                message: format!("CPU usage ({:.1}%) is critically high", performance_metrics.resource_usage.cpu_usage_percent),
                created_at: Utc::now(),
                acknowledged: false,
            });
        }

        // Usage anomaly alerts
        let total_api_calls: u64 = usage_data.api_usage_stats.values().map(|s| s.total_calls).sum();
        if total_api_calls == 0 {
            alerts.push(AnalyticsAlert {
                id: "no_api_activity".to_string(),
                alert_type: AlertType::UsageAnomaly,
                severity: AlertSeverity::Warning,
                title: "No API Activity".to_string(),
                message: "No API calls detected in the last 24 hours".to_string(),
                created_at: Utc::now(),
                acknowledged: false,
            });
        }

        // Predictive alerts
        for warning in &predictions.early_warnings {
            if matches!(warning.severity, super::AlertSeverity::Critical) {
                alerts.push(AnalyticsAlert {
                    id: format!("predictive_{:?}", warning.warning_type).to_lowercase(),
                    alert_type: AlertType::PredictiveWarning,
                    severity: AlertSeverity::Critical,
                    title: "Predictive Warning".to_string(),
                    message: warning.message.clone(),
                    created_at: Utc::now(),
                    acknowledged: false,
                });
            }
        }

        // Quota alerts
        for forecast in &predictions.quota_forecasts {
            if matches!(forecast.risk_level, super::predictive_analytics::QuotaRiskLevel::Critical) {
                alerts.push(AnalyticsAlert {
                    id: format!("quota_risk_{}", forecast.service_name),
                    alert_type: AlertType::PredictiveWarning,
                    severity: AlertSeverity::Critical,
                    title: "Quota Risk".to_string(),
                    message: format!("Service {} is at critical quota risk", forecast.service_name),
                    created_at: Utc::now(),
                    acknowledged: false,
                });
            }
        }

        // Sort alerts by severity
        alerts.sort_by(|a, b| {
            let severity_order = |s: &AlertSeverity| match s {
                AlertSeverity::Critical => 0,
                AlertSeverity::Warning => 1,
                AlertSeverity::Info => 2,
            };
            severity_order(&a.severity).cmp(&severity_order(&b.severity))
        });

        Ok(alerts)
    }

    /// Get real-time dashboard updates
    pub async fn get_realtime_updates(&self) -> AppResult<DashboardUpdates> {
        let performance_metrics = self.performance_monitor.read().await.get_current_metrics().await?;
        let usage_analytics = self.usage_analytics.read().await;
        let realtime_metrics = usage_analytics.get_analytics_data(TimePeriod::LastHour).await?;

        Ok(DashboardUpdates {
            current_response_time: performance_metrics.response_times.api_average_ms,
            current_throughput: performance_metrics.throughput.requests_per_second,
            current_cpu_usage: performance_metrics.resource_usage.cpu_usage_percent,
            current_memory_usage: performance_metrics.resource_usage.memory_usage_percent,
            active_sessions: realtime_metrics.total_research_sessions,
            last_updated: Utc::now(),
        })
    }

    /// Get dashboard configuration
    pub async fn get_dashboard_config(&self) -> DashboardConfig {
        self.config.clone()
    }

    /// Update dashboard configuration
    pub async fn update_config(&mut self, config: DashboardConfig) -> AppResult<()> {
        info!("Updating dashboard configuration");
        self.config = config;
        
        // Clear cache to force refresh with new configuration
        {
            let mut cache = self.dashboard_cache.write().await;
            cache.clear_cache();
        }
        
        Ok(())
    }

    /// Clear dashboard cache
    pub async fn clear_cache(&self) -> AppResult<()> {
        let mut cache = self.dashboard_cache.write().await;
        cache.clear_cache();
        Ok(())
    }

    /// Health check
    pub async fn health_check(&self) -> AppResult<()> {
        // Check if dashboard data can be generated
        let _dashboard_data = self.generate_dashboard_data().await?;
        Ok(())
    }

    /// Shutdown
    pub async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down dashboard engine...");
        info!("Dashboard engine shutdown complete");
        Ok(())
    }
}

/// Dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub cache_duration_minutes: u64,
    pub auto_refresh_interval_seconds: u64,
    pub max_alerts_displayed: usize,
    pub enable_realtime_updates: bool,
    pub performance_thresholds: PerformanceThresholds,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            cache_duration_minutes: 5,
            auto_refresh_interval_seconds: 30,
            max_alerts_displayed: 10,
            enable_realtime_updates: true,
            performance_thresholds: PerformanceThresholds::default(),
        }
    }
}

/// Performance thresholds for alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    pub response_time_warning_ms: f64,
    pub response_time_critical_ms: f64,
    pub cpu_usage_warning_percent: f64,
    pub cpu_usage_critical_percent: f64,
    pub memory_usage_warning_percent: f64,
    pub memory_usage_critical_percent: f64,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            response_time_warning_ms: 1000.0,
            response_time_critical_ms: 3000.0,
            cpu_usage_warning_percent: 80.0,
            cpu_usage_critical_percent: 90.0,
            memory_usage_warning_percent: 80.0,
            memory_usage_critical_percent: 90.0,
        }
    }
}

/// Usage summary for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageSummary {
    pub total_research_sessions: u64,
    pub active_methodologies: u64,
    pub total_api_calls: u64,
    pub average_success_rate: f64,
    pub cost_savings: f64,
    pub peak_usage_hour: Option<u32>,
}

/// Performance summary for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
    pub average_response_time: f64,
    pub current_throughput: f64,
    pub cpu_utilization: f64,
    pub memory_utilization: f64,
    pub active_bottlenecks: u64,
    pub system_health_score: f64,
}

/// Prediction summary for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionSummary {
    pub predicted_growth_rate: f64,
    pub quota_risk_services: u64,
    pub capacity_warnings: u64,
    pub early_warnings: u64,
    pub model_accuracy: f64,
    pub next_prediction_update: DateTime<Utc>,
}

/// Real-time dashboard updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardUpdates {
    pub current_response_time: f64,
    pub current_throughput: f64,
    pub current_cpu_usage: f64,
    pub current_memory_usage: f64,
    pub active_sessions: u64,
    pub last_updated: DateTime<Utc>,
}

/// Dashboard cache for performance optimization
#[derive(Debug, Clone)]
struct DashboardCache {
    cached_dashboard_data: Option<CachedDashboardData>,
}

impl DashboardCache {
    fn new() -> Self {
        Self {
            cached_dashboard_data: None,
        }
    }

    fn clear_cache(&mut self) {
        self.cached_dashboard_data = None;
    }
}

/// Cached dashboard data
#[derive(Debug, Clone)]
struct CachedDashboardData {
    data: AnalyticsDashboardData,
    generated_at: DateTime<Utc>,
}
