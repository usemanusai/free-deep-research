use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, warn, error};
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::error::{AppResult, ApiError};
use crate::models::api_key::{ServiceProvider, ApiKey};
use crate::services::api_manager::integrations::openrouter::{OpenRouterModel, OpenRouterIntegration};

/// Enhanced model management service for V1.1.0
pub struct ModelManager {
    openrouter_integration: Arc<OpenRouterIntegration>,
    model_cache: Arc<RwLock<ModelCache>>,
    model_configs: Arc<RwLock<HashMap<String, ModelConfiguration>>>,
    performance_metrics: Arc<RwLock<HashMap<String, ModelPerformanceMetrics>>>,
}

/// Model cache with TTL
#[derive(Debug, Clone)]
pub struct ModelCache {
    pub models: HashMap<ServiceProvider, Vec<OpenRouterModel>>,
    pub last_updated: HashMap<ServiceProvider, DateTime<Utc>>,
    pub cache_ttl_hours: u32,
}

/// Model configuration for fine-tuning parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfiguration {
    pub model_id: String,
    pub display_name: String,
    pub provider: ServiceProvider,
    pub default_temperature: f32,
    pub default_max_tokens: u32,
    pub default_top_p: f32,
    pub supports_function_calling: bool,
    pub supports_streaming: bool,
    pub context_window: u32,
    pub cost_per_1k_tokens: f64,
    pub recommended_use_cases: Vec<String>,
    pub performance_tier: ModelTier,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Model performance tiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelTier {
    Premium,    // Claude 3.5 Sonnet, GPT-4 Turbo
    Standard,   // GPT-4, Claude 3 Opus
    Efficient,  // GPT-3.5 Turbo, Claude 3 Haiku
    Specialized, // Code-specific, Math-specific models
}

/// Model performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPerformanceMetrics {
    pub model_id: String,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
    pub average_tokens_per_request: f64,
    pub total_cost: f64,
    pub quality_score: Option<f64>,
    pub last_used: DateTime<Utc>,
    pub usage_trend: Vec<UsageDataPoint>,
}

/// Usage data point for trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageDataPoint {
    pub timestamp: DateTime<Utc>,
    pub requests_count: u32,
    pub average_response_time: f64,
    pub success_rate: f64,
}

/// Model recommendation based on use case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRecommendation {
    pub model_id: String,
    pub confidence_score: f64,
    pub reasoning: String,
    pub estimated_cost: f64,
    pub estimated_performance: ModelPerformanceEstimate,
}

/// Performance estimate for a model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPerformanceEstimate {
    pub expected_response_time_ms: f64,
    pub expected_quality_score: f64,
    pub expected_success_rate: f64,
}

impl ModelManager {
    /// Create a new model manager
    pub async fn new() -> AppResult<Self> {
        info!("Initializing enhanced model manager...");

        let openrouter_integration = Arc::new(OpenRouterIntegration::new());
        let model_cache = Arc::new(RwLock::new(ModelCache {
            models: HashMap::new(),
            last_updated: HashMap::new(),
            cache_ttl_hours: 24, // Cache models for 24 hours
        }));
        let model_configs = Arc::new(RwLock::new(HashMap::new()));
        let performance_metrics = Arc::new(RwLock::new(HashMap::new()));

        let manager = Self {
            openrouter_integration,
            model_cache,
            model_configs,
            performance_metrics,
        };

        // Initialize default model configurations
        manager.initialize_default_configurations().await?;

        info!("Enhanced model manager initialized successfully");
        Ok(manager)
    }

    /// Initialize default model configurations for latest models
    async fn initialize_default_configurations(&self) -> AppResult<()> {
        let mut configs = self.model_configs.write().await;

        // Claude 3.5 Sonnet
        configs.insert("anthropic/claude-3.5-sonnet".to_string(), ModelConfiguration {
            model_id: "anthropic/claude-3.5-sonnet".to_string(),
            display_name: "Claude 3.5 Sonnet".to_string(),
            provider: ServiceProvider::OpenRouter,
            default_temperature: 0.7,
            default_max_tokens: 4096,
            default_top_p: 0.9,
            supports_function_calling: true,
            supports_streaming: true,
            context_window: 200000,
            cost_per_1k_tokens: 0.003,
            recommended_use_cases: vec![
                "Research Analysis".to_string(),
                "Complex Reasoning".to_string(),
                "Code Generation".to_string(),
                "Creative Writing".to_string(),
            ],
            performance_tier: ModelTier::Premium,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        // GPT-4 Turbo
        configs.insert("openai/gpt-4-turbo".to_string(), ModelConfiguration {
            model_id: "openai/gpt-4-turbo".to_string(),
            display_name: "GPT-4 Turbo".to_string(),
            provider: ServiceProvider::OpenRouter,
            default_temperature: 0.7,
            default_max_tokens: 4096,
            default_top_p: 0.9,
            supports_function_calling: true,
            supports_streaming: true,
            context_window: 128000,
            cost_per_1k_tokens: 0.01,
            recommended_use_cases: vec![
                "General Research".to_string(),
                "Data Analysis".to_string(),
                "Problem Solving".to_string(),
            ],
            performance_tier: ModelTier::Premium,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        // Add more latest models...
        // GPT-4o, Gemini 1.5 Pro, Llama 3.1 405B, etc.

        info!("Initialized {} default model configurations", configs.len());
        Ok(())
    }

    /// Get all available models with caching
    pub async fn get_available_models(&self, api_key: &ApiKey) -> AppResult<Vec<OpenRouterModel>> {
        let cache = self.model_cache.read().await;
        let provider = ServiceProvider::OpenRouter;

        // Check if cache is valid
        if let Some(last_updated) = cache.last_updated.get(&provider) {
            let cache_age = Utc::now().signed_duration_since(*last_updated);
            if cache_age < Duration::hours(cache.cache_ttl_hours as i64) {
                if let Some(models) = cache.models.get(&provider) {
                    debug!("Returning cached models for provider: {:?}", provider);
                    return Ok(models.clone());
                }
            }
        }

        drop(cache);

        // Cache is invalid or missing, fetch fresh data
        info!("Fetching fresh model data from OpenRouter");
        let models = self.openrouter_integration.get_models_detailed(api_key).await?;

        // Update cache
        {
            let mut cache = self.model_cache.write().await;
            cache.models.insert(provider, models.clone());
            cache.last_updated.insert(provider, Utc::now());
        }

        Ok(models)
    }

    /// Get latest high-performance models
    pub async fn get_latest_models(&self, api_key: &ApiKey) -> AppResult<Vec<OpenRouterModel>> {
        self.openrouter_integration.get_latest_models(api_key).await
    }

    /// Get model configuration
    pub async fn get_model_configuration(&self, model_id: &str) -> AppResult<Option<ModelConfiguration>> {
        let configs = self.model_configs.read().await;
        Ok(configs.get(model_id).cloned())
    }

    /// Update model configuration
    pub async fn update_model_configuration(&self, config: ModelConfiguration) -> AppResult<()> {
        let mut configs = self.model_configs.write().await;
        configs.insert(config.model_id.clone(), config);
        Ok(())
    }

    /// Record model usage for performance tracking
    pub async fn record_model_usage(
        &self,
        model_id: &str,
        response_time_ms: u64,
        tokens_used: u32,
        success: bool,
        cost: f64,
    ) -> AppResult<()> {
        let mut metrics = self.performance_metrics.write().await;
        
        let metric = metrics.entry(model_id.to_string()).or_insert_with(|| {
            ModelPerformanceMetrics {
                model_id: model_id.to_string(),
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_response_time_ms: 0.0,
                average_tokens_per_request: 0.0,
                total_cost: 0.0,
                quality_score: None,
                last_used: Utc::now(),
                usage_trend: Vec::new(),
            }
        });

        // Update metrics
        metric.total_requests += 1;
        if success {
            metric.successful_requests += 1;
        } else {
            metric.failed_requests += 1;
        }

        // Update averages
        let total_response_time = metric.average_response_time_ms * (metric.total_requests - 1) as f64 + response_time_ms as f64;
        metric.average_response_time_ms = total_response_time / metric.total_requests as f64;

        let total_tokens = metric.average_tokens_per_request * (metric.total_requests - 1) as f64 + tokens_used as f64;
        metric.average_tokens_per_request = total_tokens / metric.total_requests as f64;

        metric.total_cost += cost;
        metric.last_used = Utc::now();

        // Add to usage trend (keep last 100 data points)
        metric.usage_trend.push(UsageDataPoint {
            timestamp: Utc::now(),
            requests_count: 1,
            average_response_time: response_time_ms as f64,
            success_rate: if success { 1.0 } else { 0.0 },
        });

        if metric.usage_trend.len() > 100 {
            metric.usage_trend.remove(0);
        }

        Ok(())
    }

    /// Get model performance metrics
    pub async fn get_model_performance(&self, model_id: &str) -> AppResult<Option<ModelPerformanceMetrics>> {
        let metrics = self.performance_metrics.read().await;
        Ok(metrics.get(model_id).cloned())
    }

    /// Get model recommendations for a specific use case
    pub async fn get_model_recommendations(&self, use_case: &str, budget_limit: Option<f64>) -> AppResult<Vec<ModelRecommendation>> {
        let configs = self.model_configs.read().await;
        let metrics = self.performance_metrics.read().await;

        let mut recommendations = Vec::new();

        for (model_id, config) in configs.iter() {
            // Check if model is suitable for use case
            let use_case_match = config.recommended_use_cases.iter()
                .any(|case| case.to_lowercase().contains(&use_case.to_lowercase()));

            if !use_case_match {
                continue;
            }

            // Check budget constraint
            if let Some(budget) = budget_limit {
                if config.cost_per_1k_tokens > budget {
                    continue;
                }
            }

            // Calculate confidence score based on performance metrics
            let confidence_score = if let Some(metric) = metrics.get(model_id) {
                let success_rate = metric.successful_requests as f64 / metric.total_requests as f64;
                let performance_factor = 1.0 / (1.0 + metric.average_response_time_ms / 1000.0);
                (success_rate * 0.6 + performance_factor * 0.4).min(1.0)
            } else {
                0.5 // Default confidence for untested models
            };

            recommendations.push(ModelRecommendation {
                model_id: model_id.clone(),
                confidence_score,
                reasoning: format!("Suitable for {} with {} performance tier", use_case, 
                    match config.performance_tier {
                        ModelTier::Premium => "premium",
                        ModelTier::Standard => "standard",
                        ModelTier::Efficient => "efficient",
                        ModelTier::Specialized => "specialized",
                    }
                ),
                estimated_cost: config.cost_per_1k_tokens,
                estimated_performance: ModelPerformanceEstimate {
                    expected_response_time_ms: metrics.get(model_id)
                        .map(|m| m.average_response_time_ms)
                        .unwrap_or(2000.0),
                    expected_quality_score: metrics.get(model_id)
                        .and_then(|m| m.quality_score)
                        .unwrap_or(0.8),
                    expected_success_rate: metrics.get(model_id)
                        .map(|m| m.successful_requests as f64 / m.total_requests as f64)
                        .unwrap_or(0.95),
                },
            });
        }

        // Sort by confidence score
        recommendations.sort_by(|a, b| b.confidence_score.partial_cmp(&a.confidence_score).unwrap());

        Ok(recommendations)
    }

    /// Clear model cache
    pub async fn clear_cache(&self) -> AppResult<()> {
        let mut cache = self.model_cache.write().await;
        cache.models.clear();
        cache.last_updated.clear();
        info!("Model cache cleared");
        Ok(())
    }

    /// Get all model configurations
    pub async fn get_all_configurations(&self) -> AppResult<Vec<ModelConfiguration>> {
        let configs = self.model_configs.read().await;
        Ok(configs.values().cloned().collect())
    }

    /// Get performance summary for all models
    pub async fn get_performance_summary(&self) -> AppResult<HashMap<String, ModelPerformanceMetrics>> {
        let metrics = self.performance_metrics.read().await;
        Ok(metrics.clone())
    }
}
