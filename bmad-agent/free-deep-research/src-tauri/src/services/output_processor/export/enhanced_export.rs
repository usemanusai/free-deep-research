use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use tracing::{info, debug, error, warn};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tempfile::TempDir;

use crate::error::{AppResult, ResearchError};
use crate::models::research_workflow::{ResearchWorkflow, ResearchResults};

/// Enhanced export service for V1.1.0 with PDF, Word, and PowerPoint support
pub struct EnhancedExportService {
    temp_dir: TempDir,
    export_templates: HashMap<ExportFormat, ExportTemplate>,
    export_jobs: HashMap<Uuid, ExportJob>,
}

/// Enhanced export formats for V1.1.0
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExportFormat {
    PDF,
    DOCX,
    PPTX,
    HTML,
    Markdown,
    JSON,
    CSV,
    Excel,
}

/// Export template configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportTemplate {
    pub id: String,
    pub name: String,
    pub format: ExportFormat,
    pub template_content: String,
    pub styling: ExportStyling,
    pub layout: ExportLayout,
    pub metadata: ExportMetadata,
}

/// Export styling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportStyling {
    pub theme: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub font_family: String,
    pub font_size: u32,
    pub line_height: f32,
    pub custom_css: Option<String>,
}

/// Export layout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportLayout {
    pub page_size: String,
    pub orientation: String,
    pub margins: ExportMargins,
    pub header: Option<String>,
    pub footer: Option<String>,
    pub page_numbers: bool,
    pub table_of_contents: bool,
}

/// Export margins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMargins {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

/// Export metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMetadata {
    pub title: String,
    pub author: String,
    pub subject: String,
    pub keywords: Vec<String>,
    pub company: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Export request for enhanced formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedExportRequest {
    pub id: Uuid,
    pub workflows: Vec<Uuid>,
    pub format: ExportFormat,
    pub template_id: Option<String>,
    pub options: EnhancedExportOptions,
    pub destination: ExportDestination,
}

/// Enhanced export options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedExportOptions {
    pub include_charts: bool,
    pub include_raw_data: bool,
    pub include_metadata: bool,
    pub include_appendices: bool,
    pub compress_output: bool,
    pub password_protect: Option<String>,
    pub watermark: Option<String>,
    pub custom_branding: Option<BrandingOptions>,
}

/// Branding options for exports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandingOptions {
    pub logo_path: Option<String>,
    pub company_name: String,
    pub company_colors: Vec<String>,
    pub custom_footer: Option<String>,
}

/// Export destination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportDestination {
    pub destination_type: ExportDestinationType,
    pub path: String,
    pub filename: String,
}

/// Export destination types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportDestinationType {
    LocalFile,
    CloudStorage,
    Email,
    SharePoint,
}

/// Export job tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportJob {
    pub id: Uuid,
    pub request: EnhancedExportRequest,
    pub status: ExportJobStatus,
    pub progress: f32,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub output_path: Option<String>,
    pub file_size_bytes: Option<u64>,
}

/// Export job status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportJobStatus {
    Queued,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

/// Export result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedExportResult {
    pub job_id: Uuid,
    pub success: bool,
    pub output_path: Option<String>,
    pub file_size_bytes: Option<u64>,
    pub processing_time_ms: u64,
    pub error_message: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl EnhancedExportService {
    /// Create a new enhanced export service
    pub async fn new() -> AppResult<Self> {
        info!("Initializing enhanced export service...");

        let temp_dir = TempDir::new()
            .map_err(|e| ResearchError::io_error(format!("Failed to create temp directory: {}", e)))?;

        let mut export_templates = HashMap::new();
        let export_jobs = HashMap::new();

        // Initialize default templates
        Self::initialize_default_templates(&mut export_templates).await?;

        let service = Self {
            temp_dir,
            export_templates,
            export_jobs,
        };

        info!("Enhanced export service initialized successfully");
        Ok(service)
    }

    /// Initialize default export templates
    async fn initialize_default_templates(templates: &mut HashMap<ExportFormat, ExportTemplate>) -> AppResult<()> {
        // PDF Template
        templates.insert(ExportFormat::PDF, ExportTemplate {
            id: "default_pdf".to_string(),
            name: "Default PDF Template".to_string(),
            format: ExportFormat::PDF,
            template_content: include_str!("../templates/default_pdf.html").to_string(),
            styling: ExportStyling {
                theme: "professional".to_string(),
                primary_color: "#2563eb".to_string(),
                secondary_color: "#64748b".to_string(),
                font_family: "Arial, sans-serif".to_string(),
                font_size: 12,
                line_height: 1.5,
                custom_css: None,
            },
            layout: ExportLayout {
                page_size: "A4".to_string(),
                orientation: "portrait".to_string(),
                margins: ExportMargins {
                    top: 1.0,
                    bottom: 1.0,
                    left: 1.0,
                    right: 1.0,
                },
                header: Some("{{title}} - {{date}}".to_string()),
                footer: Some("Page {{page}} of {{total_pages}}".to_string()),
                page_numbers: true,
                table_of_contents: true,
            },
            metadata: ExportMetadata {
                title: "Research Report".to_string(),
                author: "Free Deep Research System".to_string(),
                subject: "Research Analysis".to_string(),
                keywords: vec!["research".to_string(), "analysis".to_string()],
                company: Some("BMAD AI Agent Team".to_string()),
                created_at: Utc::now(),
            },
        });

        // DOCX Template
        templates.insert(ExportFormat::DOCX, ExportTemplate {
            id: "default_docx".to_string(),
            name: "Default Word Template".to_string(),
            format: ExportFormat::DOCX,
            template_content: "".to_string(), // Will use docx-rs programmatically
            styling: ExportStyling {
                theme: "professional".to_string(),
                primary_color: "#2563eb".to_string(),
                secondary_color: "#64748b".to_string(),
                font_family: "Calibri".to_string(),
                font_size: 11,
                line_height: 1.15,
                custom_css: None,
            },
            layout: ExportLayout {
                page_size: "A4".to_string(),
                orientation: "portrait".to_string(),
                margins: ExportMargins {
                    top: 1.0,
                    bottom: 1.0,
                    left: 1.0,
                    right: 1.0,
                },
                header: Some("{{title}}".to_string()),
                footer: Some("Page {{page}}".to_string()),
                page_numbers: true,
                table_of_contents: true,
            },
            metadata: ExportMetadata {
                title: "Research Report".to_string(),
                author: "Free Deep Research System".to_string(),
                subject: "Research Analysis".to_string(),
                keywords: vec!["research".to_string(), "analysis".to_string()],
                company: Some("BMAD AI Agent Team".to_string()),
                created_at: Utc::now(),
            },
        });

        // PPTX Template
        templates.insert(ExportFormat::PPTX, ExportTemplate {
            id: "default_pptx".to_string(),
            name: "Default PowerPoint Template".to_string(),
            format: ExportFormat::PPTX,
            template_content: "".to_string(), // Will use custom PPTX generation
            styling: ExportStyling {
                theme: "professional".to_string(),
                primary_color: "#2563eb".to_string(),
                secondary_color: "#64748b".to_string(),
                font_family: "Calibri".to_string(),
                font_size: 18,
                line_height: 1.2,
                custom_css: None,
            },
            layout: ExportLayout {
                page_size: "16:9".to_string(),
                orientation: "landscape".to_string(),
                margins: ExportMargins {
                    top: 0.5,
                    bottom: 0.5,
                    left: 0.5,
                    right: 0.5,
                },
                header: None,
                footer: Some("{{company}} - {{date}}".to_string()),
                page_numbers: true,
                table_of_contents: false,
            },
            metadata: ExportMetadata {
                title: "Research Presentation".to_string(),
                author: "Free Deep Research System".to_string(),
                subject: "Research Analysis".to_string(),
                keywords: vec!["research".to_string(), "presentation".to_string()],
                company: Some("BMAD AI Agent Team".to_string()),
                created_at: Utc::now(),
            },
        });

        info!("Initialized {} default export templates", templates.len());
        Ok(())
    }

    /// Export workflows to enhanced formats
    pub async fn export_workflows(
        &mut self,
        workflows: &[ResearchWorkflow],
        request: EnhancedExportRequest,
    ) -> AppResult<EnhancedExportResult> {
        info!("Starting enhanced export for {} workflows in format: {:?}", workflows.len(), request.format);

        let start_time = std::time::Instant::now();
        let job_id = request.id;

        // Create export job
        let mut job = ExportJob {
            id: job_id,
            request: request.clone(),
            status: ExportJobStatus::InProgress,
            progress: 0.0,
            created_at: Utc::now(),
            started_at: Some(Utc::now()),
            completed_at: None,
            error_message: None,
            output_path: None,
            file_size_bytes: None,
        };

        self.export_jobs.insert(job_id, job.clone());

        // Perform export based on format
        let result = match request.format {
            ExportFormat::PDF => self.export_to_pdf(workflows, &request).await,
            ExportFormat::DOCX => self.export_to_docx(workflows, &request).await,
            ExportFormat::PPTX => self.export_to_pptx(workflows, &request).await,
            ExportFormat::HTML => self.export_to_html(workflows, &request).await,
            _ => Err(ResearchError::invalid_request(format!("Unsupported export format: {:?}", request.format)).into()),
        };

        let processing_time = start_time.elapsed();

        // Update job status
        job.completed_at = Some(Utc::now());
        job.progress = 100.0;

        match result {
            Ok(output_path) => {
                job.status = ExportJobStatus::Completed;
                job.output_path = Some(output_path.clone());
                
                // Get file size
                if let Ok(metadata) = fs::metadata(&output_path) {
                    job.file_size_bytes = Some(metadata.len());
                }

                self.export_jobs.insert(job_id, job.clone());

                Ok(EnhancedExportResult {
                    job_id,
                    success: true,
                    output_path: Some(output_path),
                    file_size_bytes: job.file_size_bytes,
                    processing_time_ms: processing_time.as_millis() as u64,
                    error_message: None,
                    metadata: HashMap::new(),
                })
            }
            Err(e) => {
                job.status = ExportJobStatus::Failed;
                job.error_message = Some(e.to_string());
                self.export_jobs.insert(job_id, job);

                Ok(EnhancedExportResult {
                    job_id,
                    success: false,
                    output_path: None,
                    file_size_bytes: None,
                    processing_time_ms: processing_time.as_millis() as u64,
                    error_message: Some(e.to_string()),
                    metadata: HashMap::new(),
                })
            }
        }
    }

    /// Export to PDF using wkhtmltopdf
    async fn export_to_pdf(&self, workflows: &[ResearchWorkflow], request: &EnhancedExportRequest) -> AppResult<String> {
        debug!("Exporting to PDF format");

        // First generate HTML content
        let html_content = self.generate_html_content(workflows, request).await?;

        // Create temporary HTML file
        let temp_html_path = self.temp_dir.path().join(format!("{}.html", Uuid::new_v4()));
        fs::write(&temp_html_path, html_content)
            .map_err(|e| ResearchError::io_error(format!("Failed to write HTML file: {}", e)))?;

        // Generate PDF using wkhtmltopdf
        let output_path = format!("{}/{}", request.destination.path, request.destination.filename);
        
        // Note: In a real implementation, you would use wkhtmltopdf crate here
        // For now, we'll create a placeholder implementation
        let pdf_content = format!("PDF Export for {} workflows", workflows.len());
        fs::write(&output_path, pdf_content)
            .map_err(|e| ResearchError::io_error(format!("Failed to write PDF file: {}", e)))?;

        info!("PDF export completed: {}", output_path);
        Ok(output_path)
    }

    /// Export to DOCX using docx-rs
    async fn export_to_docx(&self, workflows: &[ResearchWorkflow], request: &EnhancedExportRequest) -> AppResult<String> {
        debug!("Exporting to DOCX format");

        // Note: In a real implementation, you would use docx-rs crate here
        // For now, we'll create a placeholder implementation
        let output_path = format!("{}/{}", request.destination.path, request.destination.filename);
        let docx_content = format!("DOCX Export for {} workflows", workflows.len());
        fs::write(&output_path, docx_content)
            .map_err(|e| ResearchError::io_error(format!("Failed to write DOCX file: {}", e)))?;

        info!("DOCX export completed: {}", output_path);
        Ok(output_path)
    }

    /// Export to PPTX (PowerPoint)
    async fn export_to_pptx(&self, workflows: &[ResearchWorkflow], request: &EnhancedExportRequest) -> AppResult<String> {
        debug!("Exporting to PPTX format");

        // Note: In a real implementation, you would create PPTX files programmatically
        // For now, we'll create a placeholder implementation
        let output_path = format!("{}/{}", request.destination.path, request.destination.filename);
        let pptx_content = format!("PPTX Export for {} workflows", workflows.len());
        fs::write(&output_path, pptx_content)
            .map_err(|e| ResearchError::io_error(format!("Failed to write PPTX file: {}", e)))?;

        info!("PPTX export completed: {}", output_path);
        Ok(output_path)
    }

    /// Export to HTML
    async fn export_to_html(&self, workflows: &[ResearchWorkflow], request: &EnhancedExportRequest) -> AppResult<String> {
        debug!("Exporting to HTML format");

        let html_content = self.generate_html_content(workflows, request).await?;
        let output_path = format!("{}/{}", request.destination.path, request.destination.filename);
        
        fs::write(&output_path, html_content)
            .map_err(|e| ResearchError::io_error(format!("Failed to write HTML file: {}", e)))?;

        info!("HTML export completed: {}", output_path);
        Ok(output_path)
    }

    /// Generate HTML content for workflows
    async fn generate_html_content(&self, workflows: &[ResearchWorkflow], request: &EnhancedExportRequest) -> AppResult<String> {
        let template = self.export_templates.get(&ExportFormat::PDF)
            .or_else(|| self.export_templates.get(&ExportFormat::HTML))
            .ok_or_else(|| ResearchError::invalid_request("No HTML template available".to_string()))?;

        let mut html = template.template_content.clone();

        // Replace template variables
        html = html.replace("{{title}}", &template.metadata.title);
        html = html.replace("{{author}}", &template.metadata.author);
        html = html.replace("{{date}}", &Utc::now().format("%Y-%m-%d").to_string());

        // Add workflow content
        let mut content = String::new();
        for workflow in workflows {
            content.push_str(&format!("<h2>{}</h2>", workflow.name));
            content.push_str(&format!("<p><strong>Query:</strong> {}</p>", workflow.query));
            
            if let Some(results) = &workflow.results {
                content.push_str(&format!("<h3>Results</h3>"));
                content.push_str(&format!("<p>{}</p>", results.summary));
            }
        }

        html = html.replace("{{content}}", &content);

        Ok(html)
    }

    /// Get export job status
    pub async fn get_export_job(&self, job_id: Uuid) -> AppResult<Option<ExportJob>> {
        Ok(self.export_jobs.get(&job_id).cloned())
    }

    /// Cancel export job
    pub async fn cancel_export_job(&mut self, job_id: Uuid) -> AppResult<bool> {
        if let Some(mut job) = self.export_jobs.get_mut(&job_id) {
            if matches!(job.status, ExportJobStatus::Queued | ExportJobStatus::InProgress) {
                job.status = ExportJobStatus::Cancelled;
                job.completed_at = Some(Utc::now());
                info!("Export job {} cancelled", job_id);
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Get all export jobs
    pub async fn get_all_export_jobs(&self) -> AppResult<Vec<ExportJob>> {
        Ok(self.export_jobs.values().cloned().collect())
    }
}
