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

    /// Sort order for results: "relevance", "name", "name_desc", "date", "date_desc", "size", "size_desc"
    #[serde(default = "default_sort_by")]
    pub sort_by: String,
}

/// Default value for recursive search (true)
fn default_recursive() -> bool {
    true
}

/// Default limit for search results (100)
fn default_limit() -> usize {
    100
}

/// Default sort_by value
fn default_sort_by() -> String {
    "relevance".to_string()
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
            sort_by: default_sort_by(),
        }
    }
}

/// A file search result enriched with server-computed metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFileResultDto {
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
    /// Relevance score (0-100) computed server-side
    pub relevance_score: u32,
    /// Human-readable file size (e.g., "2.5 MB")
    pub size_formatted: String,
    /// CSS icon class for the file type (e.g., "fas fa-file-pdf")
    pub icon_class: String,
    /// Content category: "document", "image", "video", "audio", "archive", "code", "other"
    pub category: String,
}

/// A folder search result enriched with server-computed metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFolderResultDto {
    /// Folder ID
    pub id: String,
    /// Folder name
    pub name: String,
    /// Path to the folder (relative)
    pub path: String,
    /// Parent folder ID
    pub parent_id: Option<String>,
    /// Creation timestamp
    pub created_at: u64,
    /// Last modification timestamp
    pub modified_at: u64,
    /// Whether it is a root folder
    pub is_root: bool,
    /// Relevance score (0-100) computed server-side
    pub relevance_score: u32,
}

/**
 * Data Transfer Object for search results.
 *
 * This structure encapsulates the results of a search operation, including
 * both files and folders that match the search criteria, along with pagination
 * information and server-computed metadata.
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultsDto {
    /// Files matching the search criteria (enriched with metadata)
    pub files: Vec<SearchFileResultDto>,

    /// Folders matching the search criteria (enriched with metadata)
    pub folders: Vec<SearchFolderResultDto>,

    /// Total count of matching items (for pagination)
    pub total_count: Option<usize>,

    /// Limit used in the search
    pub limit: usize,

    /// Offset used in the search
    pub offset: usize,

    /// Whether there are more results available
    pub has_more: bool,

    /// Query execution time in milliseconds (server-side)
    pub query_time_ms: u64,

    /// Sort order used
    pub sort_by: String,
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
            query_time_ms: 0,
            sort_by: "relevance".to_string(),
        }
    }

    /// Creates a new search results object from files and folders
    pub fn new(
        files: Vec<SearchFileResultDto>,
        folders: Vec<SearchFolderResultDto>,
        limit: usize,
        offset: usize,
        total_count: Option<usize>,
        query_time_ms: u64,
        sort_by: String,
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
            query_time_ms,
            sort_by,
        }
    }
}

/// DTO for search suggestion results (quick prefix search)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSuggestionsDto {
    /// Suggested file/folder names matching the query prefix
    pub suggestions: Vec<SearchSuggestionItem>,
    /// Query execution time in milliseconds
    pub query_time_ms: u64,
}

/// Individual search suggestion item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSuggestionItem {
    /// The suggested name
    pub name: String,
    /// Type: "file" or "folder"
    pub item_type: String,
    /// Item ID for navigation
    pub id: String,
    /// Path for context
    pub path: String,
    /// CSS icon class
    pub icon_class: String,
    /// Relevance score
    pub relevance_score: u32,
}
