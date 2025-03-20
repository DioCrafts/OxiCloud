use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use thiserror::Error;

/// Error en la creación o manipulación de permisos compartidos
#[derive(Debug, Error)]
pub enum SharedFileError {
    #[error("Nivel de permiso inválido: {0}")]
    InvalidPermission(String),
    
    #[error("Error en la validación: {0}")]
    ValidationError(String),
}

/// Tipo de resultado para operaciones con permisos compartidos
pub type SharedFileResult<T> = Result<T, SharedFileError>;

/// Niveles de permiso para compartir archivos
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PermissionLevel {
    /// Solo lectura - puede ver y descargar el archivo
    Read,
    
    /// Lectura y escritura - puede modificar pero no eliminar
    Write,
    
    /// Administrador - puede modificar y eliminar
    Admin,
}

impl PermissionLevel {
    /// Convierte una cadena en un nivel de permiso
    pub fn from_string(s: &str) -> SharedFileResult<Self> {
        match s.to_lowercase().as_str() {
            "read" => Ok(PermissionLevel::Read),
            "write" => Ok(PermissionLevel::Write),
            "admin" => Ok(PermissionLevel::Admin),
            _ => Err(SharedFileError::InvalidPermission(s.to_string())),
        }
    }
    
    /// Convierte un nivel de permiso en una cadena
    pub fn to_string(&self) -> String {
        match self {
            PermissionLevel::Read => "read".to_string(),
            PermissionLevel::Write => "write".to_string(),
            PermissionLevel::Admin => "admin".to_string(),
        }
    }
    
    /// Verifica si el permiso permite lectura
    pub fn can_read(&self) -> bool {
        // Todos los niveles permiten lectura
        true
    }
    
    /// Verifica si el permiso permite escritura
    pub fn can_write(&self) -> bool {
        matches!(self, PermissionLevel::Write | PermissionLevel::Admin)
    }
    
    /// Verifica si el permiso permite eliminar
    pub fn can_delete(&self) -> bool {
        matches!(self, PermissionLevel::Admin)
    }
}

/// Representa un permiso compartido en el dominio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedFile {
    /// ID único del permiso compartido
    id: Option<i32>,
    
    /// ID del archivo compartido
    file_id: String,
    
    /// ID del propietario del archivo
    owner_id: String,
    
    /// ID del usuario con quien se comparte
    user_id: String,
    
    /// Nivel de permiso otorgado
    permission_level: PermissionLevel,
    
    /// Fecha de creación
    created_at: DateTime<Utc>,
    
    /// Fecha de última actualización
    updated_at: DateTime<Utc>,
}

impl SharedFile {
    /// Crea un nuevo permiso compartido
    pub fn new(
        file_id: String,
        owner_id: String,
        user_id: String,
        permission_level: PermissionLevel,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            id: None,
            file_id,
            owner_id,
            user_id,
            permission_level,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Crea un permiso compartido con ID existente (para reconstrucción)
    pub fn with_id(
        id: i32,
        file_id: String,
        owner_id: String,
        user_id: String,
        permission_level: PermissionLevel,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: Some(id),
            file_id,
            owner_id,
            user_id,
            permission_level,
            created_at,
            updated_at,
        }
    }
    
    // Getters
    pub fn id(&self) -> Option<i32> {
        self.id
    }
    
    pub fn file_id(&self) -> &str {
        &self.file_id
    }
    
    pub fn owner_id(&self) -> &str {
        &self.owner_id
    }
    
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
    
    pub fn permission_level(&self) -> &PermissionLevel {
        &self.permission_level
    }
    
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
    
    // Métodos para crear nuevas versiones inmutables
    
    /// Crea una nueva versión con permiso actualizado
    pub fn with_permission(&self, permission_level: PermissionLevel) -> Self {
        Self {
            id: self.id,
            file_id: self.file_id.clone(),
            owner_id: self.owner_id.clone(),
            user_id: self.user_id.clone(),
            permission_level,
            created_at: self.created_at,
            updated_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_permission_level_from_string() {
        assert!(matches!(PermissionLevel::from_string("read").unwrap(), PermissionLevel::Read));
        assert!(matches!(PermissionLevel::from_string("write").unwrap(), PermissionLevel::Write));
        assert!(matches!(PermissionLevel::from_string("admin").unwrap(), PermissionLevel::Admin));
        assert!(PermissionLevel::from_string("invalid").is_err());
    }
    
    #[test]
    fn test_permission_capabilities() {
        let read = PermissionLevel::Read;
        let write = PermissionLevel::Write;
        let admin = PermissionLevel::Admin;
        
        assert!(read.can_read());
        assert!(!read.can_write());
        assert!(!read.can_delete());
        
        assert!(write.can_read());
        assert!(write.can_write());
        assert!(!write.can_delete());
        
        assert!(admin.can_read());
        assert!(admin.can_write());
        assert!(admin.can_delete());
    }
    
    #[test]
    fn test_create_shared_file() {
        let shared = SharedFile::new(
            "file-123".to_string(),
            "owner-456".to_string(),
            "user-789".to_string(),
            PermissionLevel::Read,
        );
        
        assert_eq!(shared.file_id(), "file-123");
        assert_eq!(shared.owner_id(), "owner-456");
        assert_eq!(shared.user_id(), "user-789");
        assert!(matches!(shared.permission_level(), PermissionLevel::Read));
        assert!(shared.id().is_none());
    }
    
    #[test]
    fn test_with_permission() {
        let shared = SharedFile::new(
            "file-123".to_string(),
            "owner-456".to_string(),
            "user-789".to_string(),
            PermissionLevel::Read,
        );
        
        let updated = shared.with_permission(PermissionLevel::Write);
        
        assert_eq!(updated.file_id(), "file-123");
        assert_eq!(updated.owner_id(), "owner-456");
        assert_eq!(updated.user_id(), "user-789");
        assert!(matches!(updated.permission_level(), PermissionLevel::Write));
    }
}