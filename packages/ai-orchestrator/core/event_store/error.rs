// Event Store Error Types
// Phase 4.1: Event Sourcing Foundation

use std::fmt;
use thiserror::Error;

/// Event store specific errors
#[derive(Error, Debug)]
pub enum EventStoreError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Concurrency conflict: expected version {expected}, actual version {actual}")]
    ConcurrencyConflict { expected: u64, actual: u64 },

    #[error("Stream not found: {stream_id}")]
    StreamNotFound { stream_id: uuid::Uuid },

    #[error("Event not found: {event_id}")]
    EventNotFound { event_id: uuid::Uuid },

    #[error("Invalid event type: {event_type}")]
    InvalidEventType { event_type: String },

    #[error("Event schema validation failed: {message}")]
    SchemaValidation { message: String },

    #[error("Snapshot error: {message}")]
    Snapshot { message: String },

    #[error("Projection error: {message}")]
    Projection { message: String },

    #[error("Event bus error: {message}")]
    EventBus { message: String },

    #[error("Configuration error: {message}")]
    Configuration { message: String },

    #[error("Timeout error: operation timed out after {seconds} seconds")]
    Timeout { seconds: u64 },

    #[error("Invalid operation: {message}")]
    InvalidOperation { message: String },

    #[error("Internal error: {message}")]
    Internal { message: String },
}

impl EventStoreError {
    pub fn concurrency_conflict(expected: u64, actual: u64) -> Self {
        Self::ConcurrencyConflict { expected, actual }
    }

    pub fn stream_not_found(stream_id: uuid::Uuid) -> Self {
        Self::StreamNotFound { stream_id }
    }

    pub fn event_not_found(event_id: uuid::Uuid) -> Self {
        Self::EventNotFound { event_id }
    }

    pub fn invalid_event_type(event_type: impl Into<String>) -> Self {
        Self::InvalidEventType {
            event_type: event_type.into(),
        }
    }

    pub fn schema_validation(message: impl Into<String>) -> Self {
        Self::SchemaValidation {
            message: message.into(),
        }
    }

    pub fn snapshot_error(message: impl Into<String>) -> Self {
        Self::Snapshot {
            message: message.into(),
        }
    }

    pub fn projection_error(message: impl Into<String>) -> Self {
        Self::Projection {
            message: message.into(),
        }
    }

    pub fn event_bus_error(message: impl Into<String>) -> Self {
        Self::EventBus {
            message: message.into(),
        }
    }

    pub fn configuration_error(message: impl Into<String>) -> Self {
        Self::Configuration {
            message: message.into(),
        }
    }

    pub fn timeout(seconds: u64) -> Self {
        Self::Timeout { seconds }
    }

    pub fn invalid_operation(message: impl Into<String>) -> Self {
        Self::InvalidOperation {
            message: message.into(),
        }
    }

    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            EventStoreError::Database(sqlx_error) => {
                // Check if it's a connection error or temporary failure
                matches!(
                    sqlx_error,
                    sqlx::Error::Io(_) | sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed
                )
            }
            EventStoreError::Timeout { .. } => true,
            EventStoreError::ConcurrencyConflict { .. } => true,
            _ => false,
        }
    }

    /// Get error category for monitoring/alerting
    pub fn category(&self) -> ErrorCategory {
        match self {
            EventStoreError::Database(_) => ErrorCategory::Infrastructure,
            EventStoreError::Serialization(_) => ErrorCategory::Data,
            EventStoreError::ConcurrencyConflict { .. } => ErrorCategory::Concurrency,
            EventStoreError::StreamNotFound { .. } => ErrorCategory::NotFound,
            EventStoreError::EventNotFound { .. } => ErrorCategory::NotFound,
            EventStoreError::InvalidEventType { .. } => ErrorCategory::Validation,
            EventStoreError::SchemaValidation { .. } => ErrorCategory::Validation,
            EventStoreError::Snapshot { .. } => ErrorCategory::Storage,
            EventStoreError::Projection { .. } => ErrorCategory::Processing,
            EventStoreError::EventBus { .. } => ErrorCategory::Communication,
            EventStoreError::Configuration { .. } => ErrorCategory::Configuration,
            EventStoreError::Timeout { .. } => ErrorCategory::Performance,
            EventStoreError::InvalidOperation { .. } => ErrorCategory::Logic,
            EventStoreError::Internal { .. } => ErrorCategory::Internal,
        }
    }
}

/// Error categories for monitoring and alerting
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorCategory {
    Infrastructure,
    Data,
    Concurrency,
    NotFound,
    Validation,
    Storage,
    Processing,
    Communication,
    Configuration,
    Performance,
    Logic,
    Internal,
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCategory::Infrastructure => write!(f, "infrastructure"),
            ErrorCategory::Data => write!(f, "data"),
            ErrorCategory::Concurrency => write!(f, "concurrency"),
            ErrorCategory::NotFound => write!(f, "not_found"),
            ErrorCategory::Validation => write!(f, "validation"),
            ErrorCategory::Storage => write!(f, "storage"),
            ErrorCategory::Processing => write!(f, "processing"),
            ErrorCategory::Communication => write!(f, "communication"),
            ErrorCategory::Configuration => write!(f, "configuration"),
            ErrorCategory::Performance => write!(f, "performance"),
            ErrorCategory::Logic => write!(f, "logic"),
            ErrorCategory::Internal => write!(f, "internal"),
        }
    }
}

/// Result type alias for event store operations
pub type EventStoreResult<T> = Result<T, EventStoreError>;

/// Retry configuration for event store operations
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_multiplier: 2.0,
        }
    }
}

/// Retry helper for event store operations
pub async fn retry_operation<F, T, Fut>(
    operation: F,
    config: &RetryConfig,
) -> EventStoreResult<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = EventStoreResult<T>>,
{
    let mut last_error = None;
    let mut delay_ms = config.base_delay_ms;

    for attempt in 1..=config.max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                if !error.is_retryable() || attempt == config.max_attempts {
                    return Err(error);
                }

                last_error = Some(error);

                // Wait before retry
                tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;

                // Exponential backoff
                delay_ms = std::cmp::min(
                    (delay_ms as f64 * config.backoff_multiplier) as u64,
                    config.max_delay_ms,
                );
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        EventStoreError::internal_error("Retry operation failed without error")
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_categories() {
        let db_error = EventStoreError::Database(sqlx::Error::RowNotFound);
        assert_eq!(db_error.category(), ErrorCategory::Infrastructure);

        let concurrency_error = EventStoreError::concurrency_conflict(1, 2);
        assert_eq!(concurrency_error.category(), ErrorCategory::Concurrency);

        let validation_error = EventStoreError::schema_validation("Invalid schema");
        assert_eq!(validation_error.category(), ErrorCategory::Validation);
    }

    #[test]
    fn test_retryable_errors() {
        let concurrency_error = EventStoreError::concurrency_conflict(1, 2);
        assert!(concurrency_error.is_retryable());

        let timeout_error = EventStoreError::timeout(30);
        assert!(timeout_error.is_retryable());

        let validation_error = EventStoreError::schema_validation("Invalid schema");
        assert!(!validation_error.is_retryable());
    }

    #[tokio::test]
    async fn test_retry_operation() {
        let config = RetryConfig {
            max_attempts: 3,
            base_delay_ms: 1,
            max_delay_ms: 10,
            backoff_multiplier: 2.0,
        };

        let mut attempt_count = 0;
        let result = retry_operation(
            || {
                attempt_count += 1;
                async move {
                    if attempt_count < 3 {
                        Err(EventStoreError::timeout(1))
                    } else {
                        Ok("success")
                    }
                }
            },
            &config,
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
        assert_eq!(attempt_count, 3);
    }
}
