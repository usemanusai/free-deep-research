use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, warn, error};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, ResearchError};

/// Intelligent recommendation system for research optimization
#[derive(Clone)]
pub struct RecommendationEngine {
    user_profiles: Arc<RwLock<HashMap<Uuid, UserProfile>>>,
    recommendation_cache: Arc<RwLock<RecommendationCache>>,
    config: RecommendationConfig,
    recommendation_metrics: Arc<RwLock<RecommendationMetrics>>,
}

impl RecommendationEngine {
    pub fn new(config: RecommendationConfig) -> Self {
        Self {
            user_profiles: Arc::new(RwLock::new(HashMap::new())),
            recommendation_cache: Arc::new(RwLock::new(RecommendationCache::new())),
            config,
            recommendation_metrics: Arc::new(RwLock::new(RecommendationMetrics::new())),
        }
    }

    /// Generate personalized recommendations for a user
    pub async fn generate_recommendations(&self, user_id: Uuid, context: RecommendationContext) -> AppResult<Vec<Recommendation>> {
        info!("Generating recommendations for user: {}", user_id);

        // Get or create user profile
        let user_profile = self.get_or_create_user_profile(user_id).await?;

        let mut recommendations = Vec::new();

        // Generate methodology recommendations
        if let Some(methodology_recs) = self.generate_methodology_recommendations(&user_profile, &context).await? {
            recommendations.extend(methodology_recs);
        }

        // Generate API optimization recommendations
        if let Some(api_recs) = self.generate_api_recommendations(&user_profile, &context).await? {
            recommendations.extend(api_recs);
        }

        // Generate workflow optimization recommendations
        if let Some(workflow_recs) = self.generate_workflow_recommendations(&user_profile, &context).await? {
            recommendations.extend(workflow_recs);
        }

        // Generate cost optimization recommendations
        if let Some(cost_recs) = self.generate_cost_recommendations(&user_profile, &context).await? {
            recommendations.extend(cost_recs);
        }

        // Generate performance recommendations
        if let Some(performance_recs) = self.generate_performance_recommendations(&user_profile, &context).await? {
            recommendations.extend(performance_recs);
        }

        // Sort by relevance score
        recommendations.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap_or(std::cmp::Ordering::Equal));

        // Limit to max recommendations
        recommendations.truncate(self.config.max_recommendations_per_request);

        // Update metrics
        self.update_recommendation_metrics(recommendations.len()).await?;

        info!("Generated {} recommendations for user: {}", recommendations.len(), user_id);
        Ok(recommendations)
    }

    /// Generate methodology recommendations
    async fn generate_methodology_recommendations(&self, profile: &UserProfile, context: &RecommendationContext) -> AppResult<Option<Vec<Recommendation>>> {
        let mut recommendations = Vec::new();

        // Analyze user's methodology usage patterns
        let current_methodology = context.current_methodology.as_deref().unwrap_or("hybrid");
        let success_rate = profile.methodology_success_rates.get(current_methodology).unwrap_or(&0.7);

        // Recommend better methodology if current success rate is low
        if *success_rate < 0.8 {
            let best_methodology = profile.methodology_success_rates
                .iter()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(method, rate)| (method.clone(), *rate));

            if let Some((best_method, best_rate)) = best_methodology {
                if best_rate > success_rate + 0.1 {
                    recommendations.push(Recommendation {
                        id: Uuid::new_v4(),
                        recommendation_type: RecommendationType::Methodology,
                        title: format!("Switch to {} Methodology", best_method.to_uppercase()),
                        description: format!(
                            "Based on your research patterns, {} methodology could improve your success rate from {:.1}% to {:.1}%",
                            best_method, success_rate * 100.0, best_rate * 100.0
                        ),
                        relevance_score: 0.9,
                        confidence: 0.85,
                        impact_estimate: format!("{:.1}% improvement in success rate", (best_rate - success_rate) * 100.0),
                        action_required: ActionType::MethodologyChange,
                        parameters: serde_json::json!({
                            "recommended_methodology": best_method,
                            "current_success_rate": success_rate,
                            "projected_success_rate": best_rate
                        }),
                        created_at: Utc::now(),
                        expires_at: Some(Utc::now() + chrono::Duration::days(7)),
                    });
                }
            }
        }

        // Recommend methodology based on query complexity
        if let Some(complexity) = context.query_complexity {
            let recommended_methodology = if complexity > 0.8 {
                "hybrid"
            } else if complexity > 0.5 {
                "nick_scamara"
            } else {
                "don_lim"
            };

            if recommended_methodology != current_methodology {
                recommendations.push(Recommendation {
                    id: Uuid::new_v4(),
                    recommendation_type: RecommendationType::Methodology,
                    title: format!("Use {} for This Query", recommended_methodology.to_uppercase()),
                    description: format!(
                        "For queries with {:.1}% complexity, {} methodology typically performs better",
                        complexity * 100.0, recommended_methodology
                    ),
                    relevance_score: 0.75,
                    confidence: 0.78,
                    impact_estimate: "15-25% better results".to_string(),
                    action_required: ActionType::MethodologyChange,
                    parameters: serde_json::json!({
                        "recommended_methodology": recommended_methodology,
                        "query_complexity": complexity,
                        "reason": "complexity_optimization"
                    }),
                    created_at: Utc::now(),
                    expires_at: Some(Utc::now() + chrono::Duration::hours(24)),
                });
            }
        }

        Ok(if recommendations.is_empty() { None } else { Some(recommendations) })
    }

    /// Generate API optimization recommendations
    async fn generate_api_recommendations(&self, profile: &UserProfile, context: &RecommendationContext) -> AppResult<Option<Vec<Recommendation>>> {
        let mut recommendations = Vec::new();

        // Analyze API usage patterns
        let total_api_calls: u64 = profile.api_usage_stats.values().sum();
        
        if total_api_calls > 0 {
            // Find most used API
            let most_used_api = profile.api_usage_stats
                .iter()
                .max_by_key(|(_, &count)| count)
                .map(|(api, count)| (api.clone(), *count));

            if let Some((api_name, usage_count)) = most_used_api {
                let usage_percentage = (usage_count as f64 / total_api_calls as f64) * 100.0;
                
                // Recommend API optimization if heavily used
                if usage_percentage > 50.0 {
                    recommendations.push(Recommendation {
                        id: Uuid::new_v4(),
                        recommendation_type: RecommendationType::ApiOptimization,
                        title: format!("Optimize {} API Usage", api_name),
                        description: format!(
                            "{} accounts for {:.1}% of your API usage. Consider implementing caching or rate limiting.",
                            api_name, usage_percentage
                        ),
                        relevance_score: 0.8,
                        confidence: 0.82,
                        impact_estimate: "20-30% cost reduction".to_string(),
                        action_required: ActionType::ApiConfiguration,
                        parameters: serde_json::json!({
                            "api_name": api_name,
                            "usage_percentage": usage_percentage,
                            "optimization_type": "caching_and_rate_limiting"
                        }),
                        created_at: Utc::now(),
                        expires_at: Some(Utc::now() + chrono::Duration::days(14)),
                    });
                }
            }

            // Recommend alternative APIs if error rates are high
            for (api_name, error_rate) in &profile.api_error_rates {
                if *error_rate > 0.1 {
                    let alternative_api = self.suggest_alternative_api(api_name).await?;
                    
                    recommendations.push(Recommendation {
                        id: Uuid::new_v4(),
                        recommendation_type: RecommendationType::ApiOptimization,
                        title: format!("Consider Alternative to {}", api_name),
                        description: format!(
                            "{} has a {:.1}% error rate. Consider switching to {} for better reliability.",
                            api_name, error_rate * 100.0, alternative_api
                        ),
                        relevance_score: 0.7,
                        confidence: 0.75,
                        impact_estimate: format!("Reduce error rate from {:.1}% to ~2%", error_rate * 100.0),
                        action_required: ActionType::ApiConfiguration,
                        parameters: serde_json::json!({
                            "current_api": api_name,
                            "alternative_api": alternative_api,
                            "current_error_rate": error_rate
                        }),
                        created_at: Utc::now(),
                        expires_at: Some(Utc::now() + chrono::Duration::days(30)),
                    });
                }
            }
        }

        Ok(if recommendations.is_empty() { None } else { Some(recommendations) })
    }

    /// Generate workflow optimization recommendations
    async fn generate_workflow_recommendations(&self, profile: &UserProfile, _context: &RecommendationContext) -> AppResult<Option<Vec<Recommendation>>> {
        let mut recommendations = Vec::new();

        // Analyze research timing patterns
        if let Some(peak_hour) = profile.peak_usage_hours.first() {
            recommendations.push(Recommendation {
                id: Uuid::new_v4(),
                recommendation_type: RecommendationType::WorkflowOptimization,
                title: "Optimize Research Timing".to_string(),
                description: format!(
                    "You typically research at {}:00. Consider scheduling during off-peak hours (2-6 AM) for 15% cost savings.",
                    peak_hour
                ),
                relevance_score: 0.6,
                confidence: 0.68,
                impact_estimate: "15% cost reduction".to_string(),
                action_required: ActionType::SchedulingChange,
                parameters: serde_json::json!({
                    "current_peak_hour": peak_hour,
                    "recommended_hours": [2, 3, 4, 5, 6],
                    "estimated_savings": 0.15
                }),
                created_at: Utc::now(),
                expires_at: Some(Utc::now() + chrono::Duration::days(7)),
            });
        }

        // Recommend batch processing if user does many small queries
        if profile.average_query_length < 50.0 && profile.total_research_sessions > 20 {
            recommendations.push(Recommendation {
                id: Uuid::new_v4(),
                recommendation_type: RecommendationType::WorkflowOptimization,
                title: "Consider Batch Processing".to_string(),
                description: "Your queries are typically short. Combining related queries could improve efficiency by 25%.".to_string(),
                relevance_score: 0.65,
                confidence: 0.72,
                impact_estimate: "25% efficiency improvement".to_string(),
                action_required: ActionType::WorkflowChange,
                parameters: serde_json::json!({
                    "average_query_length": profile.average_query_length,
                    "total_sessions": profile.total_research_sessions,
                    "optimization_type": "batch_processing"
                }),
                created_at: Utc::now(),
                expires_at: Some(Utc::now() + chrono::Duration::days(14)),
            });
        }

        Ok(if recommendations.is_empty() { None } else { Some(recommendations) })
    }

    /// Generate cost optimization recommendations
    async fn generate_cost_recommendations(&self, profile: &UserProfile, _context: &RecommendationContext) -> AppResult<Option<Vec<Recommendation>>> {
        let mut recommendations = Vec::new();

        // Analyze spending patterns
        if profile.monthly_api_cost > 50.0 {
            recommendations.push(Recommendation {
                id: Uuid::new_v4(),
                recommendation_type: RecommendationType::CostOptimization,
                title: "API Cost Optimization Opportunity".to_string(),
                description: format!(
                    "Your monthly API costs are ${:.2}. Implementing smart caching could reduce this by 30%.",
                    profile.monthly_api_cost
                ),
                relevance_score: 0.85,
                confidence: 0.8,
                impact_estimate: format!("Save ${:.2}/month", profile.monthly_api_cost * 0.3),
                action_required: ActionType::CostOptimization,
                parameters: serde_json::json!({
                    "current_monthly_cost": profile.monthly_api_cost,
                    "potential_savings": profile.monthly_api_cost * 0.3,
                    "optimization_method": "smart_caching"
                }),
                created_at: Utc::now(),
                expires_at: Some(Utc::now() + chrono::Duration::days(30)),
            });
        }

        Ok(if recommendations.is_empty() { None } else { Some(recommendations) })
    }

    /// Generate performance recommendations
    async fn generate_performance_recommendations(&self, profile: &UserProfile, _context: &RecommendationContext) -> AppResult<Option<Vec<Recommendation>>> {
        let mut recommendations = Vec::new();

        // Analyze response times
        if profile.average_response_time > 5000.0 {
            recommendations.push(Recommendation {
                id: Uuid::new_v4(),
                recommendation_type: RecommendationType::PerformanceOptimization,
                title: "Improve Response Times".to_string(),
                description: format!(
                    "Your average response time is {:.1}s. Optimizing query structure could reduce this to ~3s.",
                    profile.average_response_time / 1000.0
                ),
                relevance_score: 0.75,
                confidence: 0.77,
                impact_estimate: "40% faster responses".to_string(),
                action_required: ActionType::PerformanceOptimization,
                parameters: serde_json::json!({
                    "current_response_time": profile.average_response_time,
                    "target_response_time": 3000.0,
                    "optimization_method": "query_structure"
                }),
                created_at: Utc::now(),
                expires_at: Some(Utc::now() + chrono::Duration::days(14)),
            });
        }

        Ok(if recommendations.is_empty() { None } else { Some(recommendations) })
    }

    /// Get or create user profile
    async fn get_or_create_user_profile(&self, user_id: Uuid) -> AppResult<UserProfile> {
        let mut profiles = self.user_profiles.write().await;
        
        if let Some(profile) = profiles.get(&user_id) {
            Ok(profile.clone())
        } else {
            let new_profile = UserProfile::new(user_id);
            profiles.insert(user_id, new_profile.clone());
            Ok(new_profile)
        }
    }

    /// Suggest alternative API
    async fn suggest_alternative_api(&self, current_api: &str) -> AppResult<String> {
        let alternative = match current_api {
            "openrouter" => "anthropic",
            "anthropic" => "openai",
            "openai" => "openrouter",
            "tavily" => "exa",
            "exa" => "tavily",
            "jina" => "firecrawl",
            "firecrawl" => "jina",
            _ => "openrouter",
        };
        Ok(alternative.to_string())
    }

    /// Update recommendation metrics
    async fn update_recommendation_metrics(&self, recommendations_count: usize) -> AppResult<()> {
        let mut metrics = self.recommendation_metrics.write().await;
        metrics.total_recommendations_generated += recommendations_count as u64;
        metrics.total_recommendation_requests += 1;
        metrics.average_recommendations_per_request = 
            metrics.total_recommendations_generated as f64 / metrics.total_recommendation_requests as f64;
        metrics.last_recommendation_generated = Utc::now();
        Ok(())
    }

    /// Get recommendation metrics
    pub async fn get_metrics(&self) -> AppResult<RecommendationMetrics> {
        let metrics = self.recommendation_metrics.read().await;
        Ok(metrics.clone())
    }
}

// Supporting data structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationConfig {
    pub max_recommendations_per_request: usize,
    pub min_confidence_threshold: f64,
    pub cache_ttl_seconds: u64,
    pub personalization_weight: f64,
}

impl Default for RecommendationConfig {
    fn default() -> Self {
        Self {
            max_recommendations_per_request: 10,
            min_confidence_threshold: 0.6,
            cache_ttl_seconds: 3600,
            personalization_weight: 0.8,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationContext {
    pub current_methodology: Option<String>,
    pub query_complexity: Option<f64>,
    pub recent_performance: Option<f64>,
    pub budget_constraints: Option<f64>,
    pub time_constraints: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: Uuid,
    pub recommendation_type: RecommendationType,
    pub title: String,
    pub description: String,
    pub relevance_score: f64,
    pub confidence: f64,
    pub impact_estimate: String,
    pub action_required: ActionType,
    pub parameters: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    Methodology,
    ApiOptimization,
    WorkflowOptimization,
    CostOptimization,
    PerformanceOptimization,
    SecurityOptimization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    MethodologyChange,
    ApiConfiguration,
    SchedulingChange,
    WorkflowChange,
    CostOptimization,
    PerformanceOptimization,
    SecurityUpdate,
}

#[derive(Debug, Clone)]
pub struct UserProfile {
    pub user_id: Uuid,
    pub methodology_success_rates: HashMap<String, f64>,
    pub api_usage_stats: HashMap<String, u64>,
    pub api_error_rates: HashMap<String, f64>,
    pub peak_usage_hours: Vec<u32>,
    pub average_query_length: f64,
    pub average_response_time: f64,
    pub total_research_sessions: u64,
    pub monthly_api_cost: f64,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl UserProfile {
    fn new(user_id: Uuid) -> Self {
        let mut methodology_success_rates = HashMap::new();
        methodology_success_rates.insert("hybrid".to_string(), 0.75);
        methodology_success_rates.insert("don_lim".to_string(), 0.70);
        methodology_success_rates.insert("nick_scamara".to_string(), 0.72);

        let mut api_usage_stats = HashMap::new();
        api_usage_stats.insert("openrouter".to_string(), 45);
        api_usage_stats.insert("tavily".to_string(), 30);
        api_usage_stats.insert("exa".to_string(), 25);

        let mut api_error_rates = HashMap::new();
        api_error_rates.insert("openrouter".to_string(), 0.05);
        api_error_rates.insert("tavily".to_string(), 0.03);
        api_error_rates.insert("exa".to_string(), 0.08);

        Self {
            user_id,
            methodology_success_rates,
            api_usage_stats,
            api_error_rates,
            peak_usage_hours: vec![14, 15, 16], // 2-4 PM
            average_query_length: 85.0,
            average_response_time: 3500.0, // 3.5 seconds
            total_research_sessions: 15,
            monthly_api_cost: 35.50,
            created_at: Utc::now(),
            last_updated: Utc::now(),
        }
    }
}

#[derive(Debug, Clone)]
struct RecommendationCache {
    cache: HashMap<String, (Vec<Recommendation>, DateTime<Utc>)>,
    ttl_seconds: u64,
}

impl RecommendationCache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
            ttl_seconds: 3600,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationMetrics {
    pub total_recommendations_generated: u64,
    pub total_recommendation_requests: u64,
    pub average_recommendations_per_request: f64,
    pub recommendation_acceptance_rate: f64,
    pub last_recommendation_generated: DateTime<Utc>,
}

impl RecommendationMetrics {
    fn new() -> Self {
        Self {
            total_recommendations_generated: 0,
            total_recommendation_requests: 0,
            average_recommendations_per_request: 0.0,
            recommendation_acceptance_rate: 0.0,
            last_recommendation_generated: Utc::now(),
        }
    }
}
