use async_trait::async_trait;
use crate::common::errors::Result;
use crate::application::dtos::favorites_dto::FavoriteItemDto;

/// Defines operations for managing user favorites
#[async_trait]
pub trait FavoritesUseCase: Send + Sync {
    /// Get all favorites for a user
    async fn get_favorites(&self, user_id: &str) -> Result<Vec<FavoriteItemDto>>;
    
    /// Add an item to user's favorites
    async fn add_to_favorites(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<()>;
    
    /// Remove an item from user's favorites
    async fn remove_from_favorites(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<bool>;
    
    /// Check if an item is in user's favorites
    async fn is_favorite(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<bool>;
}

// ─────────────────────────────────────────────────────
// Outbound port — persistence abstraction
// ─────────────────────────────────────────────────────

/// Secondary (outbound) port for favorites persistence.
///
/// Application services depend on this trait instead of
/// accessing `PgPool` directly. The concrete implementation
/// lives in `infrastructure::repositories::pg`.
#[async_trait]
pub trait FavoritesRepositoryPort: Send + Sync + 'static {
    /// Gets all favorites for a user.
    async fn get_favorites(&self, user_id: &str) -> Result<Vec<FavoriteItemDto>>;

    /// Adds an item to favorites. Returns `Ok(())` if it already existed (idempotent).
    async fn add_favorite(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<()>;

    /// Removes an item from favorites. Returns `true` if it existed.
    async fn remove_favorite(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<bool>;

    /// Checks if an item is in favorites.
    async fn is_favorite(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<bool>;
}