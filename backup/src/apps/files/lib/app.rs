// Copyright (c) 2013 Morris Jobke <morris.jobke@gmail.com>
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

use std::path::Path;

// Dependency interfaces
pub trait L10n {
    fn t(&self, text: &str) -> String;
    fn t_with_args(&self, text: &str, args: &[&str]) -> String;
}

pub trait FileView {
    fn file_exists(&self, path: &str) -> bool;
    fn rename(&self, old_path: &str, new_path: &str) -> bool;
}

#[derive(Debug, Clone)]
pub struct RenameResult {
    pub success: bool,
    pub data: Option<RenameData>,
}

#[derive(Debug, Clone)]
pub enum RenameData {
    Success {
        dir: String,
        file: String,
        newname: String,
    },
    Error {
        message: String,
    },
}

pub struct App<V, L> {
    view: V,
    l10n: L,
}

impl<V: FileView, L: L10n> App<V, L> {
    pub fn new(view: V, l10n: L) -> Self {
        Self { view, l10n }
    }

    /// Rename a file
    ///
    /// # Arguments
    ///
    /// * `dir` - Directory path
    /// * `oldname` - Current filename
    /// * `newname` - New filename
    ///
    /// # Returns
    ///
    /// Result containing success status and associated data
    pub fn rename(&self, dir: &str, oldname: &str, newname: &str) -> RenameResult {
        // rename to "/Shared" is denied
        if dir == "/" && newname == "Shared" {
            return RenameResult {
                success: false,
                data: Some(RenameData::Error {
                    message: self.l10n.t("Invalid folder name. Usage of 'Shared' is reserved."),
                }),
            };
        }
        
        // rename to existing file is denied
        let combined_path = format!("{}/{}", dir, newname);
        if self.view.file_exists(&combined_path) {
            return RenameResult {
                success: false,
                data: Some(RenameData::Error {
                    message: self.l10n.t_with_args(
                        "The name %s is already used in the folder %s. Please choose a different name.",
                        &[newname, dir],
                    ),
                }),
            };
        }
        
        // Check conditions and attempt rename
        if newname != "." && !(dir == "/" && oldname == "Shared") {
            let old_path = format!("{}/{}", dir, oldname);
            let new_path = format!("{}/{}", dir, newname);
            
            if self.view.rename(&old_path, &new_path) {
                // successful rename
                return RenameResult {
                    success: true,
                    data: Some(RenameData::Success {
                        dir: dir.to_string(),
                        file: oldname.to_string(),
                        newname: newname.to_string(),
                    }),
                };
            }
        }
        
        // rename failed
        RenameResult {
            success: false,
            data: Some(RenameData::Error {
                message: self.l10n.t_with_args("%s could not be renamed", &[oldname]),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Test implementations would go here
}