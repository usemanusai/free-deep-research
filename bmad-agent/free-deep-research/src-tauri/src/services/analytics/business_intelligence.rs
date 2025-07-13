use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error, warn};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

use crate::error::{AppError, AppResult};
use super::{OptimizationRecommendation, OptimizationCategory, Priority, ImpactEstimate, EffortLevel, BusinessReport, BusinessReportType, TimePeriod};

/// Business intelligence engine for generating insights and recommendations
#[derive(Clone)]
pub struct BusinessIntelligenceEngine {
    usage_analytics: Arc<RwLock<super::usage_analytics::UsageAnalyticsEngine>>,
    performance_monitor: Arc<RwLock<super::performance_monitor::PerformanceMonitor>>,
    predictive_analytics: Arc<RwLock<super::predictive_analytics::PredictiveAnalyticsEngine>>,
    data_persistence: Arc<RwLock<crate::services::data_persistence::DataPersistenceService>>,
    config: BusinessIntelligenceConfig,
    insights_cache: Arc<RwLock<InsightsCache>>,
}

impl BusinessIntelligenceEngine {
    /// Create a new business intelligence engine
    pub async fn new(
        usage_analytics: Arc<RwLock<super::usage_analytics::UsageAnalyticsEngine>>,
        performance_monitor: Arc<RwLock<super::performance_monitor::PerformanceMonitor>>,
        predictive_analytics: Arc<RwLock<super::predictive_analytics::PredictiveAnalyticsEngine>>,
        data_persistence: Arc<RwLock<crate::services::data_persistence::DataPersistenceService>>,
    ) -> AppResult<Self> {
        info!("Initializing business intelligence engine...");

        let config = BusinessIntelligenceConfig::default();
        let insights_cache = Arc::new(RwLock::new(InsightsCache::new()));

        let engine = Self {
            usage_analytics,
            performance_monitor,
            predictive_analytics,
            data_persistence,
            config,
            insights_cache,
        };

        info!("Business intelligence engine initialized successfully");
        Ok(engine)
    }

    /// Start business intelligence processing
    pub async fn start_intelligence_processing(&self) -> AppResult<()> {
        info!("Starting business intelligence processing...");

        // Start background processing tasks
        let engine_clone = self.clone();
        tokio::spawn(async move {
            engine_clone.generate_insights().await;
        });

        let engine_clone = self.clone();
        tokio::spawn(async move {
            engine_clone.update_recommendations().await;
        });

        let engine_clone = self.clone();
        tokio::spawn(async move {
            engine_clone.calculate_kpis().await;
        });

        let engine_clone = self.clone();
        tokio::spawn(async move {
            engine_clone.trend_analysis().await;
        });

        info!("Business intelligence processing started successfully");
        Ok(())
    }

    /// Get optimization recommendations
    pub async fn get_optimization_recommendations(&self) -> AppResult<Vec<OptimizationRecommendation>> {
        // Check cache first
        {
            let cache = self.insights_cache.read().await;
            if let Some(cached_recommendations) = &cache.cached_recommendations {
                if cached_recommendations.generated_at > Utc::now() - Duration::minutes(self.config.cache_duration_minutes as i64) {
                    return Ok(cached_recommendations.recommendations.clone());
                }
            }
        }

        // Generate fresh recommendations
        let recommendations = self.generate_optimization_recommendations().await?;

        // Update cache
        {
            let mut cache = self.insights_cache.write().await;
            cache.cached_recommendations = Some(CachedRecommendations {
                recommendations: recommendations.clone(),
                generated_at: Utc::now(),
            });
        }

        Ok(recommendations)
    }

    /// Generate comprehensive optimization recommendations
    async fn generate_optimization_recommendations(&self) -> AppResult<Vec<OptimizationRecommendation>> {
        info!("Generating optimization recommendations...");

        let mut recommendations = Vec::new();

        // Get data from all analytics engines
        let usage_data = self.usage_analytics.read().await.get_analytics_data(TimePeriod::LastWeek).await?;
        let performance_metrics = self.performance_monitor.read().await.get_current_metrics().await?;
        let predictions = self.predictive_analytics.read().await.get_predictions().await?;

        // Generate performance-based recommendations
        recommendations.extend(self.generate_performance_recommendations(&performance_metrics).await?);

        // Generate usage-based recommendations
        recommendations.extend(self.generate_usage_recommendations(&usage_data).await?);

        // Generate predictive recommendations
        recommendations.extend(self.generate_predictive_recommendations(&predictions).await?);

        // Generate cost optimization recommendations
        recommendations.extend(self.generate_cost_recommendations(&usage_data).await?);

        // Generate security recommendations
        recommendations.extend(self.generate_security_recommendations().await?);

        // Sort by priority and impact
        recommendations.sort_by(|a, b| {
            let priority_order = |p: &Priority| match p {
                Priority::Critical => 0,
                Priority::High => 1,
                Priority::Medium => 2,
                Priority::Low => 3,
            };
            priority_order(&a.priority).cmp(&priority_order(&b.priority))
        });

        Ok(recommendations)
    }

    /// Generate performance-based recommendations
    async fn generate_performance_recommendations(&self, metrics: &super::PerformanceMetrics) -> AppResult<Vec<OptimizationRecommendation>> {
        let mut recommendations = Vec::new();

        // Check response time optimization
        if metrics.response_times.api_average_ms > 1000.0 {
            recommendations.push(OptimizationRecommendation {
                id: "optimize_api_response_time".to_string(),
                title: "Optimize API Response Time".to_string(),
                description: format!("API response time ({:.2}ms) can be improved", metrics.response_times.api_average_ms),
                category: OptimizationCategory::Performance,
                priority: if metrics.response_times.api_average_ms > 3000.0 { Priority::Critical } else { Priority::High },
                estimated_impact: ImpactEstimate {
                    performance_improvement: Some(30.0),
                    cost_savings: None,
                    user_satisfaction: Some(25.0),
                    description: "Reduce API response time by 30% and improve user satisfaction".to_string(),
                },
                implementation_effort: EffortLevel::Medium,
                created_at: Utc::now(),
            });
        }

        // Check resource utilization optimization
        if metrics.resource_usage.cpu_usage_percent > 80.0 {
            recommendations.push(OptimizationRecommendation {
                id: "optimize_cpu_usage".to_string(),
                title: "Optimize CPU Usage".to_string(),
                description: format!("High CPU usage ({:.1}%) detected", metrics.resource_usage.cpu_usage_percent),
                category: OptimizationCategory::Performance,
                priority: Priority::High,
                estimated_impact: ImpactEstimate {
                    performance_improvement: Some(20.0),
                    cost_savings: None,
                    user_satisfaction: Some(15.0),
                    description: "Reduce CPU usage and improve system stability".to_string(),
                },
                implementation_effort: EffortLevel::Medium,
                created_at: Utc::now(),
            });
        }

        // Check throughput optimization
        if metrics.throughput.requests_per_second < 5.0 {
            recommendations.push(OptimizationRecommendation {
                id: "improve_throughput".to_string(),
                title: "Improve System Throughput".to_string(),
                description: format!("Low throughput ({:.1} RPS) can be optimized", metrics.throughput.requests_per_second),
                category: OptimizationCategory::Performance,
                priority: Priority::Medium,
                estimated_impact: ImpactEstimate {
                    performance_improvement: Some(50.0),
                    cost_savings: None,
                    user_satisfaction: Some(20.0),
                    description: "Increase system throughput by 50%".to_string(),
                },
                implementation_effort: EffortLevel::High,
                created_at: Utc::now(),
            });
        }

        Ok(recommendations)
    }

    /// Generate usage-based recommendations
    async fn generate_usage_recommendations(&self, usage_data: &super::usage_analytics::UsageAnalyticsData) -> AppResult<Vec<OptimizationRecommendation>> {
        let mut recommendations = Vec::new();

        // Analyze methodology usage patterns
        let total_sessions = usage_data.total_research_sessions;
        if total_sessions > 0 {
            // Find most and least used methodologies
            let mut methodology_usage: Vec<_> = usage_data.methodology_usage.iter().collect();
            methodology_usage.sort_by(|a, b| b.1.cmp(a.1));

            if let Some((most_used, most_count)) = methodology_usage.first() {
                if let Some((least_used, least_count)) = methodology_usage.last() {
                    if *most_count > *least_count * 3 {
                        recommendations.push(OptimizationRecommendation {
                            id: "balance_methodology_usage".to_string(),
                            title: "Balance Methodology Usage".to_string(),
                            description: format!("Methodology '{}' is heavily used while '{}' is underutilized", most_used, least_used),
                            category: OptimizationCategory::UserExperience,
                            priority: Priority::Medium,
                            estimated_impact: ImpactEstimate {
                                performance_improvement: Some(15.0),
                                cost_savings: Some(10.0),
                                user_satisfaction: Some(20.0),
                                description: "Better load distribution and improved user experience".to_string(),
                            },
                            implementation_effort: EffortLevel::Low,
                            created_at: Utc::now(),
                        });
                    }
                }
            }
        }

        // Analyze API usage efficiency
        for (service, stats) in &usage_data.api_usage_stats {
            if stats.success_rate < 90.0 {
                recommendations.push(OptimizationRecommendation {
                    id: format!("improve_{}_reliability", service),
                    title: format!("Improve {} Service Reliability", service),
                    description: format!("Service {} has {:.1}% success rate", service, stats.success_rate),
                    category: OptimizationCategory::Reliability,
                    priority: if stats.success_rate < 80.0 { Priority::High } else { Priority::Medium },
                    estimated_impact: ImpactEstimate {
                        performance_improvement: Some(25.0),
                        cost_savings: None,
                        user_satisfaction: Some(30.0),
                        description: "Improve service reliability and user experience".to_string(),
                    },
                    implementation_effort: EffortLevel::Medium,
                    created_at: Utc::now(),
                });
            }

            if stats.average_response_time > 2000.0 {
                recommendations.push(OptimizationRecommendation {
                    id: format!("optimize_{}_response_time", service),
                    title: format!("Optimize {} Response Time", service),
                    description: format!("Service {} has {:.2}ms average response time", service, stats.average_response_time),
                    category: OptimizationCategory::Performance,
                    priority: Priority::Medium,
                    estimated_impact: ImpactEstimate {
                        performance_improvement: Some(40.0),
                        cost_savings: None,
                        user_satisfaction: Some(25.0),
                        description: "Reduce response time and improve user experience".to_string(),
                    },
                    implementation_effort: EffortLevel::Medium,
                    created_at: Utc::now(),
                });
            }
        }

        Ok(recommendations)
    }

    /// Generate predictive recommendations
    async fn generate_predictive_recommendations(&self, predictions: &super::PredictiveAnalyticsData) -> AppResult<Vec<OptimizationRecommendation>> {
        let mut recommendations = Vec::new();

        // Check quota forecasts
        for forecast in &predictions.quota_forecasts {
            if matches!(forecast.risk_level, super::predictive_analytics::QuotaRiskLevel::High | super::predictive_analytics::QuotaRiskLevel::Critical) {
                recommendations.push(OptimizationRecommendation {
                    id: format!("optimize_{}_quota_usage", forecast.service_name),
                    title: format!("Optimize {} Quota Usage", forecast.service_name),
                    description: format!("Service {} is approaching quota limits", forecast.service_name),
                    category: OptimizationCategory::CostSavings,
                    priority: if matches!(forecast.risk_level, super::predictive_analytics::QuotaRiskLevel::Critical) { 
                        Priority::Critical 
                    } else { 
                        Priority::High 
                    },
                    estimated_impact: ImpactEstimate {
                        performance_improvement: None,
                        cost_savings: Some(20.0),
                        user_satisfaction: Some(10.0),
                        description: "Prevent quota exhaustion and maintain service availability".to_string(),
                    },
                    implementation_effort: EffortLevel::Medium,
                    created_at: Utc::now(),
                });
            }
        }

        // Check capacity planning recommendations
        for recommendation in &predictions.capacity_planning.scaling_recommendations {
            if matches!(recommendation.priority, Priority::Critical | Priority::High) {
                recommendations.push(OptimizationRecommendation {
                    id: format!("scale_{}", recommendation.resource_type.to_lowercase()),
                    title: format!("Scale {} Resources", recommendation.resource_type),
                    description: recommendation.recommended_action.clone(),
                    category: OptimizationCategory::Performance,
                    priority: recommendation.priority.clone(),
                    estimated_impact: ImpactEstimate {
                        performance_improvement: Some(35.0),
                        cost_savings: None,
                        user_satisfaction: Some(20.0),
                        description: "Prevent resource bottlenecks and maintain performance".to_string(),
                    },
                    implementation_effort: EffortLevel::High,
                    created_at: Utc::now(),
                });
            }
        }

        // Check early warnings
        for warning in &predictions.early_warnings {
            if matches!(warning.severity, super::AlertSeverity::Critical | super::AlertSeverity::Warning) {
                recommendations.push(OptimizationRecommendation {
                    id: format!("address_{:?}", warning.warning_type).to_lowercase(),
                    title: "Address Early Warning".to_string(),
                    description: warning.message.clone(),
                    category: OptimizationCategory::Reliability,
                    priority: if matches!(warning.severity, super::AlertSeverity::Critical) { 
                        Priority::Critical 
                    } else { 
                        Priority::High 
                    },
                    estimated_impact: ImpactEstimate {
                        performance_improvement: Some(25.0),
                        cost_savings: None,
                        user_satisfaction: Some(30.0),
                        description: "Prevent potential issues and maintain system stability".to_string(),
                    },
                    implementation_effort: EffortLevel::Medium,
                    created_at: Utc::now(),
                });
            }
        }

        Ok(recommendations)
    }

    /// Generate cost optimization recommendations
    async fn generate_cost_recommendations(&self, usage_data: &super::usage_analytics::UsageAnalyticsData) -> AppResult<Vec<OptimizationRecommendation>> {
        let mut recommendations = Vec::new();

        // Analyze cost savings opportunities
        if usage_data.cost_savings.total_theoretical_cost > 100.0 {
            recommendations.push(OptimizationRecommendation {
                id: "maximize_free_tier_usage".to_string(),
                title: "Maximize Free Tier Usage".to_string(),
                description: format!("Theoretical cost savings of ${:.2} achieved", usage_data.cost_savings.total_savings),
                category: OptimizationCategory::CostSavings,
                priority: Priority::Medium,
                estimated_impact: ImpactEstimate {
                    performance_improvement: None,
                    cost_savings: Some(usage_data.cost_savings.total_savings),
                    user_satisfaction: Some(15.0),
                    description: "Continue maximizing free tier benefits".to_string(),
                },
                implementation_effort: EffortLevel::Low,
                created_at: Utc::now(),
            });
        }

        // Check for inefficient API usage patterns
        let total_api_calls: u64 = usage_data.api_usage_stats.values().map(|s| s.total_calls).sum();
        let failed_calls: u64 = usage_data.api_usage_stats.values().map(|s| s.failed_calls).sum();
        
        if failed_calls > 0 && total_api_calls > 0 {
            let failure_rate = (failed_calls as f64 / total_api_calls as f64) * 100.0;
            if failure_rate > 5.0 {
                recommendations.push(OptimizationRecommendation {
                    id: "reduce_api_failures".to_string(),
                    title: "Reduce API Call Failures".to_string(),
                    description: format!("API failure rate is {:.1}%, wasting quota", failure_rate),
                    category: OptimizationCategory::CostSavings,
                    priority: Priority::Medium,
                    estimated_impact: ImpactEstimate {
                        performance_improvement: Some(20.0),
                        cost_savings: Some(15.0),
                        user_satisfaction: Some(25.0),
                        description: "Reduce wasted API calls and improve efficiency".to_string(),
                    },
                    implementation_effort: EffortLevel::Medium,
                    created_at: Utc::now(),
                });
            }
        }

        Ok(recommendations)
    }

    /// Generate security recommendations
    async fn generate_security_recommendations(&self) -> AppResult<Vec<OptimizationRecommendation>> {
        let mut recommendations = Vec::new();

        // This would integrate with the security service to generate security-related recommendations
        // For now, add some general security recommendations

        recommendations.push(OptimizationRecommendation {
            id: "regular_security_audit".to_string(),
            title: "Regular Security Audit".to_string(),
            description: "Perform regular security audits to maintain system security".to_string(),
            category: OptimizationCategory::Security,
            priority: Priority::Medium,
            estimated_impact: ImpactEstimate {
                performance_improvement: None,
                cost_savings: None,
                user_satisfaction: Some(10.0),
                description: "Maintain security posture and user trust".to_string(),
            },
            implementation_effort: EffortLevel::Low,
            created_at: Utc::now(),
        });

        Ok(recommendations)
    }

    /// Background task for generating insights
    async fn generate_insights(&self) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(self.config.insights_generation_interval_seconds));
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.update_insights().await {
                error!("Failed to update insights: {}", e);
            }
        }
    }

    /// Update business insights
    async fn update_insights(&self) -> AppResult<()> {
        info!("Updating business insights...");
        
        // Generate fresh recommendations
        let _recommendations = self.generate_optimization_recommendations().await?;
        
        // Update insights cache
        {
            let mut cache = self.insights_cache.write().await;
            cache.last_insights_update = Utc::now();
        }
        
        Ok(())
    }

    /// Background task for updating recommendations
    async fn update_recommendations(&self) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1800)); // 30 minutes
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.refresh_recommendations().await {
                error!("Failed to refresh recommendations: {}", e);
            }
        }
    }

    /// Refresh optimization recommendations
    async fn refresh_recommendations(&self) -> AppResult<()> {
        info!("Refreshing optimization recommendations...");
        
        // Clear cache to force regeneration
        {
            let mut cache = self.insights_cache.write().await;
            cache.cached_recommendations = None;
        }
        
        // Generate fresh recommendations
        let _recommendations = self.get_optimization_recommendations().await?;
        
        Ok(())
    }

    /// Background task for calculating KPIs
    async fn calculate_kpis(&self) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600)); // 1 hour
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.update_kpis().await {
                error!("Failed to update KPIs: {}", e);
            }
        }
    }

    /// Update key performance indicators
    async fn update_kpis(&self) -> AppResult<()> {
        info!("Updating KPIs...");
        
        // Calculate various KPIs
        let usage_data = self.usage_analytics.read().await.get_analytics_data(TimePeriod::Last24Hours).await?;
        let performance_metrics = self.performance_monitor.read().await.get_current_metrics().await?;
        
        // Store KPIs for reporting
        let _kpis = BusinessKPIs {
            total_research_sessions: usage_data.total_research_sessions,
            average_response_time: performance_metrics.response_times.api_average_ms,
            system_uptime: 99.9, // This would be calculated from actual uptime data
            cost_savings: usage_data.cost_savings.total_savings,
            user_satisfaction: 85.0, // This would be calculated from user feedback
            calculated_at: Utc::now(),
        };
        
        Ok(())
    }

    /// Background task for trend analysis
    async fn trend_analysis(&self) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(7200)); // 2 hours
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.analyze_trends().await {
                error!("Failed to analyze trends: {}", e);
            }
        }
    }

    /// Analyze business trends
    async fn analyze_trends(&self) -> AppResult<()> {
        info!("Analyzing business trends...");
        
        // Analyze usage trends
        let usage_data = self.usage_analytics.read().await.get_analytics_data(TimePeriod::LastWeek).await?;
        
        // Analyze performance trends
        let performance_monitor = self.performance_monitor.read().await;
        let _performance_trends = performance_monitor.get_performance_trends().await?;
        
        // Generate trend insights
        // This would analyze patterns and generate insights about business trends
        
        Ok(())
    }

    /// Health check
    pub async fn health_check(&self) -> AppResult<()> {
        let cache = self.insights_cache.read().await;
        
        // Check if insights are being updated regularly
        if let Some(last_update) = cache.last_insights_update {
            let age = Utc::now() - last_update;
            if age > Duration::hours(2) {
                return Err(AppError::Analytics("Business insights are stale".to_string()));
            }
        }
        
        Ok(())
    }

    /// Shutdown
    pub async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down business intelligence engine...");
        info!("Business intelligence engine shutdown complete");
        Ok(())
    }
}

/// Business intelligence configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessIntelligenceConfig {
    pub insights_generation_interval_seconds: u64,
    pub cache_duration_minutes: u64,
    pub enable_trend_analysis: bool,
    pub enable_kpi_calculation: bool,
    pub recommendation_refresh_interval_seconds: u64,
}

impl Default for BusinessIntelligenceConfig {
    fn default() -> Self {
        Self {
            insights_generation_interval_seconds: 3600, // 1 hour
            cache_duration_minutes: 30,
            enable_trend_analysis: true,
            enable_kpi_calculation: true,
            recommendation_refresh_interval_seconds: 1800, // 30 minutes
        }
    }
}

/// Business key performance indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessKPIs {
    pub total_research_sessions: u64,
    pub average_response_time: f64,
    pub system_uptime: f64,
    pub cost_savings: f64,
    pub user_satisfaction: f64,
    pub calculated_at: DateTime<Utc>,
}

/// Insights cache for performance optimization
#[derive(Debug, Clone)]
struct InsightsCache {
    cached_recommendations: Option<CachedRecommendations>,
    last_insights_update: Option<DateTime<Utc>>,
}

impl InsightsCache {
    fn new() -> Self {
        Self {
            cached_recommendations: None,
            last_insights_update: None,
        }
    }
}

/// Cached recommendations
#[derive(Debug, Clone)]
struct CachedRecommendations {
    recommendations: Vec<OptimizationRecommendation>,
    generated_at: DateTime<Utc>,
}
