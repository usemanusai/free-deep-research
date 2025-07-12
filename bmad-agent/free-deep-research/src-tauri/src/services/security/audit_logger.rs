use std::sync::Arc;
use std::path::PathBuf;
use tokio::sync::RwLock;
use tracing::{info, debug, error};
use rusqlite::{Connection, params};
use chrono::Utc;
use dirs;
use serde_json;

use crate::error::{AppResult, StorageError};
use crate::services::security::EncryptionManager;
use crate::models::security::{SecurityAuditEntry, SecurityEventType, SecuritySeverity, SecurityResult};
use crate::utils::file_utils::ensure_dir_exists;

// Type alias for compatibility
pub type AuditEvent = SecurityAuditEntry;

/// Audit logger for tracking security-related events
pub struct AuditLogger {
    encryption_manager: Arc<RwLock<EncryptionManager>>,
    db_path: PathBuf,
    connection: Option<Connection>,
}

impl AuditLogger {
    /// Create a new audit logger
    pub async fn new(encryption_manager: Arc<RwLock<EncryptionManager>>) -> AppResult<Self> {
        info!("Initializing audit logger...");

        // Determine database path
        let mut db_path = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("free-deep-research");

        ensure_dir_exists(&db_path)?;
        db_path.push("audit_log.db");

        let mut logger = Self {
            encryption_manager,
            db_path,
            connection: None,
        };

        // Initialize database
        logger.initialize_database().await?;

        info!("Audit logger initialized successfully");
        Ok(logger)
    }

    /// Initialize the database schema
    async fn initialize_database(&mut self) -> AppResult<()> {
        debug!("Initializing audit log database");

        let conn = Connection::open(&self.db_path)
            .map_err(|e| StorageError::Database { message: e.to_string() })?;

        // Create audit_events table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS audit_events (
                id TEXT PRIMARY KEY,
                timestamp DATETIME NOT NULL,
                event_type TEXT NOT NULL,
                severity TEXT NOT NULL,
                user_id TEXT,
                source_ip TEXT,
                resource TEXT NOT NULL,
                action TEXT NOT NULL,
                result TEXT NOT NULL,
                details TEXT,
                risk_score INTEGER NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        // Create indexes for better query performance
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_audit_timestamp ON audit_events(timestamp)",
            [],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_audit_event_type ON audit_events(event_type)",
            [],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        self.connection = Some(conn);
        debug!("Audit log database initialized");
        Ok(())
    }

    /// Log an audit event
    pub async fn log_event(&mut self, event: AuditEvent) -> AppResult<()> {
        debug!("Logging audit event: {:?}", event.event_type);

        let conn = self.connection.as_ref()
            .ok_or_else(|| StorageError::Database { message: "Database not initialized".to_string() })?;

        // Serialize details to JSON
        let details_json = serde_json::to_string(&event.details)
            .map_err(|e| StorageError::Database { message: format!("Failed to serialize details: {}", e) })?;

        // Insert into database
        conn.execute(
            "INSERT INTO audit_events (
                id, timestamp, event_type, severity, user_id, source_ip,
                resource, action, result, details, risk_score
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                event.id.to_string(),
                event.timestamp.to_rfc3339(),
                format!("{:?}", event.event_type),
                format!("{:?}", event.severity),
                event.user_id,
                event.source_ip,
                event.resource,
                event.action,
                format!("{:?}", event.result),
                details_json,
                event.risk_score,
            ],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        debug!("Audit event logged successfully");
        Ok(())
    }

    /// Get audit logs
    pub async fn get_logs(&self, limit: Option<u32>) -> AppResult<Vec<AuditEvent>> {
        debug!("Getting audit logs");

        let conn = self.connection.as_ref()
            .ok_or_else(|| StorageError::Database { message: "Database not initialized".to_string() })?;

        let limit_clause = if let Some(limit) = limit {
            format!("LIMIT {}", limit)
        } else {
            String::new()
        };

        let query = format!(
            "SELECT id, timestamp, event_type, severity, user_id, source_ip,
                    resource, action, result, details, risk_score
             FROM audit_events
             ORDER BY timestamp DESC {}",
            limit_clause
        );

        let mut stmt = conn.prepare(&query)
            .map_err(|e| StorageError::Database { message: e.to_string() })?;

        let event_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,  // id
                row.get::<_, String>(1)?,  // timestamp
                row.get::<_, String>(2)?,  // event_type
                row.get::<_, String>(3)?,  // severity
                row.get::<_, Option<String>>(4)?, // user_id
                row.get::<_, Option<String>>(5)?, // source_ip
                row.get::<_, String>(6)?,  // resource
                row.get::<_, String>(7)?,  // action
                row.get::<_, String>(8)?,  // result
                row.get::<_, String>(9)?,  // details
                row.get::<_, u8>(10)?,     // risk_score
            ))
        }).map_err(|e| StorageError::Database { message: e.to_string() })?;

        let mut events = Vec::new();

        for event_result in event_iter {
            let (id_str, timestamp_str, event_type_str, severity_str, user_id, source_ip,
                 resource, action, result_str, details_json, risk_score) =
                event_result.map_err(|e| StorageError::Database { message: e.to_string() })?;

            // Parse components (simplified)
            let id = id_str.parse()
                .map_err(|_| StorageError::Database { message: "Invalid UUID in audit log".to_string() })?;
            let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str)
                .map_err(|_| StorageError::Database { message: "Invalid timestamp in audit log".to_string() })?
                .with_timezone(&Utc);

            let details = serde_json::from_str(&details_json)
                .unwrap_or_else(|_| serde_json::Map::new());

            // Create simplified event (using defaults for enum parsing)
            events.push(AuditEvent {
                id,
                timestamp,
                event_type: SecurityEventType::SystemAccess, // Simplified
                severity: SecuritySeverity::Medium,          // Simplified
                user_id,
                source_ip,
                resource,
                action,
                result: SecurityResult::Success,             // Simplified
                details,
                risk_score,
            });
        }

        debug!("Retrieved {} audit events", events.len());
        Ok(events)
    }

    /// Shutdown the audit logger
    pub async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down audit logger...");
        Ok(())
    }
}
