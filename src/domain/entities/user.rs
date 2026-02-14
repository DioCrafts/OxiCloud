use uuid::Uuid;
use chrono::{DateTime, Utc};

// Re-export entity errors from the centralized module
pub use super::entity_errors::{UserError, UserResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// We'll handle conversion manually for now until the type is properly set up in the database
pub enum UserRole {
    Admin,
    User,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "admin"),
            UserRole::User => write!(f, "user"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    id: String,
    username: String, 
    email: String,
    password_hash: String,
    role: UserRole,
    storage_quota_bytes: i64,
    storage_used_bytes: i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    last_login_at: Option<DateTime<Utc>>,
    active: bool,
    oidc_provider: Option<String>,
    oidc_subject: Option<String>,
}

impl User {
    /// Create a new user with a pre-hashed password.
    /// 
    /// The password hashing should be done externally using PasswordHasherPort
    /// to maintain clean architecture and keep cryptographic dependencies
    /// out of the domain layer.
    /// 
    /// # Arguments
    /// * `username` - User's username (3-32 characters)
    /// * `email` - User's email address
    /// * `password_hash` - Pre-hashed password (from PasswordHasherPort)
    /// * `role` - User's role
    /// * `storage_quota_bytes` - Storage quota in bytes
    pub fn new(
        username: String,
        email: String, 
        password_hash: String,
        role: UserRole,
        storage_quota_bytes: i64,
    ) -> UserResult<Self> {
        // Validations
        if username.is_empty() || username.len() < 3 || username.len() > 32 {
            return Err(UserError::InvalidUsername("Username must be between 3 and 32 characters".to_string()));
        }
        
        if !email.contains('@') || email.len() < 5 {
            return Err(UserError::ValidationError("Invalid email".to_string()));
        }
        
        if password_hash.is_empty() {
            return Err(UserError::InvalidPassword("Password hash cannot be empty".to_string()));
        }
        
        let now = Utc::now();
        
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            username,
            email,
            password_hash,
            role,
            storage_quota_bytes,
            storage_used_bytes: 0,
            created_at: now,
            updated_at: now,
            last_login_at: None,
            active: true,
            oidc_provider: None,
            oidc_subject: None,
        })
    }
    
    /// Create a new OIDC-authenticated user (no password required).
    pub fn new_oidc(
        username: String,
        email: String,
        role: UserRole,
        storage_quota_bytes: i64,
        oidc_provider: String,
        oidc_subject: String,
    ) -> UserResult<Self> {
        if username.is_empty() || username.len() < 3 || username.len() > 32 {
            return Err(UserError::InvalidUsername(
                "Username must be between 3 and 32 characters".to_string(),
            ));
        }
        if !email.contains('@') || email.len() < 5 {
            return Err(UserError::ValidationError(
                "Invalid email".to_string(),
            ));
        }
        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            username,
            email,
            password_hash: "__OIDC_NO_PASSWORD__".to_string(),
            role,
            storage_quota_bytes,
            storage_used_bytes: 0,
            created_at: now,
            updated_at: now,
            last_login_at: None,
            active: true,
            oidc_provider: Some(oidc_provider),
            oidc_subject: Some(oidc_subject),
        })
    }
    
    // Create from existing values (for reconstruction from DB)
    pub fn from_data(
        id: String,
        username: String,
        email: String,
        password_hash: String,
        role: UserRole,
        storage_quota_bytes: i64,
        storage_used_bytes: i64,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        last_login_at: Option<DateTime<Utc>>,
        active: bool,
    ) -> Self {
        Self {
            id,
            username,
            email,
            password_hash,
            role,
            storage_quota_bytes,
            storage_used_bytes,
            created_at,
            updated_at,
            last_login_at,
            active,
            oidc_provider: None,
            oidc_subject: None,
        }
    }

    /// Reconstruct from DB with OIDC fields
    pub fn from_data_full(
        id: String,
        username: String,
        email: String,
        password_hash: String,
        role: UserRole,
        storage_quota_bytes: i64,
        storage_used_bytes: i64,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        last_login_at: Option<DateTime<Utc>>,
        active: bool,
        oidc_provider: Option<String>,
        oidc_subject: Option<String>,
    ) -> Self {
        Self {
            id,
            username,
            email,
            password_hash,
            role,
            storage_quota_bytes,
            storage_used_bytes,
            created_at,
            updated_at,
            last_login_at,
            active,
            oidc_provider,
            oidc_subject,
        }
    }
    
    // Getters
    pub fn id(&self) -> &str {
        &self.id
    }
    
    pub fn username(&self) -> &str {
        &self.username
    }
    
    pub fn email(&self) -> &str {
        &self.email
    }
    
    pub fn role(&self) -> UserRole {
        self.role
    }
    
    pub fn storage_quota_bytes(&self) -> i64 {
        self.storage_quota_bytes
    }
    
    pub fn storage_used_bytes(&self) -> i64 {
        self.storage_used_bytes
    }
    
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
    
    pub fn last_login_at(&self) -> Option<DateTime<Utc>> {
        self.last_login_at
    }
    
    pub fn is_active(&self) -> bool {
        self.active
    }
    
    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }

    pub fn oidc_provider(&self) -> Option<&str> {
        self.oidc_provider.as_deref()
    }

    pub fn oidc_subject(&self) -> Option<&str> {
        self.oidc_subject.as_deref()
    }

    /// Returns true if this is an OIDC-only user (no password)
    pub fn is_oidc_user(&self) -> bool {
        self.oidc_provider.is_some()
    }
    
    /// Update the password hash.
    /// 
    /// The new password should be hashed externally using PasswordHasherPort
    /// before calling this method.
    pub fn update_password_hash(&mut self, new_hash: String) {
        self.password_hash = new_hash;
        self.updated_at = Utc::now();
    }
    
    // Update storage usage
    pub fn update_storage_used(&mut self, storage_used_bytes: i64) {
        self.storage_used_bytes = storage_used_bytes;
        self.updated_at = Utc::now();
    }
    
    // Register login
    pub fn register_login(&mut self) {
        let now = Utc::now();
        self.last_login_at = Some(now);
        self.updated_at = now;
    }
    
    // Deactivate user
    pub fn deactivate(&mut self) {
        self.active = false;
        self.updated_at = Utc::now();
    }
    
    // Activate user
    pub fn activate(&mut self) {
        self.active = true;
        self.updated_at = Utc::now();
    }
}