use serde::{Serialize, Deserialize};
use crate::domain::entities::folder::Folder;

/// DTO for folder creation requests
#[derive(Debug, Deserialize)]
pub struct CreateFolderDto {
    /// Name of the folder to create
    pub name: String,
    
    /// Parent folder ID (None for root level)
    pub parent_id: Option<String>,
    
    /// Owner user ID (None for system folders)
    pub user_id: Option<String>,
}

/// DTO for folder rename requests
#[derive(Debug, Deserialize)]
pub struct RenameFolderDto {
    /// New name for the folder
    pub name: String,
}

/// DTO for folder move requests
#[derive(Debug, Deserialize)]
pub struct MoveFolderDto {
    /// New parent folder ID (None for root level)
    pub parent_id: Option<String>,
}

/// DTO for folder responses
#[derive(Debug, Serialize)]
pub struct FolderDto {
    /// Folder ID
    pub id: String,
    
    /// Folder name
    pub name: String,
    
    /// Path to the folder (relative)
    pub path: String,
    
    /// Parent folder ID
    pub parent_id: Option<String>,
    
    /// Owner user ID
    pub user_id: Option<String>,
    
    /// Creation timestamp
    pub created_at: u64,
    
    /// Last modification timestamp
    pub modified_at: u64,
    
    /// Whether this is a root folder
    pub is_root: bool,
}

impl From<Folder> for FolderDto {
    fn from(folder: Folder) -> Self {
        let is_root = folder.parent_id().is_none();
        
        Self {
            id: folder.id().to_string(),
            name: folder.name().to_string(),
            path: folder.path_string().to_string(),
            parent_id: folder.parent_id().map(String::from),
            user_id: folder.user_id().map(String::from),
            created_at: folder.created_at(),
            modified_at: folder.modified_at(),
            is_root,
        }
    }
}

// Para convertir de FolderDto a Folder para los batch handlers
impl From<FolderDto> for Folder {
    fn from(dto: FolderDto) -> Self {
        // Usar constructor para crear una entidad desde DTO
        // Nota: esto debe simplificarse si Folder tiene un constructor adecuado
        Folder::from_dto(
            dto.id,
            dto.name,
            dto.path,
            dto.parent_id,
            dto.user_id,
            dto.created_at,
            dto.modified_at
        )
    }
}

impl FolderDto {
    /// Creates an empty folder DTO for stub implementations
    pub fn empty() -> Self {
        Self {
            id: "stub-id".to_string(),
            name: "stub-folder".to_string(),
            path: "/stub/path".to_string(),
            parent_id: None,
            user_id: None,
            created_at: 0,
            modified_at: 0,
            is_root: true,
        }
    }
}

impl Default for FolderDto {
    fn default() -> Self {
        Self::empty()
    }
}