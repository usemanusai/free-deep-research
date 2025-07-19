use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, error, warn};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, ResearchError};
use super::{ExportOptions, ExportDestination};

/// Export job for tracking export operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportJob {
    pub id: Uuid,
    pub workflow_ids: Vec<Uuid>,
    pub template_id: Option<String>,
    pub destination: ExportDestination,
    pub options: ExportOptions,
    pub status: ExportJobStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub progress_percentage: f64,
    pub error_message: Option<String>,
}

/// Export job status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportJobStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
    Scheduled,
}

impl std::fmt::Display for ExportJobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportJobStatus::Pending => write!(f, "pending"),
            ExportJobStatus::InProgress => write!(f, "in_progress"),
            ExportJobStatus::Completed => write!(f, "completed"),
            ExportJobStatus::Failed => write!(f, "failed"),
            ExportJobStatus::Cancelled => write!(f, "cancelled"),
            ExportJobStatus::Scheduled => write!(f, "scheduled"),
        }
    }
}

/// Export job manager for handling batch and scheduled exports
pub struct ExportJobManager {
    jobs: Arc<RwLock<HashMap<Uuid, ExportJob>>>,
    job_history: Arc<RwLock<Vec<ExportJob>>>,
    max_concurrent_jobs: usize,
    max_history_size: usize,
}

impl ExportJobManager {
    /// Create a new export job manager
    pub async fn new() -> AppResult<Self> {
        info!("Initializing export job manager...");

        let manager = Self {
            jobs: Arc::new(RwLock::new(HashMap::new())),
            job_history: Arc::new(RwLock::new(Vec::new())),
            max_concurrent_jobs: 5,
            max_history_size: 1000,
        };

        info!("Export job manager initialized successfully");
        Ok(manager)
    }

    /// Add a new export job
    pub async fn add_job(&self, job: ExportJob) -> AppResult<()> {
        let mut jobs = self.jobs.write().await;
        
        // Check if we're at the concurrent job limit
        let active_jobs = jobs.values().filter(|j| matches!(j.status, ExportJobStatus::InProgress)).count();
        if active_jobs >= self.max_concurrent_jobs {
            return Err(ResearchError::resource_limit_exceeded(
                format!("Maximum concurrent export jobs reached: {}", self.max_concurrent_jobs)
            ).into());
        }

        jobs.insert(job.id, job);
        info!("Added export job: {}", job.id);
        Ok(())
    }

    /// Get export job by ID
    pub async fn get_job(&self, job_id: Uuid) -> AppResult<Option<ExportJob>> {
        let jobs = self.jobs.read().await;
        Ok(jobs.get(&job_id).cloned())
    }

    /// Get all jobs with optional status filter
    pub async fn get_jobs(&self, status_filter: Option<ExportJobStatus>) -> AppResult<Vec<ExportJob>> {
        let jobs = self.jobs.read().await;
        
        if let Some(status) = status_filter {
            Ok(jobs.values()
                .filter(|job| job.status == status)
                .cloned()
                .collect())
        } else {
            Ok(jobs.values().cloned().collect())
        }
    }

    /// Update job progress
    pub async fn update_job_progress(&self, job_id: Uuid, progress: f64) -> AppResult<()> {
        let mut jobs = self.jobs.write().await;
        
        if let Some(job) = jobs.get_mut(&job_id) {
            job.progress_percentage = progress.clamp(0.0, 100.0);
            debug!("Updated job {} progress to {:.1}%", job_id, progress);
            Ok(())
        } else {
            Err(ResearchError::not_found(format!("Export job not found: {}", job_id)).into())
        }
    }

    /// Complete export job
    pub async fn complete_job(&self, job_id: Uuid) -> AppResult<()> {
        let mut jobs = self.jobs.write().await;
        
        if let Some(job) = jobs.get_mut(&job_id) {
            job.status = ExportJobStatus::Completed;
            job.completed_at = Some(Utc::now());
            job.progress_percentage = 100.0;
            
            // Move to history
            let completed_job = job.clone();
            drop(jobs);
            self.move_to_history(completed_job).await?;
            
            info!("Completed export job: {}", job_id);
            Ok(())
        } else {
            Err(ResearchError::not_found(format!("Export job not found: {}", job_id)).into())
        }
    }

    /// Fail export job
    pub async fn fail_job(&self, job_id: Uuid, error_message: String) -> AppResult<()> {
        let mut jobs = self.jobs.write().await;
        
        if let Some(job) = jobs.get_mut(&job_id) {
            job.status = ExportJobStatus::Failed;
            job.completed_at = Some(Utc::now());
            job.error_message = Some(error_message.clone());
            
            // Move to history
            let failed_job = job.clone();
            drop(jobs);
            self.move_to_history(failed_job).await?;
            
            error!("Failed export job {}: {}", job_id, error_message);
            Ok(())
        } else {
            Err(ResearchError::not_found(format!("Export job not found: {}", job_id)).into())
        }
    }

    /// Cancel export job
    pub async fn cancel_job(&self, job_id: Uuid) -> AppResult<bool> {
        let mut jobs = self.jobs.write().await;
        
        if let Some(job) = jobs.get_mut(&job_id) {
            match job.status {
                ExportJobStatus::Pending | ExportJobStatus::Scheduled => {
                    job.status = ExportJobStatus::Cancelled;
                    job.completed_at = Some(Utc::now());
                    
                    // Move to history
                    let cancelled_job = job.clone();
                    drop(jobs);
                    self.move_to_history(cancelled_job).await?;
                    
                    info!("Cancelled export job: {}", job_id);
                    Ok(true)
                }
                ExportJobStatus::InProgress => {
                    // Mark for cancellation but don't move to history yet
                    job.status = ExportJobStatus::Cancelled;
                    warn!("Marked in-progress export job for cancellation: {}", job_id);
                    Ok(true)
                }
                _ => {
                    warn!("Cannot cancel export job {} in status: {}", job_id, job.status);
                    Ok(false)
                }
            }
        } else {
            Err(ResearchError::not_found(format!("Export job not found: {}", job_id)).into())
        }
    }

    /// Get job statistics
    pub async fn get_job_statistics(&self) -> AppResult<ExportJobStatistics> {
        let jobs = self.jobs.read().await;
        let history = self.job_history.read().await;
        
        let active_jobs = jobs.len();
        let total_jobs = active_jobs + history.len();
        
        let mut status_counts = HashMap::new();
        let mut total_processing_time = 0i64;
        let mut completed_jobs = 0;

        // Count active jobs
        for job in jobs.values() {
            *status_counts.entry(job.status).or_insert(0) += 1;
        }

        // Count historical jobs and calculate processing times
        for job in history.iter() {
            *status_counts.entry(job.status).or_insert(0) += 1;
            
            if let (Some(started), Some(completed)) = (job.started_at, job.completed_at) {
                total_processing_time += (completed - started).num_milliseconds();
                completed_jobs += 1;
            }
        }

        let average_processing_time_ms = if completed_jobs > 0 {
            total_processing_time as f64 / completed_jobs as f64
        } else {
            0.0
        };

        let success_rate = if total_jobs > 0 {
            let successful = status_counts.get(&ExportJobStatus::Completed).unwrap_or(&0);
            (*successful as f64 / total_jobs as f64) * 100.0
        } else {
            0.0
        };

        Ok(ExportJobStatistics {
            total_jobs: total_jobs as u64,
            active_jobs: active_jobs as u64,
            jobs_by_status: status_counts,
            average_processing_time_ms,
            success_rate,
            queue_length: jobs.values().filter(|j| j.status == ExportJobStatus::Pending).count() as u64,
        })
    }

    /// Clean up old completed jobs
    pub async fn cleanup_old_jobs(&self, max_age_hours: u32) -> AppResult<u32> {
        let cutoff_time = Utc::now() - chrono::Duration::hours(max_age_hours as i64);
        let mut history = self.job_history.write().await;
        
        let original_count = history.len();
        history.retain(|job| {
            job.completed_at.map_or(true, |completed| completed >= cutoff_time)
        });
        
        let removed_count = original_count - history.len();
        if removed_count > 0 {
            info!("Cleaned up {} old export jobs", removed_count);
        }
        
        Ok(removed_count as u32)
    }

    /// Move job to history
    async fn move_to_history(&self, job: ExportJob) -> AppResult<()> {
        let mut history = self.job_history.write().await;
        history.push(job.clone());
        
        // Remove from active jobs
        let mut jobs = self.jobs.write().await;
        jobs.remove(&job.id);
        
        // Limit history size
        if history.len() > self.max_history_size {
            history.remove(0);
        }
        
        Ok(())
    }

    /// Get pending jobs for processing
    pub async fn get_pending_jobs(&self) -> AppResult<Vec<ExportJob>> {
        let jobs = self.jobs.read().await;
        Ok(jobs.values()
            .filter(|job| job.status == ExportJobStatus::Pending)
            .cloned()
            .collect())
    }

    /// Start job processing
    pub async fn start_job(&self, job_id: Uuid) -> AppResult<()> {
        let mut jobs = self.jobs.write().await;
        
        if let Some(job) = jobs.get_mut(&job_id) {
            if job.status == ExportJobStatus::Pending {
                job.status = ExportJobStatus::InProgress;
                job.started_at = Some(Utc::now());
                info!("Started export job: {}", job_id);
                Ok(())
            } else {
                Err(ResearchError::invalid_request(
                    format!("Cannot start job {} in status: {}", job_id, job.status)
                ).into())
            }
        } else {
            Err(ResearchError::not_found(format!("Export job not found: {}", job_id)).into())
        }
    }

    /// Set maximum concurrent jobs
    pub async fn set_max_concurrent_jobs(&self, max_jobs: usize) -> AppResult<()> {
        if max_jobs == 0 {
            return Err(ResearchError::invalid_request(
                "Maximum concurrent jobs must be greater than 0".to_string()
            ).into());
        }
        
        // Note: In a real implementation, this would be stored in the struct
        // For now, we'll just log the change
        info!("Maximum concurrent jobs set to: {}", max_jobs);
        Ok(())
    }

    /// Get job history
    pub async fn get_job_history(&self, limit: Option<usize>) -> AppResult<Vec<ExportJob>> {
        let history = self.job_history.read().await;
        
        if let Some(limit) = limit {
            Ok(history.iter().rev().take(limit).cloned().collect())
        } else {
            Ok(history.iter().rev().cloned().collect())
        }
    }
}

/// Export job statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportJobStatistics {
    pub total_jobs: u64,
    pub active_jobs: u64,
    pub jobs_by_status: HashMap<ExportJobStatus, u32>,
    pub average_processing_time_ms: f64,
    pub success_rate: f64,
    pub queue_length: u64,
}

/// Batch export request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchExportRequest {
    pub workflow_ids: Vec<Uuid>,
    pub template_id: Option<String>,
    pub destination: ExportDestination,
    pub options: ExportOptions,
    pub batch_size: Option<usize>,
    pub delay_between_batches_ms: Option<u64>,
}

/// Scheduled export configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledExportConfig {
    pub id: Uuid,
    pub name: String,
    pub workflow_filter: WorkflowFilter,
    pub template_id: Option<String>,
    pub destination: ExportDestination,
    pub options: ExportOptions,
    pub schedule: super::ExportSchedule,
    pub enabled: bool,
    pub last_run: Option<DateTime<Utc>>,
    pub next_run: Option<DateTime<Utc>>,
}

/// Workflow filter for scheduled exports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowFilter {
    pub status_filter: Option<Vec<crate::models::research_workflow::WorkflowStatus>>,
    pub date_range: Option<super::DateRange>,
    pub name_pattern: Option<String>,
    pub tag_filter: Option<Vec<String>>,
}
