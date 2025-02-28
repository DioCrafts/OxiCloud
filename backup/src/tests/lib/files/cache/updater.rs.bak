// tests/lib/files/cache/updater.rs

use oc_fs::{
    files::{
        cache::{cache::Cache, scanner::Scanner, updater::Updater as OCUpdater},
        filesystem::{self as Filesystem, FilesystemInterface},
    },
    storage::temporary::Temporary,
};
use oc_app::{App, AppManager};
use oc_user::User;
use oc_hook::Hook;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};

/// Test for the filesystem cache updater functionality
#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    struct UpdaterTest {
        storage: Arc<Temporary>,
        scanner: Arc<Scanner>,
        cache: Arc<Cache>,
        state_files_encryption: bool,
        user: String,
    }

    impl UpdaterTest {
        fn new() -> Self {
            // Remember files_encryption state
            let state_files_encryption = AppManager::is_enabled("files_encryption");
            // We want to test with the encryption app disabled
            AppManager::disable("files_encryption");

            let storage = Arc::new(Temporary::new());
            let text_data = "dummy file data\n".to_string();
            let server_root = std::env::var("SERVER_ROOT").unwrap_or_else(|_| ".".to_string());
            let mut img_data = Vec::new();
            File::open(Path::new(&server_root).join("core/img/logo.png"))
                .unwrap()
                .read_to_end(&mut img_data)
                .unwrap();

            storage.mkdir("folder").unwrap();
            storage.file_put_contents("foo.txt", text_data.as_bytes()).unwrap();
            storage.file_put_contents("foo.png", &img_data).unwrap();
            storage.file_put_contents("folder/bar.txt", text_data.as_bytes()).unwrap();
            storage.file_put_contents("folder/bar2.txt", text_data.as_bytes()).unwrap();

            let scanner = storage.get_scanner();
            scanner.scan("").unwrap();
            let cache = storage.get_cache();

            Filesystem::tear_down();
            let user = format!("user_{}", uuid::Uuid::new_v4().to_string().replace("-", ""));

            User::create_user(&user, "password").unwrap();
            User::set_user_id(&user);

            Filesystem::init(&user, &format!("/{}/files", &user));

            Filesystem::clear_mounts();
            Filesystem::mount(storage.clone(), &format!("/{}/files", &user));

            Hook::clear("OC_Filesystem");

            Hook::connect("OC_Filesystem", "post_write", "\\OC\\Files\\Cache\\Updater", "write_hook");
            Hook::connect("OC_Filesystem", "post_delete", "\\OC\\Files\\Cache\\Updater", "delete_hook");
            Hook::connect("OC_Filesystem", "post_rename", "\\OC\\Files\\Cache\\Updater", "rename_hook");
            Hook::connect("OC_Filesystem", "post_touch", "\\OC\\Files\\Cache\\Updater", "touch_hook");

            Self {
                storage,
                scanner,
                cache,
                state_files_encryption,
                user,
            }
        }

        fn tear_down(&self) {
            if self.cache.is_initialized() {
                self.cache.clear().unwrap();
            }
            
            assert!(User::delete_user(&self.user));
            Filesystem::tear_down();

            // Reset app files_encryption
            if self.state_files_encryption {
                AppManager::enable("files_encryption");
            }
        }
    }

    #[test]
    fn test_write() {
        let test = UpdaterTest::new();
        let text_size = "dummy file data\n".len() as i64;
        let server_root = std::env::var("SERVER_ROOT").unwrap_or_else(|_| ".".to_string());
        let image_size = std::fs::metadata(Path::new(&server_root).join("core/img/logo.png"))
            .unwrap()
            .len() as i64;

        test.cache.put("foo.txt", &[("mtime", 100)].iter().cloned().collect()).unwrap();
        let root_cached_data = test.cache.get("").unwrap();
        assert_eq!(3 * text_size + image_size, root_cached_data.get("size").unwrap().as_i64().unwrap());

        let foo_cached_data = test.cache.get("foo.txt").unwrap();
        Filesystem::file_put_contents("foo.txt", "asd".as_bytes()).unwrap();
        let cached_data = test.cache.get("foo.txt").unwrap();
        assert_eq!(3, cached_data.get("size").unwrap().as_i64().unwrap());
        assert_ne!(
            foo_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );
        
        let cached_data = test.cache.get("").unwrap();
        assert_eq!(
            2 * text_size + image_size + 3,
            cached_data.get("size").unwrap().as_i64().unwrap()
        );
        assert_ne!(
            root_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );
        let root_cached_data = cached_data;

        assert!(!test.cache.in_cache("bar.txt"));
        Filesystem::file_put_contents("bar.txt", "asd".as_bytes()).unwrap();
        assert!(test.cache.in_cache("bar.txt"));
        
        let cached_data = test.cache.get("bar.txt").unwrap();
        assert_eq!(3, cached_data.get("size").unwrap().as_i64().unwrap());
        let mtime = cached_data.get("mtime").unwrap().as_i64().unwrap();
        
        let cached_data = test.cache.get("").unwrap();
        assert_eq!(
            2 * text_size + image_size + 2 * 3,
            cached_data.get("size").unwrap().as_i64().unwrap()
        );
        assert_ne!(
            root_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );
        assert!(
            root_cached_data.get("mtime").unwrap().as_i64().unwrap() <= mtime
        );

        test.tear_down();
    }

    #[test]
    fn test_write_with_mount_points() {
        let test = UpdaterTest::new();
        let storage2 = Arc::new(Temporary::new());
        let cache2 = storage2.get_cache();
        
        Filesystem::mount(
            storage2.clone(),
            &format!("/{}/files/folder/substorage", &test.user)
        );
        
        let folder_cached_data = test.cache.get("folder").unwrap();
        let substorage_cached_data = cache2.get("").unwrap();
        
        Filesystem::file_put_contents("folder/substorage/foo.txt", "asd".as_bytes()).unwrap();
        assert!(cache2.in_cache("foo.txt"));
        
        let cached_data = cache2.get("foo.txt").unwrap();
        assert_eq!(3, cached_data.get("size").unwrap().as_i64().unwrap());
        let mtime = cached_data.get("mtime").unwrap().as_i64().unwrap();

        let cached_data = cache2.get("").unwrap();
        assert_ne!(
            substorage_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );
        assert_eq!(mtime, cached_data.get("mtime").unwrap().as_i64().unwrap());

        let cached_data = test.cache.get("folder").unwrap();
        assert_ne!(
            folder_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );
        assert_eq!(mtime, cached_data.get("mtime").unwrap().as_i64().unwrap());

        test.tear_down();
    }

    #[test]
    fn test_delete() {
        let test = UpdaterTest::new();
        let text_size = "dummy file data\n".len() as i64;
        let server_root = std::env::var("SERVER_ROOT").unwrap_or_else(|_| ".".to_string());
        let image_size = std::fs::metadata(Path::new(&server_root).join("core/img/logo.png"))
            .unwrap()
            .len() as i64;

        let root_cached_data = test.cache.get("").unwrap();
        assert_eq!(
            3 * text_size + image_size,
            root_cached_data.get("size").unwrap().as_i64().unwrap()
        );

        assert!(test.cache.in_cache("foo.txt"));
        Filesystem::unlink("foo.txt").unwrap();
        assert!(!test.cache.in_cache("foo.txt"));
        
        let cached_data = test.cache.get("").unwrap();
        assert_eq!(
            2 * text_size + image_size,
            cached_data.get("size").unwrap().as_i64().unwrap()
        );
        assert_ne!(
            root_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );
        assert!(
            root_cached_data.get("mtime").unwrap().as_i64().unwrap() <= cached_data.get("mtime").unwrap().as_i64().unwrap()
        );
        let root_cached_data = cached_data;

        Filesystem::mkdir("bar_folder").unwrap();
        assert!(test.cache.in_cache("bar_folder"));
        
        let cached_data = test.cache.get("").unwrap();
        assert_ne!(
            root_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );
        let root_cached_data = cached_data;
        
        Filesystem::rmdir("bar_folder").unwrap();
        assert!(!test.cache.in_cache("bar_folder"));
        
        let cached_data = test.cache.get("").unwrap();
        assert_ne!(
            root_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );
        assert!(
            root_cached_data.get("mtime").unwrap().as_i64().unwrap() <= cached_data.get("mtime").unwrap().as_i64().unwrap()
        );

        test.tear_down();
    }

    #[test]
    fn test_delete_with_mount_points() {
        let test = UpdaterTest::new();
        let storage2 = Arc::new(Temporary::new());
        let cache2 = storage2.get_cache();
        
        Filesystem::mount(
            storage2.clone(),
            &format!("/{}/files/folder/substorage", &test.user)
        );
        
        Filesystem::file_put_contents("folder/substorage/foo.txt", "asd".as_bytes()).unwrap();
        assert!(cache2.in_cache("foo.txt"));
        
        let folder_cached_data = test.cache.get("folder").unwrap();
        let substorage_cached_data = cache2.get("").unwrap();
        
        Filesystem::unlink("folder/substorage/foo.txt").unwrap();
        assert!(!cache2.in_cache("foo.txt"));

        let cached_data = cache2.get("").unwrap();
        assert_ne!(
            substorage_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );
        assert!(
            substorage_cached_data.get("mtime").unwrap().as_i64().unwrap() <= cached_data.get("mtime").unwrap().as_i64().unwrap()
        );

        let cached_data = test.cache.get("folder").unwrap();
        assert_ne!(
            folder_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );
        assert!(
            folder_cached_data.get("mtime").unwrap().as_i64().unwrap() <= cached_data.get("mtime").unwrap().as_i64().unwrap()
        );

        test.tear_down();
    }

    #[test]
    fn test_rename() {
        let test = UpdaterTest::new();
        let text_size = "dummy file data\n".len() as i64;
        let server_root = std::env::var("SERVER_ROOT").unwrap_or_else(|_| ".".to_string());
        let image_size = std::fs::metadata(Path::new(&server_root).join("core/img/logo.png"))
            .unwrap()
            .len() as i64;

        let root_cached_data = test.cache.get("").unwrap();
        assert_eq!(
            3 * text_size + image_size,
            root_cached_data.get("size").unwrap().as_i64().unwrap()
        );

        assert!(test.cache.in_cache("foo.txt"));
        let foo_cached_data = test.cache.get("foo.txt").unwrap();
        assert!(!test.cache.in_cache("bar.txt"));
        
        Filesystem::rename("foo.txt", "bar.txt").unwrap();
        assert!(!test.cache.in_cache("foo.txt"));
        assert!(test.cache.in_cache("bar.txt"));
        
        let cached_data = test.cache.get("bar.txt").unwrap();
        assert_eq!(
            foo_cached_data.get("fileid").unwrap().as_i64().unwrap(),
            cached_data.get("fileid").unwrap().as_i64().unwrap()
        );
        let mtime = cached_data.get("mtime").unwrap().as_i64().unwrap();
        
        let cached_data = test.cache.get("").unwrap();
        assert_eq!(
            3 * text_size + image_size,
            cached_data.get("size").unwrap().as_i64().unwrap()
        );
        assert_ne!(
            root_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );

        test.tear_down();
    }

    #[test]
    fn test_rename_with_mount_points() {
        let test = UpdaterTest::new();
        let storage2 = Arc::new(Temporary::new());
        let cache2 = storage2.get_cache();
        
        Filesystem::mount(
            storage2.clone(),
            &format!("/{}/files/folder/substorage", &test.user)
        );
        
        Filesystem::file_put_contents("folder/substorage/foo.txt", "asd".as_bytes()).unwrap();
        assert!(cache2.in_cache("foo.txt"));
        
        let folder_cached_data = test.cache.get("folder").unwrap();
        let substorage_cached_data = cache2.get("").unwrap();
        let foo_cached_data = cache2.get("foo.txt").unwrap();
        
        Filesystem::rename("folder/substorage/foo.txt", "folder/substorage/bar.txt").unwrap();
        assert!(!cache2.in_cache("foo.txt"));
        assert!(cache2.in_cache("bar.txt"));
        
        let cached_data = cache2.get("bar.txt").unwrap();
        assert_eq!(
            foo_cached_data.get("fileid").unwrap().as_i64().unwrap(),
            cached_data.get("fileid").unwrap().as_i64().unwrap()
        );
        let mtime = cached_data.get("mtime").unwrap().as_i64().unwrap();

        let cached_data = cache2.get("").unwrap();
        assert_ne!(
            substorage_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );
        // rename can cause mtime change - invalid assert
        // assert_eq!(mtime, cached_data.get("mtime").unwrap().as_i64().unwrap());

        let cached_data = test.cache.get("folder").unwrap();
        assert_ne!(
            folder_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );
        // rename can cause mtime change - invalid assert
        // assert_eq!(mtime, cached_data.get("mtime").unwrap().as_i64().unwrap());

        test.tear_down();
    }

    #[test]
    fn test_touch() {
        let test = UpdaterTest::new();
        
        let root_cached_data = test.cache.get("").unwrap();
        let foo_cached_data = test.cache.get("foo.txt").unwrap();
        
        Filesystem::touch("foo.txt", None).unwrap();
        let cached_data = test.cache.get("foo.txt").unwrap();
        assert_ne!(
            foo_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );
        assert!(
            foo_cached_data.get("mtime").unwrap().as_i64().unwrap() <= cached_data.get("mtime").unwrap().as_i64().unwrap()
        );

        let cached_data = test.cache.get("").unwrap();
        assert_ne!(
            root_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );
        assert!(
            root_cached_data.get("mtime").unwrap().as_i64().unwrap() <= cached_data.get("mtime").unwrap().as_i64().unwrap()
        );
        let root_cached_data = cached_data;

        let time = 1371006070;
        let bar_cached_data = test.cache.get("folder/bar.txt").unwrap();
        let folder_cached_data = test.cache.get("folder").unwrap();
        
        Filesystem::touch("folder/bar.txt", Some(time)).unwrap();
        let cached_data = test.cache.get("folder/bar.txt").unwrap();
        assert_ne!(
            bar_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );
        assert_eq!(time, cached_data.get("mtime").unwrap().as_i64().unwrap());

        let cached_data = test.cache.get("folder").unwrap();
        assert_ne!(
            folder_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );

        let cached_data = test.cache.get("").unwrap();
        assert_ne!(
            root_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );
        assert_eq!(time, cached_data.get("mtime").unwrap().as_i64().unwrap());

        test.tear_down();
    }

    #[test]
    fn test_touch_with_mount_points() {
        let test = UpdaterTest::new();
        let storage2 = Arc::new(Temporary::new());
        let cache2 = storage2.get_cache();
        
        Filesystem::mount(
            storage2.clone(),
            &format!("/{}/files/folder/substorage", &test.user)
        );
        
        Filesystem::file_put_contents("folder/substorage/foo.txt", "asd".as_bytes()).unwrap();
        assert!(cache2.in_cache("foo.txt"));
        
        let folder_cached_data = test.cache.get("folder").unwrap();
        let substorage_cached_data = cache2.get("").unwrap();
        let foo_cached_data = cache2.get("foo.txt").unwrap();
        
        let time = 1371006070;
        Filesystem::touch("folder/substorage/foo.txt", Some(time)).unwrap();
        
        let cached_data = cache2.get("foo.txt").unwrap();
        assert_ne!(
            foo_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );
        assert_eq!(time, cached_data.get("mtime").unwrap().as_i64().unwrap());

        let cached_data = cache2.get("").unwrap();
        assert_ne!(
            substorage_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );

        let cached_data = test.cache.get("folder").unwrap();
        assert_ne!(
            folder_cached_data.get("etag").unwrap().as_str().unwrap(),
            cached_data.get("etag").unwrap().as_str().unwrap()
        );
        assert_eq!(time, cached_data.get("mtime").unwrap().as_i64().unwrap());

        test.tear_down();
    }

    #[test]
    fn test_update_permissions_on_rescan_only_for_updated_file() {
        let test = UpdaterTest::new();
        let permissions_cache = test.storage.get_permissions_cache();
        let scanner = test.storage.get_scanner();
        scanner.scan("").unwrap();
        let cache = test.storage.get_cache();
        
        let logged_in_user = User::get_user();
        User::set_user_id(&test.user);
        
        Filesystem::get_directory_content("/").unwrap();
        let past = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - 600;
            
        cache.put("", &[("storage_mtime", past as i64)].iter().cloned().collect()).unwrap();

        assert_ne!(-1, permissions_cache.get(cache.get_id("foo.txt"), &test.user).unwrap());
        assert_ne!(-1, permissions_cache.get(cache.get_id("foo.png"), &test.user).unwrap());

        permissions_cache.set(cache.get_id("foo.png"), &test.user, 15).unwrap();
        Filesystem::file_put_contents("/foo.txt", "asd".as_bytes()).unwrap();

        assert_eq!(-1, permissions_cache.get(cache.get_id("foo.txt"), &test.user).unwrap());
        assert_eq!(15, permissions_cache.get(cache.get_id("foo.png"), &test.user).unwrap());

        Filesystem::get_directory_content("/").unwrap();

        assert_eq!(15, permissions_cache.get(cache.get_id("foo.png"), &test.user).unwrap());

        Filesystem::file_put_contents("/qwerty.txt", "asd".as_bytes()).unwrap();
        Filesystem::get_directory_content("/").unwrap();

        assert_eq!(15, permissions_cache.get(cache.get_id("foo.png"), &test.user).unwrap());

        User::set_user_id(&logged_in_user);
        test.tear_down();
    }
}