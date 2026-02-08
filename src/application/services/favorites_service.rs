use std::sync::Arc;
use async_trait::async_trait;
use tracing::info;
use crate::common::errors::{Result, DomainError, ErrorKind};
use crate::application::ports::favorites_ports::{FavoritesUseCase, FavoritesRepositoryPort};
use crate::application::dtos::favorites_dto::FavoriteItemDto;

/// Implementation of the FavoritesUseCase for managing user favorites.
///
/// Depends on `FavoritesRepositoryPort` (outbound port) instead of
/// accessing the database directly, following hexagonal architecture.
pub struct FavoritesService {
    repo: Arc<dyn FavoritesRepositoryPort>,
}

impl FavoritesService {
    /// Create a new FavoritesService with the given repository port
    pub fn new(repo: Arc<dyn FavoritesRepositoryPort>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl FavoritesUseCase for FavoritesService {
    /// Get all favorites for a user
    async fn get_favorites(&self, user_id: &str) -> Result<Vec<FavoriteItemDto>> {
        info!("Getting favorites for user: {}", user_id);
        let favorites = self.repo.get_favorites(user_id).await?;
        info!("Retrieved {} favorites for user {}", favorites.len(), user_id);
        Ok(favorites)
    }

    /// Add an item to user's favorites
    async fn add_to_favorites(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<()> {
        info!("Adding {} '{}' to favorites for user {}", item_type, item_id, user_id);

        if item_type != "file" && item_type != "folder" {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "Favorites",
                "Item type must be 'file' or 'folder'",
            ));
        }

        self.repo.add_favorite(user_id, item_id, item_type).await?;
        info!("Successfully added {} '{}' to favorites for user {}", item_type, item_id, user_id);
        Ok(())
    }

    /// Remove an item from user's favorites
    async fn remove_from_favorites(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<bool> {
        info!("Removing {} '{}' from favorites for user {}", item_type, item_id, user_id);
        let removed = self.repo.remove_favorite(user_id, item_id, item_type).await?;
        info!(
            "{} {} '{}' from favorites for user {}",
            if removed { "Successfully removed" } else { "Did not find" },
            item_type, item_id, user_id
        );
        Ok(removed)
    }

    /// Check if an item is in user's favorites
    async fn is_favorite(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<bool> {
        info!("Checking if {} '{}' is favorite for user {}", item_type, item_id, user_id);
        self.repo.is_favorite(user_id, item_id, item_type).await
    }
}