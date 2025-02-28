// Copyright (c) 2012 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::time::{SystemTime, UNIX_EPOCH};

    use async_trait::async_trait;
    use tempfile::TempDir;
    use uuid::Uuid;

    use nextcloud_core::files::cache::{Cache, PermissionsCache, Scanner};
    use nextcloud_core::files::storage::{Storage, Temporary};

    struct TestPermissions {
        permissions_cache: Arc<dyn PermissionsCache>,
    }

    impl TestPermissions {
        fn new() -> Self {
            Self {
                permissions_cache: Arc::new(nextcloud_core::files::cache::Permissions::new("dummy")),
            }
        }
    }

    #[tokio::test]
    async fn test_simple() {
        let test = TestPermissions::new();
        let ids: Vec<i64> = (1..11).collect();
        let user = Uuid::new_v4().to_string();

        assert_eq!(-1, test.permissions_cache.get(1, &user).await);
        assert!(!test.permissions_cache.get_users(1).await.contains(&user));
        
        test.permissions_cache.set(1, &user, 1).await;
        assert_eq!(1, test.permissions_cache.get(1, &user).await);
        assert!(test.permissions_cache.get_users(1).await.contains(&user));
        assert_eq!(-1, test.permissions_cache.get(2, &user).await);
        assert_eq!(-1, test.permissions_cache.get(1, &format!("{}2", user)).await);

        test.permissions_cache.set(1, &user, 2).await;
        assert_eq!(2, test.permissions_cache.get(1, &user).await);

        test.permissions_cache.set(2, &user, 1).await;
        assert_eq!(1, test.permissions_cache.get(2, &user).await);

        test.permissions_cache.remove(1, &user).await;
        assert_eq!(-1, test.permissions_cache.get(1, &user).await);
        test.permissions_cache.remove(1, &format!("{}2", user)).await;
        assert_eq!(1, test.permissions_cache.get(2, &user).await);

        let mut expected = HashMap::new();
        for id in &ids {
            test.permissions_cache.set(*id, &user, 10 + id).await;
            expected.insert(*id, 10 + id);
        }
        assert_eq!(expected, test.permissions_cache.get_multiple(&ids, &user).await);

        test.permissions_cache.remove_multiple(&[10, 9], &user).await;
        expected.remove(&9);
        expected.remove(&10);
        assert_eq!(expected, test.permissions_cache.get_multiple(&ids, &user).await);

        test.permissions_cache.remove_multiple(&ids, &user).await;
    }

    #[tokio::test]
    async fn test_update_permissions_on_rescan() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().to_path_buf();
        
        let storage = Arc::new(Temporary::new(path));
        let scanner = storage.get_scanner();
        let cache = storage.get_cache();
        let permissions_cache = storage.get_permissions_cache();

        storage.file_put_contents("foo.txt", "bar".as_bytes()).await.unwrap();
        scanner.scan("").await.unwrap();
        let id = cache.get_id("foo.txt").await.unwrap();
        permissions_cache.set(id, "test", 1).await;

        scanner.scan("").await.unwrap();
        assert_eq!(-1, permissions_cache.get(id, "test").await);
    }
}