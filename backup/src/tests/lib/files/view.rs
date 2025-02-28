// tests/files/view.rs
use std::sync::Arc;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;

use owncloud_lib::files::storage::{Storage, Temporary};
use owncloud_lib::files::filesystem::{Filesystem, Mount};
use owncloud_lib::files::view::View;
use owncloud_lib::files::cache::{Cache, PermissionsCache};
use owncloud_lib::user::{User, UserBackend, DummyBackend};
use owncloud_lib::hooks::Hook;
use owncloud_lib::server::SERVERROOT;

struct TemporaryNoTouch {
    inner: Temporary,
}

impl TemporaryNoTouch {
    fn new(params: &[(&str, &str)]) -> Self {
        Self {
            inner: Temporary::new(params),
        }
    }
}

impl Storage for TemporaryNoTouch {
    fn touch(&self, path: &str, mtime: Option<u64>) -> bool {
        false
    }

    // Forward all other methods to inner storage
    fn mkdir(&self, path: &str) -> bool {
        self.inner.mkdir(path)
    }

    fn file_put_contents(&self, path: &str, data: &[u8]) -> bool {
        self.inner.file_put_contents(path, data)
    }

    fn file_exists(&self, path: &str) -> bool {
        self.inner.file_exists(path)
    }

    fn rename(&self, source: &str, target: &str) -> bool {
        self.inner.rename(source, target)
    }

    fn is_dir(&self, path: &str) -> bool {
        self.inner.is_dir(path)
    }

    fn get_cache(&self) -> Arc<dyn Cache> {
        self.inner.get_cache()
    }

    fn get_scanner(&self) -> Arc<dyn Scanner> {
        self.inner.get_scanner()
    }

    fn get_permissions_cache(&self) -> Arc<dyn PermissionsCache> {
        self.inner.get_permissions_cache()
    }

    // Implement other Storage methods as needed
}

pub struct ViewTest {
    storages: Vec<Arc<dyn Storage>>,
    user: String,
}

impl ViewTest {
    fn new() -> Self {
        Self {
            storages: Vec::new(),
            user: String::new(),
        }
    }

    fn set_up(&mut self) {
        User::clear_backends();
        User::use_backend(Arc::new(DummyBackend::new()));

        // Login
        User::create_user("test", "test");
        self.user = User::get_user();
        User::set_user_id("test");

        Filesystem::clear_mounts();
    }

    fn tear_down(&mut self) {
        User::set_user_id(&self.user);
        for storage in &self.storages {
            let cache = storage.get_cache();
            let ids = cache.get_all();
            let permissions_cache = storage.get_permissions_cache();
            permissions_cache.remove_multiple(&ids, &User::get_user());
            cache.clear();
        }
    }

    fn get_test_storage(&mut self, scan: bool, class_type: StorageType) -> Arc<dyn Storage> {
        let storage: Arc<dyn Storage> = match class_type {
            StorageType::Temporary => Arc::new(Temporary::new(&[])),
            StorageType::TemporaryNoTouch => Arc::new(TemporaryNoTouch::new(&[])),
        };

        let text_data = b"dummy file data\n";
        let img_data = fs::read(format!("{}/core/img/logo.png", *SERVERROOT)).unwrap();
        
        storage.mkdir("folder").unwrap();
        storage.file_put_contents("foo.txt", text_data).unwrap();
        storage.file_put_contents("foo.png", &img_data).unwrap();
        storage.file_put_contents("folder/bar.txt", text_data).unwrap();

        if scan {
            let scanner = storage.get_scanner();
            scanner.scan("");
        }
        
        self.storages.push(storage.clone());
        storage
    }

    fn dummy_hook(&mut self, params: &[(&str, &str)]) {
        for (key, value) in params {
            if *key == "path" {
                self.hook_path = Some(value.to_string());
                break;
            }
        }
    }

    fn dummy_create_hook(&mut self, params: &[(&str, &str)]) {
        for (key, value) in params {
            if *key == "path" {
                self.create_hook_path = Some(value.to_string());
                break;
            }
        }
    }

    fn test_cache_api(&mut self) {
        let storage1 = self.get_test_storage(true, StorageType::Temporary);
        let storage2 = self.get_test_storage(true, StorageType::Temporary);
        let storage3 = self.get_test_storage(true, StorageType::Temporary);
        
        Filesystem::mount(storage1.clone(), &[], "/");
        Filesystem::mount(storage2.clone(), &[], "/substorage");
        Filesystem::mount(storage3.clone(), &[], "/folder/anotherstorage");
        
        let text_size = "dummy file data\n".len() as u64;
        let image_size = fs::metadata(format!("{}/core/img/logo.png", *SERVERROOT)).unwrap().len();
        let storage_size = text_size * 2 + image_size;

        let root_view = View::new("");

        let cached_data = root_view.get_file_info("/foo.txt").unwrap();
        assert_eq!(text_size, cached_data.size);
        assert_eq!("text/plain", cached_data.mimetype);
        assert_ne!(-1, cached_data.permissions);

        let cached_data = root_view.get_file_info("/").unwrap();
        assert_eq!(storage_size * 3, cached_data.size);
        assert_eq!("httpd/unix-directory", cached_data.mimetype);

        let cached_data = root_view.get_file_info("/folder").unwrap();
        assert_eq!(storage_size + text_size, cached_data.size);
        assert_eq!("httpd/unix-directory", cached_data.mimetype);

        let folder_data = root_view.get_directory_content("/").unwrap();
        // Expected entries: folder, foo.png, foo.txt, substorage
        assert_eq!(4, folder_data.len());
        assert_eq!("folder", folder_data[0].name);
        assert_eq!("foo.png", folder_data[1].name);
        assert_eq!("foo.txt", folder_data[2].name);
        assert_eq!("substorage", folder_data[3].name);

        assert_eq!(storage_size + text_size, folder_data[0].size);
        assert_eq!(image_size, folder_data[1].size);
        assert_eq!(text_size, folder_data[2].size);
        assert_eq!(storage_size, folder_data[3].size);

        let folder_data = root_view.get_directory_content("/substorage").unwrap();
        // Expected entries: folder, foo.png, foo.txt
        assert_eq!(3, folder_data.len());
        assert_eq!("folder", folder_data[0].name);
        assert_eq!("foo.png", folder_data[1].name);
        assert_eq!("foo.txt", folder_data[2].name);

        let folder_view = View::new("/folder");
        assert_eq!(
            root_view.get_file_info("/folder").unwrap(),
            folder_view.get_file_info("/").unwrap()
        );

        let cached_data = root_view.get_file_info("/foo.txt").unwrap();
        assert!(!cached_data.encrypted);
        let id = root_view.put_file_info("/foo.txt", &[("encrypted", "true")]).unwrap();
        let cached_data = root_view.get_file_info("/foo.txt").unwrap();
        assert!(cached_data.encrypted);
        assert_eq!(cached_data.fileid, id);

        assert!(root_view.get_file_info("/non/existing").is_none());
        assert!(root_view.get_directory_content("/non/existing").unwrap().is_empty());
    }

    fn test_get_path(&mut self) {
        let storage1 = self.get_test_storage(true, StorageType::Temporary);
        let storage2 = self.get_test_storage(true, StorageType::Temporary);
        let storage3 = self.get_test_storage(true, StorageType::Temporary);
        
        Filesystem::mount(storage1.clone(), &[], "/");
        Filesystem::mount(storage2.clone(), &[], "/substorage");
        Filesystem::mount(storage3.clone(), &[], "/folder/anotherstorage");

        let root_view = View::new("");

        let cached_data = root_view.get_file_info("/foo.txt").unwrap();
        let id1 = cached_data.fileid;
        assert_eq!("/foo.txt", root_view.get_path(id1).unwrap());

        let cached_data = root_view.get_file_info("/substorage/foo.txt").unwrap();
        let id2 = cached_data.fileid;
        assert_eq!("/substorage/foo.txt", root_view.get_path(id2).unwrap());

        let folder_view = View::new("/substorage");
        assert_eq!("/foo.txt", folder_view.get_path(id2).unwrap());
        assert!(folder_view.get_path(id1).is_none());
    }

    fn test_mount_point_overwrite(&mut self) {
        let storage1 = self.get_test_storage(false, StorageType::Temporary);
        let storage2 = self.get_test_storage(true, StorageType::Temporary);
        
        storage1.mkdir("substorage").unwrap();
        Filesystem::mount(storage1.clone(), &[], "/");
        Filesystem::mount(storage2.clone(), &[], "/substorage");

        let root_view = View::new("");
        let folder_content = root_view.get_directory_content("/").unwrap();
        assert_eq!(4, folder_content.len());
    }

    fn test_cache_incomplete_folder(&mut self) {
        let storage1 = self.get_test_storage(false, StorageType::Temporary);
        Filesystem::mount(storage1.clone(), &[], "/");
        let root_view = View::new("");

        let entries = root_view.get_directory_content("/").unwrap();
        assert_eq!(3, entries.len());

        // /folder will already be in the cache but not scanned
        let entries = root_view.get_directory_content("/folder").unwrap();
        assert_eq!(1, entries.len());
    }

    fn test_auto_scan(&mut self) {
        let storage1 = self.get_test_storage(false, StorageType::Temporary);
        let storage2 = self.get_test_storage(false, StorageType::Temporary);
        
        Filesystem::mount(storage1.clone(), &[], "/");
        Filesystem::mount(storage2.clone(), &[], "/substorage");
        let text_size = "dummy file data\n".len() as u64;

        let root_view = View::new("");

        let cached_data = root_view.get_file_info("/").unwrap();
        assert_eq!("httpd/unix-directory", cached_data.mimetype);
        assert_eq!(-1, cached_data.size);

        let folder_data = root_view.get_directory_content("/substorage/folder").unwrap();
        assert_eq!("text/plain", folder_data[0].mimetype);
        assert_eq!(text_size, folder_data[0].size);
    }

    fn test_search(&mut self) {
        let storage1 = self.get_test_storage(true, StorageType::Temporary);
        let storage2 = self.get_test_storage(true, StorageType::Temporary);
        let storage3 = self.get_test_storage(true, StorageType::Temporary);
        
        Filesystem::mount(storage1.clone(), &[], "/");
        Filesystem::mount(storage2.clone(), &[], "/substorage");
        Filesystem::mount(storage3.clone(), &[], "/folder/anotherstorage");

        let root_view = View::new("");

        let results = root_view.search("foo").unwrap();
        assert_eq!(6, results.len());
        
        let mut paths = Vec::new();
        for result in &results {
            assert_eq!(result.path, Filesystem::normalize_path(&result.path));
            paths.push(result.path.clone());
        }
        
        assert!(paths.contains(&"/foo.txt".to_string()));
        assert!(paths.contains(&"/foo.png".to_string()));
        assert!(paths.contains(&"/substorage/foo.txt".to_string()));
        assert!(paths.contains(&"/substorage/foo.png".to_string()));
        assert!(paths.contains(&"/folder/anotherstorage/foo.txt".to_string()));
        assert!(paths.contains(&"/folder/anotherstorage/foo.png".to_string()));

        let folder_view = View::new("/folder");
        let results = folder_view.search("bar").unwrap();
        assert_eq!(2, results.len());
        
        let mut paths = Vec::new();
        for result in &results {
            paths.push(result.path.clone());
        }
        
        assert!(paths.contains(&"/anotherstorage/folder/bar.txt".to_string()));
        assert!(paths.contains(&"/bar.txt".to_string()));

        let results = folder_view.search("foo").unwrap();
        assert_eq!(2, results.len());
        
        let mut paths = Vec::new();
        for result in &results {
            paths.push(result.path.clone());
        }
        
        assert!(paths.contains(&"/anotherstorage/foo.txt".to_string()));
        assert!(paths.contains(&"/anotherstorage/foo.png".to_string()));

        assert_eq!(6, root_view.search_by_mime("text").unwrap().len());
        assert_eq!(3, folder_view.search_by_mime("text").unwrap().len());
    }

    fn test_watcher(&mut self) {
        let storage1 = self.get_test_storage(true, StorageType::Temporary);
        Filesystem::mount(storage1.clone(), &[], "/");

        let root_view = View::new("");

        let cached_data = root_view.get_file_info("foo.txt").unwrap();
        assert_eq!(16, cached_data.size);

        root_view.put_file_info("foo.txt", &[("storage_mtime", "10")]).unwrap();
        storage1.file_put_contents("foo.txt", b"foo").unwrap();

        let cached_data = root_view.get_file_info("foo.txt").unwrap();
        assert_eq!(3, cached_data.size);
    }

    fn test_copy_between_storages(&mut self) {
        let storage1 = self.get_test_storage(true, StorageType::Temporary);
        let storage2 = self.get_test_storage(true, StorageType::Temporary);
        
        Filesystem::mount(storage1.clone(), &[], "/");
        Filesystem::mount(storage2.clone(), &[], "/substorage");

        let root_view = View::new("");
        root_view.mkdir("substorage/emptyfolder").unwrap();
        root_view.copy("substorage", "anotherfolder").unwrap();
        
        assert!(root_view.is_dir("/anotherfolder").unwrap());
        assert!(root_view.is_dir("/substorage").unwrap());
        assert!(root_view.is_dir("/anotherfolder/emptyfolder").unwrap());
        assert!(root_view.is_dir("/substorage/emptyfolder").unwrap());
        assert!(root_view.file_exists("/anotherfolder/foo.txt").unwrap());
        assert!(root_view.file_exists("/anotherfolder/foo.png").unwrap());
        assert!(root_view.file_exists("/anotherfolder/folder/bar.txt").unwrap());
        assert!(root_view.file_exists("/substorage/foo.txt").unwrap());
        assert!(root_view.file_exists("/substorage/foo.png").unwrap());
        assert!(root_view.file_exists("/substorage/folder/bar.txt").unwrap());
    }

    fn test_move_between_storages(&mut self) {
        let storage1 = self.get_test_storage(true, StorageType::Temporary);
        let storage2 = self.get_test_storage(true, StorageType::Temporary);
        
        Filesystem::mount(storage1.clone(), &[], "/");
        Filesystem::mount(storage2.clone(), &[], "/substorage");

        let root_view = View::new("");
        root_view.rename("foo.txt", "substorage/folder/foo.txt").unwrap();
        
        assert!(!root_view.file_exists("foo.txt").unwrap());
        assert!(root_view.file_exists("substorage/folder/foo.txt").unwrap());
        
        root_view.rename("substorage/folder", "anotherfolder").unwrap();
        
        assert!(!root_view.is_dir("substorage/folder").unwrap());
        assert!(root_view.file_exists("anotherfolder/foo.txt").unwrap());
        assert!(root_view.file_exists("anotherfolder/bar.txt").unwrap());
    }

    fn test_touch(&mut self) {
        let storage = self.get_test_storage(true, StorageType::TemporaryNoTouch);
        Filesystem::mount(storage.clone(), &[], "/");

        let root_view = View::new("");
        let old_cached_data = root_view.get_file_info("foo.txt").unwrap();

        root_view.touch("foo.txt", Some(500)).unwrap();

        let cached_data = root_view.get_file_info("foo.txt").unwrap();
        assert_eq!(500, cached_data.mtime);
        assert_eq!(old_cached_data.storage_mtime, cached_data.storage_mtime);

        // Make sure the watcher detects the change
        root_view.put_file_info("foo.txt", &[("storage_mtime", "1000")]).unwrap();
        root_view.file_put_contents("foo.txt", b"asd").unwrap();
        
        let cached_data = root_view.get_file_info("foo.txt").unwrap();
        assert!(cached_data.mtime >= old_cached_data.mtime);
        assert_eq!(cached_data.storage_mtime, cached_data.mtime);
    }

    fn test_view_hooks(&mut self) -> Result<(), String> {
        let storage1 = self.get_test_storage(true, StorageType::Temporary);
        let storage2 = self.get_test_storage(true, StorageType::Temporary);
        
        let default_root = Filesystem::get_root();
        Filesystem::mount(storage1.clone(), &[], "/");
        Filesystem::mount(storage2.clone(), &[], &format!("{}/substorage", default_root));
        
        Hook::connect("OC_Filesystem", "post_write", Box::new(|params| {
            self.dummy_hook(params);
        }));

        let root_view = View::new("");
        let sub_view = View::new(&format!("{}/substorage", default_root));
        self.hook_path = None;

        root_view.file_put_contents("/foo.txt", b"asd").unwrap();
        assert!(self.hook_path.is_none());

        sub_view.file_put_contents("/foo.txt", b"asd").unwrap();
        assert!(self.hook_path.is_some());
        assert_eq!("/substorage/foo.txt", self.hook_path.unwrap());
        
        Ok(())
    }

    fn test_search_not_outside_view(&mut self) {
        let storage1 = self.get_test_storage(true, StorageType::Temporary);
        Filesystem::mount(storage1.clone(), &[], "/");
        
        storage1.rename("folder", "foo").unwrap();
        let scanner = storage1.get_scanner();
        scanner.scan("");

        let view = View::new("/foo");

        let result = view.search(".txt").unwrap();
        assert_eq!(1, result.len());
    }

    fn test_view_hooks_if_root_starts_the_same(&mut self) -> Result<(), String> {
        let storage1 = self.get_test_storage(true, StorageType::Temporary);
        let storage2 = self.get_test_storage(true, StorageType::Temporary);
        
        let default_root = Filesystem::get_root();
        Filesystem::mount(storage1.clone(), &[], "/");
        Filesystem::mount(storage2.clone(), &[], &format!("{}_substorage", default_root));
        
        Hook::connect("OC_Filesystem", "post_write", Box::new(|params| {
            self.dummy_hook(params);
        }));

        let sub_view = View::new(&format!("{}_substorage", default_root));
        self.hook_path = None;

        sub_view.file_put_contents("/foo.txt", b"asd").unwrap();
        assert!(self.hook_path.is_none());
        
        Ok(())
    }

    fn test_edit_no_create_hook(&mut self) -> Result<(), String> {
        let storage1 = self.get_test_storage(true, StorageType::Temporary);
        let storage2 = self.get_test_storage(true, StorageType::Temporary);
        
        let default_root = Filesystem::get_root();
        Filesystem::mount(storage1.clone(), &[], "/");
        Filesystem::mount(storage2.clone(), &[], &default_root);
        
        Hook::connect("OC_Filesystem", "post_create", Box::new(|params| {
            self.dummy_create_hook(params);
        }));

        let view = View::new(&default_root);
        self.create_hook_path = None;

        view.file_put_contents("/asd.txt", b"foo").unwrap();
        assert_eq!("/asd.txt", self.create_hook_path.unwrap());
        self.create_hook_path = None;

        view.file_put_contents("/asd.txt", b"foo").unwrap();
        assert!(self.create_hook_path.is_none());
        
        Ok(())
    }

    fn test_resolve_path(&mut self) {
        let storage1 = self.get_test_storage(true, StorageType::Temporary);
        Filesystem::mount(storage1.clone(), &[], "/");

        let view = View::new("");

        let test_paths = [
            ("foo.txt", "foo.txt"),
            ("foo.txt", "/foo.txt"),
            ("folder", "folder"),
            ("folder", "/folder"),
            ("folder", "folder/"),
            ("folder", "/folder/"),
            ("folder/bar.txt", "folder/bar.txt"),
            ("folder/bar.txt", "/folder/bar.txt"),
            ("", ""),
            ("", "/"),
        ];

        for (expected, path_to_test) in &test_paths {
            let result = view.resolve_path(path_to_test).unwrap();
            assert_eq!(*expected, result.1);

            let exists = view.file_exists(path_to_test).unwrap();
            assert!(exists);

            let exists = view.file_exists(&result.1).unwrap();
            assert!(exists);
        }
    }
}

enum StorageType {
    Temporary,
    TemporaryNoTouch,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_api() {
        let mut view_test = ViewTest::new();
        view_test.set_up();
        view_test.test_cache_api();
        view_test.tear_down();
    }

    #[test]
    fn test_get_path() {
        let mut view_test = ViewTest::new();
        view_test.set_up();
        view_test.test_get_path();
        view_test.tear_down();
    }

    #[test]
    fn test_mount_point_overwrite() {
        let mut view_test = ViewTest::new();
        view_test.set_up();
        view_test.test_mount_point_overwrite();
        view_test.tear_down();
    }

    #[test]
    fn test_cache_incomplete_folder() {
        let mut view_test = ViewTest::new();
        view_test.set_up();
        view_test.test_cache_incomplete_folder();
        view_test.tear_down();
    }

    #[test]
    fn test_auto_scan() {
        let mut view_test = ViewTest::new();
        view_test.set_up();
        view_test.test_auto_scan();
        view_test.tear_down();
    }

    #[test]
    fn test_search() {
        let mut view_test = ViewTest::new();
        view_test.set_up();
        view_test.test_search();
        view_test.tear_down();
    }

    #[test]
    fn test_watcher() {
        let mut view_test = ViewTest::new();
        view_test.set_up();
        view_test.test_watcher();
        view_test.tear_down();
    }

    #[test]
    fn test_copy_between_storages() {
        let mut view_test = ViewTest::new();
        view_test.set_up();
        view_test.test_copy_between_storages();
        view_test.tear_down();
    }

    #[test]
    fn test_move_between_storages() {
        let mut view_test = ViewTest::new();
        view_test.set_up();
        view_test.test_move_between_storages();
        view_test.tear_down();
    }

    #[test]
    fn test_touch() {
        let mut view_test = ViewTest::new();
        view_test.set_up();
        view_test.test_touch();
        view_test.tear_down();
    }

    #[test]
    fn test_view_hooks() {
        let mut view_test = ViewTest::new();
        view_test.set_up();
        view_test.test_view_hooks().unwrap();
        view_test.tear_down();
    }

    #[test]
    fn test_search_not_outside_view() {
        let mut view_test = ViewTest::new();
        view_test.set_up();
        view_test.test_search_not_outside_view();
        view_test.tear_down();
    }

    #[test]
    fn test_view_hooks_if_root_starts_the_same() {
        let mut view_test = ViewTest::new();
        view_test.set_up();
        view_test.test_view_hooks_if_root_starts_the_same().unwrap();
        view_test.tear_down();
    }

    #[test]
    fn test_edit_no_create_hook() {
        let mut view_test = ViewTest::new();
        view_test.set_up();
        view_test.test_edit_no_create_hook().unwrap();
        view_test.tear_down();
    }

    #[test]
    fn test_resolve_path() {
        let mut view_test = ViewTest::new();
        view_test.set_up();
        view_test.test_resolve_path();
        view_test.tear_down();
    }
}