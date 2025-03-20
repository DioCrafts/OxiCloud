/// Abstracto servicio de dominio para rutas, sin dependencias de sistema de archivos
/// Representa una ruta de almacenamiento en el dominio
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StoragePath {
    segments: Vec<String>,
}

impl StoragePath {
    /// Crea una nueva ruta de almacenamiento
    #[allow(dead_code)]
    pub fn new(segments: Vec<String>) -> Self {
        Self { segments }
    }
    
    /// Crea una ruta vacía (raíz)
    pub fn root() -> Self {
        Self { segments: Vec::new() }
    }
    
    /// Crea una ruta a partir de una cadena con segmentos separados por /
    pub fn from_string(path: &str) -> Self {
        let segments = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        Self { segments }
    }
    
    /// Crea una ruta a partir de un PathBuf
    pub fn from(path_buf: PathBuf) -> Self {
        let segments = path_buf
            .components()
            .filter_map(|c| match c {
                std::path::Component::Normal(os_str) => Some(os_str.to_string_lossy().to_string()),
                _ => None,
            })
            .collect();
        Self { segments }
    }
    
    /// Añade un segmento a la ruta
    pub fn join(&self, segment: &str) -> Self {
        let mut new_segments = self.segments.clone();
        new_segments.push(segment.to_string());
        Self { segments: new_segments }
    }
    
    /// Obtiene el nombre del archivo (último segmento)
    pub fn file_name(&self) -> Option<String> {
        self.segments.last().cloned()
    }
    
    /// Obtiene la ruta del directorio padre
    pub fn parent(&self) -> Option<Self> {
        if self.segments.is_empty() {
            None
        } else {
            let parent_segments = self.segments[..self.segments.len() - 1].to_vec();
            Some(Self { segments: parent_segments })
        }
    }
    
    /// Verifica si la ruta está vacía (es la raíz)
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }
    
    /// Convierte la ruta a una cadena con formato "/segment1/segment2/..."
    pub fn to_string(&self) -> String {
        if self.segments.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", self.segments.join("/"))
        }
    }
    
    /// Obtiene los segmentos de la ruta
    pub fn segments(&self) -> &[String] {
        &self.segments
    }
}

use std::path::{Path, PathBuf};
use async_trait::async_trait;
use tokio::fs;

use crate::common::errors::{DomainError, ErrorKind};
use crate::application::ports::outbound::StoragePort;
use crate::application::services::storage_mediator::{StorageMediator, StorageMediatorResult, StorageMediatorError};
use crate::domain::entities::folder::Folder;

/// Servicio de dominio para manejar operaciones con rutas de almacenamiento
pub struct PathService {
    root_path: PathBuf, // Necesario para la implementación
}

impl PathService {
    /// Crea un nuevo servicio de rutas con una raíz específica
    pub fn new(root_path: PathBuf) -> Self {
        Self { root_path }
    }
    
    /// Convierte una ruta del dominio a una ruta física absoluta
    pub fn resolve_path(&self, storage_path: &StoragePath) -> PathBuf {
        let mut path = self.root_path.clone();
        for segment in storage_path.segments() {
            path.push(segment);
        }
        path
    }
    
    /// Convierte una ruta física a una ruta de dominio
    #[allow(dead_code)]
    pub fn to_storage_path(&self, physical_path: &Path) -> Option<StoragePath> {
        physical_path.strip_prefix(&self.root_path).ok().map(|rel_path| {
            let segments = rel_path
                .components()
                .filter_map(|c| match c {
                    std::path::Component::Normal(os_str) => Some(os_str.to_string_lossy().to_string()),
                    _ => None,
                })
                .collect();
            StoragePath { segments }
        })
    }
    
    /// Crea una ruta de archivo dentro de una carpeta
    #[allow(dead_code)]
    pub fn create_file_path(&self, folder_path: &StoragePath, file_name: &str) -> StoragePath {
        folder_path.join(file_name)
    }
    
    /// Verifica si una ruta es directamente hija de otra
    #[allow(dead_code)]
    pub fn is_direct_child(&self, parent_path: &StoragePath, potential_child: &StoragePath) -> bool {
        if let Some(child_parent) = potential_child.parent() {
            &child_parent == parent_path
        } else {
            parent_path.is_empty()
        }
    }
    
    /// Verifica si una ruta está en la raíz
    #[allow(dead_code)]
    pub fn is_in_root(&self, path: &StoragePath) -> bool {
        path.parent().map_or(true, |p| p.is_empty())
    }
    
    /// Gets the root path used by this service
    #[allow(dead_code)]
    pub fn get_root_path(&self) -> &Path {
        &self.root_path
    }
    
    /// Valida una ruta para asegurar que no contiene componentes peligrosos
    pub fn validate_path(&self, path: &StoragePath) -> Result<(), DomainError> {
        // Verificar que no haya segmentos vacíos
        if path.segments().iter().any(|s| s.is_empty()) {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "Path",
                format!("Path contains empty segments: {}", path.to_string())
            ));
        }
        
        // Verificar que no haya caracteres peligrosos
        let dangerous_chars = ['\\', ':', '*', '?', '"', '<', '>', '|'];
        for segment in path.segments() {
            if segment.contains(&dangerous_chars[..]) {
                return Err(DomainError::new(
                    ErrorKind::InvalidInput,
                    "Path",
                    format!("Path contains dangerous characters: {}", segment)
                ));
            }
            
            // Verificar que no empiece con . (oculto en Unix)
            if segment.starts_with('.') && segment != ".well-known" {
                return Err(DomainError::new(
                    ErrorKind::InvalidInput,
                    "Path",
                    format!("Path segments cannot start with dot: {}", segment)
                ));
            }
        }
        
        Ok(())
    }
}

#[async_trait]
impl StoragePort for PathService {
    fn resolve_path(&self, storage_path: &StoragePath) -> PathBuf {
        let mut path = self.root_path.clone();
        for segment in storage_path.segments() {
            path.push(segment);
        }
        path
    }
    
    async fn ensure_directory(&self, storage_path: &StoragePath) -> Result<(), DomainError> {
        // Primero validar la ruta
        self.validate_path(storage_path)?;
        
        // Resolver a ruta física
        let physical_path = self.resolve_path(storage_path);
        
        // Crear directorios si no existen
        if !physical_path.exists() {
            fs::create_dir_all(&physical_path).await
                .map_err(|e| DomainError::new(
                    ErrorKind::AccessDenied,
                    "Storage",
                    format!("Failed to create directory: {}", physical_path.display())
                ).with_source(e))?;
                
            tracing::debug!("Created directory: {}", physical_path.display());
        } else if !physical_path.is_dir() {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "Storage",
                format!("Path exists but is not a directory: {}", physical_path.display())
            ));
        }
        
        Ok(())
    }
    
    async fn file_exists(&self, storage_path: &StoragePath) -> Result<bool, DomainError> {
        let physical_path = self.resolve_path(storage_path);
        
        let exists = physical_path.exists() && physical_path.is_file();
        Ok(exists)
    }
    
    async fn directory_exists(&self, storage_path: &StoragePath) -> Result<bool, DomainError> {
        let physical_path = self.resolve_path(storage_path);
        
        let exists = physical_path.exists() && physical_path.is_dir();
        Ok(exists)
    }
}

#[async_trait]
impl StorageMediator for PathService {
    async fn get_folder_path(&self, folder_id: &str) -> StorageMediatorResult<PathBuf> {
        // This is a simplified implementation since PathService doesn't have direct
        // access to folder repository. It's typically used through a proxy.
        Err(StorageMediatorError::NotFound(format!("Folder with ID {} not found", folder_id)))
    }
    
    async fn get_folder_storage_path(&self, folder_id: &str) -> StorageMediatorResult<StoragePath> {
        // Simplified implementation - should be overridden by actual implementations
        Err(StorageMediatorError::NotFound(format!("Folder with ID {} not found", folder_id)))
    }
    
    async fn get_folder(&self, folder_id: &str) -> StorageMediatorResult<Folder> {
        // Simplified implementation - should be overridden by actual implementations
        Err(StorageMediatorError::NotFound(format!("Folder with ID {} not found", folder_id)))
    }
    
    async fn file_exists_at_path(&self, path: &Path) -> StorageMediatorResult<bool> {
        let abs_path = self.resolve_path(&StoragePath::from_string(&path.to_string_lossy()));
        Ok(abs_path.exists() && abs_path.is_file())
    }
    
    async fn file_exists_at_storage_path(&self, storage_path: &StoragePath) -> StorageMediatorResult<bool> {
        let abs_path = self.resolve_path(storage_path);
        Ok(abs_path.exists() && abs_path.is_file())
    }
    
    async fn folder_exists_at_path(&self, path: &Path) -> StorageMediatorResult<bool> {
        let abs_path = self.resolve_path(&StoragePath::from_string(&path.to_string_lossy()));
        Ok(abs_path.exists() && abs_path.is_dir())
    }
    
    async fn folder_exists_at_storage_path(&self, storage_path: &StoragePath) -> StorageMediatorResult<bool> {
        let abs_path = self.resolve_path(storage_path);
        Ok(abs_path.exists() && abs_path.is_dir())
    }
    
    fn resolve_path(&self, relative_path: &Path) -> PathBuf {
        // Convert path to storage path then resolve
        let path_str = relative_path.to_string_lossy().to_string();
        let storage_path = StoragePath::from_string(&path_str);
        self.resolve_path(&storage_path)
    }
    
    fn resolve_storage_path(&self, storage_path: &StoragePath) -> PathBuf {
        self.resolve_path(storage_path)
    }
    
    async fn ensure_directory(&self, path: &Path) -> StorageMediatorResult<()> {
        let abs_path = self.resolve_path(&StoragePath::from_string(&path.to_string_lossy()));
        
        if !abs_path.exists() {
            fs::create_dir_all(&abs_path).await
                .map_err(|e| StorageMediatorError::AccessError(format!("Failed to create directory: {}", e)))?;
        } else if !abs_path.is_dir() {
            return Err(StorageMediatorError::InvalidPath(
                format!("Path exists but is not a directory: {}", abs_path.display())
            ));
        }
        
        Ok(())
    }
    
    async fn ensure_storage_directory(&self, storage_path: &StoragePath) -> StorageMediatorResult<()> {
        let abs_path = self.resolve_path(storage_path);
        
        if !abs_path.exists() {
            fs::create_dir_all(&abs_path).await
                .map_err(|e| StorageMediatorError::AccessError(format!("Failed to create directory: {}", e)))?;
        } else if !abs_path.is_dir() {
            return Err(StorageMediatorError::InvalidPath(
                format!("Path exists but is not a directory: {}", abs_path.display())
            ));
        }
        
        Ok(())
    }
    
    async fn get_file_owner(&self, file_id: &str) -> StorageMediatorResult<Option<String>> {
        // PathService doesn't have direct access to file repository
        // This is a simplified implementation that should be overridden by actual implementations
        tracing::debug!("File owner lookup requested for ID: {} in PathService", file_id);
        
        // Returns None to indicate no specific owner for this base implementation
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_resolve_path() {
        let service = PathService::new(PathBuf::from("/storage"));
        
        let storage_path = StoragePath::from_string("test/file.txt");
        let absolute = service.resolve_path(&storage_path);
        
        assert_eq!(absolute, PathBuf::from("/storage/test/file.txt"));
    }
    
    #[test]
    fn test_to_storage_path() {
        let service = PathService::new(PathBuf::from("/storage"));
        
        let physical_path = PathBuf::from("/storage/folder/file.txt");
        let storage_path = service.to_storage_path(&physical_path).unwrap();
        
        assert_eq!(storage_path.to_string(), "/folder/file.txt");
    }
    
    #[test]
    fn test_is_in_root() {
        let service = PathService::new(PathBuf::from("/storage"));
        
        let root_path = StoragePath::from_string("file.txt");
        let nested_path = StoragePath::from_string("folder/file.txt");
        
        assert!(service.is_in_root(&root_path));
        assert!(!service.is_in_root(&nested_path));
    }
    
    #[test]
    fn test_is_direct_child() {
        let service = PathService::new(PathBuf::from("/storage"));
        
        let parent = StoragePath::from_string("folder");
        let child = StoragePath::from_string("folder/file.txt");
        let not_child = StoragePath::from_string("folder2/file.txt");
        
        assert!(service.is_direct_child(&parent, &child));
        assert!(!service.is_direct_child(&parent, &not_child));
    }
    
    #[test]
    fn test_create_file_path() {
        let service = PathService::new(PathBuf::from("/storage"));
        
        let folder_path = StoragePath::from_string("folder");
        let file_path = service.create_file_path(&folder_path, "file.txt");
        
        assert_eq!(file_path.to_string(), "/folder/file.txt");
    }
}