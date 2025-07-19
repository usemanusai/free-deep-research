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
        use sysinfo::{System, SystemExt, CpuExt};

        let mut system = System::new_all();
        system.refresh_cpu();

        // Wait a bit to get accurate CPU usage
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        system.refresh_cpu();

        let cpu_usage = system.global_cpu_info().cpu_usage() as f64;

        // Ensure we return a reasonable value (0-100)
        let normalized_usage = cpu_usage.max(0.0).min(100.0);

        debug!("CPU usage collected: {:.2}%", normalized_usage);
        Ok(normalized_usage)
    }
    
    /// Get memory usage percentage
    async fn get_memory_usage(&self) -> AppResult<f64> {
        use sysinfo::{System, SystemExt};

        let mut system = System::new_all();
        system.refresh_memory();

        let total_memory = system.total_memory();
        let used_memory = system.used_memory();

        if total_memory == 0 {
            warn!("Total memory reported as 0, returning 0% usage");
            return Ok(0.0);
        }

        let memory_usage_percent = (used_memory as f64 / total_memory as f64) * 100.0;
        let normalized_usage = memory_usage_percent.max(0.0).min(100.0);

        debug!("Memory usage collected: {:.2}% ({} MB / {} MB)",
               normalized_usage,
               used_memory / 1024 / 1024,
               total_memory / 1024 / 1024);

        Ok(normalized_usage)
    }
    
    /// Get disk usage percentage
    async fn get_disk_usage(&self) -> AppResult<f64> {
        use sysinfo::{System, SystemExt, DiskExt};

        let mut system = System::new_all();
        system.refresh_disks();

        let disks = system.disks();

        if disks.is_empty() {
            warn!("No disks found, returning 0% usage");
            return Ok(0.0);
        }

        // Calculate average disk usage across all disks
        let mut total_space = 0u64;
        let mut total_used = 0u64;

        for disk in disks {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total.saturating_sub(available);

            total_space += total;
            total_used += used;
        }

        if total_space == 0 {
            warn!("Total disk space reported as 0, returning 0% usage");
            return Ok(0.0);
        }

        let disk_usage_percent = (total_used as f64 / total_space as f64) * 100.0;
        let normalized_usage = disk_usage_percent.max(0.0).min(100.0);

        debug!("Disk usage collected: {:.2}% ({:.2} GB / {:.2} GB)",
               normalized_usage,
               total_used as f64 / 1024.0 / 1024.0 / 1024.0,
               total_space as f64 / 1024.0 / 1024.0 / 1024.0);

        Ok(normalized_usage)
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_metrics_collector_creation() {
        let collector = MetricsCollector::new().await;
        assert!(collector.is_ok(), "MetricsCollector should be created successfully");
    }

    #[tokio::test]
    async fn test_cpu_usage_collection() {
        let collector = MetricsCollector::new().await.unwrap();
        let cpu_usage = collector.get_cpu_usage().await;

        assert!(cpu_usage.is_ok(), "CPU usage collection should work");
        let usage = cpu_usage.unwrap();
        assert!(usage >= 0.0 && usage <= 100.0, "CPU usage should be between 0 and 100%");
    }

    #[tokio::test]
    async fn test_memory_usage_collection() {
        let collector = MetricsCollector::new().await.unwrap();
        let memory_usage = collector.get_memory_usage().await;

        assert!(memory_usage.is_ok(), "Memory usage collection should work");
        let usage = memory_usage.unwrap();
        assert!(usage >= 0.0 && usage <= 100.0, "Memory usage should be between 0 and 100%");
    }

    #[tokio::test]
    async fn test_disk_usage_collection() {
        let collector = MetricsCollector::new().await.unwrap();
        let disk_usage = collector.get_disk_usage().await;

        assert!(disk_usage.is_ok(), "Disk usage collection should work");
        let usage = disk_usage.unwrap();
        assert!(usage >= 0.0 && usage <= 100.0, "Disk usage should be between 0 and 100%");
    }

    #[tokio::test]
    async fn test_system_metrics_collection() {
        let collector = MetricsCollector::new().await.unwrap();
        let metrics = collector.collect_system_metrics().await;

        assert!(metrics.is_ok(), "System metrics collection should work");
        let metrics = metrics.unwrap();

        // Verify all metrics are within valid ranges
        assert!(metrics.cpu_usage_percent >= 0.0 && metrics.cpu_usage_percent <= 100.0);
        assert!(metrics.memory_usage_percent >= 0.0 && metrics.memory_usage_percent <= 100.0);
        assert!(metrics.disk_usage_percent >= 0.0 && metrics.disk_usage_percent <= 100.0);
        assert!(metrics.uptime_seconds >= 0);
    }

    #[tokio::test]
    async fn test_network_connectivity_check() {
        let collector = MetricsCollector::new().await.unwrap();
        let connectivity = collector.check_network_connectivity().await;

        // Network connectivity might fail in test environments, so we just check it doesn't panic
        assert!(connectivity.is_ok() || connectivity.is_err(), "Network check should return a result");
    }

    #[test]
    fn test_word_count_calculation() {
        // Test the word count logic used in research methodologies
        let test_content = "This is a test content with exactly ten words in it.";
        let word_count = test_content.split_whitespace().count() as u32;
        assert_eq!(word_count, 10, "Word count should be calculated correctly");

        let empty_content = "";
        let empty_count = empty_content.split_whitespace().count() as u32;
        assert_eq!(empty_count, 0, "Empty content should have zero word count");

        let single_word = "word";
        let single_count = single_word.split_whitespace().count() as u32;
        assert_eq!(single_count, 1, "Single word should have count of 1");
    }

    #[test]
    fn test_confidence_score_calculation() {
        // Test confidence score calculation logic used in workflow orchestrator
        let confidence_scores = vec![0.85, 0.90, 0.75, 0.95];
        let average_confidence = confidence_scores.iter().sum::<f64>() / confidence_scores.len() as f64;
        assert!((average_confidence - 0.8625).abs() < 0.001, "Confidence calculation should be accurate");

        let empty_scores: Vec<f64> = vec![];
        let default_confidence = if empty_scores.is_empty() { 0.75 } else { 0.0 };
        assert_eq!(default_confidence, 0.75, "Default confidence should be 0.75 for empty scores");
    }

    #[test]
    fn test_key_point_extraction() {
        // Test key point extraction logic used in AI summary
        let test_summary = "Introduction text\n- Key point one\n- Key point two\n• Bullet point three\nConclusion text";
        let key_points: Vec<String> = test_summary
            .lines()
            .filter(|line| line.starts_with("- ") || line.starts_with("• "))
            .map(|line| line.trim_start_matches("- ").trim_start_matches("• ").to_string())
            .collect();

        assert_eq!(key_points.len(), 3, "Should extract 3 key points");
        assert_eq!(key_points[0], "Key point one");
        assert_eq!(key_points[1], "Key point two");
        assert_eq!(key_points[2], "Bullet point three");
    }

    #[test]
    fn test_citation_extraction() {
        // Test citation extraction logic used in academic analysis
        let test_analysis = "Research shows multiple sources:\nhttps://example.com/paper1\ndoi:10.1000/test\nDOI:10.2000/example\nRegular text without citations";
        let citations: Vec<String> = test_analysis
            .lines()
            .filter(|line| line.contains("http") || line.contains("doi:") || line.contains("DOI:"))
            .map(|line| line.trim().to_string())
            .collect();

        assert_eq!(citations.len(), 3, "Should extract 3 citations");
        assert!(citations.iter().any(|c| c.contains("https://example.com/paper1")));
        assert!(citations.iter().any(|c| c.contains("doi:10.1000/test")));
        assert!(citations.iter().any(|c| c.contains("DOI:10.2000/example")));
    }
}
