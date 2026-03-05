use serde::Serialize;

use super::file_dto::FileDto;
use super::folder_dto::FolderDto;

/// Combined DTO that returns both sub-folders and files for a given folder
/// in a single response, eliminating the double-fetch on every navigation.
#[derive(Debug, Serialize)]
pub struct FolderListingDto {
    /// Sub-folders inside the requested folder
    pub folders: Vec<FolderDto>,
    /// Files inside the requested folder
    pub files: Vec<FileDto>,
}
