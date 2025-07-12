use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Security configuration model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfiguration {
    pub id: Uuid,
    pub encryption_algorithm: EncryptionAlgorithm,
    pub key_derivation_iterations: u32,
    pub session_timeout_minutes: u32,
    pub max_login_attempts: u32,
    pub audit_log_retention_days: u32,
    pub require_master_password: bool,
    pub auto_lock_enabled: bool,
    pub auto_lock_timeout_minutes: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for SecurityConfiguration {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            encryption_algorithm: EncryptionAlgorithm::Aes256Gcm,
            key_derivation_iterations: 100_000,
            session_timeout_minutes: 60,
            max_login_attempts: 5,
            audit_log_retention_days: 365,
            require_master_password: true,
            auto_lock_enabled: true,
            auto_lock_timeout_minutes: 15,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Supported encryption algorithms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EncryptionAlgorithm {
    Aes256Gcm,
    ChaCha20Poly1305,
}

impl EncryptionAlgorithm {
    /// Get the display name for the encryption algorithm
    pub fn display_name(&self) -> &'static str {
        match self {
            EncryptionAlgorithm::Aes256Gcm => "AES-256-GCM",
            EncryptionAlgorithm::ChaCha20Poly1305 => "ChaCha20-Poly1305",
        }
    }
    
    /// Get the key size in bytes
    pub fn key_size(&self) -> usize {
        match self {
            EncryptionAlgorithm::Aes256Gcm => 32,        // 256 bits
            EncryptionAlgorithm::ChaCha20Poly1305 => 32, // 256 bits
        }
    }
    
    /// Get the nonce size in bytes
    pub fn nonce_size(&self) -> usize {
        match self {
            EncryptionAlgorithm::Aes256Gcm => 12,        // 96 bits
            EncryptionAlgorithm::ChaCha20Poly1305 => 12, // 96 bits
        }
    }
}

/// Authentication session model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationSession {
    pub id: Uuid,
    pub user_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
    pub session_data: HashMap<String, serde_json::Value>,
}

impl AuthenticationSession {
    /// Create a new authentication session
    pub fn new(timeout_minutes: u32) -> Self {
        let now = Utc::now();
        let expires_at = now + chrono::Duration::minutes(timeout_minutes as i64);
        
        Self {
            id: Uuid::new_v4(),
            user_id: None,
            created_at: now,
            last_activity: now,
            expires_at,
            is_active: true,
            session_data: HashMap::new(),
        }
    }
    
    /// Check if the session is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
    
    /// Update the last activity timestamp
    pub fn update_activity(&mut self) {
        self.last_activity = Utc::now();
    }
    
    /// Extend the session expiration
    pub fn extend_session(&mut self, timeout_minutes: u32) {
        self.expires_at = Utc::now() + chrono::Duration::minutes(timeout_minutes as i64);
        self.update_activity();
    }
    
    /// Invalidate the session
    pub fn invalidate(&mut self) {
        self.is_active = false;
    }
}

/// Encryption key metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionKeyMetadata {
    pub id: Uuid,
    pub algorithm: EncryptionAlgorithm,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
    pub usage_count: u64,
    pub is_active: bool,
    pub key_derivation_salt: Vec<u8>,
    pub key_derivation_iterations: u32,
}

impl EncryptionKeyMetadata {
    /// Create new encryption key metadata
    pub fn new(algorithm: EncryptionAlgorithm, salt: Vec<u8>, iterations: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            algorithm,
            created_at: Utc::now(),
            last_used: None,
            usage_count: 0,
            is_active: true,
            key_derivation_salt: salt,
            key_derivation_iterations: iterations,
        }
    }
    
    /// Update usage statistics
    pub fn update_usage(&mut self) {
        self.last_used = Some(Utc::now());
        self.usage_count += 1;
    }
    
    /// Deactivate the key
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}

/// Security audit trail entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: SecurityEventType,
    pub severity: SecuritySeverity,
    pub user_id: Option<String>,
    pub source_ip: Option<String>,
    pub resource: String,
    pub action: String,
    pub result: SecurityResult,
    pub details: HashMap<String, serde_json::Value>,
    pub risk_score: u8, // 0-100
}

/// Types of security events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SecurityEventType {
    Authentication,
    Authorization,
    DataAccess,
    DataModification,
    SystemAccess,
    ConfigurationChange,
    SecurityViolation,
    EncryptionOperation,
    KeyManagement,
}

/// Security event severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Security event results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SecurityResult {
    Success,
    Failure,
    Warning,
    Blocked,
}

impl SecurityAuditEntry {
    /// Create a new security audit entry
    pub fn new(
        event_type: SecurityEventType,
        severity: SecuritySeverity,
        resource: String,
        action: String,
        result: SecurityResult,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type,
            severity,
            user_id: None,
            source_ip: None,
            resource,
            action,
            result,
            details: HashMap::new(),
            risk_score: 0,
        }
    }
    
    /// Add details to the audit entry
    pub fn with_details(mut self, key: String, value: serde_json::Value) -> Self {
        self.details.insert(key, value);
        self
    }
    
    /// Set the risk score
    pub fn with_risk_score(mut self, score: u8) -> Self {
        self.risk_score = score.min(100);
        self
    }
    
    /// Set the user ID
    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }
    
    /// Set the source IP
    pub fn with_source_ip(mut self, ip: String) -> Self {
        self.source_ip = Some(ip);
        self
    }
}
