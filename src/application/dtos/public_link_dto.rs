use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::domain::entities::public_link::PublicLink;
use crate::domain::entities::shared_file::PermissionLevel;

/// DTO para las solicitudes de creación de enlaces públicos
#[derive(Debug, Deserialize)]
pub struct CreatePublicLinkDto {
    /// ID del archivo a compartir
    pub file_id: String,
    
    /// Nivel de permiso a otorgar
    pub permission: String,
    
    /// Contraseña opcional para proteger el enlace
    pub password: Option<String>,
    
    /// Fecha de caducidad opcional
    pub expires_at: Option<DateTime<Utc>>,
}

/// DTO para las respuestas de enlaces públicos
#[derive(Debug, Serialize)]
pub struct PublicLinkDto {
    /// ID único del enlace
    pub id: String,
    
    /// ID del archivo compartido
    pub file_id: String,
    
    /// Nombre del archivo (opcional, se rellena si se dispone)
    pub file_name: Option<String>,
    
    /// ID del propietario del enlace
    pub owner_id: String,
    
    /// Nivel de permiso otorgado
    pub permission: String,
    
    /// Indicador de si tiene contraseña
    pub has_password: bool,
    
    /// Fecha de caducidad (si existe)
    pub expires_at: Option<DateTime<Utc>>,
    
    /// Número de accesos al enlace
    pub access_count: i32,
    
    /// ¿Ha caducado?
    pub is_expired: bool,
    
    /// Fecha de creación
    pub created_at: DateTime<Utc>,
    
    /// URL completa para acceder al enlace público
    pub share_url: Option<String>,
}

impl From<PublicLink> for PublicLinkDto {
    fn from(link: PublicLink) -> Self {
        Self {
            id: link.id().to_string(),
            file_id: link.file_id().to_string(),
            file_name: None, // Se rellenará posteriormente si es necesario
            owner_id: link.owner_id().to_string(),
            permission: link.permission_level().to_string(),
            has_password: link.has_password(),
            expires_at: *link.expires_at(),
            access_count: link.access_count(),
            is_expired: link.is_expired(),
            created_at: *link.created_at(),
            share_url: None, // Se rellenará posteriormente
        }
    }
}

/// DTO para actualizar la contraseña de un enlace
#[derive(Debug, Deserialize)]
pub struct UpdatePasswordDto {
    /// Nueva contraseña (None para quitar la contraseña)
    pub password: Option<String>,
}

/// DTO para actualizar los permisos de un enlace
#[derive(Debug, Deserialize)]
pub struct UpdatePermissionDto {
    /// Nuevo nivel de permiso
    pub permission: String,
}

impl UpdatePermissionDto {
    /// Convierte el string de permiso en un PermissionLevel
    pub fn to_permission_level(&self) -> Result<PermissionLevel, String> {
        PermissionLevel::from_string(&self.permission)
            .map_err(|e| e.to_string())
    }
}

/// DTO para actualizar la fecha de caducidad
#[derive(Debug, Deserialize)]
pub struct UpdateExpirationDto {
    /// Nueva fecha de caducidad (None para no expirar)
    pub expires_at: Option<DateTime<Utc>>,
}

/// DTO para verificar contraseña de un enlace
#[derive(Debug, Deserialize)]
pub struct VerifyPasswordDto {
    /// Contraseña a verificar
    pub password: String,
}

/// DTO para la respuesta de un acceso público a un archivo
#[derive(Debug, Serialize)]
pub struct PublicFileAccessDto {
    /// ID del enlace público
    pub link_id: String,
    
    /// Información del archivo
    pub file: PublicFileInfoDto,
    
    /// Requiere contraseña
    pub requires_password: bool,
    
    /// Nivel de permiso
    pub permission: String,
    
    /// Fecha de caducidad
    pub expires_at: Option<DateTime<Utc>>,
}

/// DTO para la información pública de un archivo
#[derive(Debug, Serialize)]
pub struct PublicFileInfoDto {
    /// ID del archivo
    pub id: String,
    
    /// Nombre del archivo
    pub name: String,
    
    /// Tamaño en bytes
    pub size: u64,
    
    /// Tipo MIME
    pub mime_type: String,
}