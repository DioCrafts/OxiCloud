use serde::{Serialize, Deserialize};
use crate::domain::services::path_service::StoragePath;

/// Error en la creación o manipulación de entidades de archivo
#[derive(Debug, thiserror::Error)]
pub enum FileError {
    #[error("Nombre de archivo inválido: {0}")]
    InvalidFileName(String),
    
    #[error("Error en la validación: {0}")]
    #[allow(dead_code)]
    ValidationError(String),
}

/// Tipo de resultado para operaciones con entidades de archivo
pub type FileResult<T> = Result<T, FileError>;

/// Represents a file entity in the domain
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct File {
    /// Unique identifier for the file
    id: String,
    
    /// Name of the file
    name: String,
    
    /// Path to the file in the domain model
    #[serde(skip_serializing, skip_deserializing)]
    storage_path: StoragePath,
    
    /// String representation of the path (for serialization compatibility)
    #[serde(rename = "path")]
    path_string: String,
    
    /// Size of the file in bytes
    size: u64,
    
    /// MIME type of the file
    mime_type: String,
    
    /// Parent folder ID
    folder_id: Option<String>,
    
    /// Owner user ID (None for system files)
    user_id: Option<String>,
    
    /// Creation timestamp
    created_at: u64,
    
    /// Last modification timestamp
    modified_at: u64,
}

// Ya no necesitamos este módulo, ahora usamos un String directamente

impl Default for File {
    fn default() -> Self {
        Self {
            id: "stub-id".to_string(),
            name: "stub-file.txt".to_string(),
            storage_path: StoragePath::from_string("/"),
            path_string: "/".to_string(),
            size: 0,
            mime_type: "application/octet-stream".to_string(),
            folder_id: None,
            user_id: None,
            created_at: 0,
            modified_at: 0,
        }
    }
}

impl File {
    /// Crea un nuevo archivo con validación
    pub fn new(
        id: String,
        name: String,
        storage_path: StoragePath,
        size: u64,
        mime_type: String,
        folder_id: Option<String>,
        user_id: Option<String>,
    ) -> FileResult<Self> {
        // Validar nombre de archivo
        if name.is_empty() || name.contains('/') || name.contains('\\') {
            return Err(FileError::InvalidFileName(name));
        }
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Almacenamos el string de la ruta para compatibilidad con serialización
        let path_string = storage_path.to_string();
            
        Ok(Self {
            id,
            name,
            storage_path,
            path_string,
            size,
            mime_type,
            folder_id,
            user_id,
            created_at: now,
            modified_at: now,
        })
    }
    
    /// Crea un archivo con timestamps específicos (para reconstrucción)
    pub fn with_timestamps(
        id: String,
        name: String,
        storage_path: StoragePath,
        size: u64,
        mime_type: String,
        folder_id: Option<String>,
        user_id: Option<String>,
        created_at: u64,
        modified_at: u64,
    ) -> FileResult<Self> {
        // Validar nombre de archivo
        if name.is_empty() || name.contains('/') || name.contains('\\') {
            return Err(FileError::InvalidFileName(name));
        }
        
        // Almacenamos el string de la ruta para compatibilidad con serialización
        let path_string = storage_path.to_string();
            
        Ok(Self {
            id,
            name,
            storage_path,
            path_string,
            size,
            mime_type,
            folder_id,
            user_id,
            created_at,
            modified_at,
        })
    }
    
    // Getters
    pub fn id(&self) -> &str {
        &self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn storage_path(&self) -> &StoragePath {
        &self.storage_path
    }
    
    pub fn path_string(&self) -> &str {
        &self.path_string
    }
    
    pub fn size(&self) -> u64 {
        self.size
    }
    
    pub fn mime_type(&self) -> &str {
        &self.mime_type
    }
    
    pub fn folder_id(&self) -> Option<&str> {
        self.folder_id.as_deref()
    }
    
    pub fn user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }
    
    pub fn created_at(&self) -> u64 {
        self.created_at
    }
    
    pub fn modified_at(&self) -> u64 {
        self.modified_at
    }
    
    /// Crea una nueva instancia de File desde un DTO
    /// Esta función es principalmente para conversiones en los batch handlers
    pub fn from_dto(
        id: String,
        name: String,
        path: String,
        size: u64,
        mime_type: String,
        folder_id: Option<String>,
        user_id: Option<String>,
        created_at: u64,
        modified_at: u64,
    ) -> Self {
        // Crear storage_path desde el string
        let storage_path = StoragePath::from_string(&path);
        
        // Crear directamente sin validación para evitar errores en conversiones DTO
        Self {
            id,
            name,
            storage_path,
            path_string: path,
            size,
            mime_type,
            folder_id,
            user_id,
            created_at,
            modified_at,
        }
    }
    
    // Métodos para crear nuevas versiones del archivo (inmutable)
    
    /// Crea una nueva versión del archivo con nombre actualizado
    #[allow(dead_code)]
    pub fn with_name(&self, new_name: String) -> FileResult<Self> {
        // Validar nombre de archivo
        if new_name.is_empty() || new_name.contains('/') || new_name.contains('\\') {
            return Err(FileError::InvalidFileName(new_name));
        }
        
        // Actualizar ruta basada en el nombre
        let parent_path = self.storage_path.parent();
        let new_storage_path = match parent_path {
            Some(parent) => parent.join(&new_name),
            None => StoragePath::from_string(&new_name),
        };
        
        // Actualizar representación en string
        let new_path_string = new_storage_path.to_string();
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
            
        Ok(Self {
            id: self.id.clone(),
            name: new_name,
            storage_path: new_storage_path,
            path_string: new_path_string,
            size: self.size,
            mime_type: self.mime_type.clone(),
            folder_id: self.folder_id.clone(),
            user_id: self.user_id.clone(),
            created_at: self.created_at,
            modified_at: now,
        })
    }
    
    /// Crea una nueva versión del archivo con carpeta actualizada
    pub fn with_folder(&self, folder_id: Option<String>, folder_path: Option<StoragePath>) -> FileResult<Self> {
        // Necesitamos una ruta de carpeta para actualizar la ruta del archivo
        let new_storage_path = match folder_path {
            Some(path) => path.join(&self.name),
            None => StoragePath::from_string(&self.name), // Raíz
        };
        
        // Actualizar representación en string
        let new_path_string = new_storage_path.to_string();
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
            
        Ok(Self {
            id: self.id.clone(),
            name: self.name.clone(),
            storage_path: new_storage_path,
            path_string: new_path_string,
            size: self.size,
            mime_type: self.mime_type.clone(),
            folder_id,
            user_id: self.user_id.clone(),
            created_at: self.created_at,
            modified_at: now,
        })
    }
    
    /// Crea una nueva versión del archivo con tamaño actualizado
    #[allow(dead_code)]
    pub fn with_size(&self, new_size: u64) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
            
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            storage_path: self.storage_path.clone(),
            path_string: self.path_string.clone(),
            size: new_size,
            mime_type: self.mime_type.clone(),
            folder_id: self.folder_id.clone(),
            user_id: self.user_id.clone(),
            created_at: self.created_at,
            modified_at: now,
        }
    }
    
    /// Crea una nueva versión del archivo con un usuario propietario diferente
    pub fn with_user_id(&self, new_user_id: Option<String>) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
            
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            storage_path: self.storage_path.clone(),
            path_string: self.path_string.clone(),
            size: self.size,
            mime_type: self.mime_type.clone(),
            folder_id: self.folder_id.clone(),
            user_id: new_user_id,
            created_at: self.created_at,
            modified_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_file_creation_with_valid_name() {
        let storage_path = StoragePath::from_string("/test/file.txt");
        let file = File::new(
            "123".to_string(),
            "file.txt".to_string(),
            storage_path,
            100,
            "text/plain".to_string(),
            None,
            Some("user1".to_string()),
        );
        
        assert!(file.is_ok());
        let file = file.unwrap();
        assert_eq!(file.user_id(), Some("user1"));
    }
    
    #[test]
    fn test_file_creation_with_invalid_name() {
        let storage_path = StoragePath::from_string("/test/invalid/file.txt");
        let file = File::new(
            "123".to_string(),
            "file/with/slash.txt".to_string(), // Nombre inválido
            storage_path,
            100,
            "text/plain".to_string(),
            None,
            None,
        );
        
        assert!(file.is_err());
        match file {
            Err(FileError::InvalidFileName(_)) => (),
            _ => panic!("Expected InvalidFileName error"),
        }
    }
    
    #[test]
    fn test_file_with_name() {
        let storage_path = StoragePath::from_string("/test/file.txt");
        let file = File::new(
            "123".to_string(),
            "file.txt".to_string(),
            storage_path,
            100,
            "text/plain".to_string(),
            None,
            Some("user1".to_string()),
        ).unwrap();
        
        let renamed = file.with_name("newname.txt".to_string());
        assert!(renamed.is_ok());
        let renamed = renamed.unwrap();
        assert_eq!(renamed.name(), "newname.txt");
        assert_eq!(renamed.id(), "123"); // El ID no cambia
        assert_eq!(renamed.user_id(), Some("user1")); // El usuario no cambia
    }
    
    #[test]
    fn test_file_with_user_id() {
        let storage_path = StoragePath::from_string("/test/file.txt");
        let file = File::new(
            "123".to_string(),
            "file.txt".to_string(),
            storage_path,
            100,
            "text/plain".to_string(),
            None,
            Some("user1".to_string()),
        ).unwrap();
        
        let transferred = file.with_user_id(Some("user2".to_string()));
        assert_eq!(transferred.user_id(), Some("user2"));
        assert_eq!(transferred.id(), "123"); // El ID no cambia
    }
}