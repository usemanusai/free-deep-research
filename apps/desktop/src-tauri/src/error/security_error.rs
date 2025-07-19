use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Security-related errors
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum SecurityError {
    #[error("Encryption failed: {message}")]
    EncryptionFailed { message: String },
    
    #[error("Decryption failed: {message}")]
    DecryptionFailed { message: String },
    
    #[error("Invalid master password")]
    InvalidMasterPassword,
    
    #[error("Key derivation failed: {message}")]
    KeyDerivationFailed { message: String },
    
    #[error("Invalid encryption key")]
    InvalidEncryptionKey,
    
    #[error("Audit log write failed: {message}")]
    AuditLogFailed { message: String },
    
    #[error("Authentication required")]
    AuthenticationRequired,
    
    #[error("Session expired")]
    SessionExpired,
    
    #[error("Invalid signature: {resource}")]
    InvalidSignature { resource: String },
    
    #[error("Certificate validation failed: {message}")]
    CertificateValidationFailed { message: String },
    
    #[error("Secure random generation failed")]
    RandomGenerationFailed,
    
    #[error("Key rotation failed: {message}")]
    KeyRotationFailed { message: String },

    #[error("Session limit exceeded")]
    SessionLimitExceeded,
}

impl SecurityError {
    /// Create a new encryption failed error
    pub fn encryption_failed(message: impl Into<String>) -> Self {
        Self::EncryptionFailed {
            message: message.into(),
        }
    }
    
    /// Create a new decryption failed error
    pub fn decryption_failed(message: impl Into<String>) -> Self {
        Self::DecryptionFailed {
            message: message.into(),
        }
    }
    
    /// Create a new key derivation failed error
    pub fn key_derivation_failed(message: impl Into<String>) -> Self {
        Self::KeyDerivationFailed {
            message: message.into(),
        }
    }

    /// Create a session limit exceeded error
    pub fn session_limit_exceeded() -> Self {
        Self::SessionLimitExceeded
    }
    
    /// Check if this error requires user authentication
    pub fn requires_auth(&self) -> bool {
        matches!(
            self,
            SecurityError::AuthenticationRequired
                | SecurityError::SessionExpired
                | SecurityError::InvalidMasterPassword
        )
    }
    
    /// Check if this error indicates a critical security issue
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            SecurityError::InvalidSignature { .. }
                | SecurityError::CertificateValidationFailed { .. }
                | SecurityError::KeyRotationFailed { .. }
        )
    }
}
