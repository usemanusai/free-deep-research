use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{info, debug, error};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::error::{AppResult, MonitoringError};
use crate::services::{Service, DataPersistenceService};

pub mod metrics_collector;
pub mod health_checker;
pub mod alert_manager;

/// Monitoring Service that tracks system health and performance
pub struct MonitoringService {
    data_persistence: Arc<RwLock<DataPersistenceService>>,
    current_metrics: Arc<RwLock<SystemMetrics>>,
    monitoring_enabled: Arc<RwLock<bool>>,
    background_tasks_running: Arc<std::sync::atomic::AtomicBool>,
}

/// System metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: DateTime<Utc>,
    pub cpu_usage_percent: f64,
    pub memory_usage_percent: f64,
    pub disk_usage_percent: f64,
    pub network_active: bool,
    pub active_workflows: u32,
    pub queue_length: u32,
    pub api_response_times: HashMap<String, u32>,
    pub error_count_last_hour: u32,
    pub uptime_seconds: u64,
    pub system_errors: Option<u32>,
    pub network_errors: Option<u32>,
}

/// System health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub overall_status: HealthLevel,
    pub components: HashMap<String, ComponentHealth>,
    pub last_check: DateTime<Utc>,
    pub uptime_seconds: u64,
}

/// Health level indicators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthLevel {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

/// Component health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub status: HealthLevel,
    pub message: String,
    pub last_check: DateTime<Utc>,
    pub response_time_ms: Option<u32>,
}

impl MonitoringService {
    /// Create a new monitoring service
    pub async fn new(data_persistence: Arc<RwLock<DataPersistenceService>>) -> AppResult<Self> {
        info!("Initializing monitoring service...");

        let current_metrics = Arc::new(RwLock::new(SystemMetrics {
            timestamp: Utc::now(),
            cpu_usage_percent: 0.0,
            memory_usage_percent: 0.0,
            disk_usage_percent: 0.0,
            network_active: true,
            active_workflows: 0,
            queue_length: 0,
            api_response_times: HashMap::new(),
            error_count_last_hour: 0,
            uptime_seconds: 0,
            system_errors: Some(0),
            network_errors: Some(0),
        }));

        let service = Self {
            data_persistence,
            current_metrics,
            monitoring_enabled: Arc::new(RwLock::new(false)),
            background_tasks_running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        };

        info!("Monitoring service initialized successfully");
        Ok(service)
    }

    /// Start monitoring
    pub async fn start_monitoring(&self) -> AppResult<()> {
        info!("Starting monitoring...");

        {
            let mut enabled = self.monitoring_enabled.write().await;
            *enabled = true;
        }

        // Start metrics collection task
        let current_metrics = self.current_metrics.clone();
        let monitoring_enabled = self.monitoring_enabled.clone();
        let background_tasks_running = self.background_tasks_running.clone();

        // Set background tasks as running
        background_tasks_running.store(true, std::sync::atomic::Ordering::Relaxed);

        tokio::spawn(async move {
            loop {
                {
                    let enabled = monitoring_enabled.read().await;
                    if !*enabled {
                        break;
                    }
                }

                // Collect system metrics
                let mut metrics = current_metrics.write().await;
                metrics.timestamp = Utc::now();
                metrics.uptime_seconds += 30; // Increment by collection interval

                // TODO: Collect actual system metrics
                // For now, simulate some basic metrics
                metrics.cpu_usage_percent = 25.0 + (rand::random::<f64>() * 50.0);
                metrics.memory_usage_percent = 40.0 + (rand::random::<f64>() * 30.0);
                metrics.disk_usage_percent = 60.0 + (rand::random::<f64>() * 20.0);

                drop(metrics);

                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
            }
        });

        info!("Monitoring started successfully");
        Ok(())
    }

    /// Stop monitoring
    pub async fn stop_monitoring(&self) -> AppResult<()> {
        info!("Stopping monitoring...");

        let mut enabled = self.monitoring_enabled.write().await;
        *enabled = false;
        drop(enabled);

        info!("Monitoring stopped");
        Ok(())
    }

    /// Get current system metrics
    pub async fn get_current_metrics(&self) -> AppResult<SystemMetrics> {
        let metrics = self.current_metrics.read().await;
        Ok(metrics.clone())
    }

    /// Get system health status
    pub async fn get_health_status(&self) -> AppResult<HealthStatus> {
        let metrics = self.get_current_metrics().await?;

        let mut components = HashMap::new();

        // Check CPU health
        let cpu_status = if metrics.cpu_usage_percent > 90.0 {
            HealthLevel::Critical
        } else if metrics.cpu_usage_percent > 70.0 {
            HealthLevel::Warning
        } else {
            HealthLevel::Healthy
        };

        components.insert("cpu".to_string(), ComponentHealth {
            status: cpu_status,
            message: format!("CPU usage: {:.1}%", metrics.cpu_usage_percent),
            last_check: metrics.timestamp,
            response_time_ms: None,
        });

        // Check memory health
        let memory_status = if metrics.memory_usage_percent > 90.0 {
            HealthLevel::Critical
        } else if metrics.memory_usage_percent > 80.0 {
            HealthLevel::Warning
        } else {
            HealthLevel::Healthy
        };

        components.insert("memory".to_string(), ComponentHealth {
            status: memory_status,
            message: format!("Memory usage: {:.1}%", metrics.memory_usage_percent),
            last_check: metrics.timestamp,
            response_time_ms: None,
        });

        // Determine overall status
        let overall_status = components.values()
            .map(|c| c.status)
            .max()
            .unwrap_or(HealthLevel::Unknown);

        Ok(HealthStatus {
            overall_status,
            components,
            last_check: metrics.timestamp,
            uptime_seconds: metrics.uptime_seconds,
        })
    }

    /// Record API response time
    pub async fn record_api_response_time(&self, service: &str, response_time_ms: u32) -> AppResult<()> {
        debug!("Recording API response time: {} = {}ms", service, response_time_ms);

        let mut metrics = self.current_metrics.write().await;
        metrics.api_response_times.insert(service.to_string(), response_time_ms);
        metrics.timestamp = Utc::now();

        Ok(())
    }

    /// Update workflow metrics
    pub async fn update_workflow_metrics(&self, active_count: u32, queue_length: u32) -> AppResult<()> {
        debug!("Updating workflow metrics: active={}, queue={}", active_count, queue_length);

        let mut metrics = self.current_metrics.write().await;
        metrics.active_workflows = active_count;
        metrics.queue_length = queue_length;
        metrics.timestamp = Utc::now();

        Ok(())
    }

    /// Get current metrics
    pub async fn get_current_metrics(&self) -> AppResult<SystemMetrics> {
        let metrics = self.current_metrics.read().await;
        Ok(metrics.clone())
    }

    /// Log audit event
    pub async fn log_audit_event(&self, event_type: String, description: String, resource_id: Option<String>) -> AppResult<()> {
        debug!("Logging audit event: {} - {}", event_type, description);

        // Create audit event
        let audit_event = crate::models::audit::AuditEvent {
            id: uuid::Uuid::new_v4(),
            event_type,
            description,
            resource_id,
            user_id: None, // TODO: Add user context when authentication is implemented
            timestamp: chrono::Utc::now(),
            severity: crate::models::audit::AuditSeverity::Info,
            metadata: std::collections::HashMap::new(),
        };

        // Store audit event through data persistence
        let data_persistence = self.data_persistence.read().await;
        if let Err(e) = data_persistence.store_audit_event(&audit_event).await {
            error!("Failed to store audit event: {}", e);
            return Err(e);
        }
        drop(data_persistence);

        debug!("Audit event logged successfully: {}", audit_event.id);
        Ok(())
    }
}

#[async_trait::async_trait]
impl Service for MonitoringService {
    async fn health_check(&self) -> AppResult<()> {
        debug!("Performing monitoring service health check");

        // Check if metrics collection is working
        let metrics = self.current_metrics.read().await;
        let last_update = metrics.timestamp;
        let now = Utc::now();

        // Check if metrics are recent (within last 5 minutes)
        let time_diff = now.signed_duration_since(last_update).num_minutes();
        if time_diff > 5 {
            return Err(crate::error::MonitoringError::stale_metrics(
                format!("Metrics are {} minutes old", time_diff)
            ).into());
        }

        // Check if background tasks are running
        if !self.background_tasks_running.load(std::sync::atomic::Ordering::Relaxed) {
            return Err(crate::error::MonitoringError::background_task_failure(
                "Background monitoring tasks are not running".to_string()
            ).into());
        }

        debug!("Monitoring service health check passed");
        Ok(())
    }
    
    async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down monitoring service...");

        // Stop background tasks
        self.background_tasks_running.store(false, std::sync::atomic::Ordering::Relaxed);

        // Wait a moment for tasks to finish
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        // Save final metrics snapshot
        let metrics = self.current_metrics.read().await;
        debug!("Final metrics snapshot: active_workflows={}, queue_length={}",
               metrics.active_workflows, metrics.queue_length);

        info!("Monitoring service shutdown completed");
        Ok(())
    }
}
