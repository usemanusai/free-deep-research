use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, error, warn};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, ResearchError};
use crate::models::research_workflow::ResearchWorkflow;

pub mod export_engine;
pub mod export_templates;
pub mod export_destinations;
pub mod export_jobs;

use self::export_engine::ExportEngine;
use self::export_templates::{ExportTemplate, ExportTemplateManager};
use self::export_destinations::{ExportDestination, ExportDestinationType};
use self::export_jobs::{ExportJob, ExportJobManager, ExportJobStatus};

/// Export service for research workflow results
pub struct ExportService {
    export_engine: Arc<ExportEngine>,
    template_manager: Arc<RwLock<ExportTemplateManager>>,
    job_manager: Arc<RwLock<ExportJobManager>>,
    export_history: Arc<RwLock<Vec<ExportResult>>>,
}

/// Export request configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub id: Uuid,
    pub workflow_ids: Vec<Uuid>,
    pub template_id: Option<String>,
    pub destination: ExportDestination,
    pub options: ExportOptions,
    pub schedule: Option<ExportSchedule>,
    pub metadata: HashMap<String, String>,
}

/// Export options and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    pub formats: Vec<super::OutputFormat>,
    pub include_charts: bool,
    pub chart_formats: Vec<super::ChartOutputFormat>,
    pub compression: CompressionType,
    pub encryption: Option<EncryptionConfig>,
    pub file_naming: FileNamingConfig,
    pub package_type: PackageType,
    pub include_metadata: bool,
    pub include_raw_data: bool,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            formats: vec![super::OutputFormat::Markdown, super::OutputFormat::JSON],
            include_charts: true,
            chart_formats: vec![super::ChartOutputFormat::SVG],
            compression: CompressionType::None,
            encryption: None,
            file_naming: FileNamingConfig::default(),
            package_type: PackageType::Folder,
            include_metadata: true,
            include_raw_data: false,
        }
    }
}

/// Compression types for exports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionType {
    None,
    Zip,
    Tar,
    TarGz,
    TarBz2,
}

/// Encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    pub algorithm: EncryptionAlgorithm,
    pub key_id: String,
    pub password_protected: bool,
}

/// Encryption algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    AES256,
    ChaCha20,
    RSA,
}

/// File naming configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileNamingConfig {
    pub pattern: String,
    pub include_timestamp: bool,
    pub include_workflow_id: bool,
    pub custom_prefix: Option<String>,
    pub custom_suffix: Option<String>,
}

impl Default for FileNamingConfig {
    fn default() -> Self {
        Self {
            pattern: "{workflow_name}_{timestamp}".to_string(),
            include_timestamp: true,
            include_workflow_id: false,
            custom_prefix: None,
            custom_suffix: None,
        }
    }
}

/// Package types for exports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageType {
    Folder,
    Archive,
    SingleFile,
    Manifest,
}

/// Export scheduling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSchedule {
    pub schedule_type: ScheduleType,
    pub interval: Option<ScheduleInterval>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub timezone: String,
}

/// Schedule types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduleType {
    Once,
    Recurring,
    OnCompletion,
    OnFailure,
}

/// Schedule intervals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduleInterval {
    Minutes(u32),
    Hours(u32),
    Days(u32),
    Weeks(u32),
    Months(u32),
    Cron(String),
}

/// Export result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    pub id: Uuid,
    pub request_id: Uuid,
    pub status: ExportStatus,
    pub exported_files: Vec<ExportedFile>,
    pub destination: ExportDestination,
    pub total_size_bytes: u64,
    pub export_time_ms: u64,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Export status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
    PartiallyCompleted,
}

/// Exported file information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedFile {
    pub name: String,
    pub path: String,
    pub format: String,
    pub size_bytes: u64,
    pub checksum: String,
    pub created_at: DateTime<Utc>,
}

/// Export statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportStatistics {
    pub total_exports: u64,
    pub successful_exports: u64,
    pub failed_exports: u64,
    pub exports_by_format: HashMap<String, u64>,
    pub exports_by_destination: HashMap<ExportDestinationType, u64>,
    pub average_export_time_ms: f64,
    pub total_exported_size_bytes: u64,
    pub most_used_templates: Vec<String>,
}

impl ExportService {
    /// Create a new export service
    pub async fn new() -> AppResult<Self> {
        info!("Initializing export service...");

        let export_engine = Arc::new(ExportEngine::new().await?);
        let template_manager = Arc::new(RwLock::new(ExportTemplateManager::new().await?));
        let job_manager = Arc::new(RwLock::new(ExportJobManager::new().await?));
        let export_history = Arc::new(RwLock::new(Vec::new()));

        let service = Self {
            export_engine,
            template_manager,
            job_manager,
            export_history,
        };

        info!("Export service initialized successfully");
        Ok(service)
    }

    /// Export workflows using a template
    pub async fn export_workflows(
        &self,
        workflows: &[ResearchWorkflow],
        request: ExportRequest,
    ) -> AppResult<ExportResult> {
        info!("Exporting {} workflows with request: {}", workflows.len(), request.id);

        let start_time = std::time::Instant::now();

        // Create export job
        let export_job = ExportJob {
            id: request.id,
            workflow_ids: request.workflow_ids.clone(),
            template_id: request.template_id.clone(),
            destination: request.destination.clone(),
            options: request.options.clone(),
            status: ExportJobStatus::InProgress,
            created_at: Utc::now(),
            started_at: Some(Utc::now()),
            completed_at: None,
            progress_percentage: 0.0,
            error_message: None,
        };

        // Register job
        {
            let mut job_manager = self.job_manager.write().await;
            job_manager.add_job(export_job).await?;
        }

        // Perform export
        let result = self.export_engine.export_workflows(workflows, &request).await;

        let export_time = start_time.elapsed();

        // Update job status
        {
            let mut job_manager = self.job_manager.write().await;
            match &result {
                Ok(_) => {
                    job_manager.complete_job(request.id).await?;
                }
                Err(e) => {
                    job_manager.fail_job(request.id, e.to_string()).await?;
                }
            }
        }

        match result {
            Ok(mut export_result) => {
                export_result.export_time_ms = export_time.as_millis() as u64;
                export_result.completed_at = Some(Utc::now());

                // Store in history
                {
                    let mut history = self.export_history.write().await;
                    history.push(export_result.clone());

                    // Keep only last 1000 exports
                    if history.len() > 1000 {
                        history.remove(0);
                    }
                }

                info!("Export completed successfully in {}ms", export_time.as_millis());
                Ok(export_result)
            }
            Err(e) => {
                error!("Export failed: {}", e);
                Err(e)
            }
        }
    }

    /// Get export templates
    pub async fn get_export_templates(&self) -> AppResult<Vec<ExportTemplate>> {
        let template_manager = self.template_manager.read().await;
        template_manager.get_all_templates().await
    }

    /// Create export template
    pub async fn create_export_template(&self, template: ExportTemplate) -> AppResult<()> {
        let template_manager = self.template_manager.read().await;
        template_manager.create_template(template).await
    }

    /// Get export jobs
    pub async fn get_export_jobs(&self, status_filter: Option<ExportJobStatus>) -> AppResult<Vec<ExportJob>> {
        let job_manager = self.job_manager.read().await;
        job_manager.get_jobs(status_filter).await
    }

    /// Cancel export job
    pub async fn cancel_export_job(&self, job_id: Uuid) -> AppResult<bool> {
        let mut job_manager = self.job_manager.write().await;
        job_manager.cancel_job(job_id).await
    }

    /// Get export statistics
    pub async fn get_export_statistics(&self) -> AppResult<ExportStatistics> {
        let history = self.export_history.read().await;
        
        let total_exports = history.len() as u64;
        let successful_exports = history.iter().filter(|e| matches!(e.status, ExportStatus::Completed)).count() as u64;
        let failed_exports = history.iter().filter(|e| matches!(e.status, ExportStatus::Failed)).count() as u64;

        let mut exports_by_format = HashMap::new();
        let mut exports_by_destination = HashMap::new();
        let mut total_export_time = 0u64;
        let mut total_size = 0u64;

        for export in history.iter() {
            for file in &export.exported_files {
                *exports_by_format.entry(file.format.clone()).or_insert(0) += 1;
            }
            *exports_by_destination.entry(export.destination.destination_type).or_insert(0) += 1;
            total_export_time += export.export_time_ms;
            total_size += export.total_size_bytes;
        }

        let average_export_time = if total_exports > 0 {
            total_export_time as f64 / total_exports as f64
        } else {
            0.0
        };

        Ok(ExportStatistics {
            total_exports,
            successful_exports,
            failed_exports,
            exports_by_format,
            exports_by_destination,
            average_export_time_ms: average_export_time,
            total_exported_size_bytes: total_size,
            most_used_templates: Vec::new(), // TODO: Track template usage
        })
    }
}

// Re-export types for external use
pub use export_engine::ExportEngine;
pub use export_templates::{ExportTemplate, ExportTemplateManager};
pub use export_destinations::{ExportDestination, ExportDestinationType};
pub use export_jobs::{ExportJob, ExportJobManager, ExportJobStatus};
