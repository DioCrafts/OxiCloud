// Copyright (c) 2012 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::sync::{Arc, Mutex};

use nextcloud_core::files::mount::Mount;
use nextcloud_core::files::storage::temporary::Temporary;
use nextcloud_core::files::utils::Scanner as CoreScanner;

// Test module for Scanner tests
#[cfg(test)]
mod tests {
    use super::*;
    use nextcloud_testing::test_case;

    /// Test implementation of Scanner that allows adding mounts for testing
    struct TestScanner {
        /// Collection of mounts for testing
        mounts: Vec<Arc<Mount>>,
        
        /// Root path for the scanner
        root: String,
    }

    impl TestScanner {
        /// Create a new test scanner with the given root path
        pub fn new(root: &str) -> Self {
            Self {
                mounts: Vec::new(),
                root: root.to_string(),
            }
        }

        /// Add a mount to the test scanner
        pub fn add_mount(&mut self, mount: Arc<Mount>) {
            self.mounts.push(mount);
        }
    }

    impl CoreScanner for TestScanner {
        fn get_mounts(&self, _dir: &str) -> Vec<Arc<Mount>> {
            self.mounts.clone()
        }
        
        fn get_root(&self) -> &str {
            &self.root
        }
    }

    #[test_case]
    async fn test_reuse_existing_root() {
        let storage = Arc::new(Mutex::new(Temporary::new()));
        let mount = Arc::new(Mount::new(storage.clone(), "".to_string()));
        let cache = storage.lock().unwrap().get_cache();

        storage.lock().unwrap().mkdir("folder").await.unwrap();
        storage.lock().unwrap().file_put_contents("foo.txt", "qwerty").await.unwrap();
        storage.lock().unwrap().file_put_contents("folder/bar.txt", "qwerty").await.unwrap();

        let mut scanner = TestScanner::new("");
        scanner.add_mount(mount);

        scanner.scan("").await.unwrap();
        assert!(cache.lock().unwrap().in_cache("folder/bar.txt").await.unwrap());
        let old_root = cache.lock().unwrap().get("").await.unwrap();

        scanner.scan("").await.unwrap();
        let new_root = cache.lock().unwrap().get("").await.unwrap();
        assert_eq!(old_root, new_root);
    }

    #[test_case]
    async fn test_reuse_existing_file() {
        let storage = Arc::new(Mutex::new(Temporary::new()));
        let mount = Arc::new(Mount::new(storage.clone(), "".to_string()));
        let cache = storage.lock().unwrap().get_cache();

        storage.lock().unwrap().mkdir("folder").await.unwrap();
        storage.lock().unwrap().file_put_contents("foo.txt", "qwerty").await.unwrap();
        storage.lock().unwrap().file_put_contents("folder/bar.txt", "qwerty").await.unwrap();

        let mut scanner = TestScanner::new("");
        scanner.add_mount(mount);

        scanner.scan("").await.unwrap();
        assert!(cache.lock().unwrap().in_cache("folder/bar.txt").await.unwrap());
        let old = cache.lock().unwrap().get("folder/bar.txt").await.unwrap();

        scanner.scan("").await.unwrap();
        let new = cache.lock().unwrap().get("folder/bar.txt").await.unwrap();
        assert_eq!(old, new);
    }
}