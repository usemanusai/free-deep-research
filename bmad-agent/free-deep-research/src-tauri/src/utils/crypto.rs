use ring::{digest, pbkdf2, rand};
use ring::rand::SecureRandom;
use std::num::NonZeroU32;

use crate::error::{AppResult, SecurityError};

/// Generate a secure random salt
pub fn generate_salt() -> AppResult<Vec<u8>> {
    let rng = rand::SystemRandom::new();
    let mut salt = vec![0u8; 32]; // 256-bit salt
    rng.fill(&mut salt)
        .map_err(|_| SecurityError::RandomGenerationFailed)?;
    Ok(salt)
}

/// Derive a key from a password using PBKDF2
pub fn derive_key(password: &str, salt: &[u8], iterations: u32) -> AppResult<Vec<u8>> {
    let iterations = NonZeroU32::new(iterations)
        .ok_or_else(|| SecurityError::key_derivation_failed("Invalid iteration count"))?;
    
    let mut key = vec![0u8; 32]; // 256-bit key
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        iterations,
        salt,
        password.as_bytes(),
        &mut key,
    );
    
    Ok(key)
}

/// Generate a secure random string
pub fn generate_random_string(length: usize) -> AppResult<String> {
    let rng = rand::SystemRandom::new();
    let mut bytes = vec![0u8; length];
    rng.fill(&mut bytes)
        .map_err(|_| SecurityError::RandomGenerationFailed)?;
    Ok(base64::encode(bytes))
}

/// Generate secure random bytes
pub fn generate_random_bytes(length: usize) -> AppResult<Vec<u8>> {
    let rng = rand::SystemRandom::new();
    let mut bytes = vec![0u8; length];
    rng.fill(&mut bytes)
        .map_err(|_| SecurityError::RandomGenerationFailed)?;
    Ok(bytes)
}

/// Hash data using SHA-256
pub fn hash_sha256(data: &[u8]) -> Vec<u8> {
    digest::digest(&digest::SHA256, data).as_ref().to_vec()
}

/// Verify a hash
pub fn verify_hash(data: &[u8], expected_hash: &[u8]) -> bool {
    let actual_hash = hash_sha256(data);
    actual_hash == expected_hash
}

/// Constant-time comparison of byte slices
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    
    result == 0
}

/// Secure memory clearing (best effort)
pub fn secure_clear(data: &mut [u8]) {
    // Use volatile writes to prevent compiler optimization
    for byte in data.iter_mut() {
        unsafe {
            std::ptr::write_volatile(byte, 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_salt() {
        let salt1 = generate_salt().unwrap();
        let salt2 = generate_salt().unwrap();
        
        assert_eq!(salt1.len(), 32);
        assert_eq!(salt2.len(), 32);
        assert_ne!(salt1, salt2); // Should be different
    }

    #[test]
    fn test_derive_key() {
        let password = "test_password";
        let salt = generate_salt().unwrap();
        let iterations = 100_000;
        
        let key1 = derive_key(password, &salt, iterations).unwrap();
        let key2 = derive_key(password, &salt, iterations).unwrap();
        
        assert_eq!(key1.len(), 32);
        assert_eq!(key1, key2); // Same password and salt should produce same key
        
        // Different salt should produce different key
        let different_salt = generate_salt().unwrap();
        let key3 = derive_key(password, &different_salt, iterations).unwrap();
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_hash_sha256() {
        let data = b"test data";
        let hash1 = hash_sha256(data);
        let hash2 = hash_sha256(data);
        
        assert_eq!(hash1.len(), 32); // SHA-256 produces 32-byte hash
        assert_eq!(hash1, hash2); // Same data should produce same hash
        
        let different_data = b"different data";
        let hash3 = hash_sha256(different_data);
        assert_ne!(hash1, hash3); // Different data should produce different hash
    }

    #[test]
    fn test_verify_hash() {
        let data = b"test data";
        let hash = hash_sha256(data);
        
        assert!(verify_hash(data, &hash));
        assert!(!verify_hash(b"different data", &hash));
    }

    #[test]
    fn test_constant_time_eq() {
        let a = b"test";
        let b = b"test";
        let c = b"different";
        
        assert!(constant_time_eq(a, b));
        assert!(!constant_time_eq(a, c));
        assert!(!constant_time_eq(a, b"tes")); // Different length
    }
}
