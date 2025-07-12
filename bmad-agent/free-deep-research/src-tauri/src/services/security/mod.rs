use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error, debug};
use ring::{aead, pbkdf2, rand};
use ring::rand::SecureRandom;
use std::num::NonZeroU32;

use crate::error::{AppResult, SecurityError};
use crate::services::Service;

pub mod encryption_manager;
pub mod authentication;
pub mod audit_logger;
pub mod key_vault;

use encryption_manager::EncryptionManager;
use authentication::AuthenticationManager;
use audit_logger::AuditLogger;
use key_vault::KeyVault;

/// Security service that manages encryption, authentication, and audit logging
pub struct SecurityService {
    encryption_manager: Arc<RwLock<EncryptionManager>>,
    authentication_manager: Arc<RwLock<AuthenticationManager>>,
    audit_logger: Arc<RwLock<AuditLogger>>,
    key_vault: Arc<RwLock<KeyVault>>,
}

impl SecurityService {
    /// Create a new security service
    pub async fn new() -> AppResult<Self> {
        info!("Initializing security service...");
        
        // Initialize encryption manager
        let encryption_manager = EncryptionManager::new().await?;
        let encryption_manager = Arc::new(RwLock::new(encryption_manager));
        
        // Initialize key vault
        let key_vault = KeyVault::new(encryption_manager.clone()).await?;
        let key_vault = Arc::new(RwLock::new(key_vault));
        
        // Initialize authentication manager
        let authentication_manager = AuthenticationManager::new(key_vault.clone()).await?;
        let authentication_manager = Arc::new(RwLock::new(authentication_manager));
        
        // Initialize audit logger
        let audit_logger = AuditLogger::new(encryption_manager.clone()).await?;
        let audit_logger = Arc::new(RwLock::new(audit_logger));
        
        let service = Self {
            encryption_manager,
            authentication_manager,
            audit_logger,
            key_vault,
        };
        
        info!("Security service initialized successfully");
        Ok(service)
    }
    
    /// Encrypt sensitive data
    pub async fn encrypt(&self, data: &[u8]) -> AppResult<Vec<u8>> {
        let encryption_manager = self.encryption_manager.read().await;
        encryption_manager.encrypt(data).await
    }
    
    /// Decrypt sensitive data
    pub async fn decrypt(&self, encrypted_data: &[u8]) -> AppResult<Vec<u8>> {
        let encryption_manager = self.encryption_manager.read().await;
        encryption_manager.decrypt(encrypted_data).await
    }
    
    /// Encrypt a string
    pub async fn encrypt_string(&self, data: &str) -> AppResult<String> {
        let encrypted = self.encrypt(data.as_bytes()).await?;
        Ok(base64::encode(encrypted))
    }
    
    /// Decrypt a string
    pub async fn decrypt_string(&self, encrypted_data: &str) -> AppResult<String> {
        let encrypted_bytes = base64::decode(encrypted_data)
            .map_err(|e| SecurityError::decryption_failed(format!("Invalid base64: {}", e)))?;
        let decrypted = self.decrypt(&encrypted_bytes).await?;
        String::from_utf8(decrypted)
            .map_err(|e| SecurityError::decryption_failed(format!("Invalid UTF-8: {}", e)).into())
    }
    
    /// Authenticate with master password
    pub async fn authenticate(&self, password: &str) -> AppResult<bool> {
        let auth_manager = self.authentication_manager.read().await;
        auth_manager.authenticate(password).await
    }
    
    /// Set master password
    pub async fn set_master_password(&self, password: &str) -> AppResult<()> {
        let mut auth_manager = self.authentication_manager.write().await;
        auth_manager.set_master_password(password).await
    }
    
    /// Check if master password is set
    pub async fn has_master_password(&self) -> AppResult<bool> {
        let auth_manager = self.authentication_manager.read().await;
        auth_manager.has_master_password().await
    }
    
    /// Log an audit event
    pub async fn log_audit_event(&self, event: audit_logger::AuditEvent) -> AppResult<()> {
        let mut audit_logger = self.audit_logger.write().await;
        audit_logger.log_event(event).await
    }

    /// Get audit logs
    pub async fn get_audit_logs(&self, limit: Option<u32>) -> AppResult<Vec<audit_logger::AuditEvent>> {
        let audit_logger = self.audit_logger.read().await;
        audit_logger.get_logs(limit).await
    }
    
    /// Store a secret in the key vault
    pub async fn store_secret(&self, key: &str, value: &str) -> AppResult<()> {
        let mut key_vault = self.key_vault.write().await;
        key_vault.store_secret(key, value).await
    }
    
    /// Retrieve a secret from the key vault
    pub async fn get_secret(&self, key: &str) -> AppResult<Option<String>> {
        let key_vault = self.key_vault.read().await;
        key_vault.get_secret(key).await
    }
    
    /// Delete a secret from the key vault
    pub async fn delete_secret(&self, key: &str) -> AppResult<()> {
        let mut key_vault = self.key_vault.write().await;
        key_vault.delete_secret(key).await
    }
    
    /// Generate a secure random string
    pub async fn generate_random_string(&self, length: usize) -> AppResult<String> {
        let encryption_manager = self.encryption_manager.read().await;
        encryption_manager.generate_random_string(length).await
    }
    
    /// Generate a secure random bytes
    pub async fn generate_random_bytes(&self, length: usize) -> AppResult<Vec<u8>> {
        let encryption_manager = self.encryption_manager.read().await;
        encryption_manager.generate_random_bytes(length).await
    }
}

#[async_trait::async_trait]
impl Service for SecurityService {
    async fn health_check(&self) -> AppResult<()> {
        debug!("Performing security service health check");
        
        // Test encryption/decryption
        let test_data = "health_check_test_data";
        let encrypted = self.encrypt_string(test_data).await?;
        let decrypted = self.decrypt_string(&encrypted).await?;
        
        if decrypted != test_data {
            return Err(SecurityError::encryption_failed("Health check encryption test failed").into());
        }
        
        // Check if components are responsive
        {
            let _encryption_manager = self.encryption_manager.read().await;
            let _auth_manager = self.authentication_manager.read().await;
            let _audit_logger = self.audit_logger.read().await;
            let _key_vault = self.key_vault.read().await;
        }
        
        debug!("Security service health check passed");
        Ok(())
    }
    
    async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down security service...");
        
        // Shutdown audit logger (ensure all logs are written)
        {
            let audit_logger = self.audit_logger.write().await;
            audit_logger.shutdown().await?;
        }
        
        // Shutdown key vault
        {
            let key_vault = self.key_vault.write().await;
            key_vault.shutdown().await?;
        }
        
        // Shutdown authentication manager
        {
            let auth_manager = self.authentication_manager.write().await;
            auth_manager.shutdown().await?;
        }
        
        // Shutdown encryption manager
        {
            let encryption_manager = self.encryption_manager.write().await;
            encryption_manager.shutdown().await?;
        }
        
        info!("Security service shutdown complete");
        Ok(())
    }
}

/// Audit event for logging security-related actions
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuditEvent {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event_type: AuditEventType,
    pub user_id: Option<String>,
    pub resource: String,
    pub action: String,
    pub result: AuditResult,
    pub details: std::collections::HashMap<String, serde_json::Value>,
}

impl AuditEvent {
    /// Create a new audit event
    pub fn new(
        event_type: AuditEventType,
        resource: String,
        action: String,
        result: AuditResult,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            event_type,
            user_id: None,
            resource,
            action,
            result,
            details: std::collections::HashMap::new(),
        }
    }
    
    /// Add details to the audit event
    pub fn with_details(mut self, key: String, value: serde_json::Value) -> Self {
        self.details.insert(key, value);
        self
    }
    
    /// Set user ID for the audit event
    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }
}

/// Types of audit events
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AuditEventType {
    Authentication,
    Authorization,
    DataAccess,
    DataModification,
    SystemAccess,
    ConfigurationChange,
    SecurityEvent,
}

/// Result of an audited action
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AuditResult {
    Success,
    Failure,
    Warning,
}
