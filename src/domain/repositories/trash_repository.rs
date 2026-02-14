use async_trait::async_trait;
use uuid::Uuid;

use crate::common::errors::Result;
use crate::domain::entities::trashed_item::TrashedItem;

#[async_trait]
pub trait TrashRepository: Send + Sync {
    async fn add_to_trash(&self, item: &TrashedItem) -> Result<()>;
    async fn get_trash_items(&self, user_id: &Uuid) -> Result<Vec<TrashedItem>>;
    async fn get_trash_item(&self, id: &Uuid, user_id: &Uuid) -> Result<Option<TrashedItem>>;
    async fn restore_from_trash(&self, id: &Uuid, user_id: &Uuid) -> Result<()>;
    async fn delete_permanently(&self, id: &Uuid, user_id: &Uuid) -> Result<()>;
    async fn clear_trash(&self, user_id: &Uuid) -> Result<()>;
    async fn get_expired_items(&self) -> Result<Vec<TrashedItem>>;
}
