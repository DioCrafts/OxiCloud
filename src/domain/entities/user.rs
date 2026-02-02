use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("Username inválido: {0}")]
    InvalidUsername(String),
    
    #[error("Password inválido: {0}")]
    InvalidPassword(String),
    
    #[error("Error en la validación: {0}")]
    ValidationError(String),
    
    #[error("Error en la autenticación: {0}")]
    AuthenticationError(String),
}

pub type UserResult<T> = Result<T, UserError>;

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
        // Validaciones
        if username.is_empty() || username.len() < 3 || username.len() > 32 {
            return Err(UserError::InvalidUsername(format!(
                "Username debe tener entre 3 y 32 caracteres"
            )));
        }
        
        if !email.contains('@') || email.len() < 5 {
            return Err(UserError::ValidationError(format!(
                "Email inválido"
            )));
        }
        
        if password_hash.is_empty() {
            return Err(UserError::InvalidPassword(format!(
                "Password hash no puede estar vacío"
            )));
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
        })
    }
    
    // Crear desde valores existentes (para reconstrucción desde BD)
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
    
    /// Update the password hash.
    /// 
    /// The new password should be hashed externally using PasswordHasherPort
    /// before calling this method.
    pub fn update_password_hash(&mut self, new_hash: String) {
        self.password_hash = new_hash;
        self.updated_at = Utc::now();
    }
    
    // Actualizar uso de almacenamiento
    pub fn update_storage_used(&mut self, storage_used_bytes: i64) {
        self.storage_used_bytes = storage_used_bytes;
        self.updated_at = Utc::now();
    }
    
    // Registrar login
    pub fn register_login(&mut self) {
        let now = Utc::now();
        self.last_login_at = Some(now);
        self.updated_at = now;
    }
    
    // Desactivar usuario
    pub fn deactivate(&mut self) {
        self.active = false;
        self.updated_at = Utc::now();
    }
    
    // Activar usuario
    pub fn activate(&mut self) {
        self.active = true;
        self.updated_at = Utc::now();
    }
}