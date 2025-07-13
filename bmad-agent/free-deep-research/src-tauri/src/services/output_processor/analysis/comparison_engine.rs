use std::collections::HashMap;
use tracing::{info, debug, warn};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, ResearchError};
use crate::models::research_workflow::{ResearchWorkflow, StepStatus, WorkflowStatus};

/// Comparison engine for analyzing workflow differences
pub struct ComparisonEngine;

/// Comparison request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonRequest {
    pub id: Uuid,
    pub workflow_ids: Vec<Uuid>,
    pub comparison_type: ComparisonType,
    pub options: ComparisonOptions,
}

/// Types of comparisons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonType {
    SideBySide,
    Comprehensive,
    Performance,
    Quality,
    Timeline,
    Results,
}

/// Comparison options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonOptions {
    pub include_step_details: bool,
    pub include_performance_metrics: bool,
    pub include_result_analysis: bool,
    pub similarity_threshold: f64,
    pub highlight_differences: bool,
}

impl Default for ComparisonOptions {
    fn default() -> Self {
        Self {
            include_step_details: true,
            include_performance_metrics: true,
            include_result_analysis: true,
            similarity_threshold: 0.8,
            highlight_differences: true,
        }
    }
}

/// Comparison result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    pub id: Uuid,
    pub request_id: Uuid,
    pub workflow_ids: Vec<Uuid>,
    pub comparison_type: ComparisonType,
    pub overall_similarity: f64,
    pub differences: Vec<WorkflowDifference>,
    pub similarities: Vec<WorkflowSimilarity>,
    pub performance_comparison: PerformanceComparison,
    pub quality_comparison: QualityComparison,
    pub summary: ComparisonSummary,
    pub created_at: DateTime<Utc>,
    pub processing_time_ms: u64,
}

/// Workflow difference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDifference {
    pub difference_type: DifferenceType,
    pub field: String,
    pub workflow_values: HashMap<Uuid, serde_json::Value>,
    pub significance: DifferenceSignificance,
    pub description: String,
}

/// Types of differences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DifferenceType {
    Structural,
    Performance,
    Quality,
    Content,
    Timing,
    Configuration,
}

/// Significance levels for differences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DifferenceSignificance {
    Critical,
    Major,
    Minor,
    Negligible,
}

/// Workflow similarity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowSimilarity {
    pub similarity_type: SimilarityType,
    pub field: String,
    pub similarity_score: f64,
    pub description: String,
}

/// Types of similarities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimilarityType {
    Structural,
    Performance,
    Quality,
    Content,
    Pattern,
}

/// Performance comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceComparison {
    pub execution_times: HashMap<Uuid, u64>,
    pub step_counts: HashMap<Uuid, usize>,
    pub success_rates: HashMap<Uuid, f64>,
    pub resource_usage: HashMap<Uuid, ResourceUsageMetrics>,
    pub performance_ranking: Vec<Uuid>,
    pub bottlenecks: Vec<PerformanceBottleneck>,
}

/// Resource usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageMetrics {
    pub memory_peak_mb: u64,
    pub cpu_average_percent: f64,
    pub api_calls_total: u32,
    pub bandwidth_mb: f64,
}

/// Performance bottleneck
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBottleneck {
    pub workflow_id: Uuid,
    pub step_index: usize,
    pub bottleneck_type: BottleneckType,
    pub impact_severity: ImpactSeverity,
    pub description: String,
    pub suggested_fix: String,
}

/// Types of bottlenecks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BottleneckType {
    Memory,
    CPU,
    Network,
    API,
    Processing,
    IO,
}

/// Impact severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactSeverity {
    High,
    Medium,
    Low,
}

/// Quality comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityComparison {
    pub quality_scores: HashMap<Uuid, f64>,
    pub result_completeness: HashMap<Uuid, f64>,
    pub source_quality: HashMap<Uuid, f64>,
    pub accuracy_metrics: HashMap<Uuid, AccuracyMetrics>,
    pub quality_ranking: Vec<Uuid>,
}

/// Accuracy metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccuracyMetrics {
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub confidence: f64,
}

/// Comparison summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonSummary {
    pub total_workflows: usize,
    pub major_differences: usize,
    pub minor_differences: usize,
    pub key_insights: Vec<String>,
    pub recommendations: Vec<String>,
    pub best_performing_workflow: Option<Uuid>,
    pub highest_quality_workflow: Option<Uuid>,
}

impl ComparisonEngine {
    /// Create a new comparison engine
    pub async fn new() -> AppResult<Self> {
        info!("Initializing comparison engine...");
        Ok(Self)
    }

    /// Compare multiple workflows
    pub async fn compare_workflows(
        &self,
        workflows: &[ResearchWorkflow],
        request: ComparisonRequest,
    ) -> AppResult<ComparisonResult> {
        info!("Comparing {} workflows", workflows.len());

        let start_time = std::time::Instant::now();

        // Filter workflows to only those requested
        let target_workflows: Vec<&ResearchWorkflow> = workflows.iter()
            .filter(|w| request.workflow_ids.contains(&w.id))
            .collect();

        if target_workflows.len() < 2 {
            return Err(ResearchError::invalid_request(
                "At least 2 workflows are required for comparison".to_string()
            ).into());
        }

        // Perform different types of comparison based on request
        let differences = self.identify_differences(&target_workflows, &request.options).await?;
        let similarities = self.identify_similarities(&target_workflows, &request.options).await?;
        let performance_comparison = self.compare_performance(&target_workflows).await?;
        let quality_comparison = self.compare_quality(&target_workflows).await?;
        let overall_similarity = self.calculate_overall_similarity(&similarities, &differences).await?;
        let summary = self.generate_summary(&target_workflows, &differences, &similarities, &performance_comparison, &quality_comparison).await?;

        let processing_time = start_time.elapsed();

        Ok(ComparisonResult {
            id: Uuid::new_v4(),
            request_id: request.id,
            workflow_ids: request.workflow_ids,
            comparison_type: request.comparison_type,
            overall_similarity,
            differences,
            similarities,
            performance_comparison,
            quality_comparison,
            summary,
            created_at: Utc::now(),
            processing_time_ms: processing_time.as_millis() as u64,
        })
    }

    /// Identify differences between workflows
    async fn identify_differences(
        &self,
        workflows: &[&ResearchWorkflow],
        _options: &ComparisonOptions,
    ) -> AppResult<Vec<WorkflowDifference>> {
        let mut differences = Vec::new();

        // Compare workflow statuses
        let statuses: HashMap<Uuid, WorkflowStatus> = workflows.iter()
            .map(|w| (w.id, w.status))
            .collect();

        if statuses.values().collect::<std::collections::HashSet<_>>().len() > 1 {
            let workflow_values: HashMap<Uuid, serde_json::Value> = statuses.iter()
                .map(|(id, status)| (*id, serde_json::json!(format!("{:?}", status))))
                .collect();

            differences.push(WorkflowDifference {
                difference_type: DifferenceType::Structural,
                field: "status".to_string(),
                workflow_values,
                significance: DifferenceSignificance::Major,
                description: "Workflows have different completion statuses".to_string(),
            });
        }

        // Compare step counts
        let step_counts: HashMap<Uuid, usize> = workflows.iter()
            .map(|w| (w.id, w.steps.len()))
            .collect();

        if step_counts.values().collect::<std::collections::HashSet<_>>().len() > 1 {
            let workflow_values: HashMap<Uuid, serde_json::Value> = step_counts.iter()
                .map(|(id, count)| (*id, serde_json::json!(count)))
                .collect();

            differences.push(WorkflowDifference {
                difference_type: DifferenceType::Structural,
                field: "step_count".to_string(),
                workflow_values,
                significance: DifferenceSignificance::Minor,
                description: "Workflows have different numbers of steps".to_string(),
            });
        }

        // Compare execution times
        let execution_times: HashMap<Uuid, Option<i64>> = workflows.iter()
            .map(|w| {
                let duration = if let (Some(started), Some(completed)) = (w.started_at, w.completed_at) {
                    Some((completed - started).num_minutes())
                } else {
                    None
                };
                (w.id, duration)
            })
            .collect();

        let completed_times: Vec<i64> = execution_times.values().filter_map(|&t| t).collect();
        if completed_times.len() > 1 {
            let min_time = completed_times.iter().min().unwrap();
            let max_time = completed_times.iter().max().unwrap();
            
            if max_time - min_time > 5 { // More than 5 minutes difference
                let workflow_values: HashMap<Uuid, serde_json::Value> = execution_times.iter()
                    .map(|(id, time)| (*id, serde_json::json!(time)))
                    .collect();

                differences.push(WorkflowDifference {
                    difference_type: DifferenceType::Performance,
                    field: "execution_time".to_string(),
                    workflow_values,
                    significance: DifferenceSignificance::Major,
                    description: "Significant differences in execution time detected".to_string(),
                });
            }
        }

        Ok(differences)
    }

    /// Identify similarities between workflows
    async fn identify_similarities(
        &self,
        workflows: &[&ResearchWorkflow],
        _options: &ComparisonOptions,
    ) -> AppResult<Vec<WorkflowSimilarity>> {
        let mut similarities = Vec::new();

        // Check for similar query patterns
        let queries: Vec<&str> = workflows.iter().map(|w| w.query.as_str()).collect();
        let query_similarity = self.calculate_text_similarity(&queries);
        
        if query_similarity > 0.7 {
            similarities.push(WorkflowSimilarity {
                similarity_type: SimilarityType::Content,
                field: "query".to_string(),
                similarity_score: query_similarity,
                description: "Workflows have similar research queries".to_string(),
            });
        }

        // Check for similar step patterns
        let step_types: Vec<Vec<&str>> = workflows.iter()
            .map(|w| w.steps.iter().map(|s| s.step_type.as_str()).collect())
            .collect();

        if step_types.len() > 1 {
            let pattern_similarity = self.calculate_pattern_similarity(&step_types);
            if pattern_similarity > 0.6 {
                similarities.push(WorkflowSimilarity {
                    similarity_type: SimilarityType::Structural,
                    field: "step_pattern".to_string(),
                    similarity_score: pattern_similarity,
                    description: "Workflows follow similar step patterns".to_string(),
                });
            }
        }

        Ok(similarities)
    }

    /// Compare performance metrics
    async fn compare_performance(&self, workflows: &[&ResearchWorkflow]) -> AppResult<PerformanceComparison> {
        let mut execution_times = HashMap::new();
        let mut step_counts = HashMap::new();
        let mut success_rates = HashMap::new();
        let mut resource_usage = HashMap::new();

        for workflow in workflows {
            // Execution time
            if let (Some(started), Some(completed)) = (workflow.started_at, workflow.completed_at) {
                execution_times.insert(workflow.id, (completed - started).num_minutes() as u64);
            }

            // Step count
            step_counts.insert(workflow.id, workflow.steps.len());

            // Success rate
            let completed_steps = workflow.steps.iter().filter(|s| s.status == StepStatus::Completed).count();
            let success_rate = if workflow.steps.is_empty() {
                0.0
            } else {
                completed_steps as f64 / workflow.steps.len() as f64
            };
            success_rates.insert(workflow.id, success_rate);

            // Resource usage (placeholder)
            resource_usage.insert(workflow.id, ResourceUsageMetrics {
                memory_peak_mb: 100, // Placeholder
                cpu_average_percent: 25.0, // Placeholder
                api_calls_total: workflow.steps.len() as u32 * 2, // Estimate
                bandwidth_mb: 10.0, // Placeholder
            });
        }

        // Performance ranking based on execution time and success rate
        let mut performance_ranking: Vec<Uuid> = workflows.iter().map(|w| w.id).collect();
        performance_ranking.sort_by(|a, b| {
            let score_a = success_rates.get(a).unwrap_or(&0.0) - (execution_times.get(a).unwrap_or(&0) as f64 / 100.0);
            let score_b = success_rates.get(b).unwrap_or(&0.0) - (execution_times.get(b).unwrap_or(&0) as f64 / 100.0);
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(PerformanceComparison {
            execution_times,
            step_counts,
            success_rates,
            resource_usage,
            performance_ranking,
            bottlenecks: Vec::new(), // TODO: Implement bottleneck detection
        })
    }

    /// Compare quality metrics
    async fn compare_quality(&self, workflows: &[&ResearchWorkflow]) -> AppResult<QualityComparison> {
        let mut quality_scores = HashMap::new();
        let mut result_completeness = HashMap::new();
        let mut source_quality = HashMap::new();
        let mut accuracy_metrics = HashMap::new();

        for workflow in workflows {
            // Quality score based on completion and results
            let quality_score = if workflow.status == WorkflowStatus::Completed {
                if let Some(results) = &workflow.results {
                    let completeness = if results.summary.is_empty() { 0.0 } else { 0.5 }
                        + if results.key_findings.is_empty() { 0.0 } else { 0.3 }
                        + if results.sources.is_empty() { 0.0 } else { 0.2 };
                    completeness * 100.0
                } else {
                    50.0 // Completed but no results
                }
            } else {
                0.0 // Not completed
            };

            quality_scores.insert(workflow.id, quality_score);

            // Result completeness
            let completeness = if let Some(results) = &workflow.results {
                let fields_present = [
                    !results.summary.is_empty(),
                    !results.key_findings.is_empty(),
                    !results.sources.is_empty(),
                ].iter().filter(|&&x| x).count();
                fields_present as f64 / 3.0 * 100.0
            } else {
                0.0
            };
            result_completeness.insert(workflow.id, completeness);

            // Source quality (based on number and diversity)
            let source_score = if let Some(results) = &workflow.results {
                (results.sources.len().min(10) as f64 / 10.0) * 100.0
            } else {
                0.0
            };
            source_quality.insert(workflow.id, source_score);

            // Accuracy metrics (placeholder)
            accuracy_metrics.insert(workflow.id, AccuracyMetrics {
                precision: 0.85,
                recall: 0.80,
                f1_score: 0.82,
                confidence: 0.88,
            });
        }

        // Quality ranking
        let mut quality_ranking: Vec<Uuid> = workflows.iter().map(|w| w.id).collect();
        quality_ranking.sort_by(|a, b| {
            let score_a = quality_scores.get(a).unwrap_or(&0.0);
            let score_b = quality_scores.get(b).unwrap_or(&0.0);
            score_b.partial_cmp(score_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(QualityComparison {
            quality_scores,
            result_completeness,
            source_quality,
            accuracy_metrics,
            quality_ranking,
        })
    }

    /// Calculate overall similarity score
    async fn calculate_overall_similarity(
        &self,
        similarities: &[WorkflowSimilarity],
        differences: &[WorkflowDifference],
    ) -> AppResult<f64> {
        if similarities.is_empty() && differences.is_empty() {
            return Ok(0.0);
        }

        let similarity_score = if similarities.is_empty() {
            0.0
        } else {
            similarities.iter().map(|s| s.similarity_score).sum::<f64>() / similarities.len() as f64
        };

        let difference_penalty = differences.len() as f64 * 0.1;
        let overall_similarity = (similarity_score - difference_penalty).max(0.0).min(1.0);

        Ok(overall_similarity)
    }

    /// Generate comparison summary
    async fn generate_summary(
        &self,
        workflows: &[&ResearchWorkflow],
        differences: &[WorkflowDifference],
        similarities: &[WorkflowSimilarity],
        performance_comparison: &PerformanceComparison,
        quality_comparison: &QualityComparison,
    ) -> AppResult<ComparisonSummary> {
        let major_differences = differences.iter()
            .filter(|d| matches!(d.significance, DifferenceSignificance::Major | DifferenceSignificance::Critical))
            .count();

        let minor_differences = differences.iter()
            .filter(|d| matches!(d.significance, DifferenceSignificance::Minor))
            .count();

        let mut key_insights = Vec::new();
        let mut recommendations = Vec::new();

        // Generate insights based on analysis
        if major_differences > 0 {
            key_insights.push("Significant structural or performance differences detected".to_string());
            recommendations.push("Review workflow configurations for optimization opportunities".to_string());
        }

        if similarities.len() > 2 {
            key_insights.push("Strong similarities found in workflow patterns".to_string());
            recommendations.push("Consider creating templates based on successful patterns".to_string());
        }

        Ok(ComparisonSummary {
            total_workflows: workflows.len(),
            major_differences,
            minor_differences,
            key_insights,
            recommendations,
            best_performing_workflow: performance_comparison.performance_ranking.first().copied(),
            highest_quality_workflow: quality_comparison.quality_ranking.first().copied(),
        })
    }

    /// Calculate text similarity (simplified)
    fn calculate_text_similarity(&self, texts: &[&str]) -> f64 {
        if texts.len() < 2 {
            return 0.0;
        }

        // Simple word overlap similarity
        let words: Vec<Vec<&str>> = texts.iter()
            .map(|text| text.split_whitespace().collect())
            .collect();

        let mut total_similarity = 0.0;
        let mut comparisons = 0;

        for i in 0..words.len() {
            for j in i+1..words.len() {
                let common_words = words[i].iter()
                    .filter(|word| words[j].contains(word))
                    .count();
                
                let total_words = (words[i].len() + words[j].len()) / 2;
                let similarity = if total_words > 0 {
                    common_words as f64 / total_words as f64
                } else {
                    0.0
                };

                total_similarity += similarity;
                comparisons += 1;
            }
        }

        if comparisons > 0 {
            total_similarity / comparisons as f64
        } else {
            0.0
        }
    }

    /// Calculate pattern similarity (simplified)
    fn calculate_pattern_similarity(&self, patterns: &[Vec<&str>]) -> f64 {
        if patterns.len() < 2 {
            return 0.0;
        }

        // Simple sequence similarity
        let mut total_similarity = 0.0;
        let mut comparisons = 0;

        for i in 0..patterns.len() {
            for j in i+1..patterns.len() {
                let common_elements = patterns[i].iter()
                    .filter(|element| patterns[j].contains(element))
                    .count();
                
                let max_length = patterns[i].len().max(patterns[j].len());
                let similarity = if max_length > 0 {
                    common_elements as f64 / max_length as f64
                } else {
                    0.0
                };

                total_similarity += similarity;
                comparisons += 1;
            }
        }

        if comparisons > 0 {
            total_similarity / comparisons as f64
        } else {
            0.0
        }
    }
}
