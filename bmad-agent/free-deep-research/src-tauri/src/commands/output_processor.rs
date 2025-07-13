use tauri::State;
use tracing::{info, error, debug, warn};
use uuid::Uuid;

use crate::error::AppResult;
use crate::services::ServiceManager;
use crate::services::output_processor::{
    OutputFormat, OutputRequest, OutputResult, OutputOptions, OutputStatistics,
    OutputTemplate, OutputStyling, OutputLayout, Margins,
    VisualizationRequest, ChartType, ChartOutputFormat, ChartResult, VisualizationStatistics,
    ExportRequest, ExportResult, ExportTemplateType, ExportOptions, ExportStatistics,
    ExportJob, ExportJobStatus, ExportDestination, ExportDestinationType,
    ComprehensiveAnalysisRequest, ComprehensiveAnalysisResult, AnalysisType, AnalysisOptions,
    AnalysisFilters, ComparisonResult, ClusterResult, BenchmarkResult, AnalysisStatistics
};

/// Format research workflow results
#[tauri::command]
pub async fn format_workflow_results(
    workflow_id: String,
    format: String,
    template_id: Option<String>,
    options: Option<OutputOptions>,
    service_manager: State<'_, ServiceManager>,
) -> Result<OutputResult, String> {
    info!("Formatting workflow results: {} as {}", workflow_id, format);

    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    let output_format = match format.to_lowercase().as_str() {
        "markdown" | "md" => OutputFormat::Markdown,
        "html" => OutputFormat::HTML,
        "json" => OutputFormat::JSON,
        "pdf" => OutputFormat::PDF,
        "csv" => OutputFormat::CSV,
        "xml" => OutputFormat::XML,
        "docx" => OutputFormat::DOCX,
        "txt" => OutputFormat::TXT,
        _ => return Err(format!("Unsupported output format: {}", format)),
    };

    let request = OutputRequest {
        workflow_id: workflow_uuid,
        format: output_format,
        template_id,
        options: options.unwrap_or_default(),
        custom_template: None,
    };

    // Get the workflow from research engine
    let workflow = {
        let research_engine = service_manager.inner().research_engine.read().await;
        match research_engine.get_workflow(workflow_uuid).await {
            Ok(Some(workflow)) => workflow,
            Ok(None) => return Err(format!("Workflow not found: {}", workflow_id)),
            Err(e) => return Err(format!("Failed to get workflow: {}", e)),
        }
    };

    // Format the results
    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.format_results(&workflow, request).await {
        Ok(result) => {
            info!("Successfully formatted workflow results: {}", workflow_id);
            Ok(result)
        }
        Err(e) => {
            error!("Failed to format workflow results {}: {}", workflow_id, e);
            Err(e.to_string())
        }
    }
}

/// Format multiple workflows in batch
#[tauri::command]
pub async fn format_batch_workflows(
    requests: Vec<serde_json::Value>,
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<OutputResult>, String> {
    info!("Formatting batch of {} workflows", requests.len());

    let mut output_requests = Vec::new();
    let mut workflow_ids = Vec::new();

    // Parse requests
    for request_json in requests {
        let request: OutputRequest = serde_json::from_value(request_json)
            .map_err(|e| format!("Invalid request format: {}", e))?;
        workflow_ids.push(request.workflow_id);
        output_requests.push(request);
    }

    // Get workflows from research engine
    let workflows = {
        let research_engine = service_manager.inner().research_engine.read().await;
        let mut workflows = Vec::new();
        for workflow_id in workflow_ids {
            match research_engine.get_workflow(workflow_id).await {
                Ok(Some(workflow)) => workflows.push(workflow),
                Ok(None) => return Err(format!("Workflow not found: {}", workflow_id)),
                Err(e) => return Err(format!("Failed to get workflow {}: {}", workflow_id, e)),
            }
        }
        workflows
    };

    // Process batch
    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.format_batch_results(&workflows, output_requests).await {
        Ok(results) => {
            info!("Successfully formatted {} workflows", results.len());
            Ok(results)
        }
        Err(e) => {
            error!("Failed to format batch workflows: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get supported output formats
#[tauri::command]
pub async fn get_supported_formats(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<String>, String> {
    debug!("Getting supported output formats");

    let output_processor = service_manager.inner().output_processor.read().await;
    let formats = output_processor.get_supported_formats().await;
    
    Ok(formats.into_iter().map(|f| f.to_string()).collect())
}

/// Get output processing statistics
#[tauri::command]
pub async fn get_output_statistics(
    service_manager: State<'_, ServiceManager>,
) -> Result<OutputStatistics, String> {
    info!("Getting output processing statistics");

    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.get_output_statistics().await {
        Ok(stats) => {
            info!("Retrieved output processing statistics");
            Ok(stats)
        }
        Err(e) => {
            error!("Failed to get output statistics: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get all available templates
#[tauri::command]
pub async fn get_output_templates(
    format: Option<String>,
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<OutputTemplate>, String> {
    info!("Getting output templates");

    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.get_templates(format).await {
        Ok(templates) => {
            info!("Retrieved {} output templates", templates.len());
            Ok(templates)
        }
        Err(e) => {
            error!("Failed to get output templates: {}", e);
            Err(e.to_string())
        }
    }
}

/// Create a new output template
#[tauri::command]
pub async fn create_output_template(
    template: OutputTemplate,
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Creating output template: {}", template.name);

    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.create_template(template).await {
        Ok(()) => {
            info!("Output template created successfully");
            Ok(())
        }
        Err(e) => {
            error!("Failed to create output template: {}", e);
            Err(e.to_string())
        }
    }
}

/// Update an existing output template
#[tauri::command]
pub async fn update_output_template(
    template_id: String,
    template: OutputTemplate,
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Updating output template: {}", template_id);

    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.update_template(&template_id, template).await {
        Ok(()) => {
            info!("Output template updated successfully");
            Ok(())
        }
        Err(e) => {
            error!("Failed to update output template: {}", e);
            Err(e.to_string())
        }
    }
}

/// Delete an output template
#[tauri::command]
pub async fn delete_output_template(
    template_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Deleting output template: {}", template_id);

    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.delete_template(&template_id).await {
        Ok(()) => {
            info!("Output template deleted successfully");
            Ok(())
        }
        Err(e) => {
            error!("Failed to delete output template: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get format recommendations for a workflow
#[tauri::command]
pub async fn get_format_recommendations(
    workflow_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<String>, String> {
    info!("Getting format recommendations for workflow: {}", workflow_id);

    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    // Get the workflow
    let workflow = {
        let research_engine = service_manager.inner().research_engine.read().await;
        match research_engine.get_workflow(workflow_uuid).await {
            Ok(Some(workflow)) => workflow,
            Ok(None) => return Err(format!("Workflow not found: {}", workflow_id)),
            Err(e) => return Err(format!("Failed to get workflow: {}", e)),
        }
    };

    let output_processor = service_manager.inner().output_processor.read().await;
    let recommendations = output_processor.get_format_recommendations(&workflow).await;
    
    Ok(recommendations.into_iter().map(|f| f.to_string()).collect())
}

/// Validate an output request
#[tauri::command]
pub async fn validate_output_request(
    request: OutputRequest,
    service_manager: State<'_, ServiceManager>,
) -> Result<bool, String> {
    debug!("Validating output request");

    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.validate_request(&request).await {
        Ok(()) => Ok(true),
        Err(e) => {
            warn!("Output request validation failed: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get file extension for a format
#[tauri::command]
pub async fn get_format_file_extension(
    format: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<String, String> {
    let output_format = match format.to_lowercase().as_str() {
        "markdown" | "md" => OutputFormat::Markdown,
        "html" => OutputFormat::HTML,
        "json" => OutputFormat::JSON,
        "pdf" => OutputFormat::PDF,
        "csv" => OutputFormat::CSV,
        "xml" => OutputFormat::XML,
        "docx" => OutputFormat::DOCX,
        "txt" => OutputFormat::TXT,
        _ => return Err(format!("Unsupported output format: {}", format)),
    };

    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.get_file_extension(output_format).await {
        Ok(extension) => Ok(extension.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

/// Get MIME type for a format
#[tauri::command]
pub async fn get_format_mime_type(
    format: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<String, String> {
    let output_format = match format.to_lowercase().as_str() {
        "markdown" | "md" => OutputFormat::Markdown,
        "html" => OutputFormat::HTML,
        "json" => OutputFormat::JSON,
        "pdf" => OutputFormat::PDF,
        "csv" => OutputFormat::CSV,
        "xml" => OutputFormat::XML,
        "docx" => OutputFormat::DOCX,
        "txt" => OutputFormat::TXT,
        _ => return Err(format!("Unsupported output format: {}", format)),
    };

    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.get_mime_type(output_format).await {
        Ok(mime_type) => Ok(mime_type.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

// ============================================================================
// VISUALIZATION COMMANDS
// ============================================================================

/// Generate a chart from workflow data
#[tauri::command]
pub async fn generate_workflow_chart(
    workflow_id: String,
    chart_type: String,
    output_format: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<ChartResult, String> {
    info!("Generating {} chart for workflow: {}", chart_type, workflow_id);

    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    let chart_type_enum = match chart_type.to_lowercase().as_str() {
        "bar" => ChartType::Bar,
        "line" => ChartType::Line,
        "pie" => ChartType::Pie,
        "scatter" => ChartType::Scatter,
        "timeline" => ChartType::Timeline,
        "network" => ChartType::Network,
        "heatmap" => ChartType::Heatmap,
        "histogram" => ChartType::Histogram,
        _ => return Err(format!("Unsupported chart type: {}", chart_type)),
    };

    let output_format_enum = match output_format.to_lowercase().as_str() {
        "svg" => ChartOutputFormat::SVG,
        "html" => ChartOutputFormat::HTML,
        "png" => ChartOutputFormat::PNG,
        "canvas" => ChartOutputFormat::Canvas,
        "pdf" => ChartOutputFormat::PDF,
        _ => return Err(format!("Unsupported chart output format: {}", output_format)),
    };

    // Get the workflow
    let workflow = {
        let research_engine = service_manager.inner().research_engine.read().await;
        match research_engine.get_workflow(workflow_uuid).await {
            Ok(Some(workflow)) => workflow,
            Ok(None) => return Err(format!("Workflow not found: {}", workflow_id)),
            Err(e) => return Err(format!("Failed to get workflow: {}", e)),
        }
    };

    let request = VisualizationRequest {
        workflow_id: workflow_uuid,
        chart_type: chart_type_enum,
        output_format: output_format_enum,
        config: crate::services::output_processor::ChartConfig::default_for_type(chart_type_enum),
        data_filters: None,
    };

    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.generate_chart(&workflow, request).await {
        Ok(result) => {
            info!("Successfully generated chart for workflow: {}", workflow_id);
            Ok(result)
        }
        Err(e) => {
            error!("Failed to generate chart for workflow {}: {}", workflow_id, e);
            Err(e.to_string())
        }
    }
}

/// Generate multiple charts for a workflow
#[tauri::command]
pub async fn generate_multiple_charts(
    workflow_id: String,
    chart_types: Vec<String>,
    output_format: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<ChartResult>, String> {
    info!("Generating {} charts for workflow: {}", chart_types.len(), workflow_id);

    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    let chart_type_enums: Result<Vec<ChartType>, String> = chart_types.iter()
        .map(|ct| match ct.to_lowercase().as_str() {
            "bar" => Ok(ChartType::Bar),
            "line" => Ok(ChartType::Line),
            "pie" => Ok(ChartType::Pie),
            "scatter" => Ok(ChartType::Scatter),
            "timeline" => Ok(ChartType::Timeline),
            "network" => Ok(ChartType::Network),
            "heatmap" => Ok(ChartType::Heatmap),
            "histogram" => Ok(ChartType::Histogram),
            _ => Err(format!("Unsupported chart type: {}", ct)),
        })
        .collect();

    let chart_type_enums = chart_type_enums?;

    let output_format_enum = match output_format.to_lowercase().as_str() {
        "svg" => ChartOutputFormat::SVG,
        "html" => ChartOutputFormat::HTML,
        "png" => ChartOutputFormat::PNG,
        "canvas" => ChartOutputFormat::Canvas,
        "pdf" => ChartOutputFormat::PDF,
        _ => return Err(format!("Unsupported chart output format: {}", output_format)),
    };

    // Get the workflow
    let workflow = {
        let research_engine = service_manager.inner().research_engine.read().await;
        match research_engine.get_workflow(workflow_uuid).await {
            Ok(Some(workflow)) => workflow,
            Ok(None) => return Err(format!("Workflow not found: {}", workflow_id)),
            Err(e) => return Err(format!("Failed to get workflow: {}", e)),
        }
    };

    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.generate_workflow_charts(&workflow, chart_type_enums, output_format_enum).await {
        Ok(results) => {
            info!("Successfully generated {} charts for workflow: {}", results.len(), workflow_id);
            Ok(results)
        }
        Err(e) => {
            error!("Failed to generate charts for workflow {}: {}", workflow_id, e);
            Err(e.to_string())
        }
    }
}

/// Get chart recommendations for a workflow
#[tauri::command]
pub async fn get_chart_recommendations(
    workflow_id: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<String>, String> {
    info!("Getting chart recommendations for workflow: {}", workflow_id);

    let workflow_uuid = Uuid::parse_str(&workflow_id)
        .map_err(|e| format!("Invalid workflow ID: {}", e))?;

    // Get the workflow
    let workflow = {
        let research_engine = service_manager.inner().research_engine.read().await;
        match research_engine.get_workflow(workflow_uuid).await {
            Ok(Some(workflow)) => workflow,
            Ok(None) => return Err(format!("Workflow not found: {}", workflow_id)),
            Err(e) => return Err(format!("Failed to get workflow: {}", e)),
        }
    };

    let output_processor = service_manager.inner().output_processor.read().await;
    let recommendations = output_processor.get_chart_recommendations(&workflow).await;

    Ok(recommendations.into_iter().map(|ct| ct.to_string()).collect())
}

/// Get supported chart types
#[tauri::command]
pub async fn get_supported_chart_types(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<String>, String> {
    debug!("Getting supported chart types");

    let output_processor = service_manager.inner().output_processor.read().await;
    let chart_types = output_processor.get_supported_chart_types();

    Ok(chart_types.into_iter().map(|ct| ct.to_string()).collect())
}

/// Get supported chart output formats
#[tauri::command]
pub async fn get_supported_chart_formats(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<String>, String> {
    debug!("Getting supported chart output formats");

    let output_processor = service_manager.inner().output_processor.read().await;
    let formats = output_processor.get_supported_chart_formats();

    Ok(formats.into_iter().map(|f| f.to_string()).collect())
}

/// Get visualization statistics
#[tauri::command]
pub async fn get_visualization_statistics(
    service_manager: State<'_, ServiceManager>,
) -> Result<VisualizationStatistics, String> {
    info!("Getting visualization statistics");

    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.get_visualization_statistics().await {
        Ok(stats) => {
            info!("Retrieved visualization statistics");
            Ok(stats)
        }
        Err(e) => {
            error!("Failed to get visualization statistics: {}", e);
            Err(e.to_string())
        }
    }
}

/// Clear visualization cache
#[tauri::command]
pub async fn clear_visualization_cache(
    service_manager: State<'_, ServiceManager>,
) -> Result<(), String> {
    info!("Clearing visualization cache");

    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.clear_visualization_cache().await {
        Ok(()) => {
            info!("Visualization cache cleared");
            Ok(())
        }
        Err(e) => {
            error!("Failed to clear visualization cache: {}", e);
            Err(e.to_string())
        }
    }
}

// ============================================================================
// EXPORT COMMANDS
// ============================================================================

/// Export workflows using a template
#[tauri::command]
pub async fn export_workflows(
    workflow_ids: Vec<String>,
    template_id: Option<String>,
    destination_type: String,
    destination_path: String,
    service_manager: State<'_, ServiceManager>,
) -> Result<ExportResult, String> {
    info!("Exporting {} workflows", workflow_ids.len());

    // Parse workflow IDs
    let mut parsed_ids = Vec::new();
    for id_str in workflow_ids {
        match Uuid::parse_str(&id_str) {
            Ok(id) => parsed_ids.push(id),
            Err(e) => return Err(format!("Invalid workflow ID {}: {}", id_str, e)),
        }
    }

    // Parse destination type
    let dest_type = match destination_type.to_lowercase().as_str() {
        "local" | "filesystem" => ExportDestinationType::LocalFileSystem,
        "s3" => ExportDestinationType::S3,
        "email" => ExportDestinationType::Email,
        _ => return Err(format!("Unsupported destination type: {}", destination_type)),
    };

    // Get workflows
    let workflows = {
        let research_engine = service_manager.inner().research_engine.read().await;
        let mut workflows = Vec::new();
        for workflow_id in &parsed_ids {
            match research_engine.get_workflow(*workflow_id).await {
                Ok(Some(workflow)) => workflows.push(workflow),
                Ok(None) => return Err(format!("Workflow not found: {}", workflow_id)),
                Err(e) => return Err(format!("Failed to get workflow {}: {}", workflow_id, e)),
            }
        }
        workflows
    };

    // Create export request
    let request = ExportRequest {
        id: Uuid::new_v4(),
        workflow_ids: parsed_ids,
        template_id,
        destination: ExportDestination {
            destination_type: dest_type,
            config: crate::services::output_processor::export::export_destinations::DestinationConfig {
                endpoint: None,
                region: None,
                bucket: None,
                folder: None,
                host: None,
                port: None,
                database_name: None,
                table_name: None,
                custom_fields: std::collections::HashMap::new(),
            },
            credentials: None,
            path: destination_path,
            options: crate::services::output_processor::export::export_destinations::DestinationOptions::default(),
        },
        options: ExportOptions::default(),
        schedule: None,
        metadata: std::collections::HashMap::new(),
    };

    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.export_workflows(&workflows, request).await {
        Ok(result) => {
            info!("Successfully exported {} workflows", workflows.len());
            Ok(result)
        }
        Err(e) => {
            error!("Failed to export workflows: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get export templates
#[tauri::command]
pub async fn get_export_templates(
    service_manager: State<'_, ServiceManager>,
) -> Result<Vec<ExportTemplateType>, String> {
    info!("Getting export templates");

    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.get_export_templates().await {
        Ok(templates) => {
            info!("Retrieved {} export templates", templates.len());
            Ok(templates)
        }
        Err(e) => {
            error!("Failed to get export templates: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get export statistics
#[tauri::command]
pub async fn get_export_statistics(
    service_manager: State<'_, ServiceManager>,
) -> Result<ExportStatistics, String> {
    info!("Getting export statistics");

    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.get_export_statistics().await {
        Ok(stats) => {
            info!("Retrieved export statistics");
            Ok(stats)
        }
        Err(e) => {
            error!("Failed to get export statistics: {}", e);
            Err(e.to_string())
        }
    }
}

// ============================================================================
// ANALYSIS COMMANDS
// ============================================================================

/// Perform comprehensive analysis on workflows
#[tauri::command]
pub async fn perform_comprehensive_analysis(
    workflow_ids: Vec<String>,
    analysis_types: Vec<String>,
    service_manager: State<'_, ServiceManager>,
) -> Result<ComprehensiveAnalysisResult, String> {
    info!("Performing comprehensive analysis on {} workflows", workflow_ids.len());

    // Parse workflow IDs
    let mut parsed_ids = Vec::new();
    for id_str in workflow_ids {
        match Uuid::parse_str(&id_str) {
            Ok(id) => parsed_ids.push(id),
            Err(e) => return Err(format!("Invalid workflow ID {}: {}", id_str, e)),
        }
    }

    // Parse analysis types
    let mut parsed_types = Vec::new();
    for type_str in analysis_types {
        let analysis_type = match type_str.to_lowercase().as_str() {
            "comparison" => AnalysisType::Comparison,
            "statistical" => AnalysisType::Statistical,
            "similarity" => AnalysisType::Similarity,
            "performance" => AnalysisType::Performance,
            "trend" => AnalysisType::Trend,
            "quality" => AnalysisType::Quality,
            _ => return Err(format!("Unsupported analysis type: {}", type_str)),
        };
        parsed_types.push(analysis_type);
    }

    // Get workflows
    let workflows = {
        let research_engine = service_manager.inner().research_engine.read().await;
        let mut workflows = Vec::new();
        for workflow_id in &parsed_ids {
            match research_engine.get_workflow(*workflow_id).await {
                Ok(Some(workflow)) => workflows.push(workflow),
                Ok(None) => return Err(format!("Workflow not found: {}", workflow_id)),
                Err(e) => return Err(format!("Failed to get workflow {}: {}", workflow_id, e)),
            }
        }
        workflows
    };

    // Create analysis request
    let request = ComprehensiveAnalysisRequest {
        id: Uuid::new_v4(),
        workflow_ids: parsed_ids,
        analysis_types: parsed_types,
        time_range: None,
        filters: AnalysisFilters::default(),
        options: AnalysisOptions::default(),
    };

    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.perform_comprehensive_analysis(&workflows, request).await {
        Ok(result) => {
            info!("Successfully completed comprehensive analysis");
            Ok(result)
        }
        Err(e) => {
            error!("Failed to perform comprehensive analysis: {}", e);
            Err(e.to_string())
        }
    }
}

/// Compare workflows
#[tauri::command]
pub async fn compare_workflows(
    workflow_ids: Vec<String>,
    service_manager: State<'_, ServiceManager>,
) -> Result<ComparisonResult, String> {
    info!("Comparing {} workflows", workflow_ids.len());

    // Parse workflow IDs
    let mut parsed_ids = Vec::new();
    for id_str in workflow_ids {
        match Uuid::parse_str(&id_str) {
            Ok(id) => parsed_ids.push(id),
            Err(e) => return Err(format!("Invalid workflow ID {}: {}", id_str, e)),
        }
    }

    if parsed_ids.len() < 2 {
        return Err("At least 2 workflows are required for comparison".to_string());
    }

    // Get workflows
    let workflows = {
        let research_engine = service_manager.inner().research_engine.read().await;
        let mut workflows = Vec::new();
        for workflow_id in &parsed_ids {
            match research_engine.get_workflow(*workflow_id).await {
                Ok(Some(workflow)) => workflows.push(workflow),
                Ok(None) => return Err(format!("Workflow not found: {}", workflow_id)),
                Err(e) => return Err(format!("Failed to get workflow {}: {}", workflow_id, e)),
            }
        }
        workflows
    };

    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.compare_workflows(&workflows, parsed_ids).await {
        Ok(result) => {
            info!("Successfully compared workflows");
            Ok(result)
        }
        Err(e) => {
            error!("Failed to compare workflows: {}", e);
            Err(e.to_string())
        }
    }
}

/// Analyze workflow similarity
#[tauri::command]
pub async fn analyze_workflow_similarity(
    workflow_ids: Vec<String>,
    service_manager: State<'_, ServiceManager>,
) -> Result<ClusterResult, String> {
    info!("Analyzing similarity among {} workflows", workflow_ids.len());

    // Parse workflow IDs
    let mut parsed_ids = Vec::new();
    for id_str in workflow_ids {
        match Uuid::parse_str(&id_str) {
            Ok(id) => parsed_ids.push(id),
            Err(e) => return Err(format!("Invalid workflow ID {}: {}", id_str, e)),
        }
    }

    // Get workflows
    let workflows = {
        let research_engine = service_manager.inner().research_engine.read().await;
        let mut workflows = Vec::new();
        for workflow_id in &parsed_ids {
            match research_engine.get_workflow(*workflow_id).await {
                Ok(Some(workflow)) => workflows.push(workflow),
                Ok(None) => return Err(format!("Workflow not found: {}", workflow_id)),
                Err(e) => return Err(format!("Failed to get workflow {}: {}", workflow_id, e)),
            }
        }
        workflows
    };

    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.analyze_workflow_similarity(&workflows).await {
        Ok(result) => {
            info!("Successfully analyzed workflow similarity");
            Ok(result)
        }
        Err(e) => {
            error!("Failed to analyze workflow similarity: {}", e);
            Err(e.to_string())
        }
    }
}

/// Analyze workflow performance
#[tauri::command]
pub async fn analyze_workflow_performance(
    workflow_ids: Vec<String>,
    service_manager: State<'_, ServiceManager>,
) -> Result<BenchmarkResult, String> {
    info!("Analyzing performance of {} workflows", workflow_ids.len());

    // Parse workflow IDs
    let mut parsed_ids = Vec::new();
    for id_str in workflow_ids {
        match Uuid::parse_str(&id_str) {
            Ok(id) => parsed_ids.push(id),
            Err(e) => return Err(format!("Invalid workflow ID {}: {}", id_str, e)),
        }
    }

    // Get workflows
    let workflows = {
        let research_engine = service_manager.inner().research_engine.read().await;
        let mut workflows = Vec::new();
        for workflow_id in &parsed_ids {
            match research_engine.get_workflow(*workflow_id).await {
                Ok(Some(workflow)) => workflows.push(workflow),
                Ok(None) => return Err(format!("Workflow not found: {}", workflow_id)),
                Err(e) => return Err(format!("Failed to get workflow {}: {}", workflow_id, e)),
            }
        }
        workflows
    };

    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.analyze_workflow_performance(&workflows).await {
        Ok(result) => {
            info!("Successfully analyzed workflow performance");
            Ok(result)
        }
        Err(e) => {
            error!("Failed to analyze workflow performance: {}", e);
            Err(e.to_string())
        }
    }
}

/// Get analysis statistics
#[tauri::command]
pub async fn get_analysis_statistics(
    service_manager: State<'_, ServiceManager>,
) -> Result<AnalysisStatistics, String> {
    info!("Getting analysis statistics");

    let output_processor = service_manager.inner().output_processor.read().await;
    match output_processor.get_analysis_statistics().await {
        Ok(stats) => {
            info!("Retrieved analysis statistics");
            Ok(stats)
        }
        Err(e) => {
            error!("Failed to get analysis statistics: {}", e);
            Err(e.to_string())
        }
    }
}
