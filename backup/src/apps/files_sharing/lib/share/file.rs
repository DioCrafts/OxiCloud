// Copyright (c) 2012 Michael Gapczynski mtgap@owncloud.com
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::path::Path;
use async_trait::async_trait;

/// Format constants for the file backend
pub struct FormatFileBackend;

impl FormatFileBackend {
    pub const FORMAT_SHARED_STORAGE: u8 = 0;
    pub const FORMAT_GET_FOLDER_CONTENTS: u8 = 1;
    pub const FORMAT_FILE_APP_ROOT: u8 = 2;
    pub const FORMAT_OPENDIR: u8 = 3;
    pub const FORMAT_GET_ALL: u8 = 4;
    pub const FORMAT_PERMISSIONS: u8 = 5;
}

/// Trait for file-dependent sharing backends
#[async_trait]
pub trait ShareBackendFileDependent {
    async fn is_valid_source(&mut self, item_source: i64, uid_owner: &str) -> bool;
    async fn get_file_path(&mut self, item_source: i64, uid_owner: &str) -> Option<String>;
    fn generate_target(&self, file_path: &str, share_with: &str, exclude: Option<Vec<String>>) -> String;
    fn format_items(&self, items: Vec<HashMap<String, String>>, format: u8, parameters: Option<HashMap<String, String>>) -> Vec<HashMap<String, String>>;
}

/// File sharing backend implementation
pub struct OcShareBackendFile {
    path: Option<String>,
    db: crate::db::Database,
}

impl OcShareBackendFile {
    pub fn new(db: crate::db::Database) -> Self {
        Self {
            path: None,
            db,
        }
    }
    
    /// Gets the source information for a target path
    pub async fn get_source(target: &str) -> Option<HashMap<String, String>> {
        if target.is_empty() {
            return None;
        }
        
        let target = format!("/{}", target.trim_end_matches('/'));
        
        if let Some(pos) = target[1..].find('/') {
            let pos = pos + 1; // Adjust for the prefix slash
            let folder = &target[0..pos];
            
            if let Some(mut source) = crate::share::get_item_shared_with("folder", folder, FormatFileBackend::FORMAT_SHARED_STORAGE).await {
                let path_extension = &target[folder.len()..];
                source.insert("path".to_string(), format!("{}{}", source.get("path").unwrap_or(&String::new()), path_extension));
                return Some(source);
            }
        } else {
            if let Some(source) = crate::share::get_item_shared_with("file", &target, FormatFileBackend::FORMAT_SHARED_STORAGE).await {
                return Some(source);
            }
        }
        
        let mut source = None;
        
        if let Some(source_data) = source.as_mut() {
            if let Some(parent) = source_data.get("parent") {
                let mut parent_id = parent.to_string();
                let mut file_owner = String::new();
                
                while !parent_id.is_empty() {
                    let query = "SELECT parent, uid_owner FROM *PREFIX*share WHERE id = ?";
                    if let Some(item) = self.db.query_row(query, &[&parent_id]).await {
                        if let Some(new_parent) = item.get("parent") {
                            parent_id = new_parent.to_string();
                        } else {
                            file_owner = item.get("uid_owner").unwrap_or_default().to_string();
                            break;
                        }
                    } else {
                        break;
                    }
                }
                
                source_data.insert("fileOwner".to_string(), file_owner);
            } else if let Some(uid_owner) = source_data.get("uid_owner") {
                source_data.insert("fileOwner".to_string(), uid_owner.to_string());
            }
            
            return source;
        }
        
        crate::util::write_log("files_sharing", &format!("File source not found for: {}", target), crate::util::LogLevel::Error);
        None
    }
}

#[async_trait]
impl ShareBackendFileDependent for OcShareBackendFile {
    async fn is_valid_source(&mut self, item_source: i64, _uid_owner: &str) -> bool {
        let query = "SELECT name FROM *PREFIX*filecache WHERE fileid = ?";
        if let Some(row) = self.db.query_row(query, &[&item_source.to_string()]).await {
            if let Some(name) = row.get("name") {
                self.path = Some(name.to_string());
                return true;
            }
        }
        false
    }

    async fn get_file_path(&mut self, _item_source: i64, _uid_owner: &str) -> Option<String> {
        self.path.take()
    }

    fn generate_target(&self, file_path: &str, _share_with: &str, exclude: Option<Vec<String>>) -> String {
        let file_name = Path::new(file_path).file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        let mut target = format!("/{}", file_name);
        
        if let Some(exclude_list) = exclude {
            if !exclude_list.is_empty() {
                if let Some(pos) = target.rfind('.') {
                    let name = &target[0..pos];
                    let ext = &target[pos..];
                    
                    let mut i = 2;
                    let mut append = String::new();
                    
                    while exclude_list.contains(&format!("{}{}{}", name, append, ext)) {
                        append = format!(" ({})", i);
                        i += 1;
                    }
                    
                    target = format!("{}{}{}", name, append, ext);
                } else {
                    let name = target.as_str();
                    
                    let mut i = 2;
                    let mut append = String::new();
                    
                    while exclude_list.contains(&format!("{}{}", name, append)) {
                        append = format!(" ({})", i);
                        i += 1;
                    }
                    
                    target = format!("{}{}", name, append);
                }
            }
        }
        
        target
    }

    fn format_items(&self, items: Vec<HashMap<String, String>>, format: u8, _parameters: Option<HashMap<String, String>>) -> Vec<HashMap<String, String>> {
        if items.is_empty() {
            return Vec::new();
        }
        
        match format {
            FormatFileBackend::FORMAT_SHARED_STORAGE => {
                let item = &items[0];
                let mut result = HashMap::new();
                
                for key in &["parent", "path", "storage", "permissions", "uid_owner"] {
                    if let Some(value) = item.get(*key) {
                        result.insert(key.to_string(), value.to_string());
                    }
                }
                
                vec![result]
            },
            
            FormatFileBackend::FORMAT_GET_FOLDER_CONTENTS => {
                let mut files = Vec::new();
                
                for item in items {
                    let mut file = HashMap::new();
                    file.insert("fileid".to_string(), item.get("file_source").unwrap_or(&String::new()).to_string());
                    file.insert("storage".to_string(), item.get("storage").unwrap_or(&String::new()).to_string());
                    file.insert("path".to_string(), item.get("file_target").unwrap_or(&String::new()).to_string());
                    file.insert("parent".to_string(), item.get("file_parent").unwrap_or(&String::new()).to_string());
                    
                    if let Some(file_target) = item.get("file_target") {
                        if let Some(name) = Path::new(file_target).file_name().and_then(|n| n.to_str()) {
                            file.insert("name".to_string(), name.to_string());
                        }
                    }
                    
                    for key in &["mimetype", "mimepart", "size", "mtime", "encrypted", "etag"] {
                        if let Some(value) = item.get(*key) {
                            file.insert(key.to_string(), value.to_string());
                        }
                    }
                    
                    files.push(file);
                }
                
                files
            },
            
            FormatFileBackend::FORMAT_FILE_APP_ROOT => {
                let mut mtime: i64 = 0;
                let mut size: i64 = 0;
                
                for item in &items {
                    if let Some(item_mtime) = item.get("mtime").and_then(|m| m.parse::<i64>().ok()) {
                        if item_mtime > mtime {
                            mtime = item_mtime;
                        }
                    }
                    
                    if let Some(item_size) = item.get("size").and_then(|s| s.parse::<i64>().ok()) {
                        size += item_size;
                    }
                }
                
                let mut result = HashMap::new();
                result.insert("fileid".to_string(), "-1".to_string());
                result.insert("name".to_string(), "Shared".to_string());
                result.insert("mtime".to_string(), mtime.to_string());
                result.insert("mimetype".to_string(), "httpd/unix-directory".to_string());
                result.insert("size".to_string(), size.to_string());
                
                vec![result]
            },
            
            FormatFileBackend::FORMAT_OPENDIR => {
                let mut files = Vec::new();
                
                for item in items {
                    if let Some(file_target) = item.get("file_target") {
                        if let Some(name) = Path::new(file_target).file_name().and_then(|n| n.to_str()) {
                            let mut result = HashMap::new();
                            result.insert("name".to_string(), name.to_string());
                            files.push(result);
                        }
                    }
                }
                
                files
            },
            
            FormatFileBackend::FORMAT_GET_ALL => {
                let mut ids = Vec::new();
                
                for item in items {
                    if let Some(file_source) = item.get("file_source") {
                        let mut result = HashMap::new();
                        result.insert("id".to_string(), file_source.to_string());
                        ids.push(result);
                    }
                }
                
                ids
            },
            
            FormatFileBackend::FORMAT_PERMISSIONS => {
                let mut permissions = Vec::new();
                
                for item in items {
                    let mut permission = HashMap::new();
                    if let Some(file_source) = item.get("file_source") {
                        if let Some(file_perm) = item.get("permissions") {
                            permission.insert(file_source.to_string(), file_perm.to_string());
                            permissions.push(permission);
                        }
                    }
                }
                
                permissions
            },
            
            _ => Vec::new(),
        }
    }
}