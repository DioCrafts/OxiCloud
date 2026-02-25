use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error, info, instrument};

use crate::common::errors::Result;
use crate::domain::repositories::trash_repository::TrashRepository;

/// Service for automatic cleanup of expired items in the trash.
///
/// Uses `TrashRepository::delete_expired_bulk` to purge all expired items
/// in **2 SQL statements inside a single transaction**, instead of the
/// previous N+1 pattern that issued 3 queries per expired item.
pub struct TrashCleanupService {
    trash_repository: Arc<dyn TrashRepository>,
    cleanup_interval_hours: u64,
}

impl TrashCleanupService {
    pub fn new(
        trash_repository: Arc<dyn TrashRepository>,
        cleanup_interval_hours: u64,
    ) -> Self {
        Self {
            trash_repository,
            cleanup_interval_hours: cleanup_interval_hours.max(1), // Minimum 1 hour
        }
    }

    /// Starts the periodic cleanup job
    #[instrument(skip(self))]
    pub async fn start_cleanup_job(&self) {
        let trash_repository = self.trash_repository.clone();
        let interval_hours = self.cleanup_interval_hours;

        info!(
            "Starting trash cleanup job with interval of {} hours",
            interval_hours
        );

        tokio::spawn(async move {
            let interval_duration = Duration::from_secs(interval_hours * 60 * 60);
            let mut interval = time::interval(interval_duration);

            // First immediate execution
            Self::cleanup_expired_items(trash_repository.clone())
                .await
                .unwrap_or_else(|e| error!("Error in initial trash cleanup: {:?}", e));

            loop {
                interval.tick().await;
                debug!("Running scheduled trash cleanup task");

                if let Err(e) = Self::cleanup_expired_items(trash_repository.clone()).await {
                    error!("Error in scheduled trash cleanup: {:?}", e);
                }
            }
        });
    }

    /// Bulk-delete all expired trash items in a single transaction.
    #[instrument(skip(trash_repository))]
    async fn cleanup_expired_items(
        trash_repository: Arc<dyn TrashRepository>,
    ) -> Result<()> {
        debug!("Starting bulk cleanup of expired trash items");

        let (files, folders) = trash_repository.delete_expired_bulk().await?;

        if files == 0 && folders == 0 {
            debug!("No expired items to clean up");
        } else {
            info!(
                "Trash cleanup completed: {} files + {} folders purged",
                files, folders
            );
        }

        Ok(())
    }
}
