use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, warn, error};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, ResearchError};

/// Pattern analysis engine for discovering research patterns and insights
#[derive(Clone)]
pub struct PatternAnalyzer {
    discovered_patterns: Arc<RwLock<HashMap<String, ResearchPattern>>>,
    pattern_cache: Arc<RwLock<PatternCache>>,
    config: AnalysisConfig,
    analysis_metrics: Arc<RwLock<AnalysisMetrics>>,
}

impl PatternAnalyzer {
    pub fn new(config: AnalysisConfig) -> Self {
        Self {
            discovered_patterns: Arc::new(RwLock::new(HashMap::new())),
            pattern_cache: Arc::new(RwLock::new(PatternCache::new())),
            config,
            analysis_metrics: Arc::new(RwLock::new(AnalysisMetrics::new())),
        }
    }

    /// Analyze research data to discover patterns
    pub async fn analyze_patterns(&self, data: AnalysisInput) -> AppResult<Vec<PatternInsight>> {
        info!("Starting pattern analysis for {} data points", data.data_points.len());

        let mut insights = Vec::new();

        // Analyze temporal patterns
        if let Some(temporal_insights) = self.analyze_temporal_patterns(&data).await? {
            insights.extend(temporal_insights);
        }

        // Analyze usage patterns
        if let Some(usage_insights) = self.analyze_usage_patterns(&data).await? {
            insights.extend(usage_insights);
        }

        // Analyze success patterns
        if let Some(success_insights) = self.analyze_success_patterns(&data).await? {
            insights.extend(success_insights);
        }

        // Analyze methodology patterns
        if let Some(methodology_insights) = self.analyze_methodology_patterns(&data).await? {
            insights.extend(methodology_insights);
        }

        // Update metrics
        self.update_analysis_metrics(insights.len()).await?;

        info!("Pattern analysis completed. Found {} insights", insights.len());
        Ok(insights)
    }

    /// Analyze temporal patterns in research data
    async fn analyze_temporal_patterns(&self, data: &AnalysisInput) -> AppResult<Option<Vec<PatternInsight>>> {
        debug!("Analyzing temporal patterns");

        let mut hourly_usage = HashMap::new();
        let mut daily_usage = HashMap::new();

        // Aggregate usage by time periods
        for point in &data.data_points {
            if let Some(timestamp) = point.get("timestamp").and_then(|t| t.as_str()) {
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(timestamp) {
                    let hour = dt.hour();
                    let day = dt.weekday().number_from_monday();
                    
                    *hourly_usage.entry(hour).or_insert(0) += 1;
                    *daily_usage.entry(day).or_insert(0) += 1;
                }
            }
        }

        let mut insights = Vec::new();

        // Find peak usage hours
        if let Some((peak_hour, peak_count)) = hourly_usage.iter().max_by_key(|(_, &count)| count) {
            insights.push(PatternInsight {
                pattern_id: Uuid::new_v4(),
                pattern_type: PatternType::Temporal,
                title: "Peak Usage Hour Identified".to_string(),
                description: format!("Peak usage occurs at {}:00 with {} sessions", peak_hour, peak_count),
                confidence: 0.85,
                impact_score: 0.7,
                actionable_recommendations: vec![
                    "Consider scaling resources during peak hours".to_string(),
                    "Schedule maintenance outside peak hours".to_string(),
                ],
                supporting_data: serde_json::json!({
                    "peak_hour": peak_hour,
                    "peak_count": peak_count,
                    "hourly_distribution": hourly_usage
                }),
                discovered_at: Utc::now(),
            });
        }

        // Find peak usage days
        if let Some((peak_day, peak_count)) = daily_usage.iter().max_by_key(|(_, &count)| count) {
            let day_name = match peak_day {
                1 => "Monday", 2 => "Tuesday", 3 => "Wednesday", 4 => "Thursday",
                5 => "Friday", 6 => "Saturday", 7 => "Sunday", _ => "Unknown"
            };
            
            insights.push(PatternInsight {
                pattern_id: Uuid::new_v4(),
                pattern_type: PatternType::Temporal,
                title: "Peak Usage Day Identified".to_string(),
                description: format!("Peak usage occurs on {} with {} sessions", day_name, peak_count),
                confidence: 0.82,
                impact_score: 0.6,
                actionable_recommendations: vec![
                    format!("Optimize resources for {} usage patterns", day_name),
                    "Consider promotional activities on low-usage days".to_string(),
                ],
                supporting_data: serde_json::json!({
                    "peak_day": peak_day,
                    "peak_day_name": day_name,
                    "peak_count": peak_count,
                    "daily_distribution": daily_usage
                }),
                discovered_at: Utc::now(),
            });
        }

        Ok(if insights.is_empty() { None } else { Some(insights) })
    }

    /// Analyze usage patterns
    async fn analyze_usage_patterns(&self, data: &AnalysisInput) -> AppResult<Option<Vec<PatternInsight>>> {
        debug!("Analyzing usage patterns");

        let mut api_usage = HashMap::new();
        let mut methodology_usage = HashMap::new();
        let mut query_lengths = Vec::new();

        // Aggregate usage data
        for point in &data.data_points {
            if let Some(api) = point.get("api_service").and_then(|a| a.as_str()) {
                *api_usage.entry(api.to_string()).or_insert(0) += 1;
            }
            
            if let Some(methodology) = point.get("methodology").and_then(|m| m.as_str()) {
                *methodology_usage.entry(methodology.to_string()).or_insert(0) += 1;
            }
            
            if let Some(query) = point.get("query").and_then(|q| q.as_str()) {
                query_lengths.push(query.len());
            }
        }

        let mut insights = Vec::new();

        // Most used API service
        if let Some((most_used_api, usage_count)) = api_usage.iter().max_by_key(|(_, &count)| count) {
            let total_usage: u32 = api_usage.values().sum();
            let usage_percentage = (*usage_count as f64 / total_usage as f64) * 100.0;
            
            insights.push(PatternInsight {
                pattern_id: Uuid::new_v4(),
                pattern_type: PatternType::Usage,
                title: "Dominant API Service Identified".to_string(),
                description: format!("{} accounts for {:.1}% of all API usage", most_used_api, usage_percentage),
                confidence: 0.9,
                impact_score: 0.8,
                actionable_recommendations: vec![
                    format!("Optimize {} integration for better performance", most_used_api),
                    "Consider negotiating better rates for high-usage APIs".to_string(),
                    "Implement caching for frequently used API calls".to_string(),
                ],
                supporting_data: serde_json::json!({
                    "most_used_api": most_used_api,
                    "usage_count": usage_count,
                    "usage_percentage": usage_percentage,
                    "api_distribution": api_usage
                }),
                discovered_at: Utc::now(),
            });
        }

        // Query complexity analysis
        if !query_lengths.is_empty() {
            let avg_length = query_lengths.iter().sum::<usize>() as f64 / query_lengths.len() as f64;
            let max_length = *query_lengths.iter().max().unwrap_or(&0);
            
            insights.push(PatternInsight {
                pattern_id: Uuid::new_v4(),
                pattern_type: PatternType::Usage,
                title: "Query Complexity Pattern".to_string(),
                description: format!("Average query length: {:.1} characters, Max: {} characters", avg_length, max_length),
                confidence: 0.75,
                impact_score: 0.5,
                actionable_recommendations: vec![
                    "Provide query optimization suggestions for long queries".to_string(),
                    "Consider implementing query templates for common patterns".to_string(),
                ],
                supporting_data: serde_json::json!({
                    "average_length": avg_length,
                    "max_length": max_length,
                    "total_queries": query_lengths.len()
                }),
                discovered_at: Utc::now(),
            });
        }

        Ok(if insights.is_empty() { None } else { Some(insights) })
    }

    /// Analyze success patterns
    async fn analyze_success_patterns(&self, data: &AnalysisInput) -> AppResult<Option<Vec<PatternInsight>>> {
        debug!("Analyzing success patterns");

        let mut success_by_methodology = HashMap::new();
        let mut success_by_complexity = HashMap::new();

        for point in &data.data_points {
            let success = point.get("success").and_then(|s| s.as_bool()).unwrap_or(false);
            
            if let Some(methodology) = point.get("methodology").and_then(|m| m.as_str()) {
                let entry = success_by_methodology.entry(methodology.to_string()).or_insert((0, 0));
                entry.1 += 1; // Total count
                if success {
                    entry.0 += 1; // Success count
                }
            }
            
            if let Some(complexity) = point.get("complexity").and_then(|c| c.as_f64()) {
                let complexity_bucket = if complexity < 0.3 { "low" } else if complexity < 0.7 { "medium" } else { "high" };
                let entry = success_by_complexity.entry(complexity_bucket.to_string()).or_insert((0, 0));
                entry.1 += 1;
                if success {
                    entry.0 += 1;
                }
            }
        }

        let mut insights = Vec::new();

        // Best performing methodology
        let mut best_methodology = None;
        let mut best_success_rate = 0.0;
        
        for (methodology, (successes, total)) in &success_by_methodology {
            if *total > 5 { // Only consider methodologies with sufficient data
                let success_rate = *successes as f64 / *total as f64;
                if success_rate > best_success_rate {
                    best_success_rate = success_rate;
                    best_methodology = Some((methodology.clone(), *successes, *total));
                }
            }
        }

        if let Some((methodology, successes, total)) = best_methodology {
            insights.push(PatternInsight {
                pattern_id: Uuid::new_v4(),
                pattern_type: PatternType::Success,
                title: "Best Performing Methodology".to_string(),
                description: format!("{} methodology has the highest success rate at {:.1}%", 
                    methodology, best_success_rate * 100.0),
                confidence: 0.88,
                impact_score: 0.9,
                actionable_recommendations: vec![
                    format!("Recommend {} methodology for new users", methodology),
                    "Analyze what makes this methodology more successful".to_string(),
                    "Consider making this the default methodology".to_string(),
                ],
                supporting_data: serde_json::json!({
                    "methodology": methodology,
                    "success_rate": best_success_rate,
                    "successes": successes,
                    "total_attempts": total,
                    "all_methodologies": success_by_methodology
                }),
                discovered_at: Utc::now(),
            });
        }

        Ok(if insights.is_empty() { None } else { Some(insights) })
    }

    /// Analyze methodology patterns
    async fn analyze_methodology_patterns(&self, data: &AnalysisInput) -> AppResult<Option<Vec<PatternInsight>>> {
        debug!("Analyzing methodology patterns");

        let mut methodology_performance = HashMap::new();
        let mut methodology_usage_time = HashMap::new();

        for point in &data.data_points {
            if let Some(methodology) = point.get("methodology").and_then(|m| m.as_str()) {
                let duration = point.get("duration_ms").and_then(|d| d.as_f64()).unwrap_or(0.0);
                let quality_score = point.get("quality_score").and_then(|q| q.as_f64()).unwrap_or(0.0);
                
                let entry = methodology_performance.entry(methodology.to_string()).or_insert(Vec::new());
                entry.push((duration, quality_score));
                
                let time_entry = methodology_usage_time.entry(methodology.to_string()).or_insert(Vec::new());
                time_entry.push(duration);
            }
        }

        let mut insights = Vec::new();

        // Fastest methodology
        let mut fastest_methodology = None;
        let mut fastest_avg_time = f64::INFINITY;

        for (methodology, times) in &methodology_usage_time {
            if times.len() > 3 {
                let avg_time = times.iter().sum::<f64>() / times.len() as f64;
                if avg_time < fastest_avg_time {
                    fastest_avg_time = avg_time;
                    fastest_methodology = Some(methodology.clone());
                }
            }
        }

        if let Some(methodology) = fastest_methodology {
            insights.push(PatternInsight {
                pattern_id: Uuid::new_v4(),
                pattern_type: PatternType::Methodology,
                title: "Fastest Methodology Identified".to_string(),
                description: format!("{} methodology is the fastest with average completion time of {:.1} seconds", 
                    methodology, fastest_avg_time / 1000.0),
                confidence: 0.83,
                impact_score: 0.7,
                actionable_recommendations: vec![
                    format!("Recommend {} for time-sensitive research", methodology),
                    "Investigate why this methodology is faster".to_string(),
                    "Consider optimizing other methodologies based on these insights".to_string(),
                ],
                supporting_data: serde_json::json!({
                    "methodology": methodology,
                    "average_time_ms": fastest_avg_time,
                    "average_time_seconds": fastest_avg_time / 1000.0
                }),
                discovered_at: Utc::now(),
            });
        }

        Ok(if insights.is_empty() { None } else { Some(insights) })
    }

    /// Update analysis metrics
    async fn update_analysis_metrics(&self, insights_count: usize) -> AppResult<()> {
        let mut metrics = self.analysis_metrics.write().await;
        metrics.total_analyses += 1;
        metrics.total_insights_discovered += insights_count as u64;
        metrics.last_analysis = Utc::now();
        Ok(())
    }

    /// Get discovered patterns
    pub async fn get_patterns(&self) -> AppResult<Vec<ResearchPattern>> {
        let patterns = self.discovered_patterns.read().await;
        Ok(patterns.values().cloned().collect())
    }

    /// Get analysis metrics
    pub async fn get_metrics(&self) -> AppResult<AnalysisMetrics> {
        let metrics = self.analysis_metrics.read().await;
        Ok(metrics.clone())
    }
}

// Supporting data structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    pub min_data_points: usize,
    pub confidence_threshold: f64,
    pub pattern_cache_ttl: u64,
    pub max_insights_per_analysis: usize,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            min_data_points: 10,
            confidence_threshold: 0.7,
            pattern_cache_ttl: 3600,
            max_insights_per_analysis: 20,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisInput {
    pub data_points: Vec<serde_json::Value>,
    pub analysis_type: String,
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternInsight {
    pub pattern_id: Uuid,
    pub pattern_type: PatternType,
    pub title: String,
    pub description: String,
    pub confidence: f64,
    pub impact_score: f64,
    pub actionable_recommendations: Vec<String>,
    pub supporting_data: serde_json::Value,
    pub discovered_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    Temporal,
    Usage,
    Success,
    Methodology,
    Performance,
    Cost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchPattern {
    pub pattern_id: Uuid,
    pub name: String,
    pub description: String,
    pub pattern_type: PatternType,
    pub confidence: f64,
    pub occurrences: u64,
    pub first_observed: DateTime<Utc>,
    pub last_observed: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone)]
struct PatternCache {
    cache: HashMap<String, (Vec<PatternInsight>, DateTime<Utc>)>,
    ttl_seconds: u64,
}

impl PatternCache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
            ttl_seconds: 3600,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisMetrics {
    pub total_analyses: u64,
    pub total_insights_discovered: u64,
    pub average_insights_per_analysis: f64,
    pub last_analysis: DateTime<Utc>,
}

impl AnalysisMetrics {
    fn new() -> Self {
        Self {
            total_analyses: 0,
            total_insights_discovered: 0,
            average_insights_per_analysis: 0.0,
            last_analysis: Utc::now(),
        }
    }
}
