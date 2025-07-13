use std::sync::Arc;
use std::path::PathBuf;
use tokio::sync::RwLock;
use tracing::{info, debug, error};
use rusqlite::{Connection, params};
use dirs;
use serde_json;
use uuid::Uuid;
use chrono::Utc;

use crate::error::{AppResult, StorageError};
use crate::services::{Service, SecurityService};
use crate::models::{ApiKey, SystemConfiguration};
use crate::utils::file_utils::ensure_dir_exists;

pub mod encrypted_storage;
pub mod backup_manager;
pub mod config_store;

/// Data Persistence Service that manages encrypted storage and backups
pub struct DataPersistenceService {
    security: Arc<RwLock<SecurityService>>,
    db_path: PathBuf,
    connection: Option<Connection>,
}

impl DataPersistenceService {
    /// Create a new data persistence service
    pub async fn new(security: Arc<RwLock<SecurityService>>) -> AppResult<Self> {
        info!("Initializing data persistence service...");

        // Determine database path
        let mut db_path = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("free-deep-research");

        ensure_dir_exists(&db_path)?;
        db_path.push("app_data.db");

        let mut service = Self {
            security,
            db_path,
            connection: None,
        };

        // Initialize database
        service.initialize_database().await?;

        info!("Data persistence service initialized successfully");
        Ok(service)
    }

    /// Initialize the database schema
    async fn initialize_database(&mut self) -> AppResult<()> {
        debug!("Initializing application database");

        let conn = Connection::open(&self.db_path)
            .map_err(|e| StorageError::Database { message: e.to_string() })?;

        // Create API keys table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS api_keys (
                id TEXT PRIMARY KEY,
                service TEXT NOT NULL,
                name TEXT NOT NULL,
                encrypted_key BLOB NOT NULL,
                usage_count INTEGER DEFAULT 0,
                rate_limit INTEGER NOT NULL,
                reset_period TEXT NOT NULL,
                last_used DATETIME,
                last_reset DATETIME NOT NULL,
                status TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        // Create system configuration table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS system_config (
                id TEXT PRIMARY KEY,
                backup_interval INTEGER NOT NULL,
                encryption_enabled BOOLEAN NOT NULL,
                rate_limit_buffer INTEGER NOT NULL,
                monitoring_enabled BOOLEAN NOT NULL,
                log_level TEXT NOT NULL,
                ui_theme TEXT NOT NULL,
                auto_start_monitoring BOOLEAN NOT NULL,
                max_concurrent_research INTEGER NOT NULL,
                data_retention_days INTEGER NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        // Create research workflows table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS research_workflows (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                query TEXT NOT NULL,
                status TEXT NOT NULL,
                methodology TEXT NOT NULL,
                parameters TEXT,
                results TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                completed_at DATETIME
            )",
            [],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        // Create API usage statistics table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS api_usage_stats (
                id TEXT PRIMARY KEY,
                api_key_id TEXT NOT NULL,
                service TEXT NOT NULL,
                endpoint TEXT,
                request_count INTEGER DEFAULT 1,
                success_count INTEGER DEFAULT 0,
                error_count INTEGER DEFAULT 0,
                total_response_time_ms INTEGER DEFAULT 0,
                avg_response_time_ms REAL DEFAULT 0,
                last_used DATETIME DEFAULT CURRENT_TIMESTAMP,
                date_bucket TEXT NOT NULL,
                FOREIGN KEY (api_key_id) REFERENCES api_keys (id) ON DELETE CASCADE
            )",
            [],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        // Create indexes for better performance
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_api_keys_service ON api_keys(service)",
            [],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_api_keys_status ON api_keys(status)",
            [],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_workflows_status ON research_workflows(status)",
            [],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_usage_stats_api_key ON api_usage_stats(api_key_id)",
            [],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_usage_stats_date ON api_usage_stats(date_bucket)",
            [],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        self.connection = Some(conn);
        debug!("Application database initialized");
        Ok(())
    }

    /// Store an API key
    pub async fn store_api_key(&mut self, api_key: &ApiKey) -> AppResult<()> {
        debug!("Storing API key: {}", api_key.id);

        let conn = self.connection.as_ref()
            .ok_or_else(|| StorageError::Database { message: "Database not initialized".to_string() })?;

        conn.execute(
            "INSERT OR REPLACE INTO api_keys (
                id, service, name, encrypted_key, usage_count, rate_limit,
                reset_period, last_used, last_reset, status, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, CURRENT_TIMESTAMP)",
            params![
                api_key.id.to_string(),
                format!("{:?}", api_key.service),
                api_key.name,
                api_key.encrypted_key.as_bytes(),
                api_key.usage_count,
                api_key.rate_limit,
                format!("{:?}", api_key.reset_period),
                api_key.last_used.map(|dt| dt.to_rfc3339()),
                api_key.last_reset.to_rfc3339(),
                format!("{:?}", api_key.status),
            ],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        debug!("API key stored successfully");
        Ok(())
    }

    /// Get all API keys
    pub async fn get_all_api_keys(&self) -> AppResult<Vec<ApiKey>> {
        debug!("Retrieving all API keys");

        let conn = self.connection.as_ref()
            .ok_or_else(|| StorageError::Database { message: "Database not initialized".to_string() })?;

        let mut stmt = conn.prepare(
            "SELECT id, service, name, encrypted_key, usage_count, rate_limit,
                    reset_period, last_used, last_reset, status, created_at, updated_at
             FROM api_keys ORDER BY created_at DESC"
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        let key_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,  // id
                row.get::<_, String>(1)?,  // service
                row.get::<_, String>(2)?,  // name
                row.get::<_, Vec<u8>>(3)?, // encrypted_key
                row.get::<_, u32>(4)?,     // usage_count
                row.get::<_, u32>(5)?,     // rate_limit
                row.get::<_, String>(6)?,  // reset_period
                row.get::<_, Option<String>>(7)?, // last_used
                row.get::<_, String>(8)?,  // last_reset
                row.get::<_, String>(9)?,  // status
                row.get::<_, String>(10)?, // created_at
                row.get::<_, String>(11)?, // updated_at
            ))
        }).map_err(|e| StorageError::Database { message: e.to_string() })?;

        let mut api_keys = Vec::new();

        for key_result in key_iter {
            let (id_str, service_str, name, encrypted_key_bytes, usage_count, rate_limit,
                 reset_period_str, last_used_str, last_reset_str, status_str, created_at_str, updated_at_str) =
                key_result.map_err(|e| StorageError::Database { message: e.to_string() })?;

            // Parse the data (simplified parsing for now)
            let id = id_str.parse()
                .map_err(|_| StorageError::Database { message: "Invalid UUID in API key".to_string() })?;

            let encrypted_key = String::from_utf8(encrypted_key_bytes)
                .map_err(|_| StorageError::Database { message: "Invalid encrypted key format".to_string() })?;

            let last_used = if let Some(last_used_str) = last_used_str {
                Some(chrono::DateTime::parse_from_rfc3339(&last_used_str)
                    .map_err(|_| StorageError::Database { message: "Invalid last_used timestamp".to_string() })?
                    .with_timezone(&Utc))
            } else {
                None
            };

            let last_reset = chrono::DateTime::parse_from_rfc3339(&last_reset_str)
                .map_err(|_| StorageError::Database { message: "Invalid last_reset timestamp".to_string() })?
                .with_timezone(&Utc);

            let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|_| StorageError::Database { message: "Invalid created_at timestamp".to_string() })?
                .with_timezone(&Utc);

            let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                .map_err(|_| StorageError::Database { message: "Invalid updated_at timestamp".to_string() })?
                .with_timezone(&Utc);

            // Parse enums properly
            use crate::models::api_key::{ServiceProvider, ResetPeriod, ApiKeyStatus};

            let service = ServiceProvider::from_str(&service_str)
                .unwrap_or(ServiceProvider::OpenRouter);

            let reset_period = ResetPeriod::from_str(&reset_period_str)
                .unwrap_or(ResetPeriod::Daily);

            let status = ApiKeyStatus::from_str(&status_str)
                .unwrap_or(ApiKeyStatus::Active);

            api_keys.push(ApiKey {
                id,
                service,
                name,
                encrypted_key,
                usage_count,
                rate_limit,
                reset_period,
                last_used,
                last_reset,
                status,
                created_at,
                updated_at,
            });
        }

        debug!("Retrieved {} API keys", api_keys.len());
        Ok(api_keys)
    }

    /// Delete an API key
    pub async fn delete_api_key(&mut self, key_id: Uuid) -> AppResult<()> {
        debug!("Deleting API key: {}", key_id);

        let conn = self.connection.as_ref()
            .ok_or_else(|| StorageError::Database { message: "Database not initialized".to_string() })?;

        let rows_affected = conn.execute(
            "DELETE FROM api_keys WHERE id = ?1",
            params![key_id.to_string()],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        if rows_affected == 0 {
            return Err(StorageError::Database {
                message: format!("API key with ID {} not found", key_id)
            }.into());
        }

        debug!("API key deleted successfully");
        Ok(())
    }

    /// Get API key by ID
    pub async fn get_api_key_by_id(&self, key_id: Uuid) -> AppResult<Option<ApiKey>> {
        debug!("Retrieving API key by ID: {}", key_id);

        let conn = self.connection.as_ref()
            .ok_or_else(|| StorageError::Database { message: "Database not initialized".to_string() })?;

        let mut stmt = conn.prepare(
            "SELECT id, service, name, encrypted_key, usage_count, rate_limit,
                    reset_period, last_used, last_reset, status, created_at, updated_at
             FROM api_keys WHERE id = ?1"
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        let mut key_iter = stmt.query_map([key_id.to_string()], |row| {
            Ok((
                row.get::<_, String>(0)?,  // id
                row.get::<_, String>(1)?,  // service
                row.get::<_, String>(2)?,  // name
                row.get::<_, Vec<u8>>(3)?, // encrypted_key
                row.get::<_, u32>(4)?,     // usage_count
                row.get::<_, u32>(5)?,     // rate_limit
                row.get::<_, String>(6)?,  // reset_period
                row.get::<_, Option<String>>(7)?, // last_used
                row.get::<_, String>(8)?,  // last_reset
                row.get::<_, String>(9)?,  // status
                row.get::<_, String>(10)?, // created_at
                row.get::<_, String>(11)?, // updated_at
            ))
        }).map_err(|e| StorageError::Database { message: e.to_string() })?;

        if let Some(key_result) = key_iter.next() {
            let (id_str, service_str, name, encrypted_key_bytes, usage_count, rate_limit,
                 reset_period_str, last_used_str, last_reset_str, status_str, created_at_str, updated_at_str) =
                key_result.map_err(|e| StorageError::Database { message: e.to_string() })?;

            // Parse the data
            let id = id_str.parse()
                .map_err(|_| StorageError::Database { message: "Invalid UUID in API key".to_string() })?;

            let encrypted_key = String::from_utf8(encrypted_key_bytes)
                .map_err(|_| StorageError::Database { message: "Invalid encrypted key format".to_string() })?;

            let last_used = if let Some(last_used_str) = last_used_str {
                Some(chrono::DateTime::parse_from_rfc3339(&last_used_str)
                    .map_err(|_| StorageError::Database { message: "Invalid last_used timestamp".to_string() })?
                    .with_timezone(&Utc))
            } else {
                None
            };

            let last_reset = chrono::DateTime::parse_from_rfc3339(&last_reset_str)
                .map_err(|_| StorageError::Database { message: "Invalid last_reset timestamp".to_string() })?
                .with_timezone(&Utc);

            let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|_| StorageError::Database { message: "Invalid created_at timestamp".to_string() })?
                .with_timezone(&Utc);

            let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                .map_err(|_| StorageError::Database { message: "Invalid updated_at timestamp".to_string() })?
                .with_timezone(&Utc);

            // Parse enums properly
            use crate::models::api_key::{ServiceProvider, ResetPeriod, ApiKeyStatus};

            let service = ServiceProvider::from_str(&service_str)
                .unwrap_or(ServiceProvider::OpenRouter);

            let reset_period = ResetPeriod::from_str(&reset_period_str)
                .unwrap_or(ResetPeriod::Daily);

            let status = ApiKeyStatus::from_str(&status_str)
                .unwrap_or(ApiKeyStatus::Active);

            Ok(Some(ApiKey {
                id,
                service,
                name,
                encrypted_key,
                usage_count,
                rate_limit,
                reset_period,
                last_used,
                last_reset,
                status,
                created_at,
                updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    /// Record API usage statistics
    pub async fn record_api_usage(&mut self,
        api_key_id: Uuid,
        service: &str,
        endpoint: Option<&str>,
        success: bool,
        response_time_ms: u32
    ) -> AppResult<()> {
        debug!("Recording API usage for key: {}", api_key_id);

        let conn = self.connection.as_ref()
            .ok_or_else(|| StorageError::Database { message: "Database not initialized".to_string() })?;

        let date_bucket = Utc::now().format("%Y-%m-%d").to_string();
        let endpoint_str = endpoint.unwrap_or("unknown");

        // Try to update existing record first
        let rows_affected = conn.execute(
            "UPDATE api_usage_stats SET
                request_count = request_count + 1,
                success_count = success_count + CASE WHEN ?5 THEN 1 ELSE 0 END,
                error_count = error_count + CASE WHEN ?5 THEN 0 ELSE 1 END,
                total_response_time_ms = total_response_time_ms + ?6,
                avg_response_time_ms = CAST(total_response_time_ms + ?6 AS REAL) / (request_count + 1),
                last_used = CURRENT_TIMESTAMP
             WHERE api_key_id = ?1 AND service = ?2 AND endpoint = ?3 AND date_bucket = ?4",
            params![
                api_key_id.to_string(),
                service,
                endpoint_str,
                date_bucket,
                success,
                response_time_ms
            ],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        // If no existing record, create new one
        if rows_affected == 0 {
            conn.execute(
                "INSERT INTO api_usage_stats (
                    id, api_key_id, service, endpoint, request_count, success_count,
                    error_count, total_response_time_ms, avg_response_time_ms, date_bucket
                ) VALUES (?1, ?2, ?3, ?4, 1, ?5, ?6, ?7, ?7, ?8)",
                params![
                    Uuid::new_v4().to_string(),
                    api_key_id.to_string(),
                    service,
                    endpoint_str,
                    if success { 1 } else { 0 },
                    if success { 0 } else { 1 },
                    response_time_ms,
                    date_bucket
                ],
            ).map_err(|e| StorageError::Database { message: e.to_string() })?;
        }

        debug!("API usage recorded successfully");
        Ok(())
    }

    /// Get usage statistics for an API key
    pub async fn get_api_key_usage_stats(&self, api_key_id: Uuid, days: u32) -> AppResult<Vec<(String, u32, u32, u32, f64)>> {
        debug!("Getting usage statistics for API key: {}", api_key_id);

        let conn = self.connection.as_ref()
            .ok_or_else(|| StorageError::Database { message: "Database not initialized".to_string() })?;

        let start_date = (Utc::now() - chrono::Duration::days(days as i64))
            .format("%Y-%m-%d").to_string();

        let mut stmt = conn.prepare(
            "SELECT date_bucket, request_count, success_count, error_count, avg_response_time_ms
             FROM api_usage_stats
             WHERE api_key_id = ?1 AND date_bucket >= ?2
             ORDER BY date_bucket DESC"
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        let stats_iter = stmt.query_map([api_key_id.to_string(), start_date], |row| {
            Ok((
                row.get::<_, String>(0)?,  // date_bucket
                row.get::<_, u32>(1)?,     // request_count
                row.get::<_, u32>(2)?,     // success_count
                row.get::<_, u32>(3)?,     // error_count
                row.get::<_, f64>(4)?,     // avg_response_time_ms
            ))
        }).map_err(|e| StorageError::Database { message: e.to_string() })?;

        let mut stats = Vec::new();
        for stat_result in stats_iter {
            stats.push(stat_result.map_err(|e| StorageError::Database { message: e.to_string() })?);
        }

        debug!("Retrieved {} usage statistics records", stats.len());
        Ok(stats)
    }

    /// Start background tasks
    pub async fn start_background_tasks(&self) -> AppResult<()> {
        info!("Starting data persistence background tasks...");

        // TODO: Start backup tasks, cleanup tasks

        Ok(())
    }
}

#[async_trait::async_trait]
impl Service for DataPersistenceService {
    async fn health_check(&self) -> AppResult<()> {
        debug!("Performing data persistence health check");
        
        // TODO: Implement actual health check
        
        Ok(())
    }
    
    async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down data persistence service...");
        
        // TODO: Implement graceful shutdown
        
        Ok(())
    }
}
