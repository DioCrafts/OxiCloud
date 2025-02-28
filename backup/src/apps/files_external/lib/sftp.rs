/**
 * Copyright (c) 2012 Henrik Kjölhede <hkjolhede@gmail.com>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use ssh2::{Session, Sftp};
use std::net::TcpStream;
use tempfile::NamedTempFile;

use crate::common::Storage;
use crate::files::{DirStream, FileInfo, StorageResult};
use crate::helper;
use crate::stream::CloseCallback;

lazy_static! {
    static ref TEMP_FILES: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub struct SFTP {
    host: String,
    user: String,
    password: String,
    root: String,
    client: Option<(Session, Sftp)>,
}

impl SFTP {
    pub fn new(params: HashMap<String, String>) -> StorageResult<Self> {
        let host = params.get("host").ok_or("Missing host parameter")?.to_string();
        let host = if let Some(proto_pos) = host.find("://") {
            host[proto_pos + 3..].to_string()
        } else {
            host
        };

        let user = params.get("user").ok_or("Missing user parameter")?.to_string();
        let password = params.get("password").ok_or("Missing password parameter")?.to_string();
        
        let root = params.get("root")
            .map(|r| Self::clean_path(r))
            .unwrap_or_else(|| "/".to_string());
        
        let root = if !root.starts_with('/') {
            format!("/{}", root)
        } else {
            root
        };
        
        let root = if !root.ends_with('/') {
            format!("{}/", root)
        } else {
            root
        };

        let mut sftp = SFTP {
            host,
            user,
            password,
            root,
            client: None,
        };

        sftp.connect()?;
        
        Ok(sftp)
    }

    fn connect(&mut self) -> StorageResult<()> {
        let tcp = TcpStream::connect(&self.host)?;
        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;
        
        session.userauth_password(&self.user, &self.password)?;
        
        // Verify host key
        let host_keys = self.read_host_keys();
        let current_host_key = session.host_key().ok_or("Could not get host key")?;
        
        if let Some(existing_key) = host_keys.get(&self.host) {
            if existing_key != &current_host_key {
                return Err("Host public key does not match known key".into());
            }
        } else {
            let mut updated_keys = host_keys;
            updated_keys.insert(self.host.clone(), current_host_key.to_vec());
            self.write_host_keys(&updated_keys)?;
        }
        
        let sftp = session.sftp()?;
        self.client = Some((session, sftp));
        
        Ok(())
    }

    fn abs_path(&self, path: &str) -> String {
        format!("{}{}", self.root, Self::clean_path(path))
    }

    fn host_keys_path() -> StorageResult<PathBuf> {
        let storage_view = helper::get_storage("files_external")?;
        let data_dir = helper::get_data_directory()?;
        
        Ok(data_dir.join(storage_view).join("ssh_hostKeys"))
    }

    fn write_host_keys(&self, keys: &HashMap<String, Vec<u8>>) -> StorageResult<()> {
        let key_path = Self::host_keys_path()?;
        let mut file = File::create(key_path)?;
        
        for (host, key) in keys {
            let key_str = base64::encode(key);
            writeln!(file, "{}::{}", host, key_str)?;
        }
        
        Ok(())
    }

    fn read_host_keys(&self) -> HashMap<String, Vec<u8>> {
        let mut hosts_keys = HashMap::new();
        
        if let Ok(key_path) = Self::host_keys_path() {
            if let Ok(content) = fs::read_to_string(key_path) {
                for line in content.lines() {
                    if let Some(pos) = line.find("::") {
                        let (host, key_str) = line.split_at(pos);
                        let key_str = &key_str[2..]; // Skip the "::"
                        
                        if let Ok(key) = base64::decode(key_str) {
                            hosts_keys.insert(host.to_string(), key);
                        }
                    }
                }
            }
        }
        
        hosts_keys
    }

    fn clean_path(path: &str) -> String {
        // Simplified path cleaning implementation
        let path = path.trim();
        if path.is_empty() {
            return ".".to_string();
        }
        
        // Remove duplicate slashes
        let mut result = String::new();
        let mut last_was_slash = false;
        
        for c in path.chars() {
            if c == '/' {
                if !last_was_slash {
                    result.push(c);
                }
                last_was_slash = true;
            } else {
                result.push(c);
                last_was_slash = false;
            }
        }
        
        result
    }
    
    fn get_client(&self) -> StorageResult<&Sftp> {
        match &self.client {
            Some((_, sftp)) => Ok(sftp),
            None => Err("Not connected".into()),
        }
    }
    
    fn get_file(&self, path: &str, target: &Path) -> StorageResult<()> {
        let client = self.get_client()?;
        let mut remote_file = client.open(&Path::new(path))?;
        let mut local_file = File::create(target)?;
        
        let mut buffer = Vec::new();
        remote_file.read_to_end(&mut buffer)?;
        local_file.write_all(&buffer)?;
        
        Ok(())
    }
    
    fn upload_file(&self, source: &Path, target: &str) -> StorageResult<()> {
        let client = self.get_client()?;
        let mut local_file = File::open(source)?;
        let mut remote_file = client.create(&Path::new(target))?;
        
        let mut buffer = Vec::new();
        local_file.read_to_end(&mut buffer)?;
        remote_file.write_all(&buffer)?;
        
        Ok(())
    }
    
    pub fn write_back(tmp_file: &str) -> StorageResult<()> {
        let mut temp_files = TEMP_FILES.lock().unwrap();
        
        if let Some(abs_path) = temp_files.remove(tmp_file) {
            // Getting a new instance is not ideal, but we need access to the client
            // In a real implementation, we'd use a better design pattern
            let sftp = helper::get_storage_instance::<SFTP>("sftp")?;
            sftp.upload_file(&Path::new(tmp_file), &abs_path)?;
            fs::remove_file(tmp_file)?;
        }
        
        Ok(())
    }
}

#[async_trait]
impl Storage for SFTP {
    fn get_id(&self) -> String {
        format!("sftp::{}@{}/{}", self.user, self.host, self.root)
    }
    
    async fn test(&self) -> bool {
        if self.host.is_empty() || self.user.is_empty() || self.password.is_empty() {
            return false;
        }
        
        match self.get_client() {
            Ok(client) => {
                match client.readdir(&Path::new("/")) {
                    Ok(_) => true,
                    Err(_) => false,
                }
            },
            Err(_) => false,
        }
    }
    
    async fn mkdir(&self, path: &str) -> StorageResult<()> {
        let client = self.get_client()?;
        let abs_path = self.abs_path(path);
        client.mkdir(&Path::new(&abs_path), 0o755)?;
        Ok(())
    }
    
    async fn rmdir(&self, path: &str) -> StorageResult<()> {
        let client = self.get_client()?;
        let abs_path = self.abs_path(path);
        client.rmdir(&Path::new(&abs_path))?;
        Ok(())
    }
    
    async fn opendir(&self, path: &str) -> StorageResult<Box<dyn DirStream>> {
        let client = self.get_client()?;
        let abs_path = self.abs_path(path);
        
        let entries = client.readdir(&Path::new(&abs_path))?;
        
        let files: Vec<String> = entries.iter()
            .filter_map(|(path, _)| {
                path.file_name().map(|s| s.to_string_lossy().to_string())
            })
            .filter(|name| name != "." && name != "..")
            .collect();
            
        Ok(Box::new(helper::create_dir_stream(files)))
    }
    
    async fn filetype(&self, path: &str) -> StorageResult<String> {
        let client = self.get_client()?;
        let abs_path = self.abs_path(path);
        
        let stat = client.stat(&Path::new(&abs_path))?;
        
        if stat.is_file() {
            Ok("file".to_string())
        } else if stat.is_dir() {
            Ok("dir".to_string())
        } else {
            Err("Unknown file type".into())
        }
    }
    
    async fn is_readable(&self, _path: &str) -> bool {
        true
    }
    
    async fn is_updatable(&self, _path: &str) -> bool {
        true
    }
    
    async fn file_exists(&self, path: &str) -> bool {
        match self.get_client() {
            Ok(client) => {
                let abs_path = self.abs_path(path);
                match client.stat(&Path::new(&abs_path)) {
                    Ok(_) => true,
                    Err(_) => false,
                }
            },
            Err(_) => false,
        }
    }
    
    async fn unlink(&self, path: &str) -> StorageResult<()> {
        let client = self.get_client()?;
        let abs_path = self.abs_path(path);
        client.unlink(&Path::new(&abs_path))?;
        Ok(())
    }
    
    async fn fopen(&self, path: &str, mode: &str) -> StorageResult<Box<dyn Read + Send>> {
        let abs_path = self.abs_path(path);
        
        match mode {
            "r" | "rb" => {
                if !self.file_exists(path).await {
                    return Err("File does not exist".into());
                }
                
                let ext = Path::new(path).extension()
                    .map(|e| format!(".{}", e.to_string_lossy()))
                    .unwrap_or_default();
                
                let mut temp_file = tempfile::Builder::new()
                    .suffix(&ext)
                    .tempfile()?;
                
                self.get_file(&abs_path, temp_file.path())?;
                
                Ok(Box::new(temp_file))
            },
            "w" | "wb" | "a" | "ab" | "r+" | "w+" | "wb+" | "a+" | "x" | "x+" | "c" | "c+" => {
                let ext = Path::new(path).extension()
                    .map(|e| format!(".{}", e.to_string_lossy()))
                    .unwrap_or_default();
                
                let temp_file = tempfile::Builder::new()
                    .suffix(&ext)
                    .tempfile()?;
                
                let temp_path = temp_file.path().to_string_lossy().to_string();
                
                if self.file_exists(path).await {
                    self.get_file(&abs_path, temp_file.path())?;
                }
                
                // Register callback for close
                CloseCallback::register(&temp_path, Box::new(Self::write_back))?;
                
                // Store mapping
                let mut temp_files = TEMP_FILES.lock().unwrap();
                temp_files.insert(temp_path.clone(), abs_path);
                
                Ok(Box::new(File::open(temp_path)?))
            },
            _ => Err(format!("Unsupported mode: {}", mode).into()),
        }
    }
    
    async fn touch(&self, path: &str, mtime: Option<i64>) -> StorageResult<()> {
        if mtime.is_some() {
            return Err("Setting mtime is not supported".into());
        }
        
        if !self.file_exists(path).await {
            let client = self.get_client()?;
            let abs_path = self.abs_path(path);
            
            let mut remote_file = client.create(&Path::new(&abs_path))?;
            remote_file.write_all(b"")?;
            
            Ok(())
        } else {
            Err("File already exists".into())
        }
    }
    
    async fn rename(&self, source: &str, target: &str) -> StorageResult<()> {
        let client = self.get_client()?;
        let abs_source = self.abs_path(source);
        let abs_target = self.abs_path(target);
        
        client.rename(
            &Path::new(&abs_source),
            &Path::new(&abs_target),
            None
        )?;
        
        Ok(())
    }
    
    async fn stat(&self, path: &str) -> StorageResult<FileInfo> {
        let client = self.get_client()?;
        let abs_path = self.abs_path(path);
        
        let stat = client.stat(&Path::new(&abs_path))?;
        
        let mtime = stat.mtime.unwrap_or(0);
        let size = stat.size.unwrap_or(0);
        
        Ok(FileInfo {
            mtime,
            size,
            ctime: -1,
        })
    }
}