// Copyright (c) 2012 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::fs::{self, File, OpenOptions, ReadDir};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// A common trait for storage implementations that wrap streams
pub trait Common {
    fn mkdir(&self, path: &str) -> io::Result<()>;
    fn rmdir(&self, path: &str) -> io::Result<()>;
    fn opendir(&self, path: &str) -> io::Result<ReadDir>;
    fn filetype(&self, path: &str) -> io::Result<String>;
    fn is_readable(&self, path: &str) -> bool;
    fn is_updatable(&self, path: &str) -> bool;
    fn file_exists(&self, path: &str) -> bool;
    fn unlink(&self, path: &str) -> io::Result<()>;
    fn fopen(&self, path: &str, mode: &str) -> io::Result<File>;
    fn touch(&self, path: &str, mtime: Option<SystemTime>) -> io::Result<()>;
    fn get_file(&self, path: &str, target: &str) -> io::Result<()>;
    fn upload_file(&self, path: &str, target: &str) -> io::Result<()>;
    fn rename(&self, path1: &str, path2: &str) -> io::Result<()>;
    fn stat(&self, path: &str) -> io::Result<fs::Metadata>;
    fn file_put_contents(&self, path: &str, contents: &[u8]) -> io::Result<()>;
    fn is_dir(&self, path: &str) -> bool;
}

/// Abstract stream wrapper implementation
pub trait StreamWrapper: Common {
    /// Constructs a URL for the given path
    fn construct_url(&self, path: &str) -> String;
}

/// Default implementation of Common for StreamWrapper types
impl<T: StreamWrapper> Common for T {
    fn mkdir(&self, path: &str) -> io::Result<()> {
        fs::create_dir(self.construct_url(path))
    }

    fn rmdir(&self, path: &str) -> io::Result<()> {
        if self.file_exists(path) {
            let dir = self.opendir(path)?;
            for entry in dir {
                let entry = entry?;
                let file_name = entry.file_name();
                let file_name = file_name.to_string_lossy();
                let sub_path = format!("{}/{}", path, file_name);
                
                if self.is_dir(&sub_path) {
                    self.rmdir(&sub_path)?;
                } else {
                    self.unlink(&sub_path)?;
                }
            }
            fs::remove_dir(self.construct_url(path))
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Directory not found"))
        }
    }

    fn opendir(&self, path: &str) -> io::Result<ReadDir> {
        fs::read_dir(self.construct_url(path))
    }

    fn filetype(&self, path: &str) -> io::Result<String> {
        let metadata = fs::metadata(self.construct_url(path))?;
        let file_type = if metadata.is_dir() {
            "dir"
        } else if metadata.is_file() {
            "file"
        } else {
            "unknown"
        };
        Ok(file_type.to_string())
    }

    fn is_readable(&self, _path: &str) -> bool {
        true // not properly supported
    }

    fn is_updatable(&self, _path: &str) -> bool {
        true // not properly supported
    }

    fn file_exists(&self, path: &str) -> bool {
        Path::new(&self.construct_url(path)).exists()
    }

    fn unlink(&self, path: &str) -> io::Result<()> {
        fs::remove_file(self.construct_url(path))
    }

    fn fopen(&self, path: &str, mode: &str) -> io::Result<File> {
        let mut options = OpenOptions::new();
        
        if mode.contains('r') {
            options.read(true);
        }
        if mode.contains('w') {
            options.write(true).truncate(true).create(true);
        }
        if mode.contains('a') {
            options.write(true).append(true).create(true);
        }
        if mode.contains('+') {
            options.read(true).write(true);
        }
        
        options.open(self.construct_url(path))
    }

    fn touch(&self, path: &str, mtime: Option<SystemTime>) -> io::Result<()> {
        if self.file_exists(path) {
            if mtime.is_none() {
                let mut file = self.fopen(path, "a")?;
                file.write_all(b"")?;
                Ok(())
            } else {
                // Not supported in this implementation
                Err(io::Error::new(io::ErrorKind::Unsupported, "Setting mtime is not supported"))
            }
        } else {
            self.file_put_contents(path, b"")
        }
    }

    fn get_file(&self, path: &str, target: &str) -> io::Result<()> {
        fs::copy(self.construct_url(path), target).map(|_| ())
    }

    fn upload_file(&self, path: &str, target: &str) -> io::Result<()> {
        fs::copy(path, self.construct_url(target)).map(|_| ())
    }

    fn rename(&self, path1: &str, path2: &str) -> io::Result<()> {
        fs::rename(self.construct_url(path1), self.construct_url(path2))
    }

    fn stat(&self, path: &str) -> io::Result<fs::Metadata> {
        fs::metadata(self.construct_url(path))
    }

    fn file_put_contents(&self, path: &str, contents: &[u8]) -> io::Result<()> {
        let mut file = self.fopen(path, "w")?;
        file.write_all(contents)
    }

    fn is_dir(&self, path: &str) -> bool {
        if let Ok(metadata) = fs::metadata(self.construct_url(path)) {
            metadata.is_dir()
        } else {
            false
        }
    }
}