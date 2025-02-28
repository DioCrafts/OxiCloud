use std::io::{Read, Write, Seek};
use std::io;
use std::cmp::{min, max};

const SPACE_NOT_COMPUTED: i64 = -1;

pub struct Quota<S> {
    storage: S,
    quota: i64,
}

impl<S: Storage> Quota<S> {
    /// Create a new quota wrapper
    ///
    /// # Arguments
    ///
    /// * `storage` - The underlying storage to wrap
    /// * `quota` - The quota limit in bytes
    pub fn new(storage: S, quota: i64) -> Self {
        Self {
            storage,
            quota,
        }
    }

    fn get_size(&self, path: &str) -> io::Result<i64> {
        let cache = self.get_cache()?;
        match cache.get(path)? {
            Some(data) => match data.get("size") {
                Some(size) => Ok(*size),
                None => Ok(SPACE_NOT_COMPUTED),
            },
            None => Ok(SPACE_NOT_COMPUTED),
        }
    }
}

impl<S: Storage> Storage for Quota<S> {
    fn free_space(&self, path: &str) -> io::Result<i64> {
        if self.quota < 0 {
            return self.storage.free_space(path);
        } else {
            let used = self.get_size("")?;
            if used < 0 {
                Ok(SPACE_NOT_COMPUTED)
            } else {
                let free = self.storage.free_space(path)?;
                Ok(min(free, max(self.quota - used, 0)))
            }
        }
    }

    fn file_put_contents(&self, path: &str, data: &[u8]) -> io::Result<bool> {
        let free = self.free_space("")?;
        if free < 0 || data.len() as i64 < free {
            self.storage.file_put_contents(path, data)
        } else {
            Ok(false)
        }
    }

    fn copy(&self, source: &str, target: &str) -> io::Result<bool> {
        let free = self.free_space("")?;
        let source_size = self.get_size(source)?;
        if free < 0 || source_size < free {
            self.storage.copy(source, target)
        } else {
            Ok(false)
        }
    }

    fn fopen(&self, path: &str, mode: &str) -> io::Result<Box<dyn FileHandle>> {
        let source = self.storage.fopen(path, mode)?;
        let free = self.free_space("")?;
        
        if free >= 0 && mode != "r" {
            Ok(Box::new(QuotaStream::new(source, free)))
        } else {
            Ok(source)
        }
    }

    // Pass-through methods to the wrapped storage
    fn get_cache(&self) -> io::Result<Box<dyn Cache>> {
        self.storage.get_cache()
    }
}

// QuotaStream implementation to limit write operations
pub struct QuotaStream {
    inner: Box<dyn FileHandle>,
    free: i64,
    written: i64,
}

impl QuotaStream {
    pub fn new(inner: Box<dyn FileHandle>, free: i64) -> Self {
        Self {
            inner,
            free,
            written: 0,
        }
    }
}

impl Read for QuotaStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl Write for QuotaStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let remaining = self.free - self.written;
        if remaining <= 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "Quota exceeded"));
        }
        
        let to_write = min(buf.len() as i64, remaining) as usize;
        let written = self.inner.write(&buf[..to_write])?;
        self.written += written as i64;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl Seek for QuotaStream {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.inner.seek(pos)
    }
}

impl FileHandle for QuotaStream {}

// Required interfaces/traits
pub trait FileHandle: Read + Write + Seek {}

pub trait Cache {
    fn get(&self, path: &str) -> io::Result<Option<std::collections::HashMap<String, i64>>>;
}

pub trait Storage {
    fn free_space(&self, path: &str) -> io::Result<i64>;
    fn file_put_contents(&self, path: &str, data: &[u8]) -> io::Result<bool>;
    fn copy(&self, source: &str, target: &str) -> io::Result<bool>;
    fn fopen(&self, path: &str, mode: &str) -> io::Result<Box<dyn FileHandle>>;
    fn get_cache(&self) -> io::Result<Box<dyn Cache>>;
}