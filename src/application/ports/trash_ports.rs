use async_trait::async_trait;

use crate::application::dtos::trash_dto::TrashedItemDto;
use crate::common::errors::Result;

/// Port for trash-related use cases
#[async_trait]
pub trait TrashUseCase: Send + Sync {
    /// List items in the user's trash
    async fn get_trash_items(&self, user_id: &str) -> Result<Vec<TrashedItemDto>>;

    /// Move a file or folder to trash
    async fn move_to_trash(&self, item_id: &str, item_type: &str, user_id: &str) -> Result<()>;

    /// Restore an item from trash to its original location
    async fn restore_item(&self, trash_id: &str, user_id: &str) -> Result<()>;

    /// Permanently delete an item from trash
    async fn delete_permanently(&self, trash_id: &str, user_id: &str) -> Result<()>;

    /// Empty the trash for a specific user
    async fn empty_trash(&self, user_id: &str) -> Result<()>;
}
