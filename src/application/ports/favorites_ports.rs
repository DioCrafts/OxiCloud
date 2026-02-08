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

/// Puerto secundario (outbound) para persistencia de favoritos.
///
/// Los servicios de aplicación dependen de este trait en lugar de
/// acceder directamente a `PgPool`. La implementación concreta
/// vive en `infrastructure::repositories::pg`.
#[async_trait]
pub trait FavoritesRepositoryPort: Send + Sync + 'static {
    /// Obtiene todos los favoritos de un usuario.
    async fn get_favorites(&self, user_id: &str) -> Result<Vec<FavoriteItemDto>>;

    /// Añade un ítem a favoritos. Devuelve `Ok(())` si ya existía (idempotente).
    async fn add_favorite(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<()>;

    /// Elimina un ítem de favoritos. Devuelve `true` si existía.
    async fn remove_favorite(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<bool>;

    /// Comprueba si un ítem está en favoritos.
    async fn is_favorite(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<bool>;
}