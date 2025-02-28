// Copyright (c) 2012 Robin Appelman <icewind@owncloud.com>
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library. If not, see <http://www.gnu.org/licenses/>.

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub mod storage_tests {
    use super::*;

    /// Storage test trait to be implemented by specific storage backend tests
    pub trait StorageTest {
        /// Get storage instance to be tested
        fn get_instance(&self) -> Box<dyn Storage>;
    }

    /// Test implementations for storage backends
    pub struct Storage {
        instance: Box<dyn super::Storage>,
    }

    impl Storage {
        pub fn new<T: super::Storage + 'static>(instance: T) -> Self {
            Self {
                instance: Box::new(instance),
            }
        }

        /// The root folder of the storage should always exist, be readable and be recognized as a directory
        pub fn test_root(&self) {
            assert!(self.instance.file_exists("/"), "Root folder does not exist");
            assert!(self.instance.is_readable("/"), "Root folder is not readable");
            assert!(self.instance.is_dir("/"), "Root folder is not a directory");
            assert!(!self.instance.is_file("/"), "Root folder is a file");
            assert_eq!("dir", self.instance.filetype("/"));

            // Without this, any further testing would be useless, not an actual requirement for filestorage though
            assert!(self.instance.is_updatable("/"), "Root folder is not writable");
        }

        /// Test directory operations
        pub fn test_directories(&self, directory: &str) {
            let dir_path = format!("/{}", directory);
            assert!(!self.instance.file_exists(&dir_path));

            assert!(self.instance.mkdir(&dir_path));

            assert!(self.instance.file_exists(&dir_path));
            assert!(self.instance.is_dir(&dir_path));
            assert!(!self.instance.is_file(&dir_path));
            assert_eq!("dir", self.instance.filetype(&dir_path));
            assert_eq!(0, self.instance.filesize(&dir_path));
            assert!(self.instance.is_readable(&dir_path));
            assert!(self.instance.is_updatable(&dir_path));

            let content = self.instance.read_dir("/").unwrap();
            assert_eq!(vec![directory.to_string()], content);

            // Can't create existing folders
            assert!(!self.instance.mkdir(&dir_path));
            assert!(self.instance.rmdir(&dir_path));

            assert!(!self.instance.file_exists(&dir_path));

            // Can't remove non existing folders
            assert!(!self.instance.rmdir(&dir_path));

            let content = self.instance.read_dir("/").unwrap();
            assert_eq!(Vec::<String>::new(), content);
        }

        pub fn directory_provider() -> Vec<&'static str> {
            vec!["folder", " folder", "folder "]
        }

        /// Test the various uses of file_get_contents and file_put_contents
        pub fn test_get_put_contents(&self) {
            let server_root = std::env::var("SERVER_ROOT").expect("SERVER_ROOT not set");
            let source_file = format!("{}/tests/data/lorem.txt", server_root);
            let source_text = fs::read_to_string(&source_file).expect("Failed to read source file");

            // Fill a file with string data
            self.instance.file_put_contents("/lorem.txt", &source_text);
            assert!(!self.instance.is_dir("/lorem.txt"));
            assert_eq!(
                source_text,
                self.instance.file_get_contents("/lorem.txt").unwrap(),
                "data returned from file_get_contents is not equal to the source data"
            );

            // Empty the file
            self.instance.file_put_contents("/lorem.txt", "");
            assert_eq!(
                "",
                self.instance.file_get_contents("/lorem.txt").unwrap(),
                "file not emptied"
            );
        }

        /// Test various known mimetypes
        pub fn test_mime_type(&self) {
            let server_root = std::env::var("SERVER_ROOT").expect("SERVER_ROOT not set");

            assert_eq!("httpd/unix-directory", self.instance.get_mime_type("/"));
            assert_eq!("", self.instance.get_mime_type("/non/existing/file"));

            let text_file = format!("{}/tests/data/lorem.txt", server_root);
            let text_content = fs::read_to_string(&text_file).expect("Failed to read text file");
            self.instance.file_put_contents("/lorem.txt", &text_content);
            assert_eq!("text/plain", self.instance.get_mime_type("/lorem.txt"));

            let png_file = format!("{}/tests/data/logo-wide.png", server_root);
            let png_content = fs::read(&png_file).expect("Failed to read PNG file");
            self.instance
                .file_put_contents("/logo-wide.png", &String::from_utf8_lossy(&png_content));
            assert_eq!("image/png", self.instance.get_mime_type("/logo-wide.png"));

            let svg_file = format!("{}/tests/data/logo-wide.svg", server_root);
            let svg_content = fs::read_to_string(&svg_file).expect("Failed to read SVG file");
            self.instance.file_put_contents("/logo-wide.svg", &svg_content);
            assert_eq!("image/svg+xml", self.instance.get_mime_type("/logo-wide.svg"));
        }

        pub fn test_copy_and_move(&self) {
            let server_root = std::env::var("SERVER_ROOT").expect("SERVER_ROOT not set");
            let text_file = format!("{}/tests/data/lorem.txt", server_root);
            let text_content = fs::read_to_string(&text_file).expect("Failed to read text file");

            self.instance.file_put_contents("/source.txt", &text_content);
            self.instance.copy("/source.txt", "/target.txt");
            assert!(self.instance.file_exists("/target.txt"));
            assert_eq!(
                self.instance.file_get_contents("/source.txt").unwrap(),
                self.instance.file_get_contents("/target.txt").unwrap()
            );

            self.instance.rename("/source.txt", "/target2.txt");
            assert!(self.instance.file_exists("/target2.txt"));
            assert!(!self.instance.file_exists("/source.txt"));
            assert_eq!(
                text_content,
                self.instance.file_get_contents("/target.txt").unwrap()
            );
        }

        pub fn test_local(&self) {
            let server_root = std::env::var("SERVER_ROOT").expect("SERVER_ROOT not set");
            let text_file = format!("{}/tests/data/lorem.txt", server_root);
            let text_content = fs::read_to_string(&text_file).expect("Failed to read text file");

            self.instance.file_put_contents("/lorem.txt", &text_content);
            let local_file = self.instance.get_local_file("/lorem.txt");
            assert!(local_file.exists());
            assert_eq!(
                fs::read_to_string(&local_file).unwrap(),
                text_content
            );

            self.instance.mkdir("/folder");
            self.instance.file_put_contents("/folder/lorem.txt", &text_content);
            self.instance.file_put_contents("/folder/bar.txt", "asd");
            self.instance.mkdir("/folder/recursive");
            self.instance.file_put_contents("/folder/recursive/file.txt", "foo");
            let local_folder = self.instance.get_local_folder("/folder");

            assert!(local_folder.is_dir());

            // test below require to use instance->get_local_file because the physical storage might be different
            let local_file = self.instance.get_local_file("/folder/lorem.txt");
            assert!(local_file.exists());
            assert_eq!(
                fs::read_to_string(&local_file).unwrap(),
                text_content
            );

            let local_file = self.instance.get_local_file("/folder/bar.txt");
            assert!(local_file.exists());
            assert_eq!(fs::read_to_string(&local_file).unwrap(), "asd");

            let local_file = self.instance.get_local_file("/folder/recursive/file.txt");
            assert!(local_file.exists());
            assert_eq!(fs::read_to_string(&local_file).unwrap(), "foo");
        }

        pub fn test_stat(&self) {
            let server_root = std::env::var("SERVER_ROOT").expect("SERVER_ROOT not set");
            let text_file = format!("{}/tests/data/lorem.txt", server_root);
            let text_content = fs::read_to_string(&text_file).expect("Failed to read text file");
            let text_size = text_content.len() as u64;

            let ctime_start = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            self.instance.file_put_contents("/lorem.txt", &text_content);
            assert!(self.instance.is_readable("/lorem.txt"));
            let ctime_end = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let mtime = self.instance.filemtime("/lorem.txt");
            
            assert!(self.instance.has_updated("/lorem.txt", ctime_start - 5));
            assert!(self.instance.has_updated("/", ctime_start - 5));

            assert!(ctime_start - 5 <= mtime);
            assert!(mtime <= ctime_end + 1);
            assert_eq!(text_size, self.instance.filesize("/lorem.txt") as u64);

            let stat = self.instance.stat("/lorem.txt");
            // Only size and mtime are required in the result
            assert_eq!(stat.size as u64, self.instance.filesize("/lorem.txt") as u64);
            assert_eq!(stat.mtime, mtime);

            if self.instance.touch("/lorem.txt", 100) {
                let mtime = self.instance.filemtime("/lorem.txt");
                assert_eq!(mtime, 100);
            }

            let mtime_start = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            self.instance.unlink("/lorem.txt");
            assert!(self.instance.has_updated("/", mtime_start - 5));
        }

        pub fn test_fopen(&self) {
            let server_root = std::env::var("SERVER_ROOT").expect("SERVER_ROOT not set");
            let text_file = format!("{}/tests/data/lorem.txt", server_root);
            let text_content = fs::read_to_string(&text_file).expect("Failed to read text file");

            let result = self.instance.fopen("foo", "r");
            assert!(result.is_err());
            assert!(!self.instance.file_exists("foo"));

            let mut writer = self.instance.fopen("foo", "w").expect("Failed to open file for writing");
            writer.write_all(text_content.as_bytes()).expect("Failed to write to file");
            drop(writer); // Explicit close
            assert!(self.instance.file_exists("foo"));

            let mut reader = self.instance.fopen("foo", "r").expect("Failed to open file for reading");
            let mut content = String::new();
            reader.read_to_string(&mut content).expect("Failed to read from file");
            assert_eq!(text_content, content);
        }

        pub fn test_touch_create_file(&self) {
            assert!(!self.instance.file_exists("foo"));
            self.instance.touch("foo", None);
            assert!(self.instance.file_exists("foo"));
        }

        pub fn test_recursive_rmdir(&self) {
            self.instance.mkdir("folder");
            self.instance.mkdir("folder/bar");
            self.instance.file_put_contents("folder/asd.txt", "foobar");
            self.instance.file_put_contents("folder/bar/foo.txt", "asd");
            self.instance.rmdir("folder");
            assert!(!self.instance.file_exists("folder/asd.txt"));
            assert!(!self.instance.file_exists("folder/bar/foo.txt"));
            assert!(!self.instance.file_exists("folder/bar"));
            assert!(!self.instance.file_exists("folder"));
        }
    }
}

pub struct FileStat {
    pub size: i64,
    pub mtime: u64,
}

/// Storage trait defining the interface for storage backends
pub trait Storage {
    fn file_exists(&self, path: &str) -> bool;
    fn is_readable(&self, path: &str) -> bool;
    fn is_dir(&self, path: &str) -> bool;
    fn is_file(&self, path: &str) -> bool;
    fn filetype(&self, path: &str) -> &str;
    fn is_updatable(&self, path: &str) -> bool;
    fn mkdir(&self, path: &str) -> bool;
    fn rmdir(&self, path: &str) -> bool;
    fn read_dir(&self, path: &str) -> Result<Vec<String>, std::io::Error>;
    fn file_get_contents(&self, path: &str) -> Result<String, std::io::Error>;
    fn file_put_contents(&self, path: &str, data: &str) -> bool;
    fn get_mime_type(&self, path: &str) -> &str;
    fn copy(&self, source: &str, target: &str) -> bool;
    fn rename(&self, source: &str, target: &str) -> bool;
    fn get_local_file(&self, path: &str) -> PathBuf;
    fn get_local_folder(&self, path: &str) -> PathBuf;
    fn filemtime(&self, path: &str) -> u64;
    fn has_updated(&self, path: &str, time: u64) -> bool;
    fn filesize(&self, path: &str) -> i64;
    fn stat(&self, path: &str) -> FileStat;
    fn touch(&self, path: &str, mtime: Option<u64>) -> bool;
    fn unlink(&self, path: &str) -> bool;
    fn fopen(&self, path: &str, mode: &str) -> Result<Box<dyn std::io::Read + std::io::Write>, std::io::Error>;
}