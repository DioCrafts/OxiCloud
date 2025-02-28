// ownCloud
//
// @author Robin Appelman
// @copyright 2013 Robin Appelman icewind@owncloud.com
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

use std::io::{Read, Write};
use std::path::Path;
use thiserror::Error;
use async_trait::async_trait;

use crate::node::Node;

/// Error types for file operations
#[derive(Error, Debug)]
pub enum FileError {
    #[error("Operation not permitted")]
    NotPermitted,
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Hash error: {0}")]
    HashError(String),
    #[error("Other error: {0}")]
    Other(String),
}

pub type FileResult<T> = Result<T, FileError>;

/// File interface for public API
/// This should be used by apps instead of the internal ownCloud classes
#[async_trait]
pub trait File: Node {
    /// Get the content of the file as string
    ///
    /// # Errors
    /// 
    /// Returns `FileError::NotPermitted` if the operation is not allowed
    async fn get_content(&self) -> FileResult<String>;

    /// Write to the file from string data
    ///
    /// # Errors
    /// 
    /// Returns `FileError::NotPermitted` if the operation is not allowed
    async fn put_content(&self, data: &str) -> FileResult<()>;

    /// Get the mimetype of the file
    fn get_mime_type(&self) -> String;

    /// Open the file as stream, returning a reader or writer depending on mode
    ///
    /// # Errors
    /// 
    /// Returns `FileError::NotPermitted` if the operation is not allowed
    async fn open<P: AsRef<Path> + Send + Sync>(&self, mode: &str) -> FileResult<Box<dyn std::io::Read + Send + Sync>>;

    /// Compute the hash of the file
    /// Type of hash is set with `hash_type` and can be anything supported by standard hash functions
    ///
    /// # Errors
    /// 
    /// Returns `FileError::HashError` if the hash type is not supported
    async fn hash(&self, hash_type: &str, raw: bool) -> FileResult<String>;
}