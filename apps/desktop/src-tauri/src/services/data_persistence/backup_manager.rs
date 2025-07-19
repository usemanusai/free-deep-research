use std::sync::Arc;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use tracing::{info, debug, error, warn};
use std::fs;
use std::io::{Read, Write};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use sha2::{Sha256, Digest};

use crate::error::{AppResult, StorageError};
use crate::services::security::SecurityService;

/// Enhanced backup manager for automated incremental data backups
pub struct BackupManager {
    security: Arc<RwLock<SecurityService>>,
    backup_path: PathBuf,
    source_paths: Vec<PathBuf>,
    max_backups: u32,
    backup_config: BackupConfiguration,
    file_checksums: HashMap<PathBuf, String>,
    last_backup_time: Option<DateTime<Utc>>,
    backup_scheduler: Option<tokio::task::JoinHandle<()>>,
}

/// Backup configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfiguration {
    pub auto_backup_enabled: bool,
    pub backup_interval_seconds: u64,
    pub max_backup_files: u32,
    pub backup_compression_enabled: bool,
    pub backup_encryption_enabled: bool,
    pub incremental_backup_enabled: bool,
    pub verify_backup_integrity: bool,
    pub backup_retention_days: u32,
}

impl Default for BackupConfiguration {
    fn default() -> Self {
        Self {
            auto_backup_enabled: true,
            backup_interval_seconds: 30,
            max_backup_files: 100,
            backup_compression_enabled: true,
            backup_encryption_enabled: true,
            incremental_backup_enabled: true,
            verify_backup_integrity: true,
            backup_retention_days: 30,
        }
    }
}

impl BackupManager {
    /// Create a new enhanced backup manager
    pub async fn new(
        security: Arc<RwLock<SecurityService>>,
        backup_path: PathBuf,
        source_paths: Vec<PathBuf>,
    ) -> AppResult<Self> {
        Self::new_with_config(security, backup_path, source_paths, BackupConfiguration::default()).await
    }

    /// Create a new backup manager with custom configuration
    pub async fn new_with_config(
        security: Arc<RwLock<SecurityService>>,
        backup_path: PathBuf,
        source_paths: Vec<PathBuf>,
        config: BackupConfiguration,
    ) -> AppResult<Self> {
        info!("Initializing enhanced backup manager");

        // Ensure backup directory exists
        fs::create_dir_all(&backup_path)
            .map_err(|e| StorageError::PermissionDenied { path: backup_path.to_string_lossy().to_string() })?;

        let mut manager = Self {
            security,
            backup_path,
            source_paths,
            max_backups: config.max_backup_files,
            backup_config: config,
            file_checksums: HashMap::new(),
            last_backup_time: None,
            backup_scheduler: None,
        };

        // Initialize file checksums for incremental backups
        manager.initialize_file_checksums().await?;

        // Start automatic backup scheduler if enabled
        if manager.backup_config.auto_backup_enabled {
            manager.start_backup_scheduler().await?;
        }

        info!("Enhanced backup manager initialized successfully");
        Ok(manager)
    }

    /// Initialize file checksums for incremental backup tracking
    async fn initialize_file_checksums(&mut self) -> AppResult<()> {
        debug!("Initializing file checksums for incremental backups");

        for source_path in &self.source_paths {
            if source_path.exists() {
                self.scan_directory_checksums(source_path).await?;
            }
        }

        info!("Initialized checksums for {} files", self.file_checksums.len());
        Ok(())
    }

    /// Scan directory and calculate checksums
    async fn scan_directory_checksums(&mut self, path: &Path) -> AppResult<()> {
        if path.is_file() {
            let checksum = self.calculate_file_checksum(path).await?;
            self.file_checksums.insert(path.to_path_buf(), checksum);
        } else if path.is_dir() {
            let entries = fs::read_dir(path)
                .map_err(|e| StorageError::BackupFailed { message: format!("Failed to read directory: {}", e) })?;

            for entry in entries {
                let entry = entry
                    .map_err(|e| StorageError::BackupFailed { message: format!("Failed to read directory entry: {}", e) })?;

                self.scan_directory_checksums(&entry.path()).await?;
            }
        }

        Ok(())
    }

    /// Calculate SHA-256 checksum of a file
    async fn calculate_file_checksum(&self, file_path: &Path) -> AppResult<String> {
        let file_data = fs::read(file_path)
            .map_err(|e| StorageError::BackupFailed { message: format!("Failed to read file for checksum: {}", e) })?;

        let mut hasher = Sha256::new();
        hasher.update(&file_data);
        let result = hasher.finalize();

        Ok(format!("{:x}", result))
    }

    /// Start automatic backup scheduler
    async fn start_backup_scheduler(&mut self) -> AppResult<()> {
        if self.backup_scheduler.is_some() {
            return Ok(()); // Already running
        }

        let interval_duration = Duration::from_secs(self.backup_config.backup_interval_seconds);
        let backup_path = self.backup_path.clone();
        let source_paths = self.source_paths.clone();
        let security = self.security.clone();
        let config = self.backup_config.clone();

        let handle = tokio::spawn(async move {
            let mut interval_timer = interval(interval_duration);

            loop {
                interval_timer.tick().await;

                // Create a temporary backup manager for the scheduled backup
                match Self::new_with_config(security.clone(), backup_path.clone(), source_paths.clone(), config.clone()).await {
                    Ok(mut backup_manager) => {
                        if let Err(e) = backup_manager.create_incremental_backup().await {
                            error!("Scheduled backup failed: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("Failed to create backup manager for scheduled backup: {}", e);
                    }
                }
            }
        });

        self.backup_scheduler = Some(handle);
        info!("Backup scheduler started with interval: {} seconds", self.backup_config.backup_interval_seconds);
        Ok(())
    }

    /// Stop automatic backup scheduler
    pub async fn stop_backup_scheduler(&mut self) -> AppResult<()> {
        if let Some(handle) = self.backup_scheduler.take() {
            handle.abort();
            info!("Backup scheduler stopped");
        }
        Ok(())
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

    /// Create an incremental backup of all source paths
    pub async fn create_incremental_backup(&mut self) -> AppResult<BackupInfo> {
        info!("Creating incremental backup...");

        let backup_id = Uuid::new_v4();
        let timestamp = Utc::now();

        // Determine backup type
        let is_full_backup = self.last_backup_time.is_none() || !self.backup_config.incremental_backup_enabled;
        let backup_type = if is_full_backup { "full" } else { "incremental" };

        let backup_name = format!("backup_{}_{}_{}",
            backup_type,
            timestamp.format("%Y%m%d_%H%M%S"),
            backup_id.to_string().split('-').next().unwrap_or("unknown")
        );

        let backup_dir = self.backup_path.join(&backup_name);
        fs::create_dir_all(&backup_dir)
            .map_err(|e| StorageError::BackupFailed { message: format!("Failed to create backup directory: {}", e) })?;

        // Get changed files for incremental backup
        let files_to_backup = if is_full_backup {
            self.get_all_files().await?
        } else {
            self.get_changed_files().await?
        };

        if files_to_backup.is_empty() && !is_full_backup {
            info!("No changes detected, skipping incremental backup");
            return Err(StorageError::BackupFailed {
                message: "No changes detected for incremental backup".to_string()
            }.into());
        }

        let mut total_files = 0;
        let mut total_size = 0;
        let mut failed_files = Vec::new();

        // Backup only changed files
        for source_path in &files_to_backup {
            if source_path.exists() {
                match self.backup_single_file(source_path, &backup_dir).await {
                    Ok((files, size)) => {
                        total_files += files;
                        total_size += size;
                    }
                    Err(e) => {
                        error!("Failed to backup file {:?}: {}", source_path, e);
                        failed_files.push(source_path.to_string_lossy().to_string());
                    }
                }
            }
        }

        // Create backup metadata
        let backup_info = BackupInfo {
            id: backup_id.to_string(),
            filename: backup_name.clone(),
            created_at: timestamp,
            file_size: total_size,
            source_paths: files_to_backup.clone(),
            checksum: self.calculate_directory_checksum(&backup_dir).await?,
            backup_type: backup_type.to_string(),
            files_count: files_to_backup.len(),
            is_encrypted: self.backup_config.backup_encryption_enabled,
            is_compressed: self.backup_config.backup_compression_enabled,
        };

        // Save metadata
        let metadata_path = backup_dir.join("backup_metadata.json");
        let metadata_json = serde_json::to_string_pretty(&backup_info)
            .map_err(|e| StorageError::BackupFailed { message: format!("Failed to serialize backup metadata: {}", e) })?;

        fs::write(&metadata_path, metadata_json)
            .map_err(|e| StorageError::BackupFailed { message: format!("Failed to write backup metadata: {}", e) })?;

        // Update file checksums and last backup time
        self.update_file_checksums(&files_to_backup).await?;
        self.last_backup_time = Some(timestamp);

        // Verify backup integrity if enabled
        if self.backup_config.verify_backup_integrity {
            self.verify_backup_integrity(&backup_dir, &backup_info).await?;
        }

        // Clean up old backups
        self.cleanup_old_backups().await?;

        info!("Incremental backup created successfully: {} ({} files, {} bytes)",
               backup_name, total_files, total_size);
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

/// Enhanced backup information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub id: String,
    pub filename: String,
    pub created_at: DateTime<Utc>,
    pub file_size: u64,
    pub source_paths: Vec<PathBuf>,
    pub checksum: String,
    pub backup_type: String,
    pub files_count: usize,
    pub is_encrypted: bool,
    pub is_compressed: bool,
}
