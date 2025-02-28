use std::cmp::Ordering;
use std::path::Path;

/// Helper functionality for file operations
pub struct Helper;

impl Helper {
    /// Builds file storage statistics for a directory
    pub async fn build_file_storage_statistics(dir: &str) -> Result<FileStorageStatistics, Error> {
        let l = L10n::new("files")?;
        let max_upload_filesize = Util::max_upload_filesize(dir).await?;
        let max_human_filesize = Util::human_file_size(max_upload_filesize);
        let max_human_filesize = format!("{} max. {}", l.t("Upload"), max_human_filesize);

        // information about storage capacities
        let storage_info = OcHelper::get_storage_info(dir).await?;

        Ok(FileStorageStatistics {
            upload_max_filesize: max_upload_filesize,
            max_human_filesize,
            used_space_percent: storage_info.relative as i32,
        })
    }

    /// Determines the icon for a file
    pub async fn determine_icon(file: &FileInfo) -> Result<String, Error> {
        if file.file_type == FileType::Directory {
            let dir = &file.directory;
            let abs_path = Filesystem::get_view().get_abs_path(&format!("{}/{}", dir, file.name))?;
            let mount = Filesystem::get_mount_manager().find(&abs_path)?;
            
            if let Some(mount) = mount {
                if let Some(storage_id) = mount.get_storage_id() {
                    let sid: Vec<&str> = storage_id.split(':').collect();
                    if sid[0] == "shared" {
                        return Ok(OcHelper::mimetype_icon("dir-shared"));
                    }
                    if sid[0] != "local" && sid[0] != "home" {
                        return Ok(OcHelper::mimetype_icon("dir-external"));
                    }
                }
            }
            return Ok(OcHelper::mimetype_icon("dir"));
        }

        if file.is_preview_available {
            let path_for_preview = format!("{}/{}", file.directory, file.name);
            return Ok(format!("{}&c={}", OcHelper::preview_icon(&path_for_preview), file.etag));
        }
        
        Ok(OcHelper::mimetype_icon(&file.mimetype))
    }

    /// Comparator function to sort files alphabetically and have
    /// the directories appear first
    /// 
    /// Returns Ordering::Less if a must come before b, Ordering::Greater otherwise
    pub fn file_cmp(a: &FileInfo, b: &FileInfo) -> Ordering {
        match (a.file_type, b.file_type) {
            (FileType::Directory, FileType::File) => Ordering::Less,
            (FileType::File, FileType::Directory) => Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    }

    /// Retrieves the contents of the given directory and
    /// returns it as a sorted array.
    pub async fn get_files(dir: &str) -> Result<Vec<FileInfo>, Error> {
        let content = Filesystem::get_directory_content(dir).await?;
        let preview_manager = Server::get().get_preview_manager();
        
        let mut files = Vec::with_capacity(content.len());

        for mut item in content {
            item.date = Util::format_date(item.mtime);
            
            if item.file_type == FileType::File {
                let path = Path::new(&item.name);
                item.basename = path.file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default();
                
                item.extension = path.extension()
                    .map(|ext| format!(".{}", ext.to_string_lossy()))
                    .unwrap_or_default();
            }
            
            item.directory = dir.to_string();
            item.is_preview_available = preview_manager.is_mime_supported(&item.mimetype);
            item.icon = Self::determine_icon(&item).await?;
            
            files.push(item);
        }

        files.sort_by(Self::file_cmp);

        Ok(files)
    }

    /// Splits the given path into a breadcrumb structure.
    pub fn make_breadcrumb(dir: &str) -> Vec<Breadcrumb> {
        let mut breadcrumb = Vec::new();
        let mut path_to_here = String::new();
        
        for segment in dir.split('/') {
            if !segment.is_empty() {
                path_to_here.push_str("/");
                path_to_here.push_str(segment);
                breadcrumb.push(Breadcrumb {
                    dir: path_to_here.clone(),
                    name: segment.to_string(),
                });
            }
        }
        
        breadcrumb
    }

    /// Returns the numeric permissions for the given directory.
    pub fn get_dir_permissions(dir: &str) -> Permission {
        let mut permissions = Permission::READ;
        let dir_with_slash = format!("{}/", dir);
        
        if Filesystem::is_creatable(&dir_with_slash) {
            permissions |= Permission::CREATE;
        }
        if Filesystem::is_updatable(&dir_with_slash) {
            permissions |= Permission::UPDATE;
        }
        if Filesystem::is_deletable(&dir_with_slash) {
            permissions |= Permission::DELETE;
        }
        if Filesystem::is_sharable(&dir_with_slash) {
            permissions |= Permission::SHARE;
        }
        
        permissions
    }
}

/// Statistics about file storage
#[derive(Debug, Clone, Serialize)]
pub struct FileStorageStatistics {
    pub upload_max_filesize: u64,
    pub max_human_filesize: String,
    pub used_space_percent: i32,
}

/// Represents a breadcrumb entry in a path
#[derive(Debug, Clone, Serialize)]
pub struct Breadcrumb {
    pub dir: String,
    pub name: String,
}

/// Represents information about a file or directory
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    pub directory: String,
    pub file_type: FileType,
    pub mimetype: String,
    pub mtime: i64,
    pub etag: String,
    pub size: u64,
    pub is_preview_available: bool,
    pub icon: String,
    pub date: String,
    pub basename: String,
    pub extension: String,
}

/// Type of file system entry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    File,
    Directory,
}

bitflags! {
    /// File permission flags
    pub struct Permission: u32 {
        const READ = 0x01;
        const CREATE = 0x02;
        const UPDATE = 0x04;
        const DELETE = 0x08;
        const SHARE = 0x10;
    }
}

// Error handling for the module
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Filesystem error: {0}")]
    FilesystemError(String),
    
    #[error("Localization error: {0}")]
    L10nError(String),
    
    #[error("Mount error: {0}")]
    MountError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}