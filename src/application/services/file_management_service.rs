use std::sync::Arc;

use crate::application::dtos::file_dto::FileDto;
use crate::application::ports::file_ports::FileManagementUseCase;
use crate::application::ports::storage_ports::{CopyFolderTreeResult, FileReadPort, FileWritePort};
use crate::application::ports::trash_ports::TrashUseCase;
use crate::application::services::trash_service::TrashService;
use crate::common::errors::DomainError;
use crate::infrastructure::repositories::pg::file_blob_read_repository::FileBlobReadRepository;
use crate::infrastructure::repositories::pg::file_blob_write_repository::FileBlobWriteRepository;
use crate::infrastructure::repositories::pg::folder_db_repository::FolderDbRepository;
use crate::infrastructure::services::thumbnail_service::ThumbnailService;
use tracing::{error, info, warn};
use uuid::Uuid;

/// Service for file management operations (move, delete).
///
/// Blob ref_count bookkeeping on deletion is handled by the PG trigger
/// `trg_files_decrement_blob_ref` (fires on DELETE FROM storage.files).
/// This service only orchestrates trash vs. permanent delete — it never
/// touches ref_count directly.
pub struct FileManagementService {
    file_repository: Arc<FileBlobWriteRepository>,
    file_read: Option<Arc<FileBlobReadRepository>>,
    folder_repo: Option<Arc<FolderDbRepository>>,
    trash_service: Option<Arc<TrashService>>,
    thumbnail_service: Option<Arc<ThumbnailService>>,
}

impl FileManagementService {
    /// Creates a new FileManagementService.
    pub fn new(file_repository: Arc<FileBlobWriteRepository>) -> Self {
        Self {
            file_repository,
            file_read: None,
            folder_repo: None,
            trash_service: None,
            thumbnail_service: None,
        }
    }

    /// Creates a FileManagementService with a trash service, read repo, and folder repo for ownership checks.
    pub fn with_trash(
        file_repository: Arc<FileBlobWriteRepository>,
        trash_service: Option<Arc<TrashService>>,
        file_read: Option<Arc<FileBlobReadRepository>>,
        folder_repo: Option<Arc<FolderDbRepository>>,
        thumbnail_service: Option<Arc<ThumbnailService>>,
    ) -> Self {
        Self {
            file_repository,
            file_read,
            folder_repo,
            trash_service,
            thumbnail_service,
        }
    }

    /// Verifies ownership via the read repository.
    async fn verify_owner(&self, file_id: &str, caller_id: Uuid) -> Result<(), DomainError> {
        if let Some(read) = &self.file_read {
            read.verify_file_owner(file_id, caller_id).await
        } else {
            // Fallback: no read repo injected — deny by default (fail-closed)
            Err(DomainError::internal_error(
                "FileManagement",
                "Ownership verification unavailable",
            ))
        }
    }

    /// Verifies that the target folder is owned by the caller.
    /// If folder_id is None (root), ownership is implicitly granted.
    async fn verify_target_folder_owner(
        &self,
        folder_id: &Option<String>,
        caller_id: Uuid,
    ) -> Result<(), DomainError> {
        let folder_id = match folder_id {
            Some(id) => id,
            None => return Ok(()), // Moving to root is always allowed
        };

        if let Some(folder_repo) = &self.folder_repo {
            let folder_owner = folder_repo.get_folder_user_id(folder_id).await?;
            if folder_owner != caller_id {
                return Err(DomainError::not_found(
                    "Folder",
                    "Target folder not found or access denied",
                ));
            }
            Ok(())
        } else {
            // Fallback: no folder repo injected — deny by default (fail-closed)
            Err(DomainError::internal_error(
                "FileManagement",
                "Folder ownership verification unavailable",
            ))
        }
    }
}

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

    async fn move_file_owned(
        &self,
        file_id: &str,
        caller_id: Uuid,
        folder_id: Option<String>,
    ) -> Result<FileDto, DomainError> {
        // Verify file ownership first
        self.verify_owner(file_id, caller_id).await?;
        // Verify target folder ownership (prevents file from "disappearing")
        self.verify_target_folder_owner(&folder_id, caller_id).await?;
        self.move_file(file_id, folder_id).await
    }

    async fn copy_file(
        &self,
        file_id: &str,
        target_folder_id: Option<String>,
    ) -> Result<FileDto, DomainError> {
        info!(
            "Copying file with ID: {} to folder: {:?}",
            file_id, target_folder_id
        );

        let copied_file = self
            .file_repository
            .copy_file(file_id, target_folder_id)
            .await
            .map_err(|e| {
                error!("Error copying file (ID: {}): {}", file_id, e);
                e
            })?;

        info!(
            "File copied successfully: {} (ID: {}) to folder: {:?}",
            copied_file.name(),
            copied_file.id(),
            copied_file.folder_id()
        );

        Ok(FileDto::from(copied_file))
    }

    async fn copy_file_owned(
        &self,
        file_id: &str,
        caller_id: Uuid,
        target_folder_id: Option<String>,
    ) -> Result<FileDto, DomainError> {
        self.verify_owner(file_id, caller_id).await?;
        self.copy_file(file_id, target_folder_id).await
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

    async fn rename_file_owned(
        &self,
        file_id: &str,
        caller_id: Uuid,
        new_name: &str,
    ) -> Result<FileDto, DomainError> {
        self.verify_owner(file_id, caller_id).await?;
        self.rename_file(file_id, new_name).await
    }

    async fn delete_file(&self, id: &str) -> Result<(), DomainError> {
        self.file_repository.delete_file(id).await?;
        // Best-effort thumbnail cleanup
        if let Some(thumb) = &self.thumbnail_service
            && let Err(e) = thumb.delete_thumbnails(id).await
        {
            warn!("Failed to delete thumbnails for file {}: {}", id, e);
        }
        Ok(())
    }

    async fn delete_file_owned(&self, id: &str, caller_id: Uuid) -> Result<(), DomainError> {
        self.verify_owner(id, caller_id).await?;
        self.delete_file(id).await
    }

    /// Smart delete: trash-first with dedup reference cleanup.
    ///
    /// Blob ref_count bookkeeping is handled entirely by the PG trigger
    /// `trg_files_decrement_blob_ref` which fires on DELETE FROM storage.files.
    /// We do NOT decrement here — trashing is a soft-delete (UPDATE, not DELETE)
    /// so the blob must remain referenced until the file is permanently deleted.
    async fn delete_with_cleanup(&self, id: &str, user_id: Uuid) -> Result<bool, DomainError> {
        // Step 1: Try trash (soft delete — file row stays, blob stays referenced)
        if let Some(trash) = &self.trash_service {
            info!("Moving file to trash: {}", id);
            match trash.move_to_trash(id, "file", user_id).await {
                Ok(_) => {
                    info!("File successfully moved to trash: {}", id);
                    // Do NOT decrement blob ref here — the file row still exists
                    // (is_trashed = TRUE). The trigger will decrement when the
                    // row is actually DELETEd during trash emptying.
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

        // Step 2: Permanent delete — trigger handles blob ref_count
        warn!("Permanently deleting file: {}", id);
        self.file_repository.delete_file(id).await?;
        // Best-effort thumbnail cleanup
        if let Some(thumb) = &self.thumbnail_service
            && let Err(e) = thumb.delete_thumbnails(id).await
        {
            warn!("Failed to delete thumbnails for file {}: {}", id, e);
        }
        info!("File permanently deleted: {}", id);

        Ok(false) // permanently deleted
    }

    async fn copy_folder_tree(
        &self,
        source_folder_id: &str,
        target_parent_id: Option<String>,
        dest_name: Option<String>,
    ) -> Result<CopyFolderTreeResult, DomainError> {
        info!(
            "Copying folder tree: source={}, target_parent={:?}, dest_name={:?}",
            source_folder_id, target_parent_id, dest_name
        );

        let result = self
            .file_repository
            .copy_folder_tree(source_folder_id, target_parent_id, dest_name)
            .await
            .map_err(|e| {
                error!(
                    "Error copying folder tree (source: {}): {}",
                    source_folder_id, e
                );
                e
            })?;

        info!(
            "Folder tree copied: {} folders, {} files (new root: {})",
            result.folders_copied, result.files_copied, result.new_root_folder_id
        );

        Ok(result)
    }
}
