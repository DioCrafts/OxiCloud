// Copyright (c) 2012 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::fs;
use std::io::Read;
use std::path::Path;
use async_trait::async_trait;
use tokio::fs::File;

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;
    use std::sync::Arc;

    struct TestStorage {
        temp_dir: tempfile::TempDir,
    }

    #[async_trait]
    impl Storage for TestStorage {
        async fn file_put_contents(&self, path: &str, data: &[u8]) -> Result<(), std::io::Error> {
            let full_path = self.temp_dir.path().join(path);
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut file = tokio::fs::File::create(full_path).await?;
            file.write_all(data).await?;
            Ok(())
        }

        async fn mkdir(&self, path: &str) -> Result<(), std::io::Error> {
            fs::create_dir_all(self.temp_dir.path().join(path))?;
            Ok(())
        }

        async fn unlink(&self, path: &str) -> Result<(), std::io::Error> {
            let full_path = self.temp_dir.path().join(path);
            if full_path.is_dir() {
                fs::remove_dir_all(full_path)?;
            } else {
                fs::remove_file(full_path)?;
            }
            Ok(())
        }

        async fn filemtime(&self, path: &str) -> Result<u64, std::io::Error> {
            let metadata = fs::metadata(self.temp_dir.path().join(path))?;
            let modified = metadata.modified()?;
            let since_epoch = modified
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap();
            Ok(since_epoch.as_secs())
        }
    }

    struct CacheEntry {
        fileid: i64,
        size: i64,
        etag: String,
        mimetype: String,
        parent: i64,
        mtime: u64,
    }

    struct CacheItem {
        path: String,
        entry: CacheEntry,
    }

    struct Cache {
        storage: Arc<TestStorage>,
        entries: Vec<CacheItem>,
    }

    impl Cache {
        fn new(storage: Arc<TestStorage>) -> Self {
            Self {
                storage,
                entries: Vec::new(),
            }
        }

        fn in_cache(&self, path: &str) -> bool {
            self.entries.iter().any(|item| item.path == path)
        }

        fn get(&self, path: &str) -> Option<HashMap<String, serde_json::Value>> {
            self.entries.iter()
                .find(|item| item.path == path)
                .map(|item| {
                    let mut map = HashMap::new();
                    map.insert("fileid".to_string(), serde_json::Value::Number(serde_json::Number::from(item.entry.fileid)));
                    map.insert("size".to_string(), serde_json::Value::Number(serde_json::Number::from(item.entry.size)));
                    map.insert("etag".to_string(), serde_json::Value::String(item.entry.etag.clone()));
                    map.insert("mimetype".to_string(), serde_json::Value::String(item.entry.mimetype.clone()));
                    map.insert("parent".to_string(), serde_json::Value::Number(serde_json::Number::from(item.entry.parent)));
                    map.insert("mtime".to_string(), serde_json::Value::Number(serde_json::Number::from(item.entry.mtime)));
                    map
                })
        }

        fn put(&mut self, path: &str, data: HashMap<String, serde_json::Value>) {
            if let Some(index) = self.entries.iter().position(|item| item.path == path) {
                if let Some(mtime) = data.get("mtime") {
                    if let Some(mtime) = mtime.as_u64() {
                        self.entries[index].entry.mtime = mtime;
                    }
                }
                if let Some(etag) = data.get("etag") {
                    if let Some(etag) = etag.as_str() {
                        self.entries[index].entry.etag = etag.to_string();
                    }
                }
            }
        }

        fn get_all(&self) -> Vec<HashMap<String, serde_json::Value>> {
            self.entries.iter().map(|item| {
                let mut map = HashMap::new();
                map.insert("fileid".to_string(), serde_json::Value::Number(serde_json::Number::from(item.entry.fileid)));
                map.insert("path".to_string(), serde_json::Value::String(item.path.clone()));
                map
            }).collect()
        }

        fn clear(&mut self) {
            self.entries.clear();
        }

        fn correct_folder_size(&mut self, path: &str) {
            if let Some(index) = self.entries.iter().position(|item| item.path == path) {
                self.entries[index].entry.size = 0;
                // Simulate calculating correct size by summing children
                for item in &self.entries {
                    if item.path.starts_with(&format!("{}/", path)) && !item.path.contains("/") {
                        self.entries[index].entry.size += item.entry.size;
                    }
                }
            }
        }

        fn get_incomplete(&self) -> bool {
            false
        }
    }

    struct PermissionsCache {
        // Simplified implementation
    }

    impl PermissionsCache {
        fn new(_storage: Arc<TestStorage>) -> Self {
            Self {}
        }

        fn remove_multiple(&self, _ids: Vec<HashMap<String, serde_json::Value>>, _user: &str) {
            // Simplified implementation
        }
    }

    struct Scanner {
        storage: Arc<TestStorage>,
        cache: Arc<std::sync::Mutex<Cache>>,
    }

    impl Scanner {
        const SCAN_RECURSIVE: i32 = 1;
        const SCAN_SHALLOW: i32 = 2;
        const REUSE_ETAG: i32 = 4;
        const REUSE_SIZE: i32 = 8;

        fn new(storage: Arc<TestStorage>, cache: Arc<std::sync::Mutex<Cache>>) -> Self {
            Self {
                storage,
                cache,
            }
        }

        async fn scan_file(&self, path: &str) -> Result<(), std::io::Error> {
            self.scan_file_with_options(path, 0).await
        }

        async fn scan_file_with_options(&self, path: &str, options: i32) -> Result<(), std::io::Error> {
            let full_path = self.storage.temp_dir.path().join(path);
            
            if !full_path.exists() {
                let mut cache = self.cache.lock().unwrap();
                // Remove from cache if file doesn't exist
                cache.entries.retain(|item| item.path != path);
                return Ok(());
            }

            let metadata = fs::metadata(&full_path)?;
            
            let mut mimetype = "application/octet-stream".to_string();
            if path.ends_with(".txt") {
                mimetype = "text/plain".to_string();
            } else if path.ends_with(".png") {
                mimetype = "image/png".to_string();
            }

            let new_etag = format!("{:x}", rand::random::<u64>());
            
            let mut cache = self.cache.lock().unwrap();
            
            if let Some(index) = cache.entries.iter().position(|item| item.path == path) {
                if (options & Self::REUSE_ETAG == 0) || cache.entries[index].entry.etag.is_empty() {
                    cache.entries[index].entry.etag = new_etag;
                }
                
                if options & Self::REUSE_SIZE == 0 {
                    cache.entries[index].entry.size = metadata.len() as i64;
                }
                
                cache.entries[index].entry.mtime = 
                    metadata.modified()?.duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs();
            } else {
                // Create parent folders if needed
                if let Some(parent_path) = Path::new(path).parent() {
                    let parent_path_str = parent_path.to_string_lossy();
                    if !parent_path_str.is_empty() && !cache.in_cache(&parent_path_str) {
                        drop(cache); // Release lock before recursive call
                        self.scan(&parent_path_str, Self::SCAN_SHALLOW, 0).await?;
                        cache = self.cache.lock().unwrap();
                    }
                }
                
                let parent_id = if path.contains('/') {
                    let parent_path = path.rsplit_once('/').unwrap().0;
                    if let Some(parent_data) = cache.get(parent_path) {
                        parent_data.get("fileid").and_then(|v| v.as_i64()).unwrap_or(-1)
                    } else {
                        -1
                    }
                } else {
                    // Root folder is the parent
                    if let Some(root_data) = cache.get("") {
                        root_data.get("fileid").and_then(|v| v.as_i64()).unwrap_or(-1)
                    } else {
                        -1
                    }
                };
                
                cache.entries.push(CacheItem {
                    path: path.to_string(),
                    entry: CacheEntry {
                        fileid: rand::random::<i64>(),
                        size: metadata.len() as i64,
                        etag: new_etag,
                        mimetype,
                        parent: parent_id,
                        mtime: metadata.modified()?.duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs(),
                    }
                });
            }
            
            Ok(())
        }

        async fn scan(&self, path: &str, scan_type: i32, options: i32) -> Result<(), std::io::Error> {
            let full_path = self.storage.temp_dir.path().join(path);
            
            if !full_path.exists() {
                let mut cache = self.cache.lock().unwrap();
                // Remove from cache if path doesn't exist
                let to_remove: Vec<String> = cache.entries.iter()
                    .filter(|item| item.path == path || item.path.starts_with(&format!("{}/", path)))
                    .map(|item| item.path.clone())
                    .collect();
                
                for item_path in to_remove {
                    cache.entries.retain(|item| item.path != item_path);
                }
                return Ok(());
            }
            
            // Scan current path
            self.scan_file_with_options(path, options).await?;
            
            if !full_path.is_dir() {
                return Ok(());
            }
            
            // List directory contents
            let entries = fs::read_dir(full_path)?;
            
            // Process directory contents
            for entry in entries {
                let entry = entry?;
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();
                
                let sub_path = if path.is_empty() {
                    file_name_str.to_string()
                } else {
                    format!("{}/{}", path, file_name_str)
                };
                
                if scan_type == Self::SCAN_RECURSIVE {
                    self.scan(&sub_path, scan_type, options).await?;
                } else {
                    self.scan_file_with_options(&sub_path, options).await?;
                }
            }
            
            // Update folder size if needed
            if scan_type == Self::SCAN_RECURSIVE && (options & Self::REUSE_SIZE) == 0 {
                let mut cache = self.cache.lock().unwrap();
                if let Some(index) = cache.entries.iter().position(|item| item.path == path) {
                    let mut total_size = 0;
                    for item in &cache.entries {
                        let item_path = &item.path;
                        if item_path != path && 
                           (item_path.starts_with(&format!("{}/", path)) || (path.is_empty() && !item_path.contains('/'))) {
                            total_size += item.entry.size;
                        }
                    }
                    cache.entries[index].entry.size = total_size;
                }
            } else if scan_type == Self::SCAN_SHALLOW && (options & Self::REUSE_SIZE) == 0 {
                let mut cache = self.cache.lock().unwrap();
                if let Some(index) = cache.entries.iter().position(|item| item.path == path) {
                    cache.entries[index].entry.size = -1;
                }
            }
            
            Ok(())
        }

        async fn background_scan(&self) -> Result<(), std::io::Error> {
            // Scan all entries marked as incomplete
            let mut cache = self.cache.lock().unwrap();
            let incomplete: Vec<String> = cache.entries.iter()
                .filter(|item| item.entry.size == -1)
                .map(|item| item.path.clone())
                .collect();
            drop(cache);
            
            for path in incomplete {
                self.scan(&path, Self::SCAN_RECURSIVE, 0).await?;
            }
            
            Ok(())
        }
    }

    #[async_trait]
    trait Storage {
        async fn file_put_contents(&self, path: &str, data: &[u8]) -> Result<(), std::io::Error>;
        async fn mkdir(&self, path: &str) -> Result<(), std::io::Error>;
        async fn unlink(&self, path: &str) -> Result<(), std::io::Error>;
        async fn filemtime(&self, path: &str) -> Result<u64, std::io::Error>;
    }

    use std::collections::HashMap;

    #[tokio::test]
    async fn test_file() {
        let storage = Arc::new(TestStorage {
            temp_dir: tempfile::tempdir().unwrap(),
        });
        let cache = Arc::new(std::sync::Mutex::new(Cache::new(storage.clone())));
        let scanner = Scanner::new(storage.clone(), cache.clone());

        let data = b"dummy file data\n";
        storage.file_put_contents("foo.txt", data).await.unwrap();
        scanner.scan_file("foo.txt").await.unwrap();

        let cache_guard = cache.lock().unwrap();
        assert_eq!(cache_guard.in_cache("foo.txt"), true);
        let cached_data = cache_guard.get("foo.txt").unwrap();
        assert_eq!(cached_data["size"].as_i64().unwrap(), data.len() as i64);
        assert_eq!(cached_data["mimetype"].as_str().unwrap(), "text/plain");
        assert_ne!(cached_data["parent"].as_i64().unwrap(), -1); // parent folders should be scanned automatically

        drop(cache_guard); // Release the lock before next operations

        // Not using core/img/logo.png since we're not in the same filesystem
        let img_data = b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x06\x00\x00\x00\x1f\x15\xc4\x89\x00\x00\x00\rIDATx\x9cc\xf8\xff\xff?\x03\x00\x08\xfc\x02\xfe\xa7\xef\xa0L\x00\x00\x00\x00IEND\xaeB`\x82";
        storage.file_put_contents("foo.png", img_data).await.unwrap();
        scanner.scan_file("foo.png").await.unwrap();

        let cache_guard = cache.lock().unwrap();
        assert_eq!(cache_guard.in_cache("foo.png"), true);
        let cached_data = cache_guard.get("foo.png").unwrap();
        assert_eq!(cached_data["size"].as_i64().unwrap(), img_data.len() as i64);
        assert_eq!(cached_data["mimetype"].as_str().unwrap(), "image/png");
    }

    async fn fill_test_folders(storage: &Arc<TestStorage>) -> Result<(), std::io::Error> {
        let text_data = b"dummy file data\n";
        let img_data = b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x06\x00\x00\x00\x1f\x15\xc4\x89\x00\x00\x00\rIDATx\x9cc\xf8\xff\xff?\x03\x00\x08\xfc\x02\xfe\xa7\xef\xa0L\x00\x00\x00\x00IEND\xaeB`\x82";
        
        storage.mkdir("folder").await?;
        storage.file_put_contents("foo.txt", text_data).await?;
        storage.file_put_contents("foo.png", img_data).await?;
        storage.file_put_contents("folder/bar.txt", text_data).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_folder() {
        let storage = Arc::new(TestStorage {
            temp_dir: tempfile::tempdir().unwrap(),
        });
        let cache = Arc::new(std::sync::Mutex::new(Cache::new(storage.clone())));
        let scanner = Scanner::new(storage.clone(), cache.clone());

        fill_test_folders(&storage).await.unwrap();

        scanner.scan("", Scanner::SCAN_RECURSIVE, 0).await.unwrap();
        
        let cache_guard = cache.lock().unwrap();
        assert_eq!(cache_guard.in_cache(""), true);
        assert_eq!(cache_guard.in_cache("foo.txt"), true);
        assert_eq!(cache_guard.in_cache("foo.png"), true);
        assert_eq!(cache_guard.in_cache("folder"), true);
        assert_eq!(cache_guard.in_cache("folder/bar.txt"), true);

        let cached_data_text = cache_guard.get("foo.txt").unwrap();
        let cached_data_text2 = cache_guard.get("folder/bar.txt").unwrap();
        let cached_data_image = cache_guard.get("foo.png").unwrap();
        let cached_data_folder = cache_guard.get("").unwrap();
        let cached_data_folder2 = cache_guard.get("folder").unwrap();

        assert_eq!(cached_data_image["parent"].as_i64().unwrap(), cached_data_text["parent"].as_i64().unwrap());
        assert_eq!(cached_data_folder["fileid"].as_i64().unwrap(), cached_data_image["parent"].as_i64().unwrap());
        assert_eq!(cached_data_folder["size"].as_i64().unwrap(), 
                cached_data_image["size"].as_i64().unwrap() + 
                cached_data_text["size"].as_i64().unwrap() + 
                cached_data_text2["size"].as_i64().unwrap());
        assert_eq!(cached_data_folder2["size"].as_i64().unwrap(), cached_data_text2["size"].as_i64().unwrap());
    }

    #[tokio::test]
    async fn test_shallow() {
        let storage = Arc::new(TestStorage {
            temp_dir: tempfile::tempdir().unwrap(),
        });
        let cache = Arc::new(std::sync::Mutex::new(Cache::new(storage.clone())));
        let scanner = Scanner::new(storage.clone(), cache.clone());

        fill_test_folders(&storage).await.unwrap();

        scanner.scan("", Scanner::SCAN_SHALLOW, 0).await.unwrap();
        
        let cache_guard = cache.lock().unwrap();
        assert_eq!(cache_guard.in_cache(""), true);
        assert_eq!(cache_guard.in_cache("foo.txt"), true);
        assert_eq!(cache_guard.in_cache("foo.png"), true);
        assert_eq!(cache_guard.in_cache("folder"), true);
        assert_eq!(cache_guard.in_cache("folder/bar.txt"), false);

        let cached_data_folder = cache_guard.get("").unwrap();
        let cached_data_folder2 = cache_guard.get("folder").unwrap();

        assert_eq!(cached_data_folder["size"].as_i64().unwrap(), -1);
        assert_eq!(cached_data_folder2["size"].as_i64().unwrap(), -1);
        
        drop(cache_guard); // Release lock before next operations

        scanner.scan("folder", Scanner::SCAN_SHALLOW, 0).await.unwrap();

        let cache_guard = cache.lock().unwrap();
        let cached_data_folder2 = cache_guard.get("folder").unwrap();

        assert_ne!(cached_data_folder2["size"].as_i64().unwrap(), -1);
        
        drop(cache_guard); // Release lock before next operations

        cache.lock().unwrap().correct_folder_size("folder");

        let cache_guard = cache.lock().unwrap();
        let cached_data_folder = cache_guard.get("").unwrap();
        assert_ne!(cached_data_folder["size"].as_i64().unwrap(), -1);
    }

    #[tokio::test]
    async fn test_background_scan() {
        let storage = Arc::new(TestStorage {
            temp_dir: tempfile::tempdir().unwrap(),
        });
        let cache = Arc::new(std::sync::Mutex::new(Cache::new(storage.clone())));
        let scanner = Scanner::new(storage.clone(), cache.clone());

        fill_test_folders(&storage).await.unwrap();
        storage.mkdir("folder2").await.unwrap();
        storage.file_put_contents("folder2/bar.txt", b"foobar").await.unwrap();

        scanner.scan("", Scanner::SCAN_SHALLOW, 0).await.unwrap();
        
        let cache_guard = cache.lock().unwrap();
        assert_eq!(cache_guard.in_cache("folder/bar.txt"), false);
        let cached_data = cache_guard.get("").unwrap();
        assert_eq!(cached_data["size"].as_i64().unwrap(), -1);
        
        drop(cache_guard); // Release lock before next operations

        scanner.background_scan().await.unwrap();

        let cache_guard = cache.lock().unwrap();
        assert_eq!(cache_guard.in_cache("folder/bar.txt"), true);

        let cached_data = cache_guard.get("").unwrap();
        assert_ne!(cached_data["size"].as_i64().unwrap(), -1);

        assert_eq!(cache_guard.get_incomplete(), false);
    }

    #[tokio::test]
    async fn test_reuse_existing() {
        let storage = Arc::new(TestStorage {
            temp_dir: tempfile::tempdir().unwrap(),
        });
        let cache = Arc::new(std::sync::Mutex::new(Cache::new(storage.clone())));
        let scanner = Scanner::new(storage.clone(), cache.clone());

        fill_test_folders(&storage).await.unwrap();

        scanner.scan("", Scanner::SCAN_RECURSIVE, 0).await.unwrap();
        
        let old_data = cache.lock().unwrap().get("").unwrap();
        let old_etag = old_data["etag"].as_str().unwrap().to_string();
        let old_size = old_data["size"].as_i64().unwrap();
        
        storage.unlink("folder/bar.txt").await.unwrap();
        
        {
            let mut cache_guard = cache.lock().unwrap();
            let folder_mtime = storage.filemtime("folder").await.unwrap();
            let mut folder_data = HashMap::new();
            folder_data.insert("mtime".to_string(), serde_json::Value::Number(serde_json::Number::from(folder_mtime)));
            cache_guard.put("folder", folder_data);
        }
        
        scanner.scan("", Scanner::SCAN_SHALLOW, Scanner::REUSE_SIZE).await.unwrap();
        
        let new_data = cache.lock().unwrap().get("").unwrap();
        assert_ne!(new_data["etag"].as_str().unwrap(), old_etag);
        assert_eq!(new_data["size"].as_i64().unwrap(), old_size);

        let old_data = new_data;
        let old_etag = old_data["etag"].as_str().unwrap().to_string();
        
        scanner.scan("", Scanner::SCAN_SHALLOW, Scanner::REUSE_ETAG).await.unwrap();
        
        let new_data = cache.lock().unwrap().get("").unwrap();
        assert_eq!(new_data["etag"].as_str().unwrap(), old_etag);
        assert_eq!(new_data["size"].as_i64().unwrap(), -1);

        scanner.scan("", Scanner::SCAN_RECURSIVE, 0).await.unwrap();
        
        let old_data = cache.lock().unwrap().get("").unwrap();
        let old_etag = old_data["etag"].as_str().unwrap().to_string();
        let old_size = old_data["size"].as_i64().unwrap();
        assert_ne!(old_size, -1);
        
        scanner.scan_file_with_options("", Scanner::REUSE_ETAG | Scanner::REUSE_SIZE).await.unwrap();
        
        let new_data = cache.lock().unwrap().get("").unwrap();
        assert_eq!(new_data["etag"].as_str().unwrap(), old_etag);
        assert_eq!(new_data["size"].as_i64().unwrap(), old_size);

        scanner.scan("", Scanner::SCAN_RECURSIVE, Scanner::REUSE_ETAG | Scanner::REUSE_SIZE).await.unwrap();
        
        let new_data = cache.lock().unwrap().get("").unwrap();
        assert_eq!(new_data["etag"].as_str().unwrap(), old_etag);
        assert_eq!(new_data["size"].as_i64().unwrap(), old_size);

        scanner.scan("", Scanner::SCAN_SHALLOW, Scanner::REUSE_ETAG | Scanner::REUSE_SIZE).await.unwrap();
        
        let new_data = cache.lock().unwrap().get("").unwrap();
        assert_eq!(new_data["etag"].as_str().unwrap(), old_etag);
        assert_eq!(new_data["size"].as_i64().unwrap(), old_size);
    }

    #[tokio::test]
    async fn test_removed_file() {
        let storage = Arc::new(TestStorage {
            temp_dir: tempfile::tempdir().unwrap(),
        });
        let cache = Arc::new(std::sync::Mutex::new(Cache::new(storage.clone())));
        let scanner = Scanner::new(storage.clone(), cache.clone());

        fill_test_folders(&storage).await.unwrap();

        scanner.scan("", Scanner::SCAN_RECURSIVE, 0).await.unwrap();
        assert_eq!(cache.lock().unwrap().in_cache("foo.txt"), true);
        
        storage.unlink("foo.txt").await.unwrap();
        scanner.scan("", Scanner::SCAN_SHALLOW, 0).await.unwrap();
        
        assert_eq!(cache.lock().unwrap().in_cache("foo.txt"), false);
    }

    #[tokio::test]
    async fn test_removed_folder() {
        let storage = Arc::new(TestStorage {
            temp_dir: tempfile::tempdir().unwrap(),
        });
        let cache = Arc::new(std::sync::Mutex::new(Cache::new(storage.clone())));
        let scanner = Scanner::new(storage.clone(), cache.clone());

        fill_test_folders(&storage).await.unwrap();

        scanner.scan("", Scanner::SCAN_RECURSIVE, 0).await.unwrap();
        assert_eq!(cache.lock().unwrap().in_cache("folder/bar.txt"), true);
        
        storage.unlink("folder").await.unwrap();
        scanner.scan("", Scanner::SCAN_SHALLOW, 0).await.unwrap();
        
        assert_eq!(cache.lock().unwrap().in_cache("folder"), false);
        assert_eq!(cache.lock().unwrap().in_cache("folder/bar.txt"), false);
    }

    #[tokio::test]
    async fn test_scan_removed_file() {
        let storage = Arc::new(TestStorage {
            temp_dir: tempfile::tempdir().unwrap(),
        });
        let cache = Arc::new(std::sync::Mutex::new(Cache::new(storage.clone())));
        let scanner = Scanner::new(storage.clone(), cache.clone());

        fill_test_folders(&storage).await.unwrap();

        scanner.scan("", Scanner::SCAN_RECURSIVE, 0).await.unwrap();
        assert_eq!(cache.lock().unwrap().in_cache("folder/bar.txt"), true);
        
        storage.unlink("folder/bar.txt").await.unwrap();
        scanner.scan_file("folder/bar.txt").await.unwrap();
        
        assert_eq!(cache.lock().unwrap().in_cache("folder/bar.txt"), false);
    }

    #[tokio::test]
    async fn test_etag_recreation() {
        let storage = Arc::new(TestStorage {
            temp_dir: tempfile::tempdir().unwrap(),
        });
        let cache = Arc::new(std::sync::Mutex::new(Cache::new(storage.

}} // Añadido por reparador automático