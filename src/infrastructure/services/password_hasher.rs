//! Argon2-based password hasher implementation.
//! 
//! This module provides a secure password hashing implementation using the Argon2id
//! algorithm, which is the recommended choice for password hashing as of 2023+.

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use rand_core::OsRng;

use crate::application::ports::auth_ports::PasswordHasherPort;
use crate::common::errors::{DomainError, ErrorKind};

/// Argon2-based implementation of the PasswordHasherPort.
/// 
/// Uses Argon2id algorithm which provides resistance against both side-channel
/// and GPU-based attacks. This is the recommended algorithm for password hashing.
#[derive(Debug, Clone)]
pub struct Argon2PasswordHasher {
    /// Argon2 hasher instance - uses default secure parameters
    _private: (),
}

impl Argon2PasswordHasher {
    /// Create a new Argon2PasswordHasher with default secure parameters.
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for Argon2PasswordHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl PasswordHasherPort for Argon2PasswordHasher {
    fn hash_password(&self, password: &str) -> Result<String, DomainError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        argon2.hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| DomainError::new(
                ErrorKind::InternalError,
                "PasswordHasher",
                format!("Error al generar hash de password: {}", e)
            ))
    }
    
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, DomainError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| DomainError::new(
                ErrorKind::InternalError,
                "PasswordHasher",
                format!("Error al procesar hash: {}", e)
            ))?;
        
        Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash_and_verify_password() {
        let hasher = Argon2PasswordHasher::new();
        let password = "test_password_123";
        
        let hash = hasher.hash_password(password).expect("Should hash password");
        assert!(hasher.verify_password(password, &hash).expect("Should verify"));
        assert!(!hasher.verify_password("wrong_password", &hash).expect("Should verify"));
    }
    
    #[test]
    fn test_different_hashes_for_same_password() {
        let hasher = Argon2PasswordHasher::new();
        let password = "same_password";
        
        let hash1 = hasher.hash_password(password).expect("Should hash");
        let hash2 = hasher.hash_password(password).expect("Should hash");
        
        // Hashes should be different due to random salt
        assert_ne!(hash1, hash2);
        
        // But both should verify correctly
        assert!(hasher.verify_password(password, &hash1).expect("Should verify"));
        assert!(hasher.verify_password(password, &hash2).expect("Should verify"));
    }
}
