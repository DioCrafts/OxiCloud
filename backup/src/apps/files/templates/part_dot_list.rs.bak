use std::time::{SystemTime, UNIX_EPOCH};
use askama::Template;
use chrono::{DateTime, Utc};

// Suponiendo que estas funciones existen en tu codebase
use crate::util::{encode_path, human_file_size, relative_modified_date};

#[derive(Template)]
#[template(path = "part.list.html")]
pub struct FileListTemplate<'a> {
    pub files: &'a [File],
    pub base_url: &'a str,
    pub download_url: &'a str,
    pub readonly: bool,
}

pub struct File {
    pub fileid: String,
    pub name: String,
    pub directory: String, 
    pub file_type: FileType,
    pub mimetype: String,
    pub size: u64,
    pub etag: String,
    pub permissions: String,
    pub is_preview_available: bool,
    pub icon: String,
    pub mtime: u64,
    pub date: String,
    pub basename: String,
    pub extension: String,
}

pub enum FileType {
    Dir,
    File,
}

impl FileType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileType::Dir => "dir",
            FileType::File => "file",
        }
    }
}

impl<'a> FileListTemplate<'a> {
    pub fn new(
        files: &'a [File],
        base_url: &'a str,
        download_url: &'a str,
        readonly: bool,
    ) -> Self {
        Self {
            files,
            base_url,
            download_url,
            readonly,
        }
    }

    pub fn calculate_size_color(&self, size: u64) -> u8 {
        // El archivo es más grande, el tono de gris es más oscuro; megabytes*2
        let simple_size_color = 160 - (size as f64 / (1024.0 * 1024.0) * 2.0) as i32;
        if simple_size_color < 0 {
            0
        } else {
            simple_size_color as u8
        }
    }

    pub fn calculate_date_color(&self, mtime: u64) -> u8 {
        // Cuanto más antiguo es el archivo, más brillante es el tono de gris; días*14
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let relative_date_color = ((now - mtime) as f64 / 60.0 / 60.0 / 24.0 * 14.0).round() as u8;
        if relative_date_color > 160 {
            160
        } else {
            relative_date_color
        }
    }

    pub fn get_encoded_path(&self, path: &str) -> String {
        encode_path(path)
    }

    pub fn get_relative_modified_date(&self, mtime: u64) -> String {
        relative_modified_date(mtime)
    }

    pub fn get_file_size_human_readable(&self, size: u64) -> String {
        human_file_size(size)
    }
    
    pub fn count_stats(&self) -> (usize, usize, u64) {
        let mut total_files = 0;
        let mut total_dirs = 0;
        let mut total_size = 0;
        
        for file in self.files {
            match file.file_type {
                FileType::File => {
                    total_files += 1;
                    total_size += file.size;
                },
                FileType::Dir => total_dirs += 1,
            }
        }
        
        (total_files, total_dirs, total_size)
    }
}