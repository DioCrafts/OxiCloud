use crate::domain::services::path_service::StoragePath;

/// Error in the creation or manipulation of folder entities
#[derive(Debug, thiserror::Error)]
pub enum FolderError {
    #[error("Invalid folder name: {0}")]
    InvalidFolderName(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

/// Result type for folder entity operations
pub type FolderResult<T> = Result<T, FolderError>;

/// Represents a folder entity in the domain
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Folder {
    /// Unique identifier for the folder
    id: String,
    
    /// Name of the folder
    name: String,
    
    /// Path to the folder in the domain model
    storage_path: StoragePath,
    
    /// String representation of the path (for API compatibility)
    path_string: String,
    
    /// Parent folder ID (None if it's a root folder)
    parent_id: Option<String>,
    
    /// Creation timestamp
    created_at: u64,
    
    /// Last modification timestamp
    modified_at: u64,
}

// We no longer need this module, now we use a String directly

impl Default for Folder {
    fn default() -> Self {
        Self {
            id: "stub-id".to_string(),
            name: "stub-folder".to_string(),
            storage_path: StoragePath::from_string("/"),
            path_string: "/".to_string(),
            parent_id: None,
            created_at: 0,
            modified_at: 0,
        }
    }
}

impl Folder {
    /// Creates a new folder with validation
    pub fn new(
        id: String,
        name: String,
        storage_path: StoragePath,
        parent_id: Option<String>,
    ) -> FolderResult<Self> {
        // Validate folder name
        if name.is_empty() || name.contains('/') || name.contains('\\') {
            return Err(FolderError::InvalidFolderName(name));
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
            parent_id,
            created_at: now,
            modified_at: now,
        })
    }
    
    /// Creates a folder with specific timestamps (for reconstruction)
    pub fn with_timestamps(
        id: String,
        name: String,
        storage_path: StoragePath,
        parent_id: Option<String>,
        created_at: u64,
        modified_at: u64,
    ) -> FolderResult<Self> {
        // Validate folder name
        if name.is_empty() || name.contains('/') || name.contains('\\') {
            return Err(FolderError::InvalidFolderName(name));
        }
        
        // Store the path string for serialization compatibility
        let path_string = storage_path.to_string();
            
        Ok(Self {
            id,
            name,
            storage_path,
            path_string,
            parent_id,
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
    
    pub fn parent_id(&self) -> Option<&str> {
        self.parent_id.as_deref()
    }
    
    pub fn created_at(&self) -> u64 {
        self.created_at
    }
    
    pub fn modified_at(&self) -> u64 {
        self.modified_at
    }
    
    /// Creates a new Folder instance from a DTO
    /// This function is primarily for conversions in batch handlers
    pub fn from_dto(
        id: String,
        name: String,
        path: String,
        parent_id: Option<String>,
        created_at: u64,
        modified_at: u64,
    ) -> Self {
        // Create storage_path from the string
        let storage_path = StoragePath::from_string(&path);
        
        // Create directly without validation to avoid errors in DTO conversions
        Self {
            id,
            name,
            storage_path,
            path_string: path,
            parent_id,
            created_at,
            modified_at,
        }
    }
    
    // Methods to create new versions of the folder (immutable)
    
    /// Creates a new version of the folder with updated name
    pub fn with_name(&self, new_name: String) -> FolderResult<Self> {
        // Validate folder name
        if new_name.is_empty() || new_name.contains('/') || new_name.contains('\\') {
            return Err(FolderError::InvalidFolderName(new_name));
        }
        
        // Update path based on the name
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
            parent_id: self.parent_id.clone(),
            created_at: self.created_at,
            modified_at: now,
        })
    }
    
    /// Creates a new version of the folder with updated parent
    pub fn with_parent(&self, parent_id: Option<String>, parent_path: Option<StoragePath>) -> FolderResult<Self> {
        // We need a folder path to update the path
        let new_storage_path = match parent_path {
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
            parent_id,
            created_at: self.created_at,
            modified_at: now,
        })
    }
    
    /// Returns an absolute path for this folder
    pub fn get_absolute_path<P: AsRef<std::path::Path>>(&self, root_path: P) -> std::path::PathBuf {
        let mut result = std::path::PathBuf::from(root_path.as_ref());
        
        // Skip leading '/' from path_string to avoid creating absolute path incorrectly
        let relative_path = if self.path_string.starts_with('/') {
            &self.path_string[1..]
        } else {
            &self.path_string
        };
        
        if !relative_path.is_empty() {
            result.push(relative_path);
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_folder_creation_with_valid_name() {
        let storage_path = StoragePath::from_string("/test/folder");
        let folder = Folder::new(
            "123".to_string(),
            "my_folder".to_string(),
            storage_path,
            None,
        );
        
        assert!(folder.is_ok());
    }
    
    #[test]
    fn test_folder_creation_with_invalid_name() {
        let storage_path = StoragePath::from_string("/test/invalid/folder");
        let folder = Folder::new(
            "123".to_string(),
            "folder/with/slash".to_string(), // Invalid name
            storage_path,
            None,
        );
        
        assert!(folder.is_err());
        match folder {
            Err(FolderError::InvalidFolderName(_)) => (),
            _ => panic!("Expected InvalidFolderName error"),
        }
    }
    
    #[test]
    fn test_folder_with_name() {
        let storage_path = StoragePath::from_string("/test/folder");
        let folder = Folder::new(
            "123".to_string(),
            "old_name".to_string(),
            storage_path,
            None,
        ).unwrap();
        
        let renamed = folder.with_name("new_name".to_string());
        assert!(renamed.is_ok());
        let renamed = renamed.unwrap();
        assert_eq!(renamed.name(), "new_name");
        assert_eq!(renamed.id(), "123"); // The ID doesn't change
    }
}