use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod api_error;
pub mod research_error;
pub mod storage_error;
pub mod security_error;

pub use api_error::ApiError;
pub use research_error::ResearchError;
pub use storage_error::StorageError;
pub use security_error::SecurityError;

/// Main application error type
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum AppError {
    #[error("API error: {0}")]
    Api(#[from] ApiError),
    
    #[error("Research error: {0}")]
    Research(#[from] ResearchError),
    
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    
    #[error("Security error: {0}")]
    Security(#[from] SecurityError),
    
    #[error("Configuration error: {message}")]
    Configuration { message: String },
    
    #[error("Validation error: {field}: {message}")]
    Validation { field: String, message: String },
    
    #[error("Service unavailable: {service}")]
    ServiceUnavailable { service: String },
    
    #[error("Rate limit exceeded for service: {service}")]
    RateLimitExceeded { service: String },
    
    #[error("Authentication failed: {message}")]
    Authentication { message: String },
    
    #[error("Permission denied: {action}")]
    PermissionDenied { action: String },
    
    #[error("Resource not found: {resource}")]
    NotFound { resource: String },
    
    #[error("Internal error: {message}")]
    Internal { message: String },
    
    #[error("External service error: {service}: {message}")]
    ExternalService { service: String, message: String },
    
    #[error("Timeout error: {operation}")]
    Timeout { operation: String },
    
    #[error("Network error: {message}")]
    Network { message: String },
    
    #[error("Serialization error: {message}")]
    Serialization { message: String },
    
    #[error("IO error: {message}")]
    Io { message: String },
}

impl AppError {
    /// Create a new configuration error
    pub fn configuration(message: impl Into<String>) -> Self {
        Self::Configuration {
            message: message.into(),
        }
    }
    
    /// Create a new validation error
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
        }
    }
    
    /// Create a new internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }
    
    /// Create a new external service error
    pub fn external_service(service: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ExternalService {
            service: service.into(),
            message: message.into(),
        }
    }

    /// Create a new network error
    pub fn network(message: impl Into<String>) -> Self {
        Self::Network {
            message: message.into(),
        }
    }

    /// Create a new IO error
    pub fn io(message: impl Into<String>) -> Self {
        Self::Io {
            message: message.into(),
        }
    }
    
    /// Get the error code for this error
    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::Api(_) => "API_ERROR",
            AppError::Research(_) => "RESEARCH_ERROR",
            AppError::Storage(_) => "STORAGE_ERROR",
            AppError::Security(_) => "SECURITY_ERROR",
            AppError::Configuration { .. } => "CONFIGURATION_ERROR",
            AppError::Validation { .. } => "VALIDATION_ERROR",
            AppError::ServiceUnavailable { .. } => "SERVICE_UNAVAILABLE",
            AppError::RateLimitExceeded { .. } => "RATE_LIMIT_EXCEEDED",
            AppError::Authentication { .. } => "AUTHENTICATION_ERROR",
            AppError::PermissionDenied { .. } => "PERMISSION_DENIED",
            AppError::NotFound { .. } => "NOT_FOUND",
            AppError::Internal { .. } => "INTERNAL_ERROR",
            AppError::ExternalService { .. } => "EXTERNAL_SERVICE_ERROR",
            AppError::Timeout { .. } => "TIMEOUT_ERROR",
            AppError::Network { .. } => "NETWORK_ERROR",
            AppError::Serialization { .. } => "SERIALIZATION_ERROR",
            AppError::Io { .. } => "IO_ERROR",
        }
    }
    
    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            AppError::Network { .. }
                | AppError::Timeout { .. }
                | AppError::ServiceUnavailable { .. }
                | AppError::ExternalService { .. }
        )
    }
    
    /// Check if this error should be logged as an error level
    pub fn is_error_level(&self) -> bool {
        matches!(
            self,
            AppError::Internal { .. }
                | AppError::Security(_)
                | AppError::Storage(_)
                | AppError::Configuration { .. }
        )
    }
}

// Implement conversions from common error types
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io {
            message: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Serialization {
            message: err.to_string(),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            AppError::Timeout {
                operation: "HTTP request".to_string(),
            }
        } else if err.is_connect() {
            AppError::Network {
                message: err.to_string(),
            }
        } else {
            AppError::ExternalService {
                service: "HTTP".to_string(),
                message: err.to_string(),
            }
        }
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::Storage(StorageError::Database {
            message: err.to_string(),
        })
    }
}

impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        AppError::configuration(err.to_string())
    }
}

/// Result type alias for application operations
pub type AppResult<T> = Result<T, AppError>;
