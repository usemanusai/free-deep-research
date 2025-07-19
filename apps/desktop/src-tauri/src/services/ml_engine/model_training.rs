use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, warn, error};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, ResearchError};
use super::inference_engine::{ModelType, MLModel};

/// Model training service for machine learning models
#[derive(Clone)]
pub struct ModelTrainer {
    active_training_jobs: Arc<RwLock<HashMap<Uuid, TrainingJob>>>,
    training_history: Arc<RwLock<Vec<TrainingRecord>>>,
    config: TrainingConfig,
    training_metrics: Arc<RwLock<TrainingMetrics>>,
}

impl ModelTrainer {
    pub fn new(config: TrainingConfig) -> Self {
        Self {
            active_training_jobs: Arc::new(RwLock::new(HashMap::new())),
            training_history: Arc::new(RwLock::new(Vec::new())),
            config,
            training_metrics: Arc::new(RwLock::new(TrainingMetrics::new())),
        }
    }

    /// Start training a new model
    pub async fn start_training(
        &self,
        job_id: Uuid,
        model_name: String,
        model_type: ModelType,
        training_config: TrainingConfig,
        user_id: Uuid,
    ) -> AppResult<TrainingJob> {
        info!("Starting training job {} for model: {}", job_id, model_name);

        // Validate training configuration
        self.validate_training_config(&training_config).await?;

        // Create training job
        let training_job = TrainingJob {
            job_id,
            model_name: model_name.clone(),
            model_type: model_type.clone(),
            status: TrainingStatus::Initializing,
            progress: 0.0,
            started_at: Utc::now(),
            estimated_completion: Utc::now() + Duration::hours(2),
            user_id,
            config: training_config.clone(),
            metrics: TrainingJobMetrics::new(),
            logs: Vec::new(),
        };

        // Store the job
        {
            let mut jobs = self.active_training_jobs.write().await;
            jobs.insert(job_id, training_job.clone());
        }

        // Start training in background
        let trainer_clone = self.clone();
        let job_clone = training_job.clone();
        tokio::spawn(async move {
            if let Err(e) = trainer_clone.execute_training(job_clone).await {
                error!("Training job {} failed: {}", job_id, e);
                trainer_clone.mark_job_failed(job_id, e.to_string()).await;
            }
        });

        info!("Training job {} started successfully", job_id);
        Ok(training_job)
    }

    /// Execute the training process
    async fn execute_training(&self, mut job: TrainingJob) -> AppResult<()> {
        info!("Executing training for job: {}", job.job_id);

        // Update job status
        self.update_job_status(job.job_id, TrainingStatus::DataPreparation, 10.0).await?;

        // Step 1: Data preparation
        let training_data = self.prepare_training_data(&job).await?;
        self.add_job_log(job.job_id, "Data preparation completed".to_string()).await?;

        // Update progress
        self.update_job_status(job.job_id, TrainingStatus::Training, 30.0).await?;

        // Step 2: Model training
        let trained_model = self.train_model(&job, &training_data).await?;
        self.add_job_log(job.job_id, "Model training completed".to_string()).await?;

        // Update progress
        self.update_job_status(job.job_id, TrainingStatus::Validation, 70.0).await?;

        // Step 3: Model validation
        let validation_metrics = self.validate_model(&job, &trained_model, &training_data).await?;
        self.add_job_log(job.job_id, format!("Model validation completed. Accuracy: {:.2}%", validation_metrics.accuracy * 100.0)).await?;

        // Update progress
        self.update_job_status(job.job_id, TrainingStatus::Saving, 90.0).await?;

        // Step 4: Save model
        let model_path = self.save_model(&job, &trained_model).await?;
        self.add_job_log(job.job_id, format!("Model saved to: {}", model_path)).await?;

        // Complete the job
        self.complete_training_job(job.job_id, trained_model, validation_metrics).await?;

        info!("Training job {} completed successfully", job.job_id);
        Ok(())
    }

    /// Prepare training data based on model type
    async fn prepare_training_data(&self, job: &TrainingJob) -> AppResult<TrainingData> {
        debug!("Preparing training data for model type: {:?}", job.model_type);

        match job.model_type {
            ModelType::ResearchPatternPredictor => {
                self.prepare_research_pattern_data().await
            }
            ModelType::UsageForecaster => {
                self.prepare_usage_forecasting_data().await
            }
            ModelType::PerformanceOptimizer => {
                self.prepare_performance_optimization_data().await
            }
            ModelType::RecommendationEngine => {
                self.prepare_recommendation_data().await
            }
            ModelType::AnomalyDetector => {
                self.prepare_anomaly_detection_data().await
            }
        }
    }

    /// Prepare research pattern training data
    async fn prepare_research_pattern_data(&self) -> AppResult<TrainingData> {
        // Simulate data preparation for research pattern prediction
        let mut features = Vec::new();
        let mut labels = Vec::new();

        // Generate synthetic training data
        for i in 0..1000 {
            let query_complexity = (i as f64 / 1000.0) + (rand::random::<f64>() * 0.2);
            let methodology_preference = rand::random::<f64>();
            let historical_success_rate = 0.6 + (rand::random::<f64>() * 0.4);
            
            features.push(vec![query_complexity, methodology_preference, historical_success_rate]);
            
            // Synthetic label: success probability
            let success_prob = (historical_success_rate * 0.6 + (1.0 - query_complexity) * 0.4)
                .max(0.1).min(0.95);
            labels.push(success_prob);
        }

        Ok(TrainingData {
            features,
            labels,
            feature_names: vec![
                "query_complexity".to_string(),
                "methodology_preference".to_string(),
                "historical_success_rate".to_string(),
            ],
            data_size: 1000,
            split_ratio: 0.8,
        })
    }

    /// Prepare usage forecasting training data
    async fn prepare_usage_forecasting_data(&self) -> AppResult<TrainingData> {
        let mut features = Vec::new();
        let mut labels = Vec::new();

        // Generate time series data
        for day in 0..365 {
            for hour in 0..24 {
                let time_features = vec![
                    day as f64 / 365.0,  // Day of year
                    hour as f64 / 24.0,  // Hour of day
                    (day % 7) as f64 / 7.0,  // Day of week
                ];
                
                // Simulate usage pattern
                let base_usage = 100.0;
                let daily_pattern = (hour as f64 * std::f64::consts::PI / 12.0).sin() * 50.0;
                let weekly_pattern = if day % 7 < 5 { 1.2 } else { 0.8 };
                let noise = (rand::random::<f64>() - 0.5) * 20.0;
                
                let usage = base_usage + daily_pattern * weekly_pattern + noise;
                
                features.push(time_features);
                labels.push(usage.max(0.0));
            }
        }

        Ok(TrainingData {
            features,
            labels,
            feature_names: vec![
                "day_of_year".to_string(),
                "hour_of_day".to_string(),
                "day_of_week".to_string(),
            ],
            data_size: 365 * 24,
            split_ratio: 0.8,
        })
    }

    /// Prepare performance optimization training data
    async fn prepare_performance_optimization_data(&self) -> AppResult<TrainingData> {
        let mut features = Vec::new();
        let mut labels = Vec::new();

        // Generate performance data
        for _ in 0..500 {
            let cpu_usage = rand::random::<f64>() * 100.0;
            let memory_usage = rand::random::<f64>() * 100.0;
            let response_time = 100.0 + rand::random::<f64>() * 900.0;
            let throughput = 50.0 + rand::random::<f64>() * 200.0;
            
            features.push(vec![cpu_usage, memory_usage, response_time, throughput]);
            
            // Performance score (0-100)
            let cpu_score = (100.0 - cpu_usage) / 100.0;
            let memory_score = (100.0 - memory_usage) / 100.0;
            let response_score = (1000.0 - response_time.min(1000.0)) / 1000.0;
            let throughput_score = throughput / 250.0;
            
            let performance_score = (cpu_score * 0.25 + memory_score * 0.25 + 
                                   response_score * 0.25 + throughput_score * 0.25) * 100.0;
            
            labels.push(performance_score.max(0.0).min(100.0));
        }

        Ok(TrainingData {
            features,
            labels,
            feature_names: vec![
                "cpu_usage".to_string(),
                "memory_usage".to_string(),
                "response_time".to_string(),
                "throughput".to_string(),
            ],
            data_size: 500,
            split_ratio: 0.8,
        })
    }

    /// Prepare recommendation training data
    async fn prepare_recommendation_data(&self) -> AppResult<TrainingData> {
        let mut features = Vec::new();
        let mut labels = Vec::new();

        // Generate user behavior data
        for user_id in 0..100 {
            for session in 0..10 {
                let user_features = vec![
                    user_id as f64 / 100.0,  // User ID normalized
                    session as f64 / 10.0,   // Session number
                    rand::random::<f64>(),   // Research frequency
                    rand::random::<f64>(),   // Methodology preference
                    rand::random::<f64>(),   // Success rate
                ];
                
                // Recommendation relevance score
                let relevance = user_features.iter().sum::<f64>() / user_features.len() as f64;
                
                features.push(user_features);
                labels.push(relevance);
            }
        }

        Ok(TrainingData {
            features,
            labels,
            feature_names: vec![
                "user_id".to_string(),
                "session_number".to_string(),
                "research_frequency".to_string(),
                "methodology_preference".to_string(),
                "success_rate".to_string(),
            ],
            data_size: 1000,
            split_ratio: 0.8,
        })
    }

    /// Prepare anomaly detection training data
    async fn prepare_anomaly_detection_data(&self) -> AppResult<TrainingData> {
        let mut features = Vec::new();
        let mut labels = Vec::new();

        // Generate normal and anomalous data
        for i in 0..800 {
            let is_anomaly = i >= 720; // Last 80 samples are anomalies
            
            let usage = if is_anomaly {
                100.0 + rand::random::<f64>() * 200.0  // Anomalous usage
            } else {
                50.0 + rand::random::<f64>() * 50.0    // Normal usage
            };
            
            let response_time = if is_anomaly {
                500.0 + rand::random::<f64>() * 1000.0  // Slow response
            } else {
                100.0 + rand::random::<f64>() * 200.0   // Normal response
            };
            
            let error_rate = if is_anomaly {
                0.1 + rand::random::<f64>() * 0.2  // High error rate
            } else {
                rand::random::<f64>() * 0.05       // Low error rate
            };
            
            features.push(vec![usage, response_time, error_rate]);
            labels.push(if is_anomaly { 1.0 } else { 0.0 });
        }

        Ok(TrainingData {
            features,
            labels,
            feature_names: vec![
                "usage".to_string(),
                "response_time".to_string(),
                "error_rate".to_string(),
            ],
            data_size: 800,
            split_ratio: 0.8,
        })
    }

    /// Train the model using the prepared data
    async fn train_model(&self, job: &TrainingJob, training_data: &TrainingData) -> AppResult<TrainedModel> {
        debug!("Training model for job: {}", job.job_id);

        // Simulate training process with progress updates
        for epoch in 0..job.config.max_epochs {
            // Simulate training epoch
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            
            let progress = 30.0 + (epoch as f64 / job.config.max_epochs as f64) * 40.0;
            self.update_job_progress(job.job_id, progress).await?;
            
            // Simulate loss calculation
            let loss = 1.0 - (epoch as f64 / job.config.max_epochs as f64) * 0.8;
            self.add_job_log(job.job_id, format!("Epoch {}: Loss = {:.4}", epoch + 1, loss)).await?;
        }

        // Create trained model
        let model_parameters = self.generate_model_parameters(&job.model_type, training_data).await?;
        
        let trained_model = TrainedModel {
            model_id: Uuid::new_v4(),
            name: job.model_name.clone(),
            model_type: job.model_type.clone(),
            version: "1.0.0".to_string(),
            parameters: model_parameters,
            training_metadata: serde_json::json!({
                "training_data_size": training_data.data_size,
                "feature_names": training_data.feature_names,
                "training_epochs": job.config.max_epochs,
                "learning_rate": job.config.learning_rate,
                "batch_size": job.config.batch_size,
                "trained_at": Utc::now()
            }),
            created_at: Utc::now(),
        };

        Ok(trained_model)
    }

    /// Generate model parameters based on model type
    async fn generate_model_parameters(&self, model_type: &ModelType, training_data: &TrainingData) -> AppResult<HashMap<String, f64>> {
        let mut parameters = HashMap::new();
        
        match model_type {
            ModelType::ResearchPatternPredictor => {
                // Gradient boosting parameters
                parameters.insert("n_estimators".to_string(), 100.0);
                parameters.insert("learning_rate".to_string(), 0.1);
                parameters.insert("max_depth".to_string(), 6.0);
                parameters.insert("feature_importance_query_complexity".to_string(), 0.4);
                parameters.insert("feature_importance_methodology_preference".to_string(), 0.3);
                parameters.insert("feature_importance_historical_success_rate".to_string(), 0.3);
            }
            ModelType::UsageForecaster => {
                // LSTM parameters
                parameters.insert("hidden_size".to_string(), 64.0);
                parameters.insert("num_layers".to_string(), 2.0);
                parameters.insert("dropout".to_string(), 0.2);
                parameters.insert("sequence_length".to_string(), 24.0);
            }
            ModelType::PerformanceOptimizer => {
                // Reinforcement learning parameters
                parameters.insert("learning_rate".to_string(), 0.001);
                parameters.insert("discount_factor".to_string(), 0.95);
                parameters.insert("exploration_rate".to_string(), 0.1);
                parameters.insert("replay_buffer_size".to_string(), 10000.0);
            }
            ModelType::RecommendationEngine => {
                // Collaborative filtering parameters
                parameters.insert("embedding_dim".to_string(), 50.0);
                parameters.insert("regularization".to_string(), 0.01);
                parameters.insert("num_factors".to_string(), 20.0);
            }
            ModelType::AnomalyDetector => {
                // Isolation forest parameters
                parameters.insert("n_estimators".to_string(), 100.0);
                parameters.insert("contamination".to_string(), 0.1);
                parameters.insert("max_samples".to_string(), 256.0);
            }
        }
        
        Ok(parameters)
    }

    /// Validate the trained model
    async fn validate_model(&self, job: &TrainingJob, model: &TrainedModel, training_data: &TrainingData) -> AppResult<ModelMetrics> {
        debug!("Validating model for job: {}", job.job_id);

        // Split data for validation
        let split_index = (training_data.data_size as f64 * training_data.split_ratio) as usize;
        let validation_features = &training_data.features[split_index..];
        let validation_labels = &training_data.labels[split_index..];

        // Simulate model validation
        let mut correct_predictions = 0;
        let mut total_predictions = validation_features.len();
        let mut total_error = 0.0;

        for (i, features) in validation_features.iter().enumerate() {
            let predicted = self.simulate_prediction(&job.model_type, features).await?;
            let actual = validation_labels[i];
            
            let error = (predicted - actual).abs();
            total_error += error;
            
            // For classification-like metrics
            if error < 0.1 {
                correct_predictions += 1;
            }
        }

        let accuracy = correct_predictions as f64 / total_predictions as f64;
        let mean_absolute_error = total_error / total_predictions as f64;
        let rmse = (total_error / total_predictions as f64).sqrt();

        let metrics = ModelMetrics {
            accuracy,
            precision: accuracy * 0.95, // Simulated
            recall: accuracy * 0.92,    // Simulated
            f1_score: accuracy * 0.93,  // Simulated
            mean_absolute_error,
            root_mean_square_error: rmse,
            r_squared: accuracy,
            validation_loss: mean_absolute_error,
            training_time_seconds: 120.0, // Simulated
            model_size_mb: 5.2,          // Simulated
        };

        Ok(metrics)
    }

    /// Simulate prediction for validation
    async fn simulate_prediction(&self, model_type: &ModelType, features: &[f64]) -> AppResult<f64> {
        match model_type {
            ModelType::ResearchPatternPredictor => {
                // Simulate research pattern prediction
                let complexity = features.get(0).unwrap_or(&0.5);
                let preference = features.get(1).unwrap_or(&0.5);
                let success_rate = features.get(2).unwrap_or(&0.8);
                Ok((success_rate * 0.6 + (1.0 - complexity) * 0.4).max(0.1).min(0.95))
            }
            ModelType::UsageForecaster => {
                // Simulate usage forecasting
                let day_factor = features.get(0).unwrap_or(&0.5);
                let hour_factor = features.get(1).unwrap_or(&0.5);
                let week_factor = features.get(2).unwrap_or(&0.5);
                Ok(100.0 + day_factor * 50.0 + hour_factor * 30.0 + week_factor * 20.0)
            }
            ModelType::PerformanceOptimizer => {
                // Simulate performance score
                let cpu = features.get(0).unwrap_or(&50.0);
                let memory = features.get(1).unwrap_or(&50.0);
                let response_time = features.get(2).unwrap_or(&200.0);
                let throughput = features.get(3).unwrap_or(&100.0);
                
                let cpu_score = (100.0 - cpu) / 100.0;
                let memory_score = (100.0 - memory) / 100.0;
                let response_score = (1000.0 - response_time.min(1000.0)) / 1000.0;
                let throughput_score = throughput / 250.0;
                
                Ok((cpu_score * 0.25 + memory_score * 0.25 + response_score * 0.25 + throughput_score * 0.25) * 100.0)
            }
            ModelType::RecommendationEngine => {
                // Simulate recommendation relevance
                Ok(features.iter().sum::<f64>() / features.len() as f64)
            }
            ModelType::AnomalyDetector => {
                // Simulate anomaly detection
                let usage = features.get(0).unwrap_or(&50.0);
                let response_time = features.get(1).unwrap_or(&200.0);
                let error_rate = features.get(2).unwrap_or(&0.01);
                
                if usage > &150.0 || response_time > &500.0 || error_rate > &0.1 {
                    Ok(1.0) // Anomaly
                } else {
                    Ok(0.0) // Normal
                }
            }
        }
    }

    /// Save the trained model
    async fn save_model(&self, job: &TrainingJob, model: &TrainedModel) -> AppResult<String> {
        let model_path = format!("models/{}/{}.model", job.model_type.to_string(), model.model_id);
        
        // In a real implementation, this would serialize and save the model to disk
        // For now, we'll just simulate the save operation
        
        debug!("Saving model to: {}", model_path);
        
        // Simulate file save
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        
        Ok(model_path)
    }

    /// Complete the training job
    async fn complete_training_job(&self, job_id: Uuid, model: TrainedModel, metrics: ModelMetrics) -> AppResult<()> {
        // Update job status
        self.update_job_status(job_id, TrainingStatus::Completed, 100.0).await?;
        
        // Create training record
        let training_record = TrainingRecord {
            job_id,
            model_id: model.model_id,
            model_name: model.name.clone(),
            model_type: model.model_type.clone(),
            training_started: Utc::now() - Duration::hours(2), // Simulated
            training_completed: Utc::now(),
            metrics: metrics.clone(),
            status: TrainingStatus::Completed,
        };
        
        // Store training record
        {
            let mut history = self.training_history.write().await;
            history.push(training_record);
        }
        
        // Remove from active jobs
        {
            let mut jobs = self.active_training_jobs.write().await;
            jobs.remove(&job_id);
        }
        
        // Update training metrics
        {
            let mut training_metrics = self.training_metrics.write().await;
            training_metrics.total_models_trained += 1;
            training_metrics.average_accuracy = (training_metrics.average_accuracy * (training_metrics.total_models_trained - 1) as f64 + metrics.accuracy) / training_metrics.total_models_trained as f64;
            training_metrics.last_training_completed = Utc::now();
        }
        
        info!("Training job {} completed successfully", job_id);
        Ok(())
    }

    /// Mark job as failed
    async fn mark_job_failed(&self, job_id: Uuid, error_message: String) {
        if let Ok(mut jobs) = self.active_training_jobs.try_write() {
            if let Some(job) = jobs.get_mut(&job_id) {
                job.status = TrainingStatus::Failed;
                job.logs.push(TrainingLog {
                    timestamp: Utc::now(),
                    level: LogLevel::Error,
                    message: error_message,
                });
            }
        }
    }

    /// Update job status and progress
    async fn update_job_status(&self, job_id: Uuid, status: TrainingStatus, progress: f64) -> AppResult<()> {
        let mut jobs = self.active_training_jobs.write().await;
        if let Some(job) = jobs.get_mut(&job_id) {
            job.status = status;
            job.progress = progress;
        }
        Ok(())
    }

    /// Update job progress only
    async fn update_job_progress(&self, job_id: Uuid, progress: f64) -> AppResult<()> {
        let mut jobs = self.active_training_jobs.write().await;
        if let Some(job) = jobs.get_mut(&job_id) {
            job.progress = progress;
        }
        Ok(())
    }

    /// Add log entry to job
    async fn add_job_log(&self, job_id: Uuid, message: String) -> AppResult<()> {
        let mut jobs = self.active_training_jobs.write().await;
        if let Some(job) = jobs.get_mut(&job_id) {
            job.logs.push(TrainingLog {
                timestamp: Utc::now(),
                level: LogLevel::Info,
                message,
            });
        }
        Ok(())
    }

    /// Validate training configuration
    async fn validate_training_config(&self, config: &TrainingConfig) -> AppResult<()> {
        if config.max_epochs == 0 {
            return Err(ResearchError::invalid_request("Max epochs must be greater than 0".to_string()).into());
        }
        
        if config.learning_rate <= 0.0 || config.learning_rate > 1.0 {
            return Err(ResearchError::invalid_request("Learning rate must be between 0 and 1".to_string()).into());
        }
        
        if config.batch_size == 0 {
            return Err(ResearchError::invalid_request("Batch size must be greater than 0".to_string()).into());
        }
        
        Ok(())
    }

    /// Get training job status
    pub async fn get_job_status(&self, job_id: Uuid) -> AppResult<Option<TrainingJob>> {
        let jobs = self.active_training_jobs.read().await;
        Ok(jobs.get(&job_id).cloned())
    }

    /// Get training history
    pub async fn get_training_history(&self) -> AppResult<Vec<TrainingRecord>> {
        let history = self.training_history.read().await;
        Ok(history.clone())
    }

    /// Get training metrics
    pub async fn get_training_metrics(&self) -> AppResult<TrainingMetrics> {
        let metrics = self.training_metrics.read().await;
        Ok(metrics.clone())
    }
}

// Supporting data structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub max_epochs: u32,
    pub learning_rate: f64,
    pub batch_size: u32,
    pub validation_split: f64,
    pub early_stopping: bool,
    pub patience: u32,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            max_epochs: 100,
            learning_rate: 0.001,
            batch_size: 32,
            validation_split: 0.2,
            early_stopping: true,
            patience: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingJob {
    pub job_id: Uuid,
    pub model_name: String,
    pub model_type: ModelType,
    pub status: TrainingStatus,
    pub progress: f64,
    pub started_at: DateTime<Utc>,
    pub estimated_completion: DateTime<Utc>,
    pub user_id: Uuid,
    pub config: TrainingConfig,
    pub metrics: TrainingJobMetrics,
    pub logs: Vec<TrainingLog>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrainingStatus {
    Initializing,
    DataPreparation,
    Training,
    Validation,
    Saving,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingJobMetrics {
    pub current_epoch: u32,
    pub current_loss: f64,
    pub best_validation_score: f64,
    pub training_samples_processed: u64,
}

impl TrainingJobMetrics {
    fn new() -> Self {
        Self {
            current_epoch: 0,
            current_loss: 0.0,
            best_validation_score: 0.0,
            training_samples_processed: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingLog {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct TrainingData {
    pub features: Vec<Vec<f64>>,
    pub labels: Vec<f64>,
    pub feature_names: Vec<String>,
    pub data_size: usize,
    pub split_ratio: f64,
}

#[derive(Debug, Clone)]
pub struct TrainedModel {
    pub model_id: Uuid,
    pub name: String,
    pub model_type: ModelType,
    pub version: String,
    pub parameters: HashMap<String, f64>,
    pub training_metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub mean_absolute_error: f64,
    pub root_mean_square_error: f64,
    pub r_squared: f64,
    pub validation_loss: f64,
    pub training_time_seconds: f64,
    pub model_size_mb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingRecord {
    pub job_id: Uuid,
    pub model_id: Uuid,
    pub model_name: String,
    pub model_type: ModelType,
    pub training_started: DateTime<Utc>,
    pub training_completed: DateTime<Utc>,
    pub metrics: ModelMetrics,
    pub status: TrainingStatus,
}

#[derive(Debug, Clone)]
pub struct TrainingMetrics {
    pub total_models_trained: u32,
    pub average_accuracy: f64,
    pub total_training_time_hours: f64,
    pub last_training_completed: DateTime<Utc>,
}

impl TrainingMetrics {
    fn new() -> Self {
        Self {
            total_models_trained: 0,
            average_accuracy: 0.0,
            total_training_time_hours: 0.0,
            last_training_completed: Utc::now(),
        }
    }
}

impl ToString for ModelType {
    fn to_string(&self) -> String {
        match self {
            ModelType::ResearchPatternPredictor => "research_pattern_predictor".to_string(),
            ModelType::UsageForecaster => "usage_forecaster".to_string(),
            ModelType::PerformanceOptimizer => "performance_optimizer".to_string(),
            ModelType::RecommendationEngine => "recommendation_engine".to_string(),
            ModelType::AnomalyDetector => "anomaly_detector".to_string(),
        }
    }
}
