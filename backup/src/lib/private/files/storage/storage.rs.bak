// Copyright (c) 2012 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use crate::files::cache::{Cache, Permissions, Scanner, Storage as StorageCache, Watcher};
use crate::files::storage_traits::Storage as PublicStorage;

/// Provide a common interface to all different storage options
///
/// All paths passed to the storage are relative to the storage and should NOT have a leading slash.
pub trait Storage: PublicStorage {
    /// Get a cache instance for the storage
    ///
    /// # Arguments
    /// * `path` - The path to get the cache for
    fn get_cache(&self, path: &str) -> Cache;

    /// Get a scanner instance for the storage
    ///
    /// # Arguments
    /// * `path` - The path to get the scanner for
    fn get_scanner(&self, path: &str) -> Scanner;

    /// Get the user id of the owner of a file or folder
    ///
    /// # Arguments
    /// * `path` - The path to get the owner for
    fn get_owner(&self, path: &str) -> String;

    /// Get a permissions cache instance for the cache
    ///
    /// # Arguments
    /// * `path` - The path to get the permissions cache for
    fn get_permissions_cache(&self, path: &str) -> Permissions;

    /// Get a watcher instance for the cache
    ///
    /// # Arguments
    /// * `path` - The path to get the watcher for
    fn get_watcher(&self, path: &str) -> Watcher;

    /// Get the storage cache
    fn get_storage_cache(&self) -> StorageCache;
}