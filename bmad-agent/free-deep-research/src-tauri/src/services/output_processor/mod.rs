use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, error, warn};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, ResearchError};
use crate::models::research_workflow::{ResearchWorkflow, ResearchResults};
use crate::services::Service;

pub mod formatters;
pub mod templates;
pub mod engine;
pub mod visualization;
pub mod export;
pub mod analysis;

use self::formatters::{OutputFormatter, MarkdownFormatter, HTMLFormatter, JSONFormatter, PDFFormatter, CSVFormatter, XMLFormatter, TXTFormatter, DOCXFormatter};
use self::templates::{OutputTemplate, TemplateManager};
use self::engine::OutputEngine;
use self::visualization::{VisualizationEngine, VisualizationRequest, ChartType, ChartOutputFormat};
use self::export::{ExportService, ExportRequest, ExportResult, ExportTemplate as ExportTemplateType};
use self::analysis::{AnalysisService, ComprehensiveAnalysisRequest, ComprehensiveAnalysisResult};

/// Output processing service for research results
pub struct OutputProcessorService {
    template_manager: Arc<RwLock<TemplateManager>>,
    output_engine: Arc<OutputEngine>,
    output_history: Arc<RwLock<Vec<OutputResult>>>,
    formatters: HashMap<OutputFormat, Box<dyn OutputFormatter>>,
    visualization_engine: Arc<VisualizationEngine>,
    export_service: Arc<RwLock<ExportService>>,
    analysis_service: Arc<RwLock<AnalysisService>>,
}

/// Supported output formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OutputFormat {
    Markdown,
    HTML,
    JSON,
    PDF,
    CSV,
    XML,
    DOCX,
    TXT,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Markdown => write!(f, "markdown"),
            OutputFormat::HTML => write!(f, "html"),
            OutputFormat::JSON => write!(f, "json"),
            OutputFormat::PDF => write!(f, "pdf"),
            OutputFormat::CSV => write!(f, "csv"),
            OutputFormat::XML => write!(f, "xml"),
            OutputFormat::DOCX => write!(f, "docx"),
            OutputFormat::TXT => write!(f, "txt"),
        }
    }
}

/// Output formatting request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputRequest {
    pub workflow_id: Uuid,
    pub format: OutputFormat,
    pub template_id: Option<String>,
    pub options: OutputOptions,
    pub custom_template: Option<String>,
}

/// Output formatting options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputOptions {
    pub include_metadata: bool,
    pub include_raw_data: bool,
    pub include_charts: bool,
    pub styling: OutputStyling,
    pub layout: OutputLayout,
    pub compression: bool,
    pub watermark: Option<String>,
}

impl Default for OutputOptions {
    fn default() -> Self {
        Self {
            include_metadata: true,
            include_raw_data: false,
            include_charts: true,
            styling: OutputStyling::default(),
            layout: OutputLayout::default(),
            compression: false,
            watermark: None,
        }
    }
}

/// Output styling options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputStyling {
    pub theme: String,
    pub font_family: String,
    pub font_size: u32,
    pub color_scheme: String,
    pub custom_css: Option<String>,
}

impl Default for OutputStyling {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            font_family: "Arial, sans-serif".to_string(),
            font_size: 12,
            color_scheme: "light".to_string(),
            custom_css: None,
        }
    }
}

/// Output layout options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputLayout {
    pub page_size: String,
    pub orientation: String,
    pub margins: Margins,
    pub header: Option<String>,
    pub footer: Option<String>,
    pub page_numbers: bool,
}

impl Default for OutputLayout {
    fn default() -> Self {
        Self {
            page_size: "A4".to_string(),
            orientation: "portrait".to_string(),
            margins: Margins::default(),
            header: None,
            footer: None,
            page_numbers: true,
        }
    }
}

/// Page margins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Margins {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl Default for Margins {
    fn default() -> Self {
        Self {
            top: 1.0,
            bottom: 1.0,
            left: 1.0,
            right: 1.0,
        }
    }
}

/// Output formatting result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputResult {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub format: OutputFormat,
    pub content: String,
    pub metadata: OutputMetadata,
    pub created_at: DateTime<Utc>,
    pub file_size_bytes: u64,
    pub processing_time_ms: u64,
}

/// Output metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputMetadata {
    pub title: String,
    pub description: Option<String>,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub workflow_name: String,
    pub template_used: Option<String>,
    pub format_version: String,
    pub tags: Vec<String>,
    pub custom_fields: HashMap<String, String>,
}

/// Output processing statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputStatistics {
    pub total_outputs_generated: u64,
    pub outputs_by_format: HashMap<OutputFormat, u64>,
    pub average_processing_time_ms: f64,
    pub total_file_size_bytes: u64,
    pub most_used_templates: Vec<String>,
    pub success_rate: f64,
    pub error_rate: f64,
}

impl OutputProcessorService {
    /// Create a new output processor service
    pub async fn new() -> AppResult<Self> {
        info!("Initializing output processor service...");

        let template_manager = Arc::new(RwLock::new(TemplateManager::new().await?));
        let output_engine = Arc::new(OutputEngine::new().await?);
        let output_history = Arc::new(RwLock::new(Vec::new()));
        let visualization_engine = Arc::new(VisualizationEngine::new().await?);
        let export_service = Arc::new(RwLock::new(ExportService::new().await?));
        let analysis_service = Arc::new(RwLock::new(AnalysisService::new().await?));

        // Initialize formatters
        let mut formatters: HashMap<OutputFormat, Box<dyn OutputFormatter>> = HashMap::new();
        formatters.insert(OutputFormat::Markdown, Box::new(MarkdownFormatter::new()));
        formatters.insert(OutputFormat::HTML, Box::new(HTMLFormatter::new()));
        formatters.insert(OutputFormat::JSON, Box::new(JSONFormatter::new()));
        formatters.insert(OutputFormat::PDF, Box::new(PDFFormatter::new()));
        formatters.insert(OutputFormat::CSV, Box::new(CSVFormatter::new()));
        formatters.insert(OutputFormat::XML, Box::new(XMLFormatter::new()));
        formatters.insert(OutputFormat::TXT, Box::new(TXTFormatter::new()));
        formatters.insert(OutputFormat::DOCX, Box::new(DOCXFormatter::new()));

        let service = Self {
            template_manager,
            output_engine,
            output_history,
            formatters,
            visualization_engine,
            export_service,
            analysis_service,
        };

        info!("Output processor service initialized successfully");
        Ok(service)
    }

    /// Format research results into specified format
    pub async fn format_results(
        &self,
        workflow: &ResearchWorkflow,
        request: OutputRequest,
    ) -> AppResult<OutputResult> {
        info!("Formatting results for workflow: {} in format: {}", workflow.id, request.format);

        let start_time = std::time::Instant::now();

        // Get formatter for the requested format
        let formatter = self.formatters.get(&request.format)
            .ok_or_else(|| ResearchError::invalid_request(
                format!("Unsupported output format: {}", request.format)
            ))?;

        // Get template if specified
        let template = if let Some(template_id) = &request.template_id {
            let template_manager = self.template_manager.read().await;
            Some(template_manager.get_template(template_id).await?)
        } else if let Some(custom_template) = &request.custom_template {
            Some(OutputTemplate {
                id: "custom".to_string(),
                name: "Custom Template".to_string(),
                content: custom_template.clone(),
                format: request.format,
                variables: HashMap::new(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        } else {
            None
        };

        // Format the content
        let content = formatter.format(workflow, template.as_ref(), &request.options).await?;

        let processing_time = start_time.elapsed();

        // Create output result
        let output_result = OutputResult {
            id: Uuid::new_v4(),
            workflow_id: workflow.id,
            format: request.format,
            content: content.clone(),
            metadata: OutputMetadata {
                title: workflow.name.clone(),
                description: Some(workflow.query.clone()),
                author: "Research Engine".to_string(),
                created_at: Utc::now(),
                workflow_name: workflow.name.clone(),
                template_used: template.map(|t| t.name),
                format_version: "1.0".to_string(),
                tags: vec!["research".to_string(), request.format.to_string()],
                custom_fields: HashMap::new(),
            },
            created_at: Utc::now(),
            file_size_bytes: content.len() as u64,
            processing_time_ms: processing_time.as_millis() as u64,
        };

        // Store in history
        {
            let mut history = self.output_history.write().await;
            history.push(output_result.clone());

            // Keep only last 1000 outputs
            if history.len() > 1000 {
                history.remove(0);
            }
        }

        info!("Successfully formatted results for workflow: {} ({}ms)", 
            workflow.id, processing_time.as_millis());

        Ok(output_result)
    }

    /// Format multiple workflows in batch
    pub async fn format_batch_results(
        &self,
        workflows: &[ResearchWorkflow],
        requests: Vec<OutputRequest>,
    ) -> AppResult<Vec<OutputResult>> {
        info!("Formatting batch of {} workflows", requests.len());

        let mut results = Vec::new();
        for request in requests {
            if let Some(workflow) = workflows.iter().find(|w| w.id == request.workflow_id) {
                match self.format_results(workflow, request).await {
                    Ok(result) => results.push(result),
                    Err(e) => {
                        error!("Failed to format workflow {}: {}", workflow.id, e);
                        // Continue with other workflows
                    }
                }
            }
        }

        Ok(results)
    }

    /// Get supported output formats
    pub async fn get_supported_formats(&self) -> Vec<OutputFormat> {
        self.formatters.keys().cloned().collect()
    }

    /// Get output processing statistics
    pub async fn get_output_statistics(&self) -> AppResult<OutputStatistics> {
        let history = self.output_history.read().await;

        let total_outputs = history.len() as u64;
        let mut outputs_by_format = HashMap::new();
        let mut total_processing_time = 0u64;
        let mut total_file_size = 0u64;

        for output in history.iter() {
            *outputs_by_format.entry(output.format).or_insert(0) += 1;
            total_processing_time += output.processing_time_ms;
            total_file_size += output.file_size_bytes;
        }

        let average_processing_time = if total_outputs > 0 {
            total_processing_time as f64 / total_outputs as f64
        } else {
            0.0
        };

        Ok(OutputStatistics {
            total_outputs_generated: total_outputs,
            outputs_by_format,
            average_processing_time_ms: average_processing_time,
            total_file_size_bytes: total_file_size,
            most_used_templates: Vec::new(), // TODO: Track template usage
            success_rate: 100.0, // TODO: Track failures
            error_rate: 0.0,
        })
    }

    /// Get templates
    pub async fn get_templates(&self, format: Option<String>) -> AppResult<Vec<OutputTemplate>> {
        let template_manager = self.template_manager.read().await;

        if let Some(format_str) = format {
            let output_format = match format_str.to_lowercase().as_str() {
                "markdown" | "md" => OutputFormat::Markdown,
                "html" => OutputFormat::HTML,
                "json" => OutputFormat::JSON,
                "pdf" => OutputFormat::PDF,
                "csv" => OutputFormat::CSV,
                "xml" => OutputFormat::XML,
                "docx" => OutputFormat::DOCX,
                "txt" => OutputFormat::TXT,
                _ => return Err(ResearchError::invalid_request(format!("Unsupported format: {}", format_str)).into()),
            };
            template_manager.get_templates_by_format(output_format).await
        } else {
            template_manager.get_all_templates().await
        }
    }

    /// Create template
    pub async fn create_template(&self, template: OutputTemplate) -> AppResult<()> {
        let template_manager = self.template_manager.read().await;
        template_manager.add_template(template).await
    }

    /// Update template
    pub async fn update_template(&self, template_id: &str, template: OutputTemplate) -> AppResult<()> {
        let template_manager = self.template_manager.read().await;
        template_manager.update_template(template_id, template).await
    }

    /// Delete template
    pub async fn delete_template(&self, template_id: &str) -> AppResult<()> {
        let template_manager = self.template_manager.read().await;
        template_manager.delete_template(template_id).await
    }

    /// Get format recommendations
    pub async fn get_format_recommendations(&self, workflow: &ResearchWorkflow) -> Vec<OutputFormat> {
        let mut recommendations = Vec::new();

        // Always recommend JSON for data interchange
        recommendations.push(OutputFormat::JSON);

        // Recommend Markdown for readable reports
        recommendations.push(OutputFormat::Markdown);

        // Recommend HTML for rich formatting
        recommendations.push(OutputFormat::HTML);

        recommendations
    }

    /// Validate request
    pub async fn validate_request(&self, request: &OutputRequest) -> AppResult<()> {
        if !self.formatters.contains_key(&request.format) {
            return Err(ResearchError::invalid_request(
                format!("Unsupported format: {}", request.format)
            ).into());
        }
        Ok(())
    }

    /// Get file extension
    pub async fn get_file_extension(&self, format: OutputFormat) -> AppResult<&'static str> {
        let formatter = self.formatters.get(&format)
            .ok_or_else(|| ResearchError::invalid_request(format!("Unsupported format: {}", format)))?;
        Ok(formatter.file_extension())
    }

    /// Get MIME type
    pub async fn get_mime_type(&self, format: OutputFormat) -> AppResult<&'static str> {
        let formatter = self.formatters.get(&format)
            .ok_or_else(|| ResearchError::invalid_request(format!("Unsupported format: {}", format)))?;
        Ok(formatter.mime_type())
    }

    /// Generate a chart from workflow data
    pub async fn generate_chart(
        &self,
        workflow: &ResearchWorkflow,
        request: VisualizationRequest,
    ) -> AppResult<visualization::ChartResult> {
        info!("Generating chart for workflow: {}", workflow.id);
        self.visualization_engine.generate_chart(workflow, request).await
    }

    /// Generate multiple charts for a workflow
    pub async fn generate_workflow_charts(
        &self,
        workflow: &ResearchWorkflow,
        chart_types: Vec<ChartType>,
        output_format: ChartOutputFormat,
    ) -> AppResult<Vec<visualization::ChartResult>> {
        info!("Generating {} charts for workflow: {}", chart_types.len(), workflow.id);
        self.visualization_engine.generate_workflow_charts(workflow, chart_types, output_format).await
    }

    /// Get chart recommendations for a workflow
    pub async fn get_chart_recommendations(&self, workflow: &ResearchWorkflow) -> Vec<ChartType> {
        self.visualization_engine.get_chart_recommendations(workflow).await
    }

    /// Get supported chart types
    pub fn get_supported_chart_types(&self) -> Vec<ChartType> {
        self.visualization_engine.get_supported_chart_types()
    }

    /// Get supported chart output formats
    pub fn get_supported_chart_formats(&self) -> Vec<ChartOutputFormat> {
        self.visualization_engine.get_supported_output_formats()
    }

    /// Validate visualization request
    pub fn validate_visualization_request(&self, request: &VisualizationRequest) -> AppResult<()> {
        self.visualization_engine.validate_request(request)
    }

    /// Clear visualization cache
    pub async fn clear_visualization_cache(&self) -> AppResult<()> {
        self.visualization_engine.clear_cache().await
    }

    /// Get visualization statistics
    pub async fn get_visualization_statistics(&self) -> AppResult<visualization::VisualizationStatistics> {
        self.visualization_engine.get_statistics().await
    }

    /// Export workflows using a template
    pub async fn export_workflows(
        &self,
        workflows: &[ResearchWorkflow],
        request: ExportRequest,
    ) -> AppResult<ExportResult> {
        info!("Exporting {} workflows", workflows.len());
        let export_service = self.export_service.read().await;
        export_service.export_workflows(workflows, request).await
    }

    /// Get export templates
    pub async fn get_export_templates(&self) -> AppResult<Vec<ExportTemplateType>> {
        let export_service = self.export_service.read().await;
        export_service.get_export_templates().await
    }

    /// Create export template
    pub async fn create_export_template(&self, template: ExportTemplateType) -> AppResult<()> {
        info!("Creating export template: {}", template.name);
        let export_service = self.export_service.read().await;
        export_service.create_export_template(template).await
    }

    /// Get export jobs
    pub async fn get_export_jobs(&self, status_filter: Option<export::ExportJobStatus>) -> AppResult<Vec<export::ExportJob>> {
        let export_service = self.export_service.read().await;
        export_service.get_export_jobs(status_filter).await
    }

    /// Cancel export job
    pub async fn cancel_export_job(&self, job_id: Uuid) -> AppResult<bool> {
        info!("Cancelling export job: {}", job_id);
        let export_service = self.export_service.read().await;
        export_service.cancel_export_job(job_id).await
    }

    /// Get export statistics
    pub async fn get_export_statistics(&self) -> AppResult<export::ExportStatistics> {
        let export_service = self.export_service.read().await;
        export_service.get_export_statistics().await
    }

    /// Perform comprehensive analysis on workflows
    pub async fn perform_comprehensive_analysis(
        &self,
        workflows: &[ResearchWorkflow],
        request: ComprehensiveAnalysisRequest,
    ) -> AppResult<ComprehensiveAnalysisResult> {
        info!("Performing comprehensive analysis on {} workflows", workflows.len());
        let analysis_service = self.analysis_service.read().await;
        analysis_service.perform_comprehensive_analysis(workflows, request).await
    }

    /// Compare workflows
    pub async fn compare_workflows(
        &self,
        workflows: &[ResearchWorkflow],
        workflow_ids: Vec<Uuid>,
    ) -> AppResult<analysis::ComparisonResult> {
        info!("Comparing {} workflows", workflow_ids.len());
        let analysis_service = self.analysis_service.read().await;

        let comparison_request = analysis::ComparisonRequest {
            id: Uuid::new_v4(),
            workflow_ids,
            comparison_type: analysis::comparison_engine::ComparisonType::Comprehensive,
            options: analysis::comparison_engine::ComparisonOptions::default(),
        };

        analysis_service.comparison_engine.compare_workflows(workflows, comparison_request).await
    }

    /// Analyze workflow similarity
    pub async fn analyze_workflow_similarity(
        &self,
        workflows: &[ResearchWorkflow],
    ) -> AppResult<analysis::ClusterResult> {
        info!("Analyzing similarity among {} workflows", workflows.len());
        let analysis_service = self.analysis_service.read().await;
        analysis_service.similarity_detector.detect_similarity(workflows).await
    }

    /// Analyze workflow performance
    pub async fn analyze_workflow_performance(
        &self,
        workflows: &[ResearchWorkflow],
    ) -> AppResult<analysis::BenchmarkResult> {
        info!("Analyzing performance of {} workflows", workflows.len());
        let analysis_service = self.analysis_service.read().await;
        analysis_service.performance_analyzer.analyze_performance(workflows).await
    }

    /// Get analysis statistics
    pub async fn get_analysis_statistics(&self) -> AppResult<analysis::AnalysisStatistics> {
        let analysis_service = self.analysis_service.read().await;
        analysis_service.get_analysis_statistics().await
    }
}

#[async_trait::async_trait]
impl Service for OutputProcessorService {
    async fn health_check(&self) -> AppResult<()> {
        debug!("Performing output processor health check");

        // Check template manager health
        let template_manager = self.template_manager.read().await;
        if template_manager.get_available_templates().await?.is_empty() {
            return Err(crate::error::OutputError::template_not_found("No templates available".to_string()).into());
        }
        drop(template_manager);

        // Check formatters are available
        if self.formatters.is_empty() {
            return Err(crate::error::OutputError::format_error("No formatters available".to_string()).into());
        }

        // Check export service health
        let export_service = self.export_service.read().await;
        // Export service health check would go here
        drop(export_service);

        debug!("Output processor health check completed successfully");
        Ok(())
    }

    async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down output processor service...");
        // TODO: Implement graceful shutdown
        Ok(())
    }
}

// Re-export types for external use
pub use formatters::{OutputFormatter, MarkdownFormatter, HTMLFormatter, JSONFormatter, PDFFormatter, CSVFormatter, XMLFormatter, TXTFormatter, DOCXFormatter};
pub use templates::{OutputTemplate, TemplateManager};
pub use engine::OutputEngine;
pub use visualization::{
    VisualizationEngine, VisualizationRequest, ChartType, ChartOutputFormat,
    ChartConfig, ChartData, ChartResult, ChartStyling, VisualizationStatistics
};
pub use export::{
    ExportService, ExportRequest, ExportResult, ExportTemplate as ExportTemplateType,
    ExportOptions, ExportStatistics, ExportJob, ExportJobStatus, ExportDestination,
    ExportDestinationType, CompressionType, PackageType
};
pub use analysis::{
    AnalysisService, ComprehensiveAnalysisRequest, ComprehensiveAnalysisResult,
    AnalysisType, AnalysisOptions, AnalysisFilters, ComparisonResult, ClusterResult,
    BenchmarkResult, AnalysisStatistics, AnalysisInsight, AnalysisRecommendation
};
