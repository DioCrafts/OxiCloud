use std::sync::Arc;

use crate::domain::entities::file::File;
use serde::{Deserialize, Serialize};

use super::display_helpers::{
    category_for, format_file_size, icon_class_for, icon_special_class_for,
};

/// DTO for file responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDto {
    /// File ID
    pub id: String,

    /// File name
    pub name: String,

    /// Path to the file (relative)
    pub path: String,

    /// Size in bytes
    pub size: u64,

    /// MIME type — `Arc<str>` because MIME values repeat across files
    /// and DTOs are cloned on every request (clone is O(1) atomic increment).
    pub mime_type: Arc<str>,

    /// Parent folder ID
    pub folder_id: Option<String>,

    /// Creation timestamp
    pub created_at: u64,

    /// Last modification timestamp
    pub modified_at: u64,

    // ── Pre-computed display fields (Arc<str>: values come from static tables) ──
    /// FontAwesome icon CSS class (e.g. "fas fa-file-image")
    pub icon_class: Arc<str>,

    /// Extra CSS class for icon styling (e.g. "image-icon", "" when default)
    pub icon_special_class: Arc<str>,

    /// Human-readable file category (e.g. "Image", "Document")
    pub category: Arc<str>,

    /// Human-readable formatted size (e.g. "3.27 MB")
    pub size_formatted: String,

    /// Owner user ID (omitted from JSON when None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,

    /// Sort date for Photos timeline — COALESCE(EXIF captured_at, created_at).
    /// Only populated by the /api/photos endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_date: Option<u64>,

    /// Content-addressable ETag (= blob_hash). Changes on every content write.
    /// Used for WebDAV/Nextcloud ETag headers. Omitted from REST API JSON.
    #[serde(skip)]
    pub etag: String,
}

impl From<File> for FileDto {
    fn from(file: File) -> Self {
        // Consume the entity by moving all fields — zero heap allocations
        // for id, name, path, folder_id, owner_id (previously 5× .to_string()).
        let parts = file.into_parts();

        let icon_class = Arc::from(icon_class_for(&parts.name, &parts.mime_type));
        let icon_special_class = Arc::from(icon_special_class_for(&parts.name, &parts.mime_type));
        let category = Arc::from(category_for(&parts.name, &parts.mime_type));
        let size_formatted = format_file_size(parts.size);
        let mime_type = Arc::from(parts.mime_type.as_str());

        Self {
            id: parts.id,
            name: parts.name,
            path: parts.path_string,
            size: parts.size,
            mime_type,
            folder_id: parts.folder_id,
            created_at: parts.created_at,
            modified_at: parts.modified_at,
            icon_class,
            icon_special_class,
            category,
            size_formatted,
            owner_id: parts.owner_id.map(|u| u.to_string()),
            sort_date: None,
            etag: parts.etag,
        }
    }
}

// To convert from FileDto to File for batch handlers
impl From<FileDto> for File {
    fn from(dto: FileDto) -> Self {
        // Display fields (icon_class, icon_special_class, category, size_formatted)
        // are not part of the domain entity and are ignored.
        File::from_dto(
            dto.id,
            dto.name,
            dto.path,
            dto.size,
            dto.mime_type.to_string(),
            dto.folder_id,
            dto.created_at,
            dto.modified_at,
        )
    }
}

impl FileDto {
    /// Creates an empty file DTO for stub implementations
    pub fn empty() -> Self {
        Self {
            id: "stub-id".to_string(),
            name: "stub-file".to_string(),
            path: "/stub/path".to_string(),
            size: 0,
            mime_type: Arc::from("application/octet-stream"),
            folder_id: None,
            created_at: 0,
            modified_at: 0,
            icon_class: Arc::from("fas fa-file"),
            icon_special_class: Arc::from(""),
            category: Arc::from("Document"),
            size_formatted: "0 Bytes".to_string(),
            owner_id: None,
            etag: String::new(),
            sort_date: None,
        }
    }
}

impl Default for FileDto {
    fn default() -> Self {
        Self::empty()
    }
}
