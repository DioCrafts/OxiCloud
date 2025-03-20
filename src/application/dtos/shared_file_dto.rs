use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::domain::entities::shared_file::{SharedFile, PermissionLevel};

/// DTO para las solicitudes de compartir archivos
#[derive(Debug, Deserialize)]
pub struct ShareFileRequestDto {
    /// ID del archivo a compartir
    pub file_id: String,
    
    /// ID del usuario con quien compartir
    pub user_id: String,
    
    /// Nivel de permiso a otorgar
    pub permission: String,
}

/// DTO para las respuestas de archivos compartidos
#[derive(Debug, Serialize)]
pub struct SharedFileDto {
    /// ID del permiso compartido
    pub id: Option<i32>,
    
    /// ID del archivo compartido
    pub file_id: String,
    
    /// ID del propietario del archivo
    pub owner_id: String,
    
    /// ID del usuario con quien se comparte
    pub user_id: String,
    
    /// Nombre del usuario con quien se comparte
    pub user_name: Option<String>,
    
    /// Nivel de permiso otorgado
    pub permission: String,
    
    /// Fecha de creación
    pub created_at: DateTime<Utc>,
    
    /// Fecha de última actualización
    pub updated_at: DateTime<Utc>,
}

impl From<SharedFile> for SharedFileDto {
    fn from(shared: SharedFile) -> Self {
        Self {
            id: shared.id(),
            file_id: shared.file_id().to_string(),
            owner_id: shared.owner_id().to_string(),
            user_id: shared.user_id().to_string(),
            user_name: None, // Se rellenará posteriormente si es necesario
            permission: shared.permission_level().to_string(),
            created_at: *shared.created_at(),
            updated_at: *shared.updated_at(),
        }
    }
}

/// DTO para actualizar un permiso de compartición
#[derive(Debug, Deserialize)]
pub struct UpdatePermissionDto {
    /// Nuevo nivel de permiso
    pub permission: String,
    
    /// ID del usuario con quien se comparte
    pub user_id: String,
}

impl UpdatePermissionDto {
    /// Convierte el string de permiso en un PermissionLevel
    pub fn to_permission_level(&self) -> Result<PermissionLevel, String> {
        PermissionLevel::from_string(&self.permission)
            .map_err(|e| e.to_string())
    }
}

/// DTO para la respuesta de usuarios con acceso a un archivo
#[derive(Debug, Serialize)]
pub struct FileAccessDto {
    /// ID del archivo
    pub file_id: String,
    
    /// Nombre del archivo
    pub file_name: String,
    
    /// Lista de usuarios con acceso
    pub users: Vec<UserAccessDto>,
}

/// DTO para los detalles de acceso de un usuario
#[derive(Debug, Serialize)]
pub struct UserAccessDto {
    /// ID del usuario
    pub user_id: String,
    
    /// Nombre de usuario
    pub username: String,
    
    /// Nivel de permiso
    pub permission: String,
    
    /// Fecha en que se compartió
    pub shared_at: DateTime<Utc>,
}

impl From<SharedFile> for UserAccessDto {
    fn from(shared: SharedFile) -> Self {
        Self {
            user_id: shared.user_id().to_string(),
            username: "Unknown".to_string(), // We don't have the username here
            permission: shared.permission_level().to_string(),
            shared_at: *shared.created_at(),
        }
    }
}