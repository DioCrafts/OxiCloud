use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_trait::async_trait;
use log::{debug, log_enabled, Level};
use sea_orm::{DatabaseTransaction, TransactionTrait};

use crate::db::DbConnection;
use crate::files::cache::{Cache, Permissions};
use crate::files::filesystem::Filesystem;
use crate::files::storage::Storage;
use crate::hooks::{BasicEmitter, Emitter, Hook};
use crate::util;

/**
 * Class Scanner
 *
 * Hooks available in scope \OC\Files\Cache\Scanner:
 *  - scan_file(string $path, string $storage_id)
 *  - scan_folder(string $path, string $storage_id)
 *  - post_scan_file(string $path, string $storage_id)
 *  - post_scan_folder(string $path, string $storage_id)
 */
pub struct Scanner {
    storage: Arc<dyn Storage>,
    storage_id: String,
    cache: Arc<dyn Cache>,
    permissions_cache: Arc<dyn Permissions>,
    emitter: BasicEmitter,
}

pub const SCAN_RECURSIVE: bool = true;
pub const SCAN_SHALLOW: bool = false;

pub const REUSE_ETAG: u8 = 1;
pub const REUSE_SIZE: u8 = 2;

impl Scanner {
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        let storage_id = storage.get_id();
        let cache = storage.get_cache();
        let permissions_cache = storage.get_permissions_cache();
        
        Self {
            storage,
            storage_id,
            cache,
            permissions_cache,
            emitter: BasicEmitter::new(),
        }
    }

    /**
     * get all the metadata of a file or folder
     *
     * @param path
     * @return metadata of the file
     */
    pub async fn get_data(&self, path: &str) -> Option<FileMetadata> {
        if !self.storage.is_readable(path).await {
            // can't read, nothing we can do
            if log_enabled!(Level::Debug) {
                debug!("!!! Path '{}' is not readable !!!", path);
            }
            return None;
        }

        let mimetype = self.storage.get_mime_type(path).await;
        let mtime = self.storage.filemtime(path).await;
        
        let size = if mimetype == "httpd/unix-directory" {
            -1 // unknown
        } else {
            self.storage.filesize(path).await
        };
        
        let etag = self.storage.get_etag(path).await;
        let storage_mtime = mtime;
        
        Some(FileMetadata {
            mimetype,
            mtime,
            size,
            etag,
            storage_mtime,
        })
    }

    /**
     * scan a single file and store it in the cache
     *
     * @param file
     * @param reuse_existing
     * @param parent_exists_in_cache
     * @return metadata of the scanned file
     */
    pub async fn scan_file(&self, file: &str, reuse_existing: u8, parent_exists_in_cache: bool) -> Option<FileMetadata> {
        if !Self::is_partial_file(file) && !Filesystem::is_file_blacklisted(file) {
            self.emitter.emit("\\OC\\Files\\Cache\\Scanner", "scan_file", &[file.into(), self.storage_id.clone().into()]);
            Hook::emit("\\OC\\Files\\Cache\\Scanner", "scan_file", 
                &[("path".into(), file.into()), ("storage".into(), self.storage_id.clone().into())]);

            if let Some(data) = self.get_data(file).await {
                if !file.is_empty() && !parent_exists_in_cache {
                    let parent = if let Some(p) = Path::new(file).parent() {
                        p.to_string_lossy().to_string()
                    } else {
                        String::new()
                    };
                    
                    let parent = if parent == "." || parent == "/" {
                        String::new()
                    } else {
                        parent
                    };
                    
                    if !self.cache.in_cache(&parent).await {
                        self.scan_file(&parent, 0, false).await;
                    }
                }
                
                let mut new_data = data.clone();
                let cache_data = self.cache.get(file).await;
                
                if let Some(cache_data) = cache_data {
                    if let Some(fileid) = cache_data.get("fileid") {
                        if let Some(fileid) = fileid.as_i64() {
                            self.permissions_cache.remove(fileid).await;
                        }
                    }
                    
                    if reuse_existing > 0 {
                        // Prevent empty etag
                        let mut etag = cache_data.get("etag").and_then(|e| e.as_str().map(String::from))
                            .unwrap_or_default();
                        let mut propagate_etag_change = false;
                        
                        if etag.is_empty() {
                            etag = data.etag.clone();
                            propagate_etag_change = true;
                        }
                        
                        // Only reuse data if the file hasn't explicitly changed
                        if let (Some(data_mtime), Some(cache_mtime)) = (cache_data.get("mtime").and_then(|m| m.as_i64()), 
                                                                        Some(data.mtime)) {
                            if data_mtime == cache_mtime {
                                if (reuse_existing & REUSE_SIZE > 0) && data.size == -1 {
                                    if let Some(size) = cache_data.get("size").and_then(|s| s.as_i64()) {
                                        new_data.size = size;
                                    }
                                }
                                
                                if reuse_existing & REUSE_ETAG > 0 {
                                    new_data.etag = etag.clone();
                                    
                                    if propagate_etag_change {
                                        let mut parent = file.to_string();
                                        while !parent.is_empty() {
                                            parent = if let Some(p) = Path::new(&parent).parent() {
                                                p.to_string_lossy().to_string()
                                            } else {
                                                String::new()
                                            };
                                            
                                            if parent == "." {
                                                parent = String::new();
                                            }
                                            
                                            if let Some(parent_cache_data) = self.cache.get(&parent).await {
                                                if let Some(parent_fileid) = parent_cache_data.get("fileid").and_then(|id| id.as_i64()) {
                                                    let parent_etag = self.storage.get_etag(&parent).await;
                                                    self.cache.update(parent_fileid, &[("etag", parent_etag.into())]).await;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Only update metadata that has changed
                        // In Rust we'll build new_data differently
                        if new_data.etag != data.etag {
                            if log_enabled!(Level::Debug) {
                                debug!("!!! No reuse of etag for '{}' !!! \ncache: {:?} \ndata: {:?}",
                                      file, cache_data, data);
                            }
                        }
                    }
                }
                
                // Check if new_data has any differences from cache_data
                let has_changes = if let Some(cache_data) = &cache_data {
                    // Compare fields to determine if there are changes
                    cache_data.get("mimetype").and_then(|m| m.as_str()) != Some(&new_data.mimetype) ||
                    cache_data.get("size").and_then(|s| s.as_i64()) != Some(new_data.size) ||
                    cache_data.get("mtime").and_then(|m| m.as_i64()) != Some(new_data.mtime) ||
                    cache_data.get("etag").and_then(|e| e.as_str()) != Some(&new_data.etag) ||
                    cache_data.get("storage_mtime").and_then(|m| m.as_i64()) != Some(new_data.storage_mtime)
                } else {
                    true // No cache data means there are changes
                };
                
                if has_changes {
                    let data_map = new_data.to_map();
                    self.cache.put(file, &data_map).await;
                    self.emitter.emit("\\OC\\Files\\Cache\\Scanner", "post_scan_file", &[file.into(), self.storage_id.clone().into()]);
                    Hook::emit("\\OC\\Files\\Cache\\Scanner", "post_scan_file", 
                        &[("path".into(), file.into()), ("storage".into(), self.storage_id.clone().into())]);
                }
                
                return Some(data);
            } else {
                self.cache.remove(file).await;
            }
        }
        
        None
    }

    /**
     * scan a folder and all it's children
     *
     * @param path
     * @param recursive
     * @param reuse
     * @return the size of the scanned folder or -1 if the size is unknown at this stage
     */
    pub async fn scan(&self, path: &str, recursive: bool, reuse: i8) -> i64 {
        let reuse = if reuse == -1 {
            if recursive == SCAN_SHALLOW { 
                REUSE_ETAG | REUSE_SIZE 
            } else { 
                0 
            }
        } else {
            reuse as u8
        };
        
        self.scan_file(path, reuse, false).await;
        self.scan_children(path, recursive, reuse).await
    }

    /**
     * scan all the files and folders in a folder
     *
     * @param path
     * @param recursive
     * @param reuse
     * @return the size of the scanned folder or -1 if the size is unknown at this stage
     */
    pub async fn scan_children(&self, path: &str, recursive: bool, reuse: u8) -> i64 {
        self.emitter.emit("\\OC\\Files\\Cache\\Scanner", "scan_folder", &[path.into(), self.storage_id.clone().into()]);
        
        let mut size: i64 = 0;
        let mut child_queue = Vec::new();
        let mut existing_children = HashSet::new();
        
        if self.cache.in_cache(path).await {
            let children = self.cache.get_folder_contents(path).await;
            for child in children {
                if let Some(name) = child.get("name").and_then(|n| n.as_str()) {
                    existing_children.insert(name.to_string());
                }
            }
        }
        
        let mut new_children = HashSet::new();
        if self.storage.is_dir(path).await {
            let files = self.storage.opendir(path).await;
            let mut exception_occurred = false;
            
            let db_conn = DbConnection::get_connection().await;
            let tx: DatabaseTransaction = db_conn.begin().await.unwrap();
            
            if let Some(files) = files {
                for file in files {
                    let child = if path.is_empty() { 
                        file.clone() 
                    } else { 
                        format!("{}/{}", path, file) 
                    };
                    
                    if !Filesystem::is_ignored_dir(&file) {
                        new_children.insert(file.clone());
                        
                        match self.scan_file(&child, reuse, true).await {
                            Some(data) => {
                                if data.size == -1 {
                                    if recursive == SCAN_RECURSIVE {
                                        child_queue.push(child);
                                    } else {
                                        size = -1;
                                    }
                                } else if size != -1 {
                                    size += data.size;
                                }
                            }
                            None => {
                                // Handle scan error
                            }
                        }
                    }
                }
            }
            
            let removed_children: Vec<_> = existing_children
                .difference(&new_children)
                .collect();
                
            for child_name in removed_children {
                let child = if path.is_empty() { 
                    child_name.clone() 
                } else { 
                    format!("{}/{}", path, child_name) 
                };
                self.cache.remove(&child).await;
            }
            
            tx.commit().await.unwrap();
            
            if exception_occurred {
                // It might happen that the parallel scan process has already
                // inserted mimetypes but those weren't available yet inside the transaction
                // To make sure to have the updated mime types in such cases,
                // we reload them here
                self.cache.load_mimetypes().await;
            }
            
            for child in child_queue {
                let child_size = self.scan_children(&child, SCAN_RECURSIVE, reuse).await;
                if child_size == -1 {
                    size = -1;
                } else {
                    size += child_size;
                }
            }
            
            self.cache.put(path, &[("size", size.into())]).await;
        }
        
        self.emitter.emit("\\OC\\Files\\Cache\\Scanner", "post_scan_folder", &[path.into(), self.storage_id.clone().into()]);
        size
    }

    /**
     * check if the file should be ignored when scanning
     * NOTE: files with a '.part' extension are ignored as well!
     *       prevents unfinished put requests to be scanned
     */
    pub fn is_partial_file(file: &str) -> bool {
        Path::new(file)
            .extension()
            .and_then(|ext| ext.to_str())
            .map_or(false, |ext| ext == "part")
    }

    /**
     * walk over any folders that are not fully scanned yet and scan them
     */
    pub async fn background_scan(&self) {
        let mut last_path = None;
        
        while let Some(path) = self.cache.get_incomplete().await {
            if last_path.as_ref() == Some(&path) {
                break;
            }
            
            self.scan(&path, SCAN_RECURSIVE, -1).await;
            self.cache.correct_folder_size(&path).await;
            last_path = Some(path);
        }
    }
}

impl Emitter for Scanner {
    fn emit(&self, scope: &str, event: &str, args: &[serde_json::Value]) {
        self.emitter.emit(scope, event, args);
    }
    
    fn add_listener(&mut self, scope: &str, event: &str, callback: Box<dyn Fn(&[serde_json::Value]) + Send + Sync>) {
        self.emitter.add_listener(scope, event, callback);
    }
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub mimetype: String,
    pub mtime: i64,
    pub size: i64,
    pub etag: String,
    pub storage_mtime: i64,
}

impl FileMetadata {
    pub fn to_map(&self) -> Vec<(&str, serde_json::Value)> {
        vec![
            ("mimetype", self.mimetype.clone().into()),
            ("mtime", self.mtime.into()),
            ("size", self.size.into()),
            ("etag", self.etag.clone().into()),
            ("storage_mtime", self.storage_mtime.into()),
        ]
    }
}