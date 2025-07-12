use ring::{aead, pbkdf2, rand};
use ring::rand::SecureRandom;
use std::num::NonZeroU32;
use tracing::{info, debug, error};
use base64;

use crate::error::{AppResult, SecurityError};

/// Encryption manager for handling AES-256-GCM encryption
#[derive(Debug)]
pub struct EncryptionManager {
    /// Master key for encryption operations
    master_key: Option<aead::LessSafeKey>,
}

impl EncryptionManager {
    /// Create a new encryption manager
    pub async fn new() -> AppResult<Self> {
        info!("Initializing encryption manager...");

        let manager = Self {
            master_key: None,
        };

        info!("Encryption manager initialized successfully");
        Ok(manager)
    }

    /// Initialize encryption with a master password
    pub async fn initialize_with_password(&mut self, password: &str) -> AppResult<()> {
        debug!("Initializing encryption with master password");

        // Generate a random salt
        let salt = self.generate_random_bytes(32).await?;

        // Derive key from password using PBKDF2
        let mut key_bytes = [0u8; 32]; // 256 bits for AES-256
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(100_000).unwrap(), // 100,000 iterations
            &salt,
            password.as_bytes(),
            &mut key_bytes,
        );

        // Create the encryption key
        let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes)
            .map_err(|_| SecurityError::encryption_failed("Failed to create encryption key"))?;

        self.master_key = Some(aead::LessSafeKey::new(unbound_key));

        info!("Encryption initialized with master password");
        Ok(())
    }

    /// Encrypt data using AES-256-GCM
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

        // Prepend nonce to encrypted data
        let mut result = nonce_bytes;
        result.extend_from_slice(&in_out);

        debug!("Data encrypted successfully, output length: {}", result.len());
        Ok(result)
    }

    /// Decrypt data using AES-256-GCM
    pub async fn decrypt(&self, encrypted_data: &[u8]) -> AppResult<Vec<u8>> {
        debug!("Decrypting data of length: {}", encrypted_data.len());

        if encrypted_data.len() < 12 {
            return Err(SecurityError::decryption_failed("Invalid encrypted data format").into());
        }

        let key = self.master_key.as_ref()
            .ok_or_else(|| SecurityError::encryption_failed("Encryption not initialized"))?;

        // Extract nonce and encrypted data
        let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
        let nonce = aead::Nonce::try_assume_unique_for_key(nonce_bytes)
            .map_err(|_| SecurityError::decryption_failed("Invalid nonce"))?;

        // Prepare data for decryption
        let mut in_out = ciphertext.to_vec();

        // Decrypt the data
        let plaintext = key.open_in_place(nonce, aead::Aad::empty(), &mut in_out)
            .map_err(|_| SecurityError::decryption_failed("Decryption operation failed"))?;

        debug!("Data decrypted successfully, output length: {}", plaintext.len());
        Ok(plaintext.to_vec())
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

    /// Shutdown the encryption manager
    pub async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down encryption manager...");
        // Note: master_key will be dropped automatically, clearing sensitive data
        Ok(())
    }
}
