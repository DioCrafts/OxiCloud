// Copyright notice:
// ownCloud
//
// @author Michael Gapczynski
// @copyright 2012 Michael Gapczynski mtgap@owncloud.com
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::path::Path;
use async_trait::async_trait;

/// Check the storage backends for updates and change the cache accordingly
pub struct SharedWatcher {
    storage: Box<dyn Storage>,
    inner: Watcher,
}

impl SharedWatcher {
    pub fn new(storage: Box<dyn Storage>) -> Self {
        Self {
            storage: storage.clone(),
            inner: Watcher::new(storage),
        }
    }
}

#[async_trait]
impl WatcherTrait for SharedWatcher {
    /// Check `path` for updates
    ///
    /// # Arguments
    ///
    /// * `path` - The path to check for updates
    ///
    /// # Returns
    ///
    /// `true` if the path was updated, `false` otherwise
    async fn check_update(&self, path: &str) -> Result<bool, StorageError> {
        if !path.is_empty() && self.inner.check_update(path).await? {
            // since check_update() has already updated the size of the subdirs,
            // only apply the update to the owner's parent dirs

            // find last parent before reaching the shared storage root,
            // which is the actual shared dir from the owner
            let base_dir = match path.find('/') {
                Some(sep_pos) if sep_pos > 0 => &path[0..sep_pos],
                _ => path,
            };

            // find the path relative to the data dir
            let file = self.storage.get_file(base_dir).await?;
            let view = View::new(&format!("/{}", file.file_owner));

            // find the owner's storage and path
            let (storage, internal_path) = view.resolve_path(&file.path).await?;

            // update the parent dirs' sizes in the owner's cache
            let parent_path = Path::new(&internal_path)
                .parent()
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_default();
            
            storage.get_cache().correct_folder_size(&parent_path).await?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Remove deleted files in `path` from the cache
    ///
    /// # Arguments
    ///
    /// * `path` - The path to clean
    async fn clean_folder(&self, path: &str) -> Result<(), StorageError> {
        if !path.is_empty() {
            self.inner.clean_folder(path).await?;
        }
        Ok(())
    }
}

// Type definitions to make the code compile
// In a real implementation, these would be imported from other modules

#[async_trait]
pub trait WatcherTrait {
    async fn check_update(&self, path: &str) -> Result<bool, StorageError>;
    async fn clean_folder(&self, path: &str) -> Result<(), StorageError>;
}

pub struct Watcher {
    storage: Box<dyn Storage>,
}

impl Watcher {
    pub fn new(storage: Box<dyn Storage>) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl WatcherTrait for Watcher {
    async fn check_update(&self, path: &str) -> Result<bool, StorageError> {
        // Actual implementation would go here
        Ok(false)
    }

    async fn clean_folder(&self, path: &str) -> Result<(), StorageError> {
        // Actual implementation would go here
        Ok(())
    }
}

#[async_trait]
pub trait Storage: Send + Sync {
    async fn get_file(&self, path: &str) -> Result<FileInfo, StorageError>;
    fn get_cache(&self) -> Box<dyn Cache>;
}

pub struct FileInfo {
    pub path: String,
    pub file_owner: String,
}

#[async_trait]
pub trait Cache: Send + Sync {
    async fn correct_folder_size(&self, path: &str) -> Result<(), StorageError>;
}

pub struct View {
    path: String,
}

impl View {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    async fn resolve_path(&self, path: &str) -> Result<(Box<dyn Storage>, String), StorageError> {
        // Actual implementation would go here
        Ok((Box::new(DummyStorage {}), path.to_string()))
    }
}

#[derive(Debug)]
pub enum StorageError {
    NotFound,
    PermissionDenied,
    Other(String),
}

// Dummy implementation for the Storage trait
struct DummyStorage;

#[async_trait]
impl Storage for DummyStorage {
    async fn get_file(&self, _path: &str) -> Result<FileInfo, StorageError> {
        Ok(FileInfo {
            path: String::new(),
            file_owner: String::new(),
        })
    }

    fn get_cache(&self) -> Box<dyn Cache> {
        Box::new(DummyCache {})
    }
}

// Dummy implementation for the Cache trait
struct DummyCache;

#[async_trait]
impl Cache for DummyCache {
    async fn correct_folder_size(&self, _path: &str) -> Result<(), StorageError> {
        Ok(())
    }
}