use std::path::PathBuf;
use tokio::fs;
use tracing::{debug, error};

use crate::infrastructure::repositories::folder_fs_repository::FolderFsRepository;
use crate::infrastructure::repositories::repository_errors::FolderRepositoryResult;

// This file contains the implementation of trash-related methods
// for the FolderFsRepository folder repository

// Implementation of trash methods for the folder repository
impl FolderFsRepository {
    // Gets the full path to the trash directory
    fn get_trash_dir(&self) -> PathBuf {
        self.get_root_path().join(".trash").join("folders")
    }

    // Creates a unique path in the trash for the folder
    async fn create_trash_folder_path(&self, folder_id: &str) -> FolderRepositoryResult<PathBuf> {
        let trash_dir = self.get_trash_dir();

        // Ensure the trash directory exists
        if !trash_dir.exists() {
            fs::create_dir_all(&trash_dir)
                .await
                .map_err(|e| FolderRepositoryError::StorageError(e.to_string()))?;
        }

        // Create a unique path for the folder in the trash
        Ok(trash_dir.join(folder_id))
    }
}

// Implementation of public FolderRepository trait methods related to trash
// Implementation of internal methods for trash functionality
// These will be enabled when the trash feature is re-enabled
impl FolderFsRepository {
    /// Helper method that will be used for trash functionality
    pub(crate) async fn _trash_move_to_trash(&self, folder_id: &str) -> FolderRepositoryResult<()> {
        debug!("Moving folder to trash: {}", folder_id);

        // Get the physical path of the folder
        let folder_path = match self.get_mapped_folder_path(folder_id).await {
            Ok(path) => path,
            Err(e) => {
                error!("Error getting folder path {}: {:?}", folder_id, e);
                return Err(e);
            }
        };

        let folder_path_buf = PathBuf::from(folder_path.to_string());

        // Verify the folder exists
        if !folder_path_buf.exists() {
            return Err(FolderRepositoryError::NotFound(format!(
                "Folder not found: {}",
                folder_id
            )));
        }

        // Create directory in the trash
        let trash_folder_path = self.create_trash_folder_path(folder_id).await?;

        // Physically move the folder to the trash
        match fs::rename(&folder_path_buf, &trash_folder_path).await {
            Ok(_) => {
                debug!(
                    "Folder moved to trash: {} -> {}",
                    folder_path_buf.display(),
                    trash_folder_path.display()
                );

                // Update the mapping to the new path in the trash
                if let Err(e) = self
                    .update_mapped_folder_path(folder_id, &trash_folder_path)
                    .await
                {
                    error!("Error updating folder mapping in trash: {}", e);
                    return Err(e);
                }

                Ok(())
            }
            Err(e) => {
                error!("Error moving folder to trash: {}", e);
                Err(FolderRepositoryError::StorageError(e.to_string()))
            }
        }
    }

    /// Restores a folder from the trash to its original location
    pub(crate) async fn _trash_restore_from_trash(
        &self,
        folder_id: &str,
        original_path: &str,
    ) -> FolderRepositoryResult<()> {
        debug!("Restoring folder {} to {}", folder_id, original_path);

        // Get the current path in the trash
        let current_path = match self.get_mapped_folder_path(folder_id).await {
            Ok(path) => PathBuf::from(path),
            Err(e) => {
                error!("Error getting current folder path {}: {:?}", folder_id, e);
                return Err(e);
            }
        };

        // Convert the original path to PathBuf
        let original_path_buf = PathBuf::from(original_path);

        // Ensure the destination parent directory exists
        if let Some(parent) = original_path_buf.parent()
            && !parent.exists()
        {
            fs::create_dir_all(parent).await.map_err(|e| {
                error!("Error creating parent directory for restoration: {}", e);
                FolderRepositoryError::StorageError(e.to_string())
            })?;
        }

        // Move the folder from the trash to its original location
        match fs::rename(&current_path, &original_path_buf).await {
            Ok(_) => {
                debug!(
                    "Folder restored: {} -> {}",
                    current_path.display(),
                    original_path_buf.display()
                );

                // Update the mapping to the original path
                if let Err(e) = self
                    .update_mapped_folder_path(folder_id, &original_path_buf)
                    .await
                {
                    error!("Error updating restored folder mapping: {}", e);
                    return Err(e);
                }

                Ok(())
            }
            Err(e) => {
                error!("Error restoring folder: {}", e);
                Err(FolderRepositoryError::StorageError(e.to_string()))
            }
        }
    }

    /// Permanently deletes a folder (used by the trash)
    pub(crate) async fn _trash_delete_folder_permanently(
        &self,
        folder_id: &str,
    ) -> FolderRepositoryResult<()> {
        debug!("Permanently deleting folder: {}", folder_id);

        // Similar to delete_folder but without additional validations
        let folder_path = match self.get_mapped_folder_path(folder_id).await {
            Ok(path) => PathBuf::from(path),
            Err(e) => {
                error!("Error getting folder path {}: {:?}", folder_id, e);
                return Err(e);
            }
        };

        // Delete the folder recursively
        if folder_path.exists() {
            match fs::remove_dir_all(&folder_path).await {
                Ok(_) => {
                    debug!("Folder permanently deleted: {}", folder_path.display());
                }
                Err(e) => {
                    error!("Error permanently deleting folder: {}", e);
                    // Don't report error if the folder no longer exists
                    if e.kind() != std::io::ErrorKind::NotFound {
                        return Err(FolderRepositoryError::StorageError(e.to_string()));
                    }
                }
            }
        }

        // Remove the mapping
        if let Err(e) = self.remove_mapped_folder_id(folder_id).await {
            error!("Error removing folder mapping: {}", e);
            return Err(e);
        }

        debug!("Folder permanently deleted successfully: {}", folder_id);
        Ok(())
    }
}

// Re-exports needed by the compiler
use crate::infrastructure::repositories::repository_errors::FolderRepositoryError;
