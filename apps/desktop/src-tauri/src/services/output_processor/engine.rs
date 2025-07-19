use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, error, warn};
use uuid::Uuid;
use chrono::Utc;

use crate::error::{AppResult, ResearchError};
use crate::models::research_workflow::ResearchWorkflow;
use super::{
    OutputFormat, OutputRequest, OutputResult, OutputOptions, OutputStatistics,
    formatters::{OutputFormatter, MarkdownFormatter, HTMLFormatter, JSONFormatter},
    templates::TemplateManager,
};

/// Output processing engine
pub struct OutputEngine {
    formatters: HashMap<OutputFormat, Box<dyn OutputFormatter>>,
    processing_stats: Arc<RwLock<ProcessingStats>>,
}

/// Internal processing statistics
#[derive(Debug, Default)]
struct ProcessingStats {
    total_processed: u64,
    successful_outputs: u64,
    failed_outputs: u64,
    processing_times: Vec<u64>,
    format_usage: HashMap<OutputFormat, u64>,
    total_bytes_processed: u64,
    template_usage: HashMap<String, u32>,
}

impl OutputEngine {
    /// Create a new output engine
    pub async fn new() -> AppResult<Self> {
        info!("Initializing output engine...");

        let mut formatters: HashMap<OutputFormat, Box<dyn OutputFormatter>> = HashMap::new();
        
        // Register built-in formatters
        formatters.insert(OutputFormat::Markdown, Box::new(MarkdownFormatter::new()));
        formatters.insert(OutputFormat::HTML, Box::new(HTMLFormatter::new()));
        formatters.insert(OutputFormat::JSON, Box::new(JSONFormatter::new()));

        let engine = Self {
            formatters,
            processing_stats: Arc::new(RwLock::new(ProcessingStats::default())),
        };

        info!("Output engine initialized with {} formatters", engine.formatters.len());
        Ok(engine)
    }

    /// Process a single output request
    pub async fn process_output(
        &self,
        workflow: &ResearchWorkflow,
        request: OutputRequest,
        template_manager: &TemplateManager,
    ) -> AppResult<OutputResult> {
        info!("Processing output for workflow: {} in format: {}", workflow.id, request.format);

        let start_time = std::time::Instant::now();

        // Get formatter for the requested format
        let formatter = self.formatters.get(&request.format)
            .ok_or_else(|| ResearchError::invalid_request(
                format!("Unsupported output format: {}", request.format)
            ))?;

        // Get template if specified
        let template = if let Some(template_id) = &request.template_id {
            Some(template_manager.get_template(template_id).await?)
        } else {
            template_manager.get_default_template(request.format).cloned()
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
            metadata: super::OutputMetadata {
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

        // Update statistics
        self.update_processing_stats(&output_result).await;

        info!("Successfully processed output for workflow: {} ({}ms)", 
            workflow.id, processing_time.as_millis());

        Ok(output_result)
    }

    /// Process multiple output requests in batch
    pub async fn process_batch_outputs(
        &self,
        workflows: &[ResearchWorkflow],
        requests: Vec<OutputRequest>,
        template_manager: &TemplateManager,
    ) -> AppResult<Vec<OutputResult>> {
        info!("Processing batch of {} output requests", requests.len());

        let mut results = Vec::new();
        let mut errors = Vec::new();

        for request in requests {
            // Find the corresponding workflow
            if let Some(workflow) = workflows.iter().find(|w| w.id == request.workflow_id) {
                match self.process_output(workflow, request, template_manager).await {
                    Ok(result) => results.push(result),
                    Err(e) => {
                        error!("Failed to process output for workflow {}: {}", workflow.id, e);
                        errors.push(e);
                    }
                }
            } else {
                let error = ResearchError::not_found(
                    format!("Workflow not found: {}", request.workflow_id)
                );
                errors.push(error.into());
            }
        }

        if !errors.is_empty() {
            warn!("Batch processing completed with {} errors out of {} requests", 
                errors.len(), requests.len());
        }

        info!("Batch processing completed: {} successful, {} failed", 
            results.len(), errors.len());

        Ok(results)
    }

    /// Get supported output formats
    pub fn get_supported_formats(&self) -> Vec<OutputFormat> {
        self.formatters.keys().cloned().collect()
    }

    /// Check if a format is supported
    pub fn is_format_supported(&self, format: OutputFormat) -> bool {
        self.formatters.contains_key(&format)
    }

    /// Get file extension for a format
    pub fn get_file_extension(&self, format: OutputFormat) -> AppResult<&'static str> {
        let formatter = self.formatters.get(&format)
            .ok_or_else(|| ResearchError::invalid_request(
                format!("Unsupported output format: {}", format)
            ))?;
        Ok(formatter.file_extension())
    }

    /// Get MIME type for a format
    pub fn get_mime_type(&self, format: OutputFormat) -> AppResult<&'static str> {
        let formatter = self.formatters.get(&format)
            .ok_or_else(|| ResearchError::invalid_request(
                format!("Unsupported output format: {}", format)
            ))?;
        Ok(formatter.mime_type())
    }

    /// Get processing statistics
    pub async fn get_processing_statistics(&self) -> AppResult<OutputStatistics> {
        let stats = self.processing_stats.read().await;

        let average_processing_time_ms = if !stats.processing_times.is_empty() {
            stats.processing_times.iter().sum::<u64>() as f64 / stats.processing_times.len() as f64
        } else {
            0.0
        };

        let success_rate = if stats.total_processed > 0 {
            (stats.successful_outputs as f64 / stats.total_processed as f64) * 100.0
        } else {
            0.0
        };

        let error_rate = 100.0 - success_rate;

        // Calculate total file size from all outputs
        let total_file_size_bytes = stats.total_bytes_processed;

        // Get most used templates from stats
        let mut template_usage: Vec<(String, u32)> = stats.template_usage.into_iter().collect();
        template_usage.sort_by(|a, b| b.1.cmp(&a.1));
        let most_used_templates: Vec<String> = template_usage.into_iter()
            .take(5)
            .map(|(template, _)| template)
            .collect();

        Ok(OutputStatistics {
            total_outputs_generated: stats.successful_outputs,
            outputs_by_format: stats.format_usage.clone(),
            average_processing_time_ms,
            total_file_size_bytes,
            most_used_templates,
            success_rate,
            error_rate,
        })
    }

    /// Update internal processing statistics
    async fn update_processing_stats(&self, result: &OutputResult) {
        let mut stats = self.processing_stats.write().await;

        stats.total_processed += 1;
        stats.successful_outputs += 1;
        stats.processing_times.push(result.processing_time_ms);

        // Keep only last 1000 processing times
        if stats.processing_times.len() > 1000 {
            stats.processing_times.remove(0);
        }

        *stats.format_usage.entry(result.format).or_insert(0) += 1;

        // Track file size
        stats.total_bytes_processed += result.file_size_bytes;

        // Track template usage
        if let Some(template_name) = &result.metadata.template_used {
            *stats.template_usage.entry(template_name.clone()).or_insert(0) += 1;
        }
    }

    /// Register a custom formatter
    pub fn register_formatter(&mut self, format: OutputFormat, formatter: Box<dyn OutputFormatter>) {
        info!("Registering custom formatter for format: {}", format);
        self.formatters.insert(format, formatter);
    }

    /// Validate output request
    pub fn validate_output_request(&self, request: &OutputRequest) -> AppResult<()> {
        // Check if format is supported
        if !self.is_format_supported(request.format) {
            return Err(ResearchError::invalid_request(
                format!("Unsupported output format: {}", request.format)
            ).into());
        }

        // Validate options
        if request.options.styling.font_size < 8 || request.options.styling.font_size > 72 {
            return Err(ResearchError::invalid_request(
                "Font size must be between 8 and 72".to_string()
            ).into());
        }

        // Validate layout options
        if !["A4", "A3", "Letter", "Legal"].contains(&request.options.layout.page_size.as_str()) {
            return Err(ResearchError::invalid_request(
                "Invalid page size. Supported: A4, A3, Letter, Legal".to_string()
            ).into());
        }

        if !["portrait", "landscape"].contains(&request.options.layout.orientation.as_str()) {
            return Err(ResearchError::invalid_request(
                "Invalid orientation. Supported: portrait, landscape".to_string()
            ).into());
        }

        Ok(())
    }

    /// Get format recommendations based on content type
    pub fn get_format_recommendations(&self, workflow: &ResearchWorkflow) -> Vec<OutputFormat> {
        let mut recommendations = Vec::new();

        // Always recommend JSON for data interchange
        recommendations.push(OutputFormat::JSON);

        // Recommend Markdown for readable reports
        recommendations.push(OutputFormat::Markdown);

        // Recommend HTML for rich formatting
        recommendations.push(OutputFormat::HTML);

        // If workflow has many data points, recommend CSV
        if let Some(results) = &workflow.results {
            if results.sources.len() > 10 {
                recommendations.push(OutputFormat::CSV);
            }
        }

        // If workflow is completed, recommend PDF for archival
        if workflow.status == crate::models::research_workflow::WorkflowStatus::Completed {
            recommendations.push(OutputFormat::PDF);
        }

        recommendations
    }
}
