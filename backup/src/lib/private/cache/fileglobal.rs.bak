// FileGlobal cache implementation
// 
// Copyright (c) 2012 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::appconfig;
use crate::util;

pub struct FileGlobal;

impl FileGlobal {
    fn get_cache_dir() -> io::Result<PathBuf> {
        let temp_dir = std::env::temp_dir();
        let instance_id = util::get_instance_id();
        let cache_dir = temp_dir.join(format!("owncloud-{}", instance_id));
        
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)?;
        }
        
        Ok(cache_dir)
    }

    fn fix_key(&self, key: &str) -> String {
        key.replace('/', "_")
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let key = self.fix_key(key);
        
        if self.has_key(&key) {
            if let Ok(cache_dir) = Self::get_cache_dir() {
                let path = cache_dir.join(&key);
                return fs::read_to_string(path).ok();
            }
        }
        
        None
    }

    pub fn set(&self, key: &str, value: &str, ttl: u64) -> bool {
        let key = self.fix_key(key);
        let ttl = if ttl == 0 { 86400 } else { ttl }; // 60*60*24
        
        if let Ok(cache_dir) = Self::get_cache_dir() {
            let path = cache_dir.join(&key);
            
            if let Ok(mut file) = File::create(&path) {
                if file.write_all(value.as_bytes()).is_ok() {
                    // Set the modification time to now + ttl
                    if let Ok(now) = SystemTime::now().duration_since(UNIX_EPOCH) {
                        let new_time = now + Duration::from_secs(ttl);
                        let filetime = filetime::FileTime::from_unix_time(
                            new_time.as_secs() as i64, 
                            new_time.subsec_nanos()
                        );
                        
                        return filetime::set_file_mtime(&path, filetime).is_ok();
                    }
                }
            }
        }
        
        false
    }

    pub fn has_key(&self, key: &str) -> bool {
        let key = self.fix_key(key);
        
        if let Ok(cache_dir) = Self::get_cache_dir() {
            let path = cache_dir.join(&key);
            
            if path.is_file() {
                if let Ok(metadata) = fs::metadata(&path) {
                    if let Ok(mtime) = metadata.modified() {
                        if let Ok(now) = SystemTime::now().duration_since(UNIX_EPOCH) {
                            if let Ok(file_time) = mtime.duration_since(UNIX_EPOCH) {
                                if file_time.as_secs() < now.as_secs() {
                                    let _ = fs::remove_file(&path);
                                    return false;
                                }
                                return true;
                            }
                        }
                    }
                }
                
                // If we can't read metadata or compare times, try to remove the file
                let _ = fs::remove_file(&path);
            }
        }
        
        false
    }

    pub fn remove(&self, key: &str) -> bool {
        let key = self.fix_key(key);
        
        if let Ok(cache_dir) = Self::get_cache_dir() {
            let path = cache_dir.join(&key);
            return fs::remove_file(path).is_ok();
        }
        
        false
    }

    pub fn clear(&self, prefix: &str) {
        let prefix = self.fix_key(prefix);
        
        if let Ok(cache_dir) = Self::get_cache_dir() {
            if let Ok(entries) = fs::read_dir(cache_dir) {
                for entry_result in entries {
                    if let Ok(entry) = entry_result {
                        let file_name = entry.file_name();
                        if let Some(file_name_str) = file_name.to_str() {
                            if file_name_str != "." && file_name_str != ".." && 
                               (prefix.is_empty() || file_name_str.starts_with(&prefix)) {
                                let _ = fs::remove_file(entry.path());
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn gc() {
        let last_run = appconfig::get_value("core", "global_cache_gc_lastrun", "0")
            .unwrap_or_else(|_| "0".to_string())
            .parse::<u64>()
            .unwrap_or(0);
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        if (now - last_run) < 300 {
            // only do cleanup every 5 minutes
            return;
        }
        
        let _ = appconfig::set_value("core", "global_cache_gc_lastrun", &now.to_string());
        
        if let Ok(cache_dir) = Self::get_cache_dir() {
            if let Ok(entries) = fs::read_dir(cache_dir) {
                for entry_result in entries {
                    if let Ok(entry) = entry_result {
                        let file_name = entry.file_name();
                        if let Some(file_name_str) = file_name.to_str() {
                            if file_name_str != "." && file_name_str != ".." {
                                if let Ok(metadata) = fs::metadata(entry.path()) {
                                    if let Ok(mtime) = metadata.modified() {
                                        if let Ok(file_time) = mtime.duration_since(UNIX_EPOCH) {
                                            if file_time.as_secs() < now {
                                                let _ = fs::remove_file(entry.path());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}