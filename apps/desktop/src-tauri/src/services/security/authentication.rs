use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, error};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use ring::{pbkdf2, rand};
use ring::rand::SecureRandom;
use std::num::NonZeroU32;
use base64;

use crate::error::{AppResult, SecurityError};
use crate::services::security::KeyVault;
use crate::models::security::{AuthenticationSession, SecurityAuditEntry, SecurityEventType, SecuritySeverity, SecurityResult};

const MASTER_PASSWORD_KEY: &str = "master_password_hash";
const MASTER_PASSWORD_SALT_KEY: &str = "master_password_salt";
const MASTER_PASSWORD_ITERATIONS_KEY: &str = "master_password_iterations";
const DEFAULT_ITERATIONS: u32 = 100_000;

/// Authentication manager for handling master password authentication
pub struct AuthenticationManager {
    key_vault: Arc<RwLock<KeyVault>>,
    current_session: Option<AuthenticationSession>,
    failed_attempts: u32,
    max_attempts: u32,
}

impl AuthenticationManager {
    /// Create a new authentication manager
    pub async fn new(key_vault: Arc<RwLock<KeyVault>>) -> AppResult<Self> {
        info!("Initializing authentication manager...");

        let manager = Self {
            key_vault,
            current_session: None,
            failed_attempts: 0,
            max_attempts: 5,
        };

        info!("Authentication manager initialized successfully");
        Ok(manager)
    }

    /// Authenticate with master password
    pub async fn authenticate(&mut self, password: &str) -> AppResult<bool> {
        debug!("Authenticating user");

        // Check if we've exceeded max attempts
        if self.failed_attempts >= self.max_attempts {
            error!("Authentication blocked due to too many failed attempts");
            return Err(SecurityError::authentication_failed("Too many failed attempts").into());
        }

        // Check if master password is set
        if !self.has_master_password().await? {
            error!("No master password set");
            return Err(SecurityError::authentication_failed("No master password configured").into());
        }

        // Get stored password data
        let key_vault = self.key_vault.read().await;
        let stored_hash = key_vault.get_secret(MASTER_PASSWORD_KEY).await?
            .ok_or_else(|| SecurityError::authentication_failed("Master password not found"))?;
        let salt_str = key_vault.get_secret(MASTER_PASSWORD_SALT_KEY).await?
            .ok_or_else(|| SecurityError::authentication_failed("Password salt not found"))?;
        let iterations_str = key_vault.get_secret(MASTER_PASSWORD_ITERATIONS_KEY).await?
            .unwrap_or_else(|| DEFAULT_ITERATIONS.to_string());
        drop(key_vault);

        // Parse stored data
        let stored_hash_bytes = base64::decode(&stored_hash)
            .map_err(|_| SecurityError::authentication_failed("Invalid stored hash format"))?;
        let salt_bytes = base64::decode(&salt_str)
            .map_err(|_| SecurityError::authentication_failed("Invalid salt format"))?;
        let iterations: u32 = iterations_str.parse()
            .map_err(|_| SecurityError::authentication_failed("Invalid iterations format"))?;

        // Derive key from provided password
        let mut derived_key = [0u8; 32];
        let iterations_nz = NonZeroU32::new(iterations)
            .ok_or_else(|| SecurityError::authentication_failed("Invalid iteration count"))?;

        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            iterations_nz,
            &salt_bytes,
            password.as_bytes(),
            &mut derived_key,
        );

        // Compare with stored hash
        let is_valid = derived_key.as_slice() == stored_hash_bytes.as_slice();

        if is_valid {
            debug!("Authentication successful");
            self.failed_attempts = 0;

            // Create new session
            self.current_session = Some(AuthenticationSession::new(60)); // 60 minute session

            Ok(true)
        } else {
            error!("Authentication failed - invalid password");
            self.failed_attempts += 1;
            Ok(false)
        }
    }

    /// Set master password
    pub async fn set_master_password(&mut self, password: &str) -> AppResult<()> {
        debug!("Setting master password");

        if password.len() < 8 {
            return Err(SecurityError::authentication_failed("Password must be at least 8 characters").into());
        }

        // Generate random salt
        let rng = rand::SystemRandom::new();
        let mut salt = [0u8; 32];
        rng.fill(&mut salt)
            .map_err(|_| SecurityError::RandomGenerationFailed)?;

        // Derive key from password
        let mut password_hash = [0u8; 32];
        let iterations = NonZeroU32::new(DEFAULT_ITERATIONS).unwrap();

        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            iterations,
            &salt,
            password.as_bytes(),
            &mut password_hash,
        );

        // Store in key vault
        let mut key_vault = self.key_vault.write().await;
        key_vault.store_secret(MASTER_PASSWORD_KEY, &base64::encode(&password_hash)).await?;
        key_vault.store_secret(MASTER_PASSWORD_SALT_KEY, &base64::encode(&salt)).await?;
        key_vault.store_secret(MASTER_PASSWORD_ITERATIONS_KEY, &DEFAULT_ITERATIONS.to_string()).await?;
        drop(key_vault);

        info!("Master password set successfully");
        Ok(())
    }

    /// Check if master password is set
    pub async fn has_master_password(&self) -> AppResult<bool> {
        debug!("Checking if master password is set");

        let key_vault = self.key_vault.read().await;
        let has_password = key_vault.has_secret(MASTER_PASSWORD_KEY).await?;
        drop(key_vault);

        debug!("Master password set: {}", has_password);
        Ok(has_password)
    }

    /// Check if user is currently authenticated
    pub fn is_authenticated(&self) -> bool {
        if let Some(session) = &self.current_session {
            session.is_active && !session.is_expired()
        } else {
            false
        }
    }

    /// Get current session information
    pub fn get_current_session(&self) -> Option<&AuthenticationSession> {
        self.current_session.as_ref()
    }

    /// Extend current session
    pub fn extend_session(&mut self, timeout_minutes: u32) -> AppResult<()> {
        if let Some(session) = &mut self.current_session {
            session.extend_session(timeout_minutes);
            debug!("Session extended by {} minutes", timeout_minutes);
            Ok(())
        } else {
            Err(SecurityError::authentication_failed("No active session").into())
        }
    }

    /// Logout and invalidate current session
    pub fn logout(&mut self) -> AppResult<()> {
        if let Some(session) = &mut self.current_session {
            session.invalidate();
            debug!("User logged out, session invalidated");
        }
        self.current_session = None;
        Ok(())
    }

    /// Reset failed attempt counter
    pub fn reset_failed_attempts(&mut self) {
        self.failed_attempts = 0;
        debug!("Failed attempt counter reset");
    }

    /// Get failed attempt count
    pub fn get_failed_attempts(&self) -> u32 {
        self.failed_attempts
    }

    /// Check if authentication is locked due to failed attempts
    pub fn is_locked(&self) -> bool {
        self.failed_attempts >= self.max_attempts
    }

    /// Change master password (requires current password)
    pub async fn change_master_password(&mut self, current_password: &str, new_password: &str) -> AppResult<()> {
        debug!("Changing master password");

        // Verify current password
        if !self.authenticate(current_password).await? {
            return Err(SecurityError::authentication_failed("Current password is incorrect").into());
        }

        // Set new password
        self.set_master_password(new_password).await?;

        info!("Master password changed successfully");
        Ok(())
    }

    /// Shutdown the authentication manager
    pub async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down authentication manager...");
        Ok(())
    }
}
