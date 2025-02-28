// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::io::{Read, Write, Seek, SeekFrom, Result as IoResult, Error as IoError, ErrorKind};
use std::fs::File;
use tempfile::tempfile;
use std::cell::RefCell;
use std::rc::Rc;

pub struct QuotaStream<T: Read + Write + Seek> {
    inner: T,
    quota: i64,
    used: i64,
}

impl<T: Read + Write + Seek> QuotaStream<T> {
    pub fn new(inner: T, quota: i64) -> Self {
        QuotaStream {
            inner,
            quota,
            used: 0,
        }
    }
}

impl<T: Read + Write + Seek> Read for QuotaStream<T> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        self.inner.read(buf)
    }
}

impl<T: Read + Write + Seek> Write for QuotaStream<T> {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        let bytes_left = self.quota - self.used;
        if bytes_left <= 0 {
            return Ok(0);
        }
        
        let to_write = std::cmp::min(buf.len() as i64, bytes_left) as usize;
        let written = self.inner.write(&buf[..to_write])?;
        self.used += written as i64;
        Ok(written)
    }

    fn flush(&mut self) -> IoResult<()> {
        self.inner.flush()
    }
}

impl<T: Read + Write + Seek> Seek for QuotaStream<T> {
    fn seek(&mut self, pos: SeekFrom) -> IoResult<u64> {
        let new_pos = self.inner.seek(pos)?;
        // Reset used counter if we seek back
        let current_pos = new_pos as i64;
        self.used = current_pos;
        Ok(new_pos)
    }
}

pub struct Quota;

impl Quota {
    pub fn wrap<T: Read + Write + Seek>(source: T, limit: i64) -> QuotaStream<T> {
        QuotaStream::new(source, limit)
    }

    pub fn clear() {
        // Stub for compatibility, not needed in Rust implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read, Write, Seek, SeekFrom};
    use tempfile::tempfile;

    fn get_stream(mode: &str, limit: i64) -> QuotaStream<Cursor<Vec<u8>>> {
        let source = Cursor::new(Vec::new());
        Quota::wrap(source, limit)
    }

    #[test]
    fn test_write_enough_space() {
        let mut stream = get_stream("w+", 100);
        assert_eq!(6, stream.write(b"foobar").unwrap());
        stream.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = [0; 100];
        let read = stream.read(&mut buffer).unwrap();
        assert_eq!("foobar", std::str::from_utf8(&buffer[..read]).unwrap());
    }

    #[test]
    fn test_write_not_enough_space() {
        let mut stream = get_stream("w+", 3);
        assert_eq!(3, stream.write(b"foobar").unwrap());
        stream.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = [0; 100];
        let read = stream.read(&mut buffer).unwrap();
        assert_eq!("foo", std::str::from_utf8(&buffer[..read]).unwrap());
    }

    #[test]
    fn test_write_not_enough_space_second_time() {
        let mut stream = get_stream("w+", 9);
        assert_eq!(6, stream.write(b"foobar").unwrap());
        assert_eq!(3, stream.write(b"qwerty").unwrap());
        stream.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = [0; 100];
        let read = stream.read(&mut buffer).unwrap();
        assert_eq!("foobarqwe", std::str::from_utf8(&buffer[..read]).unwrap());
    }

    #[test]
    fn test_write_enough_space_rewind() {
        let mut stream = get_stream("w+", 6);
        assert_eq!(6, stream.write(b"foobar").unwrap());
        stream.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(3, stream.write(b"qwe").unwrap());
        stream.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = [0; 100];
        let read = stream.read(&mut buffer).unwrap();
        assert_eq!("qwebar", std::str::from_utf8(&buffer[..read]).unwrap());
    }

    #[test]
    fn test_write_not_enough_space_read() {
        let mut stream = get_stream("w+", 6);
        assert_eq!(6, stream.write(b"foobar").unwrap());
        stream.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = [0; 6];
        assert_eq!(6, stream.read(&mut buffer).unwrap());
        assert_eq!("foobar", std::str::from_utf8(&buffer).unwrap());
        assert_eq!(0, stream.write(b"qwe").unwrap());
    }

    #[test]
    fn test_write_not_enough_space_existing_stream() {
        let mut source = Cursor::new(Vec::new());
        source.write(b"foobar").unwrap();
        let mut stream = Quota::wrap(source, 3);
        assert_eq!(3, stream.write(b"foobar").unwrap());
        stream.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = [0; 100];
        let read = stream.read(&mut buffer).unwrap();
        assert_eq!("foobarfoo", std::str::from_utf8(&buffer[..read]).unwrap());
    }

    #[test]
    fn test_write_not_enough_space_existing_stream_rewind() {
        let mut source = Cursor::new(Vec::new());
        source.write(b"foobar").unwrap();
        let mut stream = Quota::wrap(source, 3);
        stream.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(3, stream.write(b"qwerty").unwrap());
        stream.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = [0; 100];
        let read = stream.read(&mut buffer).unwrap();
        assert_eq!("qweity", std::str::from_utf8(&buffer[..read]).unwrap());
    }

    #[test]
    fn test_fseek_returns_success() {
        let mut stream = get_stream("w+", 100);
        stream.write(b"0123456789").unwrap();
        assert_eq!(3, stream.seek(SeekFrom::Start(3)).unwrap());
        assert_eq!(2, stream.seek(SeekFrom::Current(-1)).unwrap());
        assert_eq!(6, stream.seek(SeekFrom::End(-4)).unwrap());
    }

    #[test]
    fn test_write_after_seek_end_with_enough_space() {
        let mut stream = get_stream("w+", 100);
        stream.write(b"0123456789").unwrap();
        stream.seek(SeekFrom::End(-3)).unwrap();
        assert_eq!(11, stream.write(b"abcdefghijk").unwrap());
        stream.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = [0; 100];
        let read = stream.read(&mut buffer).unwrap();
        assert_eq!("0123456abcdefghijk", std::str::from_utf8(&buffer[..read]).unwrap());
    }

    #[test]
    fn test_write_after_seek_end_with_not_enough_space() {
        let mut stream = get_stream("w+", 13);
        stream.write(b"0123456789").unwrap();
        // seek forward first to potentially week out potential limit calculation errors
        stream.seek(SeekFrom::Start(4)).unwrap();
        // seek to the end
        stream.seek(SeekFrom::End(-3)).unwrap();
        assert_eq!(6, stream.write(b"abcdefghijk").unwrap());
        stream.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = [0; 100];
        let read = stream.read(&mut buffer).unwrap();
        assert_eq!("0123456abcdef", std::str::from_utf8(&buffer[..read]).unwrap());
    }

    #[test]
    fn test_write_after_seek_set_with_enough_space() {
        let mut stream = get_stream("w+", 100);
        stream.write(b"0123456789").unwrap();
        stream.seek(SeekFrom::Start(7)).unwrap();
        assert_eq!(11, stream.write(b"abcdefghijk").unwrap());
        stream.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = [0; 100];
        let read = stream.read(&mut buffer).unwrap();
        assert_eq!("0123456abcdefghijk", std::str::from_utf8(&buffer[..read]).unwrap());
    }

    #[test]
    fn test_write_after_seek_set_with_not_enough_space() {
        let mut stream = get_stream("w+", 13);
        stream.write(b"0123456789").unwrap();
        stream.seek(SeekFrom::Start(7)).unwrap();
        assert_eq!(6, stream.write(b"abcdefghijk").unwrap());
        stream.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = [0; 100];
        let read = stream.read(&mut buffer).unwrap();
        assert_eq!("0123456abcdef", std::str::from_utf8(&buffer[..read]).unwrap());
    }

    #[test]
    fn test_write_after_seek_cur_with_enough_space() {
        let mut stream = get_stream("w+", 100);
        stream.write(b"0123456789").unwrap();
        stream.seek(SeekFrom::Start(0)).unwrap();
        stream.seek(SeekFrom::Current(3)).unwrap();
        stream.seek(SeekFrom::Current(5)).unwrap();
        stream.seek(SeekFrom::Current(-1)).unwrap();
        assert_eq!(11, stream.write(b"abcdefghijk").unwrap());
        stream.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = [0; 100];
        let read = stream.read(&mut buffer).unwrap();
        assert_eq!("0123456abcdefghijk", std::str::from_utf8(&buffer[..read]).unwrap());
    }

    #[test]
    fn test_write_after_seek_cur_with_not_enough_space() {
        let mut stream = get_stream("w+", 13);
        stream.write(b"0123456789").unwrap();
        stream.seek(SeekFrom::Start(0)).unwrap();
        stream.seek(SeekFrom::Current(3)).unwrap();
        stream.seek(SeekFrom::Current(5)).unwrap();
        stream.seek(SeekFrom::Current(-1)).unwrap();
        assert_eq!(6, stream.write(b"abcdefghijk").unwrap());
        stream.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = [0; 100];
        let read = stream.read(&mut buffer).unwrap();
        assert_eq!("0123456abcdef", std::str::from_utf8(&buffer[..read]).unwrap());
    }
}