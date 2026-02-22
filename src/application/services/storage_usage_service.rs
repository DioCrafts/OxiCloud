use crate::application::ports::auth_ports::UserStoragePort;
use crate::application::ports::storage_ports::StorageUsagePort;
use crate::common::errors::DomainError;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::task;
use tracing::{debug, error, info};

/**
 * Service for managing and updating user storage usage statistics.
 *
 * This service is responsible for calculating how much storage each user
 * is using and updating this information in the user records.
 *
 * Storage usage is calculated directly from the `storage.files` table
 * by summing file sizes for each user (using the `user_id` column).
 */
pub struct StorageUsageService {
    pool: Arc<PgPool>,
    user_repository: Arc<dyn UserStoragePort>,
}

impl StorageUsageService {
    /// Creates a new storage usage service
    pub fn new(pool: Arc<PgPool>, user_repository: Arc<dyn UserStoragePort>) -> Self {
        Self {
            pool,
            user_repository,
        }
    }

    /// Calculates and updates storage usage for a specific user
    pub async fn update_user_storage_usage(&self, user_id: &str) -> Result<i64, DomainError> {
        info!("Updating storage usage for user: {}", user_id);

        // Calculate storage usage directly from database
        let total_usage = self.calculate_user_storage_usage(user_id).await?;

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

    /// Calculates a user's storage usage by summing all their file sizes.
    /// Uses a direct SQL query for O(1) performance.
    async fn calculate_user_storage_usage(&self, user_id: &str) -> Result<i64, DomainError> {
        debug!("Calculating storage for user: {}", user_id);

        // Direct SQL query to sum all file sizes for this user
        // This is much more efficient than recursively walking folders
        let total_size: i64 = sqlx::query_scalar(
            r#"
            SELECT COALESCE(SUM(size), 0)::bigint
              FROM storage.files
             WHERE user_id = $1::uuid AND NOT is_trashed
            "#,
        )
        .bind(user_id)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| {
            DomainError::internal_error("StorageUsage", format!("Failed to calculate usage: {e}"))
        })?;

        debug!(
            "Calculated storage for user {}: {} bytes",
            user_id, total_size
        );

        Ok(total_size)
    }

    /// Calculates and updates storage usage for a user identified by username.
    pub async fn update_user_storage_usage_by_username(
        &self,
        username: &str,
    ) -> Result<i64, DomainError> {
        info!("Updating storage usage for username: {}", username);

        let user = self.user_repository.get_user_by_username(username).await?;
        let user_id = user.id().to_string();

        // Reuse the existing calculation logic
        let total_usage = self.calculate_user_storage_usage(&user_id).await?;

        // Update the user's storage usage in the database
        self.user_repository
            .update_storage_usage(&user_id, total_usage)
            .await?;

        info!(
            "Updated storage usage for username {} (id={}) to {} bytes",
            username, user_id, total_usage
        );

        Ok(total_usage)
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

    async fn update_user_storage_usage_by_username(
        &self,
        username: &str,
    ) -> Result<i64, DomainError> {
        StorageUsageService::update_user_storage_usage_by_username(self, username).await
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

    async fn check_storage_quota(
        &self,
        user_id: &str,
        additional_bytes: u64,
    ) -> Result<(), DomainError> {
        let user = self.user_repository.get_user_by_id(user_id).await?;
        let quota = user.storage_quota_bytes();
        let used = user.storage_used_bytes();

        // Quota of 0 means unlimited
        if quota <= 0 {
            return Ok(());
        }

        let additional = additional_bytes as i64;

        // Case 1: the single file alone exceeds the entire quota
        if additional > quota {
            let quota_fmt = format_bytes(quota);
            let file_fmt = format_bytes(additional);
            return Err(DomainError::quota_exceeded(format!(
                "File size ({}) exceeds your total storage quota ({})",
                file_fmt, quota_fmt
            )));
        }

        // Case 2: the upload would push usage over the quota
        if used + additional > quota {
            let available = (quota - used).max(0);
            let avail_fmt = format_bytes(available);
            let file_fmt = format_bytes(additional);
            return Err(DomainError::quota_exceeded(format!(
                "Not enough storage space. File size: {}, available: {}",
                file_fmt, avail_fmt
            )));
        }

        Ok(())
    }

    async fn get_user_storage_info(&self, user_id: &str) -> Result<(i64, i64), DomainError> {
        let user = self.user_repository.get_user_by_id(user_id).await?;
        Ok((user.storage_used_bytes(), user.storage_quota_bytes()))
    }
}

// Make StorageUsageService cloneable to support spawning concurrent tasks
impl Clone for StorageUsageService {
    fn clone(&self) -> Self {
        Self {
            pool: Arc::clone(&self.pool),
            user_repository: Arc::clone(&self.user_repository),
        }
    }
}

/// Format bytes into human-readable units for error messages.
fn format_bytes(bytes: i64) -> String {
    const KB: i64 = 1024;
    const MB: i64 = KB * 1024;
    const GB: i64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
