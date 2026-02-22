//! Argon2-based password hasher implementation.
//!
//! This module provides a secure password hashing implementation using the Argon2id
//! algorithm, which is the recommended choice for password hashing as of 2023+.
//!
//! Both `hash_password` and `verify_password` are CPU-intensive (~300-500 ms with
//! default parameters) so they run inside `spawn_blocking` to avoid blocking Tokio
//! worker threads.

use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use async_trait::async_trait;
use rand_core::OsRng;

use crate::application::ports::auth_ports::PasswordHasherPort;
use crate::common::errors::{DomainError, ErrorKind};

/// Argon2-based implementation of the PasswordHasherPort.
///
/// Uses Argon2id algorithm which provides resistance against both side-channel
/// and GPU-based attacks. This is the recommended algorithm for password hashing.
///
/// The struct is stateless — `Argon2::default()` is constructed per call inside
/// `spawn_blocking` so it is `Send` without extra synchronisation.
#[derive(Debug, Clone)]
pub struct Argon2PasswordHasher {
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

#[async_trait]
impl PasswordHasherPort for Argon2PasswordHasher {
    async fn hash_password(&self, password: &str) -> Result<String, DomainError> {
        let pwd = password.to_owned();
        tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);
            Argon2::default()
                .hash_password(pwd.as_bytes(), &salt)
                .map(|hash| hash.to_string())
                .map_err(|e| {
                    DomainError::new(
                        ErrorKind::InternalError,
                        "PasswordHasher",
                        format!("Error generating password hash: {}", e),
                    )
                })
        })
        .await
        .map_err(|e| {
            DomainError::new(
                ErrorKind::InternalError,
                "PasswordHasher",
                format!("Task join error: {}", e),
            )
        })?
    }

    async fn verify_password(&self, password: &str, hash: &str) -> Result<bool, DomainError> {
        let pwd = password.to_owned();
        let hash = hash.to_owned();
        tokio::task::spawn_blocking(move || {
            let parsed_hash = PasswordHash::new(&hash).map_err(|e| {
                DomainError::new(
                    ErrorKind::InternalError,
                    "PasswordHasher",
                    format!("Error processing password hash: {}", e),
                )
            })?;

            Ok(Argon2::default()
                .verify_password(pwd.as_bytes(), &parsed_hash)
                .is_ok())
        })
        .await
        .map_err(|e| {
            DomainError::new(
                ErrorKind::InternalError,
                "PasswordHasher",
                format!("Task join error: {}", e),
            )
        })?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hash_and_verify_password() {
        let hasher = Argon2PasswordHasher::new();
        let password = "test_password_123";

        let hash = hasher
            .hash_password(password)
            .await
            .expect("Should hash password");
        assert!(
            hasher
                .verify_password(password, &hash)
                .await
                .expect("Should verify")
        );
        assert!(
            !hasher
                .verify_password("wrong_password", &hash)
                .await
                .expect("Should verify")
        );
    }

    #[tokio::test]
    async fn test_different_hashes_for_same_password() {
        let hasher = Argon2PasswordHasher::new();
        let password = "same_password";

        let hash1 = hasher.hash_password(password).await.expect("Should hash");
        let hash2 = hasher.hash_password(password).await.expect("Should hash");

        // Hashes should be different due to random salt
        assert_ne!(hash1, hash2);

        // But both should verify correctly
        assert!(
            hasher
                .verify_password(password, &hash1)
                .await
                .expect("Should verify")
        );
        assert!(
            hasher
                .verify_password(password, &hash2)
                .await
                .expect("Should verify")
        );
    }
}
