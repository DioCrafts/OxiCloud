//! JWT-based token service implementation.
//!
//! This module provides JWT token generation and validation functionality,
//! implementing the TokenServicePort trait defined in the application layer.
//!
//! **Performance optimisation**: a per-token validation cache (moka, lock-free)
//! avoids repeating the HMAC-SHA256 verification on every request for the same
//! token.  Entries are keyed by a fast BLAKE3 hash of the raw token string and
//! auto-expire after a short TTL (30 s by default) so revoked tokens don't stay
//! valid for long.

use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use moka::sync::Cache;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use uuid::Uuid;

use crate::application::ports::auth_ports::{TokenClaims, TokenServicePort};
use crate::common::errors::{DomainError, ErrorKind};
use crate::domain::entities::user::User;

/// Internal JWT claims structure for serialization.
/// This is the actual JWT payload structure used by jsonwebtoken crate.
#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    /// Subject identifier - contains the user ID
    pub sub: String,
    /// Expiration timestamp (seconds since Unix epoch)
    pub exp: i64,
    /// Issued at timestamp (seconds since Unix epoch)
    pub iat: i64,
    /// JWT unique ID for token tracking and revocation
    pub jti: String,
    /// Username for display and identification purposes
    pub username: String,
    /// User email for communication and identification
    pub email: String,
    /// User role for authorization checks
    pub role: String,
}

impl From<JwtClaims> for TokenClaims {
    fn from(claims: JwtClaims) -> Self {
        TokenClaims {
            sub: claims.sub,
            exp: claims.exp,
            iat: claims.iat,
            jti: claims.jti,
            username: claims.username,
            email: claims.email,
            role: claims.role,
        }
    }
}

/// JWT-based implementation of the TokenServicePort.
///
/// This service handles JWT token generation and validation for user authentication.
/// It uses HS256 algorithm for signing tokens.
///
/// ## Validation cache
///
/// `jsonwebtoken::decode()` performs HMAC-SHA256 verification on every call.
/// While fast in absolute terms (~2-4 µs on modern hardware), at 10 k req/s
/// that is 20-40 ms of pure CPU per second — and it is synchronous, blocking
/// the Tokio worker thread.
///
/// The cache uses the **BLAKE3** hash of the raw token string as key (32-byte,
/// ~0.1 µs to compute — 20× cheaper than HMAC verification) and stores the
/// validated `TokenClaims`.  On a cache hit the HMAC step is completely
/// skipped.
///
/// **Security properties**:
/// - TTL of 30 s bounds the window in which a revoked token remains valid.
/// - Max 50 000 entries (≈ 4 MB RSS) with LRU eviction prevents DoS via
///   unique-token flooding.
/// - Expired tokens are never cached (decode itself rejects them first).
pub struct JwtTokenService {
    /// Secret key used for signing JWT tokens
    jwt_secret: String,
    /// Expiration time for access tokens in seconds
    access_token_expiry: i64,
    /// Expiration time for refresh tokens in seconds
    refresh_token_expiry: i64,
    /// Validation result cache: blake3(token) → TokenClaims
    validation_cache: Cache<[u8; 32], TokenClaims>,
    /// Cache hit counter (for observability / metrics)
    cache_hits: AtomicU64,
    /// Cache miss counter
    cache_misses: AtomicU64,
}

/// Default TTL for cached validation results (seconds).
const VALIDATION_CACHE_TTL_SECS: u64 = 30;

/// Maximum number of cached token validations.
const VALIDATION_CACHE_MAX_ENTRIES: u64 = 50_000;

impl JwtTokenService {
    /// Create a new JwtTokenService with the specified configuration.
    ///
    /// # Arguments
    /// * `jwt_secret` - Secret key for signing tokens (should be at least 32 bytes)
    /// * `access_token_expiry_secs` - Lifetime of access tokens in seconds
    /// * `refresh_token_expiry_secs` - Lifetime of refresh tokens in seconds
    pub fn new(
        jwt_secret: String,
        access_token_expiry_secs: i64,
        refresh_token_expiry_secs: i64,
    ) -> Self {
        let validation_cache = Cache::builder()
            .max_capacity(VALIDATION_CACHE_MAX_ENTRIES)
            .time_to_live(Duration::from_secs(VALIDATION_CACHE_TTL_SECS))
            .build();

        tracing::info!(
            "JWT validation cache initialised: TTL={}s, max_entries={}",
            VALIDATION_CACHE_TTL_SECS,
            VALIDATION_CACHE_MAX_ENTRIES,
        );

        Self {
            jwt_secret,
            access_token_expiry: access_token_expiry_secs,
            refresh_token_expiry: refresh_token_expiry_secs,
            validation_cache,
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
        }
    }

    /// Compute a fast BLAKE3 hash of a token string, used as cache key.
    ///
    /// BLAKE3 is ~20× faster than SHA-256 and ~40× faster than HMAC-SHA256
    /// verification through `jsonwebtoken`, making it an ideal pre-filter.
    #[inline]
    fn token_hash(token: &str) -> [u8; 32] {
        blake3::hash(token.as_bytes()).into()
    }

    /// Return cache hit/miss statistics for monitoring.
    pub fn cache_stats(&self) -> (u64, u64) {
        (
            self.cache_hits.load(Ordering::Relaxed),
            self.cache_misses.load(Ordering::Relaxed),
        )
    }
}

impl TokenServicePort for JwtTokenService {
    fn generate_access_token(&self, user: &User) -> Result<String, DomainError> {
        let now = Utc::now().timestamp();

        // Log information for debugging
        tracing::debug!(
            "Generating token for user: {}, id: {}, role: {}",
            user.username(),
            user.id(),
            user.role()
        );

        let claims = JwtClaims {
            sub: user.id().to_string(),
            exp: now + self.access_token_expiry,
            iat: now,
            jti: Uuid::new_v4().to_string(),
            username: user.username().to_string(),
            email: user.email().to_string(),
            role: format!("{}", user.role()),
        };

        // Log JWT claims for debugging
        tracing::debug!(
            "JWT claims: sub={}, exp={}, iat={}",
            claims.sub,
            claims.exp,
            claims.iat
        );

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| {
            tracing::error!("Error generating token: {}", e);
            DomainError::new(
                ErrorKind::InternalError,
                "TokenService",
                format!("Error generating token: {}", e),
            )
        })
    }

    fn validate_token(&self, token: &str) -> Result<TokenClaims, DomainError> {
        // ── 1. Fast-path: check the validation cache ─────────────
        let key = Self::token_hash(token);

        if let Some(cached_claims) = self.validation_cache.get(&key) {
            // Even on a cache hit we must verify the token hasn't expired
            // since it was cached (the cached exp is an absolute timestamp).
            let now = Utc::now().timestamp();
            if cached_claims.exp > now {
                self.cache_hits.fetch_add(1, Ordering::Relaxed);
                return Ok(cached_claims);
            }
            // Token expired while cached — evict and fall through to full
            // verification which will return the proper "Token expired" error.
            self.validation_cache.invalidate(&key);
        }

        // ── 2. Slow-path: full HMAC-SHA256 verification ─────────
        self.cache_misses.fetch_add(1, Ordering::Relaxed);

        let validation = Validation::new(Algorithm::HS256);

        let token_data = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation,
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                DomainError::new(ErrorKind::AccessDenied, "TokenService", "Token expired")
            }
            _ => DomainError::new(
                ErrorKind::AccessDenied,
                "TokenService",
                format!("Invalid token: {}", e),
            ),
        })?;

        let claims: TokenClaims = token_data.claims.into();

        // ── 3. Store in cache for subsequent requests ────────────
        // Only cache tokens that won't expire within the cache TTL window,
        // avoiding stale positives right at the boundary.
        let remaining_secs = claims.exp - Utc::now().timestamp();
        if remaining_secs > VALIDATION_CACHE_TTL_SECS as i64 {
            self.validation_cache.insert(key, claims.clone());
        }

        Ok(claims)
    }

    fn generate_refresh_token(&self) -> String {
        Uuid::new_v4().to_string()
    }

    fn refresh_token_expiry_secs(&self) -> i64 {
        self.refresh_token_expiry
    }

    fn refresh_token_expiry_days(&self) -> i64 {
        self.refresh_token_expiry / (24 * 3600)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::user::{User, UserRole};

    fn create_test_user() -> User {
        User::from_data(
            "test-user-id".to_string(),
            "testuser".to_string(),
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            UserRole::User,
            1024 * 1024 * 1024, // 1GB
            0,
            chrono::Utc::now(),
            chrono::Utc::now(),
            None,
            true,
        )
    }

    #[test]
    fn test_generate_and_validate_token() {
        let service = JwtTokenService::new(
            "test_secret_key_at_least_32_bytes_long".to_string(),
            3600,  // 1 hour
            86400, // 1 day
        );

        let user = create_test_user();
        let token = service
            .generate_access_token(&user)
            .expect("Should generate token");

        let claims = service
            .validate_token(&token)
            .expect("Should validate token");
        assert_eq!(claims.sub, user.id());
        assert_eq!(claims.username, user.username());
        assert_eq!(claims.email, user.email());
    }

    #[test]
    fn test_refresh_token_is_unique() {
        let service = JwtTokenService::new("secret".to_string(), 3600, 86400);

        let token1 = service.generate_refresh_token();
        let token2 = service.generate_refresh_token();

        assert_ne!(token1, token2);
    }

    #[test]
    fn test_invalid_token() {
        let service = JwtTokenService::new("secret".to_string(), 3600, 86400);

        let result = service.validate_token("invalid_token");
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_cache_hit() {
        let service = JwtTokenService::new(
            "test_secret_key_at_least_32_bytes_long".to_string(),
            3600,
            86400,
        );

        let user = create_test_user();
        let token = service
            .generate_access_token(&user)
            .expect("Should generate token");

        // First call: cache miss — performs full HMAC verification
        let claims1 = service.validate_token(&token).expect("Should validate");

        // Second call: cache hit — skips HMAC, returns cloned claims
        let claims2 = service.validate_token(&token).expect("Should validate from cache");

        assert_eq!(claims1.sub, claims2.sub);
        assert_eq!(claims1.username, claims2.username);

        let (hits, misses) = service.cache_stats();
        assert_eq!(hits, 1, "Expected 1 cache hit");
        assert_eq!(misses, 1, "Expected 1 cache miss");
    }

    #[test]
    fn test_invalid_token_not_cached() {
        let service = JwtTokenService::new("secret".to_string(), 3600, 86400);

        // Invalid tokens should never be cached
        let _ = service.validate_token("bad_token");
        let _ = service.validate_token("bad_token");

        let (hits, _misses) = service.cache_stats();
        assert_eq!(hits, 0, "Invalid tokens should never produce cache hits");
    }
}
