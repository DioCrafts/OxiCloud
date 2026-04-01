use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// DTO representing an item in the trash
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TrashedItemDto {
    pub id: String,
    pub original_id: String,
    pub item_type: String, // "file" o "folder"
    pub name: String,
    pub original_path: String,
    pub trashed_at: DateTime<Utc>,
    pub days_until_deletion: i64,
    /// Human-readable category (e.g., "Image", "Folder", "Document")
    pub category: String,
    /// FontAwesome icon class for the file type
    pub icon_class: String,
    /// Special CSS class for icon styling (e.g., "image-icon", "pdf-icon")
    pub icon_special_class: String,
}

/// Request to move an item to trash
#[derive(Debug, Deserialize, ToSchema)]
pub struct MoveToTrashRequest {
    pub item_id: String,
    pub item_type: String, // "file" o "folder"
}

/// Request to restore an item from trash
#[derive(Debug, Deserialize, ToSchema)]
pub struct RestoreFromTrashRequest {
    pub trash_id: String,
}

/// Request to permanently delete an item from trash
#[derive(Debug, Deserialize, ToSchema)]
pub struct DeletePermanentlyRequest {
    pub trash_id: String,
}
