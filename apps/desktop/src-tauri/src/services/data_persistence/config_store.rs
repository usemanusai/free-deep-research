use std::sync::Arc;
use std::path::PathBuf;
use tokio::sync::RwLock;
use tracing::{info, debug, error};
use rusqlite::{Connection, params};
use serde_json;
use uuid::Uuid;
use chrono::Utc;

use crate::error::{AppResult, StorageError};
use crate::services::security::SecurityService;
use crate::models::{SystemConfiguration, SecurityConfiguration};

/// Configuration store for system settings
pub struct ConfigStore {
    security: Arc<RwLock<SecurityService>>,
    connection: Arc<RwLock<Connection>>,
}

impl ConfigStore {
    /// Create a new configuration store
    pub async fn new(
        security: Arc<RwLock<SecurityService>>,
        db_path: PathBuf,
    ) -> AppResult<Self> {
        info!("Initializing configuration store");
        
        let conn = Connection::open(&db_path)
            .map_err(|e| StorageError::Database { message: e.to_string() })?;
        
        // Create configuration tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS system_configurations (
                id TEXT PRIMARY KEY,
                config_data TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS security_configurations (
                id TEXT PRIMARY KEY,
                config_data TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;
        
        let store = Self {
            security,
            connection: Arc::new(RwLock::new(conn)),
        };
        
        info!("Configuration store initialized successfully");
        Ok(store)
    }
    
    /// Store system configuration
    pub async fn store_system_config(&self, config: &SystemConfiguration) -> AppResult<()> {
        debug!("Storing system configuration: {}", config.id);
        
        let config_json = serde_json::to_string(config)
            .map_err(|e| StorageError::Database { message: format!("Failed to serialize system config: {}", e) })?;
        
        let conn = self.connection.write().await;
        conn.execute(
            "INSERT OR REPLACE INTO system_configurations (id, config_data, updated_at) 
             VALUES (?1, ?2, CURRENT_TIMESTAMP)",
            params![config.id.to_string(), config_json],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;
        
        debug!("System configuration stored successfully");
        Ok(())
    }
    
    /// Get system configuration
    pub async fn get_system_config(&self, config_id: Uuid) -> AppResult<Option<SystemConfiguration>> {
        debug!("Retrieving system configuration: {}", config_id);
        
        let conn = self.connection.read().await;
        let mut stmt = conn.prepare("SELECT config_data FROM system_configurations WHERE id = ?1")
            .map_err(|e| StorageError::Database { message: e.to_string() })?;
        
        let config_json: Option<String> = stmt.query_row(params![config_id.to_string()], |row| {
            Ok(row.get(0)?)
        }).optional()
            .map_err(|e| StorageError::Database { message: e.to_string() })?;
        
        if let Some(json) = config_json {
            let config: SystemConfiguration = serde_json::from_str(&json)
                .map_err(|e| StorageError::Database { message: format!("Failed to deserialize system config: {}", e) })?;
            
            debug!("System configuration retrieved successfully");
            Ok(Some(config))
        } else {
            debug!("System configuration not found");
            Ok(None)
        }
    }
    
    /// Get current system configuration (latest)
    pub async fn get_current_system_config(&self) -> AppResult<SystemConfiguration> {
        debug!("Retrieving current system configuration");
        
        let conn = self.connection.read().await;
        let mut stmt = conn.prepare(
            "SELECT config_data FROM system_configurations ORDER BY updated_at DESC LIMIT 1"
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;
        
        let config_json: Option<String> = stmt.query_row([], |row| {
            Ok(row.get(0)?)
        }).optional()
            .map_err(|e| StorageError::Database { message: e.to_string() })?;
        
        if let Some(json) = config_json {
            let config: SystemConfiguration = serde_json::from_str(&json)
                .map_err(|e| StorageError::Database { message: format!("Failed to deserialize system config: {}", e) })?;
            
            debug!("Current system configuration retrieved successfully");
            Ok(config)
        } else {
            debug!("No system configuration found, returning default");
            Ok(SystemConfiguration::default())
        }
    }
    
    /// Store security configuration
    pub async fn store_security_config(&self, config: &SecurityConfiguration) -> AppResult<()> {
        debug!("Storing security configuration: {}", config.id);
        
        let config_json = serde_json::to_string(config)
            .map_err(|e| StorageError::Database { message: format!("Failed to serialize security config: {}", e) })?;
        
        // Encrypt sensitive configuration data
        let security = self.security.read().await;
        let encrypted_config = security.encrypt_string(&config_json).await?;
        drop(security);
        
        let conn = self.connection.write().await;
        conn.execute(
            "INSERT OR REPLACE INTO security_configurations (id, config_data, updated_at) 
             VALUES (?1, ?2, CURRENT_TIMESTAMP)",
            params![config.id.to_string(), encrypted_config],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;
        
        debug!("Security configuration stored successfully");
        Ok(())
    }
    
    /// Get security configuration
    pub async fn get_security_config(&self, config_id: Uuid) -> AppResult<Option<SecurityConfiguration>> {
        debug!("Retrieving security configuration: {}", config_id);
        
        let conn = self.connection.read().await;
        let mut stmt = conn.prepare("SELECT config_data FROM security_configurations WHERE id = ?1")
            .map_err(|e| StorageError::Database { message: e.to_string() })?;
        
        let encrypted_config: Option<String> = stmt.query_row(params![config_id.to_string()], |row| {
            Ok(row.get(0)?)
        }).optional()
            .map_err(|e| StorageError::Database { message: e.to_string() })?;
        
        if let Some(encrypted) = encrypted_config {
            // Decrypt configuration data
            let security = self.security.read().await;
            let config_json = security.decrypt_string(&encrypted).await?;
            drop(security);
            
            let config: SecurityConfiguration = serde_json::from_str(&config_json)
                .map_err(|e| StorageError::Database { message: format!("Failed to deserialize security config: {}", e) })?;
            
            debug!("Security configuration retrieved successfully");
            Ok(Some(config))
        } else {
            debug!("Security configuration not found");
            Ok(None)
        }
    }
    
    /// Get current security configuration (latest)
    pub async fn get_current_security_config(&self) -> AppResult<SecurityConfiguration> {
        debug!("Retrieving current security configuration");
        
        let conn = self.connection.read().await;
        let mut stmt = conn.prepare(
            "SELECT config_data FROM security_configurations ORDER BY updated_at DESC LIMIT 1"
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;
        
        let encrypted_config: Option<String> = stmt.query_row([], |row| {
            Ok(row.get(0)?)
        }).optional()
            .map_err(|e| StorageError::Database { message: e.to_string() })?;
        
        if let Some(encrypted) = encrypted_config {
            // Decrypt configuration data
            let security = self.security.read().await;
            let config_json = security.decrypt_string(&encrypted).await?;
            drop(security);
            
            let config: SecurityConfiguration = serde_json::from_str(&config_json)
                .map_err(|e| StorageError::Database { message: format!("Failed to deserialize security config: {}", e) })?;
            
            debug!("Current security configuration retrieved successfully");
            Ok(config)
        } else {
            debug!("No security configuration found, returning default");
            Ok(SecurityConfiguration::default())
        }
    }
    
    /// List all system configurations
    pub async fn list_system_configs(&self) -> AppResult<Vec<SystemConfiguration>> {
        debug!("Listing all system configurations");
        
        let conn = self.connection.read().await;
        let mut stmt = conn.prepare(
            "SELECT config_data FROM system_configurations ORDER BY updated_at DESC"
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;
        
        let config_iter = stmt.query_map([], |row| {
            Ok(row.get::<_, String>(0)?)
        }).map_err(|e| StorageError::Database { message: e.to_string() })?;
        
        let mut configs = Vec::new();
        
        for config_result in config_iter {
            let config_json = config_result
                .map_err(|e| StorageError::Database { message: e.to_string() })?;
            
            match serde_json::from_str::<SystemConfiguration>(&config_json) {
                Ok(config) => configs.push(config),
                Err(e) => {
                    error!("Failed to deserialize system configuration: {}", e);
                }
            }
        }
        
        debug!("Retrieved {} system configurations", configs.len());
        Ok(configs)
    }
    
    /// Delete configuration
    pub async fn delete_system_config(&self, config_id: Uuid) -> AppResult<bool> {
        debug!("Deleting system configuration: {}", config_id);
        
        let conn = self.connection.write().await;
        let rows_affected = conn.execute(
            "DELETE FROM system_configurations WHERE id = ?1",
            params![config_id.to_string()],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;
        
        let deleted = rows_affected > 0;
        if deleted {
            debug!("System configuration deleted successfully");
        } else {
            debug!("System configuration not found for deletion");
        }
        
        Ok(deleted)
    }
    
    /// Delete security configuration
    pub async fn delete_security_config(&self, config_id: Uuid) -> AppResult<bool> {
        debug!("Deleting security configuration: {}", config_id);
        
        let conn = self.connection.write().await;
        let rows_affected = conn.execute(
            "DELETE FROM security_configurations WHERE id = ?1",
            params![config_id.to_string()],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;
        
        let deleted = rows_affected > 0;
        if deleted {
            debug!("Security configuration deleted successfully");
        } else {
            debug!("Security configuration not found for deletion");
        }
        
        Ok(deleted)
    }
}
