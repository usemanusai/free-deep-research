use std::collections::HashMap;
use tracing::{info, debug};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, ResearchError};
use crate::models::research_workflow::{ResearchWorkflow, StepStatus, WorkflowStatus};

/// Performance analyzer for workflow benchmarking and optimization
pub struct PerformanceAnalyzer;

/// Performance metrics for workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub workflow_id: Uuid,
    pub execution_time_minutes: f64,
    pub step_count: usize,
    pub success_rate: f64,
    pub throughput_score: f64,
    pub efficiency_score: f64,
    pub resource_utilization: ResourceMetrics,
    pub bottlenecks: Vec<PerformanceBottleneck>,
    pub performance_grade: PerformanceGrade,
}

/// Resource utilization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub api_calls_count: u32,
    pub bandwidth_mb: f64,
    pub cache_hit_rate: f64,
    pub error_rate: f64,
}

/// Performance bottleneck
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBottleneck {
    pub step_index: usize,
    pub step_type: String,
    pub bottleneck_type: BottleneckType,
    pub severity: BottleneckSeverity,
    pub impact_percentage: f64,
    pub description: String,
    pub optimization_suggestion: String,
}

/// Types of performance bottlenecks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BottleneckType {
    CPU,
    Memory,
    Network,
    IO,
    API,
    Processing,
    Synchronization,
}

/// Bottleneck severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BottleneckSeverity {
    Critical,
    High,
    Medium,
    Low,
}

/// Performance grades
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceGrade {
    Excellent, // A+
    VeryGood,  // A
    Good,      // B
    Average,   // C
    BelowAverage, // D
    Poor,      // F
}

/// Benchmark result for performance analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub id: Uuid,
    pub workflow_metrics: Vec<PerformanceMetrics>,
    pub aggregate_metrics: AggregatePerformanceMetrics,
    pub performance_ranking: Vec<Uuid>,
    pub benchmark_insights: Vec<BenchmarkInsight>,
    pub optimization_recommendations: Vec<OptimizationRecommendation>,
    pub performance_trends: PerformanceTrends,
    pub created_at: DateTime<Utc>,
    pub processing_time_ms: u64,
}

/// Aggregate performance metrics across all workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatePerformanceMetrics {
    pub total_workflows: usize,
    pub average_execution_time: f64,
    pub median_execution_time: f64,
    pub fastest_execution_time: f64,
    pub slowest_execution_time: f64,
    pub overall_success_rate: f64,
    pub average_throughput: f64,
    pub resource_efficiency: f64,
    pub performance_distribution: PerformanceDistribution,
}

/// Performance distribution across grades
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDistribution {
    pub excellent_count: u32,
    pub very_good_count: u32,
    pub good_count: u32,
    pub average_count: u32,
    pub below_average_count: u32,
    pub poor_count: u32,
}

/// Benchmark insight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkInsight {
    pub insight_type: BenchmarkInsightType,
    pub title: String,
    pub description: String,
    pub impact_level: InsightImpactLevel,
    pub confidence: f64,
    pub affected_workflows: Vec<Uuid>,
    pub supporting_data: HashMap<String, f64>,
}

/// Types of benchmark insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BenchmarkInsightType {
    PerformancePattern,
    ResourceBottleneck,
    EfficiencyOpportunity,
    ScalabilityIssue,
    OptimizationPotential,
    PerformanceRegression,
}

/// Impact levels for insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsightImpactLevel {
    Critical,
    High,
    Medium,
    Low,
}

/// Optimization recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRecommendation {
    pub recommendation_id: Uuid,
    pub optimization_type: OptimizationType,
    pub title: String,
    pub description: String,
    pub priority: OptimizationPriority,
    pub expected_improvement: ExpectedImprovement,
    pub implementation_complexity: ImplementationComplexity,
    pub implementation_steps: Vec<String>,
    pub affected_workflows: Vec<Uuid>,
    pub estimated_effort_hours: f64,
}

/// Types of optimizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationType {
    Parallelization,
    Caching,
    ResourceOptimization,
    AlgorithmImprovement,
    ConfigurationTuning,
    ArchitecturalChange,
    ProcessOptimization,
}

/// Optimization priorities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationPriority {
    Critical,
    High,
    Medium,
    Low,
}

/// Expected improvement from optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedImprovement {
    pub performance_gain_percentage: f64,
    pub resource_savings_percentage: f64,
    pub reliability_improvement: f64,
    pub cost_reduction_percentage: f64,
}

/// Implementation complexity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationComplexity {
    Trivial,    // < 1 hour
    Simple,     // 1-4 hours
    Moderate,   // 1-2 days
    Complex,    // 1-2 weeks
    VeryComplex, // > 2 weeks
}

/// Performance trends over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrends {
    pub execution_time_trend: TrendDirection,
    pub success_rate_trend: TrendDirection,
    pub resource_usage_trend: TrendDirection,
    pub throughput_trend: TrendDirection,
    pub trend_confidence: f64,
    pub trend_analysis_period_days: u32,
}

/// Trend directions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Improving,
    Stable,
    Degrading,
    Volatile,
    InsufficientData,
}

impl PerformanceAnalyzer {
    /// Create a new performance analyzer
    pub async fn new() -> AppResult<Self> {
        info!("Initializing performance analyzer...");
        Ok(Self)
    }

    /// Analyze performance of workflows
    pub async fn analyze_performance(&self, workflows: &[ResearchWorkflow]) -> AppResult<BenchmarkResult> {
        info!("Analyzing performance of {} workflows", workflows.len());

        let start_time = std::time::Instant::now();

        if workflows.is_empty() {
            return Err(ResearchError::invalid_request(
                "No workflows provided for performance analysis".to_string()
            ).into());
        }

        // Calculate performance metrics for each workflow
        let mut workflow_metrics = Vec::new();
        for workflow in workflows {
            let metrics = self.calculate_workflow_performance_metrics(workflow).await?;
            workflow_metrics.push(metrics);
        }

        // Calculate aggregate metrics
        let aggregate_metrics = self.calculate_aggregate_metrics(&workflow_metrics).await?;

        // Create performance ranking
        let performance_ranking = self.create_performance_ranking(&workflow_metrics).await?;

        // Generate insights
        let benchmark_insights = self.generate_benchmark_insights(&workflow_metrics, &aggregate_metrics).await?;

        // Generate optimization recommendations
        let optimization_recommendations = self.generate_optimization_recommendations(&workflow_metrics, &benchmark_insights).await?;

        // Analyze performance trends
        let performance_trends = self.analyze_performance_trends(workflows).await?;

        let processing_time = start_time.elapsed();

        Ok(BenchmarkResult {
            id: Uuid::new_v4(),
            workflow_metrics,
            aggregate_metrics,
            performance_ranking,
            benchmark_insights,
            optimization_recommendations,
            performance_trends,
            created_at: Utc::now(),
            processing_time_ms: processing_time.as_millis() as u64,
        })
    }

    /// Calculate performance metrics for a single workflow
    async fn calculate_workflow_performance_metrics(&self, workflow: &ResearchWorkflow) -> AppResult<PerformanceMetrics> {
        // Calculate execution time
        let execution_time_minutes = if let (Some(started), Some(completed)) = (workflow.started_at, workflow.completed_at) {
            (completed - started).num_minutes() as f64
        } else {
            0.0
        };

        // Calculate success rate
        let success_rate = if workflow.steps.is_empty() {
            0.0
        } else {
            let completed_steps = workflow.steps.iter().filter(|s| s.status == StepStatus::Completed).count();
            completed_steps as f64 / workflow.steps.len() as f64
        };

        // Calculate throughput score (steps per minute)
        let throughput_score = if execution_time_minutes > 0.0 {
            workflow.steps.len() as f64 / execution_time_minutes
        } else {
            0.0
        };

        // Calculate efficiency score (success rate weighted by time)
        let efficiency_score = if execution_time_minutes > 0.0 {
            success_rate / (execution_time_minutes / 60.0) // Success per hour
        } else {
            success_rate
        };

        // Estimate resource utilization (placeholder values)
        let resource_utilization = ResourceMetrics {
            memory_usage_mb: 100.0 + (workflow.steps.len() as f64 * 10.0),
            cpu_usage_percent: 20.0 + (workflow.steps.len() as f64 * 2.0),
            api_calls_count: workflow.steps.len() as u32 * 3,
            bandwidth_mb: workflow.steps.len() as f64 * 5.0,
            cache_hit_rate: 0.8,
            error_rate: 1.0 - success_rate,
        };

        // Detect bottlenecks
        let bottlenecks = self.detect_workflow_bottlenecks(workflow).await?;

        // Calculate performance grade
        let performance_grade = self.calculate_performance_grade(execution_time_minutes, success_rate, efficiency_score);

        Ok(PerformanceMetrics {
            workflow_id: workflow.id,
            execution_time_minutes,
            step_count: workflow.steps.len(),
            success_rate,
            throughput_score,
            efficiency_score,
            resource_utilization,
            bottlenecks,
            performance_grade,
        })
    }

    /// Detect bottlenecks in a workflow
    async fn detect_workflow_bottlenecks(&self, workflow: &ResearchWorkflow) -> AppResult<Vec<PerformanceBottleneck>> {
        let mut bottlenecks = Vec::new();

        // Analyze each step for potential bottlenecks
        for (index, step) in workflow.steps.iter().enumerate() {
            // Check for failed steps
            if step.status == StepStatus::Failed {
                bottlenecks.push(PerformanceBottleneck {
                    step_index: index,
                    step_type: step.step_type.clone(),
                    bottleneck_type: BottleneckType::Processing,
                    severity: BottleneckSeverity::High,
                    impact_percentage: 100.0 / workflow.steps.len() as f64,
                    description: "Step failed during execution".to_string(),
                    optimization_suggestion: "Review step configuration and error handling".to_string(),
                });
            }

            // Check for long-running steps (placeholder logic)
            if let (Some(started), Some(completed)) = (step.started_at, step.completed_at) {
                let step_duration = (completed - started).num_minutes();
                if step_duration > 10 { // More than 10 minutes
                    bottlenecks.push(PerformanceBottleneck {
                        step_index: index,
                        step_type: step.step_type.clone(),
                        bottleneck_type: BottleneckType::Processing,
                        severity: BottleneckSeverity::Medium,
                        impact_percentage: step_duration as f64 / workflow.steps.len() as f64,
                        description: format!("Step took {} minutes to complete", step_duration),
                        optimization_suggestion: "Consider optimizing step logic or adding parallelization".to_string(),
                    });
                }
            }
        }

        // Check for overall workflow issues
        if workflow.steps.len() > 20 {
            bottlenecks.push(PerformanceBottleneck {
                step_index: 0,
                step_type: "workflow".to_string(),
                bottleneck_type: BottleneckType::Processing,
                severity: BottleneckSeverity::Low,
                impact_percentage: 10.0,
                description: "Workflow has many steps which may impact performance".to_string(),
                optimization_suggestion: "Consider breaking down into smaller workflows or adding parallelization".to_string(),
            });
        }

        Ok(bottlenecks)
    }

    /// Calculate performance grade based on metrics
    fn calculate_performance_grade(&self, execution_time: f64, success_rate: f64, efficiency_score: f64) -> PerformanceGrade {
        // Weighted scoring system
        let time_score = if execution_time <= 5.0 { 1.0 } else if execution_time <= 15.0 { 0.8 } else if execution_time <= 30.0 { 0.6 } else if execution_time <= 60.0 { 0.4 } else { 0.2 };
        let success_score = success_rate;
        let efficiency_normalized = efficiency_score.min(2.0) / 2.0; // Normalize to 0-1

        let overall_score = (time_score * 0.3 + success_score * 0.5 + efficiency_normalized * 0.2);

        match overall_score {
            s if s >= 0.95 => PerformanceGrade::Excellent,
            s if s >= 0.85 => PerformanceGrade::VeryGood,
            s if s >= 0.75 => PerformanceGrade::Good,
            s if s >= 0.60 => PerformanceGrade::Average,
            s if s >= 0.40 => PerformanceGrade::BelowAverage,
            _ => PerformanceGrade::Poor,
        }
    }

    /// Calculate aggregate performance metrics
    async fn calculate_aggregate_metrics(&self, workflow_metrics: &[PerformanceMetrics]) -> AppResult<AggregatePerformanceMetrics> {
        if workflow_metrics.is_empty() {
            return Ok(AggregatePerformanceMetrics {
                total_workflows: 0,
                average_execution_time: 0.0,
                median_execution_time: 0.0,
                fastest_execution_time: 0.0,
                slowest_execution_time: 0.0,
                overall_success_rate: 0.0,
                average_throughput: 0.0,
                resource_efficiency: 0.0,
                performance_distribution: PerformanceDistribution {
                    excellent_count: 0,
                    very_good_count: 0,
                    good_count: 0,
                    average_count: 0,
                    below_average_count: 0,
                    poor_count: 0,
                },
            });
        }

        let execution_times: Vec<f64> = workflow_metrics.iter().map(|m| m.execution_time_minutes).collect();
        let mut sorted_times = execution_times.clone();
        sorted_times.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let average_execution_time = execution_times.iter().sum::<f64>() / execution_times.len() as f64;
        let median_execution_time = if sorted_times.len() % 2 == 0 {
            (sorted_times[sorted_times.len() / 2 - 1] + sorted_times[sorted_times.len() / 2]) / 2.0
        } else {
            sorted_times[sorted_times.len() / 2]
        };

        let fastest_execution_time = sorted_times.first().copied().unwrap_or(0.0);
        let slowest_execution_time = sorted_times.last().copied().unwrap_or(0.0);

        let overall_success_rate = workflow_metrics.iter().map(|m| m.success_rate).sum::<f64>() / workflow_metrics.len() as f64;
        let average_throughput = workflow_metrics.iter().map(|m| m.throughput_score).sum::<f64>() / workflow_metrics.len() as f64;
        let resource_efficiency = workflow_metrics.iter().map(|m| m.efficiency_score).sum::<f64>() / workflow_metrics.len() as f64;

        // Calculate performance distribution
        let mut distribution = PerformanceDistribution {
            excellent_count: 0,
            very_good_count: 0,
            good_count: 0,
            average_count: 0,
            below_average_count: 0,
            poor_count: 0,
        };

        for metrics in workflow_metrics {
            match metrics.performance_grade {
                PerformanceGrade::Excellent => distribution.excellent_count += 1,
                PerformanceGrade::VeryGood => distribution.very_good_count += 1,
                PerformanceGrade::Good => distribution.good_count += 1,
                PerformanceGrade::Average => distribution.average_count += 1,
                PerformanceGrade::BelowAverage => distribution.below_average_count += 1,
                PerformanceGrade::Poor => distribution.poor_count += 1,
            }
        }

        Ok(AggregatePerformanceMetrics {
            total_workflows: workflow_metrics.len(),
            average_execution_time,
            median_execution_time,
            fastest_execution_time,
            slowest_execution_time,
            overall_success_rate,
            average_throughput,
            resource_efficiency,
            performance_distribution: distribution,
        })
    }

    /// Create performance ranking of workflows
    async fn create_performance_ranking(&self, workflow_metrics: &[PerformanceMetrics]) -> AppResult<Vec<Uuid>> {
        let mut ranked_metrics = workflow_metrics.to_vec();

        // Sort by efficiency score (higher is better)
        ranked_metrics.sort_by(|a, b| b.efficiency_score.partial_cmp(&a.efficiency_score).unwrap());

        Ok(ranked_metrics.iter().map(|m| m.workflow_id).collect())
    }

    /// Generate benchmark insights
    async fn generate_benchmark_insights(
        &self,
        workflow_metrics: &[PerformanceMetrics],
        aggregate_metrics: &AggregatePerformanceMetrics,
    ) -> AppResult<Vec<BenchmarkInsight>> {
        let mut insights = Vec::new();

        // Performance distribution insight
        if aggregate_metrics.performance_distribution.poor_count > 0 {
            insights.push(BenchmarkInsight {
                insight_type: BenchmarkInsightType::PerformancePattern,
                title: "Poor Performance Detected".to_string(),
                description: format!("{} workflows showing poor performance", aggregate_metrics.performance_distribution.poor_count),
                impact_level: InsightImpactLevel::High,
                confidence: 0.9,
                affected_workflows: workflow_metrics.iter()
                    .filter(|m| matches!(m.performance_grade, PerformanceGrade::Poor))
                    .map(|m| m.workflow_id)
                    .collect(),
                supporting_data: HashMap::from([
                    ("poor_count".to_string(), aggregate_metrics.performance_distribution.poor_count as f64),
                    ("total_workflows".to_string(), aggregate_metrics.total_workflows as f64),
                ]),
            });
        }

        // Resource efficiency insight
        if aggregate_metrics.resource_efficiency < 0.5 {
            insights.push(BenchmarkInsight {
                insight_type: BenchmarkInsightType::EfficiencyOpportunity,
                title: "Low Resource Efficiency".to_string(),
                description: "Overall resource efficiency is below optimal levels".to_string(),
                impact_level: InsightImpactLevel::Medium,
                confidence: 0.8,
                affected_workflows: workflow_metrics.iter().map(|m| m.workflow_id).collect(),
                supporting_data: HashMap::from([
                    ("resource_efficiency".to_string(), aggregate_metrics.resource_efficiency),
                    ("threshold".to_string(), 0.5),
                ]),
            });
        }

        // Execution time variance insight
        let time_variance = self.calculate_variance(&workflow_metrics.iter().map(|m| m.execution_time_minutes).collect::<Vec<_>>());
        if time_variance > 100.0 { // High variance in execution times
            insights.push(BenchmarkInsight {
                insight_type: BenchmarkInsightType::PerformancePattern,
                title: "High Execution Time Variance".to_string(),
                description: "Significant variation in workflow execution times detected".to_string(),
                impact_level: InsightImpactLevel::Medium,
                confidence: 0.85,
                affected_workflows: workflow_metrics.iter().map(|m| m.workflow_id).collect(),
                supporting_data: HashMap::from([
                    ("time_variance".to_string(), time_variance),
                    ("average_time".to_string(), aggregate_metrics.average_execution_time),
                ]),
            });
        }

        Ok(insights)
    }

    /// Generate optimization recommendations
    async fn generate_optimization_recommendations(
        &self,
        workflow_metrics: &[PerformanceMetrics],
        insights: &[BenchmarkInsight],
    ) -> AppResult<Vec<OptimizationRecommendation>> {
        let mut recommendations = Vec::new();

        // Recommendations based on insights
        for insight in insights {
            match insight.insight_type {
                BenchmarkInsightType::PerformancePattern => {
                    if insight.title.contains("Poor Performance") {
                        recommendations.push(OptimizationRecommendation {
                            recommendation_id: Uuid::new_v4(),
                            optimization_type: OptimizationType::ProcessOptimization,
                            title: "Optimize Poor Performing Workflows".to_string(),
                            description: "Review and optimize workflows with poor performance grades".to_string(),
                            priority: OptimizationPriority::High,
                            expected_improvement: ExpectedImprovement {
                                performance_gain_percentage: 30.0,
                                resource_savings_percentage: 20.0,
                                reliability_improvement: 0.15,
                                cost_reduction_percentage: 15.0,
                            },
                            implementation_complexity: ImplementationComplexity::Moderate,
                            implementation_steps: vec![
                                "Identify bottlenecks in poor performing workflows".to_string(),
                                "Optimize resource allocation and step logic".to_string(),
                                "Implement performance monitoring".to_string(),
                                "Test and validate improvements".to_string(),
                            ],
                            affected_workflows: insight.affected_workflows.clone(),
                            estimated_effort_hours: 16.0,
                        });
                    }
                }
                BenchmarkInsightType::EfficiencyOpportunity => {
                    recommendations.push(OptimizationRecommendation {
                        recommendation_id: Uuid::new_v4(),
                        optimization_type: OptimizationType::ResourceOptimization,
                        title: "Improve Resource Efficiency".to_string(),
                        description: "Optimize resource utilization across all workflows".to_string(),
                        priority: OptimizationPriority::Medium,
                        expected_improvement: ExpectedImprovement {
                            performance_gain_percentage: 20.0,
                            resource_savings_percentage: 35.0,
                            reliability_improvement: 0.10,
                            cost_reduction_percentage: 25.0,
                        },
                        implementation_complexity: ImplementationComplexity::Complex,
                        implementation_steps: vec![
                            "Analyze current resource usage patterns".to_string(),
                            "Implement resource pooling and caching".to_string(),
                            "Optimize API call patterns".to_string(),
                            "Monitor and adjust resource allocation".to_string(),
                        ],
                        affected_workflows: insight.affected_workflows.clone(),
                        estimated_effort_hours: 40.0,
                    });
                }
                _ => {}
            }
        }

        // General recommendations based on metrics
        let high_bottleneck_workflows: Vec<Uuid> = workflow_metrics.iter()
            .filter(|m| m.bottlenecks.len() > 2)
            .map(|m| m.workflow_id)
            .collect();

        if !high_bottleneck_workflows.is_empty() {
            recommendations.push(OptimizationRecommendation {
                recommendation_id: Uuid::new_v4(),
                optimization_type: OptimizationType::Parallelization,
                title: "Implement Parallelization".to_string(),
                description: "Add parallel processing to workflows with multiple bottlenecks".to_string(),
                priority: OptimizationPriority::Medium,
                expected_improvement: ExpectedImprovement {
                    performance_gain_percentage: 40.0,
                    resource_savings_percentage: 10.0,
                    reliability_improvement: 0.05,
                    cost_reduction_percentage: 5.0,
                },
                implementation_complexity: ImplementationComplexity::Complex,
                implementation_steps: vec![
                    "Identify parallelizable workflow steps".to_string(),
                    "Design parallel execution architecture".to_string(),
                    "Implement parallel processing logic".to_string(),
                    "Test and validate parallel execution".to_string(),
                ],
                affected_workflows: high_bottleneck_workflows,
                estimated_effort_hours: 60.0,
            });
        }

        Ok(recommendations)
    }

    /// Analyze performance trends over time
    async fn analyze_performance_trends(&self, workflows: &[ResearchWorkflow]) -> AppResult<PerformanceTrends> {
        // For now, return stable trends as we don't have historical data
        // In a real implementation, this would analyze trends over time
        Ok(PerformanceTrends {
            execution_time_trend: TrendDirection::Stable,
            success_rate_trend: TrendDirection::Improving,
            resource_usage_trend: TrendDirection::Stable,
            throughput_trend: TrendDirection::Stable,
            trend_confidence: 0.6,
            trend_analysis_period_days: 30,
        })
    }

    /// Calculate variance of a dataset
    fn calculate_variance(&self, data: &[f64]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }

        let mean = data.iter().sum::<f64>() / data.len() as f64;
        let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
        variance
    }
}