use std::collections::HashMap;
use std::io::{Read, Write, Seek, SeekFrom, Result, Error, ErrorKind};
use std::time::SystemTime;
use url::Url;

/// Copy of http://dk1.php.net/manual/en/stream.streamwrapper.example-1.php
/// Used to simulate php://input for Request tests
pub struct RequestStream {
    position: usize,
    varname: String,
    globals: HashMap<String, String>,
}

impl RequestStream {
    pub fn new() -> Self {
        RequestStream {
            position: 0,
            varname: String::new(),
            globals: HashMap::new(),
        }
    }

    pub fn stream_open(&mut self, path: &str, _mode: &str, _options: u32) -> Result<bool> {
        let url = Url::parse(path).map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid URL"))?;
        self.varname = url.host_str().unwrap_or("").to_string();
        self.position = 0;

        Ok(true)
    }

    pub fn set_global(&mut self, key: String, value: String) {
        self.globals.insert(key, value);
    }

    pub fn get_global(&self, key: &str) -> Option<&String> {
        self.globals.get(key)
    }

    pub fn get_global_mut(&mut self, key: &str) -> Option<&mut String> {
        self.globals.get_mut(key)
    }

    pub fn stream_metadata(&mut self, path: &str, _option: u32) -> Result<bool> {
        let url = Url::parse(path).map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid URL"))?;
        let varname = url.host_str().unwrap_or("").to_string();
        
        if !self.globals.contains_key(&varname) {
            self.globals.insert(varname, String::new());
        }
        
        Ok(true)
    }
}

impl Read for RequestStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if let Some(content) = self.globals.get(&self.varname) {
            let count = buf.len();
            let content_bytes = content.as_bytes();
            
            if self.position >= content_bytes.len() {
                return Ok(0);
            }
            
            let bytes_to_read = std::cmp::min(count, content_bytes.len() - self.position);
            buf[..bytes_to_read].copy_from_slice(&content_bytes[self.position..self.position + bytes_to_read]);
            self.position += bytes_to_read;
            
            Ok(bytes_to_read)
        } else {
            Err(Error::new(ErrorKind::NotFound, "Variable not found"))
        }
    }
}

impl Write for RequestStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if let Some(content) = self.globals.get_mut(&self.varname) {
            let data = String::from_utf8_lossy(buf).to_string();
            let left = content[..self.position].to_string();
            let right = if self.position + data.len() < content.len() {
                content[self.position + data.len()..].to_string()
            } else {
                String::new()
            };
            
            *content = left + &data + &right;
            self.position += data.len();
            
            Ok(data.len())
        } else {
            Err(Error::new(ErrorKind::NotFound, "Variable not found"))
        }
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Seek for RequestStream {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        if let Some(content) = self.globals.get(&self.varname) {
            let content_len = content.len();
            
            match pos {
                SeekFrom::Start(offset) => {
                    let offset = offset as usize;
                    if offset < content_len {
                        self.position = offset;
                        Ok(offset as u64)
                    } else {
                        Err(Error::new(ErrorKind::InvalidInput, "Seek offset out of bounds"))
                    }
                },
                SeekFrom::Current(offset) => {
                    if offset >= 0 {
                        let new_pos = self.position + offset as usize;
                        if new_pos <= content_len {
                            self.position = new_pos;
                            Ok(new_pos as u64)
                        } else {
                            Err(Error::new(ErrorKind::InvalidInput, "Seek offset out of bounds"))
                        }
                    } else {
                        let abs_offset = (-offset) as usize;
                        if self.position >= abs_offset {
                            self.position -= abs_offset;
                            Ok(self.position as u64)
                        } else {
                            Err(Error::new(ErrorKind::InvalidInput, "Seek offset out of bounds"))
                        }
                    }
                },
                SeekFrom::End(offset) => {
                    if offset <= 0 {
                        let abs_offset = (-offset) as usize;
                        if abs_offset <= content_len {
                            self.position = content_len - abs_offset;
                            Ok(self.position as u64)
                        } else {
                            Err(Error::new(ErrorKind::InvalidInput, "Seek offset out of bounds"))
                        }
                    } else {
                        Err(Error::new(ErrorKind::InvalidInput, "Cannot seek beyond end of stream"))
                    }
                }
            }
        } else {
            Err(Error::new(ErrorKind::NotFound, "Variable not found"))
        }
    }
}

impl RequestStream {
    pub fn stream_stat(&self) -> Result<FileStat> {
        if let Some(content) = self.globals.get(&self.varname) {
            let size = content.len();
            let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            Ok(FileStat {
                dev: 0,
                ino: 0,
                mode: 0o777,
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
            Err(Error::new(ErrorKind::NotFound, "Variable not found"))
        }
    }
}

#[derive(Debug, Clone)]
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