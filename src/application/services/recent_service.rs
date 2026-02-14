use crate::application::dtos::recent_dto::RecentItemDto;
use crate::application::ports::recent_ports::{RecentItemsRepositoryPort, RecentItemsUseCase};
use crate::common::errors::{DomainError, ErrorKind, Result};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::info;

/// Implementation of the use case for managing recent items.
///
/// Depends on `RecentItemsRepositoryPort` (outbound port) instead
/// of accessing `PgPool` directly, following the hexagonal architecture.
pub struct RecentService {
    repo: Arc<dyn RecentItemsRepositoryPort>,
    max_recent_items: i32,
}

impl RecentService {
    /// Create a new recent items service
    pub fn new(repo: Arc<dyn RecentItemsRepositoryPort>, max_recent_items: i32) -> Self {
        Self {
            repo,
            max_recent_items: max_recent_items.max(1).min(100),
        }
    }
}

#[async_trait]
impl RecentItemsUseCase for RecentService {
    /// Get recent items for a user
    async fn get_recent_items(
        &self,
        user_id: &str,
        limit: Option<i32>,
    ) -> Result<Vec<RecentItemDto>> {
        info!("Getting recent items for user: {}", user_id);
        let limit_value = limit
            .unwrap_or(self.max_recent_items)
            .min(self.max_recent_items);
        let items = self.repo.get_recent_items(user_id, limit_value).await?;
        info!(
            "Retrieved {} recent items for user {}",
            items.len(),
            user_id
        );
        Ok(items)
    }

    /// Record access to an item
    async fn record_item_access(
        &self,
        user_id: &str,
        item_id: &str,
        item_type: &str,
    ) -> Result<()> {
        info!(
            "Recording access to {} '{}' for user {}",
            item_type, item_id, user_id
        );

        if item_type != "file" && item_type != "folder" {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "RecentItems",
                "Item type must be 'file' or 'folder'",
            ));
        }

        self.repo.upsert_access(user_id, item_id, item_type).await?;
        self.repo.prune(user_id, self.max_recent_items).await?;

        info!(
            "Successfully recorded access to {} '{}' for user {}",
            item_type, item_id, user_id
        );
        Ok(())
    }

    /// Remove an item from recent
    async fn remove_from_recent(
        &self,
        user_id: &str,
        item_id: &str,
        item_type: &str,
    ) -> Result<bool> {
        info!(
            "Removing {} '{}' from recent for user {}",
            item_type, item_id, user_id
        );
        let removed = self.repo.remove_item(user_id, item_id, item_type).await?;
        info!(
            "{} {} '{}' from recent items for user {}",
            if removed {
                "Successfully removed"
            } else {
                "Not found"
            },
            item_type,
            item_id,
            user_id
        );
        Ok(removed)
    }

    /// Clear all recent items
    async fn clear_recent_items(&self, user_id: &str) -> Result<()> {
        info!("Clearing all recent items for user {}", user_id);
        self.repo.clear_all(user_id).await?;
        info!("Cleared all recent items for user {}", user_id);
        Ok(())
    }
}
