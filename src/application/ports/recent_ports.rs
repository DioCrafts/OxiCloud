use crate::application::dtos::recent_dto::RecentItemDto;
use crate::common::errors::Result;
use async_trait::async_trait;

/// Defines operations for managing user recent items
#[async_trait]
pub trait RecentItemsUseCase: Send + Sync {
    /// Get all recent items for a user
    async fn get_recent_items(
        &self,
        user_id: &str,
        limit: Option<i32>,
    ) -> Result<Vec<RecentItemDto>>;

    /// Record access to an item
    async fn record_item_access(&self, user_id: &str, item_id: &str, item_type: &str)
    -> Result<()>;

    /// Remove an item from recents
    async fn remove_from_recent(
        &self,
        user_id: &str,
        item_id: &str,
        item_type: &str,
    ) -> Result<bool>;

    /// Clear the entire recent items list
    async fn clear_recent_items(&self, user_id: &str) -> Result<()>;
}

// ─────────────────────────────────────────────────────
// Outbound port — persistence abstraction
// ─────────────────────────────────────────────────────

/// Secondary (outbound) port for recent items persistence.
///
/// Abstracts access to the `auth.user_recent_files` table so that
/// `RecentService` does not depend directly on `PgPool`.
#[async_trait]
pub trait RecentItemsRepositoryPort: Send + Sync + 'static {
    /// Gets the latest recent items for a user (ordered by date desc).
    async fn get_recent_items(&self, user_id: &str, limit: i32) -> Result<Vec<RecentItemDto>>;

    /// Records/updates access to an item (upsert by user+item+type).
    async fn upsert_access(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<()>;

    /// Removes an item from recents. Returns `true` if it existed.
    async fn remove_item(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<bool>;

    /// Removes all recent items for a user.
    async fn clear_all(&self, user_id: &str) -> Result<()>;

    /// Removes items exceeding `max_items` (the oldest ones).
    async fn prune(&self, user_id: &str, max_items: i32) -> Result<()>;
}
