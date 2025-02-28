/*
 * ownCloud
 *
 * @author Robin Appelman
 * @copyright 2012 Robin Appelman icewind@owncloud.com
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU AFFERO GENERAL PUBLIC LICENSE for more details.
 *
 * You should have received a copy of the GNU Affero General Public
 * License along with this library.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

use std::path::PathBuf;
use tempfile::tempdir;

use crate::files::storage::{CommonTest as CommonTestStorage, Storage};
use crate::helpers::OcHelper;

/// Test module for common storage functionality
pub mod test_files_storage {
    use super::*;
    
    /// Test implementation for common storage operations
    pub struct CommonTest {
        /// Temporary directory path
        tmp_dir: PathBuf,
        /// Storage instance being tested
        instance: Option<CommonTestStorage>,
    }
    
    impl CommonTest {
        /// Creates a new CommonTest instance
        pub fn new() -> Self {
            Self {
                tmp_dir: PathBuf::new(),
                instance: None,
            }
        }
        
        /// Set up the test environment
        pub fn set_up(&mut self) -> Result<(), std::io::Error> {
            let temp_dir = tempdir()?;
            self.tmp_dir = temp_dir.path().to_path_buf();
            
            self.instance = Some(CommonTestStorage::new(
                &[("datadir", self.tmp_dir.to_str().unwrap())]
            ));
            
            Ok(())
        }
        
        /// Clean up the test environment
        pub fn tear_down(&mut self) -> Result<(), std::io::Error> {
            // The tempdir will be automatically cleaned up when it goes out of scope,
            // but we explicitly call remove_dir_all for compatibility with the PHP version
            if self.tmp_dir.exists() {
                OcHelper::rmdir_r(&self.tmp_dir)?;
            }
            
            self.instance = None;
            Ok(())
        }
    }
    
    impl Default for CommonTest {
        fn default() -> Self {
            Self::new()
        }
    }
}