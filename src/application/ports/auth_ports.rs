use async_trait::async_trait;
use crate::domain::entities::user::User;
use crate::domain::entities::session::Session;
use crate::common::errors::DomainError;

// ============================================================================
// Cryptography Ports - Extracted from Domain to maintain Clean Architecture
// ============================================================================

/// Port for password hashing operations.
/// 
/// This trait abstracts cryptographic password operations, allowing the domain
/// layer to remain independent of specific hashing implementations (argon2, bcrypt, etc.)
pub trait PasswordHasherPort: Send + Sync + 'static {
    /// Hash a plain text password
    fn hash_password(&self, password: &str) -> Result<String, DomainError>;
    
    /// Verify a plain text password against a hash
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, DomainError>;
}

/// Claims contained in a JWT token
#[derive(Debug, Clone)]
pub struct TokenClaims {
    /// Subject identifier (user ID)
    pub sub: String,
    /// Expiration timestamp (seconds since Unix epoch)
    pub exp: i64,
    /// Issued at timestamp (seconds since Unix epoch)
    pub iat: i64,
    /// JWT unique ID
    pub jti: String,
    /// Username
    pub username: String,
    /// User email
    pub email: String,
    /// User role
    pub role: String,
}

/// Port for JWT token operations.
/// 
/// This trait abstracts token generation and validation, allowing the domain
/// layer to remain independent of specific JWT implementations.
pub trait TokenServicePort: Send + Sync + 'static {
    /// Generate an access token for a user
    fn generate_access_token(&self, user: &User) -> Result<String, DomainError>;
    
    /// Validate a token and extract its claims
    fn validate_token(&self, token: &str) -> Result<TokenClaims, DomainError>;
    
    /// Generate a refresh token
    fn generate_refresh_token(&self) -> String;
    
    /// Get refresh token expiry in seconds
    fn refresh_token_expiry_secs(&self) -> i64;
    
    /// Get refresh token expiry in days
    fn refresh_token_expiry_days(&self) -> i64;
}

// ============================================================================
// Storage Ports
// ============================================================================

#[async_trait]
pub trait UserStoragePort: Send + Sync + 'static {
    /// Creates a new user 
    async fn create_user(&self, user: User) -> Result<User, DomainError>;
    
    /// Gets a user by ID
    async fn get_user_by_id(&self, id: &str) -> Result<User, DomainError>;
    
    /// Gets a user by username
    async fn get_user_by_username(&self, username: &str) -> Result<User, DomainError>;
    
    /// Gets a user by email
    async fn get_user_by_email(&self, email: &str) -> Result<User, DomainError>;
    
    /// Updates an existing user
    async fn update_user(&self, user: User) -> Result<User, DomainError>;
    
    /// Updates only the storage usage of a user
    async fn update_storage_usage(&self, user_id: &str, usage_bytes: i64) -> Result<(), DomainError>;
    
    /// Lists users with pagination
    async fn list_users(&self, limit: i64, offset: i64) -> Result<Vec<User>, DomainError>;
    
    /// Lists users by role (e.g., "admin" or "user")
    async fn list_users_by_role(&self, role: &str) -> Result<Vec<User>, DomainError>;
    
    /// Deletes a user by their ID
    async fn delete_user(&self, user_id: &str) -> Result<(), DomainError>;
    
    /// Changes a user's password
    async fn change_password(&self, user_id: &str, password_hash: &str) -> Result<(), DomainError>;

    /// Finds a user by OIDC provider + subject pair
    async fn get_user_by_oidc_subject(&self, provider: &str, subject: &str) -> Result<User, DomainError>;

    /// Activates or deactivates a user
    async fn set_user_active_status(&self, user_id: &str, active: bool) -> Result<(), DomainError>;

    /// Changes a user's role
    async fn change_role(&self, user_id: &str, role: &str) -> Result<(), DomainError>;

    /// Updates a user's storage quota
    async fn update_storage_quota(&self, user_id: &str, quota_bytes: i64) -> Result<(), DomainError>;

    /// Counts the total number of users
    async fn count_users(&self) -> Result<i64, DomainError>;
}

// ============================================================================
// OIDC Port
// ============================================================================

/// Represents the token set returned by the OIDC provider after code exchange
#[derive(Debug, Clone)]
pub struct OidcTokenSet {
    pub access_token: String,
    pub id_token: String,
    pub refresh_token: Option<String>,
}

/// Claims extracted from the validated OIDC ID token
#[derive(Debug, Clone)]
pub struct OidcIdClaims {
    pub sub: String,
    pub email: Option<String>,
    pub preferred_username: Option<String>,
    pub name: Option<String>,
    pub groups: Vec<String>,
}

/// Port for OIDC operations — implemented in infrastructure layer
#[async_trait]
pub trait OidcServicePort: Send + Sync + 'static {
    /// Get the authorization URL for redirecting the user to the IdP.
    /// Includes PKCE code_challenge (S256) and nonce for ID token binding.
    /// This is async because it may need to fetch the OIDC discovery document.
    async fn get_authorize_url(&self, state: &str, nonce: &str, pkce_challenge: &str) -> Result<String, DomainError>;

    /// Exchange an authorization code for tokens, providing PKCE code_verifier.
    async fn exchange_code(&self, code: &str, pkce_verifier: &str) -> Result<OidcTokenSet, DomainError>;

    /// Validate an ID token and extract claims.
    /// If `expected_nonce` is provided, verifies the `nonce` claim matches.
    async fn validate_id_token(&self, id_token: &str, expected_nonce: Option<&str>) -> Result<OidcIdClaims, DomainError>;

    /// Fetch user info from the UserInfo endpoint (fallback for missing ID token claims)
    async fn fetch_user_info(&self, access_token: &str) -> Result<OidcIdClaims, DomainError>;

    /// Get the OIDC provider display name
    fn provider_name(&self) -> &str;
}

#[async_trait]
pub trait SessionStoragePort: Send + Sync + 'static {
    /// Creates a new session
    async fn create_session(&self, session: Session) -> Result<Session, DomainError>;
    
    /// Gets a session by refresh token
    async fn get_session_by_refresh_token(&self, refresh_token: &str) -> Result<Session, DomainError>;
    
    /// Revokes a specific session
    async fn revoke_session(&self, session_id: &str) -> Result<(), DomainError>;
    
    /// Revokes all sessions of a user
    async fn revoke_all_user_sessions(&self, user_id: &str) -> Result<u64, DomainError>;
}