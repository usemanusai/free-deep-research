//! Free Deep Research System - Security Service
//! Comprehensive security features including encryption, authentication, and audit logging

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead, NewAead}};
use rand::{Rng, thread_rng};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

use crate::error::{AppResult, AppError};

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub encryption_key: String,
    pub jwt_secret: String,
    pub session_timeout_minutes: u64,
    pub max_login_attempts: u32,
    pub lockout_duration_minutes: u64,
    pub enable_audit_logging: bool,
    pub enable_rate_limiting: bool,
    pub rate_limit_requests_per_minute: u32,
    pub enable_ip_whitelist: bool,
    pub allowed_ips: Vec<String>,
    pub password_min_length: usize,
    pub password_require_special_chars: bool,
    pub password_require_numbers: bool,
    pub password_require_uppercase: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            encryption_key: "default_key_change_in_production_32".to_string(),
            jwt_secret: "default_jwt_secret_change_in_production".to_string(),
            session_timeout_minutes: 60,
            max_login_attempts: 5,
            lockout_duration_minutes: 15,
            enable_audit_logging: true,
            enable_rate_limiting: true,
            rate_limit_requests_per_minute: 60,
            enable_ip_whitelist: false,
            allowed_ips: vec![],
            password_min_length: 8,
            password_require_special_chars: true,
            password_require_numbers: true,
            password_require_uppercase: true,
        }
    }
}

/// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub user_id: i64,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub id: i64,
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<i64>,
    pub action: String,
    pub resource: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub success: bool,
    pub error_message: Option<String>,
    pub additional_data: Option<serde_json::Value>,
}

/// Login attempt tracking
#[derive(Debug, Clone)]
struct LoginAttempt {
    attempts: u32,
    last_attempt: SystemTime,
    locked_until: Option<SystemTime>,
}

/// Rate limiting entry
#[derive(Debug, Clone)]
struct RateLimitEntry {
    requests: Vec<SystemTime>,
    blocked_until: Option<SystemTime>,
}

/// Security service
pub struct SecurityService {
    config: SecurityConfig,
    cipher: Aes256Gcm,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    login_attempts: Arc<RwLock<HashMap<String, LoginAttempt>>>,
    rate_limits: Arc<RwLock<HashMap<String, RateLimitEntry>>>,
    audit_logs: Arc<RwLock<Vec<AuditLogEntry>>>,
    next_audit_id: Arc<RwLock<i64>>,
}

impl SecurityService {
    /// Create new security service
    pub fn new(config: SecurityConfig) -> AppResult<Self> {
        // Ensure encryption key is 32 bytes
        let key_bytes = if config.encryption_key.len() >= 32 {
            config.encryption_key.as_bytes()[..32].to_vec()
        } else {
            let mut key = config.encryption_key.as_bytes().to_vec();
            key.resize(32, 0);
            key
        };
        
        let key = Key::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        
        let encoding_key = EncodingKey::from_secret(config.jwt_secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(config.jwt_secret.as_bytes());
        
        Ok(Self {
            config,
            cipher,
            encoding_key,
            decoding_key,
            login_attempts: Arc::new(RwLock::new(HashMap::new())),
            rate_limits: Arc::new(RwLock::new(HashMap::new())),
            audit_logs: Arc::new(RwLock::new(Vec::new())),
            next_audit_id: Arc::new(RwLock::new(1)),
        })
    }
    
    /// Encrypt sensitive data
    pub fn encrypt(&self, data: &str) -> AppResult<String> {
        let mut rng = thread_rng();
        let nonce_bytes: [u8; 12] = rng.gen();
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = self.cipher
            .encrypt(nonce, data.as_bytes())
            .map_err(|e| AppError::Custom(format!("Encryption failed: {}", e)))?;
        
        // Combine nonce and ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);
        
        Ok(base64::encode(result))
    }
    
    /// Decrypt sensitive data
    pub fn decrypt(&self, encrypted_data: &str) -> AppResult<String> {
        let data = base64::decode(encrypted_data)
            .map_err(|e| AppError::Custom(format!("Base64 decode failed: {}", e)))?;
        
        if data.len() < 12 {
            return Err(AppError::Custom("Invalid encrypted data".to_string()));
        }
        
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let plaintext = self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| AppError::Custom(format!("Decryption failed: {}", e)))?;
        
        String::from_utf8(plaintext)
            .map_err(|e| AppError::Custom(format!("UTF-8 decode failed: {}", e)))
    }
    
    /// Hash password with salt
    pub fn hash_password(&self, password: &str) -> AppResult<String> {
        let salt: [u8; 16] = thread_rng().gen();
        let mut hasher = Sha256::new();
        hasher.update(&salt);
        hasher.update(password.as_bytes());
        let hash = hasher.finalize();
        
        let mut result = salt.to_vec();
        result.extend_from_slice(&hash);
        
        Ok(base64::encode(result))
    }
    
    /// Verify password against hash
    pub fn verify_password(&self, password: &str, hash: &str) -> AppResult<bool> {
        let data = base64::decode(hash)
            .map_err(|e| AppError::Custom(format!("Base64 decode failed: {}", e)))?;
        
        if data.len() != 48 { // 16 bytes salt + 32 bytes hash
            return Ok(false);
        }
        
        let (salt, stored_hash) = data.split_at(16);
        
        let mut hasher = Sha256::new();
        hasher.update(salt);
        hasher.update(password.as_bytes());
        let computed_hash = hasher.finalize();
        
        Ok(computed_hash.as_slice() == stored_hash)
    }
    
    /// Validate password strength
    pub fn validate_password(&self, password: &str) -> AppResult<()> {
        if password.len() < self.config.password_min_length {
            return Err(AppError::Custom(format!(
                "Password must be at least {} characters long",
                self.config.password_min_length
            )));
        }
        
        if self.config.password_require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
            return Err(AppError::Custom("Password must contain at least one uppercase letter".to_string()));
        }
        
        if self.config.password_require_numbers && !password.chars().any(|c| c.is_numeric()) {
            return Err(AppError::Custom("Password must contain at least one number".to_string()));
        }
        
        if self.config.password_require_special_chars && !password.chars().any(|c| !c.is_alphanumeric()) {
            return Err(AppError::Custom("Password must contain at least one special character".to_string()));
        }
        
        Ok(())
    }
    
    /// Generate JWT token
    pub fn generate_token(&self, user_id: i64, roles: Vec<String>, permissions: Vec<String>) -> AppResult<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        
        let exp = now + (self.config.session_timeout_minutes * 60) as usize;
        
        let claims = Claims {
            sub: user_id.to_string(),
            exp,
            iat: now,
            user_id,
            roles,
            permissions,
        };
        
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Custom(format!("Token generation failed: {}", e)))
    }
    
    /// Validate JWT token
    pub fn validate_token(&self, token: &str) -> AppResult<Claims> {
        let validation = Validation::new(Algorithm::HS256);
        
        decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|token_data| token_data.claims)
            .map_err(|e| AppError::Custom(format!("Token validation failed: {}", e)))
    }
    
    /// Check if user is locked out due to failed login attempts
    pub async fn check_login_lockout(&self, identifier: &str) -> AppResult<bool> {
        let attempts_guard = self.login_attempts.read().await;
        
        if let Some(attempt) = attempts_guard.get(identifier) {
            if let Some(locked_until) = attempt.locked_until {
                if SystemTime::now() < locked_until {
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
    
    /// Record login attempt
    pub async fn record_login_attempt(&self, identifier: &str, success: bool) -> AppResult<()> {
        let mut attempts_guard = self.login_attempts.write().await;
        let now = SystemTime::now();
        
        let attempt = attempts_guard.entry(identifier.to_string()).or_insert(LoginAttempt {
            attempts: 0,
            last_attempt: now,
            locked_until: None,
        });
        
        if success {
            // Reset on successful login
            attempt.attempts = 0;
            attempt.locked_until = None;
        } else {
            attempt.attempts += 1;
            attempt.last_attempt = now;
            
            if attempt.attempts >= self.config.max_login_attempts {
                attempt.locked_until = Some(
                    now + Duration::from_secs(self.config.lockout_duration_minutes * 60)
                );
            }
        }
        
        Ok(())
    }
    
    /// Check rate limiting
    pub async fn check_rate_limit(&self, identifier: &str) -> AppResult<bool> {
        if !self.config.enable_rate_limiting {
            return Ok(true);
        }
        
        let mut rate_limits_guard = self.rate_limits.write().await;
        let now = SystemTime::now();
        
        let entry = rate_limits_guard.entry(identifier.to_string()).or_insert(RateLimitEntry {
            requests: Vec::new(),
            blocked_until: None,
        });
        
        // Check if currently blocked
        if let Some(blocked_until) = entry.blocked_until {
            if now < blocked_until {
                return Ok(false);
            } else {
                entry.blocked_until = None;
            }
        }
        
        // Clean old requests (older than 1 minute)
        let cutoff = now - Duration::from_secs(60);
        entry.requests.retain(|&request_time| request_time > cutoff);
        
        // Check if rate limit exceeded
        if entry.requests.len() >= self.config.rate_limit_requests_per_minute as usize {
            entry.blocked_until = Some(now + Duration::from_secs(60));
            return Ok(false);
        }
        
        // Record this request
        entry.requests.push(now);
        
        Ok(true)
    }
    
    /// Check IP whitelist
    pub fn check_ip_whitelist(&self, ip: &str) -> bool {
        if !self.config.enable_ip_whitelist {
            return true;
        }
        
        self.config.allowed_ips.contains(&ip.to_string())
    }
    
    /// Log audit event
    pub async fn log_audit_event(
        &self,
        user_id: Option<i64>,
        action: &str,
        resource: &str,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
        success: bool,
        error_message: Option<&str>,
        additional_data: Option<serde_json::Value>,
    ) -> AppResult<()> {
        if !self.config.enable_audit_logging {
            return Ok(());
        }
        
        let mut next_id_guard = self.next_audit_id.write().await;
        let id = *next_id_guard;
        *next_id_guard += 1;
        drop(next_id_guard);
        
        let entry = AuditLogEntry {
            id,
            timestamp: Utc::now(),
            user_id,
            action: action.to_string(),
            resource: resource.to_string(),
            ip_address: ip_address.map(|s| s.to_string()),
            user_agent: user_agent.map(|s| s.to_string()),
            success,
            error_message: error_message.map(|s| s.to_string()),
            additional_data,
        };
        
        let mut audit_logs_guard = self.audit_logs.write().await;
        audit_logs_guard.push(entry);
        
        // Keep only last 10000 entries
        if audit_logs_guard.len() > 10000 {
            audit_logs_guard.drain(0..1000);
        }
        
        Ok(())
    }
    
    /// Get audit logs
    pub async fn get_audit_logs(&self, limit: Option<usize>) -> AppResult<Vec<AuditLogEntry>> {
        let audit_logs_guard = self.audit_logs.read().await;
        
        let logs = if let Some(limit) = limit {
            audit_logs_guard.iter()
                .rev()
                .take(limit)
                .cloned()
                .collect()
        } else {
            audit_logs_guard.clone()
        };
        
        Ok(logs)
    }
    
    /// Get security statistics
    pub async fn get_security_stats(&self) -> AppResult<serde_json::Value> {
        let attempts_guard = self.login_attempts.read().await;
        let rate_limits_guard = self.rate_limits.read().await;
        let audit_logs_guard = self.audit_logs.read().await;
        
        let total_login_attempts = attempts_guard.len();
        let locked_accounts = attempts_guard.values()
            .filter(|attempt| attempt.locked_until.is_some())
            .count();
        
        let rate_limited_ips = rate_limits_guard.values()
            .filter(|entry| entry.blocked_until.is_some())
            .count();
        
        let total_audit_events = audit_logs_guard.len();
        let failed_events = audit_logs_guard.iter()
            .filter(|entry| !entry.success)
            .count();
        
        Ok(serde_json::json!({
            "total_login_attempts": total_login_attempts,
            "locked_accounts": locked_accounts,
            "rate_limited_ips": rate_limited_ips,
            "total_audit_events": total_audit_events,
            "failed_events": failed_events,
            "success_rate": if total_audit_events > 0 {
                ((total_audit_events - failed_events) as f64 / total_audit_events as f64) * 100.0
            } else {
                100.0
            }
        }))
    }
    
    /// Clean up old security data
    pub async fn cleanup_old_data(&self) -> AppResult<()> {
        let now = SystemTime::now();
        let cutoff = now - Duration::from_secs(24 * 60 * 60); // 24 hours
        
        // Clean up old login attempts
        {
            let mut attempts_guard = self.login_attempts.write().await;
            attempts_guard.retain(|_, attempt| {
                attempt.last_attempt > cutoff || attempt.locked_until.is_some()
            });
        }
        
        // Clean up old rate limit entries
        {
            let mut rate_limits_guard = self.rate_limits.write().await;
            rate_limits_guard.retain(|_, entry| {
                !entry.requests.is_empty() || entry.blocked_until.is_some()
            });
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_encryption_decryption() {
        let config = SecurityConfig::default();
        let security_service = SecurityService::new(config).unwrap();
        
        let original = "sensitive data";
        let encrypted = security_service.encrypt(original).unwrap();
        let decrypted = security_service.decrypt(&encrypted).unwrap();
        
        assert_eq!(original, decrypted);
    }
    
    #[tokio::test]
    async fn test_password_hashing() {
        let config = SecurityConfig::default();
        let security_service = SecurityService::new(config).unwrap();
        
        let password = "test_password_123!";
        let hash = security_service.hash_password(password).unwrap();
        
        assert!(security_service.verify_password(password, &hash).unwrap());
        assert!(!security_service.verify_password("wrong_password", &hash).unwrap());
    }
    
    #[tokio::test]
    async fn test_jwt_tokens() {
        let config = SecurityConfig::default();
        let security_service = SecurityService::new(config).unwrap();
        
        let user_id = 123;
        let roles = vec!["user".to_string()];
        let permissions = vec!["read".to_string()];
        
        let token = security_service.generate_token(user_id, roles.clone(), permissions.clone()).unwrap();
        let claims = security_service.validate_token(&token).unwrap();
        
        assert_eq!(claims.user_id, user_id);
        assert_eq!(claims.roles, roles);
        assert_eq!(claims.permissions, permissions);
    }
    
    #[tokio::test]
    async fn test_rate_limiting() {
        let mut config = SecurityConfig::default();
        config.rate_limit_requests_per_minute = 2;
        let security_service = SecurityService::new(config).unwrap();
        
        let identifier = "test_user";
        
        // First two requests should pass
        assert!(security_service.check_rate_limit(identifier).await.unwrap());
        assert!(security_service.check_rate_limit(identifier).await.unwrap());
        
        // Third request should be blocked
        assert!(!security_service.check_rate_limit(identifier).await.unwrap());
    }
}
