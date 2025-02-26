//! Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.

use crate::files::node::File;
use crate::files::NotFoundException;
use std::io::{Read, Write};
use std::path::Path;

/// Represents a file that doesn't exist in the filesystem
pub struct NonExistingFile {
    inner: File,
}

impl NonExistingFile {
    /// Create a new NonExistingFile
    pub fn new(file: File) -> Self {
        Self { inner: file }
    }

    /// Rename a file
    ///
    /// # Arguments
    /// * `new_path` - New path for the file
    ///
    /// # Errors
    /// Always throws NotFoundException as the file doesn't exist
    pub fn rename<P: AsRef<Path>>(&self, _new_path: P) -> Result<(), NotFoundException> {
        Err(NotFoundException::new())
    }

    /// Delete the file
    ///
    /// # Errors
    /// Always throws NotFoundException as the file doesn't exist
    pub fn delete(&self) -> Result<(), NotFoundException> {
        Err(NotFoundException::new())
    }

    /// Copy the file
    ///
    /// # Arguments
    /// * `new_path` - Destination path
    ///
    /// # Errors
    /// Always throws NotFoundException as the file doesn't exist
    pub fn copy<P: AsRef<Path>>(&self, _new_path: P) -> Result<(), NotFoundException> {
        Err(NotFoundException::new())
    }

    /// Update the modified time
    ///
    /// # Arguments
    /// * `mtime` - Optional new modified time
    ///
    /// # Errors
    /// Always throws NotFoundException as the file doesn't exist
    pub fn touch(&self, _mtime: Option<i64>) -> Result<(), NotFoundException> {
        Err(NotFoundException::new())
    }

    /// Get the file ID
    ///
    /// # Errors
    /// Always throws NotFoundException as the file doesn't exist
    pub fn get_id(&self) -> Result<String, NotFoundException> {
        Err(NotFoundException::new())
    }

    /// Get file statistics
    ///
    /// # Errors
    /// Always throws NotFoundException as the file doesn't exist
    pub fn stat(&self) -> Result<FileStat, NotFoundException> {
        Err(NotFoundException::new())
    }

    /// Get the file's modified time
    ///
    /// # Errors
    /// Always throws NotFoundException as the file doesn't exist
    pub fn get_mtime(&self) -> Result<i64, NotFoundException> {
        Err(NotFoundException::new())
    }

    /// Get the file size
    ///
    /// # Errors
    /// Always throws NotFoundException as the file doesn't exist
    pub fn get_size(&self) -> Result<u64, NotFoundException> {
        Err(NotFoundException::new())
    }

    /// Get the file's etag
    ///
    /// # Errors
    /// Always throws NotFoundException as the file doesn't exist
    pub fn get_etag(&self) -> Result<String, NotFoundException> {
        Err(NotFoundException::new())
    }

    /// Get the file permissions
    ///
    /// # Errors
    /// Always throws NotFoundException as the file doesn't exist
    pub fn get_permissions(&self) -> Result<u32, NotFoundException> {
        Err(NotFoundException::new())
    }

    /// Check if the file is readable
    ///
    /// # Errors
    /// Always throws NotFoundException as the file doesn't exist
    pub fn is_readable(&self) -> Result<bool, NotFoundException> {
        Err(NotFoundException::new())
    }

    /// Check if the file is updateable
    ///
    /// # Errors
    /// Always throws NotFoundException as the file doesn't exist
    pub fn is_updateable(&self) -> Result<bool, NotFoundException> {
        Err(NotFoundException::new())
    }

    /// Check if the file is deletable
    ///
    /// # Errors
    /// Always throws NotFoundException as the file doesn't exist
    pub fn is_deletable(&self) -> Result<bool, NotFoundException> {
        Err(NotFoundException::new())
    }

    /// Check if the file is shareable
    ///
    /// # Errors
    /// Always throws NotFoundException as the file doesn't exist
    pub fn is_shareable(&self) -> Result<bool, NotFoundException> {
        Err(NotFoundException::new())
    }

    /// Get the file content
    ///
    /// # Errors
    /// Always throws NotFoundException as the file doesn't exist
    pub fn get_content(&self) -> Result<Vec<u8>, NotFoundException> {
        Err(NotFoundException::new())
    }

    /// Write content to the file
    ///
    /// # Arguments
    /// * `data` - Data to write to the file
    ///
    /// # Errors
    /// Always throws NotFoundException as the file doesn't exist
    pub fn put_content<T: AsRef<[u8]>>(&self, _data: T) -> Result<(), NotFoundException> {
        Err(NotFoundException::new())
    }

    /// Get the file's mime type
    ///
    /// # Errors
    /// Always throws NotFoundException as the file doesn't exist
    pub fn get_mime_type(&self) -> Result<String, NotFoundException> {
        Err(NotFoundException::new())
    }

    /// Open the file with the specified mode
    ///
    /// # Arguments
    /// * `mode` - File open mode
    ///
    /// # Errors
    /// Always throws NotFoundException as the file doesn't exist
    pub fn fopen(&self, _mode: &str) -> Result<FileHandle, NotFoundException> {
        Err(NotFoundException::new())
    }
}

// These types would be defined elsewhere in the actual implementation
pub struct FileStat;
pub struct FileHandle;