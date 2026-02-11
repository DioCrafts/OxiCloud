use async_trait::async_trait;
use crate::domain::entities::user::{User, UserRole};
use crate::common::errors::DomainError;

#[derive(Debug, thiserror::Error)]
pub enum UserRepositoryError {
    #[error("Usuario no encontrado: {0}")]
    NotFound(String),
    
    #[error("Usuario ya existe: {0}")]
    AlreadyExists(String),
    
    #[error("Error de base de datos: {0}")]
    DatabaseError(String),
    
    #[error("Error de validación: {0}")]
    ValidationError(String),
    
    #[error("Error de tiempo de espera: {0}")]
    Timeout(String),
    
    #[error("Operación no permitida: {0}")]
    OperationNotAllowed(String),
}

pub type UserRepositoryResult<T> = Result<T, UserRepositoryError>;

// Conversión de UserRepositoryError a DomainError
impl From<UserRepositoryError> for DomainError {
    fn from(err: UserRepositoryError) -> Self {
        match err {
            UserRepositoryError::NotFound(msg) => {
                DomainError::not_found("User", msg)
            },
            UserRepositoryError::AlreadyExists(msg) => {
                DomainError::already_exists("User", msg)
            },
            UserRepositoryError::DatabaseError(msg) => {
                DomainError::internal_error("Database", msg)
            },
            UserRepositoryError::ValidationError(msg) => {
                DomainError::validation_error(msg)
            },
            UserRepositoryError::Timeout(msg) => {
                DomainError::timeout("Database", msg)
            },
            UserRepositoryError::OperationNotAllowed(msg) => {
                DomainError::access_denied("User", msg)
            },
        }
    }
}

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    /// Crea un nuevo usuario
    async fn create_user(&self, user: User) -> UserRepositoryResult<User>;
    
    /// Obtiene un usuario por ID
    async fn get_user_by_id(&self, id: &str) -> UserRepositoryResult<User>;
    
    /// Obtiene un usuario por nombre de usuario
    async fn get_user_by_username(&self, username: &str) -> UserRepositoryResult<User>;
    
    /// Obtiene un usuario por correo electrónico
    async fn get_user_by_email(&self, email: &str) -> UserRepositoryResult<User>;
    
    /// Actualiza un usuario existente
    async fn update_user(&self, user: User) -> UserRepositoryResult<User>;
    
    /// Actualiza solo el uso de almacenamiento de un usuario
    async fn update_storage_usage(&self, user_id: &str, usage_bytes: i64) -> UserRepositoryResult<()>;
    
    /// Actualiza la fecha de último inicio de sesión
    async fn update_last_login(&self, user_id: &str) -> UserRepositoryResult<()>;
    
    /// Lista usuarios con paginación
    async fn list_users(&self, limit: i64, offset: i64) -> UserRepositoryResult<Vec<User>>;
    
    /// Activa o desactiva un usuario
    async fn set_user_active_status(&self, user_id: &str, active: bool) -> UserRepositoryResult<()>;
    
    /// Cambia la contraseña de un usuario
    async fn change_password(&self, user_id: &str, password_hash: &str) -> UserRepositoryResult<()>;
    
    /// Cambia el rol de un usuario
    async fn change_role(&self, user_id: &str, role: UserRole) -> UserRepositoryResult<()>;
    
    /// Lista usuarios por rol (admin o user)
    async fn list_users_by_role(&self, role: &str) -> UserRepositoryResult<Vec<User>>;
    
    /// Elimina un usuario
    async fn delete_user(&self, user_id: &str) -> UserRepositoryResult<()>;

    /// Finds a user by OIDC provider + subject pair
    async fn get_user_by_oidc_subject(&self, provider: &str, subject: &str) -> UserRepositoryResult<User>;

    /// Actualiza la cuota de almacenamiento de un usuario
    async fn update_storage_quota(&self, user_id: &str, quota_bytes: i64) -> UserRepositoryResult<()>;

    /// Cuenta el número total de usuarios
    async fn count_users(&self) -> UserRepositoryResult<i64>;

    /// Obtiene estadísticas de almacenamiento agregadas
    async fn get_storage_stats(&self) -> UserRepositoryResult<StorageStats>;
}

/// Estadísticas de almacenamiento agregadas
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub total_users: i64,
    pub active_users: i64,
    pub total_quota_bytes: i64,
    pub total_used_bytes: i64,
    pub users_over_80_percent: i64,
    pub users_over_quota: i64,
}