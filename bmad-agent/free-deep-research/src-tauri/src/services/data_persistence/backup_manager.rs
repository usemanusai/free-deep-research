use std::sync::Arc;
use std::path::{Path, PathBuf};
use tokio::sync::RwLock;
use tracing::{info, debug, error, warn};
use std::fs;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, StorageError};
use crate::services::security::SecurityService;

/// Backup manager for automated data backups
pub struct BackupManager {
    security: Arc<RwLock<SecurityService>>,
    backup_path: PathBuf,
    source_paths: Vec<PathBuf>,
    max_backups: u32,
}

impl BackupManager {
    /// Create a new backup manager
    pub async fn new(
        security: Arc<RwLock<SecurityService>>,
        backup_path: PathBuf,
        source_paths: Vec<PathBuf>,
    ) -> AppResult<Self> {
        info!("Initializing backup manager");
        
        // Ensure backup directory exists
        fs::create_dir_all(&backup_path)
            .map_err(|e| StorageError::PermissionDenied { path: backup_path.to_string_lossy().to_string() })?;
        
        let manager = Self {
            security,
            backup_path,
            source_paths,
            max_backups: 10, // Keep last 10 backups
        };
        
        info!("Backup manager initialized successfully");
        Ok(manager)
    }
    
    /// Create a full backup
    pub async fn create_backup(&self) -> AppResult<BackupInfo> {
        info!("Creating full backup");
        
        let backup_id = Uuid::new_v4();
        let timestamp = Utc::now();
        let backup_name = format!("backup_{}_{}", 
            timestamp.format("%Y%m%d_%H%M%S"), 
            backup_id.to_string().split('-').next().unwrap_or("unknown")
        );
        
        let backup_dir = self.backup_path.join(&backup_name);
        fs::create_dir_all(&backup_dir)
            .map_err(|e| StorageError::BackupFailed { message: format!("Failed to create backup directory: {}", e) })?;
        
        let mut total_files = 0;
        let mut total_size = 0;
        let mut failed_files = Vec::new();
        
        // Backup each source path
        for source_path in &self.source_paths {
            if source_path.exists() {
                match self.backup_path_recursive(source_path, &backup_dir, source_path).await {
                    Ok((files, size)) => {
                        total_files += files;
                        total_size += size;
                    }
                    Err(e) => {
                        error!("Failed to backup path {:?}: {}", source_path, e);
                        failed_files.push(source_path.to_string_lossy().to_string());
                    }
                }
            } else {
                warn!("Source path does not exist: {:?}", source_path);
            }
        }
        
        // Create backup metadata
        let backup_info = BackupInfo {
            id: backup_id,
            name: backup_name.clone(),
            created_at: timestamp,
            total_files,
            total_size_bytes: total_size,
            source_paths: self.source_paths.iter().map(|p| p.to_string_lossy().to_string()).collect(),
            backup_path: backup_dir.to_string_lossy().to_string(),
            failed_files,
            is_encrypted: true,
        };
        
        // Save backup metadata
        let metadata_path = backup_dir.join("backup_info.json");
        let metadata_json = serde_json::to_string_pretty(&backup_info)
            .map_err(|e| StorageError::BackupFailed { message: format!("Failed to serialize backup metadata: {}", e) })?;
        
        fs::write(&metadata_path, metadata_json)
            .map_err(|e| StorageError::BackupFailed { message: format!("Failed to write backup metadata: {}", e) })?;
        
        info!("Backup created successfully: {} ({} files, {} bytes)", backup_name, total_files, total_size);
        
        // Clean up old backups
        self.cleanup_old_backups().await?;
        
        Ok(backup_info)
    }
    
    /// Backup a path recursively
    async fn backup_path_recursive(
        &self,
        source_path: &Path,
        backup_root: &Path,
        original_root: &Path,
    ) -> AppResult<(u32, u64)> {
        let mut total_files = 0;
        let mut total_size = 0;
        
        if source_path.is_file() {
            // Backup single file
            let relative_path = source_path.strip_prefix(original_root)
                .map_err(|_| StorageError::BackupFailed { message: "Failed to calculate relative path".to_string() })?;
            
            let dest_path = backup_root.join(relative_path);
            
            // Ensure destination directory exists
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| StorageError::BackupFailed { message: format!("Failed to create backup subdirectory: {}", e) })?;
            }
            
            // Read and encrypt file
            let file_data = fs::read(source_path)
                .map_err(|e| StorageError::BackupFailed { message: format!("Failed to read source file: {}", e) })?;
            
            let security = self.security.read().await;
            let encrypted_data = security.encrypt(&file_data).await?;
            drop(security);
            
            // Write encrypted file
            fs::write(&dest_path, encrypted_data)
                .map_err(|e| StorageError::BackupFailed { message: format!("Failed to write backup file: {}", e) })?;
            
            total_files += 1;
            total_size += file_data.len() as u64;
            
        } else if source_path.is_dir() {
            // Backup directory recursively
            let entries = fs::read_dir(source_path)
                .map_err(|e| StorageError::BackupFailed { message: format!("Failed to read directory: {}", e) })?;
            
            for entry in entries {
                let entry = entry
                    .map_err(|e| StorageError::BackupFailed { message: format!("Failed to read directory entry: {}", e) })?;
                
                let (files, size) = self.backup_path_recursive(&entry.path(), backup_root, original_root).await?;
                total_files += files;
                total_size += size;
            }
        }
        
        Ok((total_files, total_size))
    }
    
    /// List all available backups
    pub async fn list_backups(&self) -> AppResult<Vec<BackupInfo>> {
        debug!("Listing available backups");
        
        let mut backups = Vec::new();
        
        if self.backup_path.exists() {
            let entries = fs::read_dir(&self.backup_path)
                .map_err(|e| StorageError::Database { message: format!("Failed to read backup directory: {}", e) })?;
            
            for entry in entries {
                let entry = entry
                    .map_err(|e| StorageError::Database { message: format!("Failed to read backup entry: {}", e) })?;
                
                if entry.path().is_dir() {
                    let metadata_path = entry.path().join("backup_info.json");
                    if metadata_path.exists() {
                        match self.load_backup_info(&metadata_path).await {
                            Ok(backup_info) => backups.push(backup_info),
                            Err(e) => {
                                warn!("Failed to load backup metadata from {:?}: {}", metadata_path, e);
                            }
                        }
                    }
                }
            }
        }
        
        // Sort by creation date (newest first)
        backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        debug!("Found {} backups", backups.len());
        Ok(backups)
    }
    
    /// Load backup information from metadata file
    async fn load_backup_info(&self, metadata_path: &Path) -> AppResult<BackupInfo> {
        let metadata_json = fs::read_to_string(metadata_path)
            .map_err(|e| StorageError::Database { message: format!("Failed to read backup metadata: {}", e) })?;
        
        let backup_info: BackupInfo = serde_json::from_str(&metadata_json)
            .map_err(|e| StorageError::Database { message: format!("Failed to parse backup metadata: {}", e) })?;
        
        Ok(backup_info)
    }
    
    /// Clean up old backups (keep only max_backups)
    async fn cleanup_old_backups(&self) -> AppResult<()> {
        debug!("Cleaning up old backups");
        
        let backups = self.list_backups().await?;
        
        if backups.len() > self.max_backups as usize {
            let backups_to_delete = &backups[self.max_backups as usize..];
            
            for backup in backups_to_delete {
                let backup_path = PathBuf::from(&backup.backup_path);
                if backup_path.exists() {
                    fs::remove_dir_all(&backup_path)
                        .map_err(|e| StorageError::BackupFailed { message: format!("Failed to delete old backup: {}", e) })?;
                    info!("Deleted old backup: {}", backup.name);
                }
            }
        }
        
        Ok(())
    }
    
    /// Delete a specific backup
    pub async fn delete_backup(&self, backup_id: Uuid) -> AppResult<bool> {
        info!("Deleting backup: {}", backup_id);
        
        let backups = self.list_backups().await?;
        
        for backup in backups {
            if backup.id == backup_id {
                let backup_path = PathBuf::from(&backup.backup_path);
                if backup_path.exists() {
                    fs::remove_dir_all(&backup_path)
                        .map_err(|e| StorageError::BackupFailed { message: format!("Failed to delete backup: {}", e) })?;
                    info!("Backup deleted successfully: {}", backup.name);
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
}

/// Backup information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub total_files: u32,
    pub total_size_bytes: u64,
    pub source_paths: Vec<String>,
    pub backup_path: String,
    pub failed_files: Vec<String>,
    pub is_encrypted: bool,
}
