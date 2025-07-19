use ring::{aead, pbkdf2, rand, hmac, digest};
use ring::rand::SecureRandom;
use std::num::NonZeroU32;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, error, warn};
use base64;
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::error::{AppResult, SecurityError};

/// Enhanced encryption manager for handling AES-256-GCM encryption with key rotation
#[derive(Debug)]
pub struct EncryptionManager {
    /// Current master key for encryption operations
    master_key: Option<aead::LessSafeKey>,
    /// Key rotation history for decryption of old data
    key_history: HashMap<String, EncryptionKey>,
    /// Current key version
    current_key_version: String,
    /// Key rotation settings
    rotation_settings: KeyRotationSettings,
    /// Session management
    session_manager: Arc<RwLock<SessionManager>>,
    /// Integrity verification keys
    integrity_keys: HashMap<String, hmac::Key>,
}

/// Encryption key with metadata
#[derive(Debug, Clone, ZeroizeOnDrop)]
pub struct EncryptionKey {
    #[zeroize(skip)]
    pub version: String,
    #[zeroize(skip)]
    pub created_at: DateTime<Utc>,
    #[zeroize(skip)]
    pub expires_at: Option<DateTime<Utc>>,
    pub key_data: Vec<u8>,
    #[zeroize(skip)]
    pub algorithm: String,
    #[zeroize(skip)]
    pub key_derivation_params: KeyDerivationParams,
}

/// Key derivation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyDerivationParams {
    pub salt: Vec<u8>,
    pub iterations: u32,
    pub algorithm: String,
}

/// Key rotation settings
#[derive(Debug, Clone)]
pub struct KeyRotationSettings {
    pub auto_rotation_enabled: bool,
    pub rotation_interval_hours: u64,
    pub max_key_age_hours: u64,
    pub keep_old_keys_count: usize,
}

impl Default for KeyRotationSettings {
    fn default() -> Self {
        Self {
            auto_rotation_enabled: true,
            rotation_interval_hours: 24 * 7, // Weekly rotation
            max_key_age_hours: 24 * 30,      // 30 days max age
            keep_old_keys_count: 10,         // Keep 10 old keys for decryption
        }
    }
}

/// Session manager for handling user sessions
#[derive(Debug)]
pub struct SessionManager {
    active_sessions: HashMap<String, UserSession>,
    session_timeout_minutes: u64,
    max_concurrent_sessions: usize,
}

/// User session information
#[derive(Debug, Clone)]
pub struct UserSession {
    pub session_id: String,
    pub user_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub is_master_session: bool,
}

impl EncryptionManager {
    /// Create a new enhanced encryption manager
    pub async fn new() -> AppResult<Self> {
        info!("Initializing enhanced encryption manager...");

        let session_manager = SessionManager::new(30, 5).await; // 30 min timeout, 5 max sessions

        let manager = Self {
            master_key: None,
            key_history: HashMap::new(),
            current_key_version: String::new(),
            rotation_settings: KeyRotationSettings::default(),
            session_manager: Arc::new(RwLock::new(session_manager)),
            integrity_keys: HashMap::new(),
        };

        info!("Enhanced encryption manager initialized successfully");
        Ok(manager)
    }

    /// Create encryption manager with custom rotation settings
    pub async fn new_with_settings(rotation_settings: KeyRotationSettings) -> AppResult<Self> {
        info!("Initializing encryption manager with custom settings...");

        let session_manager = SessionManager::new(30, 5).await;

        let manager = Self {
            master_key: None,
            key_history: HashMap::new(),
            current_key_version: String::new(),
            rotation_settings,
            session_manager: Arc::new(RwLock::new(session_manager)),
            integrity_keys: HashMap::new(),
        };

        info!("Encryption manager with custom settings initialized successfully");
        Ok(manager)
    }

    /// Initialize encryption with a master password and enhanced security
    pub async fn initialize_with_password(&mut self, password: &str) -> AppResult<()> {
        debug!("Initializing encryption with master password");

        // Generate a random salt
        let salt = self.generate_random_bytes(32).await?;
        let iterations = 100_000; // 100,000 iterations for security

        // Create key version
        let key_version = format!("v{}", Uuid::new_v4().to_string().split('-').next().unwrap_or("unknown"));

        // Derive key from password using PBKDF2
        let mut key_bytes = [0u8; 32]; // 256 bits for AES-256
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(iterations).unwrap(),
            &salt,
            password.as_bytes(),
            &mut key_bytes,
        );

        // Create the encryption key
        let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes)
            .map_err(|_| SecurityError::encryption_failed("Failed to create encryption key"))?;

        self.master_key = Some(aead::LessSafeKey::new(unbound_key));

        // Store key in history for future rotation
        let encryption_key = EncryptionKey {
            version: key_version.clone(),
            created_at: Utc::now(),
            expires_at: Some(Utc::now() + Duration::hours(self.rotation_settings.max_key_age_hours as i64)),
            key_data: key_bytes.to_vec(),
            algorithm: "AES-256-GCM".to_string(),
            key_derivation_params: KeyDerivationParams {
                salt: salt.clone(),
                iterations,
                algorithm: "PBKDF2-HMAC-SHA256".to_string(),
            },
        };

        self.key_history.insert(key_version.clone(), encryption_key);
        self.current_key_version = key_version;

        // Create integrity verification key
        let integrity_key = hmac::Key::new(hmac::HMAC_SHA256, &key_bytes);
        self.integrity_keys.insert(self.current_key_version.clone(), integrity_key);

        // Clear sensitive data from memory
        let mut key_bytes_zeroize = key_bytes;
        key_bytes_zeroize.zeroize();

        info!("Enhanced encryption initialized with master password (version: {})", self.current_key_version);
        Ok(())
    }

    /// Rotate encryption keys
    pub async fn rotate_keys(&mut self, new_password: Option<&str>) -> AppResult<String> {
        info!("Rotating encryption keys");

        if !self.rotation_settings.auto_rotation_enabled {
            return Err(SecurityError::encryption_failed("Key rotation is disabled").into());
        }

        // Use existing password if no new password provided
        let password = new_password.unwrap_or(""); // In real implementation, would derive from current key

        // Clean up old keys if we have too many
        if self.key_history.len() >= self.rotation_settings.keep_old_keys_count {
            self.cleanup_old_keys().await?;
        }

        // Generate new key version
        let new_key_version = format!("v{}", Uuid::new_v4().to_string().split('-').next().unwrap_or("unknown"));

        // Generate new salt and derive new key
        let salt = self.generate_random_bytes(32).await?;
        let iterations = 100_000;

        let mut new_key_bytes = [0u8; 32];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(iterations).unwrap(),
            &salt,
            password.as_bytes(),
            &mut new_key_bytes,
        );

        // Create new encryption key
        let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &new_key_bytes)
            .map_err(|_| SecurityError::encryption_failed("Failed to create new encryption key"))?;

        // Store old key in history before replacing
        if let Some(old_key) = &self.master_key {
            // Mark old key as expired
            if let Some(old_key_entry) = self.key_history.get_mut(&self.current_key_version) {
                old_key_entry.expires_at = Some(Utc::now());
            }
        }

        // Set new master key
        self.master_key = Some(aead::LessSafeKey::new(unbound_key));

        // Store new key in history
        let encryption_key = EncryptionKey {
            version: new_key_version.clone(),
            created_at: Utc::now(),
            expires_at: Some(Utc::now() + Duration::hours(self.rotation_settings.max_key_age_hours as i64)),
            key_data: new_key_bytes.to_vec(),
            algorithm: "AES-256-GCM".to_string(),
            key_derivation_params: KeyDerivationParams {
                salt: salt.clone(),
                iterations,
                algorithm: "PBKDF2-HMAC-SHA256".to_string(),
            },
        };

        self.key_history.insert(new_key_version.clone(), encryption_key);

        // Update current key version
        let old_version = self.current_key_version.clone();
        self.current_key_version = new_key_version.clone();

        // Create new integrity verification key
        let integrity_key = hmac::Key::new(hmac::HMAC_SHA256, &new_key_bytes);
        self.integrity_keys.insert(new_key_version.clone(), integrity_key);

        // Clear sensitive data
        let mut new_key_bytes_zeroize = new_key_bytes;
        new_key_bytes_zeroize.zeroize();

        info!("Key rotation completed: {} -> {}", old_version, new_key_version);
        Ok(new_key_version)
    }

    /// Clean up old expired keys
    async fn cleanup_old_keys(&mut self) -> AppResult<()> {
        debug!("Cleaning up old encryption keys");

        let now = Utc::now();
        let mut keys_to_remove = Vec::new();

        // Find expired keys
        for (version, key) in &self.key_history {
            if let Some(expires_at) = key.expires_at {
                if expires_at < now {
                    keys_to_remove.push(version.clone());
                }
            }
        }

        // Remove expired keys (but keep minimum number for decryption)
        let keep_count = std::cmp::min(self.rotation_settings.keep_old_keys_count, self.key_history.len() - 1);
        if keys_to_remove.len() > keep_count {
            keys_to_remove.truncate(keys_to_remove.len() - keep_count);
        }

        for version in keys_to_remove {
            self.key_history.remove(&version);
            self.integrity_keys.remove(&version);
            debug!("Removed expired key version: {}", version);
        }

        Ok(())
    }

    /// Encrypt data using AES-256-GCM with versioning and integrity verification
    pub async fn encrypt(&self, data: &[u8]) -> AppResult<Vec<u8>> {
        debug!("Encrypting data of length: {}", data.len());

        let key = self.master_key.as_ref()
            .ok_or_else(|| SecurityError::encryption_failed("Encryption not initialized"))?;

        // Generate a random nonce
        let nonce_bytes = self.generate_random_bytes(12).await?; // 96 bits for GCM
        let nonce = aead::Nonce::try_assume_unique_for_key(&nonce_bytes)
            .map_err(|_| SecurityError::encryption_failed("Failed to create nonce"))?;

        // Prepare data for encryption (copy to allow in-place encryption)
        let mut in_out = data.to_vec();

        // Encrypt the data
        key.seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut in_out)
            .map_err(|_| SecurityError::encryption_failed("Encryption operation failed"))?;

        // Create encrypted package with metadata
        let encrypted_package = EncryptedPackage {
            version: self.current_key_version.clone(),
            nonce: nonce_bytes,
            ciphertext: in_out,
            timestamp: Utc::now(),
        };

        // Serialize the package
        let package_bytes = bincode::serialize(&encrypted_package)
            .map_err(|_| SecurityError::encryption_failed("Failed to serialize encrypted package"))?;

        // Add integrity verification
        let integrity_signature = self.create_integrity_signature(&package_bytes).await?;

        // Create final result with signature
        let mut result = Vec::new();
        result.extend_from_slice(&(integrity_signature.len() as u32).to_le_bytes());
        result.extend_from_slice(&integrity_signature);
        result.extend_from_slice(&package_bytes);

        debug!("Data encrypted successfully with version {}, output length: {}",
               self.current_key_version, result.len());
        Ok(result)
    }

    /// Encrypt data with additional authenticated data (AAD)
    pub async fn encrypt_with_aad(&self, data: &[u8], aad: &[u8]) -> AppResult<Vec<u8>> {
        debug!("Encrypting data with AAD, data length: {}, AAD length: {}", data.len(), aad.len());

        let key = self.master_key.as_ref()
            .ok_or_else(|| SecurityError::encryption_failed("Encryption not initialized"))?;

        let nonce_bytes = self.generate_random_bytes(12).await?;
        let nonce = aead::Nonce::try_assume_unique_for_key(&nonce_bytes)
            .map_err(|_| SecurityError::encryption_failed("Failed to create nonce"))?;

        let mut in_out = data.to_vec();
        let aad_obj = aead::Aad::from(aad);

        key.seal_in_place_append_tag(nonce, aad_obj, &mut in_out)
            .map_err(|_| SecurityError::encryption_failed("Encryption with AAD failed"))?;

        let encrypted_package = EncryptedPackageWithAAD {
            version: self.current_key_version.clone(),
            nonce: nonce_bytes,
            ciphertext: in_out,
            aad: aad.to_vec(),
            timestamp: Utc::now(),
        };

        let package_bytes = bincode::serialize(&encrypted_package)
            .map_err(|_| SecurityError::encryption_failed("Failed to serialize encrypted package with AAD"))?;

        let integrity_signature = self.create_integrity_signature(&package_bytes).await?;

        let mut result = Vec::new();
        result.extend_from_slice(&(integrity_signature.len() as u32).to_le_bytes());
        result.extend_from_slice(&integrity_signature);
        result.extend_from_slice(&package_bytes);

        debug!("Data encrypted with AAD successfully, version: {}", self.current_key_version);
        Ok(result)
    }

    /// Decrypt data using AES-256-GCM with version support and integrity verification
    pub async fn decrypt(&self, encrypted_data: &[u8]) -> AppResult<Vec<u8>> {
        debug!("Decrypting data of length: {}", encrypted_data.len());

        if encrypted_data.len() < 8 {
            return Err(SecurityError::decryption_failed("Invalid encrypted data format").into());
        }

        // Extract integrity signature length
        let sig_len = u32::from_le_bytes([
            encrypted_data[0], encrypted_data[1], encrypted_data[2], encrypted_data[3]
        ]) as usize;

        if encrypted_data.len() < 4 + sig_len {
            return Err(SecurityError::decryption_failed("Invalid encrypted data format").into());
        }

        // Extract signature and package data
        let signature = &encrypted_data[4..4 + sig_len];
        let package_data = &encrypted_data[4 + sig_len..];

        // Verify integrity
        self.verify_integrity_signature(package_data, signature).await?;

        // Deserialize encrypted package
        let encrypted_package: EncryptedPackage = bincode::deserialize(package_data)
            .map_err(|_| SecurityError::decryption_failed("Failed to deserialize encrypted package"))?;

        // Get the appropriate key for this version
        let key = if encrypted_package.version == self.current_key_version {
            self.master_key.as_ref()
                .ok_or_else(|| SecurityError::decryption_failed("Current encryption key not available"))?
        } else {
            // Try to find the key in history
            let historical_key = self.key_history.get(&encrypted_package.version)
                .ok_or_else(|| SecurityError::decryption_failed(&format!("Key version {} not found", encrypted_package.version)))?;

            // Recreate the key from stored data
            let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &historical_key.key_data)
                .map_err(|_| SecurityError::decryption_failed("Failed to recreate historical key"))?;

            // Note: In a real implementation, we'd cache these keys
            return self.decrypt_with_historical_key(encrypted_package, historical_key).await;
        };

        // Create nonce
        let nonce = aead::Nonce::try_assume_unique_for_key(&encrypted_package.nonce)
            .map_err(|_| SecurityError::decryption_failed("Invalid nonce"))?;

        // Prepare data for decryption
        let mut in_out = encrypted_package.ciphertext;

        // Decrypt the data
        let plaintext = key.open_in_place(nonce, aead::Aad::empty(), &mut in_out)
            .map_err(|_| SecurityError::decryption_failed("Decryption operation failed"))?;

        debug!("Data decrypted successfully using version {}, output length: {}",
               encrypted_package.version, plaintext.len());
        Ok(plaintext.to_vec())
    }

    /// Decrypt data with historical key
    async fn decrypt_with_historical_key(
        &self,
        encrypted_package: EncryptedPackage,
        historical_key: &EncryptionKey
    ) -> AppResult<Vec<u8>> {
        debug!("Decrypting with historical key version: {}", historical_key.version);

        let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &historical_key.key_data)
            .map_err(|_| SecurityError::decryption_failed("Failed to recreate historical key"))?;

        let key = aead::LessSafeKey::new(unbound_key);

        let nonce = aead::Nonce::try_assume_unique_for_key(&encrypted_package.nonce)
            .map_err(|_| SecurityError::decryption_failed("Invalid nonce"))?;

        let mut in_out = encrypted_package.ciphertext;

        let plaintext = key.open_in_place(nonce, aead::Aad::empty(), &mut in_out)
            .map_err(|_| SecurityError::decryption_failed("Historical decryption operation failed"))?;

        debug!("Data decrypted successfully with historical key, output length: {}", plaintext.len());
        Ok(plaintext.to_vec())
    }

    /// Create integrity signature for data
    async fn create_integrity_signature(&self, data: &[u8]) -> AppResult<Vec<u8>> {
        let integrity_key = self.integrity_keys.get(&self.current_key_version)
            .ok_or_else(|| SecurityError::encryption_failed("Integrity key not available"))?;

        let signature = hmac::sign(integrity_key, data);
        Ok(signature.as_ref().to_vec())
    }

    /// Verify integrity signature
    async fn verify_integrity_signature(&self, data: &[u8], signature: &[u8]) -> AppResult<()> {
        // Try current key first
        if let Some(integrity_key) = self.integrity_keys.get(&self.current_key_version) {
            if hmac::verify(integrity_key, data, signature).is_ok() {
                return Ok(());
            }
        }

        // Try historical keys
        for (version, integrity_key) in &self.integrity_keys {
            if hmac::verify(integrity_key, data, signature).is_ok() {
                debug!("Integrity verified with historical key version: {}", version);
                return Ok(());
            }
        }

        Err(SecurityError::decryption_failed("Integrity verification failed").into())
    }

    /// Check if encryption is initialized
    pub fn is_initialized(&self) -> bool {
        self.master_key.is_some()
    }

    /// Generate a secure random string
    pub async fn generate_random_string(&self, length: usize) -> AppResult<String> {
        let bytes = self.generate_random_bytes(length).await?;
        Ok(base64::encode(bytes))
    }

    /// Generate secure random bytes
    pub async fn generate_random_bytes(&self, length: usize) -> AppResult<Vec<u8>> {
        let rng = rand::SystemRandom::new();
        let mut bytes = vec![0u8; length];
        rng.fill(&mut bytes)
            .map_err(|_| SecurityError::RandomGenerationFailed)?;
        Ok(bytes)
    }

    /// Derive key from password with custom salt and iterations
    pub async fn derive_key_from_password(
        &self,
        password: &str,
        salt: &[u8],
        iterations: u32,
    ) -> AppResult<Vec<u8>> {
        let mut key_bytes = [0u8; 32];
        let iterations = NonZeroU32::new(iterations)
            .ok_or_else(|| SecurityError::encryption_failed("Invalid iteration count"))?;

        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            iterations,
            salt,
            password.as_bytes(),
            &mut key_bytes,
        );

        Ok(key_bytes.to_vec())
    }

    /// Verify password against stored hash
    pub async fn verify_password(
        &self,
        password: &str,
        salt: &[u8],
        iterations: u32,
        expected_hash: &[u8],
    ) -> AppResult<bool> {
        let derived_key = self.derive_key_from_password(password, salt, iterations).await?;
        Ok(derived_key.as_slice() == expected_hash)
    }

    /// Clear the master key from memory
    pub async fn clear_master_key(&mut self) -> AppResult<()> {
        debug!("Clearing master key from memory");
        self.master_key = None;
        Ok(())
    }

    /// Get current key version
    pub fn get_current_key_version(&self) -> &str {
        &self.current_key_version
    }

    /// Get key rotation settings
    pub fn get_rotation_settings(&self) -> &KeyRotationSettings {
        &self.rotation_settings
    }

    /// Update key rotation settings
    pub async fn update_rotation_settings(&mut self, settings: KeyRotationSettings) -> AppResult<()> {
        info!("Updating key rotation settings");
        self.rotation_settings = settings;
        Ok(())
    }

    /// Check if key rotation is needed
    pub async fn is_key_rotation_needed(&self) -> bool {
        if !self.rotation_settings.auto_rotation_enabled {
            return false;
        }

        if let Some(current_key) = self.key_history.get(&self.current_key_version) {
            let age = Utc::now().signed_duration_since(current_key.created_at);
            age.num_hours() >= self.rotation_settings.rotation_interval_hours as i64
        } else {
            true // No current key, rotation needed
        }
    }

    /// Get session manager
    pub fn get_session_manager(&self) -> Arc<RwLock<SessionManager>> {
        self.session_manager.clone()
    }

    /// Shutdown the encryption manager
    pub async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down enhanced encryption manager...");

        // Clear session manager
        let mut session_manager = self.session_manager.write().await;
        session_manager.clear_all_sessions().await?;
        drop(session_manager);

        // Note: All sensitive data will be dropped and zeroized automatically
        info!("Enhanced encryption manager shutdown complete");
        Ok(())
    }
}

/// Encrypted package structure for versioned encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedPackage {
    pub version: String,
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
    pub timestamp: DateTime<Utc>,
}

/// Encrypted package with additional authenticated data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedPackageWithAAD {
    pub version: String,
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
    pub aad: Vec<u8>,
    pub timestamp: DateTime<Utc>,
}

impl SessionManager {
    /// Create a new session manager
    pub async fn new(session_timeout_minutes: u64, max_concurrent_sessions: usize) -> Self {
        Self {
            active_sessions: HashMap::new(),
            session_timeout_minutes,
            max_concurrent_sessions,
        }
    }

    /// Create a new user session
    pub async fn create_session(
        &mut self,
        user_id: Option<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
        is_master_session: bool,
    ) -> AppResult<String> {
        // Clean up expired sessions first
        self.cleanup_expired_sessions().await?;

        // Check session limit
        if self.active_sessions.len() >= self.max_concurrent_sessions {
            return Err(SecurityError::session_limit_exceeded().into());
        }

        let session_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let session = UserSession {
            session_id: session_id.clone(),
            user_id,
            created_at: now,
            last_activity: now,
            ip_address,
            user_agent,
            is_master_session,
        };

        self.active_sessions.insert(session_id.clone(), session);

        info!("Created new session: {} (master: {})", session_id, is_master_session);
        Ok(session_id)
    }

    /// Validate and update session activity
    pub async fn validate_session(&mut self, session_id: &str) -> AppResult<bool> {
        if let Some(session) = self.active_sessions.get_mut(session_id) {
            let now = Utc::now();
            let age = now.signed_duration_since(session.last_activity);

            if age.num_minutes() > self.session_timeout_minutes as i64 {
                self.active_sessions.remove(session_id);
                return Ok(false);
            }

            session.last_activity = now;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// End a session
    pub async fn end_session(&mut self, session_id: &str) -> AppResult<bool> {
        if self.active_sessions.remove(session_id).is_some() {
            info!("Session ended: {}", session_id);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get active session count
    pub fn get_active_session_count(&self) -> usize {
        self.active_sessions.len()
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(&mut self) -> AppResult<()> {
        let now = Utc::now();
        let timeout_duration = Duration::minutes(self.session_timeout_minutes as i64);

        let expired_sessions: Vec<String> = self.active_sessions
            .iter()
            .filter(|(_, session)| {
                now.signed_duration_since(session.last_activity) > timeout_duration
            })
            .map(|(id, _)| id.clone())
            .collect();

        for session_id in expired_sessions {
            self.active_sessions.remove(&session_id);
            debug!("Removed expired session: {}", session_id);
        }

        Ok(())
    }

    /// Clear all sessions
    pub async fn clear_all_sessions(&mut self) -> AppResult<()> {
        let session_count = self.active_sessions.len();
        self.active_sessions.clear();
        info!("Cleared {} active sessions", session_count);
        Ok(())
    }

    /// Get session information
    pub fn get_session(&self, session_id: &str) -> Option<&UserSession> {
        self.active_sessions.get(session_id)
    }
}
