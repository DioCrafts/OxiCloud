use async_trait::async_trait;
use std::path::Path;
use std::sync::Arc;

use crate::application::dtos::file_dto::FileDto;
use crate::application::ports::file_ports::FileUploadUseCase;
use crate::application::ports::storage_ports::{FileReadPort, FileWritePort};
use crate::common::errors::DomainError;
use tracing::{debug, info, warn};

/// Helper function to extract username from folder path string.
/// e.g. "My Folder - user1/subfolder/file.txt" → "user1"
fn extract_username_from_path(path: &str) -> Option<String> {
    if !path.contains("My Folder - ") {
        return None;
    }
    let parts: Vec<&str> = path.split("My Folder - ").collect();
    if parts.len() <= 1 {
        return None;
    }
    let remainder = parts[1].trim();
    let username = remainder.split('/').next().unwrap_or(remainder);
    let username = username.trim();
    if username.is_empty() {
        return None;
    }
    Some(username.to_string())
}

/// Service for file upload operations.
///
/// All upload paths converge on streaming-to-disk:
/// - **Normal uploads**: handler spools multipart to temp file → `upload_file_streaming`
/// - **Chunked uploads**: chunks already on disk → `upload_file_from_path`
/// - **WebDAV PUT / empty files**: small in-memory buffer → `upload_file`
///
/// Peak RAM usage during upload: ~256 KB (streaming hash) regardless of file size.
pub struct FileUploadService {
    /// Write port — handles save, streaming, deferred registration
    file_write: Arc<dyn FileWritePort>,
    /// Read port — needed for WebDAV create_file / update_file
    file_read: Option<Arc<dyn FileReadPort>>,
    /// Optional storage usage tracking
    storage_usage_service:
        Option<Arc<dyn crate::application::ports::storage_ports::StorageUsagePort>>,
}

impl FileUploadService {
    /// Constructor with write port only (minimal).
    pub fn new(file_repository: Arc<dyn FileWritePort>) -> Self {
        Self {
            file_write: file_repository,
            file_read: None,
            storage_usage_service: None,
        }
    }

    /// Constructor for blob-storage model: write + read ports.
    pub fn new_with_read(
        file_write: Arc<dyn FileWritePort>,
        file_read: Arc<dyn FileReadPort>,
    ) -> Self {
        Self {
            file_write,
            file_read: Some(file_read),
            storage_usage_service: None,
        }
    }

    /// Configures the storage usage service
    pub fn with_storage_usage_service(
        mut self,
        storage_usage_service: Arc<dyn crate::application::ports::storage_ports::StorageUsagePort>,
    ) -> Self {
        self.storage_usage_service = Some(storage_usage_service);
        self
    }

    // ── private helpers ──────────────────────────────────────────

    /// Optionally update storage usage after a successful upload.
    fn maybe_update_storage_usage(&self, file: &FileDto) {
        if let Some(storage_service) = &self.storage_usage_service {
            let file_path = file.path.clone();
            if let Some(username) = extract_username_from_path(&file_path) {
                let service_clone = Arc::clone(storage_service);
                tokio::spawn(async move {
                    match service_clone
                        .update_user_storage_usage_by_username(&username)
                        .await
                    {
                        Ok(usage) => debug!(
                            "Updated storage usage for user {} to {} bytes",
                            username, usage
                        ),
                        Err(e) => warn!("Failed to update storage usage for {}: {}", username, e),
                    }
                });
            }
        }
    }
}

#[async_trait]
impl FileUploadUseCase for FileUploadService {
    /// Streaming upload from a temp file on disk.
    ///
    /// Peak RAM: ~256 KB (hash calculation) regardless of file size.
    /// The temp file is consumed (moved/deleted) by the blob store.
    async fn upload_file_streaming(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        temp_path: &Path,
        size: u64,
        pre_computed_hash: Option<String>,
    ) -> Result<FileDto, DomainError> {
        let file = self
            .file_write
            .save_file_from_temp(
                name.clone(),
                folder_id,
                content_type,
                temp_path,
                size,
                pre_computed_hash,
            )
            .await?;
        let dto = FileDto::from(file);
        info!(
            "📡 STREAMING UPLOAD: {} ({} bytes, ID: {})",
            name, size, dto.id
        );
        self.maybe_update_storage_usage(&dto);
        Ok(dto)
    }

    /// Simple byte-based upload (for WebDAV and empty files only).
    async fn upload_file(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
    ) -> Result<FileDto, DomainError> {
        let file = self
            .file_write
            .save_file(name, folder_id, content_type, content)
            .await?;
        let dto = FileDto::from(file);
        self.maybe_update_storage_usage(&dto);
        Ok(dto)
    }

    /// Upload from a file already on disk (chunked uploads).
    async fn upload_file_from_path(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        file_path: &Path,
        pre_computed_hash: Option<String>,
    ) -> Result<FileDto, DomainError> {
        let size = tokio::fs::metadata(file_path)
            .await
            .map_err(|e| {
                DomainError::internal_error(
                    "FileUpload",
                    format!("Failed to read file metadata: {}", e),
                )
            })?
            .len();

        self.upload_file_streaming(
            name,
            folder_id,
            content_type,
            file_path,
            size,
            pre_computed_hash,
        )
        .await
    }

    /// Creates a file at a specific path (for WebDAV PUT on new resource).
    async fn create_file(
        &self,
        parent_path: &str,
        filename: &str,
        content: &[u8],
        content_type: &str,
    ) -> Result<FileDto, DomainError> {
        let parent_id = if !parent_path.is_empty() {
            if let Some(file_read) = &self.file_read {
                file_read.get_parent_folder_id(parent_path).await.ok()
            } else {
                None
            }
        } else {
            None
        };

        let file = self
            .file_write
            .save_file(
                filename.to_string(),
                parent_id,
                content_type.to_string(),
                content.to_vec(),
            )
            .await?;
        let dto = FileDto::from(file);
        self.maybe_update_storage_usage(&dto);
        Ok(dto)
    }

    /// Updates an existing file's content, or creates it if not found (for WebDAV PUT).
    async fn update_file(&self, path: &str, content: &[u8]) -> Result<(), DomainError> {
        // Direct SQL lookup — O(folder_depth) instead of O(total_files)
        if let Some(file_read) = &self.file_read
            && let Some(file) = file_read.find_file_by_path(path).await?
        {
            self.file_write
                .update_file_content(file.id(), content.to_vec())
                .await?;
            return Ok(());
        }

        let path_normalized = path.trim_start_matches('/').trim_end_matches('/');
        let (parent_path, filename) = if let Some(idx) = path_normalized.rfind('/') {
            (&path_normalized[..idx], &path_normalized[idx + 1..])
        } else {
            ("", path_normalized)
        };
        self.create_file(parent_path, filename, content, "application/octet-stream")
            .await?;
        Ok(())
    }

    /// Streaming update — replaces file content from a temp file on disk.
    ///
    /// Uses `update_file_content_from_temp` which passes the pre-computed hash
    /// to dedup, avoiding a second full read of the file.
    /// For new files (not found at `path`), falls back to `upload_file_streaming`.
    ///
    /// Peak RAM: ~256 KB regardless of file size.
    async fn update_file_streaming(
        &self,
        path: &str,
        temp_path: &Path,
        size: u64,
        content_type: &str,
        pre_computed_hash: Option<String>,
    ) -> Result<(), DomainError> {
        // Try to find the existing file first
        if let Some(file_read) = &self.file_read
            && let Some(file) = file_read.find_file_by_path(path).await?
        {
            self.file_write
                .update_file_content_from_temp(
                    file.id(),
                    temp_path,
                    size,
                    Some(content_type.to_string()),
                    pre_computed_hash,
                )
                .await?;
            return Ok(());
        }

        // File doesn't exist — create it via streaming upload
        let path_normalized = path.trim_start_matches('/').trim_end_matches('/');
        let (parent_path, filename) = if let Some(idx) = path_normalized.rfind('/') {
            (&path_normalized[..idx], &path_normalized[idx + 1..])
        } else {
            ("", path_normalized)
        };

        let parent_id = if !parent_path.is_empty() {
            if let Some(file_read) = &self.file_read {
                file_read.get_parent_folder_id(parent_path).await.ok()
            } else {
                None
            }
        } else {
            None
        };

        self.file_write
            .save_file_from_temp(
                filename.to_string(),
                parent_id,
                content_type.to_string(),
                temp_path,
                size,
                pre_computed_hash,
            )
            .await?;
        Ok(())
    }
}
