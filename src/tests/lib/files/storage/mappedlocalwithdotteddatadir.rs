//! ownCloud
//!
//! @author Robin Appelman
//! @copyright 2012 Robin Appelman icewind@owncloud.com
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

use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, anyhow};
use async_trait::async_trait;

use crate::files::storage::{Storage, MappedLocal};
use crate::helper::OcHelper;

/// Storage implementation for testing MappedLocal with a dotted data directory
pub struct MappedLocalWithDottedDataDir {
    /// Temporary directory path
    tmp_dir: PathBuf,
    /// Storage instance
    instance: Option<MappedLocal>,
}

#[async_trait]
impl Storage for MappedLocalWithDottedDataDir {
    async fn set_up(&mut self) -> Result<()> {
        self.tmp_dir = OcHelper::tmp_folder().await?
            .join("dir.123")
            .with_trailing_slash();
        
        fs::create_dir_all(&self.tmp_dir)
            .map_err(|e| anyhow!("Failed to create temp directory: {}", e))?;
        
        self.instance = Some(MappedLocal::new(
            serde_json::json!({ "datadir": self.tmp_dir.to_string_lossy() })
        )?);
        
        Ok(())
    }

    async fn tear_down(&mut self) -> Result<()> {
        OcHelper::rmdir_r(&self.tmp_dir).await?;
        self.instance = None;
        Ok(())
    }
}

impl MappedLocalWithDottedDataDir {
    /// Create a new instance of MappedLocalWithDottedDataDir
    pub fn new() -> Self {
        Self {
            tmp_dir: PathBuf::new(),
            instance: None,
        }
    }
    
    /// Get the instance reference
    pub fn instance(&self) -> Result<&MappedLocal> {
        self.instance.as_ref().ok_or_else(|| anyhow!("Storage instance not initialized"))
    }
    
    /// Get the mutable instance reference
    pub fn instance_mut(&mut self) -> Result<&mut MappedLocal> {
        self.instance.as_mut().ok_or_else(|| anyhow!("Storage instance not initialized"))
    }
}

/// Extension trait for PathBuf to add a trailing slash
trait PathBufExt {
    /// Add a trailing slash to the path if it doesn't have one
    fn with_trailing_slash(self) -> Self;
}

impl PathBufExt for PathBuf {
    fn with_trailing_slash(mut self) -> Self {
        let path_str = self.to_string_lossy();
        if !path_str.ends_with('/') && !path_str.ends_with('\\') {
            self.push("");  // This adds a trailing slash
        }
        self
    }
}