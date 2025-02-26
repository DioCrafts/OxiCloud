// tests/lib/files/filesystem.rs

/**
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
 */

use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::fs;
use uuid::Uuid;
use regex::Regex;
use unicode_normalization::UnicodeNormalization;

use crate::oc_helper;
use crate::files::filesystem::{self, View};
use crate::user;
use crate::hook;

struct Filesystem {
    /// Temporary directories created during tests
    tmp_dirs: Vec<PathBuf>,
}

impl Filesystem {
    fn new() -> Self {
        Self {
            tmp_dirs: Vec::new()
        }
    }

    /// Get storage data with a temporary directory
    fn get_storage_data(&mut self) -> std::collections::HashMap<String, String> {
        let dir = oc_helper::tmp_folder();
        self.tmp_dirs.push(dir.clone());
        let mut data = std::collections::HashMap::new();
        data.insert("datadir".to_string(), dir.to_string_lossy().to_string());
        data
    }

    /// Clean up temporary directories
    fn tear_down(&self) {
        for dir in &self.tmp_dirs {
            let _ = oc_helper::rmdir_r(dir);
        }
    }

    /// Set up test environment
    fn set_up(&self) {
        filesystem::clear_mounts();
    }

    /// Test mount functionality
    fn test_mount(&mut self) {
        filesystem::mount(
            "OC\\Files\\Storage\\Local",
            self.get_storage_data(),
            "/".to_string(),
        );
        
        assert_eq!("/", filesystem::get_mount_point("/"));
        assert_eq!("/", filesystem::get_mount_point("/some/folder"));
        
        let (_, internal_path) = filesystem::resolve_path("/").unwrap();
        assert_eq!("", internal_path);
        
        let (_, internal_path) = filesystem::resolve_path("/some/folder").unwrap();
        assert_eq!("some/folder", internal_path);

        filesystem::mount(
            "OC\\Files\\Storage\\Local",
            self.get_storage_data(),
            "/some".to_string(),
        );
        
        assert_eq!("/", filesystem::get_mount_point("/"));
        assert_eq!("/some/", filesystem::get_mount_point("/some/folder"));
        assert_eq!("/some/", filesystem::get_mount_point("/some/"));
        assert_eq!("/some/", filesystem::get_mount_point("/some"));
        
        let (_, internal_path) = filesystem::resolve_path("/some/folder").unwrap();
        assert_eq!("folder", internal_path);
    }

    /// Test path normalization
    fn test_normalize(&self) {
        assert_eq!("/path", filesystem::normalize_path("/path/"));
        assert_eq!("/path/", filesystem::normalize_path("/path/", false));
        assert_eq!("/path", filesystem::normalize_path("path"));
        assert_eq!("/path", filesystem::normalize_path("\\path"));
        assert_eq!("/foo/bar", filesystem::normalize_path("/foo//bar/"));
        assert_eq!("/foo/bar", filesystem::normalize_path("/foo////bar"));
        
        // Unicode normalization test
        let normalized = filesystem::normalize_path("/foo/barü");
        assert_eq!("/foo/barü", normalized);
    }

    /// Test filesystem hooks
    fn test_hooks(&mut self) {
        let user;
        
        if filesystem::get_view().is_some() {
            user = user::get_user();
        } else {
            user = Uuid::new_v4().to_string();
            filesystem::init(&user, &format!("/{}/files", user));
        }
        
        hook::clear("OC_Filesystem");
        hook::connect("OC_Filesystem", "post_write", Box::new(|args| {
            let path = args.get("path").unwrap().to_string();
            assert_eq!(path, filesystem::normalize_path(&path));
        }));

        filesystem::mount(
            "OC\\Files\\Storage\\Temporary",
            std::collections::HashMap::new(),
            "/".to_string(),
        );

        let root_view = View::new("".to_string());
        root_view.mkdir(&format!("/{}", user)).unwrap();
        root_view.mkdir(&format!("/{}/files", user)).unwrap();

        filesystem::mkdir("/bar").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mount_functionality() {
        let mut fs = Filesystem::new();
        fs.set_up();
        fs.test_mount();
        fs.tear_down();
    }
    
    #[test]
    fn test_path_normalization() {
        let fs = Filesystem::new();
        fs.test_normalize();
    }
    
    #[test]
    fn test_filesystem_hooks() {
        let mut fs = Filesystem::new();
        fs.set_up();
        fs.test_hooks();
        fs.tear_down();
    }
}