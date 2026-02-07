use std::path::PathBuf;
use tokio::fs;
use tracing::{debug, error, instrument};

use crate::domain::repositories::file_repository::FileRepositoryResult;
use crate::infrastructure::repositories::file_fs_repository::FileFsRepository;

// This file contains the implementation of trash-related methods
// for the FileFsRepository file repository

// Implementation of trash methods for the file repository
impl FileFsRepository {
    // Gets the complete path to the trash directory
    fn get_trash_dir(&self) -> PathBuf {
        let trash_dir = self.get_root_path().join(".trash").join("files");
        debug!("Base trash directory: {}", trash_dir.display());
        trash_dir
    }
    
    // Gets the trash directory path for a specific user (if provided)
    fn get_user_trash_dir(&self, user_id: Option<&str>) -> PathBuf {
        let base_trash_dir = self.get_trash_dir();
        
        if let Some(uid) = user_id {
            let user_trash_dir = base_trash_dir.join(uid);
            debug!("User-specific trash directory: {}", user_trash_dir.display());
            user_trash_dir
        } else {
            // No user ID provided - this should not happen in production
            tracing::warn!("No user_id provided for trash directory, this indicates a bug");
            let default_dir = base_trash_dir.join("unknown-user");
            debug!("Fallback user trash directory: {}", default_dir.display());
            default_dir
        }
    }
    
    // Creates a unique path in the trash for the file
    async fn create_trash_file_path(&self, file_id: &str) -> FileRepositoryResult<PathBuf> {
        debug!("Creating trash file path for file ID: {}", file_id);
        
        // Get the trash directory for the default user
        let user_trash_dir = self.get_user_trash_dir(None);
        
        // Ensure the user's trash directory exists
        debug!("Ensuring user trash directory exists: {}", user_trash_dir.display());
        if !user_trash_dir.exists() {
            debug!("Creating user trash directory: {}", user_trash_dir.display());
            fs::create_dir_all(&user_trash_dir).await
                .map_err(|e| {
                    error!("Failed to create user trash directory: {}", e);
                    FileRepositoryError::IoError(e)
                })?;
            debug!("User trash directory created successfully");
        } else {
            debug!("User trash directory already exists");
        }
        
        // Create a unique path for the file in the trash
        let trash_file_path = user_trash_dir.join(file_id);
        debug!("Trash file path: {}", trash_file_path.display());
        
        Ok(trash_file_path)
    }
}

// Implementation of the public methods of the FileRepository trait related to trash
// Note: The FileRepository trait implementation has been moved to file_fs_repository.rs
// to avoid duplicate implementations

// Implementation of internal methods for trash functionality
impl FileFsRepository {
    /// Helper method that will be used for trash functionality 
    pub(crate) async fn _trash_move_to_trash(&self, file_id: &str) -> FileRepositoryResult<()> {
        debug!("Moving file to trash: {}", file_id);
        
        // Get the physical path of the file
        // We create an independent method to access the ID mapping service
        debug!("Getting file path with ID: {}", file_id);
        let file_path = match self.id_mapping_service().get_file_path(file_id).await {
            Ok(path) => {
                debug!("File path obtained: {}", path.display());
                path
            },
            Err(e) => {
                error!("Error getting file path {}: {:?}", file_id, e);
                return Err(FileRepositoryError::IdMappingError(format!("Failed to get file path: {}", e)));
            }
        };
        
        // Verify that the file exists
        debug!("Verifying that the file exists: {}", file_path.display());
        if !self.file_exists(&file_path).await? {
            error!("File not found at the specified path: {}", file_path.display());
            return Err(FileRepositoryError::NotFound(format!("File not found: {}", file_id)));
        }
        debug!("File found, continuing with the operation");
        
        // Create directory in trash if it doesn't exist
        debug!("Creating path for file in trash");
        let trash_file_path = self.create_trash_file_path(file_id).await?;
        debug!("Path in trash: {}", trash_file_path.display());
        
        // Physically move the file to trash (doesn't update mappings)
        debug!("Physically moving file to trash: {} -> {}", file_path.display(), trash_file_path.display());
        match fs::rename(&file_path, &trash_file_path).await {
            Ok(_) => {
                debug!("File successfully moved to trash: {} -> {}", file_path.display(), trash_file_path.display());
                
                // Invalidate the cache for the original file
                debug!("Invalidating cache for: {}", file_path.display());
                self.metadata_cache().invalidate(&file_path).await;
                
                // Update the mapping to the new path in trash
                debug!("Updating ID mapping to new path in trash");
                if let Err(e) = self.id_mapping_service().update_file_path(file_id, &trash_file_path).await {
                    error!("Error updating file mapping in trash: {}", e);
                    return Err(FileRepositoryError::MappingError(format!("Failed to update mapping: {}", e)));
                }
                debug!("Mapping successfully updated");
                
                debug!("Move to trash operation completed successfully for file: {}", file_id);
                Ok(())
            },
            Err(e) => {
                error!("Error moving file to trash: {} -> {}: {}", 
                       file_path.display(), trash_file_path.display(), e);
                Err(FileRepositoryError::IoError(e))
            }
        }
    }
    
    /// Restores a file from trash to its original location
    #[instrument(skip(self))]
    pub(crate) async fn _trash_restore_from_trash(&self, file_id: &str, original_path: &str) -> FileRepositoryResult<()> {
        debug!("Restoring file {} to {}", file_id, original_path);
        
        // Try to get the current path from the ID mapping service
        let current_path_result = self.id_mapping_service().get_file_path(file_id).await;
        
        match current_path_result {
            Ok(current_path) => {
                debug!("Current path in trash: {}", current_path.display());
                
                // Check if the file exists in the trash
                let file_exists = match fs::metadata(&current_path).await {
                    Ok(_) => {
                        debug!("File exists in trash");
                        true
                    },
                    Err(e) => {
                        debug!("File does not exist in trash: {} - {}", current_path.display(), e);
                        false
                    }
                };
                
                if !file_exists {
                    error!("The file does not physically exist in the trash: {}", current_path.display());
                    return Err(FileRepositoryError::NotFound(format!("File not found in trash: {}", file_id)));
                }
                
                // Parse the original path to a PathBuf
                let original_path_buf = PathBuf::from(original_path);
                debug!("Original path for restoration: {}", original_path_buf.display());
                
                // Check if a file already exists at the destination
                let target_exists = fs::metadata(&original_path_buf).await.is_ok();
                if target_exists {
                    debug!("A file already exists at the destination path, generating alternative path");
                    
                    // Generate a unique path by adding a suffix
                    // Extract filename and extension
                    let file_name = original_path_buf.file_name()
                        .map(|name| name.to_string_lossy().into_owned())
                        .unwrap_or_else(|| "restored_file".to_string());
                        
                    let parent_dir = original_path_buf.parent()
                        .unwrap_or_else(|| std::path::Path::new(""));
                        
                    let (stem, ext) = if let Some(dot_pos) = file_name.rfind('.') {
                        (file_name[..dot_pos].to_string(), file_name[dot_pos..].to_string())
                    } else {
                        (file_name, "".to_string())
                    };
                    
                    // Create a new name with a timestamp
                    let timestamp = chrono::Utc::now().timestamp();
                    let new_name = format!("{}_{}{}", stem, timestamp, ext);
                    
                    // Create the alternative path
                    let alternative_path = parent_dir.join(new_name);
                    debug!("Alternative path for restoration: {}", alternative_path.display());
                    
                    // Ensure the parent directory exists
                    if let Some(parent) = alternative_path.parent() {
                        if !parent.exists() {
                            debug!("Creating parent directory for restoration: {}", parent.display());
                            match fs::create_dir_all(parent).await {
                                Ok(_) => debug!("Parent directory created successfully"),
                                Err(e) => {
                                    error!("Error creating parent directory: {} - {}", parent.display(), e);
                                    return Err(FileRepositoryError::IoError(e));
                                }
                            }
                        }
                    }
                    
                    // Move the file from trash to the alternative location
                    debug!("Moving file from trash to alternative location: {} -> {}", 
                           current_path.display(), alternative_path.display());
                    match fs::rename(&current_path, &alternative_path).await {
                        Ok(_) => {
                            debug!("File successfully restored to alternative location");
                            
                            // Invalidate cache entries
                            debug!("Invalidating cache for file in trash");
                            self.metadata_cache().invalidate(&current_path).await;
                            
                            // Update the ID mapping
                            debug!("Updating ID mapping to new location");
                            if let Err(e) = self.id_mapping_service().update_file_path(file_id, &alternative_path).await {
                                error!("Error updating mapping of restored file: {}", e);
                                return Err(FileRepositoryError::MappingError(
                                    format!("Failed to update mapping: {}", e)
                                ));
                            }
                            
                            debug!("Restoration to alternative location completed successfully");
                            Ok(())
                        },
                        Err(e) => {
                            error!("Error restoring file to alternative location: {}", e);
                            Err(FileRepositoryError::IoError(e))
                        }
                    }
                } else {
                    // Ensure the parent directory exists
                    if let Some(parent) = original_path_buf.parent() {
                        if !parent.exists() {
                            debug!("Creating parent directory for restoration: {}", parent.display());
                            match fs::create_dir_all(parent).await {
                                Ok(_) => debug!("Parent directory created successfully"),
                                Err(e) => {
                                    error!("Error creating parent directory: {} - {}", parent.display(), e);
                                    return Err(FileRepositoryError::IoError(e));
                                }
                            }
                        }
                    }
                    
                    // Move the file from trash to its original location
                    debug!("Moving file from trash to original location: {} -> {}", 
                           current_path.display(), original_path_buf.display());
                    match fs::rename(&current_path, &original_path_buf).await {
                        Ok(_) => {
                            debug!("File successfully restored to original location");
                            
                            // Invalidate cache entries
                            debug!("Invalidating cache for file in trash");
                            self.metadata_cache().invalidate(&current_path).await;
                            
                            // Update the ID mapping
                            debug!("Updating ID mapping to original location");
                            if let Err(e) = self.id_mapping_service().update_file_path(file_id, &original_path_buf).await {
                                error!("Error updating mapping of restored file: {}", e);
                                return Err(FileRepositoryError::MappingError(
                                    format!("Failed to update mapping: {}", e)
                                ));
                            }
                            
                            debug!("Restoration to original location completed successfully");
                            Ok(())
                        },
                        Err(e) => {
                            error!("Error restoring file to original location: {}", e);
                            Err(FileRepositoryError::IoError(e))
                        }
                    }
                }
            },
            Err(e) => {
                error!("Error getting current path of file {}: {:?}", file_id, e);
                
                // Check if the error is because the ID was not found
                if format!("{}", e).contains("not found") {
                    debug!("ID not found in mapping, file no longer exists in trash");
                    return Err(FileRepositoryError::NotFound(format!("File not found in trash: {}", file_id)));
                }
                
                return Err(FileRepositoryError::IdMappingError(
                    format!("Failed to get file path: {}", e)
                ));
            }
        }
    }
    
    /// Permanently deletes a file (used by trash)
    #[instrument(skip(self))]
    pub(crate) async fn _trash_delete_file_permanently(&self, file_id: &str) -> FileRepositoryResult<()> {
        debug!("Permanently deleting file: {}", file_id);
        
        // Get the file path using the ID mapping service
        let file_path_result = self.id_mapping_service().get_file_path(file_id).await;
        
        match file_path_result {
            Ok(file_path) => {
                debug!("Found path for file: {} -> {}", file_id, file_path.display());
                
                // Check if the file physically exists before attempting to delete
                let file_exists = fs::metadata(&file_path).await.is_ok();
                
                if file_exists {
                    debug!("File exists physically, deleting: {}", file_path.display());
                    
                    // Delete the file physically
                    if let Err(e) = fs::remove_file(&file_path).await {
                        error!("Error permanently deleting file: {} - {}", file_path.display(), e);
                        // Don't report error if the file already doesn't exist
                        if e.kind() != std::io::ErrorKind::NotFound {
                            return Err(FileRepositoryError::IoError(e));
                        }
                    } else {
                        debug!("File physically deleted successfully");
                    }
                    
                    // Invalidate cache for this file
                    debug!("Invalidating cache for file: {}", file_path.display());
                    self.metadata_cache().invalidate(&file_path).await;
                } else {
                    debug!("File does not exist physically, only cleaning mappings: {}", file_path.display());
                }
                
                // Always remove the ID mapping regardless of whether the file exists
                debug!("Removing ID mapping: {}", file_id);
                match self.id_mapping_service().remove_id(file_id).await {
                    Ok(_) => debug!("ID mapping successfully removed"),
                    Err(e) => {
                        error!("Error removing file mapping: {}", e);
                        // Only return error for critical mapping errors, otherwise continue
                        if format!("{}", e).contains("not found") {
                            debug!("ID mapping not found, ignoring this error for deletion");
                        } else {
                            return Err(FileRepositoryError::MappingError(format!("Failed to remove mapping: {}", e)));
                        }
                    }
                };
                
                debug!("File permanently deleted successfully: {}", file_id);
                Ok(())
            },
            Err(e) => {
                // This could happen if the file is already deleted or wasn't properly indexed
                error!("Error getting file path {}: {:?}", file_id, e);
                
                // Check if the error is because the ID was not found
                if format!("{}", e).contains("not found") {
                    debug!("ID not found in mapping, considering deletion successful: {}", file_id);
                    // In this case, we consider the file already deleted
                    return Ok(());
                }
                
                return Err(FileRepositoryError::IdMappingError(format!("Failed to get file path: {}", e)));
            }
        }
    }
}

// Re-exports needed for the compiler
use crate::domain::repositories::file_repository::FileRepositoryError;