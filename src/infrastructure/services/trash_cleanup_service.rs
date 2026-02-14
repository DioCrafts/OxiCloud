use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error, info, instrument};

use crate::application::ports::trash_ports::TrashUseCase;
use crate::common::errors::Result;
use crate::domain::repositories::trash_repository::TrashRepository;

/// Service for automatic cleanup of expired items in the trash
pub struct TrashCleanupService {
    trash_service: Arc<dyn TrashUseCase>,
    trash_repository: Arc<dyn TrashRepository>,
    cleanup_interval_hours: u64,
}

impl TrashCleanupService {
    pub fn new(
        trash_service: Arc<dyn TrashUseCase>,
        trash_repository: Arc<dyn TrashRepository>,
        cleanup_interval_hours: u64,
    ) -> Self {
        Self {
            trash_service,
            trash_repository,
            cleanup_interval_hours: cleanup_interval_hours.max(1), // Minimum 1 hour
        }
    }

    /// Starts the periodic cleanup job
    #[instrument(skip(self))]
    pub async fn start_cleanup_job(&self) {
        let trash_repository = self.trash_repository.clone();
        let trash_service = self.trash_service.clone();
        let interval_hours = self.cleanup_interval_hours;

        info!(
            "Starting trash cleanup job with interval of {} hours",
            interval_hours
        );

        tokio::spawn(async move {
            let interval_duration = Duration::from_secs(interval_hours * 60 * 60);
            let mut interval = time::interval(interval_duration);

            // First immediate execution
            Self::cleanup_expired_items(trash_repository.clone(), trash_service.clone())
                .await
                .unwrap_or_else(|e| error!("Error in initial trash cleanup: {:?}", e));

            loop {
                interval.tick().await;
                debug!("Running scheduled trash cleanup task");

                if let Err(e) =
                    Self::cleanup_expired_items(trash_repository.clone(), trash_service.clone())
                        .await
                {
                    error!("Error in scheduled trash cleanup: {:?}", e);
                }
            }
        });
    }

    /// Cleans up expired items in the trash
    #[instrument(skip(trash_repository, trash_service))]
    async fn cleanup_expired_items(
        trash_repository: Arc<dyn TrashRepository>,
        trash_service: Arc<dyn TrashUseCase>,
    ) -> Result<()> {
        debug!("Starting cleanup of expired items in the trash");

        // Get all expired items
        let expired_items = trash_repository.get_expired_items().await?;

        if expired_items.is_empty() {
            debug!("No expired items to clean up");
            return Ok(());
        }

        info!("Found {} expired items to delete", expired_items.len());

        // Delete each expired item
        for item in expired_items {
            let trash_id = item.id().to_string();
            let user_id = item.user_id().to_string();

            debug!("Deleting expired item: id={}, user={}", trash_id, user_id);

            // If a deletion fails, continue with the rest
            if let Err(e) = trash_service.delete_permanently(&trash_id, &user_id).await {
                error!("Error deleting expired item {}: {:?}", trash_id, e);
            } else {
                debug!("Expired item deleted successfully: {}", trash_id);
            }
        }

        info!("Trash cleanup completed");
        Ok(())
    }
}
