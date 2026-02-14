use async_trait::async_trait;
use std::sync::Arc;

use crate::application::dtos::file_dto::FileDto;
use crate::application::ports::dedup_ports::DedupPort;
use crate::application::ports::file_ports::FileManagementUseCase;
use crate::application::ports::storage_ports::{FileReadPort, FileWritePort};
use crate::application::ports::trash_ports::TrashUseCase;
use crate::common::errors::DomainError;
use tracing::{debug, error, info, warn};

/// Service for file management operations (move, delete).
///
/// The `delete_with_cleanup` method internalises:
/// 1. Content-hash computation for dedup tracking
/// 2. Trash-first soft-delete
/// 3. Fallback to permanent delete
/// 4. Dedup reference-count decrement
pub struct FileManagementService {
    file_repository: Arc<dyn FileWritePort>,
    file_read: Option<Arc<dyn FileReadPort>>,
    trash_service: Option<Arc<dyn TrashUseCase>>,
    dedup_service: Option<Arc<dyn DedupPort>>,
}

impl FileManagementService {
    /// Backward-compatible constructor (no trash, no dedup).
    pub fn new(file_repository: Arc<dyn FileWritePort>) -> Self {
        Self {
            file_repository,
            file_read: None,
            trash_service: None,
            dedup_service: None,
        }
    }

    /// Full constructor with trash + dedup ports.
    pub fn new_full(
        file_repository: Arc<dyn FileWritePort>,
        file_read: Arc<dyn FileReadPort>,
        trash_service: Option<Arc<dyn TrashUseCase>>,
        dedup_service: Arc<dyn DedupPort>,
    ) -> Self {
        Self {
            file_repository,
            file_read: Some(file_read),
            trash_service,
            dedup_service: Some(dedup_service),
        }
    }

    /// Setter for late-bound trash service.
    pub fn with_trash_service(mut self, trash_service: Arc<dyn TrashUseCase>) -> Self {
        self.trash_service = Some(trash_service);
        self
    }

    // ── private helpers ──────────────────────────────────────────

    /// Compute the content hash for dedup tracking. Returns `None` on failure.
    async fn compute_content_hash(&self, id: &str) -> Option<String> {
        let dedup = self.dedup_service.as_ref()?;
        let file_read = self.file_read.as_ref()?;
        match file_read.get_file_content(id).await {
            Ok(content) => {
                let hash = dedup.hash_bytes(&content);
                debug!("🔗 DEDUP: File {} has content hash: {}", id, &hash[..12]);
                Some(hash)
            }
            Err(e) => {
                debug!("Could not read file content for dedup: {}", e);
                None
            }
        }
    }

    /// Decrement dedup reference count; log result.
    async fn decrement_dedup_ref(&self, hash: &str) {
        let Some(dedup) = &self.dedup_service else {
            return;
        };
        match dedup.remove_reference(hash).await {
            Ok(true) => info!(
                "🗑️ DEDUP: Blob {} deleted (no more references)",
                &hash[..12]
            ),
            Ok(false) => debug!("🔗 DEDUP: Reference removed from blob {}", &hash[..12]),
            Err(e) => warn!("⚠️ DEDUP: Failed to decrement reference: {}", e),
        }
    }
}

#[async_trait]
impl FileManagementUseCase for FileManagementService {
    async fn move_file(
        &self,
        file_id: &str,
        folder_id: Option<String>,
    ) -> Result<FileDto, DomainError> {
        info!(
            "Moving file with ID: {} to folder: {:?}",
            file_id, folder_id
        );

        let moved_file = self
            .file_repository
            .move_file(file_id, folder_id)
            .await
            .map_err(|e| {
                error!("Error moving file (ID: {}): {}", file_id, e);
                e
            })?;

        info!(
            "File moved successfully: {} (ID: {}) to folder: {:?}",
            moved_file.name(),
            moved_file.id(),
            moved_file.folder_id()
        );

        Ok(FileDto::from(moved_file))
    }

    async fn rename_file(&self, file_id: &str, new_name: &str) -> Result<FileDto, DomainError> {
        info!("Renaming file with ID: {} to \"{}\"", file_id, new_name);

        let renamed_file = self
            .file_repository
            .rename_file(file_id, new_name)
            .await
            .map_err(|e| {
                error!("Error renaming file (ID: {}): {}", file_id, e);
                e
            })?;

        info!(
            "File renamed successfully: {} (ID: {})",
            renamed_file.name(),
            renamed_file.id()
        );

        Ok(FileDto::from(renamed_file))
    }

    async fn delete_file(&self, id: &str) -> Result<(), DomainError> {
        self.file_repository.delete_file(id).await
    }

    /// Smart delete: trash-first with dedup reference cleanup.
    async fn delete_with_cleanup(&self, id: &str, user_id: &str) -> Result<bool, DomainError> {
        // Step 1: Compute content hash for dedup tracking
        let content_hash = self.compute_content_hash(id).await;

        // Step 2: Try trash (soft delete)
        if let Some(trash) = &self.trash_service {
            info!("Moving file to trash: {}", id);
            match trash.move_to_trash(id, "file", user_id).await {
                Ok(_) => {
                    info!("File successfully moved to trash: {}", id);
                    if let Some(hash) = &content_hash {
                        self.decrement_dedup_ref(hash).await;
                    }
                    return Ok(true); // trashed
                }
                Err(err) => {
                    error!("Could not move file to trash: {:?}", err);
                    warn!("Falling back to permanent delete");
                    // fall through
                }
            }
        } else {
            warn!("Trash service not available, using permanent delete");
        }

        // Step 3: Permanent delete
        warn!("Permanently deleting file: {}", id);
        self.file_repository.delete_file(id).await?;
        info!("File permanently deleted: {}", id);

        if let Some(hash) = &content_hash {
            self.decrement_dedup_ref(hash).await;
        }

        Ok(false) // permanently deleted
    }
}
