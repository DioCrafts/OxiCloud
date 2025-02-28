// ownCloud
//
// @author Robin Appelman
// @copyright 2012 Robin Appelman icewind@owncloud.com
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

use crate::files::storage::{MappedLocal as OCMappedLocal, Storage as StorageTrait};
use crate::helper::Helper;
use std::path::PathBuf;
use tempfile::TempDir;

/// Test implementation for the MappedLocal storage
pub struct MappedLocal {
    /// Temporary directory used for testing
    tmp_dir: PathBuf,
    /// Storage instance being tested
    instance: Option<OCMappedLocal>,
}

impl super::Storage for MappedLocal {
    fn set_up(&mut self) -> Result<(), std::io::Error> {
        self.tmp_dir = Helper::tmp_folder()?;
        self.instance = Some(OCMappedLocal::new(
            [("datadir".to_string(), self.tmp_dir.to_string_lossy().to_string())]
                .iter()
                .cloned()
                .collect(),
        ));
        Ok(())
    }

    fn tear_down(&mut self) -> Result<(), std::io::Error> {
        Helper::rmdir_r(&self.tmp_dir)?;
        self.instance = None;
        Ok(())
    }
}

impl Default for MappedLocal {
    fn default() -> Self {
        Self {
            tmp_dir: PathBuf::new(),
            instance: None,
        }
    }
}