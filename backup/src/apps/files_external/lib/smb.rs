// Copyright (c) 2012 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::io;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use async_trait::async_trait;
use url::percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use std::fs::ReadDir;

use crate::files::storage::stream_wrapper::StreamWrapper;
use crate::files::storage::StorageError;
use crate::files::storage::Result;

pub struct Smb {
    password: String,
    user: String,
    host: String,
    root: String,
    share: String,
}

impl Smb {
    pub fn new(params: &std::collections::HashMap<String, String>) -> Result<Self> {
        let host = params.get("host").ok_or(StorageError::InvalidParameters)?;
        let user = params.get("user").ok_or(StorageError::InvalidParameters)?;
        let password = params.get("password").ok_or(StorageError::InvalidParameters)?;
        let share = params.get("share").ok_or(StorageError::InvalidParameters)?;
        
        let mut root = params.get("root").map(|s| s.to_string()).unwrap_or_else(|| "/".to_string());
        
        if root.is_empty() || !root.starts_with('/') {
            root = format!("/{}", root);
        }
        
        if !root.ends_with('/') {
            root.push('/');
        }
        
        let mut share = share.to_string();
        if share.is_empty() || !share.starts_with('/') {
            share = format!("/{}", share);
        }
        
        if share.ends_with('/') {
            share.pop();
        }
        
        Ok(Self {
            host: host.to_string(),
            user: user.to_string(),
            password: password.to_string(),
            share,
            root,
        })
    }
    
    pub fn get_id(&self) -> String {
        format!("smb::{}@{}{}{}", self.user, self.host, self.share, self.root)
    }
    
    fn construct_url(&self, path: &str) -> String {
        let mut path = path.to_string();
        if path.ends_with('/') {
            path.pop();
        }
        
        let encoded_path = percent_encode(path.as_bytes(), NON_ALPHANUMERIC).to_string();
        let encoded_user = percent_encode(self.user.as_bytes(), NON_ALPHANUMERIC).to_string();
        let encoded_pass = percent_encode(self.password.as_bytes(), NON_ALPHANUMERIC).to_string();
        
        format!("smb://{}:{}@{}{}{}{}", encoded_user, encoded_pass, self.host, self.share, self.root, encoded_path)
    }
    
    async fn stat(&self, path: &str) -> Result<std::fs::Metadata> {
        if path.is_empty() && self.root == "/" {
            // mtime doesn't work for shares
            let url = self.construct_url(path);
            let stat = self.stat_file(&url).await?;
            
            if let Ok(mtime) = self.share_mtime().await {
                // Note: In Rust we can't modify the metadata directly,
                // so we would need to create a custom struct to return
                // that wraps the metadata and provides the modified mtime.
                // This is a simplified implementation.
                return Ok(stat);
            }
            
            Ok(stat)
        } else {
            let url = self.construct_url(path);
            self.stat_file(&url).await
        }
    }
    
    async fn stat_file(&self, url: &str) -> Result<std::fs::Metadata> {
        // This would need to use an actual SMB library implementation
        // For example, using the Samba crate or similar
        // This is a placeholder that would need to be implemented
        Err(StorageError::NotImplemented)
    }
    
    /// check if a file or folder has been updated since $time
    async fn has_updated(&self, path: &str, time: u64) -> Result<bool> {
        if path.is_empty() && self.root == "/" {
            // mtime doesn't work for shares, but giving the nature of the backend,
            // doing a full update is still just fast enough
            Ok(true)
        } else {
            let actual_time = self.filemtime(path).await?;
            Ok(actual_time > time)
        }
    }
    
    async fn filemtime(&self, path: &str) -> Result<u64> {
        let metadata = self.stat(path).await?;
        let modified = metadata.modified()
            .map_err(|_| StorageError::IOError(io::Error::new(io::ErrorKind::Other, "Could not get mtime")))?;
            
        let duration = modified.duration_since(UNIX_EPOCH)
            .map_err(|_| StorageError::IOError(io::Error::new(io::ErrorKind::Other, "Time went backwards")))?;
            
        Ok(duration.as_secs())
    }
    
    /// get the best guess for the modification time of the share
    async fn share_mtime(&self) -> Result<u64> {
        let mut dh = self.opendir("").await?;
        let mut last_ctime = 0;
        
        while let Some(entry) = self.readdir(&mut dh).await? {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name != "." && file_name != ".." {
                if let Ok(ctime) = self.filemtime(&file_name).await {
                    if ctime > last_ctime {
                        last_ctime = ctime;
                    }
                }
            }
        }
        
        Ok(last_ctime)
    }
    
    async fn opendir(&self, path: &str) -> Result<ReadDir> {
        // This would need actual SMB implementation
        Err(StorageError::NotImplemented)
    }
    
    async fn readdir(&self, dh: &mut ReadDir) -> Result<Option<std::fs::DirEntry>> {
        // This would need actual SMB implementation
        Err(StorageError::NotImplemented)
    }
}

#[async_trait]
impl StreamWrapper for Smb {
    async fn open_file(&self, path: &str, mode: &str) -> Result<Box<dyn io::Read + Send + Sync>> {
        // Implementation would need actual SMB library
        Err(StorageError::NotImplemented)
    }
    
    // Other StreamWrapper trait methods would be implemented here
}