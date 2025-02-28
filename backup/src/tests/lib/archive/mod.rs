// Módulos generados automáticamente

pub mod zip;
pub mod tar;

// Contenido fusionado desde src/tests/lib/archive.rs
extern crate test_utils;
extern crate archives;

use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use test_utils::test_helpers::{create_temp_file, create_temp_dir, remove_dir_all};
use archives::{Archive, ArchiveError};

/// Abstract test class for archive implementations
pub trait TestArchive {
    /// Get the existing test archive
    fn get_existing(&self) -> Box<dyn Archive>;
    
    /// Get a new archive for write testing
    fn get_new(&self) -> Box<dyn Archive>;
}

#[derive(Default)]
pub struct TestArchiveImpl {
    instance: Option<Box<dyn Archive>>,
}

impl TestArchiveImpl {
    pub fn new() -> Self {
        Self { instance: None }
    }
    
    pub fn test_get_files(&mut self, test_archive: &dyn TestArchive) -> Result<(), ArchiveError> {
        self.instance = Some(test_archive.get_existing());
        let all_files = self.instance.as_ref().unwrap().get_files()?;
        
        let expected = vec!["lorem.txt", "logo-wide.png", "dir/", "dir/lorem.txt"];
        assert_eq!(all_files.len(), 4, "only found {} out of 4 expected files", all_files.len());
        
        for file in &expected {
            assert!(all_files.contains(&file.to_string()), "cant find {} in archive", file);
            assert!(self.instance.as_ref().unwrap().file_exists(file)?, "file {} does not exist in archive", file);
        }
        
        assert!(!self.instance.as_ref().unwrap().file_exists("non/existing/file")?);
        
        let root_content = self.instance.as_ref().unwrap().get_folder("")?;
        let expected_root = vec!["lorem.txt", "logo-wide.png", "dir/"];
        assert_eq!(root_content.len(), 3);
        
        for file in &expected_root {
            assert!(root_content.contains(&file.to_string()), "cant find {} in archive", file);
        }
        
        let dir_content = self.instance.as_ref().unwrap().get_folder("dir/")?;
        let expected_dir = vec!["lorem.txt"];
        assert_eq!(dir_content.len(), 1);
        
        for file in &expected_dir {
            assert!(dir_content.contains(&file.to_string()), "cant find {} in archive", file);
        }
        
        Ok(())
    }
    
    pub fn test_content(&mut self, test_archive: &dyn TestArchive) -> Result<(), ArchiveError> {
        self.instance = Some(test_archive.get_existing());
        let server_root = env!("CARGO_MANIFEST_DIR");
        let dir = PathBuf::from(server_root).join("tests/data");
        let text_file = dir.join("lorem.txt");
        
        let content_from_archive = self.instance.as_ref().unwrap().get_file("lorem.txt")?;
        let content_from_file = fs::read_to_string(&text_file)?;
        assert_eq!(content_from_file, content_from_archive);
        
        let tmp_file = create_temp_file(".txt")?;
        self.instance.as_ref().unwrap().extract_file("lorem.txt", tmp_file.clone())?;
        
        let extracted_content = fs::read_to_string(tmp_file)?;
        assert_eq!(content_from_file, extracted_content);
        
        Ok(())
    }
    
    pub fn test_write(&mut self, test_archive: &dyn TestArchive) -> Result<(), ArchiveError> {
        let server_root = env!("CARGO_MANIFEST_DIR");
        let dir = PathBuf::from(server_root).join("tests/data");
        let text_file = dir.join("lorem.txt");
        
        self.instance = Some(test_archive.get_new());
        assert_eq!(self.instance.as_ref().unwrap().get_files()?.len(), 0);
        
        self.instance.as_ref().unwrap().add_file("lorem.txt", text_file.to_str().unwrap())?;
        assert_eq!(self.instance.as_ref().unwrap().get_files()?.len(), 1);
        assert!(self.instance.as_ref().unwrap().file_exists("lorem.txt")?);
        assert!(!self.instance.as_ref().unwrap().file_exists("lorem.txt/")?);
        
        let content_from_file = fs::read_to_string(&text_file)?;
        let content_from_archive = self.instance.as_ref().unwrap().get_file("lorem.txt")?;
        assert_eq!(content_from_file, content_from_archive);
        
        self.instance.as_ref().unwrap().add_file("lorem.txt", "foobar")?;
        assert_eq!("foobar", self.instance.as_ref().unwrap().get_file("lorem.txt")?);
        
        Ok(())
    }
    
    pub fn test_read_stream(&mut self, test_archive: &dyn TestArchive) -> Result<(), ArchiveError> {
        let server_root = env!("CARGO_MANIFEST_DIR");
        let dir = PathBuf::from(server_root).join("tests/data");
        
        self.instance = Some(test_archive.get_existing());
        let mut file_handle = self.instance.as_ref().unwrap().get_stream("lorem.txt", "r")?;
        
        let file_size = self.instance.as_ref().unwrap().filesize("lorem.txt")?;
        let mut content = String::new();
        file_handle.read_to_string(&mut content)?;
        
        let expected_content = fs::read_to_string(dir.join("lorem.txt"))?;
        assert_eq!(expected_content, content);
        
        Ok(())
    }
    
    pub fn test_write_stream(&mut self, test_archive: &dyn TestArchive) -> Result<(), ArchiveError> {
        let server_root = env!("CARGO_MANIFEST_DIR");
        let dir = PathBuf::from(server_root).join("tests/data");
        
        self.instance = Some(test_archive.get_new());
        let mut dest_handle = self.instance.as_ref().unwrap().get_stream("lorem.txt", "w")?;
        
        let mut source_handle = fs::File::open(dir.join("lorem.txt"))?;
        let mut content = Vec::new();
        source_handle.read_to_end(&mut content)?;
        dest_handle.write_all(&content)?;
        
        // Close handles explicitly
        drop(source_handle);
        drop(dest_handle);
        
        assert!(self.instance.as_ref().unwrap().file_exists("lorem.txt")?);
        
        let expected_content = fs::read_to_string(dir.join("lorem.txt"))?;
        let archive_content = self.instance.as_ref().unwrap().get_file("lorem.txt")?;
        assert_eq!(expected_content, archive_content);
        
        Ok(())
    }
    
    pub fn test_folder(&mut self, test_archive: &dyn TestArchive) -> Result<(), ArchiveError> {
        self.instance = Some(test_archive.get_new());
        
        assert!(!self.instance.as_ref().unwrap().file_exists("/test")?);
        assert!(!self.instance.as_ref().unwrap().file_exists("/test/")?);
        
        self.instance.as_ref().unwrap().add_folder("/test")?;
        
        assert!(self.instance.as_ref().unwrap().file_exists("/test")?);
        assert!(self.instance.as_ref().unwrap().file_exists("/test/")?);
        
        self.instance.as_ref().unwrap().remove("/test")?;
        
        assert!(!self.instance.as_ref().unwrap().file_exists("/test")?);
        assert!(!self.instance.as_ref().unwrap().file_exists("/test/")?);
        
        Ok(())
    }
    
    pub fn test_extract(&mut self, test_archive: &dyn TestArchive) -> Result<(), ArchiveError> {
        let server_root = env!("CARGO_MANIFEST_DIR");
        let dir = PathBuf::from(server_root).join("tests/data");
        
        self.instance = Some(test_archive.get_existing());
        let tmp_dir = create_temp_dir()?;
        
        self.instance.as_ref().unwrap().extract(tmp_dir.clone())?;
        
        assert!(Path::new(&format!("{}/lorem.txt", tmp_dir)).exists());
        assert!(Path::new(&format!("{}/dir/lorem.txt", tmp_dir)).exists());
        assert!(Path::new(&format!("{}/logo-wide.png", tmp_dir)).exists());
        
        let original_content = fs::read_to_string(dir.join("lorem.txt"))?;
        let extracted_content = fs::read_to_string(format!("{}/lorem.txt", tmp_dir))?;
        assert_eq!(original_content, extracted_content);
        
        remove_dir_all(tmp_dir)?;
        
        Ok(())
    }
    
    pub fn test_move_remove(&mut self, test_archive: &dyn TestArchive) -> Result<(), ArchiveError> {
        let server_root = env!("CARGO_MANIFEST_DIR");
        let dir = PathBuf::from(server_root).join("tests/data");
        let text_file = dir.join("lorem.txt");
        
        self.instance = Some(test_archive.get_new());
        self.instance.as_ref().unwrap().add_file("lorem.txt", text_file.to_str().unwrap())?;
        
        assert!(!self.instance.as_ref().unwrap().file_exists("target.txt")?);
        self.instance.as_ref().unwrap().rename("lorem.txt", "target.txt")?;
        
        assert!(self.instance.as_ref().unwrap().file_exists("target.txt")?);
        assert!(!self.instance.as_ref().unwrap().file_exists("lorem.txt")?);
        
        let original_content = fs::read_to_string(&text_file)?;
        let archive_content = self.instance.as_ref().unwrap().get_file("target.txt")?;
        assert_eq!(original_content, archive_content);
        
        self.instance.as_ref().unwrap().remove("target.txt")?;
        assert!(!self.instance.as_ref().unwrap().file_exists("target.txt")?);
        
        Ok(())
    }
    
    pub fn test_recursive(&mut self, test_archive: &dyn TestArchive) -> Result<(), ArchiveError> {
        let server_root = env!("CARGO_MANIFEST_DIR");
        let dir = PathBuf::from(server_root).join("tests/data");
        
        self.instance = Some(test_archive.get_new());
        self.instance.as_ref().unwrap().add_recursive("/dir", dir.to_str().unwrap())?;
        
        assert!(self.instance.as_ref().unwrap().file_exists("/dir/lorem.txt")?);
        assert!(self.instance.as_ref().unwrap().file_exists("/dir/data.zip")?);
        assert!(self.instance.as_ref().unwrap().file_exists("/dir/data.tar.gz")?);
        
        Ok(())
    }
}