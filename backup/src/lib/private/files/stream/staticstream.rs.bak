// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::collections::HashMap;
use std::io::{Read, Write, Seek, SeekFrom, Result, Error, ErrorKind};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::TryInto;
use once_cell::sync::Lazy;

const MODE_FILE: u32 = 0o100000;

/// A static in-memory stream implementation
pub struct StaticStream {
    context: Option<String>,
    path: String,
    pointer: usize,
    writable: bool,
}

static DATA: Lazy<Arc<Mutex<HashMap<String, Vec<u8>>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});

impl StaticStream {
    pub fn new() -> Self {
        StaticStream {
            context: None,
            path: String::new(),
            pointer: 0,
            writable: false,
        }
    }

    fn get_data(&self) -> MutexGuard<HashMap<String, Vec<u8>>> {
        DATA.lock().unwrap()
    }

    /// Clear all stored data
    pub fn clear() {
        let mut data = DATA.lock().unwrap();
        data.clear();
    }

    /// Open a stream with the given path and mode
    pub fn open(&mut self, path: &str, mode: &str) -> Result<String> {
        let mut data = self.get_data();
        
        match mode.chars().next() {
            Some('r') => {
                if !data.contains_key(path) {
                    return Err(Error::new(ErrorKind::NotFound, "Path not found"));
                }
                self.path = path.to_string();
                self.writable = mode.len() > 1 && mode.chars().nth(1) == Some('+');
            },
            Some('w') => {
                data.insert(path.to_string(), Vec::new());
                self.path = path.to_string();
                self.writable = true;
            },
            Some('a') => {
                if !data.contains_key(path) {
                    data.insert(path.to_string(), Vec::new());
                }
                self.path = path.to_string();
                self.writable = true;
                self.pointer = data.get(path).map_or(0, |v| v.len());
            },
            Some('x') => {
                if data.contains_key(path) {
                    return Err(Error::new(ErrorKind::AlreadyExists, "Path already exists"));
                }
                data.insert(path.to_string(), Vec::new());
                self.path = path.to_string();
                self.writable = true;
            },
            Some('c') => {
                if !data.contains_key(path) {
                    data.insert(path.to_string(), Vec::new());
                }
                self.path = path.to_string();
                self.writable = true;
            },
            _ => return Err(Error::new(ErrorKind::InvalidInput, "Invalid mode")),
        }
        
        Ok(self.path.clone())
    }

    /// Check if the stream is at the end of file
    pub fn eof(&self) -> bool {
        let data = self.get_data();
        if let Some(content) = data.get(&self.path) {
            self.pointer >= content.len()
        } else {
            true
        }
    }

    /// Unlink (remove) a path
    pub fn unlink(&self, path: &str) -> Result<()> {
        let mut data = self.get_data();
        data.remove(path);
        Ok(())
    }

    /// Get file statistics
    pub fn stat(&self, path: &str) -> Result<FileStat> {
        let data = self.get_data();
        
        if let Some(content) = data.get(path) {
            let size = content.len();
            let time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as u64;
            
            Ok(FileStat {
                dev: 0,
                ino: 0,
                mode: MODE_FILE | 0o777,
                nlink: 1,
                uid: 0,
                gid: 0,
                rdev: 0,
                size: size as u64,
                atime: time,
                mtime: time,
                ctime: time,
                blksize: -1i64 as u64,
                blocks: -1i64 as u64,
            })
        } else {
            Err(Error::new(ErrorKind::NotFound, "Path not found"))
        }
    }
}

impl Read for StaticStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let data = self.get_data();
        
        if let Some(content) = data.get(&self.path) {
            let bytes = std::cmp::min(content.len() - self.pointer, buf.len());
            if bytes > 0 {
                buf[..bytes].copy_from_slice(&content[self.pointer..self.pointer + bytes]);
                self.pointer += bytes;
            }
            Ok(bytes)
        } else {
            Err(Error::new(ErrorKind::NotFound, "Path not found"))
        }
    }
}

impl Write for StaticStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if !self.writable {
            return Ok(0);
        }
        
        let mut data = self.get_data();
        let size = buf.len();
        
        if let Some(content) = data.get_mut(&self.path) {
            if self.pointer >= content.len() {
                content.extend_from_slice(buf);
            } else {
                // Ensure the content is long enough
                if self.pointer + size > content.len() {
                    content.resize(self.pointer + size, 0);
                }
                
                // Replace at pointer position
                content[self.pointer..self.pointer + size].copy_from_slice(buf);
            }
            self.pointer += size;
            Ok(size)
        } else {
            Err(Error::new(ErrorKind::NotFound, "Path not found"))
        }
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Seek for StaticStream {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let data = self.get_data();
        
        if let Some(content) = data.get(&self.path) {
            let len = content.len();
            let new_pos = match pos {
                SeekFrom::Start(offset) => {
                    if offset <= len as u64 {
                        offset as usize
                    } else {
                        return Err(Error::new(ErrorKind::InvalidInput, "Invalid seek position"));
                    }
                },
                SeekFrom::Current(offset) => {
                    let new_offset = self.pointer as i64 + offset;
                    if new_offset >= 0 && new_offset <= len as i64 {
                        new_offset as usize
                    } else {
                        return Err(Error::new(ErrorKind::InvalidInput, "Invalid seek position"));
                    }
                },
                SeekFrom::End(offset) => {
                    let new_offset = len as i64 + offset;
                    if new_offset >= 0 && new_offset <= len as i64 {
                        new_offset as usize
                    } else {
                        return Err(Error::new(ErrorKind::InvalidInput, "Invalid seek position"));
                    }
                },
            };
            
            self.pointer = new_pos;
            Ok(new_pos as u64)
        } else {
            Err(Error::new(ErrorKind::NotFound, "Path not found"))
        }
    }
}

/// File statistics structure similar to the one used in PHP
pub struct FileStat {
    pub dev: u64,
    pub ino: u64,
    pub mode: u32,
    pub nlink: u64,
    pub uid: u32,
    pub gid: u32,
    pub rdev: u64,
    pub size: u64,
    pub atime: u64,
    pub mtime: u64,
    pub ctime: u64,
    pub blksize: u64,
    pub blocks: u64,
}

impl Default for StaticStream {
    fn default() -> Self {
        Self::new()
    }
}