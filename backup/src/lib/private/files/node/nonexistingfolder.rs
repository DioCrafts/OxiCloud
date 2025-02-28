// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use crate::files::node::folder::Folder;
use crate::files::error::NotFoundException;
use crate::files::node::{Node, NodeTrait};
use async_trait::async_trait;
use std::path::Path;
use chrono::{DateTime, Utc};

/// Represents a folder in the file system that does not exist
pub struct NonExistingFolder {
    folder: Folder,
}

impl NonExistingFolder {
    /// Creates a new instance of a non-existing folder
    pub fn new(folder: Folder) -> Self {
        Self { folder }
    }
}

#[async_trait]
impl NodeTrait for NonExistingFolder {
    async fn rename<P: AsRef<Path> + Send>(&mut self, _new_path: P) -> Result<(), NotFoundException> {
        Err(NotFoundException)
    }

    async fn delete(&self) -> Result<(), NotFoundException> {
        Err(NotFoundException)
    }

    async fn copy<P: AsRef<Path> + Send>(&self, _new_path: P) -> Result<(), NotFoundException> {
        Err(NotFoundException)
    }

    async fn touch(&self, _mtime: Option<DateTime<Utc>>) -> Result<(), NotFoundException> {
        Err(NotFoundException)
    }

    async fn get_id(&self) -> Result<String, NotFoundException> {
        Err(NotFoundException)
    }

    async fn stat(&self) -> Result<std::fs::Metadata, NotFoundException> {
        Err(NotFoundException)
    }

    async fn get_mtime(&self) -> Result<DateTime<Utc>, NotFoundException> {
        Err(NotFoundException)
    }

    async fn get_size(&self) -> Result<u64, NotFoundException> {
        Err(NotFoundException)
    }

    async fn get_etag(&self) -> Result<String, NotFoundException> {
        Err(NotFoundException)
    }

    async fn get_permissions(&self) -> Result<u32, NotFoundException> {
        Err(NotFoundException)
    }

    async fn is_readable(&self) -> Result<bool, NotFoundException> {
        Err(NotFoundException)
    }

    async fn is_updateable(&self) -> Result<bool, NotFoundException> {
        Err(NotFoundException)
    }

    async fn is_deletable(&self) -> Result<bool, NotFoundException> {
        Err(NotFoundException)
    }

    async fn is_shareable(&self) -> Result<bool, NotFoundException> {
        Err(NotFoundException)
    }

    async fn is_creatable(&self) -> Result<bool, NotFoundException> {
        Err(NotFoundException)
    }
}

impl NonExistingFolder {
    pub async fn get<P: AsRef<Path> + Send>(&self, _path: P) -> Result<Box<dyn Node>, NotFoundException> {
        Err(NotFoundException)
    }

    pub async fn get_directory_listing(&self) -> Result<Vec<Box<dyn Node>>, NotFoundException> {
        Err(NotFoundException)
    }

    pub fn node_exists<P: AsRef<Path>>(&self, _path: P) -> bool {
        false
    }

    pub async fn new_folder<P: AsRef<Path> + Send>(&self, _path: P) -> Result<Box<dyn Node>, NotFoundException> {
        Err(NotFoundException)
    }

    pub async fn new_file<P: AsRef<Path> + Send>(&self, _path: P) -> Result<Box<dyn Node>, NotFoundException> {
        Err(NotFoundException)
    }

    pub async fn search(&self, _pattern: &str) -> Result<Vec<Box<dyn Node>>, NotFoundException> {
        Err(NotFoundException)
    }

    pub async fn search_by_mime(&self, _mime: &str) -> Result<Vec<Box<dyn Node>>, NotFoundException> {
        Err(NotFoundException)
    }

    pub async fn get_by_id(&self, _id: &str) -> Result<Vec<Box<dyn Node>>, NotFoundException> {
        Err(NotFoundException)
    }

    pub async fn get_free_space(&self) -> Result<u64, NotFoundException> {
        Err(NotFoundException)
    }
}