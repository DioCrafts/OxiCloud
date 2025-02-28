use chrono::prelude::*;
use sqlx::{query, Pool, Sqlite};
use std::collections::HashMap;
use std::fs::ReadDir;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct Helper;

impl Helper {
    /// Retrieves the contents of a trash bin directory.
    /// 
    /// # Arguments
    /// * `dir` - path to the directory inside the trashbin
    ///   or empty to retrieve the root of the trashbin
    /// 
    /// # Returns
    /// Array of files
    pub async fn get_trash_files(
        dir: &str,
        db_pool: &Pool<Sqlite>,
        user: &str,
        preview_manager: Arc<dyn PreviewManager>,
    ) -> Result<Vec<HashMap<String, String>>, TrashbinError> {
        let mut files = Vec::new();

        if !dir.is_empty() && dir != "/" {
            let path = format!("/{}/files_trashbin/files", user);
            let view = FilesystemView::new(&path);
            
            let dir_content = view.opendir(dir)?;
            
            for entry in dir_content {
                let entry_name = entry?;
                if !Filesystem::is_ignored_dir(&entry_name) {
                    let pos = dir.find('/').map(|p| p + 1).unwrap_or(0);
                    let tmp = &dir[0..pos];
                    let pos = tmp.rfind(".d").unwrap_or(0) + 2;
                    let timestamp = &tmp[pos..];
                    
                    let full_path = format!("{}/{}", dir, entry_name);
                    
                    let entry_type = if view.is_dir(&full_path) { "dir" } else { "file" };
                    
                    let mut item = HashMap::new();
                    item.insert("id".to_string(), entry_name);
                    item.insert("timestamp".to_string(), timestamp.to_string());
                    item.insert("mime".to_string(), view.get_mime_type(&full_path));
                    item.insert("type".to_string(), entry_type.to_string());
                    item.insert("location".to_string(), dir.to_string());
                    
                    files.push(item);
                }
            }
        } else {
            let results = query("SELECT `id`,`location`,`timestamp`,`type`,`mime` FROM `files_trash` WHERE `user` = ?")
                .bind(user)
                .fetch_all(db_pool)
                .await?;

            for row in results {
                let id: String = row.get("id");
                let timestamp: i64 = row.get("timestamp");
                let mime: String = row.get("mime");
                let entry_type: String = row.get("type");
                let location: String = row.get("location");
                
                let mut item = HashMap::new();
                item.insert("name".to_string(), id.clone());
                item.insert("date".to_string(), format_date(timestamp));
                item.insert("timestamp".to_string(), timestamp.to_string());
                item.insert("mimetype".to_string(), mime.clone());
                item.insert("type".to_string(), entry_type.clone());
                
                if entry_type == "file" {
                    let path = Path::new(&id);
                    if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
                        item.insert("basename".to_string(), filename.to_string());
                    }
                    
                    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                        item.insert("extension".to_string(), format!(".{}", ext));
                    } else {
                        item.insert("extension".to_string(), String::new());
                    }
                }
                
                let directory = if location == "/" {
                    String::new()
                } else {
                    location
                };
                
                item.insert("directory".to_string(), directory);
                item.insert("permissions".to_string(), "1".to_string()); // READ permission
                item.insert("isPreviewAvailable".to_string(), 
                    if preview_manager.is_mime_supported(&mime) { "true" } else { "false" }.to_string());
                
                item.insert("icon".to_string(), FilesHelper::determine_icon(&item));
                
                files.push(item);
            }
        }

        // Sort files using file_cmp function
        files.sort_by(|a, b| FilesHelper::file_cmp(a, b));
        
        Ok(files)
    }

    /// Splits the given path into a breadcrumb structure.
    /// 
    /// # Arguments
    /// * `dir` - path to process
    /// 
    /// # Returns
    /// Array where each entry is a hash of the absolute directory path and its name
    pub fn make_breadcrumb(dir: &str) -> Vec<HashMap<String, String>> {
        let mut path_to_here = String::new();
        let mut breadcrumb = Vec::new();
        
        for component in dir.split('/') {
            if !component.is_empty() {
                let name = if let Some(captures) = regex::Regex::new(r"^(.+)\.d[0-9]+$")
                    .unwrap()
                    .captures(component) 
                {
                    captures.get(1).unwrap().as_str().to_string()
                } else {
                    component.to_string()
                };
                
                path_to_here.push_str("/");
                path_to_here.push_str(component);
                
                let mut item = HashMap::new();
                item.insert("dir".to_string(), path_to_here.clone());
                item.insert("name".to_string(), name);
                
                breadcrumb.push(item);
            }
        }
        
        breadcrumb
    }
}

// These would be defined elsewhere in your codebase
pub trait PreviewManager: Send + Sync {
    fn is_mime_supported(&self, mime: &str) -> bool;
}

pub struct FilesystemView {
    root: String,
}

impl FilesystemView {
    pub fn new(root: &str) -> Self {
        Self { root: root.to_string() }
    }
    
    pub fn opendir(&self, path: &str) -> Result<ReadDir, TrashbinError> {
        let full_path = format!("{}{}", self.root, path);
        std::fs::read_dir(full_path)
            .map_err(|e| TrashbinError::IoError(e))
    }
    
    pub fn is_dir(&self, path: &str) -> bool {
        let full_path = format!("{}{}", self.root, path);
        Path::new(&full_path).is_dir()
    }
    
    pub fn get_mime_type(&self, path: &str) -> String {
        // Implement mime type detection
        // This is a simplified version
        let full_path = format!("{}{}", self.root, path);
        if Path::new(&full_path).is_dir() {
            "httpd/unix-directory".to_string()
        } else {
            match Path::new(path).extension().and_then(|s| s.to_str()) {
                Some("txt") => "text/plain".to_string(),
                Some("pdf") => "application/pdf".to_string(),
                Some("png") => "image/png".to_string(),
                Some("jpg") | Some("jpeg") => "image/jpeg".to_string(),
                _ => "application/octet-stream".to_string(),
            }
        }
    }
}

pub struct Filesystem;

impl Filesystem {
    pub fn is_ignored_dir(dir: &str) -> bool {
        dir == "." || dir == ".."
    }
}

pub struct FilesHelper;

impl FilesHelper {
    pub fn determine_icon(file_info: &HashMap<String, String>) -> String {
        // Implement icon determination based on file type
        // Simplified version
        if file_info.get("type").map_or(false, |t| t == "dir") {
            "folder".to_string()
        } else {
            match file_info.get("mimetype") {
                Some(mime) if mime.starts_with("image/") => "image".to_string(),
                Some(mime) if mime.starts_with("text/") => "text".to_string(),
                Some(mime) if mime.starts_with("audio/") => "audio".to_string(),
                Some(mime) if mime.starts_with("video/") => "video".to_string(),
                Some(mime) if mime.contains("pdf") => "pdf".to_string(),
                _ => "file".to_string(),
            }
        }
    }
    
    pub fn file_cmp(a: &HashMap<String, String>, b: &HashMap<String, String>) -> std::cmp::Ordering {
        // Implement file comparison for sorting
        // This is a simplified version
        let a_name = a.get("name").unwrap_or(&String::new());
        let b_name = b.get("name").unwrap_or(&String::new());
        a_name.cmp(b_name)
    }
}

fn format_date(timestamp: i64) -> String {
    let dt = Utc.timestamp_opt(timestamp, 0).unwrap();
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[derive(Debug, thiserror::Error)]
pub enum TrashbinError {
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Other error: {0}")]
    Other(String),
}