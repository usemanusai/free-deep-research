use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Storage-related errors
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum StorageError {
    #[error("Database error: {message}")]
    Database { message: String },
    
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("Permission denied: {path}")]
    PermissionDenied { path: String },
    
    #[error("Disk full: {path}")]
    DiskFull { path: String },
    
    #[error("Corruption detected: {resource}")]
    Corruption { resource: String },
    
    #[error("Backup failed: {message}")]
    BackupFailed { message: String },
    
    #[error("Restore failed: {message}")]
    RestoreFailed { message: String },
    
    #[error("Migration failed: {from_version} -> {to_version}: {message}")]
    MigrationFailed {
        from_version: String,
        to_version: String,
        message: String,
    },
    
    #[error("Lock acquisition failed: {resource}")]
    LockFailed { resource: String },
    
    #[error("Transaction failed: {message}")]
    TransactionFailed { message: String },
    
    #[error("Integrity check failed: {resource}")]
    IntegrityCheckFailed { resource: String },
    
    #[error("Storage quota exceeded: {current}/{limit} bytes")]
    QuotaExceeded { current: u64, limit: u64 },
}

impl StorageError {
    /// Create a new database error
    pub fn database(message: impl Into<String>) -> Self {
        Self::Database {
            message: message.into(),
        }
    }
    
    /// Create a new file not found error
    pub fn file_not_found(path: impl Into<String>) -> Self {
        Self::FileNotFound {
            path: path.into(),
        }
    }
    
    /// Create a new backup failed error
    pub fn backup_failed(message: impl Into<String>) -> Self {
        Self::BackupFailed {
            message: message.into(),
        }
    }
    
    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            StorageError::LockFailed { .. }
                | StorageError::TransactionFailed { .. }
                | StorageError::DiskFull { .. }
        )
    }
    
    /// Check if this error indicates data corruption
    pub fn is_corruption(&self) -> bool {
        matches!(
            self,
            StorageError::Corruption { .. } | StorageError::IntegrityCheckFailed { .. }
        )
    }
}
