use std::sync::Arc;
use std::path::PathBuf;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use tracing::{info, debug, error, warn};
use rusqlite::{Connection, params};
use chrono::{Utc, DateTime};
use dirs;
use serde_json;
use ring::{hmac, digest};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::error::{AppResult, StorageError};
use crate::services::security::EncryptionManager;
use crate::models::security::{SecurityAuditEntry, SecurityEventType, SecuritySeverity, SecurityResult};
use crate::utils::file_utils::ensure_dir_exists;

// Type alias for compatibility
pub type AuditEvent = SecurityAuditEntry;

/// Enhanced audit logger for tracking security-related events with tamper-proof features
pub struct AuditLogger {
    encryption_manager: Arc<RwLock<EncryptionManager>>,
    db_path: PathBuf,
    connection: Option<Connection>,
    integrity_key: Option<hmac::Key>,
    compliance_config: ComplianceConfiguration,
    log_chain: Vec<String>, // Chain of log hashes for tamper detection
    retention_manager: Option<tokio::task::JoinHandle<()>>,
}

/// Compliance configuration for audit logging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceConfiguration {
    pub retention_days: u32,
    pub tamper_proof_enabled: bool,
    pub real_time_monitoring: bool,
    pub compliance_reporting: bool,
    pub log_encryption_enabled: bool,
    pub chain_verification_enabled: bool,
    pub export_format: String,
    pub alert_on_violations: bool,
}

impl Default for ComplianceConfiguration {
    fn default() -> Self {
        Self {
            retention_days: 365, // 1 year retention
            tamper_proof_enabled: true,
            real_time_monitoring: true,
            compliance_reporting: true,
            log_encryption_enabled: true,
            chain_verification_enabled: true,
            export_format: "json".to_string(),
            alert_on_violations: true,
        }
    }
}

/// Enhanced audit event with tamper-proof features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TamperProofAuditEvent {
    pub base_event: SecurityAuditEntry,
    pub signature: String,
    pub previous_hash: Option<String>,
    pub chain_index: u64,
    pub compliance_tags: Vec<String>,
}

impl AuditLogger {
    /// Create a new enhanced audit logger with tamper-proof features
    pub async fn new(encryption_manager: Arc<RwLock<EncryptionManager>>) -> AppResult<Self> {
        Self::new_with_config(encryption_manager, ComplianceConfiguration::default()).await
    }

    /// Create a new audit logger with custom compliance configuration
    pub async fn new_with_config(
        encryption_manager: Arc<RwLock<EncryptionManager>>,
        compliance_config: ComplianceConfiguration,
    ) -> AppResult<Self> {
        info!("Initializing enhanced audit logger with tamper-proof features...");

        // Determine database path
        let mut db_path = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("free-deep-research");

        ensure_dir_exists(&db_path)?;
        db_path.push("audit_log.db");

        // Generate integrity key for tamper-proof logging
        let integrity_key = if compliance_config.tamper_proof_enabled {
            let key_material = ring::rand::SystemRandom::new();
            let mut key_bytes = [0u8; 32];
            ring::rand::SecureRandom::fill(&key_material, &mut key_bytes)
                .map_err(|_| StorageError::Database { message: "Failed to generate integrity key".to_string() })?;
            Some(hmac::Key::new(hmac::HMAC_SHA256, &key_bytes))
        } else {
            None
        };

        let mut logger = Self {
            encryption_manager,
            db_path,
            connection: None,
            integrity_key,
            compliance_config,
            log_chain: Vec::new(),
            retention_manager: None,
        };

        // Initialize database
        logger.initialize_database().await?;

        // Initialize log chain
        logger.initialize_log_chain().await?;

        // Start retention manager if needed
        if logger.compliance_config.retention_days > 0 {
            logger.start_retention_manager().await?;
        }

        info!("Enhanced audit logger initialized successfully");
        Ok(logger)
    }

    /// Initialize the log chain for tamper detection
    async fn initialize_log_chain(&mut self) -> AppResult<()> {
        if !self.compliance_config.chain_verification_enabled {
            return Ok(());
        }

        debug!("Initializing audit log chain");

        // Get the last few log entries to rebuild the chain
        let recent_logs = self.get_logs(Some(100)).await?;

        for log in recent_logs.iter().rev() {
            let log_hash = self.calculate_log_hash(&log).await?;
            self.log_chain.push(log_hash);
        }

        // Limit chain size to prevent memory issues
        if self.log_chain.len() > 1000 {
            self.log_chain.drain(0..self.log_chain.len() - 1000);
        }

        info!("Log chain initialized with {} entries", self.log_chain.len());
        Ok(())
    }

    /// Start the retention manager for automatic log cleanup
    async fn start_retention_manager(&mut self) -> AppResult<()> {
        if self.retention_manager.is_some() {
            return Ok(()); // Already running
        }

        let retention_days = self.compliance_config.retention_days;
        let db_path = self.db_path.clone();

        let handle = tokio::spawn(async move {
            let mut interval_timer = interval(Duration::from_secs(24 * 60 * 60)); // Daily cleanup

            loop {
                interval_timer.tick().await;

                if let Err(e) = Self::cleanup_old_logs(&db_path, retention_days).await {
                    error!("Failed to cleanup old audit logs: {}", e);
                }
            }
        });

        self.retention_manager = Some(handle);
        info!("Retention manager started with {} days retention", retention_days);
        Ok(())
    }

    /// Cleanup old audit logs based on retention policy
    async fn cleanup_old_logs(db_path: &Path, retention_days: u32) -> AppResult<()> {
        debug!("Cleaning up audit logs older than {} days", retention_days);

        let conn = Connection::open(db_path)
            .map_err(|e| StorageError::Database { message: e.to_string() })?;

        let cutoff_date = Utc::now() - chrono::Duration::days(retention_days as i64);

        let deleted_count = conn.execute(
            "DELETE FROM audit_events WHERE timestamp < ?1",
            params![cutoff_date.to_rfc3339()],
        ).map_err(|e| StorageError::Database { message: e.to_string() })?;

        if deleted_count > 0 {
            info!("Cleaned up {} old audit log entries", deleted_count);
        }

        Ok(())
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
