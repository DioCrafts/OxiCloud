use crate::application::ports::auth_ports::UserStoragePort;
use crate::application::ports::storage_ports::{FileReadPort, StorageUsagePort};
use crate::common::errors::DomainError;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::task;
use tracing::{debug, error, info};

/**
 * Service for managing and updating user storage usage statistics.
 *
 * This service is responsible for calculating how much storage each user
 * is using and updating this information in the user records.
 */
pub struct StorageUsageService {
    file_repository: Arc<dyn FileReadPort>,
    user_repository: Arc<dyn UserStoragePort>,
}

impl StorageUsageService {
    /// Creates a new storage usage service
    pub fn new(
        file_repository: Arc<dyn FileReadPort>,
        user_repository: Arc<dyn UserStoragePort>,
    ) -> Self {
        Self {
            file_repository,
            user_repository,
        }
    }

    /// Calculates and updates storage usage for a specific user
    pub async fn update_user_storage_usage(&self, user_id: &str) -> Result<i64, DomainError> {
        info!("Updating storage usage for user: {}", user_id);

        // Get user's home folder pattern
        let user = self.user_repository.get_user_by_id(user_id).await?;
        let username = user.username();

        // Calculate storage usage for this user
        let total_usage = self.calculate_user_storage_usage(username).await?;

        // Update the user's storage usage in the database
        self.user_repository
            .update_storage_usage(user_id, total_usage)
            .await?;

        info!(
            "Updated storage usage for user {} to {} bytes",
            user_id, total_usage
        );

        Ok(total_usage)
    }

    /// Calculates a user's storage usage based on their home folder
    async fn calculate_user_storage_usage(&self, username: &str) -> Result<i64, DomainError> {
        debug!("Calculating storage for user: {}", username);

        // First, try to find the user's home folder
        // List all folders to locate the user's folder
        let all_folders = self
            .file_repository
            .list_files(None)
            .await
            .map_err(|e| DomainError::internal_error("File repository", e.to_string()))?;

        // Find the user's home folder (named "My Folder - {username}")
        let home_folder_name = format!("My Folder - {}", username);
        debug!("Looking for home folder: {}", home_folder_name);

        let mut total_usage: i64 = 0;
        let mut home_folder_id = None;

        // Find the home folder ID
        for folder in &all_folders {
            if folder.name() == home_folder_name {
                home_folder_id = Some(folder.id().to_string());
                debug!(
                    "Found home folder for user {}: ID={}",
                    username,
                    folder.id()
                );
                break;
            }
        }

        // If we found the home folder, calculate total size
        if let Some(folder_id) = home_folder_id {
            // Calculate recursively
            total_usage = self.calculate_folder_size(&folder_id).await?;
        } else {
            // If no home folder found, just return 0
            debug!("No home folder found for user: {}", username);
        }

        Ok(total_usage)
    }

    /// Recursively calculates the size of a folder and all its contents
    async fn calculate_folder_size(&self, folder_id: &str) -> Result<i64, DomainError> {
        // Implementation with explicit boxing to handle recursion in async functions
        async fn inner_calculate_size(
            repo: Arc<dyn FileReadPort>,
            folder_id: &str,
        ) -> Result<i64, DomainError> {
            let mut total_size: i64 = 0;

            // Get files directly in this folder
            let files = repo
                .list_files(Some(folder_id))
                .await
                .map_err(|e| DomainError::internal_error("File repository", e.to_string()))?;

            // Sum the size of all files
            for file in &files {
                // Skip subdirectories at this level - we'll process them separately
                if file.mime_type() == "directory" || file.mime_type() == "application/directory" {
                    // Recursively calculate subfolder size with explicit boxing
                    let subfolder_id = file.id().to_string(); // Create owned copy
                    let repo_clone = repo.clone(); // Clone the repository

                    // Use Box::pin to handle recursive async call
                    let subfolder_size_future =
                        Box::pin(inner_calculate_size(repo_clone, &subfolder_id));

                    match subfolder_size_future.await {
                        Ok(size) => {
                            total_size += size;
                        }
                        Err(e) => {
                            error!(
                                "Error calculating size for subfolder {}: {}",
                                subfolder_id, e
                            );
                            // Continue with other folders even if one fails
                        }
                    }
                } else {
                    // Add file size to total
                    total_size += file.size() as i64;
                }
            }

            Ok(total_size)
        }

        // Start the calculation with a clone of our repository reference
        let repo_clone = Arc::clone(&self.file_repository);
        inner_calculate_size(repo_clone, folder_id).await
    }
}

/**
 * Implementation of the StorageUsagePort trait to expose storage usage services
 * to the application layer.
 */
#[async_trait]
impl StorageUsagePort for StorageUsageService {
    async fn update_user_storage_usage(&self, user_id: &str) -> Result<i64, DomainError> {
        StorageUsageService::update_user_storage_usage(self, user_id).await
    }

    async fn update_all_users_storage_usage(&self) -> Result<(), DomainError> {
        info!("Starting batch update of all users' storage usage");

        // Get the list of all users
        let users = self.user_repository.list_users(1000, 0).await?;

        let mut update_tasks = Vec::new();

        // Process users in parallel
        for user in users {
            let user_id = user.id().to_string();
            let service_clone = self.clone();

            // Spawn a background task for each user
            let task = task::spawn(async move {
                match service_clone.update_user_storage_usage(&user_id).await {
                    Ok(usage) => {
                        debug!(
                            "Updated storage usage for user {}: {} bytes",
                            user_id, usage
                        );
                        Ok(())
                    }
                    Err(e) => {
                        error!("Failed to update storage for user {}: {}", user_id, e);
                        Err(e)
                    }
                }
            });

            update_tasks.push(task);
        }

        // Wait for all tasks to complete
        for task in update_tasks {
            // We don't propagate errors from individual users to avoid failing the entire batch
            let _ = task.await;
        }

        info!("Completed batch update of all users' storage usage");
        Ok(())
    }
}

// Make StorageUsageService cloneable to support spawning concurrent tasks
impl Clone for StorageUsageService {
    fn clone(&self) -> Self {
        Self {
            file_repository: Arc::clone(&self.file_repository),
            user_repository: Arc::clone(&self.user_repository),
        }
    }
}
