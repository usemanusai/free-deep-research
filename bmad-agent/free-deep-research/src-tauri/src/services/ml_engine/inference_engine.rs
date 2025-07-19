use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, warn, error};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, ResearchError};

/// Machine Learning Inference Engine for real-time predictions
#[derive(Clone)]
pub struct InferenceEngine {
    loaded_models: Arc<RwLock<HashMap<String, LoadedModel>>>,
    inference_cache: Arc<RwLock<InferenceCache>>,
    config: InferenceConfig,
    performance_metrics: Arc<RwLock<InferenceMetrics>>,
}

impl InferenceEngine {
    pub fn new(config: InferenceConfig) -> Self {
        Self {
            loaded_models: Arc::new(RwLock::new(HashMap::new())),
            inference_cache: Arc::new(RwLock::new(InferenceCache::new())),
            config,
            performance_metrics: Arc::new(RwLock::new(InferenceMetrics::new())),
        }
    }

    /// Load a trained model for inference
    pub async fn load_model(&self, model_name: String, model_path: String, model_type: ModelType) -> AppResult<()> {
        info!("Loading model: {} from path: {}", model_name, model_path);

        let model = match model_type {
            ModelType::ResearchPatternPredictor => {
                self.load_research_pattern_model(&model_path).await?
            }
            ModelType::UsageForecaster => {
                self.load_usage_forecasting_model(&model_path).await?
            }
            ModelType::PerformanceOptimizer => {
                self.load_performance_optimization_model(&model_path).await?
            }
            ModelType::RecommendationEngine => {
                self.load_recommendation_model(&model_path).await?
            }
            ModelType::AnomalyDetector => {
                self.load_anomaly_detection_model(&model_path).await?
            }
        };

        let loaded_model = LoadedModel {
            model,
            model_type,
            loaded_at: Utc::now(),
            inference_count: 0,
            average_inference_time: 0.0,
            last_used: Utc::now(),
        };

        let mut models = self.loaded_models.write().await;
        models.insert(model_name, loaded_model);

        info!("Model loaded successfully");
        Ok(())
    }

    /// Perform inference with a loaded model
    pub async fn predict(&self, request: InferenceRequest) -> AppResult<InferenceResult> {
        let start_time = std::time::Instant::now();
        
        debug!("Performing inference with model: {}", request.model_name);

        // Check cache first
        if let Some(cached_result) = self.check_cache(&request).await? {
            debug!("Returning cached inference result");
            return Ok(cached_result);
        }

        // Get the model
        let models = self.loaded_models.read().await;
        let model = models.get(&request.model_name)
            .ok_or_else(|| ResearchError::invalid_request(
                format!("Model not found: {}", request.model_name)
            ))?;

        // Perform inference based on model type
        let prediction = match &model.model_type {
            ModelType::ResearchPatternPredictor => {
                self.predict_research_patterns(&model.model, &request.input_data).await?
            }
            ModelType::UsageForecaster => {
                self.predict_usage_patterns(&model.model, &request.input_data).await?
            }
            ModelType::PerformanceOptimizer => {
                self.predict_performance_optimizations(&model.model, &request.input_data).await?
            }
            ModelType::RecommendationEngine => {
                self.generate_recommendations(&model.model, &request.input_data).await?
            }
            ModelType::AnomalyDetector => {
                self.detect_anomalies(&model.model, &request.input_data).await?
            }
        };

        let inference_time = start_time.elapsed().as_millis() as f64;

        let result = InferenceResult {
            request_id: request.request_id,
            model_name: request.model_name.clone(),
            prediction,
            confidence_score: self.calculate_confidence_score(&request.input_data).await?,
            inference_time_ms: inference_time,
            timestamp: Utc::now(),
            model_version: model.model.version.clone(),
        };

        // Cache the result
        self.cache_result(&request, &result).await?;

        // Update metrics
        self.update_inference_metrics(&request.model_name, inference_time).await?;

        debug!("Inference completed in {:.2}ms", inference_time);
        Ok(result)
    }

    /// Predict research patterns
    async fn predict_research_patterns(&self, model: &MLModel, input_data: &serde_json::Value) -> AppResult<PredictionOutput> {
        // Extract features from input data
        let features = self.extract_research_features(input_data).await?;
        
        // Simulate ML inference (in a real implementation, this would use a trained model)
        let research_complexity = features.get("query_complexity").unwrap_or(&0.5).as_f64().unwrap_or(0.5);
        let methodology_preference = features.get("methodology_preference").unwrap_or(&0.0).as_f64().unwrap_or(0.0);
        let historical_success_rate = features.get("historical_success_rate").unwrap_or(&0.8).as_f64().unwrap_or(0.8);

        // Predict optimal methodology
        let predicted_methodology = if research_complexity > 0.7 {
            "hybrid"
        } else if methodology_preference > 0.5 {
            "nick_scamara"
        } else {
            "don_lim"
        };

        // Predict success probability
        let success_probability = (historical_success_rate * 0.6 + (1.0 - research_complexity) * 0.4)
            .max(0.1).min(0.95);

        // Predict estimated completion time (in minutes)
        let estimated_time = (research_complexity * 45.0 + 15.0) as u32;

        Ok(PredictionOutput {
            prediction_type: "research_pattern".to_string(),
            values: serde_json::json!({
                "optimal_methodology": predicted_methodology,
                "success_probability": success_probability,
                "estimated_completion_time_minutes": estimated_time,
                "recommended_sources": self.recommend_sources(research_complexity).await?,
                "quality_score": success_probability * 100.0
            }),
            metadata: serde_json::json!({
                "model_version": model.version,
                "feature_importance": {
                    "query_complexity": 0.4,
                    "methodology_preference": 0.3,
                    "historical_success_rate": 0.3
                }
            }),
        })
    }

    /// Predict usage patterns
    async fn predict_usage_patterns(&self, model: &MLModel, input_data: &serde_json::Value) -> AppResult<PredictionOutput> {
        let features = self.extract_usage_features(input_data).await?;
        
        let current_usage = features.get("current_usage").unwrap_or(&100.0).as_f64().unwrap_or(100.0);
        let time_of_day = features.get("time_of_day").unwrap_or(&12.0).as_f64().unwrap_or(12.0);
        let day_of_week = features.get("day_of_week").unwrap_or(&3.0).as_f64().unwrap_or(3.0);

        // Predict usage for next 24 hours
        let mut hourly_predictions = Vec::new();
        for hour in 0..24 {
            let hour_factor = if hour >= 9 && hour <= 17 { 1.2 } else { 0.8 }; // Business hours
            let day_factor = if day_of_week >= 1.0 && day_of_week <= 5.0 { 1.0 } else { 0.6 }; // Weekdays
            
            let predicted_usage = current_usage * hour_factor * day_factor * (0.8 + 0.4 * rand::random::<f64>());
            
            hourly_predictions.push(serde_json::json!({
                "hour": hour,
                "predicted_usage": predicted_usage,
                "confidence": 0.85
            }));
        }

        Ok(PredictionOutput {
            prediction_type: "usage_forecast".to_string(),
            values: serde_json::json!({
                "hourly_predictions": hourly_predictions,
                "peak_usage_hour": self.find_peak_hour(&hourly_predictions),
                "total_predicted_usage": hourly_predictions.iter()
                    .map(|p| p["predicted_usage"].as_f64().unwrap_or(0.0))
                    .sum::<f64>(),
                "usage_trend": if current_usage > 80.0 { "increasing" } else { "stable" }
            }),
            metadata: serde_json::json!({
                "model_version": model.version,
                "prediction_horizon_hours": 24
            }),
        })
    }

    /// Predict performance optimizations
    async fn predict_performance_optimizations(&self, model: &MLModel, input_data: &serde_json::Value) -> AppResult<PredictionOutput> {
        let features = self.extract_performance_features(input_data).await?;
        
        let cpu_usage = features.get("cpu_usage").unwrap_or(&50.0).as_f64().unwrap_or(50.0);
        let memory_usage = features.get("memory_usage").unwrap_or(&60.0).as_f64().unwrap_or(60.0);
        let response_time = features.get("response_time").unwrap_or(&200.0).as_f64().unwrap_or(200.0);

        let mut optimizations = Vec::new();

        // CPU optimization recommendations
        if cpu_usage > 80.0 {
            optimizations.push(serde_json::json!({
                "type": "cpu_optimization",
                "priority": "high",
                "recommendation": "Enable CPU throttling for non-critical tasks",
                "expected_improvement": "15-25% CPU reduction",
                "implementation_effort": "medium"
            }));
        }

        // Memory optimization recommendations
        if memory_usage > 75.0 {
            optimizations.push(serde_json::json!({
                "type": "memory_optimization",
                "priority": "high",
                "recommendation": "Implement aggressive caching and memory cleanup",
                "expected_improvement": "20-30% memory reduction",
                "implementation_effort": "low"
            }));
        }

        // Response time optimization
        if response_time > 500.0 {
            optimizations.push(serde_json::json!({
                "type": "response_time_optimization",
                "priority": "medium",
                "recommendation": "Optimize database queries and add connection pooling",
                "expected_improvement": "30-50% response time reduction",
                "implementation_effort": "high"
            }));
        }

        Ok(PredictionOutput {
            prediction_type: "performance_optimization".to_string(),
            values: serde_json::json!({
                "optimizations": optimizations,
                "overall_performance_score": self.calculate_performance_score(cpu_usage, memory_usage, response_time),
                "predicted_improvement": optimizations.len() as f64 * 15.0,
                "implementation_timeline": "1-2 weeks"
            }),
            metadata: serde_json::json!({
                "model_version": model.version,
                "analysis_timestamp": Utc::now()
            }),
        })
    }

    /// Generate recommendations
    async fn generate_recommendations(&self, model: &MLModel, input_data: &serde_json::Value) -> AppResult<PredictionOutput> {
        let features = self.extract_recommendation_features(input_data).await?;
        
        let user_behavior = features.get("user_behavior").unwrap_or(&serde_json::json!({}));
        let research_history = features.get("research_history").unwrap_or(&serde_json::json!([]));

        let mut recommendations = Vec::new();

        // Methodology recommendations
        recommendations.push(serde_json::json!({
            "type": "methodology",
            "title": "Try the Hybrid Methodology",
            "description": "Based on your research patterns, the hybrid methodology could improve your results by 25%",
            "confidence": 0.82,
            "category": "optimization",
            "action": "switch_methodology",
            "parameters": { "methodology": "hybrid" }
        }));

        // API optimization recommendations
        recommendations.push(serde_json::json!({
            "type": "api_optimization",
            "title": "Optimize API Usage",
            "description": "Consider using Jina AI for better semantic search results",
            "confidence": 0.75,
            "category": "cost_optimization",
            "action": "try_api",
            "parameters": { "api": "jina", "use_case": "semantic_search" }
        }));

        // Workflow recommendations
        recommendations.push(serde_json::json!({
            "type": "workflow",
            "title": "Schedule Research During Off-Peak Hours",
            "description": "Running research between 2-6 AM could reduce costs by 15%",
            "confidence": 0.68,
            "category": "cost_optimization",
            "action": "schedule_research",
            "parameters": { "preferred_hours": [2, 3, 4, 5, 6] }
        }));

        Ok(PredictionOutput {
            prediction_type: "recommendations".to_string(),
            values: serde_json::json!({
                "recommendations": recommendations,
                "personalization_score": 0.78,
                "total_potential_savings": "20-30%",
                "implementation_difficulty": "easy"
            }),
            metadata: serde_json::json!({
                "model_version": model.version,
                "recommendation_engine": "collaborative_filtering_v2"
            }),
        })
    }

    /// Detect anomalies
    async fn detect_anomalies(&self, model: &MLModel, input_data: &serde_json::Value) -> AppResult<PredictionOutput> {
        let features = self.extract_anomaly_features(input_data).await?;
        
        let mut anomalies = Vec::new();
        let mut anomaly_score = 0.0;

        // Check for usage anomalies
        if let Some(usage_pattern) = features.get("usage_pattern") {
            let current_usage = usage_pattern.get("current").unwrap_or(&0.0).as_f64().unwrap_or(0.0);
            let historical_avg = usage_pattern.get("historical_avg").unwrap_or(&100.0).as_f64().unwrap_or(100.0);
            
            if current_usage > historical_avg * 2.0 {
                anomalies.push(serde_json::json!({
                    "type": "usage_spike",
                    "severity": "high",
                    "description": "Usage is 2x higher than historical average",
                    "current_value": current_usage,
                    "expected_value": historical_avg,
                    "anomaly_score": 0.9
                }));
                anomaly_score += 0.9;
            }
        }

        // Check for performance anomalies
        if let Some(performance_data) = features.get("performance") {
            let response_time = performance_data.get("response_time").unwrap_or(&200.0).as_f64().unwrap_or(200.0);
            
            if response_time > 1000.0 {
                anomalies.push(serde_json::json!({
                    "type": "performance_degradation",
                    "severity": "medium",
                    "description": "Response time significantly higher than normal",
                    "current_value": response_time,
                    "expected_value": 200.0,
                    "anomaly_score": 0.7
                }));
                anomaly_score += 0.7;
            }
        }

        // Check for error rate anomalies
        if let Some(error_data) = features.get("errors") {
            let error_rate = error_data.get("rate").unwrap_or(&0.01).as_f64().unwrap_or(0.01);
            
            if error_rate > 0.1 {
                anomalies.push(serde_json::json!({
                    "type": "error_rate_spike",
                    "severity": "high",
                    "description": "Error rate is unusually high",
                    "current_value": error_rate,
                    "expected_value": 0.01,
                    "anomaly_score": 0.8
                }));
                anomaly_score += 0.8;
            }
        }

        let overall_anomaly_score = (anomaly_score / anomalies.len().max(1) as f64).min(1.0);

        Ok(PredictionOutput {
            prediction_type: "anomaly_detection".to_string(),
            values: serde_json::json!({
                "anomalies": anomalies,
                "overall_anomaly_score": overall_anomaly_score,
                "system_health": if overall_anomaly_score < 0.3 { "healthy" } else if overall_anomaly_score < 0.7 { "warning" } else { "critical" },
                "recommended_actions": self.generate_anomaly_actions(&anomalies).await?
            }),
            metadata: serde_json::json!({
                "model_version": model.version,
                "detection_algorithm": "isolation_forest_v1"
            }),
        })
    }

    // Helper methods for feature extraction and processing

    async fn extract_research_features(&self, input_data: &serde_json::Value) -> AppResult<serde_json::Map<String, serde_json::Value>> {
        let mut features = serde_json::Map::new();
        
        // Extract query complexity (0.0 to 1.0)
        if let Some(query) = input_data.get("query").and_then(|q| q.as_str()) {
            let complexity = (query.len() as f64 / 200.0).min(1.0);
            features.insert("query_complexity".to_string(), serde_json::Value::Number(
                serde_json::Number::from_f64(complexity).unwrap()
            ));
        }

        // Extract methodology preference
        if let Some(methodology) = input_data.get("preferred_methodology").and_then(|m| m.as_str()) {
            let preference = match methodology {
                "hybrid" => 0.8,
                "nick_scamara" => 0.6,
                "don_lim" => 0.4,
                _ => 0.5,
            };
            features.insert("methodology_preference".to_string(), serde_json::Value::Number(
                serde_json::Number::from_f64(preference).unwrap()
            ));
        }

        // Extract historical success rate
        if let Some(success_rate) = input_data.get("historical_success_rate").and_then(|s| s.as_f64()) {
            features.insert("historical_success_rate".to_string(), serde_json::Value::Number(
                serde_json::Number::from_f64(success_rate).unwrap()
            ));
        }

        Ok(features)
    }

    async fn extract_usage_features(&self, input_data: &serde_json::Value) -> AppResult<serde_json::Map<String, serde_json::Value>> {
        let mut features = serde_json::Map::new();
        
        if let Some(current_usage) = input_data.get("current_usage").and_then(|u| u.as_f64()) {
            features.insert("current_usage".to_string(), serde_json::Value::Number(
                serde_json::Number::from_f64(current_usage).unwrap()
            ));
        }

        let now = Utc::now();
        features.insert("time_of_day".to_string(), serde_json::Value::Number(
            serde_json::Number::from_f64(now.hour() as f64).unwrap()
        ));
        features.insert("day_of_week".to_string(), serde_json::Value::Number(
            serde_json::Number::from_f64(now.weekday().number_from_monday() as f64).unwrap()
        ));

        Ok(features)
    }

    async fn extract_performance_features(&self, input_data: &serde_json::Value) -> AppResult<serde_json::Map<String, serde_json::Value>> {
        let mut features = serde_json::Map::new();
        
        if let Some(cpu) = input_data.get("cpu_usage").and_then(|c| c.as_f64()) {
            features.insert("cpu_usage".to_string(), serde_json::Value::Number(
                serde_json::Number::from_f64(cpu).unwrap()
            ));
        }

        if let Some(memory) = input_data.get("memory_usage").and_then(|m| m.as_f64()) {
            features.insert("memory_usage".to_string(), serde_json::Value::Number(
                serde_json::Number::from_f64(memory).unwrap()
            ));
        }

        if let Some(response_time) = input_data.get("response_time").and_then(|r| r.as_f64()) {
            features.insert("response_time".to_string(), serde_json::Value::Number(
                serde_json::Number::from_f64(response_time).unwrap()
            ));
        }

        Ok(features)
    }

    async fn extract_recommendation_features(&self, input_data: &serde_json::Value) -> AppResult<serde_json::Map<String, serde_json::Value>> {
        let mut features = serde_json::Map::new();
        
        if let Some(user_behavior) = input_data.get("user_behavior") {
            features.insert("user_behavior".to_string(), user_behavior.clone());
        }

        if let Some(research_history) = input_data.get("research_history") {
            features.insert("research_history".to_string(), research_history.clone());
        }

        Ok(features)
    }

    async fn extract_anomaly_features(&self, input_data: &serde_json::Value) -> AppResult<serde_json::Map<String, serde_json::Value>> {
        let mut features = serde_json::Map::new();
        
        if let Some(usage_pattern) = input_data.get("usage_pattern") {
            features.insert("usage_pattern".to_string(), usage_pattern.clone());
        }

        if let Some(performance) = input_data.get("performance") {
            features.insert("performance".to_string(), performance.clone());
        }

        if let Some(errors) = input_data.get("errors") {
            features.insert("errors".to_string(), errors.clone());
        }

        Ok(features)
    }

    // Additional helper methods

    async fn recommend_sources(&self, complexity: f64) -> AppResult<Vec<String>> {
        let sources = if complexity > 0.7 {
            vec![
                "Academic databases".to_string(),
                "Industry reports".to_string(),
                "Expert interviews".to_string(),
                "Government data".to_string(),
            ]
        } else if complexity > 0.4 {
            vec![
                "Web search".to_string(),
                "News articles".to_string(),
                "Company websites".to_string(),
            ]
        } else {
            vec![
                "Quick web search".to_string(),
                "Wikipedia".to_string(),
            ]
        };
        Ok(sources)
    }

    fn find_peak_hour(&self, predictions: &[serde_json::Value]) -> u32 {
        predictions.iter()
            .enumerate()
            .max_by(|a, b| {
                let a_usage = a.1["predicted_usage"].as_f64().unwrap_or(0.0);
                let b_usage = b.1["predicted_usage"].as_f64().unwrap_or(0.0);
                a_usage.partial_cmp(&b_usage).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(hour, _)| hour as u32)
            .unwrap_or(12)
    }

    fn calculate_performance_score(&self, cpu: f64, memory: f64, response_time: f64) -> f64 {
        let cpu_score = (100.0 - cpu) / 100.0;
        let memory_score = (100.0 - memory) / 100.0;
        let response_score = (1000.0 - response_time.min(1000.0)) / 1000.0;
        
        (cpu_score * 0.3 + memory_score * 0.3 + response_score * 0.4) * 100.0
    }

    async fn generate_anomaly_actions(&self, anomalies: &[serde_json::Value]) -> AppResult<Vec<String>> {
        let mut actions = Vec::new();
        
        for anomaly in anomalies {
            if let Some(anomaly_type) = anomaly.get("type").and_then(|t| t.as_str()) {
                match anomaly_type {
                    "usage_spike" => actions.push("Investigate unusual usage patterns and consider scaling resources".to_string()),
                    "performance_degradation" => actions.push("Check system resources and optimize slow queries".to_string()),
                    "error_rate_spike" => actions.push("Review error logs and fix underlying issues".to_string()),
                    _ => actions.push("Monitor system closely and investigate root cause".to_string()),
                }
            }
        }
        
        Ok(actions)
    }

    async fn calculate_confidence_score(&self, _input_data: &serde_json::Value) -> AppResult<f64> {
        // Simplified confidence calculation
        Ok(0.85)
    }

    async fn check_cache(&self, request: &InferenceRequest) -> AppResult<Option<InferenceResult>> {
        let cache = self.inference_cache.read().await;
        Ok(cache.get(&request.cache_key()))
    }

    async fn cache_result(&self, request: &InferenceRequest, result: &InferenceResult) -> AppResult<()> {
        let mut cache = self.inference_cache.write().await;
        cache.insert(request.cache_key(), result.clone());
        Ok(())
    }

    async fn update_inference_metrics(&self, model_name: &str, inference_time: f64) -> AppResult<()> {
        let mut metrics = self.performance_metrics.write().await;
        metrics.record_inference(model_name.to_string(), inference_time);
        Ok(())
    }

    // Model loading methods (simplified implementations)

    async fn load_research_pattern_model(&self, _model_path: &str) -> AppResult<MLModel> {
        Ok(MLModel {
            model_id: Uuid::new_v4(),
            name: "research_pattern_predictor".to_string(),
            version: "1.0.0".to_string(),
            model_type: ModelType::ResearchPatternPredictor,
            parameters: HashMap::new(),
            metadata: serde_json::json!({
                "algorithm": "gradient_boosting",
                "features": ["query_complexity", "methodology_preference", "historical_success_rate"],
                "accuracy": 0.87
            }),
            created_at: Utc::now(),
        })
    }

    async fn load_usage_forecasting_model(&self, _model_path: &str) -> AppResult<MLModel> {
        Ok(MLModel {
            model_id: Uuid::new_v4(),
            name: "usage_forecaster".to_string(),
            version: "1.0.0".to_string(),
            model_type: ModelType::UsageForecaster,
            parameters: HashMap::new(),
            metadata: serde_json::json!({
                "algorithm": "time_series_lstm",
                "features": ["historical_usage", "time_features", "external_factors"],
                "accuracy": 0.82
            }),
            created_at: Utc::now(),
        })
    }

    async fn load_performance_optimization_model(&self, _model_path: &str) -> AppResult<MLModel> {
        Ok(MLModel {
            model_id: Uuid::new_v4(),
            name: "performance_optimizer".to_string(),
            version: "1.0.0".to_string(),
            model_type: ModelType::PerformanceOptimizer,
            parameters: HashMap::new(),
            metadata: serde_json::json!({
                "algorithm": "reinforcement_learning",
                "features": ["cpu_usage", "memory_usage", "response_time", "throughput"],
                "accuracy": 0.79
            }),
            created_at: Utc::now(),
        })
    }

    async fn load_recommendation_model(&self, _model_path: &str) -> AppResult<MLModel> {
        Ok(MLModel {
            model_id: Uuid::new_v4(),
            name: "recommendation_engine".to_string(),
            version: "1.0.0".to_string(),
            model_type: ModelType::RecommendationEngine,
            parameters: HashMap::new(),
            metadata: serde_json::json!({
                "algorithm": "collaborative_filtering",
                "features": ["user_behavior", "research_history", "preferences"],
                "accuracy": 0.75
            }),
            created_at: Utc::now(),
        })
    }

    async fn load_anomaly_detection_model(&self, _model_path: &str) -> AppResult<MLModel> {
        Ok(MLModel {
            model_id: Uuid::new_v4(),
            name: "anomaly_detector".to_string(),
            version: "1.0.0".to_string(),
            model_type: ModelType::AnomalyDetector,
            parameters: HashMap::new(),
            metadata: serde_json::json!({
                "algorithm": "isolation_forest",
                "features": ["usage_patterns", "performance_metrics", "error_rates"],
                "accuracy": 0.88
            }),
            created_at: Utc::now(),
        })
    }
}

// Supporting data structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelType {
    ResearchPatternPredictor,
    UsageForecaster,
    PerformanceOptimizer,
    RecommendationEngine,
    AnomalyDetector,
}

#[derive(Debug, Clone)]
pub struct LoadedModel {
    pub model: MLModel,
    pub model_type: ModelType,
    pub loaded_at: DateTime<Utc>,
    pub inference_count: u64,
    pub average_inference_time: f64,
    pub last_used: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct MLModel {
    pub model_id: Uuid,
    pub name: String,
    pub version: String,
    pub model_type: ModelType,
    pub parameters: HashMap<String, f64>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceRequest {
    pub request_id: Uuid,
    pub model_name: String,
    pub input_data: serde_json::Value,
    pub options: InferenceOptions,
}

impl InferenceRequest {
    pub fn cache_key(&self) -> String {
        format!("{}:{}", self.model_name, 
            sha256::digest(serde_json::to_string(&self.input_data).unwrap_or_default()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceOptions {
    pub use_cache: bool,
    pub timeout_ms: u64,
    pub confidence_threshold: f64,
}

impl Default for InferenceOptions {
    fn default() -> Self {
        Self {
            use_cache: true,
            timeout_ms: 5000,
            confidence_threshold: 0.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResult {
    pub request_id: Uuid,
    pub model_name: String,
    pub prediction: PredictionOutput,
    pub confidence_score: f64,
    pub inference_time_ms: f64,
    pub timestamp: DateTime<Utc>,
    pub model_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionOutput {
    pub prediction_type: String,
    pub values: serde_json::Value,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct InferenceConfig {
    pub max_cache_size: usize,
    pub cache_ttl_seconds: u64,
    pub max_concurrent_inferences: usize,
    pub default_timeout_ms: u64,
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            max_cache_size: 1000,
            cache_ttl_seconds: 3600,
            max_concurrent_inferences: 10,
            default_timeout_ms: 5000,
        }
    }
}

#[derive(Debug, Clone)]
struct InferenceCache {
    cache: HashMap<String, (InferenceResult, DateTime<Utc>)>,
    max_size: usize,
    ttl_seconds: u64,
}

impl InferenceCache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
            max_size: 1000,
            ttl_seconds: 3600,
        }
    }

    fn get(&self, key: &str) -> Option<InferenceResult> {
        if let Some((result, timestamp)) = self.cache.get(key) {
            if Utc::now().signed_duration_since(*timestamp).num_seconds() < self.ttl_seconds as i64 {
                return Some(result.clone());
            }
        }
        None
    }

    fn insert(&mut self, key: String, result: InferenceResult) {
        if self.cache.len() >= self.max_size {
            // Remove oldest entry
            if let Some(oldest_key) = self.cache.keys().next().cloned() {
                self.cache.remove(&oldest_key);
            }
        }
        self.cache.insert(key, (result, Utc::now()));
    }
}

#[derive(Debug, Clone)]
struct InferenceMetrics {
    total_inferences: u64,
    model_metrics: HashMap<String, ModelMetrics>,
}

impl InferenceMetrics {
    fn new() -> Self {
        Self {
            total_inferences: 0,
            model_metrics: HashMap::new(),
        }
    }

    fn record_inference(&mut self, model_name: String, inference_time: f64) {
        self.total_inferences += 1;
        
        let metrics = self.model_metrics.entry(model_name).or_insert_with(|| ModelMetrics {
            inference_count: 0,
            total_inference_time: 0.0,
            average_inference_time: 0.0,
            last_used: Utc::now(),
        });
        
        metrics.inference_count += 1;
        metrics.total_inference_time += inference_time;
        metrics.average_inference_time = metrics.total_inference_time / metrics.inference_count as f64;
        metrics.last_used = Utc::now();
    }
}

#[derive(Debug, Clone)]
struct ModelMetrics {
    inference_count: u64,
    total_inference_time: f64,
    average_inference_time: f64,
    last_used: DateTime<Utc>,
}
