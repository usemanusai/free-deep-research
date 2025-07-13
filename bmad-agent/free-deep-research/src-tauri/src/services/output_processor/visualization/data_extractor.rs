use std::collections::HashMap;
use tracing::{info, debug, warn};
use chrono::Utc;

use crate::error::{AppResult, ResearchError};
use crate::models::research_workflow::{ResearchWorkflow, StepStatus};
use super::{DataFilters, DateRange};
use super::chart_types::{
    ChartType, ChartData, Dataset, DataPoint, DataValue, ChartMetadata, LineStyle
};

/// Data extractor for creating chart data from research workflows
pub struct DataExtractor;

impl DataExtractor {
    pub fn new() -> Self {
        Self
    }

    /// Extract chart data from a research workflow
    pub async fn extract_chart_data(
        &self,
        workflow: &ResearchWorkflow,
        chart_type: ChartType,
        filters: Option<&DataFilters>,
    ) -> AppResult<ChartData> {
        debug!("Extracting {} chart data from workflow: {}", chart_type, workflow.id);

        match chart_type {
            ChartType::Bar => self.extract_bar_chart_data(workflow, filters).await,
            ChartType::Line => self.extract_line_chart_data(workflow, filters).await,
            ChartType::Pie => self.extract_pie_chart_data(workflow, filters).await,
            ChartType::Timeline => self.extract_timeline_data(workflow, filters).await,
            ChartType::Scatter => self.extract_scatter_data(workflow, filters).await,
            ChartType::Network => self.extract_network_data(workflow, filters).await,
            ChartType::Heatmap => self.extract_heatmap_data(workflow, filters).await,
            ChartType::Histogram => self.extract_histogram_data(workflow, filters).await,
            _ => Err(ResearchError::invalid_request(
                format!("Chart type {:?} not yet implemented", chart_type)
            ).into()),
        }
    }

    /// Extract data for bar charts (step durations)
    async fn extract_bar_chart_data(
        &self,
        workflow: &ResearchWorkflow,
        _filters: Option<&DataFilters>,
    ) -> AppResult<ChartData> {
        let mut labels = Vec::new();
        let mut data_points = Vec::new();

        for (index, step) in workflow.steps.iter().enumerate() {
            labels.push(format!("Step {}: {}", index + 1, step.step_type));

            let duration = if let (Some(started), Some(completed)) = (step.started_at, step.completed_at) {
                (completed - started).num_minutes() as f64
            } else {
                0.0
            };

            data_points.push(DataPoint {
                x: DataValue::String(format!("Step {}", index + 1)),
                y: DataValue::Number(duration),
                label: Some(step.step_type.clone()),
                metadata: Some(HashMap::from([
                    ("status".to_string(), format!("{:?}", step.status)),
                    ("duration_minutes".to_string(), duration.to_string()),
                ])),
            });
        }

        let dataset = Dataset {
            label: "Step Duration (minutes)".to_string(),
            data: data_points,
            color: "#3498db".to_string(),
            border_color: Some("#2980b9".to_string()),
            fill: false,
            line_style: LineStyle::Solid,
        };

        Ok(ChartData {
            labels,
            datasets: vec![dataset],
            metadata: ChartMetadata {
                workflow_id: workflow.id,
                chart_type: ChartType::Bar,
                generated_at: Utc::now(),
                data_source: "workflow_steps".to_string(),
                total_data_points: workflow.steps.len(),
                custom_fields: HashMap::new(),
            },
        })
    }

    /// Extract data for line charts (progress over time)
    async fn extract_line_chart_data(
        &self,
        workflow: &ResearchWorkflow,
        _filters: Option<&DataFilters>,
    ) -> AppResult<ChartData> {
        let mut labels = Vec::new();
        let mut data_points = Vec::new();
        let mut completed_count = 0;

        for (index, step) in workflow.steps.iter().enumerate() {
            if step.status == StepStatus::Completed {
                completed_count += 1;
            }

            let progress_percentage = (completed_count as f64 / workflow.steps.len() as f64) * 100.0;
            
            labels.push(format!("Step {}", index + 1));
            data_points.push(DataPoint {
                x: DataValue::Number(index as f64),
                y: DataValue::Number(progress_percentage),
                label: Some(step.step_type.clone()),
                metadata: Some(HashMap::from([
                    ("completed_steps".to_string(), completed_count.to_string()),
                    ("total_steps".to_string(), workflow.steps.len().to_string()),
                ])),
            });
        }

        let dataset = Dataset {
            label: "Progress (%)".to_string(),
            data: data_points,
            color: "#2ecc71".to_string(),
            border_color: Some("#27ae60".to_string()),
            fill: true,
            line_style: LineStyle::Solid,
        };

        Ok(ChartData {
            labels,
            datasets: vec![dataset],
            metadata: ChartMetadata {
                workflow_id: workflow.id,
                chart_type: ChartType::Line,
                generated_at: Utc::now(),
                data_source: "workflow_progress".to_string(),
                total_data_points: workflow.steps.len(),
                custom_fields: HashMap::new(),
            },
        })
    }

    /// Extract data for pie charts (status distribution)
    async fn extract_pie_chart_data(
        &self,
        workflow: &ResearchWorkflow,
        _filters: Option<&DataFilters>,
    ) -> AppResult<ChartData> {
        let mut status_counts: HashMap<StepStatus, usize> = HashMap::new();

        for step in &workflow.steps {
            *status_counts.entry(step.status).or_insert(0) += 1;
        }

        let mut labels = Vec::new();
        let mut data_points = Vec::new();

        for (status, count) in status_counts {
            labels.push(format!("{:?}", status));
            data_points.push(DataPoint {
                x: DataValue::String(format!("{:?}", status)),
                y: DataValue::Number(count as f64),
                label: Some(format!("{:?}", status)),
                metadata: Some(HashMap::from([
                    ("count".to_string(), count.to_string()),
                    ("percentage".to_string(), format!("{:.1}", (count as f64 / workflow.steps.len() as f64) * 100.0)),
                ])),
            });
        }

        let colors = vec![
            "#2ecc71".to_string(), // Completed - Green
            "#f39c12".to_string(), // Running - Orange
            "#e74c3c".to_string(), // Failed - Red
            "#95a5a6".to_string(), // Pending - Gray
        ];

        let dataset = Dataset {
            label: "Step Status Distribution".to_string(),
            data: data_points,
            color: colors[0].clone(),
            border_color: Some("#ffffff".to_string()),
            fill: false,
            line_style: LineStyle::Solid,
        };

        Ok(ChartData {
            labels,
            datasets: vec![dataset],
            metadata: ChartMetadata {
                workflow_id: workflow.id,
                chart_type: ChartType::Pie,
                generated_at: Utc::now(),
                data_source: "step_status".to_string(),
                total_data_points: status_counts.len(),
                custom_fields: HashMap::new(),
            },
        })
    }

    /// Extract data for timeline charts
    async fn extract_timeline_data(
        &self,
        workflow: &ResearchWorkflow,
        _filters: Option<&DataFilters>,
    ) -> AppResult<ChartData> {
        let mut labels = Vec::new();
        let mut data_points = Vec::new();

        for (index, step) in workflow.steps.iter().enumerate() {
            let timestamp = step.started_at.unwrap_or(workflow.created_at);
            let duration = if let (Some(started), Some(completed)) = (step.started_at, step.completed_at) {
                (completed - started).num_minutes() as f64
            } else {
                0.0
            };

            labels.push(format!("{} - {}", step.step_type, format!("{:?}", step.status)));
            data_points.push(DataPoint {
                x: DataValue::DateTime(timestamp),
                y: DataValue::Number(duration),
                label: Some(step.step_type.clone()),
                metadata: Some(HashMap::from([
                    ("step_index".to_string(), index.to_string()),
                    ("status".to_string(), format!("{:?}", step.status)),
                    ("started_at".to_string(), timestamp.to_rfc3339()),
                ])),
            });
        }

        let dataset = Dataset {
            label: "Workflow Timeline".to_string(),
            data: data_points,
            color: "#9b59b6".to_string(),
            border_color: Some("#8e44ad".to_string()),
            fill: false,
            line_style: LineStyle::Solid,
        };

        Ok(ChartData {
            labels,
            datasets: vec![dataset],
            metadata: ChartMetadata {
                workflow_id: workflow.id,
                chart_type: ChartType::Timeline,
                generated_at: Utc::now(),
                data_source: "workflow_timeline".to_string(),
                total_data_points: workflow.steps.len(),
                custom_fields: HashMap::new(),
            },
        })
    }

    /// Extract data for scatter plots
    async fn extract_scatter_data(
        &self,
        workflow: &ResearchWorkflow,
        _filters: Option<&DataFilters>,
    ) -> AppResult<ChartData> {
        let mut data_points = Vec::new();

        for (index, step) in workflow.steps.iter().enumerate() {
            let duration = if let (Some(started), Some(completed)) = (step.started_at, step.completed_at) {
                (completed - started).num_minutes() as f64
            } else {
                0.0
            };

            let complexity_score = step.step_type.len() as f64; // Simple complexity metric

            data_points.push(DataPoint {
                x: DataValue::Number(complexity_score),
                y: DataValue::Number(duration),
                label: Some(step.step_type.clone()),
                metadata: Some(HashMap::from([
                    ("step_index".to_string(), index.to_string()),
                    ("complexity".to_string(), complexity_score.to_string()),
                ])),
            });
        }

        let dataset = Dataset {
            label: "Complexity vs Duration".to_string(),
            data: data_points,
            color: "#e67e22".to_string(),
            border_color: Some("#d35400".to_string()),
            fill: false,
            line_style: LineStyle::Solid,
        };

        Ok(ChartData {
            labels: vec![], // Scatter plots don't use labels
            datasets: vec![dataset],
            metadata: ChartMetadata {
                workflow_id: workflow.id,
                chart_type: ChartType::Scatter,
                generated_at: Utc::now(),
                data_source: "step_complexity_duration".to_string(),
                total_data_points: workflow.steps.len(),
                custom_fields: HashMap::new(),
            },
        })
    }

    /// Extract data for network graphs (source relationships)
    async fn extract_network_data(
        &self,
        workflow: &ResearchWorkflow,
        _filters: Option<&DataFilters>,
    ) -> AppResult<ChartData> {
        let mut labels = Vec::new();
        let mut data_points = Vec::new();

        if let Some(results) = &workflow.results {
            // Create nodes for each source
            for (index, source) in results.sources.iter().enumerate() {
                labels.push(source.title.clone());
                data_points.push(DataPoint {
                    x: DataValue::Number(index as f64),
                    y: DataValue::Number(1.0), // All sources have equal weight for now
                    label: Some(source.title.clone()),
                    metadata: Some(HashMap::from([
                        ("url".to_string(), source.url.clone()),
                        ("source_type".to_string(), "research_source".to_string()),
                    ])),
                });
            }
        }

        let dataset = Dataset {
            label: "Source Network".to_string(),
            data: data_points,
            color: "#1abc9c".to_string(),
            border_color: Some("#16a085".to_string()),
            fill: false,
            line_style: LineStyle::Solid,
        };

        Ok(ChartData {
            labels,
            datasets: vec![dataset],
            metadata: ChartMetadata {
                workflow_id: workflow.id,
                chart_type: ChartType::Network,
                generated_at: Utc::now(),
                data_source: "research_sources".to_string(),
                total_data_points: workflow.results.as_ref().map(|r| r.sources.len()).unwrap_or(0),
                custom_fields: HashMap::new(),
            },
        })
    }

    /// Extract data for heatmaps
    async fn extract_heatmap_data(
        &self,
        workflow: &ResearchWorkflow,
        _filters: Option<&DataFilters>,
    ) -> AppResult<ChartData> {
        // For now, create a simple heatmap based on step execution times
        let mut labels = Vec::new();
        let mut data_points = Vec::new();

        for (index, step) in workflow.steps.iter().enumerate() {
            let hour = if let Some(started) = step.started_at {
                started.hour() as f64
            } else {
                0.0
            };

            let intensity = match step.status {
                StepStatus::Completed => 1.0,
                StepStatus::Running => 0.7,
                StepStatus::Failed => 0.3,
                StepStatus::Pending => 0.1,
            };

            labels.push(format!("Step {}", index + 1));
            data_points.push(DataPoint {
                x: DataValue::Number(hour),
                y: DataValue::Number(index as f64),
                label: Some(step.step_type.clone()),
                metadata: Some(HashMap::from([
                    ("intensity".to_string(), intensity.to_string()),
                    ("hour".to_string(), hour.to_string()),
                ])),
            });
        }

        let dataset = Dataset {
            label: "Activity Heatmap".to_string(),
            data: data_points,
            color: "#e74c3c".to_string(),
            border_color: Some("#c0392b".to_string()),
            fill: false,
            line_style: LineStyle::Solid,
        };

        Ok(ChartData {
            labels,
            datasets: vec![dataset],
            metadata: ChartMetadata {
                workflow_id: workflow.id,
                chart_type: ChartType::Heatmap,
                generated_at: Utc::now(),
                data_source: "step_activity".to_string(),
                total_data_points: workflow.steps.len(),
                custom_fields: HashMap::new(),
            },
        })
    }

    /// Extract data for histograms
    async fn extract_histogram_data(
        &self,
        workflow: &ResearchWorkflow,
        _filters: Option<&DataFilters>,
    ) -> AppResult<ChartData> {
        let mut durations = Vec::new();

        for step in &workflow.steps {
            if let (Some(started), Some(completed)) = (step.started_at, step.completed_at) {
                let duration = (completed - started).num_minutes() as f64;
                durations.push(duration);
            }
        }

        // Create histogram bins
        let bin_count = 10;
        let max_duration = durations.iter().fold(0.0, |acc, &x| acc.max(x));
        let bin_size = max_duration / bin_count as f64;

        let mut bins = vec![0; bin_count];
        let mut labels = Vec::new();
        let mut data_points = Vec::new();

        for duration in durations {
            let bin_index = ((duration / bin_size) as usize).min(bin_count - 1);
            bins[bin_index] += 1;
        }

        for (i, count) in bins.iter().enumerate() {
            let bin_start = i as f64 * bin_size;
            let bin_end = (i + 1) as f64 * bin_size;
            labels.push(format!("{:.1}-{:.1}min", bin_start, bin_end));
            data_points.push(DataPoint {
                x: DataValue::Number(bin_start),
                y: DataValue::Number(*count as f64),
                label: Some(format!("Bin {}", i + 1)),
                metadata: Some(HashMap::from([
                    ("bin_start".to_string(), bin_start.to_string()),
                    ("bin_end".to_string(), bin_end.to_string()),
                    ("count".to_string(), count.to_string()),
                ])),
            });
        }

        let dataset = Dataset {
            label: "Duration Distribution".to_string(),
            data: data_points,
            color: "#f39c12".to_string(),
            border_color: Some("#e67e22".to_string()),
            fill: false,
            line_style: LineStyle::Solid,
        };

        Ok(ChartData {
            labels,
            datasets: vec![dataset],
            metadata: ChartMetadata {
                workflow_id: workflow.id,
                chart_type: ChartType::Histogram,
                generated_at: Utc::now(),
                data_source: "step_durations".to_string(),
                total_data_points: bins.len(),
                custom_fields: HashMap::new(),
            },
        })
    }

    /// Apply filters to chart data
    pub fn apply_filters(&self, data: &mut ChartData, filters: &DataFilters) {
        // Apply date range filter
        if let Some(date_range) = &filters.date_range {
            for dataset in &mut data.datasets {
                dataset.data.retain(|point| {
                    if let DataValue::DateTime(dt) = point.x {
                        dt >= date_range.start && dt <= date_range.end
                    } else {
                        true // Keep non-datetime data
                    }
                });
            }
        }

        // Apply max data points limit
        if let Some(max_points) = filters.max_data_points {
            for dataset in &mut data.datasets {
                if dataset.data.len() > max_points {
                    dataset.data.truncate(max_points);
                }
            }
        }
    }
}
