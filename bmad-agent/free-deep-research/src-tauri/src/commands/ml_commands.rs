use tauri::State;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use tracing::{info, debug, error};

use crate::error::AppResult;
use crate::services::ml_engine::{
    inference_engine::{InferenceEngine, InferenceRequest, InferenceOptions, ModelType},
    model_training::{ModelTrainer, TrainingConfig, TrainingStatus},
    pattern_analysis::{PatternAnalyzer, AnalysisInput, PatternType},
    recommendation_system::{RecommendationEngine, RecommendationContext}
};

/// Start ML model training
#[tauri::command]
pub async fn start_model_training(
    model_name: String,
    model_type: String,
    training_config: TrainingConfigRequest,
    user_id: String,
    trainer: State<'_, ModelTrainer>
) -> AppResult<TrainingJobResponse> {
    info!("Starting model training: {} ({})", model_name, model_type);

    let job_id = Uuid::new_v4();
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid user ID".to_string()))?;

    let model_type_enum = match model_type.as_str() {
        "research_pattern_predictor" => ModelType::ResearchPatternPredictor,
        "usage_forecaster" => ModelType::UsageForecaster,
        "performance_optimizer" => ModelType::PerformanceOptimizer,
        "recommendation_engine" => ModelType::RecommendationEngine,
        "anomaly_detector" => ModelType::AnomalyDetector,
        _ => return Err(crate::error::ResearchError::invalid_request(
            format!("Unsupported model type: {}", model_type)
        ).into()),
    };

    let config = TrainingConfig {
        max_epochs: training_config.max_epochs,
        learning_rate: training_config.learning_rate,
        batch_size: training_config.batch_size,
        validation_split: training_config.validation_split,
        early_stopping: training_config.early_stopping,
        patience: training_config.patience,
    };

    let training_job = trainer.start_training(
        job_id,
        model_name,
        model_type_enum,
        config,
        user_uuid
    ).await?;

    Ok(TrainingJobResponse {
        job_id: training_job.job_id.to_string(),
        model_name: training_job.model_name,
        status: format!("{:?}", training_job.status),
        progress: training_job.progress,
        started_at: training_job.started_at.to_rfc3339(),
        estimated_completion: training_job.estimated_completion.to_rfc3339(),
    })
}

/// Get training job status
#[tauri::command]
pub async fn get_training_job_status(
    job_id: String,
    trainer: State<'_, ModelTrainer>
) -> AppResult<Option<TrainingJobResponse>> {
    debug!("Getting training job status: {}", job_id);

    let job_uuid = Uuid::parse_str(&job_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid job ID".to_string()))?;

    if let Some(job) = trainer.get_job_status(job_uuid).await? {
        Ok(Some(TrainingJobResponse {
            job_id: job.job_id.to_string(),
            model_name: job.model_name,
            status: format!("{:?}", job.status),
            progress: job.progress,
            started_at: job.started_at.to_rfc3339(),
            estimated_completion: job.estimated_completion.to_rfc3339(),
        }))
    } else {
        Ok(None)
    }
}

/// Perform ML inference
#[tauri::command]
pub async fn perform_ml_inference(
    model_name: String,
    input_data: serde_json::Value,
    options: Option<InferenceOptionsRequest>,
    inference_engine: State<'_, InferenceEngine>
) -> AppResult<InferenceResponse> {
    info!("Performing ML inference with model: {}", model_name);

    let request = InferenceRequest {
        request_id: Uuid::new_v4(),
        model_name,
        input_data,
        options: options.map(|o| InferenceOptions {
            use_cache: o.use_cache,
            timeout_ms: o.timeout_ms,
            confidence_threshold: o.confidence_threshold,
        }).unwrap_or_default(),
    };

    let result = inference_engine.predict(request).await?;

    Ok(InferenceResponse {
        request_id: result.request_id.to_string(),
        model_name: result.model_name,
        prediction: result.prediction,
        confidence_score: result.confidence_score,
        inference_time_ms: result.inference_time_ms,
        timestamp: result.timestamp.to_rfc3339(),
        model_version: result.model_version,
    })
}

/// Analyze patterns in research data
#[tauri::command]
pub async fn analyze_research_patterns(
    data_points: Vec<serde_json::Value>,
    analysis_type: String,
    pattern_analyzer: State<'_, PatternAnalyzer>
) -> AppResult<PatternAnalysisResponse> {
    info!("Analyzing research patterns: {} data points", data_points.len());

    let analysis_input = AnalysisInput {
        data_points,
        analysis_type,
        time_range: None,
    };

    let insights = pattern_analyzer.analyze_patterns(analysis_input).await?;

    let pattern_insights: Vec<PatternInsightResponse> = insights.into_iter().map(|insight| {
        PatternInsightResponse {
            pattern_id: insight.pattern_id.to_string(),
            pattern_type: format!("{:?}", insight.pattern_type),
            title: insight.title,
            description: insight.description,
            confidence: insight.confidence,
            impact_score: insight.impact_score,
            actionable_recommendations: insight.actionable_recommendations,
            supporting_data: insight.supporting_data,
            discovered_at: insight.discovered_at.to_rfc3339(),
        }
    }).collect();

    Ok(PatternAnalysisResponse {
        total_insights: pattern_insights.len(),
        insights: pattern_insights,
        analysis_completed_at: chrono::Utc::now().to_rfc3339(),
    })
}

/// Generate personalized recommendations
#[tauri::command]
pub async fn generate_recommendations(
    user_id: String,
    context: RecommendationContextRequest,
    recommendation_engine: State<'_, RecommendationEngine>
) -> AppResult<RecommendationResponse> {
    info!("Generating recommendations for user: {}", user_id);

    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| crate::error::ResearchError::invalid_request("Invalid user ID".to_string()))?;

    let rec_context = RecommendationContext {
        current_methodology: context.current_methodology,
        query_complexity: context.query_complexity,
        recent_performance: context.recent_performance,
        budget_constraints: context.budget_constraints,
        time_constraints: context.time_constraints,
    };

    let recommendations = recommendation_engine.generate_recommendations(user_uuid, rec_context).await?;

    let recommendation_responses: Vec<RecommendationItemResponse> = recommendations.into_iter().map(|rec| {
        RecommendationItemResponse {
            id: rec.id.to_string(),
            recommendation_type: format!("{:?}", rec.recommendation_type),
            title: rec.title,
            description: rec.description,
            relevance_score: rec.relevance_score,
            confidence: rec.confidence,
            impact_estimate: rec.impact_estimate,
            action_required: format!("{:?}", rec.action_required),
            parameters: rec.parameters,
            created_at: rec.created_at.to_rfc3339(),
            expires_at: rec.expires_at.map(|dt| dt.to_rfc3339()),
        }
    }).collect();

    Ok(RecommendationResponse {
        total_recommendations: recommendation_responses.len(),
        recommendations: recommendation_responses,
        generated_at: chrono::Utc::now().to_rfc3339(),
    })
}

/// Get ML engine metrics
#[tauri::command]
pub async fn get_ml_metrics(
    inference_engine: State<'_, InferenceEngine>,
    trainer: State<'_, ModelTrainer>,
    pattern_analyzer: State<'_, PatternAnalyzer>,
    recommendation_engine: State<'_, RecommendationEngine>
) -> AppResult<MLMetricsResponse> {
    debug!("Getting ML engine metrics");

    let training_metrics = trainer.get_training_metrics().await?;
    let analysis_metrics = pattern_analyzer.get_metrics().await?;
    let recommendation_metrics = recommendation_engine.get_metrics().await?;

    Ok(MLMetricsResponse {
        training_metrics: TrainingMetricsResponse {
            total_models_trained: training_metrics.total_models_trained,
            average_accuracy: training_metrics.average_accuracy,
            total_training_time_hours: training_metrics.total_training_time_hours,
            last_training_completed: training_metrics.last_training_completed.to_rfc3339(),
        },
        analysis_metrics: AnalysisMetricsResponse {
            total_analyses: analysis_metrics.total_analyses,
            total_insights_discovered: analysis_metrics.total_insights_discovered,
            average_insights_per_analysis: analysis_metrics.average_insights_per_analysis,
            last_analysis: analysis_metrics.last_analysis.to_rfc3339(),
        },
        recommendation_metrics: RecommendationMetricsResponse {
            total_recommendations_generated: recommendation_metrics.total_recommendations_generated,
            total_recommendation_requests: recommendation_metrics.total_recommendation_requests,
            average_recommendations_per_request: recommendation_metrics.average_recommendations_per_request,
            recommendation_acceptance_rate: recommendation_metrics.recommendation_acceptance_rate,
            last_recommendation_generated: recommendation_metrics.last_recommendation_generated.to_rfc3339(),
        },
    })
}

// Request/Response structures

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingConfigRequest {
    pub max_epochs: u32,
    pub learning_rate: f64,
    pub batch_size: u32,
    pub validation_split: f64,
    pub early_stopping: bool,
    pub patience: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingJobResponse {
    pub job_id: String,
    pub model_name: String,
    pub status: String,
    pub progress: f64,
    pub started_at: String,
    pub estimated_completion: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InferenceOptionsRequest {
    pub use_cache: bool,
    pub timeout_ms: u64,
    pub confidence_threshold: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InferenceResponse {
    pub request_id: String,
    pub model_name: String,
    pub prediction: crate::services::ml_engine::inference_engine::PredictionOutput,
    pub confidence_score: f64,
    pub inference_time_ms: f64,
    pub timestamp: String,
    pub model_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatternAnalysisResponse {
    pub total_insights: usize,
    pub insights: Vec<PatternInsightResponse>,
    pub analysis_completed_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatternInsightResponse {
    pub pattern_id: String,
    pub pattern_type: String,
    pub title: String,
    pub description: String,
    pub confidence: f64,
    pub impact_score: f64,
    pub actionable_recommendations: Vec<String>,
    pub supporting_data: serde_json::Value,
    pub discovered_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecommendationContextRequest {
    pub current_methodology: Option<String>,
    pub query_complexity: Option<f64>,
    pub recent_performance: Option<f64>,
    pub budget_constraints: Option<f64>,
    pub time_constraints: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecommendationResponse {
    pub total_recommendations: usize,
    pub recommendations: Vec<RecommendationItemResponse>,
    pub generated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecommendationItemResponse {
    pub id: String,
    pub recommendation_type: String,
    pub title: String,
    pub description: String,
    pub relevance_score: f64,
    pub confidence: f64,
    pub impact_estimate: String,
    pub action_required: String,
    pub parameters: serde_json::Value,
    pub created_at: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MLMetricsResponse {
    pub training_metrics: TrainingMetricsResponse,
    pub analysis_metrics: AnalysisMetricsResponse,
    pub recommendation_metrics: RecommendationMetricsResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingMetricsResponse {
    pub total_models_trained: u32,
    pub average_accuracy: f64,
    pub total_training_time_hours: f64,
    pub last_training_completed: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisMetricsResponse {
    pub total_analyses: u64,
    pub total_insights_discovered: u64,
    pub average_insights_per_analysis: f64,
    pub last_analysis: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecommendationMetricsResponse {
    pub total_recommendations_generated: u64,
    pub total_recommendation_requests: u64,
    pub average_recommendations_per_request: f64,
    pub recommendation_acceptance_rate: f64,
    pub last_recommendation_generated: String,
}
