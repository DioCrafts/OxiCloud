// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

lazy_static::lazy_static! {
    static ref STREAMS: RwLock<HashMap<String, (Arc<Mutex<File>>, usize)>> = RwLock::new(HashMap::new());
}

/// Stream wrapper that limits the amount of data that can be written to a stream
///
/// Usage: `Quota::register(id, stream, limit)`
/// or:    `Quota::wrap(stream, limit)`
pub struct Quota {
    source: Arc<Mutex<File>>,
    limit: usize,
}

impl Quota {
    /// Register a stream with a quota limit
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the stream
    /// * `stream` - The file stream to wrap
    /// * `limit` - Maximum number of bytes that can be written
    pub fn register(id: &str, stream: File, limit: usize) {
        let mut streams = STREAMS.write().expect("Failed to lock streams for writing");
        streams.insert(id.to_string(), (Arc::new(Mutex::new(stream)), limit));
    }

    /// Remove all registered streams
    pub fn clear() {
        let mut streams = STREAMS.write().expect("Failed to lock streams for writing");
        streams.clear();
    }

    /// Wrap a stream with a quota limit
    ///
    /// # Arguments
    /// * `stream` - The file stream to wrap
    /// * `limit` - Maximum number of bytes that can be written
    ///
    /// # Returns
    /// A new Quota instance wrapping the stream
    pub fn wrap(stream: File, limit: usize) -> io::Result<Self> {
        let id = format!("{}", uuid::Uuid::new_v4());
        let arc_stream = Arc::new(Mutex::new(stream));
        
        let mut streams = STREAMS.write().expect("Failed to lock streams for writing");
        streams.insert(id, (arc_stream.clone(), limit));
        
        Ok(Self {
            source: arc_stream,
            limit,
        })
    }
    
    /// Open a quota stream by ID
    ///
    /// # Arguments
    /// * `id` - The ID of the registered stream
    ///
    /// # Returns
    /// A Result containing a new Quota instance or an error
    pub fn open(id: &str) -> io::Result<Self> {
        let streams = STREAMS.read().expect("Failed to lock streams for reading");
        
        if let Some((source, limit)) = streams.get(id) {
            Ok(Self {
                source: source.clone(),
                limit: *limit,
            })
        } else {
            Err(io::Error::new(io::Error::Kind::NotFound, "Stream not found"))
        }
    }
    
    /// Set blocking mode for the stream
    pub fn set_blocking(&self, blocking: bool) -> io::Result<()> {
        // In Rust, File doesn't have direct non-blocking mode
        // This would require a more complex implementation with async I/O
        Ok(())
    }
    
    /// Set timeout for read operations
    pub fn set_timeout(&self, duration: Duration) -> io::Result<()> {
        // This would require a more complex implementation
        // Standard File in Rust doesn't have timeout operations
        Ok(())
    }
    
    /// Set the write buffer size
    pub fn set_write_buffer(&self, buffer_size: usize) -> io::Result<()> {
        // Standard File in Rust doesn't have direct buffer size control
        // This would require using BufWriter with custom settings
        Ok(())
    }
}

impl Read for Quota {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let count = buf.len();
        self.limit = self.limit.saturating_sub(count);
        
        let mut source = self.source.lock().expect("Failed to lock source");
        source.read(buf)
    }
}

impl Write for Quota {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let size = buf.len();
        
        let to_write = if size > self.limit {
            &buf[0..self.limit]
        } else {
            buf
        };
        
        let mut source = self.source.lock().expect("Failed to lock source");
        let written = source.write(to_write)?;
        
        self.limit = self.limit.saturating_sub(written);
        Ok(written)
    }
    
    fn flush(&mut self) -> io::Result<()> {
        let mut source = self.source.lock().expect("Failed to lock source");
        source.flush()
    }
}

impl Seek for Quota {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let mut source = self.source.lock().expect("Failed to lock source");
        
        match pos {
            SeekFrom::End(offset) => {
                // Go to the end to find out last position's offset
                let old_offset = source.stream_position()?;
                let end_pos = source.seek(SeekFrom::End(0))?;
                let new_pos = (end_pos as i64 + offset) as u64;
                
                let seek_result = source.seek(SeekFrom::Start(new_pos))?;
                self.limit = self.limit.saturating_add((old_offset as i64 - new_pos as i64).abs() as usize);
                Ok(seek_result)
            },
            SeekFrom::Start(offset) => {
                let old_offset = source.stream_position()?;
                let seek_result = source.seek(SeekFrom::Start(offset))?;
                self.limit = self.limit.saturating_add((old_offset as i64 - offset as i64).abs() as usize);
                Ok(seek_result)
            },
            SeekFrom::Current(offset) => {
                self.limit = if offset < 0 {
                    self.limit.saturating_add((-offset) as usize)
                } else {
                    self.limit.saturating_sub(offset as usize)
                };
                source.seek(pos)
            }
        }
    }
}