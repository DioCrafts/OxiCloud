// ownCloud
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
//
// @author Robin Appelman
// @copyright 2012 Robin Appelman icewind@owncloud.com

use std::collections::HashMap;
use std::io::{Read, Write, Seek};
use std::path::Path;
use std::time::SystemTime;

/// A type alias for file and directory handles
pub type ResourceHandle = Box<dyn std::any::Any + Send>;

/// File stats information structure
pub struct FileStat {
    pub size: u64,
    pub mtime: SystemTime,
    // Additional fields can be added as needed
}

/// Provide a common interface to all different storage options
///
/// All paths passed to the storage are relative to the storage and should NOT have a leading slash.
#[async_trait::async_trait]
pub trait Storage: Send + Sync {
    /// Create a new storage instance with the given parameters
    ///
    /// $parameters is a free form array with the configuration options needed to construct the storage
    fn new(parameters: HashMap<String, String>) -> Self where Self: Sized;

    /// Get the identifier for the storage,
    /// the returned id should be the same for every storage object that is created with the same parameters
    /// and two storage objects with the same id should refer to two storages that display the same files.
    fn get_id(&self) -> String;

    /// Create a directory
    ///
    /// See http://php.net/manual/en/function.mkdir.php
    async fn mkdir(&self, path: &str) -> Result<(), std::io::Error>;

    /// Remove a directory
    ///
    /// See http://php.net/manual/en/function.rmdir.php
    async fn rmdir(&self, path: &str) -> Result<(), std::io::Error>;

    /// Open a directory for reading
    ///
    /// See http://php.net/manual/en/function.opendir.php
    async fn opendir(&self, path: &str) -> Result<ResourceHandle, std::io::Error>;

    /// Check if a path is a directory
    ///
    /// See http://php.net/manual/en/function.is_dir.php
    async fn is_dir(&self, path: &str) -> Result<bool, std::io::Error>;

    /// Check if a path is a file
    ///
    /// See http://php.net/manual/en/function.is_file.php
    async fn is_file(&self, path: &str) -> Result<bool, std::io::Error>;

    /// Get file or directory information
    ///
    /// See http://php.net/manual/en/function.stat.php
    /// only the following keys are required in the result: size and mtime
    async fn stat(&self, path: &str) -> Result<FileStat, std::io::Error>;

    /// Get the type of a file
    ///
    /// See http://php.net/manual/en/function.filetype.php
    async fn filetype(&self, path: &str) -> Result<String, std::io::Error>;

    /// Get the size of a file
    ///
    /// See http://php.net/manual/en/function.filesize.php
    /// The result for filesize when called on a folder is required to be 0
    async fn filesize(&self, path: &str) -> Result<u64, std::io::Error>;

    /// Check if a file can be created in the given path
    async fn is_creatable(&self, path: &str) -> Result<bool, std::io::Error>;

    /// Check if a file can be read
    async fn is_readable(&self, path: &str) -> Result<bool, std::io::Error>;

    /// Check if a file can be written to
    async fn is_updatable(&self, path: &str) -> Result<bool, std::io::Error>;

    /// Check if a file can be deleted
    async fn is_deletable(&self, path: &str) -> Result<bool, std::io::Error>;

    /// Check if a file can be shared
    async fn is_sharable(&self, path: &str) -> Result<bool, std::io::Error>;

    /// Get the full permissions of a path.
    /// Should return a combination of the PERMISSION_ constants defined in lib/public/constants.php
    async fn get_permissions(&self, path: &str) -> Result<i32, std::io::Error>;

    /// Check if a file or directory exists
    ///
    /// See http://php.net/manual/en/function.file_exists.php
    async fn file_exists(&self, path: &str) -> Result<bool, std::io::Error>;

    /// Get the modification time of a file
    ///
    /// See http://php.net/manual/en/function.filemtime.php
    async fn filemtime(&self, path: &str) -> Result<SystemTime, std::io::Error>;

    /// Get the contents of a file
    ///
    /// See http://php.net/manual/en/function.file_get_contents.php
    async fn file_get_contents(&self, path: &str) -> Result<Vec<u8>, std::io::Error>;

    /// Write data to a file
    ///
    /// See http://php.net/manual/en/function.file_put_contents.php
    async fn file_put_contents(&self, path: &str, data: &[u8]) -> Result<(), std::io::Error>;

    /// Delete a file
    ///
    /// See http://php.net/manual/en/function.unlink.php
    async fn unlink(&self, path: &str) -> Result<(), std::io::Error>;

    /// Rename a file or directory
    ///
    /// See http://php.net/manual/en/function.rename.php
    async fn rename(&self, path1: &str, path2: &str) -> Result<(), std::io::Error>;

    /// Copy a file
    ///
    /// See http://php.net/manual/en/function.copy.php
    async fn copy(&self, path1: &str, path2: &str) -> Result<(), std::io::Error>;

    /// Open a file for reading/writing
    ///
    /// See http://php.net/manual/en/function.fopen.php
    async fn fopen(&self, path: &str, mode: &str) -> Result<Box<dyn Any + Send>, std::io::Error>;

    /// Get the mimetype for a file or folder
    /// The mimetype for a folder is required to be "httpd/unix-directory"
    async fn get_mime_type(&self, path: &str) -> Result<String, std::io::Error>;

    /// Calculate a hash for a file
    ///
    /// See http://php.net/manual/en/function.hash-file.php
    async fn hash(&self, hash_type: &str, path: &str, raw: bool) -> Result<String, std::io::Error>;

    /// Get the amount of free space
    ///
    /// See http://php.net/manual/en/function.free_space.php
    async fn free_space(&self, path: &str) -> Result<u64, std::io::Error>;

    /// Search for occurrences of a query in file names
    async fn search(&self, query: &str) -> Result<Vec<String>, std::io::Error>;

    /// Update the access time of a file
    ///
    /// See http://php.net/manual/en/function.touch.php
    /// If the backend does not support the operation, an error should be returned
    async fn touch(&self, path: &str, mtime: Option<SystemTime>) -> Result<(), std::io::Error>;

    /// Get the path to a local version of the file.
    /// The local version of the file can be temporary and doesn't have to be persistent across requests
    async fn get_local_file(&self, path: &str) -> Result<String, std::io::Error>;

    /// Get the path to a local version of the folder.
    /// The local version of the folder can be temporary and doesn't have to be persistent across requests
    async fn get_local_folder(&self, path: &str) -> Result<String, std::io::Error>;

    /// Check if a file or folder has been updated since a given time
    ///
    /// hasUpdated for folders should return at least true if a file inside the folder is add, removed or renamed.
    /// returning true for other changes in the folder is optional
    async fn has_updated(&self, path: &str, time: SystemTime) -> Result<bool, std::io::Error>;

    /// Get the ETag for a file or folder
    async fn get_etag(&self, path: &str) -> Result<String, std::io::Error>;
}