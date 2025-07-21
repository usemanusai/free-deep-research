// CQRS Error Types and Handling
// Phase 4.2: CQRS Pattern Implementation

use std::fmt;
use thiserror::Error;

/// CQRS specific errors
#[derive(Error, Debug)]
pub enum CQRSError {
    #[error("Command validation failed: {0}")]
    ValidationError(String),

    #[error("Query validation failed: {0}")]
    QueryValidationError(String),

    #[error("Command handler not found for: {0}")]
    HandlerNotFound(String),

    #[error("Query handler not found for: {0}")]
    QueryHandlerNotFound(String),

    #[error("Failed to cast handler for: {0}")]
    HandlerCastError(String),

    #[error("Command execution timeout")]
    CommandTimeout,

    #[error("Query execution timeout")]
    QueryTimeout,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Event store error: {0}")]
    EventStoreError(String),

    #[error("Projection error: {0}")]
    ProjectionError(String),

    #[error("Read model error: {0}")]
    ReadModelError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Concurrency error: {0}")]
    ConcurrencyError(String),

    #[error("Authorization error: {0}")]
    AuthorizationError(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Resource conflict: {0}")]
    Conflict(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl CQRSError {
    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::ValidationError(message.into())
    }

    pub fn query_validation_error(message: impl Into<String>) -> Self {
        Self::QueryValidationError(message.into())
    }

    pub fn handler_not_found(handler_type: impl Into<String>) -> Self {
        Self::HandlerNotFound(handler_type.into())
    }

    pub fn query_handler_not_found(handler_type: impl Into<String>) -> Self {
        Self::QueryHandlerNotFound(handler_type.into())
    }

    pub fn handler_cast_error(handler_type: impl Into<String>) -> Self {
        Self::HandlerCastError(handler_type.into())
    }

    pub fn database_error(message: impl Into<String>) -> Self {
        Self::DatabaseError(message.into())
    }

    pub fn event_store_error(message: impl Into<String>) -> Self {
        Self::EventStoreError(message.into())
    }

    pub fn projection_error(message: impl Into<String>) -> Self {
        Self::ProjectionError(message.into())
    }

    pub fn read_model_error(message: impl Into<String>) -> Self {
        Self::ReadModelError(message.into())
    }

    pub fn cache_error(message: impl Into<String>) -> Self {
        Self::CacheError(message.into())
    }

    pub fn configuration_error(message: impl Into<String>) -> Self {
        Self::Configuration(message.into())
    }

    pub fn concurrency_error(message: impl Into<String>) -> Self {
        Self::ConcurrencyError(message.into())
    }

    pub fn authorization_error(message: impl Into<String>) -> Self {
        Self::AuthorizationError(message.into())
    }

    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound(resource.into())
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::Conflict(message.into())
    }

    pub fn rate_limit_exceeded(message: impl Into<String>) -> Self {
        Self::RateLimitExceeded(message.into())
    }

    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::ServiceUnavailable(message.into())
    }

    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            CQRSError::CommandTimeout => true,
            CQRSError::QueryTimeout => true,
            CQRSError::DatabaseError(_) => true,
            CQRSError::ServiceUnavailable(_) => true,
            CQRSError::ConcurrencyError(_) => true,
            CQRSError::RateLimitExceeded(_) => true,
            _ => false,
        }
    }

    /// Get error category for monitoring/alerting
    pub fn category(&self) -> ErrorCategory {
        match self {
            CQRSError::ValidationError(_) => ErrorCategory::Validation,
            CQRSError::QueryValidationError(_) => ErrorCategory::Validation,
            CQRSError::HandlerNotFound(_) => ErrorCategory::Configuration,
            CQRSError::QueryHandlerNotFound(_) => ErrorCategory::Configuration,
            CQRSError::HandlerCastError(_) => ErrorCategory::Internal,
            CQRSError::CommandTimeout => ErrorCategory::Performance,
            CQRSError::QueryTimeout => ErrorCategory::Performance,
            CQRSError::DatabaseError(_) => ErrorCategory::Infrastructure,
            CQRSError::SerializationError(_) => ErrorCategory::Data,
            CQRSError::EventStoreError(_) => ErrorCategory::Infrastructure,
            CQRSError::ProjectionError(_) => ErrorCategory::Processing,
            CQRSError::ReadModelError(_) => ErrorCategory::Data,
            CQRSError::CacheError(_) => ErrorCategory::Infrastructure,
            CQRSError::Configuration(_) => ErrorCategory::Configuration,
            CQRSError::ConcurrencyError(_) => ErrorCategory::Concurrency,
            CQRSError::AuthorizationError(_) => ErrorCategory::Security,
            CQRSError::NotFound(_) => ErrorCategory::NotFound,
            CQRSError::Conflict(_) => ErrorCategory::Business,
            CQRSError::RateLimitExceeded(_) => ErrorCategory::RateLimit,
            CQRSError::ServiceUnavailable(_) => ErrorCategory::Infrastructure,
            CQRSError::Internal(_) => ErrorCategory::Internal,
        }
    }

    /// Get HTTP status code for this error
    pub fn http_status_code(&self) -> u16 {
        match self {
            CQRSError::ValidationError(_) => 400,
            CQRSError::QueryValidationError(_) => 400,
            CQRSError::HandlerNotFound(_) => 501,
            CQRSError::QueryHandlerNotFound(_) => 501,
            CQRSError::HandlerCastError(_) => 500,
            CQRSError::CommandTimeout => 408,
            CQRSError::QueryTimeout => 408,
            CQRSError::DatabaseError(_) => 500,
            CQRSError::SerializationError(_) => 500,
            CQRSError::EventStoreError(_) => 500,
            CQRSError::ProjectionError(_) => 500,
            CQRSError::ReadModelError(_) => 500,
            CQRSError::CacheError(_) => 500,
            CQRSError::Configuration(_) => 500,
            CQRSError::ConcurrencyError(_) => 409,
            CQRSError::AuthorizationError(_) => 403,
            CQRSError::NotFound(_) => 404,
            CQRSError::Conflict(_) => 409,
            CQRSError::RateLimitExceeded(_) => 429,
            CQRSError::ServiceUnavailable(_) => 503,
            CQRSError::Internal(_) => 500,
        }
    }

    /// Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            CQRSError::ValidationError(_) => ErrorSeverity::Low,
            CQRSError::QueryValidationError(_) => ErrorSeverity::Low,
            CQRSError::HandlerNotFound(_) => ErrorSeverity::High,
            CQRSError::QueryHandlerNotFound(_) => ErrorSeverity::High,
            CQRSError::HandlerCastError(_) => ErrorSeverity::Critical,
            CQRSError::CommandTimeout => ErrorSeverity::Medium,
            CQRSError::QueryTimeout => ErrorSeverity::Medium,
            CQRSError::DatabaseError(_) => ErrorSeverity::High,
            CQRSError::SerializationError(_) => ErrorSeverity::Medium,
            CQRSError::EventStoreError(_) => ErrorSeverity::High,
            CQRSError::ProjectionError(_) => ErrorSeverity::Medium,
            CQRSError::ReadModelError(_) => ErrorSeverity::Medium,
            CQRSError::CacheError(_) => ErrorSeverity::Low,
            CQRSError::Configuration(_) => ErrorSeverity::High,
            CQRSError::ConcurrencyError(_) => ErrorSeverity::Medium,
            CQRSError::AuthorizationError(_) => ErrorSeverity::Medium,
            CQRSError::NotFound(_) => ErrorSeverity::Low,
            CQRSError::Conflict(_) => ErrorSeverity::Medium,
            CQRSError::RateLimitExceeded(_) => ErrorSeverity::Low,
            CQRSError::ServiceUnavailable(_) => ErrorSeverity::High,
            CQRSError::Internal(_) => ErrorSeverity::Critical,
        }
    }
}

/// Error categories for monitoring and alerting
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorCategory {
    Validation,
    Configuration,
    Internal,
    Performance,
    Infrastructure,
    Data,
    Processing,
    Concurrency,
    Security,
    NotFound,
    Business,
    RateLimit,
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCategory::Validation => write!(f, "validation"),
            ErrorCategory::Configuration => write!(f, "configuration"),
            ErrorCategory::Internal => write!(f, "internal"),
            ErrorCategory::Performance => write!(f, "performance"),
            ErrorCategory::Infrastructure => write!(f, "infrastructure"),
            ErrorCategory::Data => write!(f, "data"),
            ErrorCategory::Processing => write!(f, "processing"),
            ErrorCategory::Concurrency => write!(f, "concurrency"),
            ErrorCategory::Security => write!(f, "security"),
            ErrorCategory::NotFound => write!(f, "not_found"),
            ErrorCategory::Business => write!(f, "business"),
            ErrorCategory::RateLimit => write!(f, "rate_limit"),
        }
    }
}

/// Error severity levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSeverity::Low => write!(f, "low"),
            ErrorSeverity::Medium => write!(f, "medium"),
            ErrorSeverity::High => write!(f, "high"),
            ErrorSeverity::Critical => write!(f, "critical"),
        }
    }
}

/// Result type alias for CQRS operations
pub type CQRSResult<T> = Result<T, CQRSError>;

/// Error context for better error reporting
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub operation: String,
    pub component: String,
    pub correlation_id: Option<uuid::Uuid>,
    pub user_id: Option<uuid::Uuid>,
    pub additional_data: std::collections::HashMap<String, String>,
}

impl ErrorContext {
    pub fn new(operation: impl Into<String>, component: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            component: component.into(),
            correlation_id: None,
            user_id: None,
            additional_data: std::collections::HashMap::new(),
        }
    }

    pub fn with_correlation_id(mut self, correlation_id: uuid::Uuid) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    pub fn with_user_id(mut self, user_id: uuid::Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.additional_data.insert(key.into(), value.into());
        self
    }
}

/// Enhanced error with context
#[derive(Debug)]
pub struct ContextualError {
    pub error: CQRSError,
    pub context: ErrorContext,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ContextualError {
    pub fn new(error: CQRSError, context: ErrorContext) -> Self {
        Self {
            error,
            context,
            timestamp: chrono::Utc::now(),
        }
    }
}

impl fmt::Display for ContextualError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} in {}: {}",
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            self.context.operation,
            self.context.component,
            self.error
        )
    }
}

impl std::error::Error for ContextualError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

/// Retry configuration for CQRS operations
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
    pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_multiplier: 2.0,
            jitter: true,
        }
    }
}

/// Retry helper for CQRS operations
pub async fn retry_operation<F, T, Fut>(
    operation: F,
    config: &RetryConfig,
) -> CQRSResult<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = CQRSResult<T>>,
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

                // Add jitter if enabled
                let actual_delay = if config.jitter {
                    let jitter_range = delay_ms / 4; // 25% jitter
                    let jitter = rand::random::<u64>() % (jitter_range * 2);
                    delay_ms.saturating_sub(jitter_range).saturating_add(jitter)
                } else {
                    delay_ms
                };

                // Wait before retry
                tokio::time::sleep(tokio::time::Duration::from_millis(actual_delay)).await;

                // Exponential backoff
                delay_ms = std::cmp::min(
                    (delay_ms as f64 * config.backoff_multiplier) as u64,
                    config.max_delay_ms,
                );
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        CQRSError::internal_error("Retry operation failed without error")
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_categories() {
        let validation_error = CQRSError::validation_error("Invalid input");
        assert_eq!(validation_error.category(), ErrorCategory::Validation);
        assert_eq!(validation_error.http_status_code(), 400);
        assert_eq!(validation_error.severity(), ErrorSeverity::Low);

        let database_error = CQRSError::database_error("Connection failed");
        assert_eq!(database_error.category(), ErrorCategory::Infrastructure);
        assert_eq!(database_error.http_status_code(), 500);
        assert_eq!(database_error.severity(), ErrorSeverity::High);
    }

    #[test]
    fn test_retryable_errors() {
        let timeout_error = CQRSError::CommandTimeout;
        assert!(timeout_error.is_retryable());

        let validation_error = CQRSError::validation_error("Invalid input");
        assert!(!validation_error.is_retryable());

        let concurrency_error = CQRSError::concurrency_error("Version conflict");
        assert!(concurrency_error.is_retryable());
    }

    #[test]
    fn test_error_context() {
        let context = ErrorContext::new("create_workflow", "command_handler")
            .with_correlation_id(uuid::Uuid::new_v4())
            .with_data("workflow_id", "123");

        assert_eq!(context.operation, "create_workflow");
        assert_eq!(context.component, "command_handler");
        assert!(context.correlation_id.is_some());
        assert!(context.additional_data.contains_key("workflow_id"));
    }

    #[test]
    fn test_contextual_error() {
        let error = CQRSError::validation_error("Test error");
        let context = ErrorContext::new("test_operation", "test_component");
        let contextual_error = ContextualError::new(error, context);

        let error_string = contextual_error.to_string();
        assert!(error_string.contains("test_operation"));
        assert!(error_string.contains("test_component"));
        assert!(error_string.contains("Test error"));
    }

    #[tokio::test]
    async fn test_retry_operation() {
        let config = RetryConfig {
            max_attempts: 3,
            base_delay_ms: 1,
            max_delay_ms: 10,
            backoff_multiplier: 2.0,
            jitter: false,
        };

        let mut attempt_count = 0;
        let result = retry_operation(
            || {
                attempt_count += 1;
                async move {
                    if attempt_count < 3 {
                        Err(CQRSError::CommandTimeout)
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

    #[tokio::test]
    async fn test_retry_non_retryable_error() {
        let config = RetryConfig::default();

        let result = retry_operation(
            || async { Err(CQRSError::validation_error("Not retryable")) },
            &config,
        )
        .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CQRSError::ValidationError(_)));
    }
}
