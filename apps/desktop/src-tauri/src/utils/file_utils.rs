use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, error};

use crate::error::{AppResult, AppError};

/// Create a directory if it doesn't exist
pub fn ensure_dir_exists<P: AsRef<Path>>(path: P) -> AppResult<()> {
    let path = path.as_ref();
    
    if !path.exists() {
        debug!("Creating directory: {:?}", path);
        fs::create_dir_all(path)
            .map_err(|e| AppError::io(e.to_string()))?;
    }
    
    Ok(())
}

/// Read a file to string
pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> AppResult<String> {
    let path = path.as_ref();
    debug!("Reading file: {:?}", path);
    
    fs::read_to_string(path)
        .map_err(|e| AppError::io(e.to_string()))
}

/// Write string to file
pub fn write_string_to_file<P: AsRef<Path>>(path: P, content: &str) -> AppResult<()> {
    let path = path.as_ref();
    debug!("Writing file: {:?}", path);
    
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        ensure_dir_exists(parent)?;
    }
    
    fs::write(path, content)
        .map_err(|e| AppError::io(e.to_string()))
}

/// Read file as bytes
pub fn read_file_bytes<P: AsRef<Path>>(path: P) -> AppResult<Vec<u8>> {
    let path = path.as_ref();
    debug!("Reading file bytes: {:?}", path);
    
    fs::read(path)
        .map_err(|e| AppError::io(e.to_string()))
}

/// Write bytes to file
pub fn write_file_bytes<P: AsRef<Path>>(path: P, content: &[u8]) -> AppResult<()> {
    let path = path.as_ref();
    debug!("Writing file bytes: {:?}", path);
    
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        ensure_dir_exists(parent)?;
    }
    
    fs::write(path, content)
        .map_err(|e| AppError::io(e.to_string()))
}

/// Check if file exists
pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

/// Get file size in bytes
pub fn get_file_size<P: AsRef<Path>>(path: P) -> AppResult<u64> {
    let path = path.as_ref();
    
    let metadata = fs::metadata(path)
        .map_err(|e| AppError::io(e.to_string()))?;
    
    Ok(metadata.len())
}

/// Delete a file
pub fn delete_file<P: AsRef<Path>>(path: P) -> AppResult<()> {
    let path = path.as_ref();
    debug!("Deleting file: {:?}", path);
    
    if path.exists() {
        fs::remove_file(path)
            .map_err(|e| AppError::io(e.to_string()))?;
    }
    
    Ok(())
}

/// Copy a file
pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> AppResult<()> {
    let from = from.as_ref();
    let to = to.as_ref();
    debug!("Copying file from {:?} to {:?}", from, to);
    
    // Ensure destination directory exists
    if let Some(parent) = to.parent() {
        ensure_dir_exists(parent)?;
    }
    
    fs::copy(from, to)
        .map_err(|e| AppError::io(e.to_string()))?;
    
    Ok(())
}

/// Get application data directory
pub fn get_app_data_dir() -> AppResult<PathBuf> {
    let app_data = dirs::data_dir()
        .ok_or_else(|| AppError::internal("Could not determine data directory"))?;
    
    let app_dir = app_data.join("free-deep-research");
    ensure_dir_exists(&app_dir)?;
    
    Ok(app_dir)
}

/// Get application config directory
pub fn get_app_config_dir() -> AppResult<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| AppError::internal("Could not determine config directory"))?;
    
    let app_dir = config_dir.join("free-deep-research");
    ensure_dir_exists(&app_dir)?;
    
    Ok(app_dir)
}

/// Get application cache directory
pub fn get_app_cache_dir() -> AppResult<PathBuf> {
    let cache_dir = dirs::cache_dir()
        .ok_or_else(|| AppError::internal("Could not determine cache directory"))?;
    
    let app_dir = cache_dir.join("free-deep-research");
    ensure_dir_exists(&app_dir)?;
    
    Ok(app_dir)
}

/// List files in directory
pub fn list_files_in_dir<P: AsRef<Path>>(path: P) -> AppResult<Vec<PathBuf>> {
    let path = path.as_ref();
    debug!("Listing files in directory: {:?}", path);
    
    let mut files = Vec::new();
    
    let entries = fs::read_dir(path)
        .map_err(|e| AppError::io(e.to_string()))?;
    
    for entry in entries {
        let entry = entry.map_err(|e| AppError::io(e.to_string()))?;
        let path = entry.path();
        
        if path.is_file() {
            files.push(path);
        }
    }
    
    Ok(files)
}
