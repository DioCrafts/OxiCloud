use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// DTO for recent items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentItemDto {
    /// Unique identifier for the recent item
    pub id: String,
    
    /// Owner user ID
    pub user_id: String,
    
    /// Item ID (file or folder)
    pub item_id: String,
    
    /// Item type ('file' or 'folder')
    pub item_type: String,
    
    /// When the item was accessed
    pub accessed_at: DateTime<Utc>,
}