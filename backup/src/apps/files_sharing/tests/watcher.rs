/*
 * ownCloud
 *
 * @author Vincent Petry
 * @copyright 2013 Vincent Petry <pvince81@owncloud.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU AFFERO GENERAL PUBLIC LICENSE for more details.
 *
 * You should have received a copy of the GNU Affero General Public
 * License along with this library.  If not, see <http://www.gnu.org/licenses/>.
 */

mod base;
use base::*;
use std::collections::HashMap;
use std::path::Path;

struct TestFilesSharingWatcher {
    view: FileView,
    owner_storage: Storage,
    owner_cache: Cache,
    shared_storage: Storage,
    shared_cache: Cache,
}

impl TestCase for TestFilesSharingWatcher {
    fn set_up(&mut self) -> Result<(), Error> {
        self.set_up_base()?;

        login_helper(TEST_FILES_SHARING_API_USER1)?;

        // prepare user1's dir structure
        let text_data = "dummy file data\n";
        self.view.mkdir("container")?;
        self.view.mkdir("container/shareddir")?;
        self.view.mkdir("container/shareddir/subdir")?;

        let (owner_storage, internal_path) = self.view.resolve_path("")?;
        self.owner_storage = owner_storage;
        self.owner_cache = self.owner_storage.get_cache()?;
        self.owner_storage.get_scanner().scan("")?;

        // share "shareddir" with user2
        let file_info = self.view.get_file_info("container/shareddir")?;
        Share::share_item(
            "folder",
            file_info.file_id,
            ShareType::User,
            TEST_FILES_SHARING_API_USER2,
            31,
        )?;

        // login as user2
        login_helper(TEST_FILES_SHARING_API_USER2)?;

        // retrieve the shared storage
        let second_view = FileView::new(&format!("/{}", TEST_FILES_SHARING_API_USER2));
        let (shared_storage, internal_path) = second_view.resolve_path("files/Shared/shareddir")?;
        self.shared_storage = shared_storage;
        self.shared_cache = self.shared_storage.get_cache()?;

        Ok(())
    }

    fn tear_down(&mut self) -> Result<(), Error> {
        self.shared_cache.clear()?;

        login_helper(TEST_FILES_SHARING_API_USER1)?;

        let file_info = self.view.get_file_info("container/shareddir")?;
        Share::unshare(
            "folder", 
            file_info.file_id, 
            ShareType::User,
            TEST_FILES_SHARING_API_USER2,
        )?;

        self.view.delete_all("container")?;

        self.owner_cache.clear()?;

        self.tear_down_base()
    }
}

impl TestFilesSharingWatcher {
    fn new() -> Self {
        Self {
            view: FileView::new(""),
            owner_storage: Storage::default(),
            owner_cache: Cache::default(),
            shared_storage: Storage::default(),
            shared_cache: Cache::default(),
        }
    }

    /// Returns the sizes of the path and its parent dirs in a hash
    /// where the key is the path and the value is the size.
    fn get_owner_dir_sizes(&self, path: &str) -> Result<HashMap<String, usize>, Error> {
        let mut result = HashMap::new();
        let mut current_path = path.to_string();

        while !current_path.is_empty() && current_path != "." {
            let cached_data = self.owner_cache.get(&current_path)?;
            result.insert(current_path.clone(), cached_data.size);
            current_path = Path::new(&current_path)
                .parent()
                .unwrap_or(Path::new(""))
                .to_string_lossy()
                .to_string();
        }

        let cached_data = self.owner_cache.get("")?;
        result.insert("".to_string(), cached_data.size);
        
        Ok(result)
    }

    /// Tests that writing a file using the shared storage will propagate the file
    /// size to the owner's parent folders.
    fn test_folder_size_propagation_to_owner_storage(&self) -> Result<(), Error> {
        let initial_sizes = self.get_owner_dir_sizes("files/container/shareddir")?;

        let text_data = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let data_len = text_data.len();
        
        self.shared_cache.put("shareddir/bar.txt", &CacheEntry {
            storage_mtime: 10,
            ..Default::default()
        })?;
        
        self.shared_storage.file_put_contents("shareddir/bar.txt", text_data)?;
        
        self.shared_cache.put("shareddir", &CacheEntry {
            storage_mtime: 10,
            ..Default::default()
        })?;

        // run the propagation code
        let result = self.shared_storage.get_watcher().check_update("shareddir")?;

        assert!(result);

        // the owner's parent dirs must have increase size
        let new_sizes = self.get_owner_dir_sizes("files/container/shareddir")?;
        assert_eq!(initial_sizes[""] + data_len, new_sizes[""]);
        assert_eq!(initial_sizes["files"] + data_len, new_sizes["files"]);
        assert_eq!(initial_sizes["files/container"] + data_len, new_sizes["files/container"]);
        assert_eq!(initial_sizes["files/container/shareddir"] + data_len, new_sizes["files/container/shareddir"]);

        // no more updates
        let result = self.shared_storage.get_watcher().check_update("shareddir")?;

        assert!(!result);
        
        Ok(())
    }

    /// Tests that writing a file using the shared storage will propagate the file
    /// size to the owner's parent folders.
    fn test_sub_folder_size_propagation_to_owner_storage(&self) -> Result<(), Error> {
        let initial_sizes = self.get_owner_dir_sizes("files/container/shareddir/subdir")?;

        let text_data = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let data_len = text_data.len();
        
        self.shared_cache.put("shareddir/subdir/bar.txt", &CacheEntry {
            storage_mtime: 10,
            ..Default::default()
        })?;
        
        self.shared_storage.file_put_contents("shareddir/subdir/bar.txt", text_data)?;
        
        self.shared_cache.put("shareddir/subdir", &CacheEntry {
            storage_mtime: 10,
            ..Default::default()
        })?;

        // run the propagation code
        let result = self.shared_storage.get_watcher().check_update("shareddir/subdir")?;

        assert!(result);

        // the owner's parent dirs must have increase size
        let new_sizes = self.get_owner_dir_sizes("files/container/shareddir/subdir")?;
        assert_eq!(initial_sizes[""] + data_len, new_sizes[""]);
        assert_eq!(initial_sizes["files"] + data_len, new_sizes["files"]);
        assert_eq!(initial_sizes["files/container"] + data_len, new_sizes["files/container"]);
        assert_eq!(initial_sizes["files/container/shareddir"] + data_len, new_sizes["files/container/shareddir"]);
        assert_eq!(initial_sizes["files/container/shareddir/subdir"] + data_len, new_sizes["files/container/shareddir/subdir"]);

        // no more updates
        let result = self.shared_storage.get_watcher().check_update("shareddir/subdir")?;

        assert!(!result);
        
        Ok(())
    }

    fn test_no_update_on_root(&self) -> Result<(), Error> {
        // no updates when called for root path
        let result = self.shared_storage.get_watcher().check_update("")?;

        assert!(!result);
        
        // FIXME: for some reason when running this "naked" test,
        // there will be remaining nonsensical entries in the
        // database with a path "test-share-user1/container/..."
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_folder_size_propagation_to_owner_storage() {
        let mut test = TestFilesSharingWatcher::new();
        test.set_up().unwrap();
        test.test_folder_size_propagation_to_owner_storage().unwrap();
        test.tear_down().unwrap();
    }

    #[test]
    fn test_sub_folder_size_propagation_to_owner_storage() {
        let mut test = TestFilesSharingWatcher::new();
        test.set_up().unwrap();
        test.test_sub_folder_size_propagation_to_owner_storage().unwrap();
        test.tear_down().unwrap();
    }

    #[test]
    fn test_no_update_on_root() {
        let mut test = TestFilesSharingWatcher::new();
        test.set_up().unwrap();
        test.test_no_update_on_root().unwrap();
        test.tear_down().unwrap();
    }
}