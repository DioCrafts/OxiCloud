use serde::{Deserialize, Serialize};

/**
 * Data Transfer Object for file search criteria.
 *
 * This structure represents all possible search parameters that can be used
 * to filter files and folders in the system. It supports various filter types
 * including name matching, file types, date ranges, and size constraints.
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCriteriaDto {
    /// Optional text to search in file/folder names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,

    /// Optional list of file extensions to include (e.g., "pdf", "jpg")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_types: Option<Vec<String>>,

    /// Optional minimum creation date (seconds since epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<u64>,

    /// Optional maximum creation date (seconds since epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<u64>,

    /// Optional minimum modification date (seconds since epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_after: Option<u64>,

    /// Optional maximum modification date (seconds since epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_before: Option<u64>,

    /// Optional minimum file size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<u64>,

    /// Optional maximum file size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<u64>,

    /// Optional folder ID to limit search scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,

    /// Whether to search recursively within subfolders (default: true)
    #[serde(default = "default_recursive")]
    pub recursive: bool,

    /// Maximum number of results to return
    #[serde(default = "default_limit")]
    pub limit: usize,

    /// Offset for pagination
    #[serde(default)]
    pub offset: usize,
}

/// Default value for recursive search (true)
fn default_recursive() -> bool {
    true
}

/// Default limit for search results (100)
fn default_limit() -> usize {
    100
}

impl Default for SearchCriteriaDto {
    fn default() -> Self {
        Self {
            name_contains: None,
            file_types: None,
            created_after: None,
            created_before: None,
            modified_after: None,
            modified_before: None,
            min_size: None,
            max_size: None,
            folder_id: None,
            recursive: default_recursive(),
            limit: default_limit(),
            offset: 0,
        }
    }
}

/**
 * Data Transfer Object for search results.
 *
 * This structure encapsulates the results of a search operation, including
 * both files and folders that match the search criteria, along with pagination information.
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultsDto {
    /// Files matching the search criteria
    pub files: Vec<crate::application::dtos::file_dto::FileDto>,

    /// Folders matching the search criteria
    pub folders: Vec<crate::application::dtos::folder_dto::FolderDto>,

    /// Total count of matching items (for pagination)
    pub total_count: Option<usize>,

    /// Limit used in the search
    pub limit: usize,

    /// Offset used in the search
    pub offset: usize,

    /// Whether there are more results available
    pub has_more: bool,
}

impl SearchResultsDto {
    /// Creates a new empty search results object
    pub fn empty() -> Self {
        Self {
            files: Vec::new(),
            folders: Vec::new(),
            total_count: None,
            limit: 0,
            offset: 0,
            has_more: false,
        }
    }

    /// Creates a new search results object from files and folders
    pub fn new(
        files: Vec<crate::application::dtos::file_dto::FileDto>,
        folders: Vec<crate::application::dtos::folder_dto::FolderDto>,
        limit: usize,
        offset: usize,
        total_count: Option<usize>,
    ) -> Self {
        let has_more = match total_count {
            Some(total) => (offset + files.len() + folders.len()) < total,
            None => false,
        };

        Self {
            files,
            folders,
            total_count,
            limit,
            offset,
            has_more,
        }
    }
}
