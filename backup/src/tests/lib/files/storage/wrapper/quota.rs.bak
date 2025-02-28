// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::io::{Read, Write, Seek};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use async_trait::async_trait;

use crate::files::storage::Storage;
use crate::files::storage::local::LocalStorage;
use crate::files::storage::wrapper::quota::QuotaStorage;
use crate::files::scanner::Scanner;
use crate::helper;

pub struct QuotaTest {
    instance: QuotaStorage<LocalStorage>,
    temp_dir: TempDir,
}

#[async_trait]
impl crate::test::TestCase for QuotaTest {
    async fn set_up(&mut self) {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let storage = LocalStorage::new(temp_dir.path().to_path_buf());
        self.instance = QuotaStorage::new(storage, 10_000_000);
        self.temp_dir = temp_dir;
    }

    async fn tear_down(&mut self) {
        // TempDir will clean up automatically when dropped
    }
}

impl QuotaTest {
    fn get_limited_storage(&self, limit: u64) -> QuotaStorage<LocalStorage> {
        let storage = LocalStorage::new(self.temp_dir.path().to_path_buf());
        let scanner = storage.get_scanner();
        scanner.scan("").expect("Failed to scan storage");
        QuotaStorage::new(storage, limit)
    }

    #[test]
    fn test_file_put_contents_not_enough_space() {
        let instance = self.get_limited_storage(3);
        assert!(instance.file_put_contents("foo", b"foobar".to_vec()).is_err());
    }

    #[test]
    fn test_copy_not_enough_space() {
        let instance = self.get_limited_storage(9);
        assert_eq!(
            instance.file_put_contents("foo", b"foobar".to_vec()).unwrap(),
            6
        );
        
        instance.get_scanner().scan("").expect("Failed to scan");
        assert!(instance.copy("foo", "bar").is_err());
    }

    #[test]
    fn test_free_space() {
        let instance = self.get_limited_storage(9);
        assert_eq!(instance.free_space("").unwrap(), 9);
    }

    #[test]
    fn test_write_not_enough_space() {
        let instance = self.get_limited_storage(9);
        let mut file = instance.open("foo", crate::files::OpenMode::Write).unwrap();
        
        assert_eq!(file.write(b"foobar").unwrap(), 6);
        assert_eq!(file.write(b"qwerty").unwrap(), 3);
        drop(file);
        
        assert_eq!(
            instance.file_get_contents("foo").unwrap(),
            b"foobarqwe".to_vec()
        );
    }

    #[test]
    fn test_return_regular_stream_on_read() {
        let instance = self.get_limited_storage(9);

        // Create test file first
        {
            let mut file = instance.open("foo", crate::files::OpenMode::Write).unwrap();
            file.write(b"blablacontent").unwrap();
        }

        let file = instance.open("foo", crate::files::OpenMode::Read).unwrap();
        // In Rust we can't check the stream type like PHP does with stream_get_meta_data
        // Instead, we verify we can read the content correctly
        let mut content = Vec::new();
        file.read_to_end(&mut content).unwrap();
        assert_eq!(content, b"blablacontent");
    }

    #[test]
    fn test_return_quota_stream_on_write() {
        let instance = self.get_limited_storage(9);
        let file = instance.open("foo", crate::files::OpenMode::Write).unwrap();
        // In Rust we can't check the stream type like PHP does
        // We verify it's a QuotaFile by checking the quota is enforced
        drop(file);
        
        let mut file = instance.open("foo", crate::files::OpenMode::Write).unwrap();
        assert_eq!(file.write(b"foobar").unwrap(), 6);
        assert_eq!(file.write(b"qwerty").unwrap(), 3);
        assert_eq!(file.write(b"more").unwrap(), 0); // Should be limited by quota
    }
}