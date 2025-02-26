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
 */

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use tokio::fs as tokio_fs;
use tokio::io::AsyncReadExt;
use tempfile::NamedTempFile;

use crate::oc::files::filesystem;
use crate::oc::files::stream::{Dir, Close};
use crate::oc::files::storage::Temporary;
use crate::oc::helper;

#[derive(Debug)]
struct StreamWrapperError(String);

impl std::fmt::Display for StreamWrapperError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for StreamWrapperError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_fake_dir() -> Result<(), Box<dyn std::error::Error>> {
        let items = vec!["foo".to_string(), "bar".to_string()];
        Dir::register("test", items.clone())?;
        
        let dir_contents = Dir::read("test")?;
        for file in &dir_contents {
            assert!(items.contains(file));
        }
        
        assert_eq!(items.len(), dir_contents.len());
        Ok(())
    }

    #[tokio::test]
    async fn test_close_stream() -> Result<(), Box<dyn std::error::Error>> {
        // Ensure all basic stream stuff works
        let source_file = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/lorem.txt");
        let tmp_file = helper::tmp_file(".txt")?;
        let file_path = format!("close://{}", tmp_file.to_string_lossy());
        
        assert!(Path::new(&tmp_file).exists());
        
        let source_content = fs::read_to_string(&source_file)?;
        fs::write(&tmp_file, &source_content)?;
        
        let file_content = fs::read_to_string(&tmp_file)?;
        assert_eq!(source_content, file_content);
        
        fs::remove_file(&tmp_file)?;
        assert!(!Path::new(&tmp_file).exists());

        // Test callback
        let tmp_file = helper::tmp_file(".txt")?;
        let file_path = format!("close://{}", tmp_file.to_string_lossy());
        
        Close::register_callback(&tmp_file, Box::new(close_callback))?;
        
        let mut file = File::create(&tmp_file)?;
        file.write_all(b"asd")?;
        
        match Close::close(&tmp_file) {
            Ok(_) => panic!("Expected error"),
            Err(e) => {
                assert_eq!(e.to_string(), tmp_file.to_string_lossy());
            }
        }
        
        Ok(())
    }

    fn close_callback(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        Err(Box::new(StreamWrapperError(path.to_string_lossy().to_string())))
    }

    #[tokio::test]
    async fn test_oc() -> Result<(), Box<dyn std::error::Error>> {
        filesystem::clear_mounts();
        
        let storage = Arc::new(Temporary::new()); 
        storage.file_put_contents("foo.txt", "asd")?;
        filesystem::mount(storage.clone(), "/");

        assert!(filesystem::file_exists("/foo.txt")?);
        assert_eq!("asd", filesystem::file_get_contents("/foo.txt")?);
        
        let dir_contents = filesystem::scan_dir("/")?;
        assert_eq!(vec![".".to_string(), "..".to_string(), "foo.txt".to_string()], dir_contents);

        filesystem::file_put_contents("/bar.txt", "qwerty")?;
        assert_eq!("qwerty", storage.file_get_contents("bar.txt")?);
        
        let dir_contents = filesystem::scan_dir("/")?;
        assert_eq!(
            vec![".".to_string(), "..".to_string(), "bar.txt".to_string(), "foo.txt".to_string()], 
            dir_contents
        );
        assert_eq!("qwerty", filesystem::file_get_contents("/bar.txt")?);

        filesystem::unlink("/foo.txt")?;
        let dir_contents = filesystem::scan_dir("/")?;
        assert_eq!(
            vec![".".to_string(), "..".to_string(), "bar.txt".to_string()], 
            dir_contents
        );
        
        Ok(())
    }
}

// Necessary trait definitions and module structure below

#[async_trait]
pub trait Storage {
    async fn file_put_contents(&self, path: &str, data: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn file_get_contents(&self, path: &str) -> Result<String, Box<dyn std::error::Error>>;
    async fn file_exists(&self, path: &str) -> Result<bool, Box<dyn std::error::Error>>;
    async fn unlink(&self, path: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn scan_dir(&self, path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}