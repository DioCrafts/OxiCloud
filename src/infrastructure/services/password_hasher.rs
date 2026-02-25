//! Argon2-based password hasher implementation.
//!
//! This module provides a secure password hashing implementation using the Argon2id
//! algorithm, which is the recommended choice for password hashing as of 2023+.
//!
//! Both `hash_password` and `verify_password` are CPU-intensive so they run inside
//! `spawn_blocking` to avoid blocking Tokio worker threads.
//!
//! The Argon2id parameters (`m_cost`, `t_cost`, `p_cost`) are injected at
//! construction time from `AuthConfig`, so operators can tune security vs.
//! latency via environment variables.

use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use async_trait::async_trait;
use rand_core::OsRng;

use crate::application::ports::auth_ports::PasswordHasherPort;
use crate::common::errors::{DomainError, ErrorKind};

/// Argon2-based implementation of the PasswordHasherPort.
///
/// Uses Argon2id algorithm which provides resistance against both side-channel
/// and GPU-based attacks. This is the recommended algorithm for password hashing.
///
/// The struct stores the validated `Params` so that `Argon2` can be cheaply
/// reconstructed inside each `spawn_blocking` call (it is not `Send`).
#[derive(Debug, Clone)]
pub struct Argon2PasswordHasher {
    params: Params,
}

impl Argon2PasswordHasher {
    /// Create a new hasher with explicit Argon2id parameters.
    ///
    /// - `memory_cost`:  memory in KiB (e.g. 65536 = 64 MiB)
    /// - `time_cost`:    number of iterations (e.g. 3)
    /// - `parallelism`:  lanes of parallelism (e.g. 2)
    ///
    /// Panics at startup if the parameters are invalid (caught immediately).
    pub fn new(memory_cost: u32, time_cost: u32, parallelism: u32) -> Self {
        let params = Params::new(memory_cost, time_cost, parallelism, None).unwrap_or_else(|e| {
            panic!(
                "Invalid Argon2 parameters (m={}, t={}, p={}): {}",
                memory_cost, time_cost, parallelism, e
            )
        });

        tracing::info!(
            "Argon2PasswordHasher initialized: m_cost={} KiB, t_cost={}, p_cost={}",
            memory_cost,
            time_cost,
            parallelism,
        );

        Self { params }
    }
}

#[async_trait]
impl PasswordHasherPort for Argon2PasswordHasher {
    async fn hash_password(&self, password: &str) -> Result<String, DomainError> {
        let pwd = password.to_owned();
        let params = self.params.clone();
        tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
            argon2
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

            // verify_password reads m/t/p from the hash string itself,
            // so existing hashes (with old params) verify correctly.
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

    /// Test params: small values so tests are fast (~10 ms instead of ~400 ms)
    fn test_hasher() -> Argon2PasswordHasher {
        Argon2PasswordHasher::new(16384, 1, 1)
    }

    #[tokio::test]
    async fn test_hash_and_verify_password() {
        let hasher = test_hasher();
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
        let hasher = test_hasher();
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
