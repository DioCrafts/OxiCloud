use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::fs::{self, File as FsFile, OpenOptions, DirBuilder};

use log::{error};

use crate::auth::user;
use crate::files::{Filesystem, FileView};
use crate::log;
use crate::file_proxy::FileProxy;

/// File-based cache implementation
pub struct File {
    storage: Option<FileView>,
}

impl File {
    pub fn new() -> Self {
        File {
            storage: None,
        }
    }

    fn get_storage(&mut self) -> Option<&FileView> {
        if self.storage.is_some() {
            return self.storage.as_ref();
        }

        if user::is_logged_in() {
            let user = match user::get_user() {
                Some(user) => user,
                None => {
                    error!("User logged in but can't get username");
                    return None;
                }
            };

            Filesystem::init_mount_points(&user);
            let subdir = "cache";
            let user_view = FileView::new(&format!("/{}", user));
            
            if !user_view.file_exists(subdir) {
                if let Err(e) = user_view.mkdir(subdir) {
                    error!("Failed to create cache directory: {}", e);
                    return None;
                }
            }
            
            self.storage = Some(FileView::new(&format!("/{}/{}", user, subdir)));
            self.storage.as_ref()
        } else {
            error!("Can't get cache storage, user not logged in");
            None
        }
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
        let proxy_status = FileProxy::is_enabled();
        FileProxy::set_enabled(false);
        
        let result = if self.has_key(key) {
            if let Some(storage) = self.get_storage() {
                storage.file_get_contents(key).ok()
            } else {
                None
            }
        } else {
            None
        };
        
        FileProxy::set_enabled(proxy_status);
        result
    }

    /// Returns the size of the stored/cached data
    ///
    /// # Arguments
    ///
    /// * `key` - Cache key to get size for
    pub fn size(&mut self, key: &str) -> u64 {
        let proxy_status = FileProxy::is_enabled();
        FileProxy::set_enabled(false);
        
        let result = if self.has_key(key) {
            if let Some(storage) = self.get_storage() {
                storage.filesize(key).unwrap_or(0)
            } else {
                0
            }
        } else {
            0
        };
        
        FileProxy::set_enabled(proxy_status);
        result
    }

    pub fn set(&mut self, key: &str, value: &str, ttl: u64) -> bool {
        let storage = match self.get_storage() {
            Some(s) => s,
            None => return false,
        };
        
        let proxy_status = FileProxy::is_enabled();
        FileProxy::set_enabled(false);
        
        let result = if storage.file_put_contents(key, value).is_ok() {
            let ttl_sec = if ttl == 0 { 86400 } else { ttl }; // 60*60*24
            match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(now) => {
                    let new_time = now.as_secs() + ttl_sec;
                    storage.touch(key, new_time).is_ok()
                },
                Err(_) => false,
            }
        } else {
            false
        };
        
        FileProxy::set_enabled(proxy_status);
        result
    }

    pub fn has_key(&mut self, key: &str) -> bool {
        let storage = match self.get_storage() {
            Some(s) => s,
            None => return false,
        };
        
        if storage.is_file(key) {
            match storage.filemtime(key) {
                Ok(mtime) => {
                    match SystemTime::now().duration_since(UNIX_EPOCH) {
                        Ok(now) => {
                            if mtime < now.as_secs() {
                                let _ = storage.unlink(key);
                                false
                            } else {
                                true
                            }
                        },
                        Err(_) => false,
                    }
                },
                Err(_) => false,
            }
        } else {
            false
        }
    }

    pub fn remove(&mut self, key: &str) -> bool {
        match self.get_storage() {
            Some(storage) => storage.unlink(key).is_ok(),
            None => false,
        }
    }

    pub fn clear(&mut self, prefix: &str) -> bool {
        let storage = match self.get_storage() {
            Some(s) => s,
            None => return false,
        };
        
        if storage.is_dir("/") {
            if let Ok(entries) = storage.read_dir("/") {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let filename = entry.file_name().to_string_lossy().to_string();
                        if filename != "." && filename != ".." && 
                           (prefix.is_empty() || filename.starts_with(prefix)) {
                            let _ = storage.unlink(&format!("/{}", filename));
                        }
                    }
                }
            }
        }
        
        true
    }

    pub fn gc(&mut self) {
        let storage = match self.get_storage() {
            Some(s) => s,
            None => return,
        };
        
        if !storage.is_dir("/") {
            return;
        }
        
        let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => return,
        };
        
        if let Ok(entries) = storage.read_dir("/") {
            for entry in entries {
                if let Ok(entry) = entry {
                    let filename = entry.file_name().to_string_lossy().to_string();
                    if filename != "." && filename != ".." {
                        let path = format!("/{}", filename);
                        if let Ok(mtime) = storage.filemtime(&path) {
                            if mtime < now {
                                let _ = storage.unlink(&path);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn login_listener() {
        let mut cache = Self::new();
        cache.gc();
    }
}