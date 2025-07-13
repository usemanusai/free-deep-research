use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error, warn};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

use crate::error::{AppError, AppResult};
use crate::services::Service;

pub mod usage_analytics;
pub mod performance_monitor;
pub mod predictive_analytics;
pub mod business_intelligence;
pub mod metrics_collector;
pub mod dashboard_engine;
pub mod report_generator;

use usage_analytics::UsageAnalyticsEngine;
use performance_monitor::PerformanceMonitor;
use predictive_analytics::PredictiveAnalyticsEngine;
use business_intelligence::BusinessIntelligenceEngine;
use metrics_collector::MetricsCollector;
use dashboard_engine::DashboardEngine;
use report_generator::ReportGenerator;

/// Comprehensive analytics service for the Free Deep Research System
/// Provides usage analytics, performance monitoring, predictive analytics, and business intelligence
#[derive(Clone)]
pub struct AnalyticsService {
    usage_analytics: Arc<RwLock<UsageAnalyticsEngine>>,
    performance_monitor: Arc<RwLock<PerformanceMonitor>>,
    predictive_analytics: Arc<RwLock<PredictiveAnalyticsEngine>>,
    business_intelligence: Arc<RwLock<BusinessIntelligenceEngine>>,
    metrics_collector: Arc<RwLock<MetricsCollector>>,
    dashboard_engine: Arc<RwLock<DashboardEngine>>,
    report_generator: Arc<RwLock<ReportGenerator>>,
    config: AnalyticsConfig,
}

impl AnalyticsService {
    /// Create a new analytics service
    pub async fn new(
        data_persistence: Arc<RwLock<crate::services::data_persistence::DataPersistenceService>>,
        monitoring: Arc<RwLock<crate::services::monitoring::MonitoringService>>,
    ) -> AppResult<Self> {
        info!("Initializing analytics service...");

        let config = AnalyticsConfig::default();

        // Initialize metrics collector first (required by other components)
        let metrics_collector = MetricsCollector::new(data_persistence.clone()).await?;
        let metrics_collector = Arc::new(RwLock::new(metrics_collector));

        // Initialize usage analytics engine
        let usage_analytics = UsageAnalyticsEngine::new(
            metrics_collector.clone(),
            data_persistence.clone(),
        ).await?;
        let usage_analytics = Arc::new(RwLock::new(usage_analytics));

        // Initialize performance monitor
        let performance_monitor = PerformanceMonitor::new(
            metrics_collector.clone(),
            monitoring.clone(),
        ).await?;
        let performance_monitor = Arc::new(RwLock::new(performance_monitor));

        // Initialize predictive analytics engine
        let predictive_analytics = PredictiveAnalyticsEngine::new(
            usage_analytics.clone(),
            performance_monitor.clone(),
            data_persistence.clone(),
        ).await?;
        let predictive_analytics = Arc::new(RwLock::new(predictive_analytics));

        // Initialize business intelligence engine
        let business_intelligence = BusinessIntelligenceEngine::new(
            usage_analytics.clone(),
            performance_monitor.clone(),
            predictive_analytics.clone(),
            data_persistence.clone(),
        ).await?;
        let business_intelligence = Arc::new(RwLock::new(business_intelligence));

        // Initialize dashboard engine
        let dashboard_engine = DashboardEngine::new(
            usage_analytics.clone(),
            performance_monitor.clone(),
            predictive_analytics.clone(),
            business_intelligence.clone(),
        ).await?;
        let dashboard_engine = Arc::new(RwLock::new(dashboard_engine));

        // Initialize report generator
        let report_generator = ReportGenerator::new(
            usage_analytics.clone(),
            performance_monitor.clone(),
            predictive_analytics.clone(),
            business_intelligence.clone(),
            data_persistence.clone(),
        ).await?;
        let report_generator = Arc::new(RwLock::new(report_generator));

        let service = Self {
            usage_analytics,
            performance_monitor,
            predictive_analytics,
            business_intelligence,
            metrics_collector,
            dashboard_engine,
            report_generator,
            config,
        };

        info!("Analytics service initialized successfully");
        Ok(service)
    }

    /// Start analytics background processing
    pub async fn start_analytics_processing(&self) -> AppResult<()> {
        info!("Starting analytics background processing...");

        // Start metrics collection
        {
            let metrics_collector = self.metrics_collector.read().await;
            metrics_collector.start_collection().await?;
        }

        // Start usage analytics processing
        {
            let usage_analytics = self.usage_analytics.read().await;
            usage_analytics.start_processing().await?;
        }

        // Start performance monitoring
        {
            let performance_monitor = self.performance_monitor.read().await;
            performance_monitor.start_monitoring().await?;
        }

        // Start predictive analytics
        {
            let predictive_analytics = self.predictive_analytics.read().await;
            predictive_analytics.start_prediction_engine().await?;
        }

        // Start business intelligence processing
        {
            let business_intelligence = self.business_intelligence.read().await;
            business_intelligence.start_intelligence_processing().await?;
        }

        info!("Analytics background processing started successfully");
        Ok(())
    }

    /// Get comprehensive analytics dashboard data
    pub async fn get_dashboard_data(&self) -> AppResult<AnalyticsDashboardData> {
        let dashboard_engine = self.dashboard_engine.read().await;
        dashboard_engine.get_dashboard_data().await
    }

    /// Get usage analytics for a specific time period
    pub async fn get_usage_analytics(&self, period: TimePeriod) -> AppResult<UsageAnalyticsData> {
        let usage_analytics = self.usage_analytics.read().await;
        usage_analytics.get_analytics_data(period).await
    }

    /// Get performance metrics for monitoring
    pub async fn get_performance_metrics(&self) -> AppResult<PerformanceMetrics> {
        let performance_monitor = self.performance_monitor.read().await;
        performance_monitor.get_current_metrics().await
    }

    /// Get predictive analytics and forecasting
    pub async fn get_predictive_analytics(&self) -> AppResult<PredictiveAnalyticsData> {
        let predictive_analytics = self.predictive_analytics.read().await;
        predictive_analytics.get_predictions().await
    }

    /// Generate comprehensive business intelligence report
    pub async fn generate_business_report(&self, report_type: BusinessReportType) -> AppResult<BusinessReport> {
        let report_generator = self.report_generator.read().await;
        report_generator.generate_report(report_type).await
    }

    /// Record a system event for analytics
    pub async fn record_event(&self, event: AnalyticsEvent) -> AppResult<()> {
        let metrics_collector = self.metrics_collector.write().await;
        metrics_collector.record_event(event).await
    }

    /// Get optimization recommendations
    pub async fn get_optimization_recommendations(&self) -> AppResult<Vec<OptimizationRecommendation>> {
        let business_intelligence = self.business_intelligence.read().await;
        business_intelligence.get_optimization_recommendations().await
    }

    /// Update analytics configuration
    pub async fn update_config(&mut self, config: AnalyticsConfig) -> AppResult<()> {
        info!("Updating analytics configuration");
        self.config = config;
        
        // Update configuration in all components
        {
            let mut usage_analytics = self.usage_analytics.write().await;
            usage_analytics.update_config(&self.config.usage_analytics).await?;
        }
        
        {
            let mut performance_monitor = self.performance_monitor.write().await;
            performance_monitor.update_config(&self.config.performance_monitoring).await?;
        }
        
        {
            let mut predictive_analytics = self.predictive_analytics.write().await;
            predictive_analytics.update_config(&self.config.predictive_analytics).await?;
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl Service for AnalyticsService {
    async fn health_check(&self) -> AppResult<()> {
        // Check all analytics components
        {
            let metrics_collector = self.metrics_collector.read().await;
            metrics_collector.health_check().await?;
        }
        
        {
            let usage_analytics = self.usage_analytics.read().await;
            usage_analytics.health_check().await?;
        }
        
        {
            let performance_monitor = self.performance_monitor.read().await;
            performance_monitor.health_check().await?;
        }
        
        {
            let predictive_analytics = self.predictive_analytics.read().await;
            predictive_analytics.health_check().await?;
        }
        
        {
            let business_intelligence = self.business_intelligence.read().await;
            business_intelligence.health_check().await?;
        }

        Ok(())
    }

    async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down analytics service...");

        // Shutdown all components in reverse order
        {
            let report_generator = self.report_generator.write().await;
            report_generator.shutdown().await?;
        }

        {
            let dashboard_engine = self.dashboard_engine.write().await;
            dashboard_engine.shutdown().await?;
        }

        {
            let business_intelligence = self.business_intelligence.write().await;
            business_intelligence.shutdown().await?;
        }

        {
            let predictive_analytics = self.predictive_analytics.write().await;
            predictive_analytics.shutdown().await?;
        }

        {
            let performance_monitor = self.performance_monitor.write().await;
            performance_monitor.shutdown().await?;
        }

        {
            let usage_analytics = self.usage_analytics.write().await;
            usage_analytics.shutdown().await?;
        }

        {
            let metrics_collector = self.metrics_collector.write().await;
            metrics_collector.shutdown().await?;
        }

        info!("Analytics service shutdown complete");
        Ok(())
    }
}

/// Analytics service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsConfig {
    pub usage_analytics: UsageAnalyticsConfig,
    pub performance_monitoring: PerformanceMonitoringConfig,
    pub predictive_analytics: PredictiveAnalyticsConfig,
    pub business_intelligence: BusinessIntelligenceConfig,
    pub data_retention_days: u32,
    pub real_time_updates: bool,
    pub export_enabled: bool,
}

impl Default for AnalyticsConfig {
    fn default() -> Self {
        Self {
            usage_analytics: UsageAnalyticsConfig::default(),
            performance_monitoring: PerformanceMonitoringConfig::default(),
            predictive_analytics: PredictiveAnalyticsConfig::default(),
            business_intelligence: BusinessIntelligenceConfig::default(),
            data_retention_days: 365,
            real_time_updates: true,
            export_enabled: true,
        }
    }
}

/// Time period for analytics queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimePeriod {
    LastHour,
    Last24Hours,
    LastWeek,
    LastMonth,
    LastQuarter,
    LastYear,
    Custom { start: DateTime<Utc>, end: DateTime<Utc> },
}

/// Analytics event for tracking system usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    pub event_type: EventType,
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<String>,
    pub session_id: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Types of events tracked by the analytics system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    // Research Events
    ResearchStarted,
    ResearchCompleted,
    ResearchFailed,
    MethodologySelected,

    // API Events
    ApiKeyAdded,
    ApiKeyRotated,
    ApiCallMade,
    RateLimitHit,

    // Performance Events
    SystemStartup,
    SystemShutdown,
    PerformanceAlert,

    // User Interface Events
    DashboardViewed,
    ReportGenerated,
    ConfigurationChanged,

    // System Events
    BackupCompleted,
    SecurityEvent,
    ErrorOccurred,
}

/// Comprehensive dashboard data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsDashboardData {
    pub usage_summary: UsageSummary,
    pub performance_summary: PerformanceSummary,
    pub predictions: PredictionSummary,
    pub recommendations: Vec<OptimizationRecommendation>,
    pub alerts: Vec<AnalyticsAlert>,
    pub last_updated: DateTime<Utc>,
}

/// Usage analytics data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageAnalyticsData {
    pub total_research_sessions: u64,
    pub methodology_usage: HashMap<String, u64>,
    pub api_usage_stats: HashMap<String, ApiUsageStats>,
    pub cost_savings: CostSavingsAnalysis,
    pub usage_trends: Vec<UsageTrend>,
    pub peak_usage_times: Vec<PeakUsageTime>,
}

/// Performance metrics structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub response_times: ResponseTimeMetrics,
    pub throughput: ThroughputMetrics,
    pub resource_usage: ResourceUsageMetrics,
    pub bottlenecks: Vec<PerformanceBottleneck>,
    pub optimization_opportunities: Vec<OptimizationOpportunity>,
}

/// Predictive analytics data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictiveAnalyticsData {
    pub usage_predictions: Vec<UsagePrediction>,
    pub quota_forecasts: Vec<QuotaForecast>,
    pub capacity_planning: CapacityPlanningData,
    pub early_warnings: Vec<EarlyWarning>,
    pub model_accuracy: ModelAccuracyMetrics,
}

/// Business report types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BusinessReportType {
    ExecutiveSummary,
    UsageReport,
    PerformanceReport,
    CostAnalysis,
    TrendAnalysis,
    CustomReport { template_id: String },
}

/// Business intelligence report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessReport {
    pub report_type: BusinessReportType,
    pub generated_at: DateTime<Utc>,
    pub period: TimePeriod,
    pub summary: ReportSummary,
    pub sections: Vec<ReportSection>,
    pub charts: Vec<ChartData>,
    pub recommendations: Vec<BusinessRecommendation>,
}

/// Optimization recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRecommendation {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: OptimizationCategory,
    pub priority: Priority,
    pub estimated_impact: ImpactEstimate,
    pub implementation_effort: EffortLevel,
    pub created_at: DateTime<Utc>,
}

/// Optimization categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationCategory {
    Performance,
    CostSavings,
    UserExperience,
    Security,
    Reliability,
}

/// Priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

/// Impact estimate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactEstimate {
    pub performance_improvement: Option<f64>,
    pub cost_savings: Option<f64>,
    pub user_satisfaction: Option<f64>,
    pub description: String,
}

/// Implementation effort levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffortLevel {
    Minimal,
    Low,
    Medium,
    High,
    Extensive,
}

/// Analytics alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsAlert {
    pub id: String,
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub title: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub acknowledged: bool,
}

/// Alert types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    PerformanceThreshold,
    UsageAnomaly,
    PredictiveWarning,
    SystemHealth,
    SecurityConcern,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Critical,
    Warning,
    Info,
}

// Re-export configuration types
pub use usage_analytics::UsageAnalyticsConfig;
pub use performance_monitor::PerformanceMonitoringConfig;
pub use predictive_analytics::PredictiveAnalyticsConfig;
pub use business_intelligence::BusinessIntelligenceConfig;

// Additional data structures needed by the analytics system
use dashboard_engine::{UsageSummary, PerformanceSummary, PredictionSummary};
use report_generator::{ReportSummary, ReportSection, ChartData, BusinessRecommendation};
