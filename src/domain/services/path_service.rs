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
    /// Checks whether a single segment is safe (no traversal, no slashes)
    fn is_safe_segment(s: &str) -> bool {
        !s.is_empty() && s != "." && s != ".." && !s.contains('/')
    }

    /// Creates a new storage path, silently dropping any traversal segments
    pub fn new(segments: Vec<String>) -> Self {
        Self {
            segments: segments.into_iter().filter(|s| Self::is_safe_segment(s)).collect(),
        }
    }

    /// Creates an empty path (root)
    pub fn root() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    /// Creates a path from a string with segments separated by /
    ///
    /// Traversal segments (`.`, `..`) are silently stripped to prevent
    /// path-traversal attacks.
    pub fn from_string(path: &str) -> Self {
        let segments = path
            .split('/')
            .filter(|s| Self::is_safe_segment(s))
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

    /// Appends a segment to the path.
    ///
    /// Traversal segments (`.`, `..`) and segments containing `/` are
    /// silently ignored to prevent path-traversal attacks.
    pub fn join(&self, segment: &str) -> Self {
        let mut new_segments = self.segments.clone();
        if Self::is_safe_segment(segment) {
            new_segments.push(segment.to_string());
        }
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

    // ── Path-traversal hardening tests (VULN-02) ──────────────

    #[test]
    fn test_from_string_strips_dot_dot() {
        let path = StoragePath::from_string("../../etc/passwd");
        assert_eq!(path.segments(), &["etc", "passwd"]);
    }

    #[test]
    fn test_from_string_strips_single_dot() {
        let path = StoragePath::from_string("folder/./file.txt");
        assert_eq!(path.segments(), &["folder", "file.txt"]);
    }

    #[test]
    fn test_from_string_strips_mixed_traversal() {
        let path = StoragePath::from_string("a/../b/./c/../../d");
        assert_eq!(path.segments(), &["a", "b", "c", "d"]);
    }

    #[test]
    fn test_from_string_all_traversal_yields_root() {
        let path = StoragePath::from_string("../../..");
        assert!(path.is_empty());
        assert_eq!(path.to_string(), "/");
    }

    #[test]
    fn test_new_strips_traversal_segments() {
        let path = StoragePath::new(vec![
            "..".into(),
            "etc".into(),
            ".".into(),
            "passwd".into(),
        ]);
        assert_eq!(path.segments(), &["etc", "passwd"]);
    }

    #[test]
    fn test_new_strips_empty_segments() {
        let path = StoragePath::new(vec!["a".into(), "".into(), "b".into()]);
        assert_eq!(path.segments(), &["a", "b"]);
    }

    #[test]
    fn test_join_rejects_dot_dot() {
        let base = StoragePath::from_string("folder");
        let joined = base.join("..");
        // ".." is silently ignored — path stays unchanged
        assert_eq!(joined.segments(), &["folder"]);
    }

    #[test]
    fn test_join_rejects_single_dot() {
        let base = StoragePath::from_string("folder");
        let joined = base.join(".");
        assert_eq!(joined.segments(), &["folder"]);
    }

    #[test]
    fn test_join_rejects_slash_in_segment() {
        let base = StoragePath::from_string("folder");
        let joined = base.join("sub/../../etc/passwd");
        // Segment contains '/' → silently ignored
        assert_eq!(joined.segments(), &["folder"]);
    }

    #[test]
    fn test_from_pathbuf_strips_traversal() {
        let path = StoragePath::from(PathBuf::from("a/../b/./c"));
        // PathBuf Component::Normal only yields the normal parts
        // On most platforms this strips . and ..
        // but regardless, our from() only accepts Component::Normal
        assert!(!path.segments().contains(&"..".to_string()));
        assert!(!path.segments().contains(&".".to_string()));
    }
}
