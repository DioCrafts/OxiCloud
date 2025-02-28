// ownCloud
//
// @author Michael Gapczynski
// @copyright 2012 Michael Gapczynski mtgap@owncloud.com
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
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use async_trait::async_trait;
use chrono::DateTime;
use dropbox_sdk::{Dropbox as DropboxApi, DropboxOAuth, DropboxResult};
use log::{error, info};
use tempfile::NamedTempFile;

use crate::files::storage::common::Storage as CommonStorage;
use crate::files::stream::{CloseCallback, Dir};
use crate::helper::OcHelper;
use crate::util::OcpUtil;

pub struct Dropbox {
    dropbox: DropboxApi,
    root: String,
    id: String,
    meta_data: HashMap<String, HashMap<String, serde_json::Value>>,
    temp_files: HashMap<String, String>,
}

impl Dropbox {
    pub fn new(params: HashMap<String, String>) -> Result<Self, Box<dyn std::error::Error>> {
        if params.get("configured") == Some(&"true".to_string())
            && params.contains_key("app_key")
            && params.contains_key("app_secret")
            && params.contains_key("token")
            && params.contains_key("token_secret")
        {
            let root = params.get("root").cloned().unwrap_or_default();
            let id = format!(
                "dropbox::{}{}{}",
                params["app_key"], params["token"], &root
            );
            
            let oauth = DropboxOAuth::new(
                params["app_key"].clone(),
                params["app_secret"].clone(),
            );
            oauth.set_token(params["token"].clone(), params["token_secret"].clone());
            
            let dropbox = DropboxApi::new(oauth, "dropbox");
            
            Ok(Self {
                dropbox,
                root,
                id,
                meta_data: HashMap::new(),
                temp_files: HashMap::new(),
            })
        } else {
            Err("Creating Dropbox storage failed".into())
        }
    }

    async fn get_meta_data(&mut self, path: &str, list: bool) -> Option<HashMap<String, serde_json::Value>> {
        let full_path = format!("{}{}", self.root, path);
        
        if !list && self.meta_data.contains_key(&full_path) {
            return self.meta_data.get(&full_path).cloned();
        } else {
            if list {
                match self.dropbox.get_meta_data(&full_path).await {
                    Ok(response) => {
                        if let Some(contents) = response.get("contents") {
                            if let Some(contents_array) = contents.as_array() {
                                // Cache folder's contents
                                for file in contents_array {
                                    if let Some(file_path) = file.get("path") {
                                        if let Some(path_str) = file_path.as_str() {
                                            let basename = Path::new(path_str)
                                                .file_name()
                                                .and_then(|n| n.to_str())
                                                .unwrap_or_default();
                                            
                                            let mut file_map = HashMap::new();
                                            for (k, v) in file.as_object().unwrap() {
                                                file_map.insert(k.clone(), v.clone());
                                            }
                                            
                                            self.meta_data.insert(
                                                format!("{}/{}", full_path, basename),
                                                file_map
                                            );
                                        }
                                    }
                                }
                                
                                // Store folder metadata without contents
                                let mut folder_meta = HashMap::new();
                                for (k, v) in response.as_object().unwrap() {
                                    if k != "contents" {
                                        folder_meta.insert(k.clone(), v.clone());
                                    }
                                }
                                self.meta_data.insert(full_path.clone(), folder_meta);
                                
                                // Return contents
                                return Some(contents_array.iter()
                                    .map(|file| {
                                        let mut file_map = HashMap::new();
                                        for (k, v) in file.as_object().unwrap() {
                                            file_map.insert(k.clone(), v.clone());
                                        }
                                        file_map
                                    })
                                    .collect());
                            }
                        }
                        
                        self.meta_data.insert(full_path.clone(), response.clone());
                        Some(response)
                    },
                    Err(e) => {
                        error!("Dropbox error: {}", e);
                        None
                    }
                }
            } else {
                match self.dropbox.get_meta_data_with_children(&full_path, false).await {
                    Ok(response) => {
                        let response_map = response.clone();
                        self.meta_data.insert(full_path.clone(), response_map.clone());
                        Some(response_map)
                    },
                    Err(e) => {
                        error!("Dropbox error: {}", e);
                        None
                    }
                }
            }
        }
    }

    fn write_back(&mut self, tmp_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = self.temp_files.get(tmp_file) {
            let mut file = File::open(tmp_file)?;
            self.dropbox.put_file(path, &mut file)?;
            fs::remove_file(tmp_file)?;
        }
        Ok(())
    }
}

#[async_trait]
impl CommonStorage for Dropbox {
    fn get_id(&self) -> &str {
        &self.id
    }

    async fn mkdir(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let full_path = format!("{}{}", self.root, path);
        match self.dropbox.create_folder(&full_path).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Dropbox error creating folder: {}", e);
                Err(e.into())
            }
        }
    }

    async fn rmdir(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.unlink(path).await
    }

    async fn opendir(&mut self, path: &str) -> Result<Box<dyn Iterator<Item = String>>, Box<dyn std::error::Error>> {
        if let Some(contents) = self.get_meta_data(path, true).await {
            let files = contents
                .iter()
                .filter_map(|file| {
                    file.get("path")
                        .and_then(|p| p.as_str())
                        .map(|p| {
                            Path::new(p)
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or_default()
                                .to_string()
                        })
                })
                .collect::<Vec<String>>();
            
            Dir::register(&format!("dropbox{}", path), files.clone());
            
            Ok(Box::new(files.into_iter()))
        } else {
            Err("Could not open directory".into())
        }
    }

    async fn stat(&mut self, path: &str) -> Result<HashMap<String, i64>, Box<dyn std::error::Error>> {
        if let Some(meta_data) = self.get_meta_data(path, false).await {
            let mut stat = HashMap::new();
            
            if let Some(bytes) = meta_data.get("bytes").and_then(|b| b.as_i64()) {
                stat.insert("size".to_string(), bytes);
            } else {
                stat.insert("size".to_string(), 0);
            }
            
            stat.insert("atime".to_string(), SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64);
            
            if let Some(modified) = meta_data.get("modified").and_then(|m| m.as_str()) {
                if let Ok(time) = DateTime::parse_from_rfc3339(modified) {
                    stat.insert("mtime".to_string(), time.timestamp());
                } else {
                    stat.insert("mtime".to_string(), SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64);
                }
            } else {
                stat.insert("mtime".to_string(), SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64);
            }
            
            Ok(stat)
        } else {
            Err("Could not get file stats".into())
        }
    }

    async fn filetype(&mut self, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        if path.is_empty() || path == "/" {
            return Ok("dir".to_string());
        }
        
        if let Some(meta_data) = self.get_meta_data(path, false).await {
            if let Some(is_dir) = meta_data.get("is_dir").and_then(|d| d.as_str()) {
                if is_dir == "true" {
                    return Ok("dir".to_string());
                } else {
                    return Ok("file".to_string());
                }
            }
        }
        
        Err("Could not determine file type".into())
    }

    async fn is_readable(&mut self, path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        self.file_exists(path).await
    }

    async fn is_updatable(&mut self, path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        self.file_exists(path).await
    }

    async fn file_exists(&mut self, path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        if path.is_empty() || path == "/" {
            return Ok(true);
        }
        
        Ok(self.get_meta_data(path, false).await.is_some())
    }

    async fn unlink(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let full_path = format!("{}{}", self.root, path);
        match self.dropbox.delete(&full_path).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Dropbox error deleting file: {}", e);
                Err(e.into())
            }
        }
    }

    async fn rename(&mut self, path1: &str, path2: &str) -> Result<(), Box<dyn std::error::Error>> {
        let full_path1 = format!("{}{}", self.root, path1);
        let full_path2 = format!("{}{}", self.root, path2);
        
        match self.dropbox.move_file(&full_path1, &full_path2).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Dropbox error renaming file: {}", e);
                Err(e.into())
            }
        }
    }

    async fn copy(&mut self, path1: &str, path2: &str) -> Result<(), Box<dyn std::error::Error>> {
        let full_path1 = format!("{}{}", self.root, path1);
        let full_path2 = format!("{}{}", self.root, path2);
        
        match self.dropbox.copy_file(&full_path1, &full_path2).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Dropbox error copying file: {}", e);
                Err(e.into())
            }
        }
    }

    async fn fopen(&mut self, path: &str, mode: &str) -> Result<Box<dyn std::io::Read + Send>, Box<dyn std::error::Error>> {
        let full_path = format!("{}{}", self.root, path);
        
        match mode {
            "r" | "rb" => {
                let tmp_file = OcHelper::tmp_file("");
                
                match self.dropbox.get_file(&full_path).await {
                    Ok(data) => {
                        let mut file = File::create(&tmp_file)?;
                        file.write_all(&data)?;
                        let file = File::open(tmp_file)?;
                        Ok(Box::new(file))
                    },
                    Err(e) => {
                        error!("Dropbox error opening file for reading: {}", e);
                        Err(e.into())
                    }
                }
            },
            "w" | "wb" | "a" | "ab" | "r+" | "w+" | "wb+" | "a+" | "x" | "x+" | "c" | "c+" => {
                let ext = if let Some(pos) = path.rfind('.') {
                    &path[pos..]
                } else {
                    ""
                };
                
                let tmp_file = OcHelper::tmp_file(ext);
                
                if self.file_exists(path).await? {
                    if let Ok(mut source) = self.fopen(path, "r").await {
                        let mut data = Vec::new();
                        source.read_to_end(&mut data)?;
                        
                        let mut file = File::create(&tmp_file)?;
                        file.write_all(&data)?;
                    }
                }
                
                CloseCallback::register(&tmp_file, Box::new(move |tmp| {
                    // This is simplified - in a real implementation you would need to handle async properly
                    self.write_back(tmp)
                }));
                
                self.temp_files.insert(tmp_file.clone(), full_path);
                
                let file = File::open(tmp_file)?;
                Ok(Box::new(file))
            },
            _ => Err("Invalid file mode".into())
        }
    }

    async fn get_mime_type(&mut self, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        if self.filetype(path).await? == "dir" {
            return Ok("httpd/unix-directory".to_string());
        }
        
        if let Some(meta_data) = self.get_meta_data(path, false).await {
            if let Some(mime_type) = meta_data.get("mime_type").and_then(|m| m.as_str()) {
                return Ok(mime_type.to_string());
            }
        }
        
        Err("Could not determine mime type".into())
    }

    async fn free_space(&mut self, _path: &str) -> Result<i64, Box<dyn std::error::Error>> {
        match self.dropbox.get_account_info().await {
            Ok(info) => {
                if let (Some(quota), Some(normal)) = (
                    info.get("quota_info").and_then(|q| q.get("quota")).and_then(|q| q.as_i64()),
                    info.get("quota_info").and_then(|q| q.get("normal")).and_then(|q| q.as_i64())
                ) {
                    Ok(quota - normal)
                } else {
                    Err("Invalid quota information format".into())
                }
            },
            Err(e) => {
                error!("Dropbox error getting account info: {}", e);
                Err(e.into())
            }
        }
    }

    async fn touch(&mut self, _path: &str, _mtime: Option<SystemTime>) -> Result<(), Box<dyn std::error::Error>> {
        // Dropbox doesn't support touch operation
        Err("Operation not supported".into())
    }
}