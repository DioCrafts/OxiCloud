use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use curl::easy::{Easy, List};
use log::{error, info};
use reqwest::{Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use url::Url;

use crate::files::{FilesError, FilesResult, Storage, StorageCommon, StreamDir};
use crate::util::Config;

/// WebDAV storage implementation for external file storage
pub struct Dav {
    password: String,
    user: String,
    host: String,
    secure: bool,
    root: String,
    ready: bool,
    client: Option<webdav_rs::Client>,
    temp_files: Arc<Mutex<HashMap<String, String>>>,
}

impl Dav {
    pub fn new(params: HashMap<String, String>) -> FilesResult<Self> {
        if let (Some(host), Some(user), Some(password)) = (
            params.get("host"),
            params.get("user"),
            params.get("password"),
        ) {
            // Remove leading http[s], will be generated in create_base_uri()
            let host = if host.starts_with("https://") {
                host[8..].to_string()
            } else if host.starts_with("http://") {
                host[7..].to_string()
            } else {
                host.to_string()
            };

            let secure = match params.get("secure") {
                Some(secure_str) => {
                    if secure_str == "true" {
                        true
                    } else if let Ok(secure_bool) = secure_str.parse::<bool>() {
                        secure_bool
                    } else {
                        false
                    }
                }
                None => false,
            };

            let mut root = params.get("root").map_or("/".to_string(), |r| r.clone());
            if root.is_empty() || !root.starts_with('/') {
                root = format!("/{}", root);
            }
            if !root.ends_with('/') {
                root = format!("{}/", root);
            }

            Ok(Self {
                password: password.clone(),
                user: user.clone(),
                host,
                secure,
                root,
                ready: false,
                client: None,
                temp_files: Arc::new(Mutex::new(HashMap::new())),
            })
        } else {
            Err(FilesError::InvalidParams("Missing required WebDAV parameters".to_string()))
        }
    }

    fn init(&mut self) -> FilesResult<()> {
        if self.ready {
            return Ok(());
        }

        let base_uri = self.create_base_uri();
        let client = webdav_rs::Client::new(
            &base_uri,
            &self.user,
            &self.password,
        )?;

        // Handle certificate path
        if let Some(ca_view) = crate::files::get_storage("files_external") {
            let data_dir = Config::get_system_value("datadirectory")?;
            let cert_path = format!("{}{}/rootcerts.crt", data_dir, ca_view.get_absolute_path(""));
            
            if Path::new(&cert_path).exists() {
                // In real implementation, would integrate with client's cert store
                // client.add_trusted_certificates(&cert_path);
            }
        }

        self.client = Some(client);
        self.ready = true;
        
        Ok(())
    }

    fn create_base_uri(&self) -> String {
        let protocol = if self.secure { "https" } else { "http" };
        format!("{}://{}{}", protocol, self.host, self.root)
    }

    fn clean_path(&self, path: &str) -> String {
        if path.is_empty() || path.starts_with('/') {
            path[1..].to_string()
        } else {
            path.to_string()
        }
    }

    fn simple_response(&mut self, method: &str, path: &str, body: Option<&[u8]>, expected: u16) -> FilesResult<bool> {
        self.init()?;
        let path = self.clean_path(path);
        
        let client = self.client.as_ref().ok_or_else(|| 
            FilesError::NotInitialized("WebDAV client not initialized".to_string())
        )?;

        match client.request(method, &path, body) {
            Ok(response) => Ok(response.status().as_u16() == expected),
            Err(e) => {
                error!("WebDAV request failed: {}", e);
                Ok(false)
            }
        }
    }

    fn write_back(&mut self, tmp_file: &str) -> FilesResult<()> {
        let temp_files = self.temp_files.lock().map_err(|e| 
            FilesError::Internal(format!("Failed to lock temp_files: {}", e))
        )?;
        
        if let Some(path) = temp_files.get(tmp_file) {
            self.upload_file(tmp_file, path)?;
            fs::remove_file(tmp_file)?;
        }
        
        Ok(())
    }
}

#[async_trait]
impl Storage for Dav {
    fn get_id(&self) -> String {
        format!("webdav::{}@{}/{}", self.user, self.host, self.root)
    }

    fn mkdir(&mut self, path: &str) -> FilesResult<bool> {
        self.init()?;
        let path = self.clean_path(path);
        self.simple_response("MKCOL", &path, None, 201)
    }

    fn rmdir(&mut self, path: &str) -> FilesResult<bool> {
        self.init()?;
        let path = self.clean_path(path);
        self.simple_response("DELETE", &path, None, 204)
    }

    fn opendir(&mut self, path: &str) -> FilesResult<Option<Box<dyn Read>>> {
        self.init()?;
        let path = self.clean_path(path);
        
        let client = self.client.as_ref().ok_or_else(|| 
            FilesError::NotInitialized("WebDAV client not initialized".to_string())
        )?;
        
        match client.propfind(&path, &[], 1) {
            Ok(response) => {
                let id = format!("webdav{}{}", self.root, path);
                let mut content = Vec::new();
                
                let files: Vec<String> = response.keys().cloned().collect();
                if !files.is_empty() {
                    // Skip first entry (current directory)
                    for file in &files[1..] {
                        let file = urlencoding::decode(Path::new(file).file_name().unwrap_or_default().to_str().unwrap_or_default())
                            .map(|s| s.to_string())
                            .unwrap_or_default();
                        content.push(file);
                    }
                }
                
                let stream_dir = StreamDir::register(&id, content)?;
                Ok(Some(Box::new(stream_dir)))
            },
            Err(e) => {
                error!("Failed to list directory: {}", e);
                Ok(None)
            }
        }
    }

    fn filetype(&mut self, path: &str) -> FilesResult<Option<String>> {
        self.init()?;
        let path = self.clean_path(path);
        
        let client = self.client.as_ref().ok_or_else(|| 
            FilesError::NotInitialized("WebDAV client not initialized".to_string())
        )?;
        
        match client.propfind(&path, &["{DAV:}resourcetype"], 0) {
            Ok(response) => {
                if let Some(resource_type) = response.get("{DAV:}resourcetype") {
                    // In a real implementation, we'd actually parse the XML response
                    // This is a simplification assuming resource_type is a string or similar type
                    if resource_type.contains("{DAV:}collection") {
                        Ok(Some("dir".to_string()))
                    } else {
                        Ok(Some("file".to_string()))
                    }
                } else {
                    Ok(None)
                }
            },
            Err(e) => {
                error!("WebDAV propfind error: {}", e);
                log::error!("webdav client: {}", e);
                Ok(None)
            }
        }
    }

    fn is_readable(&self, _path: &str) -> FilesResult<bool> {
        // Not properly supported in the original code
        Ok(true)
    }

    fn is_updatable(&self, _path: &str) -> FilesResult<bool> {
        // Not properly supported in the original code
        Ok(true)
    }

    fn file_exists(&mut self, path: &str) -> FilesResult<bool> {
        self.init()?;
        let path = self.clean_path(path);
        
        let client = self.client.as_ref().ok_or_else(|| 
            FilesError::NotInitialized("WebDAV client not initialized".to_string())
        )?;
        
        match client.propfind(&path, &["{DAV:}resourcetype"], 0) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn unlink(&mut self, path: &str) -> FilesResult<bool> {
        self.init()?;
        self.simple_response("DELETE", path, None, 204)
    }

    fn fopen(&mut self, path: &str, mode: &str) -> FilesResult<Option<Box<dyn Read + Send>>> {
        self.init()?;
        let path = self.clean_path(path);
        
        match mode {
            "r" | "rb" => {
                if !self.file_exists(&path)? {
                    return Ok(None);
                }
                
                let mut easy = Easy::new();
                let url = format!("{}{}", self.create_base_uri(), path.replace(" ", "%20"));
                
                easy.url(&url)?;
                easy.username(&self.user)?;
                easy.password(&self.password)?;
                easy.follow_location(true)?;
                
                let mut data = Vec::new();
                {
                    let mut transfer = easy.transfer();
                    transfer.write_function(|new_data| {
                        data.extend_from_slice(new_data);
                        Ok(new_data.len())
                    })?;
                    transfer.perform()?;
                }
                
                let cursor = std::io::Cursor::new(data);
                Ok(Some(Box::new(cursor)))
            },
            "w" | "wb" | "a" | "ab" | "r+" | "w+" | "wb+" | "a+" | "x" | "x+" | "c" | "c+" => {
                // Emulate these modes with temp files
                let ext = Path::new(&path)
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|s| format!(".{}", s))
                    .unwrap_or_default();
                
                let mut tmp_file = tempfile::Builder::new()
                    .suffix(&ext)
                    .tempfile()?;
                
                if self.file_exists(&path)? {
                    self.get_file(&path, tmp_file.path().to_str().unwrap())?;
                }
                
                let tmp_path = tmp_file.path().to_str().unwrap().to_string();
                
                {
                    let mut temp_files = self.temp_files.lock().map_err(|e| 
                        FilesError::Internal(format!("Failed to lock temp_files: {}", e))
                    )?;
                    temp_files.insert(tmp_path.clone(), path);
                }
                
                // This doesn't exactly mimic the PHP functionality,
                // as we're returning the file handle directly instead of using the close:// scheme
                // A proper implementation would register the callback for cleanup
                let file = tmp_file.reopen()?;
                
                // We need to persist the tempfile so it's not deleted when tmp_file goes out of scope
                tmp_file.into_temp_path();
                
                Ok(Some(Box::new(file)))
            },
            _ => Err(FilesError::InvalidArgument(format!("Unsupported file mode: {}", mode))),
        }
    }

    fn free_space(&mut self, path: &str) -> FilesResult<i64> {
        self.init()?;
        let path = self.clean_path(path);
        
        let client = self.client.as_ref().ok_or_else(|| 
            FilesError::NotInitialized("WebDAV client not initialized".to_string())
        )?;
        
        match client.propfind(&path, &["{DAV:}quota-available-bytes"], 0) {
            Ok(response) => {
                if let Some(bytes) = response.get("{DAV:}quota-available-bytes") {
                    // In a real implementation, this would parse the value properly
                    if let Ok(bytes) = bytes.parse::<i64>() {
                        Ok(bytes)
                    } else {
                        Ok(0)
                    }
                } else {
                    Ok(0)
                }
            },
            Err(_) => Ok(-1), // SPACE_UNKNOWN
        }
    }

    fn touch(&mut self, path: &str, mtime: Option<i64>) -> FilesResult<()> {
        self.init()?;
        let mtime = mtime.unwrap_or_else(|| chrono::Utc::now().timestamp());
        let path = self.clean_path(path);
        
        let client = self.client.as_ref().ok_or_else(|| 
            FilesError::NotInitialized("WebDAV client not initialized".to_string())
        )?;
        
        if self.file_exists(&path)? {
            // Update the mtime via proppatch
            let properties = HashMap::from([
                ("{DAV:}lastmodified".to_string(), mtime.to_string())
            ]);
            
            match client.proppatch(&path, properties) {
                Ok(_) => Ok(()),
                Err(e) => Err(FilesError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other, 
                    format!("Failed to update mtime: {}", e)
                ))),
            }
        } else {
            // Create empty file
            match client.put(&path, &[]) {
                Ok(_) => Ok(()),
                Err(e) => Err(FilesError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other, 
                    format!("Failed to create file: {}", e)
                ))),
            }
        }
    }

    fn get_file(&mut self, path: &str, target: &str) -> FilesResult<()> {
        self.init()?;
        let mut source = match self.fopen(path, "r")? {
            Some(file) => file,
            None => return Err(FilesError::NotFound(format!("File not found: {}", path))),
        };
        
        let mut target_file = File::create(target)?;
        let mut buffer = Vec::new();
        source.read_to_end(&mut buffer)?;
        target_file.write_all(&buffer)?;
        
        Ok(())
    }

    fn upload_file(&mut self, path: &str, target: &str) -> FilesResult<()> {
        self.init()?;
        let mut source_file = File::open(path)?;
        let mut source_data = Vec::new();
        source_file.read_to_end(&mut source_data)?;
        
        let mut easy = Easy::new();
        let url = format!("{}{}", self.create_base_uri(), target.replace(" ", "%20"));
        
        easy.url(&url)?;
        easy.username(&self.user)?;
        easy.password(&self.password)?;
        easy.upload(true)?;
        easy.in_filesize(source_data.len() as u64)?;
        
        let mut data_cursor = std::io::Cursor::new(source_data);
        
        let mut transfer = easy.transfer();
        transfer.read_function(|buf| {
            let mut slice = Vec::new();
            let n = data_cursor.read(buf).unwrap_or(0);
            slice.extend_from_slice(&buf[..n]);
            Ok(slice)
        })?;
        
        transfer.perform()?;
        
        Ok(())
    }

    fn rename(&mut self, path1: &str, path2: &str) -> FilesResult<bool> {
        self.init()?;
        let path1 = self.clean_path(path1);
        let path2 = format!("{}{}", self.root, self.clean_path(path2));
        
        let client = self.client.as_ref().ok_or_else(|| 
            FilesError::NotInitialized("WebDAV client not initialized".to_string())
        )?;
        
        let mut headers = HashMap::new();
        headers.insert("Destination".to_string(), path2);
        
        match client.request_with_headers("MOVE", &path1, None, headers) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn copy(&mut self, path1: &str, path2: &str) -> FilesResult<bool> {
        self.init()?;
        let path1 = self.clean_path(path1);
        let path2 = format!("{}{}", self.root, self.clean_path(path2));
        
        let client = self.client.as_ref().ok_or_else(|| 
            FilesError::NotInitialized("WebDAV client not initialized".to_string())
        )?;
        
        let mut headers = HashMap::new();
        headers.insert("Destination".to_string(), path2);
        
        match client.request_with_headers("COPY", &path1, None, headers) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn stat(&mut self, path: &str) -> FilesResult<Option<HashMap<String, i64>>> {
        self.init()?;
        let path = self.clean_path(path);
        
        let client = self.client.as_ref().ok_or_else(|| 
            FilesError::NotInitialized("WebDAV client not initialized".to_string())
        )?;
        
        match client.propfind(&path, &["{DAV:}getlastmodified", "{DAV:}getcontentlength"], 0) {
            Ok(response) => {
                let mut stats = HashMap::new();
                
                if let Some(last_modified) = response.get("{DAV:}getlastmodified") {
                    // Assuming parsing of the date string to timestamp
                    if let Ok(dt) = chrono::DateTime::parse_from_rfc2822(last_modified) {
                        stats.insert("mtime".to_string(), dt.timestamp());
                    }
                }
                
                let content_length = response.get("{DAV:}getcontentlength")
                    .and_then(|s| s.parse::<i64>().ok())
                    .unwrap_or(0);
                
                stats.insert("size".to_string(), content_length);
                
                Ok(Some(stats))
            },
            Err(_) => Ok(None),
        }
    }

    fn get_mime_type(&mut self, path: &str) -> FilesResult<Option<String>> {
        self.init()?;
        let path = self.clean_path(path);
        
        let client = self.client.as_ref().ok_or_else(|| 
            FilesError::NotInitialized("WebDAV client not initialized".to_string())
        )?;
        
        match client.propfind(&path, &["{DAV:}getcontenttype", "{DAV:}resourcetype"], 0) {
            Ok(response) => {
                if let Some(resource_type) = response.get("{DAV:}resourcetype") {
                    if resource_type.contains("{DAV:}collection") {
                        return Ok(Some("httpd/unix-directory".to_string()));
                    }
                }
                
                if let Some(content_type) = response.get("{DAV:}getcontenttype") {
                    Ok(Some(content_type.clone()))
                } else {
                    Ok(None)
                }
            },
            Err(_) => Ok(None),
        }
    }
}

impl StorageCommon for Dav {}