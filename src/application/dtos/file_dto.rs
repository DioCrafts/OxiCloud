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

    /// MIME type
    pub mime_type: String,

    /// Parent folder ID
    pub folder_id: Option<String>,

    /// Creation timestamp
    pub created_at: u64,

    /// Last modification timestamp
    pub modified_at: u64,

    // ── Pre-computed display fields ──
    /// FontAwesome icon CSS class (e.g. "fas fa-file-image")
    pub icon_class: String,

    /// Extra CSS class for icon styling (e.g. "image-icon", "" when default)
    pub icon_special_class: String,

    /// Human-readable file category (e.g. "Image", "Document")
    pub category: String,

    /// Human-readable formatted size (e.g. "3.27 MB")
    pub size_formatted: String,
}

impl From<File> for FileDto {
    fn from(file: File) -> Self {
        let name = file.name();
        let mime = file.mime_type();
        let size = file.size();

        Self {
            id: file.id().to_string(),
            name: name.to_string(),
            path: file.path_string().to_string(),
            size,
            mime_type: mime.to_string(),
            folder_id: file.folder_id().map(String::from),
            created_at: file.created_at(),
            modified_at: file.modified_at(),
            icon_class: icon_class_for(name, mime).to_string(),
            icon_special_class: icon_special_class_for(name, mime).to_string(),
            category: category_for(name, mime).to_string(),
            size_formatted: format_file_size(size),
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
            dto.mime_type,
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
            mime_type: "application/octet-stream".to_string(),
            folder_id: None,
            created_at: 0,
            modified_at: 0,
            icon_class: "fas fa-file".to_string(),
            icon_special_class: String::new(),
            category: "Document".to_string(),
            size_formatted: "0 Bytes".to_string(),
        }
    }
}

impl Default for FileDto {
    fn default() -> Self {
        Self::empty()
    }
}
