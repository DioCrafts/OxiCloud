use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use tracing::{debug, error, info, instrument};

use crate::application::dtos::trash_dto::TrashedItemDto;
use crate::application::ports::trash_ports::TrashUseCase;
use crate::application::ports::storage_ports::{FileReadPort, FileWritePort};
use crate::application::ports::outbound::FolderStoragePort;
use crate::common::errors::{Result, DomainError, ErrorKind};
use crate::domain::entities::trashed_item::{TrashedItem, TrashedItemType};
use crate::domain::repositories::trash_repository::TrashRepository;

/**
 * Application service for trash operations.
 * 
 * The TrashService implements the trash management functionality in the application layer,
 * handling movement of files and folders to trash, restoration from trash, and permanent
 * deletion. It orchestrates interactions between the domain entities and infrastructure
 * repositories while enforcing business rules like retention policies.
 * 
 * This service follows the Clean Architecture pattern by:
 * - Depending on application ports rather than domain/infrastructure traits
 * - Orchestrating domain operations without containing domain logic
 * - Exposing its functionality through the TrashUseCase port
 */
pub struct TrashService {
    /// Repository for trash-specific operations like listing and retrieving trashed items
    trash_repository: Arc<dyn TrashRepository>,
    
    /// Port for file read operations (get file metadata)
    file_read_port: Arc<dyn FileReadPort>,
    
    /// Port for file write operations (trash, restore, delete)
    file_write_port: Arc<dyn FileWritePort>,
    
    /// Port for folder operations (get folder, trash, restore, delete)
    folder_storage_port: Arc<dyn FolderStoragePort>,
    
    /// Number of days items should be kept in trash before automatic cleanup
    retention_days: u32,
}

impl TrashService {
    pub fn new(
        trash_repository: Arc<dyn TrashRepository>,
        file_read_port: Arc<dyn FileReadPort>,
        file_write_port: Arc<dyn FileWritePort>,
        folder_storage_port: Arc<dyn FolderStoragePort>,
        retention_days: u32,
    ) -> Self {
        Self {
            trash_repository,
            file_read_port,
            file_write_port,
            folder_storage_port,
            retention_days,
        }
    }

    /// Converts a TrashedItem entity to a DTO
    fn to_dto(&self, item: TrashedItem) -> TrashedItemDto {
        // Calculate days_until_deletion before moving item fields
        let days_until_deletion = item.days_until_deletion();
        
        TrashedItemDto {
            id: item.id().to_string(),
            original_id: item.original_id().to_string(),
            item_type: match item.item_type() {
                TrashedItemType::File => "file".to_string(),
                TrashedItemType::Folder => "folder".to_string(),
            },
            name: item.name().to_string(),
            original_path: item.original_path().to_string(),
            trashed_at: item.trashed_at(),
            days_until_deletion,
        }
    }

    /// Validates that the given user owns the trashed item.
    /// Returns an error if the item does not exist or belongs to a different user.
    #[instrument(skip(self))]
    async fn validate_user_ownership(&self, item_id: &str, user_id: &str) -> Result<()> {
        let item_uuid = Uuid::parse_str(item_id)
            .map_err(|e| DomainError::validation_error(format!("Invalid item ID: {}", e)))?;
        let user_uuid = Uuid::parse_str(user_id)
            .map_err(|e| DomainError::validation_error(format!("Invalid user ID: {}", e)))?;

        match self.trash_repository.get_trash_item(&item_uuid, &user_uuid).await? {
            Some(item) => {
                if item.user_id() != user_uuid {
                    error!(
                        "User {} attempted to access trash item {} owned by {}",
                        user_id, item_id, item.user_id()
                    );
                    return Err(DomainError::access_denied(
                        "TrashItem",
                        "You do not have permission to access this trash item",
                    ));
                }
                Ok(())
            }
            None => {
                // Item not found for this user — treat as authorization error
                // to avoid leaking existence information
                Err(DomainError::not_found(
                    "TrashItem",
                    format!("{} (user: {})", item_id, user_id),
                ))
            }
        }
    }
}

#[async_trait]
impl TrashUseCase for TrashService {
    #[instrument(skip(self))]
    async fn get_trash_items(&self, user_id: &str) -> Result<Vec<TrashedItemDto>> {
        debug!("Getting trash items for user: {}", user_id);
        
        let user_uuid = Uuid::parse_str(user_id)
            .map_err(|e| DomainError::validation_error(format!("Invalid user ID: {}", e)))?;
            
        let items = self.trash_repository.get_trash_items(&user_uuid).await?;
        
        let dtos = items.into_iter()
            .map(|item| self.to_dto(item))
            .collect();
            
        Ok(dtos)
    }

    #[instrument(skip(self))]
    async fn move_to_trash(&self, item_id: &str, item_type: &str, user_id: &str) -> Result<()> {
        info!("Moving to trash: type={}, id={}, user={}", item_type, item_id, user_id);
        debug!("User UUID validation: {}", user_id);
        
        // Note: We do NOT call validate_user_ownership here because the item
        // is not yet in the trash. Ownership validation is only for operations
        // on already-trashed items (restore, delete_permanently).
        
        // Parse UUIDs with detailed error handling
        debug!("Validating item UUID: {}", item_id);
        let item_uuid = match Uuid::parse_str(item_id) {
            Ok(uuid) => {
                debug!("Valid item UUID: {}", uuid);
                uuid
            },
            Err(e) => {
                error!("Invalid item UUID: {} - Error: {}", item_id, e);
                return Err(DomainError::validation_error(format!("Invalid item ID: {}", e)));
            }
        };
        
        debug!("Validating user UUID: {}", user_id);
        let user_uuid = match Uuid::parse_str(user_id) {
            Ok(uuid) => {
                debug!("Valid user UUID: {}", uuid);
                uuid
            },
            Err(e) => {
                error!("Invalid user UUID: {} - Error: {}", user_id, e);
                return Err(DomainError::validation_error(format!("Invalid user ID: {}", e)));
            }
        };
        
        match item_type {
            "file" => {
                info!("Processing file to move to trash: {}", item_id);
                
                // Get the file to verify it exists and capture its data
                debug!("Getting file data: {}", item_id);
                let file = match self.file_read_port.get_file(item_id).await {
                    Ok(file) => {
                        debug!("File found: {} ({})", file.name(), item_id);
                        file
                    },
                    Err(e) => {
                        error!("Error getting file: {} - {}", item_id, e);
                        return Err(DomainError::new(
                            ErrorKind::NotFound,
                            "File",
                            format!("Error retrieving file {}: {}", item_id, e)
                        ));
                    }
                };
                
                let original_path = file.storage_path().to_string();
                debug!("Original file path: {}", original_path);
                
                // Create the trash item
                debug!("Creating TrashedItem object for the file");
                let trashed_item = TrashedItem::new(
                    item_uuid,
                    user_uuid,
                    TrashedItemType::File,
                    file.name().to_string(),
                    original_path,
                    self.retention_days,
                );
                debug!("TrashedItem created successfully: {} -> {}", file.name(), trashed_item.id());
                
                // First add to trash index to register the item
                info!("Adding file {} to trash index", item_id);
                match self.trash_repository.add_to_trash(&trashed_item).await {
                    Ok(_) => {
                        debug!("File added to trash index successfully");
                    },
                    Err(e) => {
                        error!("Error adding file to trash index: {}", e);
                        return Err(DomainError::internal_error("TrashRepository", format!("Failed to add file to trash: {}", e)));
                    }
                };
                
                // Then physically move the file to trash
                info!("Physically moving file to trash: {}", item_id);
                match self.file_write_port.move_to_trash(item_id).await {
                    Ok(_) => {
                        debug!("File physically moved to trash successfully: {}", item_id);
                    },
                    Err(e) => {
                        error!("Error physically moving file to trash: {} - {}", item_id, e);
                        return Err(DomainError::new(
                            ErrorKind::InternalError,
                            "File",
                            format!("Error moving file {} to trash: {}", item_id, e)
                        ));
                    }
                }
                
                info!("File completely moved to trash: {}", item_id);
                Ok(())
            },
            "folder" => {
                // Get the folder to verify it exists and capture its data
                let folder = self.folder_storage_port.get_folder(item_id).await
                    .map_err(|e| DomainError::new(
                        ErrorKind::NotFound,
                        "Folder",
                        format!("Error retrieving folder {}: {}", item_id, e)
                    ))?;
                
                let original_path = folder.storage_path().to_string();
                
                // Create the trash item
                let trashed_item = TrashedItem::new(
                    item_uuid,
                    user_uuid,
                    TrashedItemType::Folder,
                    folder.name().to_string(),
                    original_path,
                    self.retention_days,
                );
                
                // First add to trash index to register the item
                debug!("Adding folder {} to trash repository", item_id);
                match self.trash_repository.add_to_trash(&trashed_item).await {
                    Ok(_) => debug!("Successfully added folder to trash repository"),
                    Err(e) => {
                        error!("Failed to add folder to trash repository: {}", e);
                        return Err(DomainError::internal_error("TrashRepository", format!("Failed to add folder to trash: {}", e)));
                    }
                };
                
                // Then physically move the folder to trash
                self.folder_storage_port.move_to_trash(item_id).await
                    .map_err(|e| DomainError::new(
                        ErrorKind::InternalError,
                        "Folder",
                        format!("Error moving folder {} to trash: {}", item_id, e)
                    ))?;
                
                debug!("Folder moved to trash: {}", item_id);
                Ok(())
            },
            _ => Err(DomainError::validation_error(format!("Invalid item type: {}", item_type))),
        }
    }

    #[instrument(skip(self))]
    async fn restore_item(&self, trash_id: &str, user_id: &str) -> Result<()> {
        info!("Restoring item {} for user {}", trash_id, user_id);
        
        let trash_uuid = match Uuid::parse_str(trash_id) {
            Ok(id) => {
                info!("Trash UUID parsed successfully: {}", id);
                id
            },
            Err(e) => {
                error!("Invalid trash ID format: {} - {}", trash_id, e);
                return Err(DomainError::validation_error(format!("Invalid trash ID: {}", e)));
            }
        };
            
        let user_uuid = match Uuid::parse_str(user_id) {
            Ok(id) => {
                info!("User UUID parsed successfully: {}", id);
                id
            },
            Err(e) => {
                error!("Invalid user ID format: {} - {}", user_id, e);
                return Err(DomainError::validation_error(format!("Invalid user ID: {}", e)));
            }
        };
        
        // Get the trash item
        info!("Retrieving trash item from repository: ID={}", trash_id);
        let item_result = self.trash_repository.get_trash_item(&trash_uuid, &user_uuid).await;

        match item_result {
            Ok(Some(item)) => {
                info!("Found item in trash: ID={}, Type={:?}, OriginalID={}",
                    trash_id, item.item_type(), item.original_id());

                // Restore based on type
                match item.item_type() {
                    TrashedItemType::File => {
                        // Restore the file to its original location
                        let file_id = item.original_id().to_string();
                        let original_path = item.original_path().to_string();
                        
                        info!("Restoring file from trash: ID={}, OriginalPath={}", file_id, original_path);
                        match self.file_write_port.restore_from_trash(&file_id, &original_path).await {
                            Ok(_) => {
                                info!("Successfully restored file from trash: {}", file_id);
                            },
                            Err(e) => {
                                // Check if the error is because the file is not found
                                if format!("{}", e).contains("not found") {
                                    info!("File not found in trash, may already have been restored: {}", file_id);
                                    // We continue so we can clean up the trash entry
                                } else {
                                    // Return error for other kinds of errors
                                    error!("Error restoring file from trash: {} - {}", file_id, e);
                                    return Err(DomainError::new(
                                        ErrorKind::InternalError,
                                        "File",
                                        format!("Error restoring file {} from trash: {}", file_id, e)
                                    ));
                                }
                            }
                        }
                    },
                    TrashedItemType::Folder => {
                        // Restore the folder to its original location
                        let folder_id = item.original_id().to_string();
                        let original_path = item.original_path().to_string();
                        
                        info!("Restoring folder from trash: ID={}, OriginalPath={}", folder_id, original_path);
                        match self.folder_storage_port.restore_from_trash(&folder_id, &original_path).await {
                            Ok(_) => {
                                info!("Successfully restored folder from trash: {}", folder_id);
                            },
                            Err(e) => {
                                // Check if the error is because the folder is not found
                                if format!("{}", e).contains("not found") {
                                    info!("Folder not found in trash, may already have been restored: {}", folder_id);
                                    // We continue so we can clean up the trash entry
                                } else {
                                    // Return error for other kinds of errors
                                    error!("Error restoring folder from trash: {} - {}", folder_id, e);
                                    return Err(DomainError::new(
                                        ErrorKind::InternalError,
                                        "Folder",
                                        format!("Error restoring folder {} from trash: {}", folder_id, e)
                                    ));
                                }
                            }
                        }
                    }
                }
                
                // Always remove the item from the trash index to maintain consistency
                info!("Removing item from trash index after restoration: {}", trash_id);
                match self.trash_repository.restore_from_trash(&trash_uuid, &user_uuid).await {
                    Ok(_) => {
                        info!("Successfully removed entry from trash index: {}", trash_id);
                    },
                    Err(e) => {
                        error!("Error removing entry from trash index: {} - {}", trash_id, e);
                        return Err(DomainError::new(
                            ErrorKind::InternalError,
                            "Trash",
                            format!("Error removing trash entry after restoration: {}", e)
                        ));
                    }
                }
                
                info!("Item successfully restored from trash: {}", trash_id);
                Ok(())
            },
            Ok(None) => {
                // If the item isn't found in trash, we can just return success
                info!("Item not found in trash index, considering as already restored: {}", trash_id);
                Ok(())
            },
            Err(e) => {
                // Something went wrong with the repository
                error!("Error retrieving item from trash repository: {} - {}", trash_id, e);
                Err(e)
            }
        }
    }

    #[instrument(skip(self))]
    async fn delete_permanently(&self, trash_id: &str, user_id: &str) -> Result<()> {
        info!("Permanently deleting item {} for user {}", trash_id, user_id);
        
        let trash_uuid = match Uuid::parse_str(trash_id) {
            Ok(id) => {
                info!("Trash UUID parsed successfully: {}", id);
                id
            },
            Err(e) => {
                error!("Invalid trash ID format: {} - {}", trash_id, e);
                return Err(DomainError::validation_error(format!("Invalid trash ID: {}", e)));
            }
        };
            
        let user_uuid = match Uuid::parse_str(user_id) {
            Ok(id) => {
                info!("User UUID parsed successfully: {}", id);
                id
            },
            Err(e) => {
                error!("Invalid user ID format: {} - {}", user_id, e);
                return Err(DomainError::validation_error(format!("Invalid user ID: {}", e)));
            }
        };
        
        // Get the trash item
        info!("Retrieving trash item from repository: ID={}", trash_id);
        let item_result = self.trash_repository.get_trash_item(&trash_uuid, &user_uuid).await;
        
        match item_result {
            Ok(Some(item)) => {
                info!("Found item in trash: ID={}, Type={:?}, OriginalID={}", 
                      trash_id, item.item_type(), item.original_id());
                
                // Permanently delete based on type
                match item.item_type() {
                    TrashedItemType::File => {
                        // Permanently delete the file
                        let file_id = item.original_id().to_string();
                        
                        info!("Permanently deleting file: {}", file_id);
                        match self.file_write_port.delete_file_permanently(&file_id).await {
                            Ok(_) => {
                                info!("Successfully deleted file permanently: {}", file_id);
                            },
                            Err(e) => {
                                // Check if the file is not found - in that case, we can continue
                                // because we still want to remove the item from the trash index
                                if format!("{}", e).contains("not found") {
                                    info!("File not found, may already have been deleted: {}", file_id);
                                } else {
                                    // Return error for other types of errors
                                    error!("Error permanently deleting file: {} - {}", file_id, e);
                                    return Err(DomainError::new(
                                        ErrorKind::InternalError,
                                        "File",
                                        format!("Error deleting file {} permanently: {}", file_id, e)
                                    ));
                                }
                            }
                        }
                    },
                    TrashedItemType::Folder => {
                        // Permanently delete the folder
                        let folder_id = item.original_id().to_string();
                        
                        info!("Permanently deleting folder: {}", folder_id);
                        match self.folder_storage_port.delete_folder_permanently(&folder_id).await {
                            Ok(_) => {
                                info!("Successfully deleted folder permanently: {}", folder_id);
                            },
                            Err(e) => {
                                // Check if the folder is not found - in that case, we can continue
                                if format!("{}", e).contains("not found") {
                                    info!("Folder not found, may already have been deleted: {}", folder_id);
                                } else {
                                    // Return error for other types of errors
                                    error!("Error permanently deleting folder: {} - {}", folder_id, e);
                                    return Err(DomainError::new(
                                        ErrorKind::InternalError,
                                        "Folder",
                                        format!("Error deleting folder {} permanently: {}", folder_id, e)
                                    ));
                                }
                            }
                        }
                    }
                }
                
                // Always remove the item from trash index to maintain consistency
                info!("Removing entry from trash index: {}", trash_id);
                match self.trash_repository.delete_permanently(&trash_uuid, &user_uuid).await {
                    Ok(_) => {
                        info!("Successfully removed entry from trash index: {}", trash_id);
                    },
                    Err(e) => {
                        error!("Error removing entry from trash index: {} - {}", trash_id, e);
                        return Err(DomainError::new(
                            ErrorKind::InternalError,
                            "Trash",
                            format!("Error removing trash entry: {}", e)
                        ));
                    }
                };
                
                info!("Item permanently deleted from trash: {}", trash_id);
                Ok(())
            },
            Ok(None) => {
                // If the item isn't found in trash, we can just return success
                info!("Item not found in trash, considering as already deleted: {}", trash_id);
                Ok(())
            },
            Err(e) => {
                // Something went wrong with the repository
                error!("Error retrieving item from trash repository: {} - {}", trash_id, e);
                Err(e)
            }
        }
    }

    #[instrument(skip(self))]
    async fn empty_trash(&self, user_id: &str) -> Result<()> {
        info!("Emptying trash for user {}", user_id);
        
        let user_uuid = Uuid::parse_str(user_id)
            .map_err(|e| DomainError::validation_error(format!("Invalid user ID: {}", e)))?;
        
        // Get all items in the user's trash
        let items = self.trash_repository.get_trash_items(&user_uuid).await?;
        
        // Permanently delete each item
        for item in items {
            match item.item_type() {
                TrashedItemType::File => {
                    // Permanently delete the file
                    let file_id = item.original_id().to_string();
                    if let Err(e) = self.file_write_port.delete_file_permanently(&file_id).await {
                        error!("Error permanently deleting file {}: {}", file_id, e);
                    }
                },
                TrashedItemType::Folder => {
                    // Permanently delete the folder
                    let folder_id = item.original_id().to_string();
                    if let Err(e) = self.folder_storage_port.delete_folder_permanently(&folder_id).await {
                        error!("Error permanently deleting folder {}: {}", folder_id, e);
                    }
                }
            }
        }
        
        // Clear all trash records for this user
        self.trash_repository.clear_trash(&user_uuid).await?;
        
        info!("Trash completely emptied for user {}", user_id);
        Ok(())
    }
}