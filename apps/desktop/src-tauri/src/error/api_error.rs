use serde::{Deserialize, Serialize};
use thiserror::Error;

/// API-related errors
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ApiError {
    #[error("Invalid API key: {service}")]
    InvalidKey { service: String },
    
    #[error("API key not found: {key_id}")]
    KeyNotFound { key_id: String },
    
    #[error("Rate limit exceeded for {service}: {current}/{limit}")]
    RateLimitExceeded {
        service: String,
        current: u32,
        limit: u32,
    },
    
    #[error("Service unavailable: {service}")]
    ServiceUnavailable { service: String },
    
    #[error("Authentication failed for {service}: {message}")]
    AuthenticationFailed { service: String, message: String },
    
    #[error("Request failed for {service}: {status_code} - {message}")]
    RequestFailed {
        service: String,
        status_code: u16,
        message: String,
    },
    
    #[error("Invalid response from {service}: {message}")]
    InvalidResponse { service: String, message: String },
    
    #[error("Connection timeout for {service}")]
    ConnectionTimeout { service: String },
    
    #[error("Service quota exceeded for {service}")]
    QuotaExceeded { service: String },
    
    #[error("Invalid configuration for {service}: {field}")]
    InvalidConfiguration { service: String, field: String },
    
    #[error("Key rotation failed for {service}: {message}")]
    RotationFailed { service: String, message: String },
    
    #[error("Health check failed for {service}: {message}")]
    HealthCheckFailed { service: String, message: String },
}

impl ApiError {
    /// Create a new invalid key error
    pub fn invalid_key(service: impl Into<String>) -> Self {
        Self::InvalidKey {
            service: service.into(),
        }
    }
    
    /// Create a new rate limit exceeded error
    pub fn rate_limit_exceeded(service: impl Into<String>, current: u32, limit: u32) -> Self {
        Self::RateLimitExceeded {
            service: service.into(),
            current,
            limit,
        }
    }
    
    /// Create a new request failed error
    pub fn request_failed(
        service: impl Into<String>,
        status_code: u16,
        message: impl Into<String>,
    ) -> Self {
        Self::RequestFailed {
            service: service.into(),
            status_code,
            message: message.into(),
        }
    }

    /// Create a new invalid configuration error
    pub fn invalid_configuration(service: impl Into<String>, field: impl Into<String>) -> Self {
        Self::InvalidConfiguration {
            service: service.into(),
            field: field.into(),
        }
    }

    /// Create a new key not found error
    pub fn key_not_found(key_id: impl Into<String>) -> Self {
        Self::KeyNotFound {
            key_id: key_id.into(),
        }
    }
    
    /// Check if this error indicates the service is temporarily unavailable
    pub fn is_temporary(&self) -> bool {
        match self {
            ApiError::ServiceUnavailable { .. } => true,
            ApiError::ConnectionTimeout { .. } => true,
            ApiError::RequestFailed { status_code, .. } if *status_code >= 500 => true,
            _ => false,
        }
    }
    
    /// Check if this error indicates a rate limit issue
    pub fn is_rate_limit(&self) -> bool {
        matches!(
            self,
            ApiError::RateLimitExceeded { .. } | ApiError::QuotaExceeded { .. }
        )
    }
    
    /// Check if this error indicates an authentication issue
    pub fn is_auth_error(&self) -> bool {
        matches!(
            self,
            ApiError::InvalidKey { .. } | ApiError::AuthenticationFailed { .. }
        )
    }
}
