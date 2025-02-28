/**
 * ownCloud - Core
 *
 * @author Morris Jobke
 * @copyright 2013 Morris Jobke morris.jobke@gmail.com
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

use mockall::predicate::*;
use mockall::*;
use serde::{Deserialize, Serialize};

// Mock interfaces for the original PHP dependencies
#[automock]
trait L10n {
    fn t(&self, text: &str) -> String;
}

#[automock]
trait FilesView {
    fn rename(&self, dir: &str, oldname: &str, newname: &str) -> bool;
    fn normalize_path(&self, path: &str) -> String;
}

// Response structure
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct RenameResponse {
    success: bool,
    data: RenameResponseData,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum RenameResponseData {
    Success {
        dir: String,
        file: String,
        newname: String,
    },
    Error {
        message: String,
    },
}

struct FilesApp {
    view: Box<dyn FilesView>,
    l10n: Box<dyn L10n>,
}

impl FilesApp {
    fn new(view: Box<dyn FilesView>, l10n: Box<dyn L10n>) -> Self {
        FilesApp { view, l10n }
    }

    /// Renames a file or folder
    fn rename(&self, dir: &str, oldname: &str, newname: &str) -> RenameResponse {
        // Check if trying to rename "Shared" at root level
        if dir == "/" && oldname == "Shared" {
            return RenameResponse {
                success: false,
                data: RenameResponseData::Error {
                    message: self.l10n.t("%s could not be renamed"),
                },
            };
        }

        // Check if trying to rename to "Shared" at root level
        if dir == "/" && newname == "Shared" {
            return RenameResponse {
                success: false,
                data: RenameResponseData::Error {
                    message: self.l10n.t("Invalid folder name. Usage of 'Shared' is reserved."),
                },
            };
        }

        // Normal rename
        self.view.rename(dir, oldname, newname);
        
        RenameResponse {
            success: true,
            data: RenameResponseData::Success {
                dir: dir.to_string(),
                file: oldname.to_string(),
                newname: newname.to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestContext {
        files: FilesApp,
    }

    impl TestContext {
        fn setup() -> Self {
            // Mock L10n
            let mut l10n_mock = MockL10n::new();
            l10n_mock.expect_t()
                .returning(|text| text.to_string());

            // Mock FilesView
            let mut view_mock = MockFilesView::new();
            view_mock.expect_normalize_path()
                .returning(|path| path.to_string());
            view_mock.expect_rename()
                .returning(|_, _, _| true);

            let files = FilesApp::new(
                Box::new(view_mock),
                Box::new(l10n_mock),
            );

            TestContext { files }
        }
    }

    #[test]
    /// Test rename of file/folder named "Shared"
    fn test_rename_shared_folder() {
        let context = TestContext::setup();
        let dir = "/";
        let oldname = "Shared";
        let newname = "new_name";

        let result = context.files.rename(dir, oldname, newname);
        let expected = RenameResponse {
            success: false,
            data: RenameResponseData::Error {
                message: "%s could not be renamed".to_string(),
            },
        };

        assert_eq!(expected, result);
    }

    #[test]
    /// Test rename of file/folder named "Shared" in subdirectory
    fn test_rename_shared_folder_in_subdirectory() {
        let context = TestContext::setup();
        let dir = "/test";
        let oldname = "Shared";
        let newname = "new_name";

        let result = context.files.rename(dir, oldname, newname);
        let expected = RenameResponse {
            success: true,
            data: RenameResponseData::Success {
                dir: dir.to_string(),
                file: oldname.to_string(),
                newname: newname.to_string(),
            },
        };

        assert_eq!(expected, result);
    }

    #[test]
    /// Test rename of file/folder to "Shared"
    fn test_rename_folder_to_shared() {
        let context = TestContext::setup();
        let dir = "/";
        let oldname = "oldname";
        let newname = "Shared";

        let result = context.files.rename(dir, oldname, newname);
        let expected = RenameResponse {
            success: false,
            data: RenameResponseData::Error {
                message: "Invalid folder name. Usage of 'Shared' is reserved.".to_string(),
            },
        };

        assert_eq!(expected, result);
    }

    #[test]
    /// Test rename of file/folder
    fn test_rename_folder() {
        let context = TestContext::setup();
        let dir = "/";
        let oldname = "oldname";
        let newname = "newname";

        let result = context.files.rename(dir, oldname, newname);
        let expected = RenameResponse {
            success: true,
            data: RenameResponseData::Success {
                dir: dir.to_string(),
                file: oldname.to_string(),
                newname: newname.to_string(),
            },
        };

        assert_eq!(expected, result);
    }
}