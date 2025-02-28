// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, Once};
use std::time::Duration;
use std::ffi::OsStr;
use std::os::unix::fs::FileExt;
use std::os::unix::io::{AsRawFd, RawFd};

/// A stream wrapper for ownCloud's virtual filesystem
pub struct OcStream {
    path: String,
    file_source: Option<File>,
    dir_source: Option<std::fs::ReadDir>,
    meta: Option<std::fs::Metadata>,
    root_view: Arc<RootView>,
}

#[derive(Clone)]
struct RootView {
    base_path: PathBuf,
    // Adicional fields would be added based on OC\Files\View implementation
}

// Singleton pattern for root_view
lazy_static::lazy_static! {
    static ref ROOT_VIEW: Mutex<Option<Arc<RootView>>> = Mutex::new(None);
}

impl RootView {
    fn new() -> Self {
        RootView {
            base_path: PathBuf::from(""),
            // Initialize other fields as needed
        }
    }

    fn get_or_init() -> Arc<RootView> {
        let mut view = ROOT_VIEW.lock().unwrap();
        if view.is_none() {
            *view = Some(Arc::new(RootView::new()));
        }
        view.as_ref().unwrap().clone()
    }

    fn open_file(&self, path: &str, mode: &str) -> io::Result<File> {
        let path = Path::new(&self.base_path).join(path);
        let mut options = OpenOptions::new();
        
        if mode.contains('r') {
            options.read(true);
        }
        if mode.contains('w') {
            options.write(true).create(true);
        }
        if mode.contains('a') {
            options.append(true).create(true);
        }
        if mode.contains('+') {
            options.read(true).write(true);
        }
        
        options.open(path)
    }

    fn file_exists(&self, path: &str) -> bool {
        let path = Path::new(&self.base_path).join(path);
        path.exists()
    }

    fn stat(&self, path: &str) -> io::Result<std::fs::Metadata> {
        let path = Path::new(&self.base_path).join(path);
        path.metadata()
    }

    fn unlink(&self, path: &str) -> io::Result<()> {
        let path = Path::new(&self.base_path).join(path);
        std::fs::remove_file(path)
    }

    fn open_dir(&self, path: &str) -> io::Result<std::fs::ReadDir> {
        let path = Path::new(&self.base_path).join(path);
        std::fs::read_dir(path)
    }
}

impl OcStream {
    pub fn new() -> Self {
        OcStream {
            path: String::new(),
            file_source: None,
            dir_source: None,
            meta: None,
            root_view: RootView::get_or_init(),
        }
    }

    pub fn stream_open(&mut self, path: &str, mode: &str) -> io::Result<bool> {
        let path = &path[5..]; // Remove 'oc://'
        self.path = path.to_string();
        
        match self.root_view.open_file(path, mode) {
            Ok(file) => {
                self.meta = file.metadata().ok();
                self.file_source = Some(file);
                Ok(true)
            },
            Err(e) => Err(e),
        }
    }

    pub fn stream_seek(&mut self, offset: i64, whence: io::SeekFrom) -> io::Result<u64> {
        if let Some(file) = &mut self.file_source {
            file.seek(whence)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "No file source"))
        }
    }

    pub fn stream_tell(&mut self) -> io::Result<u64> {
        if let Some(file) = &mut self.file_source {
            file.seek(io::SeekFrom::Current(0))
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "No file source"))
        }
    }

    pub fn stream_read(&mut self, count: usize) -> io::Result<Vec<u8>> {
        if let Some(file) = &mut self.file_source {
            let mut buffer = vec![0; count];
            let bytes_read = file.read(&mut buffer)?;
            buffer.truncate(bytes_read);
            Ok(buffer)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "No file source"))
        }
    }

    pub fn stream_write(&mut self, data: &[u8]) -> io::Result<usize> {
        if let Some(file) = &mut self.file_source {
            file.write(data)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "No file source"))
        }
    }

    pub fn stream_set_blocking(&mut self, blocking: bool) -> io::Result<()> {
        if let Some(file) = &self.file_source {
            #[cfg(unix)]
            {
                use std::os::unix::io::AsRawFd;
                let fd = file.as_raw_fd();
                let flags = unsafe { libc::fcntl(fd, libc::F_GETFL) };
                let new_flags = if blocking {
                    flags & !libc::O_NONBLOCK
                } else {
                    flags | libc::O_NONBLOCK
                };
                if unsafe { libc::fcntl(fd, libc::F_SETFL, new_flags) } == -1 {
                    return Err(io::Error::last_os_error());
                }
                Ok(())
            }
            #[cfg(not(unix))]
            {
                Err(io::Error::new(io::ErrorKind::Other, "Platform not supported"))
            }
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "No file source"))
        }
    }

    pub fn stream_set_timeout(&mut self, seconds: u64, microseconds: u32) -> io::Result<()> {
        // This is platform specific and would need custom implementation
        // depending on the underlying file type
        Err(io::Error::new(io::ErrorKind::Other, "Not implemented"))
    }

    pub fn stream_set_write_buffer(&mut self, buffer_size: usize) -> io::Result<()> {
        // This is platform specific and would need custom implementation
        // depending on the underlying file type
        Err(io::Error::new(io::ErrorKind::Other, "Not implemented"))
    }

    pub fn stream_stat(&mut self) -> io::Result<std::fs::Metadata> {
        if let Some(file) = &self.file_source {
            file.metadata()
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "No file source"))
        }
    }

    pub fn stream_lock(&mut self, exclusive: bool) -> io::Result<()> {
        if let Some(file) = &self.file_source {
            #[cfg(unix)]
            {
                use std::os::unix::io::AsRawFd;
                let fd = file.as_raw_fd();
                let operation = if exclusive {
                    libc::LOCK_EX
                } else {
                    libc::LOCK_SH
                };
                if unsafe { libc::flock(fd, operation) } == -1 {
                    return Err(io::Error::last_os_error());
                }
                Ok(())
            }
            #[cfg(not(unix))]
            {
                Err(io::Error::new(io::ErrorKind::Other, "Platform not supported"))
            }
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "No file source"))
        }
    }

    pub fn stream_flush(&mut self) -> io::Result<()> {
        if let Some(file) = &mut self.file_source {
            file.flush()
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "No file source"))
        }
    }

    pub fn stream_eof(&mut self) -> io::Result<bool> {
        // In Rust, there's no direct equivalent to PHP's feof
        // We can check if read returns 0 bytes
        if let Some(file) = &mut self.file_source {
            let mut buf = [0; 1];
            match file.read(&mut buf) {
                Ok(0) => Ok(true),
                Ok(_) => {
                    // We need to seek back since we read a byte
                    file.seek(SeekFrom::Current(-1))?;
                    Ok(false)
                }
                Err(e) => Err(e),
            }
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "No file source"))
        }
    }

    pub fn url_stat(&self, path: &str) -> io::Result<std::fs::Metadata> {
        let path = &path[5..]; // Remove 'oc://'
        if self.root_view.file_exists(path) {
            self.root_view.stat(path)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
        }
    }

    pub fn stream_close(&mut self) -> io::Result<()> {
        self.file_source = None;
        Ok(())
    }

    pub fn unlink(&self, path: &str) -> io::Result<()> {
        let path = &path[5..]; // Remove 'oc://'
        self.root_view.unlink(path)
    }

    pub fn dir_open(&mut self, path: &str) -> io::Result<bool> {
        let path = &path[5..]; // Remove 'oc://'
        self.path = path.to_string();
        
        match self.root_view.open_dir(path) {
            Ok(dir) => {
                self.dir_source = Some(dir);
                Ok(true)
            },
            Err(e) => Err(e),
        }
    }

    pub fn dir_read(&mut self) -> Option<String> {
        if let Some(ref mut dir) = self.dir_source {
            match dir.next() {
                Some(Ok(entry)) => entry.file_name().to_str().map(String::from),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn dir_close(&mut self) -> io::Result<()> {
        self.dir_source = None;
        Ok(())
    }

    pub fn dir_rewind(&mut self) -> io::Result<()> {
        let path = self.path.clone();
        self.dir_source = Some(self.root_view.open_dir(&path)?);
        Ok(())
    }
}

impl Default for OcStream {
    fn default() -> Self {
        Self::new()
    }
}