use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// DTO for favorites item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteItemDto {
    /// Unique identifier for the favorite entry
    pub id: String,

    /// User ID who owns this favorite
    pub user_id: String,

    /// ID of the favorited item (file or folder)
    pub item_id: String,

    /// Type of the item ('file' or 'folder')
    pub item_type: String,

    /// When the item was added to favorites
    pub created_at: DateTime<Utc>,
}
