/**
 * Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use crate::files::storage::wrapper::Wrapper as StorageWrapper;
use crate::files::storage::local::Local;
use crate::files::storage::Storage;
use crate::helpers::OcHelper;
use std::path::PathBuf;
use async_trait::async_trait;

/// Test wrapper for storage functionality
pub struct Wrapper {
    /// Temporary directory used for testing
    tmp_dir: PathBuf,
    /// The storage instance being tested
    instance: StorageWrapper,
}

#[async_trait]
impl crate::tests::files::storage::Storage for Wrapper {
    async fn set_up(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.tmp_dir = OcHelper::tmp_folder().await?;
        let storage = Local::new(serde_json::json!({
            "datadir": self.tmp_dir.to_string_lossy()
        }))?;
        
        self.instance = StorageWrapper::new(serde_json::json!({
            "storage": storage
        }))?;
        
        Ok(())
    }

    async fn tear_down(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        OcHelper::rmdir_r(&self.tmp_dir).await?;
        Ok(())
    }
    
    fn get_instance(&self) -> &dyn Storage {
        &self.instance
    }
}

impl Default for Wrapper {
    fn default() -> Self {
        Self {
            tmp_dir: PathBuf::new(),
            instance: StorageWrapper::default(),
        }
    }
}