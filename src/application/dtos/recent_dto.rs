use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::display_helpers::{format_file_size, icon_class_for, icon_special_class_for, category_for};

/// DTO for recent items, enriched with item metadata via SQL JOIN
/// so the frontend does not need N+1 requests to resolve names/sizes.
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

impl RecentItemDto {
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
