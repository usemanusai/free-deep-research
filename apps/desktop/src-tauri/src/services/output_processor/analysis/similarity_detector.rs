use std::collections::HashMap;
use tracing::{info, debug};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, ResearchError};
use crate::models::research_workflow::{ResearchWorkflow, StepStatus, WorkflowStatus};

/// Similarity detector for workflow clustering and pattern recognition
pub struct SimilarityDetector;

/// Similarity score between two workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityScore {
    pub workflow_a: Uuid,
    pub workflow_b: Uuid,
    pub overall_similarity: f64,
    pub structural_similarity: f64,
    pub content_similarity: f64,
    pub performance_similarity: f64,
    pub quality_similarity: f64,
    pub similarity_factors: Vec<SimilarityFactor>,
}

/// Similarity factor contributing to overall score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityFactor {
    pub factor_name: String,
    pub weight: f64,
    pub score: f64,
    pub description: String,
}

/// Cluster result from similarity analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterResult {
    pub id: Uuid,
    pub clusters: Vec<WorkflowCluster>,
    pub outliers: Vec<Uuid>,
    pub similarity_matrix: SimilarityMatrix,
    pub clustering_method: ClusteringMethod,
    pub cluster_quality_metrics: ClusterQualityMetrics,
    pub created_at: DateTime<Utc>,
    pub processing_time_ms: u64,
}

/// Workflow cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowCluster {
    pub cluster_id: u32,
    pub workflow_ids: Vec<Uuid>,
    pub centroid: ClusterCentroid,
    pub intra_cluster_similarity: f64,
    pub cluster_characteristics: ClusterCharacteristics,
    pub representative_workflow: Option<Uuid>,
}

/// Cluster centroid (representative point)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterCentroid {
    pub average_execution_time: f64,
    pub average_step_count: f64,
    pub average_success_rate: f64,
    pub common_step_types: Vec<String>,
    pub typical_query_patterns: Vec<String>,
}

/// Cluster characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterCharacteristics {
    pub cluster_type: ClusterType,
    pub dominant_patterns: Vec<String>,
    pub performance_profile: PerformanceProfile,
    pub quality_profile: QualityProfile,
    pub temporal_patterns: TemporalPatterns,
}

/// Types of clusters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClusterType {
    HighPerformance,
    HighQuality,
    FastExecution,
    ComplexWorkflows,
    SimpleWorkflows,
    FailureProne,
    Experimental,
    Production,
}

/// Performance profile of a cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceProfile {
    pub performance_tier: PerformanceTier,
    pub consistency_score: f64,
    pub optimization_potential: f64,
    pub bottleneck_patterns: Vec<String>,
}

/// Performance tiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceTier {
    Excellent,
    Good,
    Average,
    BelowAverage,
    Poor,
}

/// Quality profile of a cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityProfile {
    pub quality_tier: QualityTier,
    pub consistency_score: f64,
    pub improvement_areas: Vec<String>,
    pub strength_areas: Vec<String>,
}

/// Quality tiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityTier {
    Excellent,
    Good,
    Average,
    BelowAverage,
    Poor,
}

/// Temporal patterns in a cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalPatterns {
    pub peak_usage_hours: Vec<u32>,
    pub seasonal_trends: Vec<String>,
    pub execution_time_patterns: Vec<String>,
}

/// Similarity matrix for all workflow pairs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityMatrix {
    pub workflow_ids: Vec<Uuid>,
    pub similarity_scores: Vec<Vec<f64>>,
    pub average_similarity: f64,
    pub max_similarity: f64,
    pub min_similarity: f64,
}

/// Clustering methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClusteringMethod {
    KMeans,
    Hierarchical,
    DBSCAN,
    Spectral,
    Custom,
}

/// Cluster quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterQualityMetrics {
    pub silhouette_score: f64,
    pub davies_bouldin_index: f64,
    pub calinski_harabasz_index: f64,
    pub inertia: f64,
    pub cluster_stability: f64,
}

/// Similarity detection options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityOptions {
    pub similarity_threshold: f64,
    pub clustering_method: ClusteringMethod,
    pub max_clusters: Option<u32>,
    pub include_outlier_detection: bool,
    pub weight_factors: SimilarityWeights,
}

/// Weights for different similarity factors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityWeights {
    pub structural_weight: f64,
    pub content_weight: f64,
    pub performance_weight: f64,
    pub quality_weight: f64,
    pub temporal_weight: f64,
}

impl Default for SimilarityWeights {
    fn default() -> Self {
        Self {
            structural_weight: 0.3,
            content_weight: 0.3,
            performance_weight: 0.2,
            quality_weight: 0.15,
            temporal_weight: 0.05,
        }
    }
}

impl Default for SimilarityOptions {
    fn default() -> Self {
        Self {
            similarity_threshold: 0.7,
            clustering_method: ClusteringMethod::KMeans,
            max_clusters: Some(5),
            include_outlier_detection: true,
            weight_factors: SimilarityWeights::default(),
        }
    }
}

impl SimilarityDetector {
    /// Create a new similarity detector
    pub async fn new() -> AppResult<Self> {
        info!("Initializing similarity detector...");
        Ok(Self)
    }

    /// Detect similarity and cluster workflows
    pub async fn detect_similarity(&self, workflows: &[ResearchWorkflow]) -> AppResult<ClusterResult> {
        self.detect_similarity_with_options(workflows, SimilarityOptions::default()).await
    }

    /// Detect similarity with custom options
    pub async fn detect_similarity_with_options(
        &self,
        workflows: &[ResearchWorkflow],
        options: SimilarityOptions,
    ) -> AppResult<ClusterResult> {
        info!("Detecting similarity among {} workflows", workflows.len());

        let start_time = std::time::Instant::now();

        if workflows.len() < 2 {
            return Err(ResearchError::invalid_request(
                "At least 2 workflows are required for similarity detection".to_string()
            ).into());
        }

        // Calculate similarity matrix
        let similarity_matrix = self.calculate_similarity_matrix(workflows, &options).await?;

        // Perform clustering
        let clusters = self.perform_clustering(workflows, &similarity_matrix, &options).await?;

        // Detect outliers
        let outliers = if options.include_outlier_detection {
            self.detect_outliers(workflows, &similarity_matrix, &options).await?
        } else {
            Vec::new()
        };

        // Calculate cluster quality metrics
        let cluster_quality_metrics = self.calculate_cluster_quality(&clusters, &similarity_matrix).await?;

        let processing_time = start_time.elapsed();

        Ok(ClusterResult {
            id: Uuid::new_v4(),
            clusters,
            outliers,
            similarity_matrix,
            clustering_method: options.clustering_method,
            cluster_quality_metrics,
            created_at: Utc::now(),
            processing_time_ms: processing_time.as_millis() as u64,
        })
    }

    /// Calculate pairwise similarity scores
    pub async fn calculate_pairwise_similarity(
        &self,
        workflow_a: &ResearchWorkflow,
        workflow_b: &ResearchWorkflow,
    ) -> AppResult<SimilarityScore> {
        let options = SimilarityOptions::default();
        self.calculate_pairwise_similarity_with_options(workflow_a, workflow_b, &options).await
    }

    /// Calculate pairwise similarity with custom options
    pub async fn calculate_pairwise_similarity_with_options(
        &self,
        workflow_a: &ResearchWorkflow,
        workflow_b: &ResearchWorkflow,
        options: &SimilarityOptions,
    ) -> AppResult<SimilarityScore> {
        let structural_similarity = self.calculate_structural_similarity(workflow_a, workflow_b).await?;
        let content_similarity = self.calculate_content_similarity(workflow_a, workflow_b).await?;
        let performance_similarity = self.calculate_performance_similarity(workflow_a, workflow_b).await?;
        let quality_similarity = self.calculate_quality_similarity(workflow_a, workflow_b).await?;

        // Calculate weighted overall similarity
        let overall_similarity = 
            structural_similarity * options.weight_factors.structural_weight +
            content_similarity * options.weight_factors.content_weight +
            performance_similarity * options.weight_factors.performance_weight +
            quality_similarity * options.weight_factors.quality_weight;

        let similarity_factors = vec![
            SimilarityFactor {
                factor_name: "Structural".to_string(),
                weight: options.weight_factors.structural_weight,
                score: structural_similarity,
                description: "Similarity in workflow structure and step patterns".to_string(),
            },
            SimilarityFactor {
                factor_name: "Content".to_string(),
                weight: options.weight_factors.content_weight,
                score: content_similarity,
                description: "Similarity in queries and content patterns".to_string(),
            },
            SimilarityFactor {
                factor_name: "Performance".to_string(),
                weight: options.weight_factors.performance_weight,
                score: performance_similarity,
                description: "Similarity in execution performance metrics".to_string(),
            },
            SimilarityFactor {
                factor_name: "Quality".to_string(),
                weight: options.weight_factors.quality_weight,
                score: quality_similarity,
                description: "Similarity in result quality and completeness".to_string(),
            },
        ];

        Ok(SimilarityScore {
            workflow_a: workflow_a.id,
            workflow_b: workflow_b.id,
            overall_similarity,
            structural_similarity,
            content_similarity,
            performance_similarity,
            quality_similarity,
            similarity_factors,
        })
    }

    /// Calculate similarity matrix for all workflow pairs
    async fn calculate_similarity_matrix(
        &self,
        workflows: &[ResearchWorkflow],
        options: &SimilarityOptions,
    ) -> AppResult<SimilarityMatrix> {
        let n = workflows.len();
        let mut similarity_scores = vec![vec![0.0; n]; n];
        let workflow_ids: Vec<Uuid> = workflows.iter().map(|w| w.id).collect();

        let mut total_similarity = 0.0;
        let mut max_similarity = 0.0;
        let mut min_similarity = 1.0;
        let mut comparison_count = 0;

        for i in 0..n {
            for j in i..n {
                let similarity = if i == j {
                    1.0 // Self-similarity
                } else {
                    let score = self.calculate_pairwise_similarity_with_options(
                        &workflows[i],
                        &workflows[j],
                        options,
                    ).await?;
                    score.overall_similarity
                };

                similarity_scores[i][j] = similarity;
                similarity_scores[j][i] = similarity; // Symmetric matrix

                if i != j {
                    total_similarity += similarity;
                    max_similarity = max_similarity.max(similarity);
                    min_similarity = min_similarity.min(similarity);
                    comparison_count += 1;
                }
            }
        }

        let average_similarity = if comparison_count > 0 {
            total_similarity / comparison_count as f64
        } else {
            0.0
        };

        Ok(SimilarityMatrix {
            workflow_ids,
            similarity_scores,
            average_similarity,
            max_similarity,
            min_similarity: if comparison_count > 0 { min_similarity } else { 0.0 },
        })
    }

    /// Calculate structural similarity between workflows
    async fn calculate_structural_similarity(
        &self,
        workflow_a: &ResearchWorkflow,
        workflow_b: &ResearchWorkflow,
    ) -> AppResult<f64> {
        // Compare step counts
        let step_count_similarity = 1.0 - ((workflow_a.steps.len() as f64 - workflow_b.steps.len() as f64).abs() / 
            (workflow_a.steps.len().max(workflow_b.steps.len()) as f64).max(1.0));

        // Compare step types
        let step_types_a: Vec<&str> = workflow_a.steps.iter().map(|s| s.step_type.as_str()).collect();
        let step_types_b: Vec<&str> = workflow_b.steps.iter().map(|s| s.step_type.as_str()).collect();
        let step_type_similarity = self.calculate_sequence_similarity(&step_types_a, &step_types_b);

        // Compare status patterns
        let status_similarity = if workflow_a.status == workflow_b.status { 1.0 } else { 0.0 };

        // Weighted average
        Ok((step_count_similarity * 0.3 + step_type_similarity * 0.5 + status_similarity * 0.2))
    }

    /// Calculate content similarity between workflows
    async fn calculate_content_similarity(
        &self,
        workflow_a: &ResearchWorkflow,
        workflow_b: &ResearchWorkflow,
    ) -> AppResult<f64> {
        // Compare queries using simple word overlap
        let query_similarity = self.calculate_text_similarity(&workflow_a.query, &workflow_b.query);

        // Compare workflow names
        let name_similarity = self.calculate_text_similarity(&workflow_a.name, &workflow_b.name);

        // Weighted average
        Ok((query_similarity * 0.8 + name_similarity * 0.2))
    }

    /// Calculate performance similarity between workflows
    async fn calculate_performance_similarity(
        &self,
        workflow_a: &ResearchWorkflow,
        workflow_b: &ResearchWorkflow,
    ) -> AppResult<f64> {
        // Compare execution times
        let time_similarity = match (
            workflow_a.started_at.zip(workflow_a.completed_at),
            workflow_b.started_at.zip(workflow_b.completed_at),
        ) {
            (Some((start_a, end_a)), Some((start_b, end_b))) => {
                let duration_a = (end_a - start_a).num_minutes() as f64;
                let duration_b = (end_b - start_b).num_minutes() as f64;
                let max_duration = duration_a.max(duration_b).max(1.0);
                1.0 - (duration_a - duration_b).abs() / max_duration
            }
            _ => 0.5, // Unknown execution times
        };

        // Compare success rates
        let success_rate_a = if workflow_a.steps.is_empty() {
            0.0
        } else {
            workflow_a.steps.iter().filter(|s| s.status == StepStatus::Completed).count() as f64 / workflow_a.steps.len() as f64
        };

        let success_rate_b = if workflow_b.steps.is_empty() {
            0.0
        } else {
            workflow_b.steps.iter().filter(|s| s.status == StepStatus::Completed).count() as f64 / workflow_b.steps.len() as f64
        };

        let success_rate_similarity = 1.0 - (success_rate_a - success_rate_b).abs();

        // Weighted average
        Ok((time_similarity * 0.6 + success_rate_similarity * 0.4))
    }

    /// Calculate quality similarity between workflows
    async fn calculate_quality_similarity(
        &self,
        workflow_a: &ResearchWorkflow,
        workflow_b: &ResearchWorkflow,
    ) -> AppResult<f64> {
        // Compare result completeness
        let completeness_a = match &workflow_a.results {
            Some(results) => {
                let fields = [
                    !results.summary.is_empty(),
                    !results.key_findings.is_empty(),
                    !results.sources.is_empty(),
                ];
                fields.iter().filter(|&&x| x).count() as f64 / 3.0
            }
            None => 0.0,
        };

        let completeness_b = match &workflow_b.results {
            Some(results) => {
                let fields = [
                    !results.summary.is_empty(),
                    !results.key_findings.is_empty(),
                    !results.sources.is_empty(),
                ];
                fields.iter().filter(|&&x| x).count() as f64 / 3.0
            }
            None => 0.0,
        };

        let completeness_similarity = 1.0 - (completeness_a - completeness_b).abs();

        // Compare source counts
        let source_count_a = workflow_a.results.as_ref().map(|r| r.sources.len()).unwrap_or(0) as f64;
        let source_count_b = workflow_b.results.as_ref().map(|r| r.sources.len()).unwrap_or(0) as f64;
        let max_sources = source_count_a.max(source_count_b).max(1.0);
        let source_similarity = 1.0 - (source_count_a - source_count_b).abs() / max_sources;

        // Weighted average
        Ok((completeness_similarity * 0.7 + source_similarity * 0.3))
    }

    /// Perform clustering on workflows
    async fn perform_clustering(
        &self,
        workflows: &[ResearchWorkflow],
        similarity_matrix: &SimilarityMatrix,
        options: &SimilarityOptions,
    ) -> AppResult<Vec<WorkflowCluster>> {
        // Simple clustering based on similarity threshold
        let mut clusters = Vec::new();
        let mut assigned = vec![false; workflows.len()];
        let mut cluster_id = 0;

        for i in 0..workflows.len() {
            if assigned[i] {
                continue;
            }

            let mut cluster_workflows = vec![workflows[i].id];
            assigned[i] = true;

            // Find similar workflows
            for j in i + 1..workflows.len() {
                if !assigned[j] && similarity_matrix.similarity_scores[i][j] >= options.similarity_threshold {
                    cluster_workflows.push(workflows[j].id);
                    assigned[j] = true;
                }
            }

            // Create cluster
            let centroid = self.calculate_cluster_centroid(&cluster_workflows, workflows).await?;
            let intra_cluster_similarity = self.calculate_intra_cluster_similarity(&cluster_workflows, similarity_matrix).await?;
            let characteristics = self.analyze_cluster_characteristics(&cluster_workflows, workflows).await?;

            clusters.push(WorkflowCluster {
                cluster_id,
                workflow_ids: cluster_workflows.clone(),
                centroid,
                intra_cluster_similarity,
                cluster_characteristics: characteristics,
                representative_workflow: cluster_workflows.first().copied(),
            });

            cluster_id += 1;
        }

        Ok(clusters)
    }

    /// Detect outlier workflows
    async fn detect_outliers(
        &self,
        _workflows: &[ResearchWorkflow],
        similarity_matrix: &SimilarityMatrix,
        _options: &SimilarityOptions,
    ) -> AppResult<Vec<Uuid>> {
        let mut outliers = Vec::new();

        // Simple outlier detection: workflows with very low average similarity
        for (i, workflow_id) in similarity_matrix.workflow_ids.iter().enumerate() {
            let avg_similarity: f64 = similarity_matrix.similarity_scores[i].iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, &sim)| sim)
                .sum::<f64>() / (similarity_matrix.workflow_ids.len() - 1) as f64;

            if avg_similarity < 0.3 { // Threshold for outliers
                outliers.push(*workflow_id);
            }
        }

        Ok(outliers)
    }

    /// Calculate cluster centroid
    async fn calculate_cluster_centroid(
        &self,
        cluster_workflow_ids: &[Uuid],
        workflows: &[ResearchWorkflow],
    ) -> AppResult<ClusterCentroid> {
        let cluster_workflows: Vec<&ResearchWorkflow> = workflows.iter()
            .filter(|w| cluster_workflow_ids.contains(&w.id))
            .collect();

        if cluster_workflows.is_empty() {
            return Ok(ClusterCentroid {
                average_execution_time: 0.0,
                average_step_count: 0.0,
                average_success_rate: 0.0,
                common_step_types: Vec::new(),
                typical_query_patterns: Vec::new(),
            });
        }

        // Calculate averages
        let execution_times: Vec<f64> = cluster_workflows.iter()
            .filter_map(|w| {
                w.started_at.zip(w.completed_at)
                    .map(|(start, end)| (end - start).num_minutes() as f64)
            })
            .collect();

        let average_execution_time = if execution_times.is_empty() {
            0.0
        } else {
            execution_times.iter().sum::<f64>() / execution_times.len() as f64
        };

        let average_step_count = cluster_workflows.iter()
            .map(|w| w.steps.len() as f64)
            .sum::<f64>() / cluster_workflows.len() as f64;

        let success_rates: Vec<f64> = cluster_workflows.iter()
            .map(|w| {
                if w.steps.is_empty() {
                    0.0
                } else {
                    w.steps.iter().filter(|s| s.status == StepStatus::Completed).count() as f64 / w.steps.len() as f64
                }
            })
            .collect();

        let average_success_rate = success_rates.iter().sum::<f64>() / success_rates.len() as f64;

        // Find common step types
        let mut step_type_counts: HashMap<String, u32> = HashMap::new();
        for workflow in &cluster_workflows {
            for step in &workflow.steps {
                *step_type_counts.entry(step.step_type.clone()).or_insert(0) += 1;
            }
        }

        let common_step_types: Vec<String> = step_type_counts.iter()
            .filter(|(_, &count)| count >= cluster_workflows.len() as u32 / 2)
            .map(|(step_type, _)| step_type.clone())
            .collect();

        Ok(ClusterCentroid {
            average_execution_time,
            average_step_count,
            average_success_rate,
            common_step_types,
            typical_query_patterns: Vec::new(), // TODO: Implement query pattern analysis
        })
    }

    /// Calculate intra-cluster similarity
    async fn calculate_intra_cluster_similarity(
        &self,
        cluster_workflow_ids: &[Uuid],
        similarity_matrix: &SimilarityMatrix,
    ) -> AppResult<f64> {
        if cluster_workflow_ids.len() < 2 {
            return Ok(1.0);
        }

        let mut total_similarity = 0.0;
        let mut comparison_count = 0;

        for i in 0..cluster_workflow_ids.len() {
            for j in i + 1..cluster_workflow_ids.len() {
                if let (Some(idx_i), Some(idx_j)) = (
                    similarity_matrix.workflow_ids.iter().position(|&id| id == cluster_workflow_ids[i]),
                    similarity_matrix.workflow_ids.iter().position(|&id| id == cluster_workflow_ids[j]),
                ) {
                    total_similarity += similarity_matrix.similarity_scores[idx_i][idx_j];
                    comparison_count += 1;
                }
            }
        }

        Ok(if comparison_count > 0 {
            total_similarity / comparison_count as f64
        } else {
            0.0
        })
    }

    /// Analyze cluster characteristics
    async fn analyze_cluster_characteristics(
        &self,
        cluster_workflow_ids: &[Uuid],
        workflows: &[ResearchWorkflow],
    ) -> AppResult<ClusterCharacteristics> {
        let cluster_workflows: Vec<&ResearchWorkflow> = workflows.iter()
            .filter(|w| cluster_workflow_ids.contains(&w.id))
            .collect();

        // Determine cluster type based on characteristics
        let completed_count = cluster_workflows.iter().filter(|w| w.status == WorkflowStatus::Completed).count();
        let completion_rate = completed_count as f64 / cluster_workflows.len() as f64;

        let avg_step_count = cluster_workflows.iter()
            .map(|w| w.steps.len())
            .sum::<usize>() as f64 / cluster_workflows.len() as f64;

        let cluster_type = if completion_rate > 0.9 && avg_step_count > 5.0 {
            ClusterType::HighPerformance
        } else if avg_step_count > 8.0 {
            ClusterType::ComplexWorkflows
        } else if avg_step_count < 3.0 {
            ClusterType::SimpleWorkflows
        } else if completion_rate < 0.5 {
            ClusterType::FailureProne
        } else {
            ClusterType::Production
        };

        Ok(ClusterCharacteristics {
            cluster_type,
            dominant_patterns: Vec::new(),
            performance_profile: PerformanceProfile {
                performance_tier: PerformanceTier::Average,
                consistency_score: 0.7,
                optimization_potential: 0.3,
                bottleneck_patterns: Vec::new(),
            },
            quality_profile: QualityProfile {
                quality_tier: QualityTier::Average,
                consistency_score: 0.7,
                improvement_areas: Vec::new(),
                strength_areas: Vec::new(),
            },
            temporal_patterns: TemporalPatterns {
                peak_usage_hours: Vec::new(),
                seasonal_trends: Vec::new(),
                execution_time_patterns: Vec::new(),
            },
        })
    }

    /// Calculate cluster quality metrics
    async fn calculate_cluster_quality(
        &self,
        _clusters: &[WorkflowCluster],
        _similarity_matrix: &SimilarityMatrix,
    ) -> AppResult<ClusterQualityMetrics> {
        // Placeholder implementation
        Ok(ClusterQualityMetrics {
            silhouette_score: 0.7,
            davies_bouldin_index: 0.5,
            calinski_harabasz_index: 100.0,
            inertia: 50.0,
            cluster_stability: 0.8,
        })
    }

    /// Calculate text similarity using simple word overlap
    fn calculate_text_similarity(&self, text_a: &str, text_b: &str) -> f64 {
        let words_a: std::collections::HashSet<&str> = text_a.split_whitespace().collect();
        let words_b: std::collections::HashSet<&str> = text_b.split_whitespace().collect();

        let intersection = words_a.intersection(&words_b).count();
        let union = words_a.union(&words_b).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    /// Calculate sequence similarity
    fn calculate_sequence_similarity(&self, seq_a: &[&str], seq_b: &[&str]) -> f64 {
        if seq_a.is_empty() && seq_b.is_empty() {
            return 1.0;
        }

        if seq_a.is_empty() || seq_b.is_empty() {
            return 0.0;
        }

        // Simple longest common subsequence similarity
        let common_elements = seq_a.iter()
            .filter(|element| seq_b.contains(element))
            .count();

        let max_length = seq_a.len().max(seq_b.len());
        common_elements as f64 / max_length as f64
    }
}
