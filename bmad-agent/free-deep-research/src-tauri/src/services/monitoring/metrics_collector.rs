use std::collections::HashMap;
use tracing::{info, debug, error};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, MonitoringError};
use super::SystemMetrics;

/// Metrics collector for gathering system performance data
pub struct MetricsCollector {
    start_time: DateTime<Utc>,
    metrics_history: Vec<SystemMetrics>,
    max_history_size: usize,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub async fn new() -> AppResult<Self> {
        info!("Initializing metrics collector");
        
        let collector = Self {
            start_time: Utc::now(),
            metrics_history: Vec::new(),
            max_history_size: 1440, // 24 hours at 1-minute intervals
        };
        
        info!("Metrics collector initialized successfully");
        Ok(collector)
    }
    
    /// Collect current system metrics
    pub async fn collect_metrics(&self) -> AppResult<SystemMetrics> {
        debug!("Collecting system metrics");
        
        let timestamp = Utc::now();
        let uptime_seconds = (timestamp - self.start_time).num_seconds() as u64;
        
        // Collect CPU usage
        let cpu_usage = self.get_cpu_usage().await?;
        
        // Collect memory usage
        let memory_usage = self.get_memory_usage().await?;
        
        // Collect disk usage
        let disk_usage = self.get_disk_usage().await?;
        
        // Check network connectivity
        let network_active = self.check_network_connectivity().await?;
        
        let metrics = SystemMetrics {
            timestamp,
            cpu_usage_percent: cpu_usage,
            memory_usage_percent: memory_usage,
            disk_usage_percent: disk_usage,
            network_active,
            active_workflows: 0, // Will be updated by monitoring service
            queue_length: 0,     // Will be updated by monitoring service
            api_response_times: HashMap::new(), // Will be updated by monitoring service
            error_count_last_hour: 0, // Will be updated by monitoring service
            uptime_seconds,
        };
        
        debug!("System metrics collected: CPU={:.1}%, Memory={:.1}%, Disk={:.1}%", 
            cpu_usage, memory_usage, disk_usage);
        
        Ok(metrics)
    }
    
    /// Store metrics in history
    pub async fn store_metrics(&mut self, metrics: SystemMetrics) -> AppResult<()> {
        debug!("Storing metrics in history");
        
        self.metrics_history.push(metrics);
        
        // Trim history if it exceeds max size
        if self.metrics_history.len() > self.max_history_size {
            self.metrics_history.remove(0);
        }
        
        debug!("Metrics stored, history size: {}", self.metrics_history.len());
        Ok(())
    }
    
    /// Get metrics history for the specified number of hours
    pub async fn get_metrics_history(&self, hours: u32) -> AppResult<Vec<SystemMetrics>> {
        debug!("Getting metrics history for last {} hours", hours);
        
        let cutoff_time = Utc::now() - chrono::Duration::hours(hours as i64);
        
        let filtered_metrics: Vec<SystemMetrics> = self.metrics_history
            .iter()
            .filter(|m| m.timestamp > cutoff_time)
            .cloned()
            .collect();
        
        debug!("Retrieved {} metrics from history", filtered_metrics.len());
        Ok(filtered_metrics)
    }
    
    /// Get CPU usage percentage
    async fn get_cpu_usage(&self) -> AppResult<f64> {
        // TODO: Implement actual CPU usage collection
        // For now, simulate CPU usage
        let base_usage = 20.0;
        let variation = rand::random::<f64>() * 60.0;
        Ok(base_usage + variation)
    }
    
    /// Get memory usage percentage
    async fn get_memory_usage(&self) -> AppResult<f64> {
        // TODO: Implement actual memory usage collection
        // For now, simulate memory usage
        let base_usage = 30.0;
        let variation = rand::random::<f64>() * 50.0;
        Ok(base_usage + variation)
    }
    
    /// Get disk usage percentage
    async fn get_disk_usage(&self) -> AppResult<f64> {
        // TODO: Implement actual disk usage collection
        // For now, simulate disk usage
        let base_usage = 50.0;
        let variation = rand::random::<f64>() * 30.0;
        Ok(base_usage + variation)
    }
    
    /// Check network connectivity
    async fn check_network_connectivity(&self) -> AppResult<bool> {
        debug!("Checking network connectivity");
        
        // Try to connect to a reliable external service
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .map_err(|e| MonitoringError::network_check_failed(e.to_string()))?;
        
        match client.get("https://httpbin.org/status/200").send().await {
            Ok(response) => {
                let is_connected = response.status().is_success();
                debug!("Network connectivity check: {}", if is_connected { "OK" } else { "Failed" });
                Ok(is_connected)
            }
            Err(e) => {
                debug!("Network connectivity check failed: {}", e);
                Ok(false)
            }
        }
    }
    
    /// Get system load averages (Unix-like systems)
    #[cfg(unix)]
    async fn get_load_averages(&self) -> AppResult<LoadAverages> {
        use std::fs;
        
        let loadavg_content = fs::read_to_string("/proc/loadavg")
            .map_err(|e| MonitoringError::metrics_collection_failed(format!("Failed to read loadavg: {}", e)))?;
        
        let parts: Vec<&str> = loadavg_content.split_whitespace().collect();
        if parts.len() >= 3 {
            let load_1min = parts[0].parse::<f64>()
                .map_err(|e| MonitoringError::metrics_collection_failed(format!("Failed to parse 1min load: {}", e)))?;
            let load_5min = parts[1].parse::<f64>()
                .map_err(|e| MonitoringError::metrics_collection_failed(format!("Failed to parse 5min load: {}", e)))?;
            let load_15min = parts[2].parse::<f64>()
                .map_err(|e| MonitoringError::metrics_collection_failed(format!("Failed to parse 15min load: {}", e)))?;
            
            Ok(LoadAverages {
                load_1min,
                load_5min,
                load_15min,
            })
        } else {
            Err(MonitoringError::metrics_collection_failed("Invalid loadavg format".to_string()).into())
        }
    }
    
    /// Get system load averages (Windows - simulated)
    #[cfg(windows)]
    async fn get_load_averages(&self) -> AppResult<LoadAverages> {
        // Windows doesn't have load averages, so we simulate based on CPU usage
        let cpu_usage = self.get_cpu_usage().await?;
        let normalized_load = cpu_usage / 100.0 * num_cpus::get() as f64;
        
        Ok(LoadAverages {
            load_1min: normalized_load,
            load_5min: normalized_load * 0.9,
            load_15min: normalized_load * 0.8,
        })
    }
    
    /// Get memory information
    async fn get_memory_info(&self) -> AppResult<MemoryInfo> {
        // TODO: Implement actual memory information collection
        // For now, simulate memory info
        let total_mb = 8192; // 8GB
        let used_percent = self.get_memory_usage().await?;
        let used_mb = (total_mb as f64 * used_percent / 100.0) as u64;
        let available_mb = total_mb - used_mb;
        
        Ok(MemoryInfo {
            total_mb,
            used_mb,
            available_mb,
            usage_percent: used_percent,
        })
    }
    
    /// Get disk information
    async fn get_disk_info(&self) -> AppResult<DiskInfo> {
        // TODO: Implement actual disk information collection
        // For now, simulate disk info
        let total_gb = 500; // 500GB
        let used_percent = self.get_disk_usage().await?;
        let used_gb = (total_gb as f64 * used_percent / 100.0) as u64;
        let available_gb = total_gb - used_gb;
        
        Ok(DiskInfo {
            total_gb,
            used_gb,
            available_gb,
            usage_percent: used_percent,
        })
    }
    
    /// Get detailed system metrics
    pub async fn get_detailed_metrics(&self) -> AppResult<DetailedSystemMetrics> {
        debug!("Collecting detailed system metrics");
        
        let basic_metrics = self.collect_metrics().await?;
        let load_averages = self.get_load_averages().await?;
        let memory_info = self.get_memory_info().await?;
        let disk_info = self.get_disk_info().await?;
        
        Ok(DetailedSystemMetrics {
            basic_metrics,
            load_averages,
            memory_info,
            disk_info,
        })
    }
}

/// System load averages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadAverages {
    pub load_1min: f64,
    pub load_5min: f64,
    pub load_15min: f64,
}

/// Memory information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total_mb: u64,
    pub used_mb: u64,
    pub available_mb: u64,
    pub usage_percent: f64,
}

/// Disk information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub total_gb: u64,
    pub used_gb: u64,
    pub available_gb: u64,
    pub usage_percent: f64,
}

/// Detailed system metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedSystemMetrics {
    pub basic_metrics: SystemMetrics,
    pub load_averages: LoadAverages,
    pub memory_info: MemoryInfo,
    pub disk_info: DiskInfo,
}
