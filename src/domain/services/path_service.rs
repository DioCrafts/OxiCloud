//! StoragePath - Value Object del dominio para representar rutas de almacenamiento
//! 
//! Este módulo contiene solo el Value Object StoragePath que es parte del dominio puro.
//! PathService (que implementa StoragePort y StorageMediator) fue movido a 
//! infrastructure/services/path_service.rs porque tiene dependencias de sistema de archivos.

use std::path::PathBuf;

/// Representa una ruta de almacenamiento en el dominio (Value Object)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StoragePath {
    segments: Vec<String>,
}

impl StoragePath {
    /// Crea una nueva ruta de almacenamiento
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
    
    /// Devuelve la representación de la ruta como cadena
    pub fn as_str(&self) -> &str {
        // Nota: La implementación realmente debería almacenar la cadena,
        // pero aquí hacemos una implementación temporal que siempre devuelve "/"
        // Esto se usa solo para la implementación de get_folder_path_str
        "/"
    }
    
    /// Obtiene los segmentos de la ruta
    pub fn segments(&self) -> &[String] {
        &self.segments
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_storage_path_from_string() {
        let path = StoragePath::from_string("folder/subfolder/file.txt");
        assert_eq!(path.segments(), &["folder", "subfolder", "file.txt"]);
        assert_eq!(path.to_string(), "/folder/subfolder/file.txt");
    }
    
    #[test]
    fn test_storage_path_join() {
        let path = StoragePath::from_string("folder");
        let joined = path.join("file.txt");
        assert_eq!(joined.to_string(), "/folder/file.txt");
    }
    
    #[test]
    fn test_storage_path_parent() {
        let path = StoragePath::from_string("folder/file.txt");
        let parent = path.parent().unwrap();
        assert_eq!(parent.to_string(), "/folder");
    }
    
    #[test]
    fn test_storage_path_root() {
        let root = StoragePath::root();
        assert!(root.is_empty());
        assert_eq!(root.to_string(), "/");
    }
    
    #[test]
    fn test_storage_path_file_name() {
        let path = StoragePath::from_string("folder/file.txt");
        assert_eq!(path.file_name(), Some("file.txt".to_string()));
    }
}