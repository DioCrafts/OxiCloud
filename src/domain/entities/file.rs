use crate::domain::services::path_service::StoragePath;

// Re-exportar errores de entidad desde el módulo centralizado
pub use super::entity_errors::{FileError, FileResult};

/**
 * Represents a file in the system's domain model.
 * 
 * The File entity is a core domain object that encapsulates all properties and behaviors
 * of a file in the system. It implements an immutable design pattern where modification
 * operations return new instances rather than modifying the existing one.
 * 
 * This entity maintains both physical storage information and logical metadata about files,
 * serving as the bridge between the storage system and the application.
 */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    /// Unique identifier for the file - used throughout the system for file operations
    id: String,
    
    /// Name of the file including extension
    name: String,
    
    /// Path to the file in the domain model
    storage_path: StoragePath,
    
    /// String representation of the path for API compatibility
    path_string: String,
    
    /// Size of the file in bytes
    size: u64,
    
    /// MIME type of the file (e.g., "text/plain", "image/jpeg")
    mime_type: String,
    
    /// Parent folder ID if the file is within a folder, None if in root
    folder_id: Option<String>,
    
    /// Creation timestamp (seconds since UNIX epoch)
    created_at: u64,
    
    /// Last modification timestamp (seconds since UNIX epoch)
    modified_at: u64,
}

// We no longer need this module, now we use a String directly

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
            created_at: 0,
            modified_at: 0,
        }
    }
}

impl File {
    /// Creates a new file with validation
    pub fn new(
        id: String,
        name: String,
        storage_path: StoragePath,
        size: u64,
        mime_type: String,
        folder_id: Option<String>,
    ) -> FileResult<Self> {
        // Validate file name
        if name.is_empty() || name.contains('/') || name.contains('\\') {
            return Err(FileError::InvalidFileName(name));
        }
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Store the path string for serialization compatibility
        let path_string = storage_path.to_string();
            
        Ok(Self {
            id,
            name,
            storage_path,
            path_string,
            size,
            mime_type,
            folder_id,
            created_at: now,
            modified_at: now,
        })
    }
    
    /// Creates a folder entity
    pub fn new_folder(
        id: String,
        name: String,
        storage_path: StoragePath,
        parent_id: Option<String>,
        created_at: u64,
        modified_at: u64,
    ) -> FileResult<Self> {
        // Validate folder name
        if name.is_empty() || name.contains('/') || name.contains('\\') {
            return Err(FileError::InvalidFileName(name));
        }
        
        // Store the path string for serialization compatibility
        let path_string = storage_path.to_string();
            
        Ok(Self {
            id,
            name,
            storage_path,
            path_string,
            size: 0, // Folders have zero size
            mime_type: "directory".to_string(), // Standard MIME type for directories
            folder_id: parent_id,
            created_at,
            modified_at,
        })
    }
    
    /// Creates a file with specific timestamps (for reconstruction)
    pub fn with_timestamps(
        id: String,
        name: String,
        storage_path: StoragePath,
        size: u64,
        mime_type: String,
        folder_id: Option<String>,
        created_at: u64,
        modified_at: u64,
    ) -> FileResult<Self> {
        // Validate file name
        if name.is_empty() || name.contains('/') || name.contains('\\') {
            return Err(FileError::InvalidFileName(name));
        }
        
        // Store the path string for serialization compatibility
        let path_string = storage_path.to_string();
            
        Ok(Self {
            id,
            name,
            storage_path,
            path_string,
            size,
            mime_type,
            folder_id,
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
    
    pub fn created_at(&self) -> u64 {
        self.created_at
    }
    
    pub fn modified_at(&self) -> u64 {
        self.modified_at
    }
    
    /// Creates a new File instance from a DTO
    /// This function is primarily for conversions in batch handlers
    pub fn from_dto(
        id: String,
        name: String,
        path: String,
        size: u64,
        mime_type: String,
        folder_id: Option<String>,
        created_at: u64,
        modified_at: u64,
    ) -> Self {
        // Create storage_path from string
        let storage_path = StoragePath::from_string(&path);
        
        // Create directly without validation to avoid errors in DTO conversions
        Self {
            id,
            name,
            storage_path,
            path_string: path,
            size,
            mime_type,
            folder_id,
            created_at,
            modified_at,
        }
    }
    
    // Methods to create new versions of the file (immutable)
    
    /// Creates a new version of the file with updated name
    pub fn with_name(&self, new_name: String) -> FileResult<Self> {
        // Validate file name
        if new_name.is_empty() || new_name.contains('/') || new_name.contains('\\') {
            return Err(FileError::InvalidFileName(new_name));
        }
        
        // Update path based on name
        let parent_path = self.storage_path.parent();
        let new_storage_path = match parent_path {
            Some(parent) => parent.join(&new_name),
            None => StoragePath::from_string(&new_name),
        };
        
        // Update string representation
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
            created_at: self.created_at,
            modified_at: now,
        })
    }
    
    /// Creates a new version of the file with updated folder
    pub fn with_folder(&self, folder_id: Option<String>, folder_path: Option<StoragePath>) -> FileResult<Self> {
        // We need a folder path to update the file path
        let new_storage_path = match folder_path {
            Some(path) => path.join(&self.name),
            None => StoragePath::from_string(&self.name), // Root
        };
        
        // Update string representation
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
            created_at: self.created_at,
            modified_at: now,
        })
    }
    
    /// Creates a new version of the file with updated size
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
        );
        
        assert!(file.is_ok());
    }
    
    #[test]
    fn test_file_creation_with_invalid_name() {
        let storage_path = StoragePath::from_string("/test/invalid/file.txt");
        let file = File::new(
            "123".to_string(),
            "file/with/slash.txt".to_string(), // Invalid name
            storage_path,
            100,
            "text/plain".to_string(),
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
        ).unwrap();
        
        let renamed = file.with_name("newname.txt".to_string());
        assert!(renamed.is_ok());
        let renamed = renamed.unwrap();
        assert_eq!(renamed.name(), "newname.txt");
        assert_eq!(renamed.id(), "123"); // El ID no cambia
    }
}