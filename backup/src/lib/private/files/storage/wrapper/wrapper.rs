// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::collections::HashMap;
use std::io::{Read, Write, Seek};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;

use crate::files::cache::{Cache, Scanner, Permissions, Watcher, Storage as StorageCache};

/// Storage trait representing basic file operations
#[async_trait]
pub trait Storage: Send + Sync {
    fn get_id(&self) -> String;
    
    async fn mkdir(&self, path: &str) -> Result<bool, StorageError>;
    async fn rmdir(&self, path: &str) -> Result<bool, StorageError>;
    async fn opendir(&self, path: &str) -> Result<Box<dyn Iterator<Item = String> + Send>, StorageError>;
    async fn is_dir(&self, path: &str) -> Result<bool, StorageError>;
    async fn is_file(&self, path: &str) -> Result<bool, StorageError>;
    async fn stat(&self, path: &str) -> Result<FileStat, StorageError>;
    async fn filetype(&self, path: &str) -> Result<String, StorageError>;
    async fn filesize(&self, path: &str) -> Result<u64, StorageError>;
    
    async fn is_creatable(&self, path: &str) -> Result<bool, StorageError>;
    async fn is_readable(&self, path: &str) -> Result<bool, StorageError>;
    async fn is_updatable(&self, path: &str) -> Result<bool, StorageError>;
    async fn is_deletable(&self, path: &str) -> Result<bool, StorageError>;
    async fn is_sharable(&self, path: &str) -> Result<bool, StorageError>;
    
    async fn get_permissions(&self, path: &str) -> Result<i32, StorageError>;
    async fn file_exists(&self, path: &str) -> Result<bool, StorageError>;
    async fn filemtime(&self, path: &str) -> Result<u64, StorageError>;
    
    async fn file_get_contents(&self, path: &str) -> Result<Vec<u8>, StorageError>;
    async fn file_put_contents(&self, path: &str, data: &[u8]) -> Result<bool, StorageError>;
    
    async fn unlink(&self, path: &str) -> Result<bool, StorageError>;
    async fn rename(&self, path1: &str, path2: &str) -> Result<bool, StorageError>;
    async fn copy(&self, path1: &str, path2: &str) -> Result<bool, StorageError>;
    
    async fn fopen(&self, path: &str, mode: &str) -> Result<Box<dyn FileHandle>, StorageError>;
    
    async fn get_mime_type(&self, path: &str) -> Result<String, StorageError>;
    async fn hash(&self, hash_type: &str, path: &str, raw: bool) -> Result<String, StorageError>;
    async fn free_space(&self, path: &str) -> Result<u64, StorageError>;
    
    async fn search(&self, query: &str) -> Result<Vec<String>, StorageError>;
    async fn touch(&self, path: &str, mtime: Option<u64>) -> Result<bool, StorageError>;
    
    async fn get_local_file(&self, path: &str) -> Result<String, StorageError>;
    async fn get_local_folder(&self, path: &str) -> Result<String, StorageError>;
    
    async fn has_updated(&self, path: &str, time: u64) -> Result<bool, StorageError>;
    
    fn get_cache(&self, path: &str) -> Box<dyn Cache>;
    fn get_scanner(&self, path: &str) -> Box<dyn Scanner>;
    async fn get_owner(&self, path: &str) -> Result<String, StorageError>;
    fn get_permissions_cache(&self, path: &str) -> Box<dyn Permissions>;
    fn get_watcher(&self, path: &str) -> Box<dyn Watcher>;
    fn get_storage_cache(&self) -> Box<dyn StorageCache>;
    async fn get_etag(&self, path: &str) -> Result<String, StorageError>;
}

pub trait FileHandle: Read + Write + Seek + Send + Sync {}

#[derive(Debug, Clone)]
pub struct FileStat {
    pub size: u64,
    pub mtime: u64,
    // Other stat fields can be added as needed
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Path not found: {0}")]
    NotFound(String),
    
    #[error("Access denied: {0}")]
    AccessDenied(String),
    
    #[error("Not supported")]
    NotSupported,
    
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    
    #[error("Other error: {0}")]
    Other(String),
}

/// A wrapper storage implementation that delegates all operations to another storage
pub struct Wrapper {
    storage: Box<dyn Storage>,
}

impl Wrapper {
    pub fn new(parameters: HashMap<String, Box<dyn Storage>>) -> Self {
        let storage = parameters.get("storage")
            .expect("Storage parameter is required")
            .clone();
        
        Self { storage }
    }
    
    pub fn get_wrapper_storage(&self) -> &dyn Storage {
        self.storage.as_ref()
    }
}

#[async_trait]
impl Storage for Wrapper {
    fn get_id(&self) -> String {
        self.storage.get_id()
    }
    
    async fn mkdir(&self, path: &str) -> Result<bool, StorageError> {
        self.storage.mkdir(path).await
    }
    
    async fn rmdir(&self, path: &str) -> Result<bool, StorageError> {
        self.storage.rmdir(path).await
    }
    
    async fn opendir(&self, path: &str) -> Result<Box<dyn Iterator<Item = String> + Send>, StorageError> {
        self.storage.opendir(path).await
    }
    
    async fn is_dir(&self, path: &str) -> Result<bool, StorageError> {
        self.storage.is_dir(path).await
    }
    
    async fn is_file(&self, path: &str) -> Result<bool, StorageError> {
        self.storage.is_file(path).await
    }
    
    async fn stat(&self, path: &str) -> Result<FileStat, StorageError> {
        self.storage.stat(path).await
    }
    
    async fn filetype(&self, path: &str) -> Result<String, StorageError> {
        self.storage.filetype(path).await
    }
    
    async fn filesize(&self, path: &str) -> Result<u64, StorageError> {
        self.storage.filesize(path).await
    }
    
    async fn is_creatable(&self, path: &str) -> Result<bool, StorageError> {
        self.storage.is_creatable(path).await
    }
    
    async fn is_readable(&self, path: &str) -> Result<bool, StorageError> {
        self.storage.is_readable(path).await
    }
    
    async fn is_updatable(&self, path: &str) -> Result<bool, StorageError> {
        self.storage.is_updatable(path).await
    }
    
    async fn is_deletable(&self, path: &str) -> Result<bool, StorageError> {
        self.storage.is_deletable(path).await
    }
    
    async fn is_sharable(&self, path: &str) -> Result<bool, StorageError> {
        self.storage.is_sharable(path).await
    }
    
    async fn get_permissions(&self, path: &str) -> Result<i32, StorageError> {
        self.storage.get_permissions(path).await
    }
    
    async fn file_exists(&self, path: &str) -> Result<bool, StorageError> {
        self.storage.file_exists(path).await
    }
    
    async fn filemtime(&self, path: &str) -> Result<u64, StorageError> {
        self.storage.filemtime(path).await
    }
    
    async fn file_get_contents(&self, path: &str) -> Result<Vec<u8>, StorageError> {
        self.storage.file_get_contents(path).await
    }
    
    async fn file_put_contents(&self, path: &str, data: &[u8]) -> Result<bool, StorageError> {
        self.storage.file_put_contents(path, data).await
    }
    
    async fn unlink(&self, path: &str) -> Result<bool, StorageError> {
        self.storage.unlink(path).await
    }
    
    async fn rename(&self, path1: &str, path2: &str) -> Result<bool, StorageError> {
        self.storage.rename(path1, path2).await
    }
    
    async fn copy(&self, path1: &str, path2: &str) -> Result<bool, StorageError> {
        self.storage.copy(path1, path2).await
    }
    
    async fn fopen(&self, path: &str, mode: &str) -> Result<Box<dyn FileHandle>, StorageError> {
        self.storage.fopen(path, mode).await
    }
    
    async fn get_mime_type(&self, path: &str) -> Result<String, StorageError> {
        self.storage.get_mime_type(path).await
    }
    
    async fn hash(&self, hash_type: &str, path: &str, raw: bool) -> Result<String, StorageError> {
        self.storage.hash(hash_type, path, raw).await
    }
    
    async fn free_space(&self, path: &str) -> Result<u64, StorageError> {
        self.storage.free_space(path).await
    }
    
    async fn search(&self, query: &str) -> Result<Vec<String>, StorageError> {
        self.storage.search(query).await
    }
    
    async fn touch(&self, path: &str, mtime: Option<u64>) -> Result<bool, StorageError> {
        self.storage.touch(path, mtime).await
    }
    
    async fn get_local_file(&self, path: &str) -> Result<String, StorageError> {
        self.storage.get_local_file(path).await
    }
    
    async fn get_local_folder(&self, path: &str) -> Result<String, StorageError> {
        self.storage.get_local_folder(path).await
    }
    
    async fn has_updated(&self, path: &str, time: u64) -> Result<bool, StorageError> {
        self.storage.has_updated(path, time).await
    }
    
    fn get_cache(&self, path: &str) -> Box<dyn Cache> {
        self.storage.get_cache(path)
    }
    
    fn get_scanner(&self, path: &str) -> Box<dyn Scanner> {
        self.storage.get_scanner(path)
    }
    
    async fn get_owner(&self, path: &str) -> Result<String, StorageError> {
        self.storage.get_owner(path).await
    }
    
    fn get_permissions_cache(&self, path: &str) -> Box<dyn Permissions> {
        self.storage.get_permissions_cache(path)
    }
    
    fn get_watcher(&self, path: &str) -> Box<dyn Watcher> {
        self.storage.get_watcher(path)
    }
    
    fn get_storage_cache(&self) -> Box<dyn StorageCache> {
        self.storage.get_storage_cache()
    }
    
    async fn get_etag(&self, path: &str) -> Result<String, StorageError> {
        self.storage.get_etag(path).await
    }
}