//! PathService - Infrastructure service for storage path management
//!
//! This service was moved from domain/services because it implements application traits
//! (StoragePort) and has file system dependencies (tokio::fs).
//!
//! StoragePath (Value Object) remains in domain/services/path_service.rs

use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::application::ports::outbound::StoragePort;
use crate::common::errors::{DomainError, ErrorKind};
use crate::domain::services::path_service::StoragePath;

/// Infrastructure service for handling storage path operations
pub struct PathService {
    root_path: PathBuf,
}

impl PathService {
    /// Creates a new path service with a specific root
    pub fn new(root_path: PathBuf) -> Self {
        Self { root_path }
    }

    /// Converts a domain path to an absolute physical path
    pub fn resolve_path(&self, storage_path: &StoragePath) -> PathBuf {
        let mut path = self.root_path.clone();
        for segment in storage_path.segments() {
            path.push(segment);
        }
        path
    }

    /// Converts a physical path to a domain path
    pub fn to_storage_path(&self, physical_path: &Path) -> Option<StoragePath> {
        physical_path
            .strip_prefix(&self.root_path)
            .ok()
            .map(|rel_path| {
                let segments: Vec<String> = rel_path
                    .components()
                    .filter_map(|c| match c {
                        std::path::Component::Normal(os_str) => {
                            Some(os_str.to_string_lossy().to_string())
                        }
                        _ => None,
                    })
                    .collect();
                StoragePath::new(segments)
            })
    }

    /// Creates a file path within a folder
    pub fn create_file_path(&self, folder_path: &StoragePath, file_name: &str) -> StoragePath {
        folder_path.join(file_name)
    }

    /// Checks if a path is a direct child of another
    pub fn is_direct_child(
        &self,
        parent_path: &StoragePath,
        potential_child: &StoragePath,
    ) -> bool {
        if let Some(child_parent) = potential_child.parent() {
            &child_parent == parent_path
        } else {
            parent_path.is_empty()
        }
    }

    /// Checks if a path is at the root
    pub fn is_in_root(&self, path: &StoragePath) -> bool {
        path.parent().is_none_or(|p| p.is_empty())
    }

    /// Gets the root path used by this service
    pub fn get_root_path(&self) -> &Path {
        &self.root_path
    }

    /// Validates a path to ensure it doesn't contain dangerous components
    pub fn validate_path(&self, path: &StoragePath) -> Result<(), DomainError> {
        // Check for empty segments
        if path.segments().iter().any(|s| s.is_empty()) {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "Path",
                format!("Path contains empty segments: {}", path),
            ));
        }

        // Check for dangerous characters
        let dangerous_chars = ['\\', ':', '*', '?', '"', '<', '>', '|'];
        for segment in path.segments() {
            if segment.contains(&dangerous_chars[..]) {
                return Err(DomainError::new(
                    ErrorKind::InvalidInput,
                    "Path",
                    format!("Path contains dangerous characters: {}", segment),
                ));
            }

            // Check that it doesn't start with . (hidden in Unix)
            if segment.starts_with('.') && segment != ".well-known" {
                return Err(DomainError::new(
                    ErrorKind::InvalidInput,
                    "Path",
                    format!("Path segments cannot start with dot: {}", segment),
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
        // First validate the path
        self.validate_path(storage_path)?;

        // Resolve to physical path
        let physical_path = self.resolve_path(storage_path);

        // Create directories if they don't exist
        if !physical_path.exists() {
            fs::create_dir_all(&physical_path).await.map_err(|e| {
                DomainError::new(
                    ErrorKind::AccessDenied,
                    "Storage",
                    format!("Failed to create directory: {}", physical_path.display()),
                )
                .with_source(e)
            })?;

            tracing::debug!("Created directory: {}", physical_path.display());
        } else if !physical_path.is_dir() {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "Storage",
                format!(
                    "Path exists but is not a directory: {}",
                    physical_path.display()
                ),
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
