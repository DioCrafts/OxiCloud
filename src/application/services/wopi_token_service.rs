//! WOPI access token service.
//!
//! Generates and validates WOPI-scoped JWT tokens that are separate from
//! the regular authentication tokens. Uses the same `jsonwebtoken` crate
//! but with a distinct `scope: "wopi"` claim to prevent token confusion.

use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::common::errors::{DomainError, ErrorKind};

/// JWT claims for WOPI access tokens.
#[derive(Debug, Serialize, Deserialize)]
pub struct WopiTokenClaims {
    /// User ID
    pub sub: String,
    /// File ID this token grants access to
    pub file_id: String,
    /// Whether the user can write (edit) the file
    pub can_write: bool,
    /// Token scope — always "wopi" to distinguish from auth tokens
    pub scope: String,
    /// Display name for the editor UI
    pub username: String,
    /// Expiration timestamp (seconds since Unix epoch)
    pub exp: i64,
    /// Issued at timestamp
    pub iat: i64,
}

/// Service for generating and validating WOPI access tokens.
pub struct WopiTokenService {
    secret: String,
    token_ttl_secs: i64,
}

impl WopiTokenService {
    pub fn new(secret: String, token_ttl_secs: i64) -> Self {
        Self {
            secret,
            token_ttl_secs,
        }
    }

    /// Generate a WOPI access token for a specific file and user.
    ///
    /// Returns `(token_string, expiration_unix_ms)`.
    pub fn generate_token(
        &self,
        file_id: &str,
        user_id: &str,
        username: &str,
        can_write: bool,
    ) -> Result<(String, i64), DomainError> {
        let now = Utc::now().timestamp();
        let claims = WopiTokenClaims {
            sub: user_id.to_string(),
            file_id: file_id.to_string(),
            can_write,
            scope: "wopi".to_string(),
            username: username.to_string(),
            exp: now + self.token_ttl_secs,
            iat: now,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| {
            DomainError::new(
                ErrorKind::InternalError,
                "WopiTokenService",
                format!("Failed to generate WOPI token: {}", e),
            )
        })?;

        let expires_at_unix_ms = claims.exp * 1000;
        Ok((token, expires_at_unix_ms))
    }

    /// Validate a WOPI access token and extract its claims.
    pub fn validate_token(&self, token: &str) -> Result<WopiTokenClaims, DomainError> {
        let validation = Validation::new(Algorithm::HS256);

        let token_data = decode::<WopiTokenClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => DomainError::new(
                ErrorKind::AccessDenied,
                "WopiTokenService",
                "WOPI token expired",
            ),
            _ => DomainError::new(
                ErrorKind::AccessDenied,
                "WopiTokenService",
                format!("Invalid WOPI token: {}", e),
            ),
        })?;

        let claims = token_data.claims;

        if claims.scope != "wopi" {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "WopiTokenService",
                "Token is not a WOPI token",
            ));
        }

        Ok(claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn service() -> WopiTokenService {
        WopiTokenService::new("test_secret_at_least_32_bytes_long!!".to_string(), 3600)
    }

    #[test]
    fn test_generate_and_validate() {
        let svc = service();
        let (token, ttl_ms) = svc
            .generate_token("file-123", "user-456", "test_user", true)
            .expect("Should generate token");

        let claims = svc.validate_token(&token).expect("Should validate");
        assert_eq!(claims.file_id, "file-123");
        assert_eq!(claims.sub, "user-456");
        assert!(claims.can_write);
        assert_eq!(claims.scope, "wopi");
        assert_eq!(claims.username, "test_user");

        // access_token_ttl must be absolute UNIX time in milliseconds.
        assert_eq!(ttl_ms, claims.exp * 1000);
        assert!(ttl_ms > claims.iat * 1000);
    }

    #[test]
    fn test_reject_invalid_token() {
        let svc = service();
        let result = svc.validate_token("garbage");
        assert!(result.is_err());
    }

    #[test]
    fn test_reject_wrong_secret() {
        let svc1 = service();
        let svc2 = WopiTokenService::new("different_secret_also_32_bytes!!".to_string(), 3600);

        let (token, _) = svc1
            .generate_token("file-1", "user-1", "test_user", false)
            .expect("Should generate");
        let result = svc2.validate_token(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_only_token() {
        let svc = service();
        let (token, _) = svc
            .generate_token("file-1", "user-1", "test_user", false)
            .expect("Should generate");
        let claims = svc.validate_token(&token).expect("Should validate");
        assert!(!claims.can_write);
    }
}
