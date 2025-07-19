use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, error, warn};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, ResearchError};
use crate::models::research_workflow::{ResearchWorkflow, StepStatus, WorkflowStatus};

pub mod comparison_engine;
pub mod analysis_engine;
pub mod similarity_detector;
pub mod performance_analyzer;

use self::comparison_engine::{ComparisonEngine, ComparisonRequest, ComparisonResult};
use self::analysis_engine::{AnalysisEngine, AnalysisRequest, AnalysisResult};
use self::similarity_detector::{SimilarityDetector, SimilarityScore, ClusterResult};
use self::performance_analyzer::{PerformanceAnalyzer, PerformanceMetrics, BenchmarkResult};

/// Analysis service for research workflow comparison and insights
pub struct AnalysisService {
    comparison_engine: Arc<ComparisonEngine>,
    analysis_engine: Arc<AnalysisEngine>,
    similarity_detector: Arc<SimilarityDetector>,
    performance_analyzer: Arc<PerformanceAnalyzer>,
    analysis_history: Arc<RwLock<Vec<AnalysisResult>>>,
    comparison_history: Arc<RwLock<Vec<ComparisonResult>>>,
}

/// Analysis request types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisType {
    Comparison,
    Statistical,
    Similarity,
    Performance,
    Trend,
    Quality,
}

/// Comprehensive analysis request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveAnalysisRequest {
    pub id: Uuid,
    pub workflow_ids: Vec<Uuid>,
    pub analysis_types: Vec<AnalysisType>,
    pub time_range: Option<TimeRange>,
    pub filters: AnalysisFilters,
    pub options: AnalysisOptions,
}

/// Time range for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

/// Analysis filters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisFilters {
    pub status_filter: Option<Vec<WorkflowStatus>>,
    pub step_status_filter: Option<Vec<StepStatus>>,
    pub min_duration_minutes: Option<u32>,
    pub max_duration_minutes: Option<u32>,
    pub query_pattern: Option<String>,
    pub tag_filter: Option<Vec<String>>,
}

impl Default for AnalysisFilters {
    fn default() -> Self {
        Self {
            status_filter: None,
            step_status_filter: None,
            min_duration_minutes: None,
            max_duration_minutes: None,
            query_pattern: None,
            tag_filter: None,
        }
    }
}

/// Analysis options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisOptions {
    pub include_detailed_metrics: bool,
    pub include_recommendations: bool,
    pub include_visualizations: bool,
    pub confidence_threshold: f64,
    pub max_results: Option<usize>,
    pub group_by_similarity: bool,
}

impl Default for AnalysisOptions {
    fn default() -> Self {
        Self {
            include_detailed_metrics: true,
            include_recommendations: true,
            include_visualizations: false,
            confidence_threshold: 0.8,
            max_results: Some(100),
            group_by_similarity: false,
        }
    }
}

/// Comprehensive analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveAnalysisResult {
    pub id: Uuid,
    pub request_id: Uuid,
    pub workflow_count: usize,
    pub analysis_types: Vec<AnalysisType>,
    pub comparison_results: Vec<ComparisonResult>,
    pub statistical_analysis: Option<AnalysisResult>,
    pub similarity_analysis: Option<ClusterResult>,
    pub performance_analysis: Option<BenchmarkResult>,
    pub insights: Vec<AnalysisInsight>,
    pub recommendations: Vec<AnalysisRecommendation>,
    pub quality_score: f64,
    pub confidence_score: f64,
    pub created_at: DateTime<Utc>,
    pub processing_time_ms: u64,
}

/// Analysis insight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisInsight {
    pub insight_type: InsightType,
    pub title: String,
    pub description: String,
    pub confidence: f64,
    pub impact_level: ImpactLevel,
    pub supporting_data: HashMap<String, serde_json::Value>,
}

/// Types of insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsightType {
    Performance,
    Quality,
    Efficiency,
    Pattern,
    Anomaly,
    Trend,
    Optimization,
}

/// Impact levels for insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

/// Analysis recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisRecommendation {
    pub recommendation_type: RecommendationType,
    pub title: String,
    pub description: String,
    pub priority: RecommendationPriority,
    pub expected_improvement: String,
    pub implementation_steps: Vec<String>,
    pub estimated_effort: ImplementationEffort,
}

/// Types of recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    ProcessOptimization,
    ResourceAllocation,
    QualityImprovement,
    PerformanceEnhancement,
    ErrorReduction,
    ConfigurationChange,
    WorkflowRedesign,
}

/// Recommendation priorities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Critical,
    High,
    Medium,
    Low,
}

/// Implementation effort levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationEffort {
    Minimal,    // < 1 hour
    Low,        // 1-4 hours
    Medium,     // 1-2 days
    High,       // 1-2 weeks
    Significant, // > 2 weeks
}

/// Analysis statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisStatistics {
    pub total_analyses_performed: u64,
    pub analyses_by_type: HashMap<AnalysisType, u64>,
    pub average_processing_time_ms: f64,
    pub insights_generated: u64,
    pub recommendations_made: u64,
    pub most_common_insights: Vec<InsightType>,
    pub success_rate: f64,
}

impl AnalysisService {
    /// Create a new analysis service
    pub async fn new() -> AppResult<Self> {
        info!("Initializing analysis service...");

        let comparison_engine = Arc::new(ComparisonEngine::new().await?);
        let analysis_engine = Arc::new(AnalysisEngine::new().await?);
        let similarity_detector = Arc::new(SimilarityDetector::new().await?);
        let performance_analyzer = Arc::new(PerformanceAnalyzer::new().await?);
        let analysis_history = Arc::new(RwLock::new(Vec::new()));
        let comparison_history = Arc::new(RwLock::new(Vec::new()));

        let service = Self {
            comparison_engine,
            analysis_engine,
            similarity_detector,
            performance_analyzer,
            analysis_history,
            comparison_history,
        };

        info!("Analysis service initialized successfully");
        Ok(service)
    }

    /// Perform comprehensive analysis on workflows
    pub async fn perform_comprehensive_analysis(
        &self,
        workflows: &[ResearchWorkflow],
        request: ComprehensiveAnalysisRequest,
    ) -> AppResult<ComprehensiveAnalysisResult> {
        info!("Performing comprehensive analysis on {} workflows", workflows.len());

        let start_time = std::time::Instant::now();
        let mut comparison_results = Vec::new();
        let mut statistical_analysis = None;
        let mut similarity_analysis = None;
        let mut performance_analysis = None;

        // Filter workflows based on request filters
        let filtered_workflows = self.apply_filters(workflows, &request.filters);

        // Perform requested analysis types
        for analysis_type in &request.analysis_types {
            match analysis_type {
                AnalysisType::Comparison => {
                    if filtered_workflows.len() >= 2 {
                        let comparison_request = ComparisonRequest {
                            id: Uuid::new_v4(),
                            workflow_ids: filtered_workflows.iter().map(|w| w.id).collect(),
                            comparison_type: comparison_engine::ComparisonType::Comprehensive,
                            options: comparison_engine::ComparisonOptions::default(),
                        };
                        
                        match self.comparison_engine.compare_workflows(&filtered_workflows, comparison_request).await {
                            Ok(result) => comparison_results.push(result),
                            Err(e) => warn!("Comparison analysis failed: {}", e),
                        }
                    }
                }
                AnalysisType::Statistical => {
                    let analysis_request = AnalysisRequest {
                        id: Uuid::new_v4(),
                        workflow_ids: filtered_workflows.iter().map(|w| w.id).collect(),
                        analysis_type: analysis_engine::StatisticalAnalysisType::Comprehensive,
                        options: analysis_engine::AnalysisEngineOptions::default(),
                    };

                    match self.analysis_engine.analyze_workflows(&filtered_workflows, analysis_request).await {
                        Ok(result) => statistical_analysis = Some(result),
                        Err(e) => warn!("Statistical analysis failed: {}", e),
                    }
                }
                AnalysisType::Similarity => {
                    match self.similarity_detector.detect_similarity(&filtered_workflows).await {
                        Ok(result) => similarity_analysis = Some(result),
                        Err(e) => warn!("Similarity analysis failed: {}", e),
                    }
                }
                AnalysisType::Performance => {
                    match self.performance_analyzer.analyze_performance(&filtered_workflows).await {
                        Ok(result) => performance_analysis = Some(result),
                        Err(e) => warn!("Performance analysis failed: {}", e),
                    }
                }
                _ => {
                    debug!("Analysis type {:?} not yet implemented", analysis_type);
                }
            }
        }

        // Generate insights and recommendations
        let insights = self.generate_insights(&filtered_workflows, &comparison_results, &statistical_analysis, &similarity_analysis, &performance_analysis).await?;
        let recommendations = self.generate_recommendations(&insights, &request.options).await?;

        // Calculate quality and confidence scores
        let quality_score = self.calculate_quality_score(&filtered_workflows, &insights).await?;
        let confidence_score = self.calculate_confidence_score(&insights, &recommendations).await?;

        let processing_time = start_time.elapsed();

        let result = ComprehensiveAnalysisResult {
            id: Uuid::new_v4(),
            request_id: request.id,
            workflow_count: filtered_workflows.len(),
            analysis_types: request.analysis_types,
            comparison_results,
            statistical_analysis,
            similarity_analysis,
            performance_analysis,
            insights,
            recommendations,
            quality_score,
            confidence_score,
            created_at: Utc::now(),
            processing_time_ms: processing_time.as_millis() as u64,
        };

        // Store in history
        {
            let mut history = self.analysis_history.write().await;
            if let Some(ref stats) = result.statistical_analysis {
                history.push(stats.clone());
            }

            // Keep only last 1000 analyses
            if history.len() > 1000 {
                history.remove(0);
            }
        }

        info!("Comprehensive analysis completed in {}ms", processing_time.as_millis());
        Ok(result)
    }

    /// Apply filters to workflows
    fn apply_filters(&self, workflows: &[ResearchWorkflow], filters: &AnalysisFilters) -> Vec<ResearchWorkflow> {
        workflows.iter()
            .filter(|workflow| {
                // Status filter
                if let Some(ref status_filter) = filters.status_filter {
                    if !status_filter.contains(&workflow.status) {
                        return false;
                    }
                }

                // Duration filter
                if let (Some(started), Some(completed)) = (workflow.started_at, workflow.completed_at) {
                    let duration_minutes = (completed - started).num_minutes() as u32;
                    
                    if let Some(min_duration) = filters.min_duration_minutes {
                        if duration_minutes < min_duration {
                            return false;
                        }
                    }
                    
                    if let Some(max_duration) = filters.max_duration_minutes {
                        if duration_minutes > max_duration {
                            return false;
                        }
                    }
                }

                // Query pattern filter
                if let Some(ref pattern) = filters.query_pattern {
                    if !workflow.query.to_lowercase().contains(&pattern.to_lowercase()) {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect()
    }

    /// Generate insights from analysis results
    async fn generate_insights(
        &self,
        _workflows: &[ResearchWorkflow],
        _comparison_results: &[ComparisonResult],
        _statistical_analysis: &Option<AnalysisResult>,
        _similarity_analysis: &Option<ClusterResult>,
        _performance_analysis: &Option<BenchmarkResult>,
    ) -> AppResult<Vec<AnalysisInsight>> {
        // For now, generate sample insights
        // In a real implementation, this would analyze the results and generate meaningful insights
        Ok(vec![
            AnalysisInsight {
                insight_type: InsightType::Performance,
                title: "Workflow Performance Variation".to_string(),
                description: "Significant performance differences detected between similar workflows".to_string(),
                confidence: 0.85,
                impact_level: ImpactLevel::Medium,
                supporting_data: HashMap::new(),
            },
            AnalysisInsight {
                insight_type: InsightType::Quality,
                title: "Result Quality Consistency".to_string(),
                description: "High consistency in result quality across analyzed workflows".to_string(),
                confidence: 0.92,
                impact_level: ImpactLevel::Low,
                supporting_data: HashMap::new(),
            },
        ])
    }

    /// Generate recommendations based on insights
    async fn generate_recommendations(
        &self,
        insights: &[AnalysisInsight],
        _options: &AnalysisOptions,
    ) -> AppResult<Vec<AnalysisRecommendation>> {
        let mut recommendations = Vec::new();

        for insight in insights {
            match insight.insight_type {
                InsightType::Performance => {
                    recommendations.push(AnalysisRecommendation {
                        recommendation_type: RecommendationType::PerformanceEnhancement,
                        title: "Optimize Workflow Performance".to_string(),
                        description: "Consider optimizing slower workflows based on performance patterns".to_string(),
                        priority: RecommendationPriority::Medium,
                        expected_improvement: "20-30% performance improvement".to_string(),
                        implementation_steps: vec![
                            "Identify performance bottlenecks".to_string(),
                            "Optimize resource allocation".to_string(),
                            "Implement caching strategies".to_string(),
                        ],
                        estimated_effort: ImplementationEffort::Medium,
                    });
                }
                InsightType::Quality => {
                    recommendations.push(AnalysisRecommendation {
                        recommendation_type: RecommendationType::QualityImprovement,
                        title: "Maintain Quality Standards".to_string(),
                        description: "Continue current practices to maintain high quality results".to_string(),
                        priority: RecommendationPriority::Low,
                        expected_improvement: "Sustained quality levels".to_string(),
                        implementation_steps: vec![
                            "Monitor quality metrics".to_string(),
                            "Regular quality assessments".to_string(),
                        ],
                        estimated_effort: ImplementationEffort::Minimal,
                    });
                }
                _ => {}
            }
        }

        Ok(recommendations)
    }

    /// Calculate quality score for workflows
    async fn calculate_quality_score(&self, workflows: &[ResearchWorkflow], _insights: &[AnalysisInsight]) -> AppResult<f64> {
        if workflows.is_empty() {
            return Ok(0.0);
        }

        let completed_workflows = workflows.iter().filter(|w| w.status == WorkflowStatus::Completed).count();
        let completion_rate = completed_workflows as f64 / workflows.len() as f64;

        // Simple quality score based on completion rate
        // In a real implementation, this would consider multiple factors
        Ok(completion_rate * 100.0)
    }

    /// Calculate confidence score for analysis
    async fn calculate_confidence_score(&self, insights: &[AnalysisInsight], _recommendations: &[AnalysisRecommendation]) -> AppResult<f64> {
        if insights.is_empty() {
            return Ok(0.0);
        }

        let average_confidence = insights.iter().map(|i| i.confidence).sum::<f64>() / insights.len() as f64;
        Ok(average_confidence * 100.0)
    }

    /// Get analysis statistics
    pub async fn get_analysis_statistics(&self) -> AppResult<AnalysisStatistics> {
        let history = self.analysis_history.read().await;
        
        // For now, return basic statistics
        // In a real implementation, this would track detailed metrics
        Ok(AnalysisStatistics {
            total_analyses_performed: history.len() as u64,
            analyses_by_type: HashMap::new(),
            average_processing_time_ms: 0.0,
            insights_generated: 0,
            recommendations_made: 0,
            most_common_insights: Vec::new(),
            success_rate: 100.0,
        })
    }
}

// Re-export types for external use
pub use comparison_engine::{ComparisonEngine, ComparisonRequest, ComparisonResult};
pub use analysis_engine::{AnalysisEngine, AnalysisRequest, AnalysisResult};
pub use similarity_detector::{SimilarityDetector, SimilarityScore, ClusterResult};
pub use performance_analyzer::{PerformanceAnalyzer, PerformanceMetrics, BenchmarkResult};
