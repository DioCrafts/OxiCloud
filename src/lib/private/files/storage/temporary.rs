//! Copyright (c) 2012 Robin Appelman <icewind@owncloud.com>
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.

use crate::files::storage::local::Local;
use crate::helper::OcHelper;
use std::path::PathBuf;

/// Local storage backend in temporary folder for testing purpose
pub struct Temporary {
    local: Local,
    data_dir: PathBuf,
}

impl Temporary {
    pub fn new(_arguments: &[(&str, &str)]) -> Self {
        let data_dir = OcHelper::tmp_folder();
        let local = Local::new(&[("datadir", data_dir.to_str().unwrap())]);
        
        Self {
            local,
            data_dir,
        }
    }

    pub fn clean_up(&self) -> std::io::Result<()> {
        OcHelper::rmdir_r(&self.data_dir)
    }
}

impl Drop for Temporary {
    fn drop(&mut self) {
        // Ignore errors during cleanup in destructor
        let _ = self.clean_up();
    }
}

// Implement necessary storage traits by delegating to Local
impl std::ops::Deref for Temporary {
    type Target = Local;

    fn deref(&self) -> &Self::Target {
        &self.local
    }
}

impl std::ops::DerefMut for Temporary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.local
    }
}