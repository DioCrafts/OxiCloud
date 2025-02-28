// ownCloud
//
// @author Christian Berendt
// @copyright 2013 Christian Berendt berendt@b1-systems.de
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
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use log::{error, info};
use openstack::ObjectStore;
use openstack::{Container, DataObject};
use tempfile::NamedTempFile;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

use crate::files::storage::common::Storage;
use crate::files::stream::dir::register_dir;
use crate::files::stream::close::register_callback;
use crate::helpers::{get_mime_type, tmp_file};
use crate::util::log;

lazy_static! {
    static ref TMP_FILES: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub struct Swift {
    id: String,
    connection: ObjectStore,
    container: Container,
    anchor: openstack::OpenStack,
    bucket: String,
}

impl Swift {
    pub async fn new(params: HashMap<String, String>) -> Result<Self, Box<dyn std::error::Error>> {
        if (!params.contains_key("key") && !params.contains_key("password"))
            || !params.contains_key("user")
            || !params.contains_key("bucket")
            || !params.contains_key("region")
        {
            return Err("API Key or password, Username, Bucket and Region have to be configured.".into());
        }

        let user = params.get("user").unwrap();
        let bucket = params.get("bucket").unwrap();
        let id = format!("swift::{}{}",user, md5::compute(bucket.as_bytes()).to_string());
        
        let url = params.get("url").unwrap_or(&"https://identity.api.rackspacecloud.com/v2.0/".to_string()).clone();
        let service_name = params.get("service_name").unwrap_or(&"cloudFiles".to_string()).clone();
        let region = params.get("region").unwrap().clone();

        let mut settings = HashMap::new();
        settings.insert("username".to_string(), user.clone());
        
        if let Some(password) = params.get("password") {
            settings.insert("password".to_string(), password.clone());
        } else if let Some(key) = params.get("key") {
            settings.insert("apiKey".to_string(), key.clone());
        }
        
        if let Some(tenant) = params.get("tenant") {
            settings.insert("tenantName".to_string(), tenant.clone());
        }

        let anchor = openstack::OpenStack::new(&url, settings)?;

        if let Some(timeout) = params.get("timeout") {
            let timeout_secs = timeout.parse::<u64>()?;
            anchor.set_http_timeout(timeout_secs);
        }

        let connection = anchor.object_store(&service_name, &region, "publicURL")?;
        
        let container = match connection.container(&bucket).await {
            Ok(container) => container,
            Err(_) => {
                let mut container = connection.new_container()?;
                let mut container_params = HashMap::new();
                container_params.insert("name".to_string(), bucket.clone());
                container.create(container_params).await?;
                container
            }
        };

        let mut swift = Self {
            id,
            connection,
            container,
            anchor,
            bucket: bucket.clone(),
        };

        if !swift.file_exists(".").await? {
            swift.mkdir(".").await?;
        }

        Ok(swift)
    }

    fn normalize_path(&self, path: &str) -> String {
        let path = path.trim_matches('/');
        
        if path.is_empty() {
            ".".to_string()
        } else {
            path.to_string()
        }
    }

    async fn does_object_exist(&self, path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        match self.container.data_object(path).await {
            Ok(_) => Ok(true),
            Err(e) => {
                error!("Object fetch error: {}", e);
                Ok(false)
            }
        }
    }

    pub async fn write_back(&self, tmp_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let tmp_files = TMP_FILES.lock().unwrap();
        
        if let Some(path) = tmp_files.get(tmp_file) {
            let mut object = self.container.new_data_object()?;
            let mime_type = get_mime_type(tmp_file);
            
            let mut params = HashMap::new();
            params.insert("name".to_string(), path.clone());
            params.insert("content_type".to_string(), mime_type);
            
            object.create_from_file(params, tmp_file).await?;
            fs::remove_file(tmp_file)?;
            Ok(())
        } else {
            Err("Temporary file not found".into())
        }
    }
}

#[async_trait]
impl Storage for Swift {
    async fn mkdir(&self, path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let path = self.normalize_path(path);
        
        if self.is_dir(&path).await? {
            return Ok(false);
        }
        
        let mut full_path = path.clone();
        if path != "." {
            full_path = format!("{}/", path);
        }
        
        let mut object = self.container.new_data_object()?;
        let mut params = HashMap::new();
        params.insert("name".to_string(), full_path);
        params.insert("content_type".to_string(), "httpd/unix-directory".to_string());
        
        match object.create(params).await {
            Ok(_) => Ok(true),
            Err(e) => {
                error!("Failed to create directory: {}", e);
                Ok(false)
            }
        }
    }

    async fn file_exists(&self, path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let path = self.normalize_path(path);
        
        if path != "." && self.is_dir(&path).await? {
            return self.does_object_exist(&format!("{}/", path)).await;
        }
        
        self.does_object_exist(&path).await
    }

    async fn rmdir(&self, path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let path = self.normalize_path(path);
        
        if !self.is_dir(&path).await? {
            return Ok(false);
        }
        
        let entries = self.opendir(&path).await?;
        for entry in entries {
            if entry == "." || entry == ".." {
                continue;
            }
            
            let entry_path = format!("{}/{}", path, entry);
            if self.is_dir(&entry_path).await? {
                self.rmdir(&entry_path).await?;
            } else {
                self.unlink(&entry_path).await?;
            }
        }
        
        let object_path = format!("{}/", path);
        match self.container.data_object(&object_path).await {
            Ok(object) => {
                match object.delete().await {
                    Ok(_) => Ok(true),
                    Err(e) => {
                        error!("Failed to delete directory: {}", e);
                        Ok(false)
                    }
                }
            },
            Err(e) => {
                error!("Failed to get directory object: {}", e);
                Ok(false)
            }
        }
    }

    async fn opendir(&self, path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let path = self.normalize_path(path);
        
        let prefix = if path == "." {
            "".to_string()
        } else {
            format!("{}/", path)
        };
        
        let mut files = Vec::new();
        let options = HashMap::from([
            ("prefix".to_string(), prefix.clone()),
            ("delimiter".to_string(), "/".to_string()),
        ]);
        
        let objects = self.container.object_list(options).await?;
        
        for object in objects {
            let file = Path::new(&object.name)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
                
            if !file.is_empty() && file != Path::new(&prefix).file_name().and_then(|n| n.to_str()).unwrap_or("") {
                files.push(file);
            }
        }
        
        register_dir(&format!("swift{}", path), files.clone())?;
        Ok(files)
    }

    async fn stat(&self, path: &str) -> Result<HashMap<String, i64>, Box<dyn std::error::Error>> {
        let path = self.normalize_path(path);
        
        let object_path = if self.is_dir(&path).await? && path != "." {
            format!("{}/", path)
        } else {
            path
        };
        
        let object = self.container.data_object(&object_path).await?;
        let headers = object.headers();
        
        let mut mtime = headers.get("X-Timestamp")
            .and_then(|t| t.parse::<i64>().ok())
            .unwrap_or_else(|| SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64);
            
        if let Some(timestamp) = headers.get("X-Object-Meta-Timestamp") {
            if let Ok(ts) = timestamp.parse::<i64>() {
                mtime = ts;
            }
        }
        
        let mut stat = HashMap::new();
        stat.insert("size".to_string(), object.content_length() as i64);
        stat.insert("mtime".to_string(), mtime);
        stat.insert("atime".to_string(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64);
        
        Ok(stat)
    }

    async fn filetype(&self, path: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let path = self.normalize_path(path);
        
        if path != "." && self.does_object_exist(&path).await? {
            return Ok(Some("file".to_string()));
        }
        
        if path != "." {
            let dir_path = format!("{}/", path);
            if self.does_object_exist(&dir_path).await? {
                return Ok(Some("dir".to_string()));
            }
        } else if self.does_object_exist(&path).await? {
            return Ok(Some("dir".to_string()));
        }
        
        Ok(None)
    }

    async fn is_readable(&self, _path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(true)
    }

    async fn is_updatable(&self, _path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(true)
    }

    async fn unlink(&self, path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let path = self.normalize_path(path);
        
        match self.container.data_object(&path).await {
            Ok(object) => {
                match object.delete().await {
                    Ok(_) => Ok(true),
                    Err(e) => {
                        error!("Failed to delete file: {}", e);
                        Ok(false)
                    }
                }
            },
            Err(e) => {
                error!("Failed to get file object: {}", e);
                Ok(false)
            }
        }
    }

    async fn fopen(&self, path: &str, mode: &str) -> Result<Option<File>, Box<dyn std::error::Error>> {
        let path = self.normalize_path(path);
        
        match mode {
            "r" | "rb" => {
                let tmp_file = tmp_file(None)?;
                let tmp_path = tmp_file.path().to_str().unwrap().to_string();
                
                {
                    let mut tmp_files = TMP_FILES.lock().unwrap();
                    tmp_files.insert(tmp_path.clone(), path.clone());
                }
                
                let object = self.container.data_object(&path).await?;
                object.save_to_filename(&tmp_path).await?;
                
                let file = File::open(&tmp_path)?;
                Ok(Some(file))
            },
            "w" | "wb" | "a" | "ab" | "r+" | "w+" | "wb+" | "a+" | "x" | "x+" | "c" | "c+" => {
                let ext = Path::new(&path)
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|e| format!(".{}", e))
                    .unwrap_or_default();
                    
                let tmp_file = tmp_file(Some(&ext))?;
                let tmp_path = tmp_file.path().to_str().unwrap().to_string();
                
                register_callback(&tmp_path, move |f| {
                    let swift = self.clone();
                    Box::pin(async move {
                        swift.write_back(f).await.unwrap_or_else(|e| {
                            error!("Failed to write back file: {}", e);
                        });
                    })
                })?;
                
                if self.file_exists(&path).await? {
                    let source = self.fopen(&path, "r").await?;
                    if let Some(mut source) = source {
                        let mut content = Vec::new();
                        source.read_to_end(&mut content)?;
                        let mut dest = File::create(&tmp_path)?;
                        dest.write_all(&content)?;
                    }
                }
                
                {
                    let mut tmp_files = TMP_FILES.lock().unwrap();
                    tmp_files.insert(tmp_path.clone(), path);
                }
                
                let file = OpenOptions::new()
                    .read(mode.contains('+') || mode.contains('r'))
                    .write(true)
                    .append(mode.contains('a'))
                    .create(true)
                    .open(&format!("close://{}", tmp_path))
                    .await?;
                
                Ok(Some(File::open(&format!("close://{}", tmp_path))?))
            },
            _ => Err("Unsupported file mode".into())
        }
    }

    async fn get_mime_type(&self, path: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let path = self.normalize_path(path);
        
        if self.is_dir(&path).await? {
            return Ok(Some("httpd/unix-directory".to_string()));
        } else if self.file_exists(&path).await? {
            let object = self.container.data_object(&path).await?;
            let headers = object.headers();
            
            if let Some(content_type) = headers.get("Content-Type") {
                return Ok(Some(content_type.clone()));
            }
        }
        
        Ok(None)
    }

    async fn touch(&self, path: &str, mtime: Option<i64>) -> Result<(), Box<dyn std::error::Error>> {
        let path = self.normalize_path(path);
        let timestamp = mtime.unwrap_or_else(|| SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64);
        
        if self.file_exists(&path).await? {
            let obj_path = if self.is_dir(&path).await? && path != "." {
                format!("{}/", path)
            } else {
                path.clone()
            };
            
            let object = self.container.data_object(&obj_path).await?;
            
            let mut settings = HashMap::new();
            settings.insert("name".to_string(), obj_path);
            
            let mut extra_headers = HashMap::new();
            extra_headers.insert("X-Object-Meta-Timestamp".to_string(), timestamp.to_string());
            settings.insert("extra_headers".to_string(), serde_json::to_string(&extra_headers)?);
            
            object.update(settings).await?;
        } else {
            let mut object = self.container.new_data_object()?;
            
            let mut settings = HashMap::new();
            settings.insert("name".to_string(), path);
            settings.insert("content_type".to_string(), "text/plain".to_string());
            
            let mut extra_headers = HashMap::new();
            extra_headers.insert("X-Object-Meta-Timestamp".to_string(), timestamp.to_string());
            settings.insert("extra_headers".to_string(), serde_json::to_string(&extra_headers)?);
            
            object.create(settings).await?;
        }
        
        Ok(())
    }

    async fn copy(&self, path1: &str, path2: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let path1 = self.normalize_path(path1);
        let path2 = self.normalize_path(path2);
        
        if self.is_file(&path1).await? {
            let source = self.container.data_object(&path1).await?;
            let mut target = self.container.new_data_object()?;
            
            let mut params = HashMap::new();
            params.insert("name".to_string(), path2);
            target.create(params).await?;
            
            match source.copy(&target).await {
                Ok(_) => Ok(true),
                Err(e) => {
                    error!("Failed to copy file: {}", e);
                    Ok(false)
                }
            }
        } else {
            if self.file_exists(&path2).await? {
                return Ok(false);
            }
            
            let source_path = format!("{}/", path1);
            let target_path = format!("{}/", path2);
            
            let source = self.container.data_object(&source_path).await?;
            let mut target = self.container.new_data_object()?;
            
            let mut params = HashMap::new();
            params.insert("name".to_string(), target_path);
            target.create(params).await?;
            
            match source.copy(&target).await {
                Ok(_) => {
                    let entries = self.opendir(&path1).await?;
                    for entry in entries {
                        if entry == "." || entry == ".." {
                            continue;
                        }
                        
                        let source = format!("{}/{}", path1, entry);
                        let target = format!("{}/{}", path2, entry);
                        self.copy(&source, &target).await?;
                    }
                    Ok(true)
                },
                Err(e) => {
                    error!("Failed to copy directory: {}", e);
                    Ok(false)
                }
            }
        }
    }

    async fn rename(&self, path1: &str, path2: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let path1 = self.normalize_path(path1);
        let path2 = self.normalize_path(path2);
        
        if self.is_file(&path1).await? {
            if !self.copy(&path1, &path2).await? {
                return Ok(false);
            }
            
            if !self.unlink(&path1).await? {
                self.unlink(&path2).await?;
                return Ok(false);
            }
        } else {
            if self.file_exists(&path2).await? {
                return Ok(false);
            }
            
            if !self.copy(&path1, &path2).await? {
                return Ok(false);
            }
            
            if !self.rmdir(&path1).await? {
                self.rmdir(&path2).await?;
                return Ok(false);
            }
        }
        
        Ok(true)
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl Swift {
    pub fn get_connection(&self) -> ObjectStore {
        self.connection.clone()
    }
}