use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;
use crate::domain::entities::shared_file::PermissionLevel;

/// Error en la creación o manipulación de enlaces públicos
#[derive(Debug, Error)]
pub enum PublicLinkError {
    #[error("ID de enlace inválido: {0}")]
    InvalidId(String),
    
    #[error("Error en la validación: {0}")]
    ValidationError(String),
}

/// Tipo de resultado para operaciones con enlaces públicos
pub type PublicLinkResult<T> = Result<T, PublicLinkError>;

/// Representa un enlace público para compartir archivos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicLink {
    /// ID único para acceder al enlace público
    id: String,
    
    /// ID del archivo compartido
    file_id: String,
    
    /// ID del propietario del archivo
    owner_id: String,
    
    /// Nivel de permiso para el enlace
    permission_level: PermissionLevel,
    
    /// Hash de contraseña opcional (None si no hay contraseña)
    password_hash: Option<String>,
    
    /// Fecha de caducidad opcional (None si no caduca)
    expires_at: Option<DateTime<Utc>>,
    
    /// Contador de accesos
    access_count: i32,
    
    /// Fecha de creación
    created_at: DateTime<Utc>,
    
    /// Fecha de última actualización
    updated_at: DateTime<Utc>,
}

impl PublicLink {
    /// Crea un nuevo enlace público
    pub fn new(
        file_id: String,
        owner_id: String,
        permission_level: PermissionLevel,
        password_hash: Option<String>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Self {
        let now = Utc::now();
        
        // Generar un ID aleatorio para el enlace (sin guiones para que sea más compacto)
        let id = Uuid::new_v4().to_string().replace('-', "");
        
        Self {
            id,
            file_id,
            owner_id,
            permission_level,
            password_hash,
            expires_at,
            access_count: 0,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Crea un enlace público con un ID específico (para reconstrucción)
    pub fn with_id(
        id: String,
        file_id: String,
        owner_id: String,
        permission_level: PermissionLevel,
        password_hash: Option<String>,
        expires_at: Option<DateTime<Utc>>,
        access_count: i32,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            file_id,
            owner_id,
            permission_level,
            password_hash,
            expires_at,
            access_count,
            created_at,
            updated_at,
        }
    }
    
    // Getters
    pub fn id(&self) -> &str {
        &self.id
    }
    
    pub fn file_id(&self) -> &str {
        &self.file_id
    }
    
    pub fn owner_id(&self) -> &str {
        &self.owner_id
    }
    
    pub fn permission_level(&self) -> &PermissionLevel {
        &self.permission_level
    }
    
    pub fn password_hash(&self) -> &Option<String> {
        &self.password_hash
    }
    
    pub fn expires_at(&self) -> &Option<DateTime<Utc>> {
        &self.expires_at
    }
    
    pub fn access_count(&self) -> i32 {
        self.access_count
    }
    
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
    
    /// Comprueba si el enlace ha caducado
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires_at {
            expires < Utc::now()
        } else {
            false
        }
    }
    
    /// Comprueba si el enlace está protegido con contraseña
    pub fn has_password(&self) -> bool {
        self.password_hash.is_some()
    }
    
    // Métodos para crear nuevas versiones inmutables
    
    /// Crea una nueva versión con permiso actualizado
    pub fn with_permission(&self, permission_level: PermissionLevel) -> Self {
        Self {
            id: self.id.clone(),
            file_id: self.file_id.clone(),
            owner_id: self.owner_id.clone(),
            permission_level,
            password_hash: self.password_hash.clone(),
            expires_at: self.expires_at,
            access_count: self.access_count,
            created_at: self.created_at,
            updated_at: Utc::now(),
        }
    }
    
    /// Crea una nueva versión con contraseña actualizada
    pub fn with_password(&self, password_hash: Option<String>) -> Self {
        Self {
            id: self.id.clone(),
            file_id: self.file_id.clone(),
            owner_id: self.owner_id.clone(),
            permission_level: self.permission_level.clone(),
            password_hash,
            expires_at: self.expires_at,
            access_count: self.access_count,
            created_at: self.created_at,
            updated_at: Utc::now(),
        }
    }
    
    /// Crea una nueva versión con fecha de caducidad actualizada
    pub fn with_expiration(&self, expires_at: Option<DateTime<Utc>>) -> Self {
        Self {
            id: self.id.clone(),
            file_id: self.file_id.clone(),
            owner_id: self.owner_id.clone(),
            permission_level: self.permission_level.clone(),
            password_hash: self.password_hash.clone(),
            expires_at,
            access_count: self.access_count,
            created_at: self.created_at,
            updated_at: Utc::now(),
        }
    }
    
    /// Crea una nueva versión con contador de accesos incrementado
    pub fn increment_access_count(&self) -> Self {
        Self {
            id: self.id.clone(),
            file_id: self.file_id.clone(),
            owner_id: self.owner_id.clone(),
            permission_level: self.permission_level.clone(),
            password_hash: self.password_hash.clone(),
            expires_at: self.expires_at,
            access_count: self.access_count + 1,
            created_at: self.created_at,
            updated_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    
    #[test]
    fn test_create_public_link() {
        let link = PublicLink::new(
            "file-123".to_string(),
            "owner-456".to_string(),
            PermissionLevel::Read,
            None,
            None,
        );
        
        assert_eq!(link.file_id(), "file-123");
        assert_eq!(link.owner_id(), "owner-456");
        assert!(matches!(link.permission_level(), PermissionLevel::Read));
        assert!(!link.has_password());
        assert!(!link.is_expired());
        assert_eq!(link.access_count(), 0);
    }
    
    #[test]
    fn test_link_with_password() {
        let link = PublicLink::new(
            "file-123".to_string(),
            "owner-456".to_string(),
            PermissionLevel::Read,
            Some("hashed_password".to_string()),
            None,
        );
        
        assert!(link.has_password());
        assert_eq!(link.password_hash(), &Some("hashed_password".to_string()));
    }
    
    #[test]
    fn test_link_expiration() {
        // Enlace expirado
        let expired_date = Utc::now() - Duration::days(1);
        let expired_link = PublicLink::new(
            "file-123".to_string(),
            "owner-456".to_string(),
            PermissionLevel::Read,
            None,
            Some(expired_date),
        );
        
        assert!(expired_link.is_expired());
        
        // Enlace vigente
        let future_date = Utc::now() + Duration::days(1);
        let valid_link = PublicLink::new(
            "file-123".to_string(),
            "owner-456".to_string(),
            PermissionLevel::Read,
            None,
            Some(future_date),
        );
        
        assert!(!valid_link.is_expired());
    }
    
    #[test]
    fn test_increment_access_count() {
        let link = PublicLink::new(
            "file-123".to_string(),
            "owner-456".to_string(),
            PermissionLevel::Read,
            None,
            None,
        );
        
        let updated = link.increment_access_count();
        assert_eq!(updated.access_count(), 1);
        
        let updated_again = updated.increment_access_count();
        assert_eq!(updated_again.access_count(), 2);
    }
}