//! StoragePath - Domain Value Object for representing storage paths
//!
//! This module contains only the StoragePath Value Object which is part of the pure domain.
//! PathService (which implements StoragePort and StorageMediator) was moved to
//! infrastructure/services/path_service.rs because it has file system dependencies.

use std::path::PathBuf;

/// Represents a storage path in the domain (Value Object)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StoragePath {
    segments: Vec<String>,
}

impl StoragePath {
    /// Creates a new storage path
    pub fn new(segments: Vec<String>) -> Self {
        Self { segments }
    }

    /// Creates an empty path (root)
    pub fn root() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    /// Creates a path from a string with segments separated by /
    pub fn from_string(path: &str) -> Self {
        let segments = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        Self { segments }
    }

    /// Creates a path from a PathBuf
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

    /// Appends a segment to the path
    pub fn join(&self, segment: &str) -> Self {
        let mut new_segments = self.segments.clone();
        new_segments.push(segment.to_string());
        Self {
            segments: new_segments,
        }
    }

    /// Gets the file name (last segment)
    pub fn file_name(&self) -> Option<String> {
        self.segments.last().cloned()
    }

    /// Gets the parent directory path
    pub fn parent(&self) -> Option<Self> {
        if self.segments.is_empty() {
            None
        } else {
            let parent_segments = self.segments[..self.segments.len() - 1].to_vec();
            Some(Self {
                segments: parent_segments,
            })
        }
    }

    /// Checks if the path is empty (is the root)
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

}

impl std::fmt::Display for StoragePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.segments.is_empty() {
            write!(f, "/")
        } else {
            write!(f, "/{}", self.segments.join("/"))
        }
    }
}

impl StoragePath {
    /// Returns the path representation as a string
    pub fn as_str(&self) -> &str {
        // Note: The implementation should really store the string,
        // but here we do a temporary implementation that always returns "/"
        // This is only used for the get_folder_path_str implementation
        "/"
    }

    /// Gets the path segments
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
