use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, error};
use std::path::{Path, PathBuf};
use std::fs;

use crate::error::{AppResult, StorageError, SecurityError};
use crate::services::security::SecurityService;

/// Encrypted storage manager for sensitive data
pub struct EncryptedStorage {
    security: Arc<RwLock<SecurityService>>,
    storage_path: PathBuf,
}

impl EncryptedStorage {
    /// Create a new encrypted storage manager
    pub async fn new(security: Arc<RwLock<SecurityService>>, storage_path: PathBuf) -> AppResult<Self> {
        info!("Initializing encrypted storage at: {:?}", storage_path);
        
        // Ensure storage directory exists
        if let Some(parent) = storage_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| StorageError::PermissionDenied { path: parent.to_string_lossy().to_string() })?;
        }
        
        let storage = Self {
            security,
            storage_path,
        };
        
        info!("Encrypted storage initialized successfully");
        Ok(storage)
    }
    
    /// Store encrypted data
    pub async fn store(&self, key: &str, data: &[u8]) -> AppResult<()> {
        debug!("Storing encrypted data for key: {}", key);
        
        // Encrypt the data
        let security = self.security.read().await;
        let encrypted_data = security.encrypt(data).await?;
        drop(security);
        
        // Create file path
        let file_path = self.storage_path.join(format!("{}.enc", key));
        
        // Write encrypted data to file
        fs::write(&file_path, encrypted_data)
            .map_err(|e| StorageError::Database { message: format!("Failed to write encrypted file: {}", e) })?;
        
        debug!("Encrypted data stored successfully");
        Ok(())
    }
    
    /// Retrieve and decrypt data
    pub async fn retrieve(&self, key: &str) -> AppResult<Option<Vec<u8>>> {
        debug!("Retrieving encrypted data for key: {}", key);
        
        let file_path = self.storage_path.join(format!("{}.enc", key));
        
        // Check if file exists
        if !file_path.exists() {
            debug!("Encrypted file not found");
            return Ok(None);
        }
        
        // Read encrypted data from file
        let encrypted_data = fs::read(&file_path)
            .map_err(|e| StorageError::Database { message: format!("Failed to read encrypted file: {}", e) })?;
        
        // Decrypt the data
        let security = self.security.read().await;
        let decrypted_data = security.decrypt(&encrypted_data).await?;
        drop(security);
        
        debug!("Encrypted data retrieved and decrypted successfully");
        Ok(Some(decrypted_data))
    }
    
    /// Delete encrypted data
    pub async fn delete(&self, key: &str) -> AppResult<bool> {
        debug!("Deleting encrypted data for key: {}", key);
        
        let file_path = self.storage_path.join(format!("{}.enc", key));
        
        if file_path.exists() {
            fs::remove_file(&file_path)
                .map_err(|e| StorageError::Database { message: format!("Failed to delete encrypted file: {}", e) })?;
            debug!("Encrypted file deleted successfully");
            Ok(true)
        } else {
            debug!("Encrypted file not found for deletion");
            Ok(false)
        }
    }
    
    /// List all stored keys
    pub async fn list_keys(&self) -> AppResult<Vec<String>> {
        debug!("Listing all encrypted storage keys");
        
        let mut keys = Vec::new();
        
        if self.storage_path.exists() {
            let entries = fs::read_dir(&self.storage_path)
                .map_err(|e| StorageError::Database { message: format!("Failed to read storage directory: {}", e) })?;
            
            for entry in entries {
                let entry = entry
                    .map_err(|e| StorageError::Database { message: format!("Failed to read directory entry: {}", e) })?;
                
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();
                
                if file_name_str.ends_with(".enc") {
                    let key = file_name_str.trim_end_matches(".enc").to_string();
                    keys.push(key);
                }
            }
        }
        
        debug!("Found {} encrypted storage keys", keys.len());
        Ok(keys)
    }
    
    /// Get storage statistics
    pub async fn get_statistics(&self) -> AppResult<StorageStatistics> {
        debug!("Getting encrypted storage statistics");
        
        let mut total_files = 0;
        let mut total_size = 0;
        
        if self.storage_path.exists() {
            let entries = fs::read_dir(&self.storage_path)
                .map_err(|e| StorageError::Database { message: format!("Failed to read storage directory: {}", e) })?;
            
            for entry in entries {
                let entry = entry
                    .map_err(|e| StorageError::Database { message: format!("Failed to read directory entry: {}", e) })?;
                
                let file_name = entry.file_name();
                if file_name.to_string_lossy().ends_with(".enc") {
                    total_files += 1;
                    
                    let metadata = entry.metadata()
                        .map_err(|e| StorageError::Database { message: format!("Failed to read file metadata: {}", e) })?;
                    total_size += metadata.len();
                }
            }
        }
        
        Ok(StorageStatistics {
            total_files,
            total_size_bytes: total_size,
            storage_path: self.storage_path.clone(),
        })
    }
    
    /// Clear all encrypted data (use with extreme caution)
    pub async fn clear_all(&self) -> AppResult<u32> {
        info!("Clearing all encrypted storage data");
        
        let mut deleted_count = 0;
        
        if self.storage_path.exists() {
            let entries = fs::read_dir(&self.storage_path)
                .map_err(|e| StorageError::Database { message: format!("Failed to read storage directory: {}", e) })?;
            
            for entry in entries {
                let entry = entry
                    .map_err(|e| StorageError::Database { message: format!("Failed to read directory entry: {}", e) })?;
                
                let file_name = entry.file_name();
                if file_name.to_string_lossy().ends_with(".enc") {
                    fs::remove_file(entry.path())
                        .map_err(|e| StorageError::Database { message: format!("Failed to delete encrypted file: {}", e) })?;
                    deleted_count += 1;
                }
            }
        }
        
        info!("Cleared {} encrypted files", deleted_count);
        Ok(deleted_count)
    }
}

/// Storage statistics
#[derive(Debug)]
pub struct StorageStatistics {
    pub total_files: u32,
    pub total_size_bytes: u64,
    pub storage_path: PathBuf,
}
