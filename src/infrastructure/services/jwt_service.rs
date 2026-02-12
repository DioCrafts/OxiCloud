//! JWT-based token service implementation.
//! 
//! This module provides JWT token generation and validation functionality,
//! implementing the TokenServicePort trait defined in the application layer.

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::Utc;

use crate::application::ports::auth_ports::{TokenServicePort, TokenClaims};
use crate::domain::entities::user::User;
use crate::common::errors::{DomainError, ErrorKind};

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
pub struct JwtTokenService {
    /// Secret key used for signing JWT tokens
    jwt_secret: String,
    /// Expiration time for access tokens in seconds
    access_token_expiry: i64,
    /// Expiration time for refresh tokens in seconds
    refresh_token_expiry: i64,
}

impl JwtTokenService {
    /// Create a new JwtTokenService with the specified configuration.
    /// 
    /// # Arguments
    /// * `jwt_secret` - Secret key for signing tokens (should be at least 32 bytes)
    /// * `access_token_expiry_secs` - Lifetime of access tokens in seconds
    /// * `refresh_token_expiry_secs` - Lifetime of refresh tokens in seconds
    pub fn new(jwt_secret: String, access_token_expiry_secs: i64, refresh_token_expiry_secs: i64) -> Self {
        Self {
            jwt_secret,
            access_token_expiry: access_token_expiry_secs,
            refresh_token_expiry: refresh_token_expiry_secs,
        }
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
        tracing::debug!("JWT claims: sub={}, exp={}, iat={}", claims.sub, claims.exp, claims.iat);
        
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes())
        )
        .map_err(|e| {
            tracing::error!("Error generating token: {}", e);
            DomainError::new(
                ErrorKind::InternalError,
                "TokenService",
                format!("Error generating token: {}", e)
            )
        })
    }
    
    fn validate_token(&self, token: &str) -> Result<TokenClaims, DomainError> {
        let validation = Validation::new(Algorithm::HS256);
        
        let token_data = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation
        )
        .map_err(|e| {
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    DomainError::new(ErrorKind::AccessDenied, "TokenService", "Token expired")
                },
                _ => DomainError::new(
                    ErrorKind::AccessDenied,
                    "TokenService",
                    format!("Invalid token: {}", e)
                ),
            }
        })?;
        
        Ok(token_data.claims.into())
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
        let token = service.generate_access_token(&user).expect("Should generate token");
        
        let claims = service.validate_token(&token).expect("Should validate token");
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
}
