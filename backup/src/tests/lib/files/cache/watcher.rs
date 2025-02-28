// Copyright (c) 2012 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use std::path::Path;
    use std::sync::Arc;
    use uuid::Uuid;

    // Mocking the necessary traits and structures
    #[async_trait]
    trait Storage: Send + Sync {
        async fn file_put_contents(&self, path: &str, contents: &[u8]) -> Result<(), StorageError>;
        async fn mkdir(&self, path: &str) -> Result<(), StorageError>;
        async fn unlink(&self, path: &str) -> Result<(), StorageError>;
        async fn rename(&self, source: &str, destination: &str) -> Result<(), StorageError>;
        fn get_cache(&self) -> Arc<dyn Cache>;
        fn get_watcher(&self) -> Arc<dyn Watcher>;
        fn get_scanner(&self) -> Arc<dyn Scanner>;
        fn get_permissions_cache(&self) -> Arc<dyn PermissionsCache>;
    }

    #[derive(Debug)]
    enum StorageError {
        NotFound,
        AlreadyExists,
        PermissionDenied,
        Other(String),
    }

    #[async_trait]
    trait Cache: Send + Sync {
        async fn put(&self, path: &str, data: CacheEntry) -> Result<(), CacheError>;
        async fn get(&self, path: &str) -> Result<CacheEntry, CacheError>;
        async fn in_cache(&self, path: &str) -> bool;
        async fn get_all(&self) -> Vec<CacheEntry>;
        async fn clear(&self) -> Result<(), CacheError>;
    }

    #[derive(Debug)]
    enum CacheError {
        NotFound,
        Other(String),
    }

    #[derive(Debug, Clone)]
    struct CacheEntry {
        path: String,
        size: u64,
        mimetype: String,
        storage_mtime: i64,
        // Other fields would be here in a complete implementation
    }

    #[async_trait]
    trait PermissionsCache: Send + Sync {
        async fn remove_multiple(&self, entries: Vec<CacheEntry>, user: &str) -> Result<(), CacheError>;
    }

    #[async_trait]
    trait Watcher: Send + Sync {
        async fn check_update(&self, path: &str) -> Result<(), StorageError>;
    }

    #[async_trait]
    trait Scanner: Send + Sync {
        async fn scan(&self, path: &str) -> Result<(), StorageError>;
    }

    struct TemporaryStorage {
        id: String,
        cache: Arc<dyn Cache>,
        watcher: Arc<dyn Watcher>,
        scanner: Arc<dyn Scanner>,
        permissions_cache: Arc<dyn PermissionsCache>,
        // Other fields would be here
    }

    impl TemporaryStorage {
        fn new() -> Self {
            let id = Uuid::new_v4().to_string();
            let cache = Arc::new(MemoryCache::new());
            let watcher = Arc::new(DefaultWatcher::new(cache.clone()));
            let scanner = Arc::new(DefaultScanner::new(cache.clone()));
            let permissions_cache = Arc::new(DefaultPermissionsCache::new());
            
            Self {
                id,
                cache,
                watcher,
                scanner,
                permissions_cache,
            }
        }
    }

    #[async_trait]
    impl Storage for TemporaryStorage {
        async fn file_put_contents(&self, path: &str, contents: &[u8]) -> Result<(), StorageError> {
            // Implementation would write to a temporary file
            Ok(())
        }

        async fn mkdir(&self, path: &str) -> Result<(), StorageError> {
            // Implementation would create a directory
            Ok(())
        }

        async fn unlink(&self, path: &str) -> Result<(), StorageError> {
            // Implementation would remove a file or directory
            Ok(())
        }

        async fn rename(&self, source: &str, destination: &str) -> Result<(), StorageError> {
            // Implementation would rename a file or directory
            Ok(())
        }

        fn get_cache(&self) -> Arc<dyn Cache> {
            self.cache.clone()
        }

        fn get_watcher(&self) -> Arc<dyn Watcher> {
            self.watcher.clone()
        }

        fn get_scanner(&self) -> Arc<dyn Scanner> {
            self.scanner.clone()
        }

        fn get_permissions_cache(&self) -> Arc<dyn PermissionsCache> {
            self.permissions_cache.clone()
        }
    }

    // Implementations of the cache, watcher, scanner, permissions cache
    struct MemoryCache {
        // Implementation details
    }

    impl MemoryCache {
        fn new() -> Self {
            Self {}
        }
    }

    #[async_trait]
    impl Cache for MemoryCache {
        async fn put(&self, path: &str, data: CacheEntry) -> Result<(), CacheError> {
            // Implementation would store data in memory
            Ok(())
        }

        async fn get(&self, path: &str) -> Result<CacheEntry, CacheError> {
            // Implementation would retrieve data from memory
            Ok(CacheEntry {
                path: path.to_string(),
                size: 0,
                mimetype: "text/plain".to_string(),
                storage_mtime: 0,
            })
        }

        async fn in_cache(&self, path: &str) -> bool {
            // Implementation would check if path exists in cache
            true
        }

        async fn get_all(&self) -> Vec<CacheEntry> {
            // Implementation would return all cache entries
            Vec::new()
        }

        async fn clear(&self) -> Result<(), CacheError> {
            // Implementation would clear cache
            Ok(())
        }
    }

    struct DefaultWatcher {
        cache: Arc<dyn Cache>,
    }

    impl DefaultWatcher {
        fn new(cache: Arc<dyn Cache>) -> Self {
            Self { cache }
        }
    }

    #[async_trait]
    impl Watcher for DefaultWatcher {
        async fn check_update(&self, path: &str) -> Result<(), StorageError> {
            // Implementation would check for updates
            Ok(())
        }
    }

    struct DefaultScanner {
        cache: Arc<dyn Cache>,
    }

    impl DefaultScanner {
        fn new(cache: Arc<dyn Cache>) -> Self {
            Self { cache }
        }
    }

    #[async_trait]
    impl Scanner for DefaultScanner {
        async fn scan(&self, path: &str) -> Result<(), StorageError> {
            // Implementation would scan the directory
            Ok(())
        }
    }

    struct DefaultPermissionsCache {}

    impl DefaultPermissionsCache {
        fn new() -> Self {
            Self {}
        }
    }

    #[async_trait]
    impl PermissionsCache for DefaultPermissionsCache {
        async fn remove_multiple(&self, entries: Vec<CacheEntry>, user: &str) -> Result<(), CacheError> {
            // Implementation would remove permissions for multiple entries
            Ok(())
        }
    }

    // Filesystem helper (mock)
    struct Filesystem;
    
    impl Filesystem {
        async fn clear_mounts() {
            // Implementation would clear mounts
        }
    }

    // User helper (mock)
    struct User;
    
    impl User {
        fn get_user() -> String {
            "test_user".to_string()
        }
    }

    // The actual test module
    mod watcher_tests {
        use super::*;
        use tokio::fs;

        struct WatcherTest {
            storages: Vec<Arc<dyn Storage>>,
        }

        impl WatcherTest {
            fn new() -> Self {
                Self {
                    storages: Vec::new(),
                }
            }

            async fn set_up(&mut self) {
                Filesystem::clear_mounts().await;
            }

            async fn tear_down(&mut self) {
                for storage in &self.storages {
                    let cache = storage.get_cache();
                    let ids = cache.get_all().await;
                    let permissions_cache = storage.get_permissions_cache();
                    permissions_cache.remove_multiple(ids, &User::get_user()).await.unwrap();
                    cache.clear().await.unwrap();
                }
            }

            async fn get_test_storage(&mut self, scan: bool) -> Arc<dyn Storage> {
                let storage = Arc::new(TemporaryStorage::new());
                
                let text_data = "dummy file data\n".as_bytes();
                // In a real implementation, we would read the image data
                let img_data = &[0u8; 100]; // Placeholder
                
                storage.mkdir("folder").await.unwrap();
                storage.file_put_contents("foo.txt", text_data).await.unwrap();
                storage.file_put_contents("foo.png", img_data).await.unwrap();
                storage.file_put_contents("folder/bar.txt", text_data).await.unwrap();
                storage.file_put_contents("folder/bar2.txt", text_data).await.unwrap();

                if scan {
                    let scanner = storage.get_scanner();
                    scanner.scan("").await.unwrap();
                }
                
                self.storages.push(storage.clone());
                storage
            }

            async fn test_watcher(&mut self) {
                let storage = self.get_test_storage(true).await;
                let cache = storage.get_cache();
                let updater = storage.get_watcher();

                // Set the mtime to the past so it can detect an mtime change
                cache.put("", CacheEntry {
                    path: "".to_string(),
                    size: 0,
                    mimetype: "httpd/unix-directory".to_string(),
                    storage_mtime: 10,
                }).await.unwrap();

                assert!(cache.in_cache("folder/bar.txt").await);
                assert!(cache.in_cache("folder/bar2.txt").await);

                assert!(!cache.in_cache("bar.test").await);
                storage.file_put_contents("bar.test", "foo".as_bytes()).await.unwrap();
                updater.check_update("").await.unwrap();
                assert!(cache.in_cache("bar.test").await);
                let cached_data = cache.get("bar.test").await.unwrap();
                assert_eq!(3, cached_data.size);

                cache.put("bar.test", CacheEntry {
                    path: "bar.test".to_string(),
                    size: 0,
                    mimetype: "text/plain".to_string(),
                    storage_mtime: 10,
                }).await.unwrap();
                storage.file_put_contents("bar.test", "test data".as_bytes()).await.unwrap();

                updater.check_update("bar.test").await.unwrap();
                let cached_data = cache.get("bar.test").await.unwrap();
                assert_eq!(9, cached_data.size);

                cache.put("folder", CacheEntry {
                    path: "folder".to_string(),
                    size: 0,
                    mimetype: "httpd/unix-directory".to_string(),
                    storage_mtime: 10,
                }).await.unwrap();

                storage.unlink("folder/bar2.txt").await.unwrap();
                updater.check_update("folder").await.unwrap();

                assert!(cache.in_cache("folder/bar.txt").await);
                assert!(!cache.in_cache("folder/bar2.txt").await);
            }

            async fn test_file_to_folder(&mut self) {
                let storage = self.get_test_storage(true).await;
                let cache = storage.get_cache();
                let updater = storage.get_watcher();

                // Set the mtime to the past so it can detect an mtime change
                cache.put("", CacheEntry {
                    path: "".to_string(),
                    size: 0,
                    mimetype: "httpd/unix-directory".to_string(),
                    storage_mtime: 10,
                }).await.unwrap();

                storage.unlink("foo.txt").await.unwrap();
                storage.rename("folder", "foo.txt").await.unwrap();
                updater.check_update("").await.unwrap();

                let entry = cache.get("foo.txt").await.unwrap();
                assert_eq!("httpd/unix-directory", entry.mimetype);
                assert!(!cache.in_cache("folder").await);
                assert!(!cache.in_cache("folder/bar.txt").await);

                let storage = self.get_test_storage(true).await;
                let cache = storage.get_cache();
                let updater = storage.get_watcher();

                // Set the mtime to the past so it can detect an mtime change
                cache.put("foo.txt", CacheEntry {
                    path: "foo.txt".to_string(),
                    size: 0, 
                    mimetype: "text/plain".to_string(),
                    storage_mtime: 10,
                }).await.unwrap();

                storage.unlink("foo.txt").await.unwrap();
                storage.rename("folder", "foo.txt").await.unwrap();
                updater.check_update("foo.txt").await.unwrap();

                let entry = cache.get("foo.txt").await.unwrap();
                assert_eq!("httpd/unix-directory", entry.mimetype);
                assert!(cache.in_cache("foo.txt/bar.txt").await);
            }
        }

        #[tokio::test]
        async fn test_watcher() {
            let mut test = WatcherTest::new();
            test.set_up().await;
            test.test_watcher().await;
            test.tear_down().await;
        }

        #[tokio::test]
        async fn test_file_to_folder() {
            let mut test = WatcherTest::new();
            test.set_up().await;
            test.test_file_to_folder().await;
            test.tear_down().await;
        }
    }
}