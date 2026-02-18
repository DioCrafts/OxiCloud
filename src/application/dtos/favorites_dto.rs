use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::display_helpers::{format_file_size, icon_class_for, icon_special_class_for, category_for};

/// DTO for favorites item, enriched with item metadata via SQL JOIN
/// so the frontend does not need N+1 requests to resolve names/sizes.
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

    // ── Enriched metadata (resolved via JOIN) ──

    /// Display name of the file or folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_name: Option<String>,

    /// Size in bytes (files only; folders → None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_size: Option<i64>,

    /// MIME type (files only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_mime_type: Option<String>,

    /// Parent folder ID (folder_id for files, parent_id for folders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,

    /// Last modification timestamp of the item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<DateTime<Utc>>,

    // ── Pre-computed display fields ──

    /// FontAwesome icon CSS class (e.g. "fas fa-file-image", "fas fa-folder")
    pub icon_class: String,

    /// Extra CSS class for icon styling (e.g. "image-icon", "folder-icon")
    pub icon_special_class: String,

    /// Human-readable category (e.g. "Image", "Folder")
    pub category: String,

    /// Formatted file size (e.g. "3.27 MB"); "--" for folders
    pub size_formatted: String,
}

impl FavoriteItemDto {
    /// Populate display fields from the enriched metadata.
    /// Call this after constructing from the SQL row.
    pub fn with_display_fields(mut self) -> Self {
        if self.item_type == "folder" {
            self.icon_class = "fas fa-folder".to_string();
            self.icon_special_class = "folder-icon".to_string();
            self.category = "Folder".to_string();
            self.size_formatted = "--".to_string();
        } else {
            let name = self.item_name.as_deref().unwrap_or("");
            let mime = self.item_mime_type.as_deref().unwrap_or("application/octet-stream");
            self.icon_class = icon_class_for(name, mime).to_string();
            self.icon_special_class = icon_special_class_for(name, mime).to_string();
            self.category = category_for(name, mime).to_string();
            self.size_formatted = format_file_size(self.item_size.unwrap_or(0) as u64);
        }
        self
    }
}

/// Result DTO for batch add-to-favorites.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchFavoritesResult {
    /// Statistics about the batch operation
    pub stats: BatchFavoritesStats,
    /// Full list of the user's favourites (enriched), so the client can
    /// replace its local cache in a single round-trip.
    pub favorites: Vec<FavoriteItemDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchFavoritesStats {
    /// How many items were requested
    pub requested: usize,
    /// How many were actually inserted (new)
    pub inserted: u64,
    /// How many were already favourites (skipped)
    pub already_existed: u64,
}
