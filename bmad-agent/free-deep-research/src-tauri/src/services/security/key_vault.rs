use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{info, debug, error};
use rusqlite::{Connection, params};
use dirs;
use std::path::PathBuf;

use crate::error::{AppResult, SecurityError, StorageError};
use crate::services::security::EncryptionManager;
use crate::utils::file_utils::ensure_dir_exists;

/// Key vault for storing encrypted secrets
pub struct KeyVault {
    encryption_manager: Arc<RwLock<EncryptionManager>>,
    db_path: PathBuf,
    connection: Option<Connection>,
}

impl KeyVault {
    /// Create a new key vault
    pub async fn new(encryption_manager: Arc<RwLock<EncryptionManager>>) -> AppResult<Self> {
        info!("Initializing key vault...");

        // Determine database path
        let mut db_path = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("free-deep-research");

        ensure_dir_exists(&db_path)?;
        db_path.push("key_vault.db");

        let mut vault = Self {
            encryption_manager,
            db_path,
            connection: None,
        };

        // Initialize database
        vault.initialize_database().await?;

        info!("Key vault initialized successfully");
        Ok(vault)
    }

    /// Initialize the database schema
    async fn initialize_database(&mut self) -> AppResult<()> {
        debug!("Initializing key vault database");

        let conn = Connection::open(&self.db_path)
            .map_err(|e| StorageError::Database { message: e.to_string() })?;

        // Create secrets table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS secrets (
                key TEXT PRIMARY KEY,
                encrypted_value BLOB NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        // Create index for faster lookups
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_secrets_created_at ON secrets(created_at)",
            [],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        self.connection = Some(conn);
        debug!("Key vault database initialized");
        Ok(())
    }

    /// Store a secret
    pub async fn store_secret(&mut self, key: &str, value: &str) -> AppResult<()> {
        debug!("Storing secret with key: {}", key);

        // Encrypt the value
        let encryption_manager = self.encryption_manager.read().await;
        if !encryption_manager.is_initialized() {
            return Err(SecurityError::encryption_failed("Encryption not initialized").into());
        }

        let encrypted_value = encryption_manager.encrypt(value.as_bytes()).await?;
        drop(encryption_manager);

        // Store in database
        let conn = self.connection.as_ref()
            .ok_or_else(|| StorageError::Database { message: "Database not initialized".to_string() })?;

        conn.execute(
            "INSERT OR REPLACE INTO secrets (key, encrypted_value, updated_at)
             VALUES (?1, ?2, CURRENT_TIMESTAMP)",
            params![key, encrypted_value],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        debug!("Secret stored successfully");
        Ok(())
    }

    /// Retrieve a secret
    pub async fn get_secret(&self, key: &str) -> AppResult<Option<String>> {
        debug!("Retrieving secret with key: {}", key);

        let conn = self.connection.as_ref()
            .ok_or_else(|| StorageError::Database { message: "Database not initialized".to_string() })?;

        // Query the database
        let mut stmt = conn.prepare("SELECT encrypted_value FROM secrets WHERE key = ?1")
            .map_err(|e| StorageError::Database { message: e.to_string() })?;

        let encrypted_value: Option<Vec<u8>> = stmt.query_row(params![key], |row| {
            Ok(row.get(0)?)
        }).optional()
            .map_err(|e| StorageError::Database { message: e.to_string() })?;

        if let Some(encrypted_data) = encrypted_value {
            // Decrypt the value
            let encryption_manager = self.encryption_manager.read().await;
            if !encryption_manager.is_initialized() {
                return Err(SecurityError::encryption_failed("Encryption not initialized").into());
            }

            let decrypted_bytes = encryption_manager.decrypt(&encrypted_data).await?;
            let decrypted_string = String::from_utf8(decrypted_bytes)
                .map_err(|_| SecurityError::decryption_failed("Invalid UTF-8 in decrypted data"))?;

            debug!("Secret retrieved successfully");
            Ok(Some(decrypted_string))
        } else {
            debug!("Secret not found");
            Ok(None)
        }
    }

    /// Delete a secret
    pub async fn delete_secret(&mut self, key: &str) -> AppResult<()> {
        debug!("Deleting secret with key: {}", key);

        let conn = self.connection.as_ref()
            .ok_or_else(|| StorageError::Database { message: "Database not initialized".to_string() })?;

        let rows_affected = conn.execute(
            "DELETE FROM secrets WHERE key = ?1",
            params![key],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        if rows_affected > 0 {
            debug!("Secret deleted successfully");
        } else {
            debug!("Secret not found for deletion");
        }

        Ok(())
    }

    /// List all secret keys (without values)
    pub async fn list_keys(&self) -> AppResult<Vec<String>> {
        debug!("Listing all secret keys");

        let conn = self.connection.as_ref()
            .ok_or_else(|| StorageError::Database { message: "Database not initialized".to_string() })?;

        let mut stmt = conn.prepare("SELECT key FROM secrets ORDER BY created_at")
            .map_err(|e| StorageError::Database { message: e.to_string() })?;

        let keys: Result<Vec<String>, rusqlite::Error> = stmt.query_map([], |row| {
            Ok(row.get(0)?)
        })?.collect();

        let keys = keys.map_err(|e| StorageError::Database { message: e.to_string() })?;

        debug!("Found {} secret keys", keys.len());
        Ok(keys)
    }

    /// Check if a secret exists
    pub async fn has_secret(&self, key: &str) -> AppResult<bool> {
        debug!("Checking if secret exists: {}", key);

        let conn = self.connection.as_ref()
            .ok_or_else(|| StorageError::Database { message: "Database not initialized".to_string() })?;

        let mut stmt = conn.prepare("SELECT 1 FROM secrets WHERE key = ?1 LIMIT 1")
            .map_err(|e| StorageError::Database { message: e.to_string() })?;

        let exists = stmt.exists(params![key])
            .map_err(|e| StorageError::Database { message: e.to_string() })?;

        debug!("Secret exists: {}", exists);
        Ok(exists)
    }

    /// Clear all secrets (use with caution)
    pub async fn clear_all_secrets(&mut self) -> AppResult<u32> {
        info!("Clearing all secrets from key vault");

        let conn = self.connection.as_ref()
            .ok_or_else(|| StorageError::Database { message: "Database not initialized".to_string() })?;

        let rows_affected = conn.execute("DELETE FROM secrets", [])
            .map_err(|e| StorageError::Database { message: e.to_string() })?;

        info!("Cleared {} secrets from key vault", rows_affected);
        Ok(rows_affected as u32)
    }

    /// Shutdown the key vault
    pub async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down key vault...");
        // Connection will be dropped automatically
        Ok(())
    }
}

/// Key vault statistics
#[derive(Debug)]
pub struct VaultStatistics {
    pub total_secrets: u32,
    pub oldest_secret: Option<String>,
    pub newest_secret: Option<String>,
}
