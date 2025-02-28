// Copyright (c) 2012 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use crate::archive::{Archive, ArchiveResult};
use std::env;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

#[cfg(not(target_os = "windows"))]
pub struct TestArchiveZip;

#[cfg(not(target_os = "windows"))]
impl crate::archive::TestArchive for TestArchiveZip {
    fn get_existing(&self) -> ArchiveResult<Box<dyn Archive>> {
        let server_root = env::var("SERVER_ROOT").expect("SERVER_ROOT environment variable not set");
        let dir = PathBuf::from(server_root).join("tests").join("data");
        let archive_path = dir.join("data.zip");
        
        Ok(Box::new(crate::archive::ZipArchive::new(archive_path)?))
    }

    fn get_new(&self) -> ArchiveResult<Box<dyn Archive>> {
        let temp_file = NamedTempFile::new()?;
        let path = temp_file.into_temp_path();
        let zip_path = path.with_extension("zip");
        
        Ok(Box::new(crate::archive::ZipArchive::new(zip_path)?))
    }
}