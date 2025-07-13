use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, error, warn};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, ResearchError};
use crate::models::research_workflow::{ResearchWorkflow, ResearchResults, StepStatus};

pub mod chart_generator;
pub mod data_extractor;
pub mod chart_types;

use self::chart_generator::{ChartGenerator, SVGChartGenerator, HTMLChartGenerator};
use self::data_extractor::DataExtractor;
use self::chart_types::{ChartType, ChartConfig, ChartData, ChartResult, ChartStyling};

/// Visualization engine for creating charts and graphs from research data
pub struct VisualizationEngine {
    chart_generators: HashMap<ChartOutputFormat, Box<dyn ChartGenerator>>,
    data_extractor: Arc<DataExtractor>,
    chart_cache: Arc<RwLock<HashMap<String, ChartResult>>>,
}

/// Chart output formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChartOutputFormat {
    SVG,
    PNG,
    HTML,
    Canvas,
    PDF,
}

impl std::fmt::Display for ChartOutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChartOutputFormat::SVG => write!(f, "svg"),
            ChartOutputFormat::PNG => write!(f, "png"),
            ChartOutputFormat::HTML => write!(f, "html"),
            ChartOutputFormat::Canvas => write!(f, "canvas"),
            ChartOutputFormat::PDF => write!(f, "pdf"),
        }
    }
}

/// Visualization request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationRequest {
    pub workflow_id: Uuid,
    pub chart_type: ChartType,
    pub output_format: ChartOutputFormat,
    pub config: ChartConfig,
    pub data_filters: Option<DataFilters>,
}

/// Data filtering options for visualizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFilters {
    pub date_range: Option<DateRange>,
    pub status_filter: Option<Vec<StepStatus>>,
    pub source_types: Option<Vec<String>>,
    pub include_errors: bool,
    pub max_data_points: Option<usize>,
}

/// Date range for filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

/// Visualization statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationStatistics {
    pub total_charts_generated: u64,
    pub charts_by_type: HashMap<ChartType, u64>,
    pub charts_by_format: HashMap<ChartOutputFormat, u64>,
    pub average_generation_time_ms: f64,
    pub cache_hit_rate: f64,
    pub most_popular_chart_types: Vec<ChartType>,
}

impl VisualizationEngine {
    /// Create a new visualization engine
    pub async fn new() -> AppResult<Self> {
        info!("Initializing visualization engine...");

        let mut chart_generators: HashMap<ChartOutputFormat, Box<dyn ChartGenerator>> = HashMap::new();
        
        // Register chart generators
        chart_generators.insert(ChartOutputFormat::SVG, Box::new(SVGChartGenerator::new()));
        chart_generators.insert(ChartOutputFormat::HTML, Box::new(HTMLChartGenerator::new()));

        let data_extractor = Arc::new(DataExtractor::new());
        let chart_cache = Arc::new(RwLock::new(HashMap::new()));

        let engine = Self {
            chart_generators,
            data_extractor,
            chart_cache,
        };

        info!("Visualization engine initialized with {} generators", engine.chart_generators.len());
        Ok(engine)
    }

    /// Generate a chart from research workflow data
    pub async fn generate_chart(
        &self,
        workflow: &ResearchWorkflow,
        request: VisualizationRequest,
    ) -> AppResult<ChartResult> {
        info!("Generating {} chart for workflow: {}", request.chart_type, workflow.id);

        let start_time = std::time::Instant::now();

        // Check cache first
        let cache_key = self.generate_cache_key(&request);
        {
            let cache = self.chart_cache.read().await;
            if let Some(cached_result) = cache.get(&cache_key) {
                debug!("Chart cache hit for key: {}", cache_key);
                return Ok(cached_result.clone());
            }
        }

        // Extract data for visualization
        let chart_data = self.data_extractor.extract_chart_data(
            workflow,
            request.chart_type,
            request.data_filters.as_ref(),
        ).await?;

        // Get chart generator for the requested format
        let generator = self.chart_generators.get(&request.output_format)
            .ok_or_else(|| ResearchError::invalid_request(
                format!("Unsupported chart output format: {}", request.output_format)
            ))?;

        // Generate the chart
        let chart_result = generator.generate_chart(chart_data, &request.config).await?;

        let generation_time = start_time.elapsed();

        // Cache the result
        {
            let mut cache = self.chart_cache.write().await;
            cache.insert(cache_key, chart_result.clone());

            // Limit cache size
            if cache.len() > 1000 {
                // Remove oldest entries (simple FIFO for now)
                let keys_to_remove: Vec<String> = cache.keys().take(100).cloned().collect();
                for key in keys_to_remove {
                    cache.remove(&key);
                }
            }
        }

        info!("Chart generated successfully in {}ms", generation_time.as_millis());
        Ok(chart_result)
    }

    /// Generate multiple charts for a workflow
    pub async fn generate_workflow_charts(
        &self,
        workflow: &ResearchWorkflow,
        chart_types: Vec<ChartType>,
        output_format: ChartOutputFormat,
    ) -> AppResult<Vec<ChartResult>> {
        info!("Generating {} charts for workflow: {}", chart_types.len(), workflow.id);

        let mut results = Vec::new();

        for chart_type in chart_types {
            let request = VisualizationRequest {
                workflow_id: workflow.id,
                chart_type,
                output_format,
                config: ChartConfig::default_for_type(chart_type),
                data_filters: None,
            };

            match self.generate_chart(workflow, request).await {
                Ok(result) => results.push(result),
                Err(e) => {
                    error!("Failed to generate {} chart: {}", chart_type, e);
                    // Continue with other charts
                }
            }
        }

        info!("Generated {} out of {} requested charts", results.len(), chart_types.len());
        Ok(results)
    }

    /// Get recommended chart types for a workflow
    pub async fn get_chart_recommendations(&self, workflow: &ResearchWorkflow) -> Vec<ChartType> {
        let mut recommendations = Vec::new();

        // Always recommend timeline for workflow steps
        recommendations.push(ChartType::Timeline);

        // Recommend status distribution if there are multiple steps
        if workflow.steps.len() > 1 {
            recommendations.push(ChartType::Pie);
        }

        // Recommend bar chart for step durations
        if workflow.steps.iter().any(|s| s.started_at.is_some() && s.completed_at.is_some()) {
            recommendations.push(ChartType::Bar);
        }

        // Recommend source analysis if there are results with sources
        if let Some(results) = &workflow.results {
            if !results.sources.is_empty() {
                recommendations.push(ChartType::Network);
            }
        }

        recommendations
    }

    /// Get supported chart types
    pub fn get_supported_chart_types(&self) -> Vec<ChartType> {
        vec![
            ChartType::Bar,
            ChartType::Line,
            ChartType::Pie,
            ChartType::Scatter,
            ChartType::Timeline,
            ChartType::Network,
            ChartType::Heatmap,
            ChartType::Histogram,
        ]
    }

    /// Get supported output formats
    pub fn get_supported_output_formats(&self) -> Vec<ChartOutputFormat> {
        self.chart_generators.keys().cloned().collect()
    }

    /// Clear chart cache
    pub async fn clear_cache(&self) -> AppResult<()> {
        let mut cache = self.chart_cache.write().await;
        cache.clear();
        info!("Chart cache cleared");
        Ok(())
    }

    /// Get visualization statistics
    pub async fn get_statistics(&self) -> AppResult<VisualizationStatistics> {
        // TODO: Implement proper statistics tracking
        Ok(VisualizationStatistics {
            total_charts_generated: 0,
            charts_by_type: HashMap::new(),
            charts_by_format: HashMap::new(),
            average_generation_time_ms: 0.0,
            cache_hit_rate: 0.0,
            most_popular_chart_types: Vec::new(),
        })
    }

    /// Generate cache key for a visualization request
    fn generate_cache_key(&self, request: &VisualizationRequest) -> String {
        format!(
            "{}_{:?}_{:?}_{:?}",
            request.workflow_id,
            request.chart_type,
            request.output_format,
            request.config.title
        )
    }

    /// Validate visualization request
    pub fn validate_request(&self, request: &VisualizationRequest) -> AppResult<()> {
        // Check if chart type is supported
        if !self.get_supported_chart_types().contains(&request.chart_type) {
            return Err(ResearchError::invalid_request(
                format!("Unsupported chart type: {:?}", request.chart_type)
            ).into());
        }

        // Check if output format is supported
        if !self.chart_generators.contains_key(&request.output_format) {
            return Err(ResearchError::invalid_request(
                format!("Unsupported output format: {}", request.output_format)
            ).into());
        }

        // Validate chart configuration
        if request.config.width == 0 || request.config.height == 0 {
            return Err(ResearchError::invalid_request(
                "Chart dimensions must be greater than 0".to_string()
            ).into());
        }

        Ok(())
    }

    /// Register a custom chart generator
    pub fn register_generator(&mut self, format: ChartOutputFormat, generator: Box<dyn ChartGenerator>) {
        info!("Registering custom chart generator for format: {}", format);
        self.chart_generators.insert(format, generator);
    }
}

// Re-export types for external use
pub use chart_types::{ChartType, ChartConfig, ChartData, ChartResult, ChartStyling};
pub use chart_generator::ChartGenerator;
pub use data_extractor::DataExtractor;
