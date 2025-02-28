use std::path::PathBuf;
use std::fs;
use crate::files::storage::Storage;
use crate::files::storage::local::Local as LocalStorage;
use crate::helpers::TempFolder;

/// Test implementation for local storage.
/// 
/// This library is free software; you can redistribute it and/or
/// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
/// License as published by the Free Software Foundation; either
/// version 3 of the License, or any later version.
///
/// This library is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
///
/// You should have received a copy of the GNU Affero General Public
/// License along with this library.  If not, see <http://www.gnu.org/licenses/>.
pub struct Local {
    /// Temporary directory path
    tmp_dir: PathBuf,
    /// Storage instance being tested
    instance: Option<LocalStorage>,
}

impl Storage for Local {
    fn set_up(&mut self) -> Result<(), std::io::Error> {
        self.tmp_dir = TempFolder::create()?;
        self.instance = Some(LocalStorage::new(self.tmp_dir.clone()));
        Ok(())
    }

    fn tear_down(&mut self) -> Result<(), std::io::Error> {
        if self.tmp_dir.exists() {
            fs::remove_dir_all(&self.tmp_dir)?;
        }
        self.instance = None;
        Ok(())
    }
}

impl Local {
    /// Create a new Local storage test instance
    pub fn new() -> Self {
        Local {
            tmp_dir: PathBuf::new(),
            instance: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_local_storage() -> Result<(), std::io::Error> {
        let mut local = Local::new();
        local.set_up()?;
        // Test operations would go here
        local.tear_down()?;
        Ok(())
    }
}