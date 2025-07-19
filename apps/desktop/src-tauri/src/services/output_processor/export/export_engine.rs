use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use tracing::{info, debug, error, warn};
use uuid::Uuid;
use chrono::Utc;

use crate::error::{AppResult, ResearchError};
use crate::models::research_workflow::ResearchWorkflow;
use super::{
    ExportRequest, ExportResult, ExportStatus, ExportedFile, CompressionType, PackageType
};

/// Export engine for processing export requests
pub struct ExportEngine {
    temp_dir: PathBuf,
    output_dir: PathBuf,
}

impl ExportEngine {
    /// Create a new export engine
    pub async fn new() -> AppResult<Self> {
        info!("Initializing export engine...");

        // Create temporary and output directories
        let temp_dir = std::env::temp_dir().join("research_exports_temp");
        let output_dir = std::env::current_dir()
            .map_err(|e| ResearchError::io_error(format!("Failed to get current directory: {}", e)))?
            .join("exports");

        // Ensure directories exist
        fs::create_dir_all(&temp_dir)
            .map_err(|e| ResearchError::io_error(format!("Failed to create temp directory: {}", e)))?;
        fs::create_dir_all(&output_dir)
            .map_err(|e| ResearchError::io_error(format!("Failed to create output directory: {}", e)))?;

        let engine = Self {
            temp_dir,
            output_dir,
        };

        info!("Export engine initialized successfully");
        Ok(engine)
    }

    /// Export workflows according to the request
    pub async fn export_workflows(
        &self,
        workflows: &[ResearchWorkflow],
        request: &ExportRequest,
    ) -> AppResult<ExportResult> {
        info!("Processing export request: {}", request.id);

        let export_dir = self.create_export_directory(request).await?;
        let mut exported_files = Vec::new();
        let mut total_size = 0u64;

        // Export each workflow
        for workflow in workflows {
            let workflow_files = self.export_single_workflow(workflow, request, &export_dir).await?;
            for file in workflow_files {
                total_size += file.size_bytes;
                exported_files.push(file);
            }
        }

        // Create package if requested
        let final_files = match request.options.package_type {
            PackageType::Archive => {
                self.create_archive(&exported_files, &export_dir, request).await?
            }
            PackageType::Manifest => {
                self.create_manifest(&exported_files, &export_dir, request).await?;
                exported_files
            }
            _ => exported_files,
        };

        // Apply compression if requested
        let compressed_files = if !matches!(request.options.compression, CompressionType::None) {
            self.apply_compression(&final_files, &export_dir, request).await?
        } else {
            final_files
        };

        // Move to final destination
        self.move_to_destination(&compressed_files, request).await?;

        Ok(ExportResult {
            id: Uuid::new_v4(),
            request_id: request.id,
            status: ExportStatus::Completed,
            exported_files: compressed_files,
            destination: request.destination.clone(),
            total_size_bytes: total_size,
            export_time_ms: 0, // Will be set by caller
            created_at: Utc::now(),
            completed_at: None, // Will be set by caller
            error_message: None,
            metadata: request.metadata.clone(),
        })
    }

    /// Create export directory for the request
    async fn create_export_directory(&self, request: &ExportRequest) -> AppResult<PathBuf> {
        let dir_name = format!("export_{}", request.id);
        let export_dir = self.temp_dir.join(dir_name);

        fs::create_dir_all(&export_dir)
            .map_err(|e| ResearchError::io_error(format!("Failed to create export directory: {}", e)))?;

        Ok(export_dir)
    }

    /// Export a single workflow
    async fn export_single_workflow(
        &self,
        workflow: &ResearchWorkflow,
        request: &ExportRequest,
        export_dir: &Path,
    ) -> AppResult<Vec<ExportedFile>> {
        debug!("Exporting workflow: {}", workflow.id);

        let mut files = Vec::new();
        let workflow_dir = export_dir.join(format!("workflow_{}", workflow.id));
        fs::create_dir_all(&workflow_dir)
            .map_err(|e| ResearchError::io_error(format!("Failed to create workflow directory: {}", e)))?;

        // Export in requested formats
        for format in &request.options.formats {
            let file = self.export_workflow_format(workflow, format, &workflow_dir, request).await?;
            files.push(file);
        }

        // Export charts if requested
        if request.options.include_charts {
            let chart_files = self.export_workflow_charts(workflow, &workflow_dir, request).await?;
            files.extend(chart_files);
        }

        // Export metadata if requested
        if request.options.include_metadata {
            let metadata_file = self.export_workflow_metadata(workflow, &workflow_dir, request).await?;
            files.push(metadata_file);
        }

        Ok(files)
    }

    /// Export workflow in a specific format
    async fn export_workflow_format(
        &self,
        workflow: &ResearchWorkflow,
        format: &super::super::OutputFormat,
        workflow_dir: &Path,
        request: &ExportRequest,
    ) -> AppResult<ExportedFile> {
        debug!("Exporting workflow {} in format: {}", workflow.id, format);

        // Generate filename
        let filename = self.generate_filename(workflow, format.to_string().as_str(), request);
        let file_path = workflow_dir.join(&filename);

        // For now, create a simple placeholder file
        // In a real implementation, this would use the OutputFormatter
        let content = format!(
            "# Research Report: {}\n\nWorkflow ID: {}\nFormat: {}\nGenerated: {}\n\nQuery: {}\nStatus: {:?}\n\n## Steps\n\n{}\n",
            workflow.name,
            workflow.id,
            format,
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            workflow.query,
            workflow.status,
            workflow.steps.iter()
                .enumerate()
                .map(|(i, step)| format!("{}. {} - {:?}", i + 1, step.step_type, step.status))
                .collect::<Vec<_>>()
                .join("\n")
        );

        fs::write(&file_path, content)
            .map_err(|e| ResearchError::io_error(format!("Failed to write file: {}", e)))?;

        let file_size = fs::metadata(&file_path)
            .map_err(|e| ResearchError::io_error(format!("Failed to get file metadata: {}", e)))?
            .len();

        Ok(ExportedFile {
            name: filename,
            path: file_path.to_string_lossy().to_string(),
            format: format.to_string(),
            size_bytes: file_size,
            checksum: self.calculate_checksum(&file_path).await?,
            created_at: Utc::now(),
        })
    }

    /// Export workflow charts
    async fn export_workflow_charts(
        &self,
        workflow: &ResearchWorkflow,
        workflow_dir: &Path,
        request: &ExportRequest,
    ) -> AppResult<Vec<ExportedFile>> {
        debug!("Exporting charts for workflow: {}", workflow.id);

        let mut files = Vec::new();
        let charts_dir = workflow_dir.join("charts");
        fs::create_dir_all(&charts_dir)
            .map_err(|e| ResearchError::io_error(format!("Failed to create charts directory: {}", e)))?;

        // For each requested chart format
        for chart_format in &request.options.chart_formats {
            // Create sample chart files
            let chart_types = vec!["timeline", "progress", "status_distribution"];
            
            for chart_type in chart_types {
                let filename = format!("{}_{}.{}", chart_type, workflow.id, chart_format.to_string().to_lowercase());
                let file_path = charts_dir.join(&filename);

                // Create placeholder chart content
                let content = match chart_format {
                    super::super::ChartOutputFormat::SVG => {
                        format!(r#"<svg width="400" height="300" xmlns="http://www.w3.org/2000/svg">
                            <rect width="100%" height="100%" fill="#f8f9fa"/>
                            <text x="200" y="150" text-anchor="middle" font-family="Arial" font-size="16">
                                {} Chart for {}
                            </text>
                        </svg>"#, chart_type, workflow.name)
                    }
                    super::super::ChartOutputFormat::HTML => {
                        format!(r#"<!DOCTYPE html>
                        <html><head><title>{} Chart</title></head>
                        <body><h1>{} Chart for {}</h1>
                        <p>Chart placeholder for workflow: {}</p>
                        </body></html>"#, chart_type, chart_type, workflow.name, workflow.id)
                    }
                    _ => format!("Chart placeholder: {} for workflow {}", chart_type, workflow.id),
                };

                fs::write(&file_path, content)
                    .map_err(|e| ResearchError::io_error(format!("Failed to write chart file: {}", e)))?;

                let file_size = fs::metadata(&file_path)
                    .map_err(|e| ResearchError::io_error(format!("Failed to get chart file metadata: {}", e)))?
                    .len();

                files.push(ExportedFile {
                    name: filename,
                    path: file_path.to_string_lossy().to_string(),
                    format: format!("chart_{}", chart_format.to_string().to_lowercase()),
                    size_bytes: file_size,
                    checksum: self.calculate_checksum(&file_path).await?,
                    created_at: Utc::now(),
                });
            }
        }

        Ok(files)
    }

    /// Export workflow metadata
    async fn export_workflow_metadata(
        &self,
        workflow: &ResearchWorkflow,
        workflow_dir: &Path,
        request: &ExportRequest,
    ) -> AppResult<ExportedFile> {
        debug!("Exporting metadata for workflow: {}", workflow.id);

        let filename = format!("metadata_{}.json", workflow.id);
        let file_path = workflow_dir.join(&filename);

        let metadata = serde_json::json!({
            "workflow_id": workflow.id,
            "name": workflow.name,
            "query": workflow.query,
            "status": workflow.status,
            "created_at": workflow.created_at,
            "updated_at": workflow.updated_at,
            "started_at": workflow.started_at,
            "completed_at": workflow.completed_at,
            "steps_count": workflow.steps.len(),
            "export_request_id": request.id,
            "export_timestamp": Utc::now(),
            "export_options": request.options
        });

        let content = serde_json::to_string_pretty(&metadata)
            .map_err(|e| ResearchError::serialization_error(format!("Failed to serialize metadata: {}", e)))?;

        fs::write(&file_path, content)
            .map_err(|e| ResearchError::io_error(format!("Failed to write metadata file: {}", e)))?;

        let file_size = fs::metadata(&file_path)
            .map_err(|e| ResearchError::io_error(format!("Failed to get metadata file size: {}", e)))?
            .len();

        Ok(ExportedFile {
            name: filename,
            path: file_path.to_string_lossy().to_string(),
            format: "json".to_string(),
            size_bytes: file_size,
            checksum: self.calculate_checksum(&file_path).await?,
            created_at: Utc::now(),
        })
    }

    /// Generate filename based on naming configuration
    fn generate_filename(&self, workflow: &ResearchWorkflow, format: &str, request: &ExportRequest) -> String {
        let mut filename = request.options.file_naming.pattern.clone();
        
        // Replace variables
        filename = filename.replace("{workflow_name}", &workflow.name.replace(" ", "_"));
        filename = filename.replace("{workflow_id}", &workflow.id.to_string());
        filename = filename.replace("{timestamp}", &Utc::now().format("%Y%m%d_%H%M%S").to_string());
        filename = filename.replace("{format}", format);

        // Add prefix and suffix
        if let Some(prefix) = &request.options.file_naming.custom_prefix {
            filename = format!("{}_{}", prefix, filename);
        }
        if let Some(suffix) = &request.options.file_naming.custom_suffix {
            filename = format!("{}_{}", filename, suffix);
        }

        // Add extension
        let extension = match format {
            "markdown" => "md",
            "html" => "html",
            "json" => "json",
            _ => "txt",
        };

        format!("{}.{}", filename, extension)
    }

    /// Create archive package
    async fn create_archive(
        &self,
        files: &[ExportedFile],
        export_dir: &Path,
        request: &ExportRequest,
    ) -> AppResult<Vec<ExportedFile>> {
        info!("Creating archive package for export: {}", request.id);

        let archive_name = format!("export_{}.zip", request.id);
        let archive_path = export_dir.join(&archive_name);

        // For now, just create a placeholder archive file
        // In a real implementation, this would use a ZIP library
        let archive_content = format!(
            "Archive created: {}\nFiles included: {}\nTotal size: {} bytes\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            files.len(),
            files.iter().map(|f| f.size_bytes).sum::<u64>()
        );

        fs::write(&archive_path, archive_content)
            .map_err(|e| ResearchError::io_error(format!("Failed to create archive: {}", e)))?;

        let file_size = fs::metadata(&archive_path)
            .map_err(|e| ResearchError::io_error(format!("Failed to get archive metadata: {}", e)))?
            .len();

        Ok(vec![ExportedFile {
            name: archive_name,
            path: archive_path.to_string_lossy().to_string(),
            format: "zip".to_string(),
            size_bytes: file_size,
            checksum: self.calculate_checksum(&archive_path).await?,
            created_at: Utc::now(),
        }])
    }

    /// Create manifest file
    async fn create_manifest(
        &self,
        files: &[ExportedFile],
        export_dir: &Path,
        request: &ExportRequest,
    ) -> AppResult<()> {
        info!("Creating manifest for export: {}", request.id);

        let manifest_path = export_dir.join("manifest.json");
        let manifest = serde_json::json!({
            "export_id": request.id,
            "created_at": Utc::now(),
            "files": files,
            "total_files": files.len(),
            "total_size_bytes": files.iter().map(|f| f.size_bytes).sum::<u64>(),
            "export_options": request.options
        });

        let content = serde_json::to_string_pretty(&manifest)
            .map_err(|e| ResearchError::serialization_error(format!("Failed to serialize manifest: {}", e)))?;

        fs::write(&manifest_path, content)
            .map_err(|e| ResearchError::io_error(format!("Failed to write manifest: {}", e)))?;

        Ok(())
    }

    /// Apply compression to files
    async fn apply_compression(
        &self,
        files: &[ExportedFile],
        _export_dir: &Path,
        _request: &ExportRequest,
    ) -> AppResult<Vec<ExportedFile>> {
        // For now, return files as-is
        // In a real implementation, this would compress the files
        info!("Compression not yet implemented, returning files as-is");
        Ok(files.to_vec())
    }

    /// Move files to final destination
    async fn move_to_destination(
        &self,
        _files: &[ExportedFile],
        _request: &ExportRequest,
    ) -> AppResult<()> {
        // For now, files remain in temp directory
        // In a real implementation, this would move files to the specified destination
        info!("File movement to destination not yet implemented");
        Ok(())
    }

    /// Calculate file checksum
    async fn calculate_checksum(&self, _file_path: &Path) -> AppResult<String> {
        // For now, return a placeholder checksum
        // In a real implementation, this would calculate MD5/SHA256
        Ok(format!("checksum_{}", Uuid::new_v4().to_string()[..8].to_string()))
    }
}
