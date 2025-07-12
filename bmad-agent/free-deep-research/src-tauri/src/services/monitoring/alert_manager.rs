use std::collections::HashMap;
use tracing::{info, debug, warn, error};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::error::{AppResult, MonitoringError};
use super::{SystemAlert, AlertType, AlertSeverity};

/// Alert manager for handling system alerts and notifications
pub struct AlertManager {
    active_alerts: HashMap<Uuid, SystemAlert>,
    alert_history: Vec<SystemAlert>,
    alert_thresholds: AlertThresholds,
    max_history_size: usize,
}

/// Alert thresholds configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub cpu_warning_percent: f64,
    pub cpu_critical_percent: f64,
    pub memory_warning_percent: f64,
    pub memory_critical_percent: f64,
    pub disk_warning_percent: f64,
    pub disk_critical_percent: f64,
    pub api_response_warning_ms: u32,
    pub api_response_critical_ms: u32,
    pub error_rate_warning_per_hour: u32,
    pub error_rate_critical_per_hour: u32,
    pub queue_backlog_warning: u32,
    pub queue_backlog_critical: u32,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            cpu_warning_percent: 70.0,
            cpu_critical_percent: 90.0,
            memory_warning_percent: 80.0,
            memory_critical_percent: 95.0,
            disk_warning_percent: 85.0,
            disk_critical_percent: 95.0,
            api_response_warning_ms: 2000,
            api_response_critical_ms: 5000,
            error_rate_warning_per_hour: 10,
            error_rate_critical_per_hour: 50,
            queue_backlog_warning: 10,
            queue_backlog_critical: 25,
        }
    }
}

impl AlertManager {
    /// Create a new alert manager
    pub async fn new() -> AppResult<Self> {
        info!("Initializing alert manager");
        
        let manager = Self {
            active_alerts: HashMap::new(),
            alert_history: Vec::new(),
            alert_thresholds: AlertThresholds::default(),
            max_history_size: 1000,
        };
        
        info!("Alert manager initialized successfully");
        Ok(manager)
    }
    
    /// Create a new alert
    pub async fn create_alert(
        &mut self,
        alert_type: AlertType,
        severity: AlertSeverity,
        title: String,
        message: String,
    ) -> AppResult<Uuid> {
        let alert_id = Uuid::new_v4();
        
        let alert = SystemAlert {
            id: alert_id,
            alert_type,
            severity,
            title,
            message,
            created_at: Utc::now(),
            acknowledged_at: None,
            resolved_at: None,
        };
        
        // Check if similar alert already exists
        let similar_exists = self.active_alerts.values()
            .any(|a| std::mem::discriminant(&a.alert_type) == std::mem::discriminant(&alert.alert_type));
        
        if !similar_exists {
            info!("Creating new alert: {} - {}", alert.title, alert.message);
            self.active_alerts.insert(alert_id, alert.clone());
            
            // Log alert based on severity
            match severity {
                AlertSeverity::Info => info!("ALERT [INFO]: {}", alert.message),
                AlertSeverity::Warning => warn!("ALERT [WARNING]: {}", alert.message),
                AlertSeverity::Critical => error!("ALERT [CRITICAL]: {}", alert.message),
            }
        } else {
            debug!("Similar alert already exists, skipping creation");
        }
        
        Ok(alert_id)
    }
    
    /// Acknowledge an alert
    pub async fn acknowledge_alert(&mut self, alert_id: Uuid) -> AppResult<()> {
        if let Some(alert) = self.active_alerts.get_mut(&alert_id) {
            alert.acknowledged_at = Some(Utc::now());
            info!("Alert acknowledged: {}", alert.title);
        } else {
            warn!("Attempted to acknowledge non-existent alert: {}", alert_id);
        }
        
        Ok(())
    }
    
    /// Resolve an alert
    pub async fn resolve_alert(&mut self, alert_id: Uuid) -> AppResult<()> {
        if let Some(mut alert) = self.active_alerts.remove(&alert_id) {
            alert.resolved_at = Some(Utc::now());
            info!("Alert resolved: {}", alert.title);
            
            // Move to history
            self.alert_history.push(alert);
            
            // Trim history if needed
            if self.alert_history.len() > self.max_history_size {
                self.alert_history.remove(0);
            }
        } else {
            warn!("Attempted to resolve non-existent alert: {}", alert_id);
        }
        
        Ok(())
    }
    
    /// Get all active alerts
    pub async fn get_active_alerts(&self) -> AppResult<Vec<SystemAlert>> {
        Ok(self.active_alerts.values().cloned().collect())
    }
    
    /// Get alert history
    pub async fn get_alert_history(&self, limit: Option<usize>) -> AppResult<Vec<SystemAlert>> {
        let limit = limit.unwrap_or(50);
        Ok(self.alert_history.iter()
            .rev()
            .take(limit)
            .cloned()
            .collect())
    }
    
    /// Check CPU usage alert
    pub async fn check_cpu_alert(&mut self, cpu_usage: f64) -> AppResult<()> {
        if cpu_usage >= self.alert_thresholds.cpu_critical_percent {
            self.create_alert(
                AlertType::HighCpuUsage,
                AlertSeverity::Critical,
                "Critical CPU Usage".to_string(),
                format!("CPU usage is critically high: {:.1}%", cpu_usage),
            ).await?;
        } else if cpu_usage >= self.alert_thresholds.cpu_warning_percent {
            self.create_alert(
                AlertType::HighCpuUsage,
                AlertSeverity::Warning,
                "High CPU Usage".to_string(),
                format!("CPU usage is high: {:.1}%", cpu_usage),
            ).await?;
        } else {
            // Resolve existing CPU alerts if usage is back to normal
            self.resolve_alerts_by_type(AlertType::HighCpuUsage).await?;
        }
        
        Ok(())
    }
    
    /// Check memory usage alert
    pub async fn check_memory_alert(&mut self, memory_usage: f64) -> AppResult<()> {
        if memory_usage >= self.alert_thresholds.memory_critical_percent {
            self.create_alert(
                AlertType::HighMemoryUsage,
                AlertSeverity::Critical,
                "Critical Memory Usage".to_string(),
                format!("Memory usage is critically high: {:.1}%", memory_usage),
            ).await?;
        } else if memory_usage >= self.alert_thresholds.memory_warning_percent {
            self.create_alert(
                AlertType::HighMemoryUsage,
                AlertSeverity::Warning,
                "High Memory Usage".to_string(),
                format!("Memory usage is high: {:.1}%", memory_usage),
            ).await?;
        } else {
            self.resolve_alerts_by_type(AlertType::HighMemoryUsage).await?;
        }
        
        Ok(())
    }
    
    /// Check disk usage alert
    pub async fn check_disk_alert(&mut self, disk_usage: f64) -> AppResult<()> {
        if disk_usage >= self.alert_thresholds.disk_critical_percent {
            self.create_alert(
                AlertType::HighDiskUsage,
                AlertSeverity::Critical,
                "Critical Disk Usage".to_string(),
                format!("Disk usage is critically high: {:.1}%", disk_usage),
            ).await?;
        } else if disk_usage >= self.alert_thresholds.disk_warning_percent {
            self.create_alert(
                AlertType::HighDiskUsage,
                AlertSeverity::Warning,
                "High Disk Usage".to_string(),
                format!("Disk usage is high: {:.1}%", disk_usage),
            ).await?;
        } else {
            self.resolve_alerts_by_type(AlertType::HighDiskUsage).await?;
        }
        
        Ok(())
    }
    
    /// Check API response time alert
    pub async fn check_response_time_alert(&mut self, service: &str, response_time_ms: u32) -> AppResult<()> {
        if response_time_ms >= self.alert_thresholds.api_response_critical_ms {
            self.create_alert(
                AlertType::SlowApiResponse,
                AlertSeverity::Critical,
                "Critical API Response Time".to_string(),
                format!("API service '{}' is responding very slowly: {}ms", service, response_time_ms),
            ).await?;
        } else if response_time_ms >= self.alert_thresholds.api_response_warning_ms {
            self.create_alert(
                AlertType::SlowApiResponse,
                AlertSeverity::Warning,
                "Slow API Response Time".to_string(),
                format!("API service '{}' is responding slowly: {}ms", service, response_time_ms),
            ).await?;
        }
        
        Ok(())
    }
    
    /// Check error rate alert
    pub async fn check_error_rate_alert(&mut self, error_type: &str) -> AppResult<()> {
        // TODO: Implement actual error rate tracking
        // For now, create a simple error alert
        self.create_alert(
            AlertType::HighErrorRate,
            AlertSeverity::Warning,
            "Error Detected".to_string(),
            format!("Error occurred: {}", error_type),
        ).await?;
        
        Ok(())
    }
    
    /// Check queue backlog alert
    pub async fn check_queue_backlog_alert(&mut self, queue_length: u32) -> AppResult<()> {
        if queue_length >= self.alert_thresholds.queue_backlog_critical {
            self.create_alert(
                AlertType::QueueBacklog,
                AlertSeverity::Critical,
                "Critical Queue Backlog".to_string(),
                format!("Research queue has critical backlog: {} items", queue_length),
            ).await?;
        } else if queue_length >= self.alert_thresholds.queue_backlog_warning {
            self.create_alert(
                AlertType::QueueBacklog,
                AlertSeverity::Warning,
                "Queue Backlog".to_string(),
                format!("Research queue has backlog: {} items", queue_length),
            ).await?;
        } else {
            self.resolve_alerts_by_type(AlertType::QueueBacklog).await?;
        }
        
        Ok(())
    }
    
    /// Resolve all alerts of a specific type
    async fn resolve_alerts_by_type(&mut self, alert_type: AlertType) -> AppResult<()> {
        let alert_ids: Vec<Uuid> = self.active_alerts
            .iter()
            .filter(|(_, alert)| std::mem::discriminant(&alert.alert_type) == std::mem::discriminant(&alert_type))
            .map(|(id, _)| *id)
            .collect();
        
        for alert_id in alert_ids {
            self.resolve_alert(alert_id).await?;
        }
        
        Ok(())
    }
    
    /// Update alert thresholds
    pub async fn update_thresholds(&mut self, thresholds: AlertThresholds) -> AppResult<()> {
        info!("Updating alert thresholds");
        self.alert_thresholds = thresholds;
        Ok(())
    }
    
    /// Get current alert thresholds
    pub async fn get_thresholds(&self) -> AppResult<AlertThresholds> {
        Ok(self.alert_thresholds.clone())
    }
    
    /// Get alert statistics
    pub async fn get_alert_stats(&self) -> AppResult<AlertStats> {
        let active_count = self.active_alerts.len();
        let total_count = self.alert_history.len() + active_count;
        
        let critical_count = self.active_alerts.values()
            .filter(|a| a.severity == AlertSeverity::Critical)
            .count();
        
        let warning_count = self.active_alerts.values()
            .filter(|a| a.severity == AlertSeverity::Warning)
            .count();
        
        Ok(AlertStats {
            active_count,
            total_count,
            critical_count,
            warning_count,
        })
    }
}

/// Alert statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertStats {
    pub active_count: usize,
    pub total_count: usize,
    pub critical_count: usize,
    pub warning_count: usize,
}
