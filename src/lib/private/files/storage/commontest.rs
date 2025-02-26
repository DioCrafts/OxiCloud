// Copyright 2012 Robin Appelman icewind@owncloud.com
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

//! Test implementation for Common storage with Local storage

use std::path::Path;
use std::fs::File;
use std::io::Result as IoResult;
use std::io::{Error, ErrorKind};
use std::time::SystemTime;
use async_trait::async_trait;

use crate::files::storage::common::Common;
use crate::files::storage::local::Local;
use crate::files::storage::Storage;

/// Test implementation for Common with Local
pub struct CommonTest {
    /// Underlying local storage used for missing functions
    storage: Local,
}

impl CommonTest {
    pub fn new(params: impl Into<std::collections::HashMap<String, String>>) -> Self {
        Self {
            storage: Local::new(params),
        }
    }
}

#[async_trait]
impl Storage for CommonTest {
    fn get_id(&self) -> String {
        format!("test::{}", self.storage.get_id())
    }

    async fn mkdir(&self, path: &Path) -> IoResult<()> {
        self.storage.mkdir(path).await
    }

    async fn rmdir(&self, path: &Path) -> IoResult<()> {
        self.storage.rmdir(path).await
    }

    async fn opendir(&self, path: &Path) -> IoResult<Box<dyn Iterator<Item = IoResult<String>>>> {
        self.storage.opendir(path).await
    }

    async fn stat(&self, path: &Path) -> IoResult<std::fs::Metadata> {
        self.storage.stat(path).await
    }

    async fn filetype(&self, path: &Path) -> IoResult<String> {
        self.storage.filetype(path).await
    }

    async fn is_readable(&self, path: &Path) -> IoResult<bool> {
        self.storage.is_readable(path).await
    }

    async fn is_updatable(&self, path: &Path) -> IoResult<bool> {
        self.storage.is_updatable(path).await
    }

    async fn file_exists(&self, path: &Path) -> IoResult<bool> {
        self.storage.file_exists(path).await
    }

    async fn unlink(&self, path: &Path) -> IoResult<()> {
        self.storage.unlink(path).await
    }

    async fn fopen(&self, path: &Path, mode: &str) -> IoResult<File> {
        self.storage.fopen(path, mode).await
    }

    async fn free_space(&self, path: &Path) -> IoResult<u64> {
        self.storage.free_space(path).await
    }

    async fn touch(&self, path: &Path, mtime: Option<SystemTime>) -> IoResult<()> {
        self.storage.touch(path, mtime).await
    }
}

impl Common for CommonTest {}