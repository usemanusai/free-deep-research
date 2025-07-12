use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Real-time system monitoring and performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringMetrics {
    pub timestamp: DateTime<Utc>,
    pub api_usage: Vec<ApiUsageMetrics>,
    pub system_performance: SystemPerformanceMetrics,
    pub research_statistics: ResearchStatistics,
    pub error_counts: ErrorCounts,
}

impl MonitoringMetrics {
    /// Create new monitoring metrics with current timestamp
    pub fn new() -> Self {
        Self {
            timestamp: Utc::now(),
            api_usage: vec![],
            system_performance: SystemPerformanceMetrics::default(),
            research_statistics: ResearchStatistics::default(),
            error_counts: ErrorCounts::default(),
        }
    }
}

/// API usage metrics for a specific service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiUsageMetrics {
    pub service: String,
    pub requests_made: u32,
    pub requests_remaining: u32,
    pub reset_time: DateTime<Utc>,
    pub success_rate: f64, // Percentage (0.0 - 100.0)
    pub average_response_time_ms: f64,
}

impl ApiUsageMetrics {
    /// Create new API usage metrics for a service
    pub fn new(service: String) -> Self {
        Self {
            service,
            requests_made: 0,
            requests_remaining: 0,
            reset_time: Utc::now(),
            success_rate: 100.0,
            average_response_time_ms: 0.0,
        }
    }
    
    /// Calculate usage percentage
    pub fn usage_percentage(&self) -> f64 {
        let total = self.requests_made + self.requests_remaining;
        if total == 0 {
            0.0
        } else {
            (self.requests_made as f64 / total as f64) * 100.0
        }
    }
    
    /// Check if the service is approaching its limit
    pub fn is_approaching_limit(&self, threshold: f64) -> bool {
        self.usage_percentage() >= threshold
    }
}

/// System performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPerformanceMetrics {
    pub cpu_usage: f64,    // Percentage (0.0 - 100.0)
    pub memory_usage: f64, // Percentage (0.0 - 100.0)
    pub disk_usage: f64,   // Percentage (0.0 - 100.0)
    pub network_io: NetworkIoMetrics,
    pub uptime_seconds: u64,
}

impl Default for SystemPerformanceMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            disk_usage: 0.0,
            network_io: NetworkIoMetrics::default(),
            uptime_seconds: 0,
        }
    }
}

/// Network I/O metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkIoMetrics {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub requests_per_second: f64,
}

impl Default for NetworkIoMetrics {
    fn default() -> Self {
        Self {
            bytes_sent: 0,
            bytes_received: 0,
            requests_per_second: 0.0,
        }
    }
}

/// Research workflow statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchStatistics {
    pub active_workflows: u32,
    pub completed_today: u32,
    pub failed_today: u32,
    pub average_duration_ms: f64,
    pub total_research_count: u32,
    pub success_rate: f64, // Percentage (0.0 - 100.0)
}

impl Default for ResearchStatistics {
    fn default() -> Self {
        Self {
            active_workflows: 0,
            completed_today: 0,
            failed_today: 0,
            average_duration_ms: 0.0,
            total_research_count: 0,
            success_rate: 100.0,
        }
    }
}

impl ResearchStatistics {
    /// Calculate completion rate for today
    pub fn completion_rate_today(&self) -> f64 {
        let total_today = self.completed_today + self.failed_today;
        if total_today == 0 {
            0.0
        } else {
            (self.completed_today as f64 / total_today as f64) * 100.0
        }
    }
}

/// Error count metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorCounts {
    pub api_errors: u32,
    pub system_errors: u32,
    pub user_errors: u32,
    pub security_errors: u32,
    pub network_errors: u32,
}

impl Default for ErrorCounts {
    fn default() -> Self {
        Self {
            api_errors: 0,
            system_errors: 0,
            user_errors: 0,
            security_errors: 0,
            network_errors: 0,
        }
    }
}

impl ErrorCounts {
    /// Get total error count
    pub fn total(&self) -> u32 {
        self.api_errors + self.system_errors + self.user_errors + self.security_errors + self.network_errors
    }
    
    /// Check if error count is above threshold
    pub fn is_above_threshold(&self, threshold: u32) -> bool {
        self.total() > threshold
    }
}

/// Historical metrics for trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalMetrics {
    pub date: DateTime<Utc>,
    pub metrics: MonitoringMetrics,
}

/// Aggregated metrics for reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedMetrics {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_api_requests: u32,
    pub total_research_workflows: u32,
    pub average_system_performance: SystemPerformanceMetrics,
    pub total_errors: ErrorCounts,
    pub peak_usage_time: Option<DateTime<Utc>>,
    pub trends: HashMap<String, f64>, // Key-value pairs for various trend metrics
}

impl AggregatedMetrics {
    /// Create new aggregated metrics for a time period
    pub fn new(period_start: DateTime<Utc>, period_end: DateTime<Utc>) -> Self {
        Self {
            period_start,
            period_end,
            total_api_requests: 0,
            total_research_workflows: 0,
            average_system_performance: SystemPerformanceMetrics::default(),
            total_errors: ErrorCounts::default(),
            peak_usage_time: None,
            trends: HashMap::new(),
        }
    }
}
