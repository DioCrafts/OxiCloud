use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;
use serde_json::Value;
use std::path::PathBuf;

use crate::application::dtos::search_dto::SearchCriteriaDto;
use crate::common::errors::DomainError;
use crate::domain::entities::file::File;
use crate::domain::services::path_service::StoragePath;

// Re-export domain repository traits for backward compatibility.
// The canonical definitions now live in domain/repositories/.
pub use crate::domain::repositories::file_repository::{
    FileReadRepository, FileRepository, FileWriteRepository,
};
pub use crate::domain::repositories::folder_repository::FolderRepository;

// ─────────────────────────────────────────────────────
// FileReadPort — application-layer alias for FileReadRepository
// ─────────────────────────────────────────────────────

/// Secondary port for file **reading**.
///
/// Encapsulates every operation that queries state without modifying it:
/// get, list, content, stream, mmap, range, path resolution.
#[async_trait]
pub trait FileReadPort: Send + Sync + 'static {
    /// Gets a file by its ID.
    async fn get_file(&self, id: &str) -> Result<File, DomainError>;

    /// Lists files in a folder.
    async fn list_files(&self, folder_id: Option<&str>) -> Result<Vec<File>, DomainError>;

    /// Gets content as a stream (ideal for large files).
    async fn get_file_stream(
        &self,
        id: &str,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError>;

    /// Stream of a byte range (HTTP Range Requests, video seek).
    async fn get_file_range_stream(
        &self,
        id: &str,
        start: u64,
        end: Option<u64>,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError>;

    /// Gets the logical storage path of a file.
    async fn get_file_path(&self, id: &str) -> Result<StoragePath, DomainError>;

    /// Gets the parent folder ID from a path (WebDAV).
    async fn get_parent_folder_id(&self, path: &str) -> Result<String, DomainError>;

    /// Gets the content-addressable blob hash for a file (O(1) DB lookup).
    ///
    /// Returns the SHA-256 hash stored in `storage.files.blob_hash`.
    /// Used for dedup reference tracking without loading file content.
    async fn get_blob_hash(&self, file_id: &str) -> Result<String, DomainError>;

    /// Find a file by its logical path (folder_name/.../file_name).
    ///
    /// The default implementation falls back to `list_files(None)` + linear
    /// scan (O(N)). Repositories should override with a direct SQL query.
    async fn find_file_by_path(&self, path: &str) -> Result<Option<File>, DomainError> {
        let path = path.trim_start_matches('/').trim_end_matches('/');
        let all_files = self.list_files(None).await?;
        for file in all_files {
            let file_path = file.path_string();
            let file_path = file_path.trim_start_matches('/').trim_end_matches('/');
            if file_path == path
                || file_path.ends_with(&format!("/{}", path))
                || path.ends_with(&format!("/{}", file_path))
            {
                return Ok(Some(file));
            }
        }
        Ok(None)
    }

    /// Search files with pagination and filtering at database level.
    ///
    /// This is more efficient than loading all files and filtering in memory,
    /// especially for large datasets. The filtering is pushed to the SQL layer.
    ///
    /// # Arguments
    /// * `folder_id` - Optional folder ID to scope the search (for recursive search, pass None)
    /// * `criteria` - Search criteria including name_contains, file_types, date ranges, size ranges
    /// * `user_id` - User ID for ownership filtering
    ///
    /// # Returns
    /// A tuple of (files, total_count) where files are paginated and filtered
    async fn search_files_paginated(
        &self,
        folder_id: Option<&str>,
        criteria: &SearchCriteriaDto,
        user_id: &str,
    ) -> Result<(Vec<File>, usize), DomainError>;

    /// Search files recursively in a folder subtree using ltree.
    ///
    /// When `root_folder_id` is Some, uses ltree descendant queries to find
    /// all files within the subtree rooted at that folder. When None, searches
    /// all files for the user. This replaces the O(N) recursive spawn-per-folder
    /// approach with O(1) SQL queries.
    ///
    /// Returns a tuple of (matching files, total count for pagination).
    async fn search_files_in_subtree(
        &self,
        root_folder_id: Option<&str>,
        criteria: &SearchCriteriaDto,
        user_id: &str,
    ) -> Result<(Vec<File>, usize), DomainError> {
        // Default: delegate to paginated search (non-recursive fallback)
        self.search_files_paginated(root_folder_id, criteria, user_id)
            .await
    }

    /// Count files matching the search criteria (without loading them).
    ///
    /// Used for pagination metadata without fetching the actual files.
    async fn count_files(
        &self,
        folder_id: Option<&str>,
        criteria: &SearchCriteriaDto,
        user_id: &str,
    ) -> Result<usize, DomainError>;
}

// ─────────────────────────────────────────────────────
// FileWritePort — all write / mutate operations
// ─────────────────────────────────────────────────────

/// Secondary port for file **writing**.
///
/// Covers: upload (buffered + streaming), move, delete, update,
/// and deferred registration for the write-behind cache.
#[async_trait]
pub trait FileWritePort: Send + Sync + 'static {
    /// Saves a new file from bytes.
    async fn save_file(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
    ) -> Result<File, DomainError>;

    /// Streaming upload — saves a file from a temp file already on disk.
    ///
    /// When `pre_computed_hash` is provided, the dedup service skips the
    /// hash re-read — zero extra I/O beyond the initial spool.
    async fn save_file_from_temp(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        temp_path: &std::path::Path,
        size: u64,
        pre_computed_hash: Option<String>,
    ) -> Result<File, DomainError>;

    /// Moves a file to another folder.
    async fn move_file(
        &self,
        file_id: &str,
        target_folder_id: Option<String>,
    ) -> Result<File, DomainError>;

    /// Renames a file (same folder, different name).
    async fn rename_file(&self, file_id: &str, new_name: &str) -> Result<File, DomainError>;

    /// Deletes a file.
    async fn delete_file(&self, id: &str) -> Result<(), DomainError>;

    /// Updates the content of an existing file.
    async fn update_file_content(&self, file_id: &str, content: Vec<u8>)
    -> Result<(), DomainError>;

    /// Streaming update — replaces file content from a temp file on disk.
    ///
    /// When `pre_computed_hash` is provided, the dedup service skips the
    /// hash re-read — zero extra I/O beyond the initial spool.
    /// Peak RAM: ~256 KB regardless of file size.
    async fn update_file_content_from_temp(
        &self,
        file_id: &str,
        temp_path: &std::path::Path,
        size: u64,
        content_type: Option<String>,
        pre_computed_hash: Option<String>,
    ) -> Result<(), DomainError>;

    /// Registers file metadata WITHOUT writing content to disk (write-behind).
    ///
    /// Returns `(File, PathBuf)` where `PathBuf` is the destination path for the
    /// deferred write that the `WriteBehindCache` will perform.
    async fn register_file_deferred(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        size: u64,
    ) -> Result<(File, PathBuf), DomainError>;

    /// Copies a file to a (possibly different) folder.
    ///
    /// With blob-dedup, this only creates a new metadata row and increments
    /// the blob reference count — zero disk I/O for the content.
    async fn copy_file(
        &self,
        file_id: &str,
        target_folder_id: Option<String>,
    ) -> Result<File, DomainError>;

    // ── Trash operations ──

    /// Moves a file to the trash
    async fn move_to_trash(&self, file_id: &str) -> Result<(), DomainError>;

    /// Restores a file from the trash to its original location
    async fn restore_from_trash(
        &self,
        file_id: &str,
        original_path: &str,
    ) -> Result<(), DomainError>;

    /// Permanently deletes a file (used by the trash)
    async fn delete_file_permanently(&self, file_id: &str) -> Result<(), DomainError>;
}

// ─────────────────────────────────────────────────────
// Auxiliary ports (unchanged)
// ─────────────────────────────────────────────────────

/// Secondary port for file path resolution
#[async_trait]
pub trait FilePathResolutionPort: Send + Sync + 'static {
    /// Gets the storage path of a file
    async fn get_file_path(&self, id: &str) -> Result<StoragePath, DomainError>;

    /// Resolves a domain path to a physical path
    fn resolve_path(&self, storage_path: &StoragePath) -> PathBuf;
}

/// Secondary port for file/directory existence verification
#[async_trait]
pub trait StorageVerificationPort: Send + Sync + 'static {
    /// Checks whether a file exists at the given path
    async fn file_exists(&self, storage_path: &StoragePath) -> Result<bool, DomainError>;

    /// Checks whether a directory exists at the given path
    async fn directory_exists(&self, storage_path: &StoragePath) -> Result<bool, DomainError>;
}

/// Secondary port for directory management
#[async_trait]
pub trait DirectoryManagementPort: Send + Sync + 'static {
    /// Creates directories if they do not exist
    async fn ensure_directory(&self, storage_path: &StoragePath) -> Result<(), DomainError>;
}

/// Secondary port for storage usage management
#[async_trait]
pub trait StorageUsagePort: Send + Sync + 'static {
    /// Updates storage usage statistics for a user
    async fn update_user_storage_usage(&self, user_id: &str) -> Result<i64, DomainError>;

    /// Updates storage usage statistics for a user, looked up by username
    async fn update_user_storage_usage_by_username(
        &self,
        username: &str,
    ) -> Result<i64, DomainError>;

    /// Updates storage usage statistics for all users
    async fn update_all_users_storage_usage(&self) -> Result<(), DomainError>;

    /// Checks if a user has enough quota for an additional upload.
    /// Returns Ok(()) if the upload is allowed, or Err(QuotaExceeded) with a
    /// descriptive message otherwise.
    async fn check_storage_quota(
        &self,
        user_id: &str,
        additional_bytes: u64,
    ) -> Result<(), DomainError>;

    /// Returns (used_bytes, quota_bytes) for a user.
    async fn get_user_storage_info(&self, user_id: &str) -> Result<(i64, i64), DomainError>;
}

/// Generic storage service interface for calendar and contact services
#[async_trait]
pub trait StorageUseCase: Send + Sync + 'static {
    /// Handle a request with the specified action and parameters
    async fn handle_request(&self, action: &str, params: Value) -> Result<Value, DomainError>;
}
