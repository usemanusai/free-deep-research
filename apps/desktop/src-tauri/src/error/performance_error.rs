use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Performance-related errors
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum PerformanceError {
    #[error("Cache error: {message}")]
    CacheError { message: String },
    
    #[error("Request deduplication error: {message}")]
    DeduplicationError { message: String },
    
    #[error("Background processing error: {message}")]
    BackgroundProcessingError { message: String },
    
    #[error("Connection pool error: {message}")]
    ConnectionPoolError { message: String },
    
    #[error("Performance optimization error: {message}")]
    OptimizationError { message: String },
    
    #[error("Poor performance detected: {message}")]
    PoorPerformance { message: String },
    
    #[error("Resource exhaustion: {resource}: {current}/{limit}")]
    ResourceExhaustion {
        resource: String,
        current: u64,
        limit: u64,
    },
    
    #[error("Timeout exceeded: {operation}: {timeout_ms}ms")]
    TimeoutExceeded {
        operation: String,
        timeout_ms: u64,
    },
    
    #[error("Queue full: {queue_type}: {size}/{capacity}")]
    QueueFull {
        queue_type: String,
        size: usize,
        capacity: usize,
    },
    
    #[error("Service degraded: {service}: {reason}")]
    ServiceDegraded {
        service: String,
        reason: String,
    },
}

impl PerformanceError {
    /// Create a new cache error
    pub fn cache_error(message: impl Into<String>) -> Self {
        Self::CacheError {
            message: message.into(),
        }
    }
    
    /// Create a new deduplication error
    pub fn deduplication_error(message: impl Into<String>) -> Self {
        Self::DeduplicationError {
            message: message.into(),
        }
    }
    
    /// Create a new background processing error
    pub fn background_processing_error(message: impl Into<String>) -> Self {
        Self::BackgroundProcessingError {
            message: message.into(),
        }
    }
    
    /// Create a new connection pool error
    pub fn connection_pool_error(message: impl Into<String>) -> Self {
        Self::ConnectionPoolError {
            message: message.into(),
        }
    }
    
    /// Create a new poor performance error
    pub fn poor_performance(message: impl Into<String>) -> Self {
        Self::PoorPerformance {
            message: message.into(),
        }
    }
    
    /// Create a new resource exhaustion error
    pub fn resource_exhaustion(resource: impl Into<String>, current: u64, limit: u64) -> Self {
        Self::ResourceExhaustion {
            resource: resource.into(),
            current,
            limit,
        }
    }
    
    /// Create a new timeout exceeded error
    pub fn timeout_exceeded(operation: impl Into<String>, timeout_ms: u64) -> Self {
        Self::TimeoutExceeded {
            operation: operation.into(),
            timeout_ms,
        }
    }
    
    /// Create a new queue full error
    pub fn queue_full(queue_type: impl Into<String>, size: usize, capacity: usize) -> Self {
        Self::QueueFull {
            queue_type: queue_type.into(),
            size,
            capacity,
        }
    }
    
    /// Create a new service degraded error
    pub fn service_degraded(service: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::ServiceDegraded {
            service: service.into(),
            reason: reason.into(),
        }
    }
    
    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            PerformanceError::TimeoutExceeded { .. }
                | PerformanceError::QueueFull { .. }
                | PerformanceError::ServiceDegraded { .. }
                | PerformanceError::ResourceExhaustion { .. }
        )
    }
    
    /// Check if this error indicates a critical performance issue
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            PerformanceError::PoorPerformance { .. }
                | PerformanceError::ResourceExhaustion { .. }
                | PerformanceError::ServiceDegraded { .. }
        )
    }
    
    /// Get the severity level of this error
    pub fn severity(&self) -> PerformanceErrorSeverity {
        match self {
            PerformanceError::PoorPerformance { .. } => PerformanceErrorSeverity::Critical,
            PerformanceError::ResourceExhaustion { .. } => PerformanceErrorSeverity::High,
            PerformanceError::ServiceDegraded { .. } => PerformanceErrorSeverity::High,
            PerformanceError::QueueFull { .. } => PerformanceErrorSeverity::Medium,
            PerformanceError::TimeoutExceeded { .. } => PerformanceErrorSeverity::Medium,
            PerformanceError::CacheError { .. } => PerformanceErrorSeverity::Low,
            PerformanceError::DeduplicationError { .. } => PerformanceErrorSeverity::Low,
            PerformanceError::BackgroundProcessingError { .. } => PerformanceErrorSeverity::Medium,
            PerformanceError::ConnectionPoolError { .. } => PerformanceErrorSeverity::Medium,
            PerformanceError::OptimizationError { .. } => PerformanceErrorSeverity::Low,
        }
    }
}

/// Performance error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PerformanceErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Cache-specific errors
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum CacheError {
    #[error("Cache miss: {key}")]
    CacheMiss { key: String },
    
    #[error("Cache full: cannot store more entries")]
    CacheFull,
    
    #[error("Cache corruption detected: {key}")]
    CacheCorruption { key: String },
    
    #[error("Cache service error: {message}")]
    ServiceError { message: String },
    
    #[error("Cache serialization error: {message}")]
    SerializationError { message: String },
    
    #[error("Cache eviction failed: {message}")]
    EvictionFailed { message: String },
}

impl CacheError {
    pub fn service_error(message: impl Into<String>) -> Self {
        Self::ServiceError {
            message: message.into(),
        }
    }
    
    pub fn serialization_error(message: impl Into<String>) -> Self {
        Self::SerializationError {
            message: message.into(),
        }
    }
}

/// Request deduplication errors
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum DeduplicationError {
    #[error("Request failed: {message}")]
    RequestFailed { message: String },
    
    #[error("Channel closed: {message}")]
    ChannelClosed { message: String },
    
    #[error("Service error: {message}")]
    ServiceError { message: String },
    
    #[error("Hash collision detected")]
    HashCollision,
    
    #[error("Timeout waiting for duplicate request")]
    DuplicateTimeout,
}

impl DeduplicationError {
    pub fn request_failed(message: impl Into<String>) -> Self {
        Self::RequestFailed {
            message: message.into(),
        }
    }
    
    pub fn channel_closed(message: impl Into<String>) -> Self {
        Self::ChannelClosed {
            message: message.into(),
        }
    }
    
    pub fn service_error(message: impl Into<String>) -> Self {
        Self::ServiceError {
            message: message.into(),
        }
    }
}

/// Background processing errors
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum BackgroundProcessingError {
    #[error("Task execution failed: {message}")]
    TaskExecutionFailed { message: String },
    
    #[error("Queue full: {current}/{max}")]
    QueueFull { current: usize, max: usize },
    
    #[error("Worker error: {worker_id}: {message}")]
    WorkerError { worker_id: usize, message: String },
    
    #[error("Task timeout: {task_id}")]
    TaskTimeout { task_id: String },
    
    #[error("Service error: {message}")]
    ServiceError { message: String },
}

impl BackgroundProcessingError {
    pub fn task_execution_failed(message: impl Into<String>) -> Self {
        Self::TaskExecutionFailed {
            message: message.into(),
        }
    }
    
    pub fn queue_full(message: impl Into<String>) -> Self {
        Self::QueueFull {
            current: 0,
            max: 0,
        }
    }
    
    pub fn service_error(message: impl Into<String>) -> Self {
        Self::ServiceError {
            message: message.into(),
        }
    }
}

/// Connection pool errors
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ConnectionPoolError {
    #[error("Connection failed: {message}")]
    ConnectionFailed { message: String },
    
    #[error("Pool exhausted: {active}/{max}")]
    PoolExhausted { active: usize, max: usize },
    
    #[error("Connection timeout")]
    ConnectionTimeout,
    
    #[error("Service error: {message}")]
    ServiceError { message: String },
    
    #[error("Health check failed: {message}")]
    HealthCheckFailed { message: String },
}

impl ConnectionPoolError {
    pub fn connection_failed(message: impl Into<String>) -> Self {
        Self::ConnectionFailed {
            message: message.into(),
        }
    }
    
    pub fn service_error(message: impl Into<String>) -> Self {
        Self::ServiceError {
            message: message.into(),
        }
    }
    
    pub fn health_check_failed(message: impl Into<String>) -> Self {
        Self::HealthCheckFailed {
            message: message.into(),
        }
    }
}
