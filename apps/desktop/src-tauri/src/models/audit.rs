use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

/// Audit event severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditSeverity {
    /// Informational events (normal operations)
    Info,
    /// Warning events (potential issues)
    Warning,
    /// Error events (operation failures)
    Error,
    /// Critical events (security issues, system failures)
    Critical,
}

impl AuditSeverity {
    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditSeverity::Info => "Info",
            AuditSeverity::Warning => "Warning", 
            AuditSeverity::Error => "Error",
            AuditSeverity::Critical => "Critical",
        }
    }
}

/// Audit event model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Unique identifier for the audit event
    pub id: Uuid,
    /// Type of event (e.g., "api_key_added", "research_started")
    pub event_type: String,
    /// Human-readable description of the event
    pub description: String,
    /// Optional resource identifier related to the event
    pub resource_id: Option<String>,
    /// Optional user identifier who triggered the event
    pub user_id: Option<String>,
    /// Timestamp when the event occurred
    pub timestamp: DateTime<Utc>,
    /// Severity level of the event
    pub severity: AuditSeverity,
    /// Additional metadata as key-value pairs
    pub metadata: HashMap<String, String>,
}

impl AuditEvent {
    /// Create a new audit event
    pub fn new(
        event_type: String,
        description: String,
        severity: AuditSeverity,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            description,
            resource_id: None,
            user_id: None,
            timestamp: Utc::now(),
            severity,
            metadata: HashMap::new(),
        }
    }

    /// Create an info-level audit event
    pub fn info(event_type: String, description: String) -> Self {
        Self::new(event_type, description, AuditSeverity::Info)
    }

    /// Create a warning-level audit event
    pub fn warning(event_type: String, description: String) -> Self {
        Self::new(event_type, description, AuditSeverity::Warning)
    }

    /// Create an error-level audit event
    pub fn error(event_type: String, description: String) -> Self {
        Self::new(event_type, description, AuditSeverity::Error)
    }

    /// Create a critical-level audit event
    pub fn critical(event_type: String, description: String) -> Self {
        Self::new(event_type, description, AuditSeverity::Critical)
    }

    /// Set resource ID
    pub fn with_resource_id(mut self, resource_id: String) -> Self {
        self.resource_id = Some(resource_id);
        self
    }

    /// Set user ID
    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Add multiple metadata entries
    pub fn with_metadata_map(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata.extend(metadata);
        self
    }
}

/// Audit query parameters for filtering audit events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditQuery {
    /// Filter by event type
    pub event_type: Option<String>,
    /// Filter by resource ID
    pub resource_id: Option<String>,
    /// Filter by user ID
    pub user_id: Option<String>,
    /// Filter by severity level
    pub severity: Option<AuditSeverity>,
    /// Filter by date range - start
    pub start_date: Option<DateTime<Utc>>,
    /// Filter by date range - end
    pub end_date: Option<DateTime<Utc>>,
    /// Maximum number of results to return
    pub limit: Option<u32>,
    /// Offset for pagination
    pub offset: Option<u32>,
}

impl Default for AuditQuery {
    fn default() -> Self {
        Self {
            event_type: None,
            resource_id: None,
            user_id: None,
            severity: None,
            start_date: None,
            end_date: None,
            limit: Some(100),
            offset: Some(0),
        }
    }
}

/// Audit statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStatistics {
    /// Total number of audit events
    pub total_events: u64,
    /// Events by severity level
    pub events_by_severity: HashMap<String, u64>,
    /// Events by type
    pub events_by_type: HashMap<String, u64>,
    /// Events in the last 24 hours
    pub events_last_24h: u64,
    /// Events in the last 7 days
    pub events_last_7d: u64,
    /// Events in the last 30 days
    pub events_last_30d: u64,
}

impl Default for AuditStatistics {
    fn default() -> Self {
        Self {
            total_events: 0,
            events_by_severity: HashMap::new(),
            events_by_type: HashMap::new(),
            events_last_24h: 0,
            events_last_7d: 0,
            events_last_30d: 0,
        }
    }
}

/// Audit event export format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditExportFormat {
    /// JSON format
    Json,
    /// CSV format
    Csv,
    /// XML format
    Xml,
}

/// Audit event export request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditExportRequest {
    /// Query parameters for filtering events to export
    pub query: AuditQuery,
    /// Export format
    pub format: AuditExportFormat,
    /// Include metadata in export
    pub include_metadata: bool,
}

/// Common audit event types as constants
pub mod event_types {
    pub const API_KEY_ADDED: &str = "api_key_added";
    pub const API_KEY_UPDATED: &str = "api_key_updated";
    pub const API_KEY_DELETED: &str = "api_key_deleted";
    pub const API_KEY_TESTED: &str = "api_key_tested";
    pub const RESEARCH_STARTED: &str = "research_started";
    pub const RESEARCH_COMPLETED: &str = "research_completed";
    pub const RESEARCH_FAILED: &str = "research_failed";
    pub const USER_LOGIN: &str = "user_login";
    pub const USER_LOGOUT: &str = "user_logout";
    pub const SYSTEM_STARTUP: &str = "system_startup";
    pub const SYSTEM_SHUTDOWN: &str = "system_shutdown";
    pub const CONFIGURATION_CHANGED: &str = "configuration_changed";
    pub const SECURITY_VIOLATION: &str = "security_violation";
    pub const RATE_LIMIT_EXCEEDED: &str = "rate_limit_exceeded";
    pub const SERVICE_ERROR: &str = "service_error";
}
