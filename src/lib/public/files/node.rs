//! ownCloud
//!
//! Originally by Robin Appelman
//! Copyright 2013 Robin Appelman icewind@owncloud.com
//!
//! This library is free software; you can redistribute it and/or
//! modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
//! License as published by the Free Software Foundation; either
//! version 3 of the License, or any later version.
//!
//! This library is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//!
//! You should have received a copy of the GNU Affero General Public
//! License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use async_trait::async_trait;

// Define permission constants
pub const PERMISSION_READ: i32 = 1;
pub const PERMISSION_UPDATE: i32 = 2;
pub const PERMISSION_CREATE: i32 = 4;
pub const PERMISSION_DELETE: i32 = 8;
pub const PERMISSION_SHARE: i32 = 16;

#[derive(Debug, thiserror::Error)]
pub enum NodeError {
    #[error("Not permitted: {0}")]
    NotPermitted(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Other error: {0}")]
    Other(String),
}

pub type NodeResult<T> = Result<T, NodeError>;

pub trait Storage {
    // Storage interface would be defined here
}

#[async_trait]
pub trait Node: Send + Sync {
    /// Move the file or folder to a new location
    ///
    /// # Arguments
    /// * `target_path` - the absolute target path
    ///
    /// # Returns
    /// The moved node
    ///
    /// # Errors
    /// `NodeError::NotPermitted` if the operation is not permitted
    async fn r#move<P: AsRef<Path> + Send>(&self, target_path: P) -> NodeResult<Box<dyn Node>>;

    /// Delete the file or folder
    ///
    /// # Errors
    /// `NodeError::NotPermitted` if the operation is not permitted
    async fn delete(&self) -> NodeResult<()>;

    /// Copy the file or folder to a new location
    ///
    /// # Arguments
    /// * `target_path` - the absolute target path
    ///
    /// # Returns
    /// The copied node
    ///
    /// # Errors
    /// `NodeError::NotPermitted` if the operation is not permitted
    async fn copy<P: AsRef<Path> + Send>(&self, target_path: P) -> NodeResult<Box<dyn Node>>;

    /// Change the modified date of the file or folder
    /// If `mtime` is None the current time will be used
    ///
    /// # Arguments
    /// * `mtime` - (optional) modified date as unix timestamp
    ///
    /// # Errors
    /// `NodeError::NotPermitted` if the operation is not permitted
    async fn touch(&self, mtime: Option<u64>) -> NodeResult<()>;

    /// Get the storage backend the file or folder is stored on
    ///
    /// # Returns
    /// The storage backend
    ///
    /// # Errors
    /// `NodeError::NotFound` if the node doesn't exist
    async fn get_storage(&self) -> NodeResult<Box<dyn Storage>>;

    /// Get the full path of the file or folder
    ///
    /// # Returns
    /// The full path
    fn get_path(&self) -> String;

    /// Get the path of the file or folder relative to the mountpoint of it's storage
    ///
    /// # Returns
    /// The internal path
    fn get_internal_path(&self) -> String;

    /// Get the internal file id for the file or folder
    ///
    /// # Returns
    /// The internal file id
    fn get_id(&self) -> i64;

    /// Get metadata of the file or folder
    /// The returned hashmap contains the following values:
    ///  - mtime
    ///  - size
    ///
    /// # Returns
    /// A hashmap with the metadata
    async fn stat(&self) -> NodeResult<HashMap<String, i64>>;

    /// Get the modified date of the file or folder as unix timestamp
    ///
    /// # Returns
    /// The modified date
    async fn get_mtime(&self) -> NodeResult<u64>;

    /// Get the size of the file or folder in bytes
    ///
    /// # Returns
    /// The size in bytes
    async fn get_size(&self) -> NodeResult<u64>;

    /// Get the Etag of the file or folder
    /// The Etag is a string id used to detect changes to a file or folder,
    /// every time the file or folder is changed the Etag will change
    ///
    /// # Returns
    /// The Etag
    async fn get_etag(&self) -> NodeResult<String>;

    /// Get the permissions of the file or folder as a combination of one or more of the following constants:
    ///  - PERMISSION_READ
    ///  - PERMISSION_UPDATE
    ///  - PERMISSION_CREATE
    ///  - PERMISSION_DELETE
    ///  - PERMISSION_SHARE
    ///
    /// # Returns
    /// The permissions
    fn get_permissions(&self) -> i32;

    /// Check if the file or folder is readable
    ///
    /// # Returns
    /// `true` if the file or folder is readable
    fn is_readable(&self) -> bool;

    /// Check if the file or folder is writable
    ///
    /// # Returns
    /// `true` if the file or folder is writable
    fn is_updateable(&self) -> bool;

    /// Check if the file or folder is deletable
    ///
    /// # Returns
    /// `true` if the file or folder is deletable
    fn is_deletable(&self) -> bool;

    /// Check if the file or folder is shareable
    ///
    /// # Returns
    /// `true` if the file or folder is shareable
    fn is_shareable(&self) -> bool;

    /// Get the parent folder of the file or folder
    ///
    /// # Returns
    /// The parent folder
    ///
    /// # Errors
    /// `NodeError::NotFound` if the parent folder doesn't exist
    async fn get_parent(&self) -> NodeResult<Box<dyn Folder>>;

    /// Get the filename of the file or folder
    ///
    /// # Returns
    /// The filename
    fn get_name(&self) -> String;
}

// Define the Folder trait that would inherit from Node
#[async_trait]
pub trait Folder: Node {
    // Additional folder methods would be defined here
}