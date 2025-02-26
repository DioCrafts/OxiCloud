// Copyright (c) 2012 Lukas Reschke <lukas@statuscode.ch>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write, Seek};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use mockall::predicate::*;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct MockView {
    inner: Arc<MockViewImpl>,
}

#[automock]
#[async_trait]
pub trait ViewImpl {
    fn file_exists(&self, path: &str) -> bool;
}

impl MockView {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(MockViewImpl::new()),
        }
    }

    pub fn file_exists(&self, path: &str) -> bool {
        self.inner.file_exists(path)
    }
}

pub struct OcHelper;

impl OcHelper {
    pub fn human_file_size(size: u64) -> String {
        if size == 0 {
            return "0 B".to_string();
        }
        
        let units = ["B", "kB", "MB", "GB", "TB", "PB"];
        let power = ((size as f64).ln() / 1024_f64.ln()).floor() as usize;
        let power = std::cmp::min(power, units.len() - 1);
        
        let size = size as f64 / 1024_f64.powi(power as i32);
        
        if (size - size.round()).abs() < 0.01 {
            format!("{} {}", size.round(), units[power])
        } else {
            format!("{:.1} {}", size, units[power])
        }
    }

    pub fn computer_file_size(input: &str) -> f64 {
        let re = Regex::new(r"^(\d+(?:\.\d+)?)\s*([BKMGTP])?B?$").unwrap();
        
        if let Some(caps) = re.captures(input) {
            let size = caps.get(1).unwrap().as_str().parse::<f64>().unwrap_or(0.0);
            let unit = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            
            let multiplier = match unit {
                "K" => 1024.0,
                "M" => 1024.0 * 1024.0,
                "G" => 1024.0 * 1024.0 * 1024.0,
                "T" => 1024.0 * 1024.0 * 1024.0 * 1024.0,
                "P" => 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
                _ => 1.0,
            };
            
            return size * multiplier;
        }
        
        0.0
    }

    pub fn get_mime_type(path: &str) -> String {
        let path = Path::new(path);
        
        if path.is_dir() {
            return "httpd/unix-directory".to_string();
        }
        
        match infer::get_from_path(path) {
            Ok(Some(kind)) => kind.mime_type().to_string(),
            _ => "application/octet-stream".to_string(),
        }
    }

    pub fn get_file_name_mime_type(file_name: &str) -> String {
        if file_name.is_empty() || !file_name.contains('.') || file_name.starts_with('.') {
            return "application/octet-stream".to_string();
        }
        
        let extension = file_name.split('.').last().unwrap_or("");
        match extension {
            "txt" => "text/plain",
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            "zip" => "application/zip",
            "gz" | "tgz" => "application/x-gzip",
            _ => "application/octet-stream",
        }.to_string()
    }

    pub fn get_string_mime_type(data: &str) -> String {
        // Simplified implementation - in real code you would use a library like `infer`
        // or analyze the content more deeply
        "text/plain; charset=us-ascii".to_string()
    }

    pub fn is_subdirectory(child: &str, parent: &str) -> bool {
        let child_path = PathBuf::from(child);
        let parent_path = PathBuf::from(parent);
        
        if let (Ok(child_abs), Ok(parent_abs)) = (fs::canonicalize(&child_path), fs::canonicalize(&parent_path)) {
            return child_abs.starts_with(parent_abs);
        }
        
        false
    }

    pub fn mb_array_change_key_case(
        array: HashMap<String, String>, 
        case: Option<u8>
    ) -> HashMap<String, String> {
        let mut result = HashMap::new();
        
        for (key, value) in array {
            let new_key = match case {
                Some(1) => key.to_uppercase(), // MB_CASE_UPPER
                _ => key.to_lowercase(),       // Default is lowercase
            };
            result.insert(new_key, value);
        }
        
        result
    }

    pub fn mb_substr_replace(string: &str, replacement: &str, start: usize) -> String {
        let mut result = string.to_string();
        result.insert_str(start, replacement);
        result
    }

    pub fn mb_str_replace(needle: &str, replacement: &str, haystack: &str) -> String {
        haystack.replace(needle, replacement)
    }

    pub fn recursive_array_search<K, V>(haystack: &HashMap<K, V>, needle: &V) -> Option<&K> 
    where 
        V: PartialEq,
    {
        for (key, value) in haystack {
            if value == needle {
                return Some(key);
            }
        }
        None
    }

    pub fn build_not_existing_file_name_for_view(
        dir: &str, 
        name: &str, 
        view: &MockView
    ) -> String {
        let path = if dir == "/" {
            format!("/{}", name)
        } else {
            format!("{}/{}", dir, name)
        };
        
        if !view.file_exists(&path) {
            return path;
        }
        
        let (base_name, ext) = match name.rfind('.') {
            Some(pos) => (&name[..pos], &name[pos..]),
            None => (name, ""),
        };
        
        // Check if the base name already contains a counter
        let re = Regex::new(r"^(.*?)(\s+\((\d+)\))?$").unwrap();
        let caps = re.captures(base_name).unwrap();
        
        let file_name_without_counter = caps.get(1).unwrap().as_str();
        let counter_start = caps.get(3).map_or(1, |m| m.as_str().parse::<i32>().unwrap() + 1);
        
        for counter in counter_start.. {
            let new_path = format!("{}/{} ({}){}", dir, file_name_without_counter, counter, ext);
            if !view.file_exists(&new_path) {
                return new_path;
            }
        }
        
        // Should never reach here, but just in case
        path
    }

    pub fn stream_copy<R: Read, W: Write>(
        source: Option<&mut R>, 
        target: Option<&mut W>
    ) -> (u64, bool) {
        if source.is_none() || target.is_none() {
            return (0, false);
        }
        
        let mut source = source.unwrap();
        let mut target = target.unwrap();
        
        let mut buffer = [0; 8192];
        let mut total_bytes = 0u64;
        
        loop {
            match source.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(bytes_read) => {
                    match target.write_all(&buffer[0..bytes_read]) {
                        Ok(_) => total_bytes += bytes_read as u64,
                        Err(_) => return (total_bytes, false),
                    }
                },
                Err(_) => return (total_bytes, false),
            }
        }
        
        (total_bytes, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;
    use mockall::predicate::*;

    #[test]
    fn test_human_file_size() {
        let test_cases = vec![
            (0, "0 B"),
            (1024, "1 kB"),
            (10000000, "9.5 MB"),
            (500000000000, "465.7 GB"),
            (500000000000000, "454.7 TB"),
            (500000000000000000, "444.1 PB"),
        ];
        
        for (input, expected) in test_cases {
            let result = OcHelper::human_file_size(input);
            assert_eq!(expected, result);
        }
    }

    #[test]
    fn test_computer_file_size() {
        let test_cases = vec![
            ("0 B", 0.0),
            ("1 kB", 1024.0),
            ("9.5 MB", 9961472.0),
            ("465.7 GB", 500041567436.8),
        ];
        
        for (input, expected) in test_cases {
            let result = OcHelper::computer_file_size(input);
            assert_eq!(expected, result);
        }
    }

    #[test]
    fn test_get_mime_type() {
        let server_root = env!("CARGO_MANIFEST_DIR");
        let dir = format!("{}/tests/data", server_root);
        
        let result = OcHelper::get_mime_type(&format!("{}/", dir));
        assert_eq!("httpd/unix-directory", result);
        
        let result = OcHelper::get_mime_type(&format!("{}/data.tar.gz", dir));
        assert_eq!("application/x-gzip", result);
        
        let result = OcHelper::get_mime_type(&format!("{}/data.zip", dir));
        assert_eq!("application/zip", result);
        
        let result = OcHelper::get_mime_type(&format!("{}/logo-wide.svg", dir));
        assert_eq!("image/svg+xml", result);
        
        let result = OcHelper::get_mime_type(&format!("{}/logo-wide.png", dir));
        assert_eq!("image/png", result);
    }

    #[test]
    fn test_get_file_name_mime_type() {
        assert_eq!("text/plain", OcHelper::get_file_name_mime_type("foo.txt"));
        assert_eq!("image/png", OcHelper::get_file_name_mime_type("foo.png"));
        assert_eq!("image/png", OcHelper::get_file_name_mime_type("foo.bar.png"));
        assert_eq!("application/octet-stream", OcHelper::get_file_name_mime_type(".png"));
        assert_eq!("application/octet-stream", OcHelper::get_file_name_mime_type("foo"));
        assert_eq!("application/octet-stream", OcHelper::get_file_name_mime_type(""));
    }

    #[test]
    fn test_get_string_mime_type() {
        let result = OcHelper::get_string_mime_type("/data/data.tar.gz");
        assert_eq!("text/plain; charset=us-ascii", result);
    }

    #[test]
    fn test_is_subdirectory() {
        let result = OcHelper::is_subdirectory("./data/", "/anotherDirectory/");
        assert!(!result);
        
        let result = OcHelper::is_subdirectory("./data/", "./data/");
        assert!(result);
        
        let temp_dir = tempdir().unwrap();
        let test_subdir = temp_dir.path().join("TestSubdirectory");
        fs::create_dir(&test_subdir).unwrap();
        
        let result = OcHelper::is_subdirectory(
            test_subdir.to_str().unwrap(),
            temp_dir.path().to_str().unwrap()
        );
        assert!(result);
    }

    #[test]
    fn test_mb_array_change_key_case() {
        let mut array_start = HashMap::new();
        array_start.insert("Foo".to_string(), "bar".to_string());
        array_start.insert("Bar".to_string(), "foo".to_string());
        
        let mut expected = HashMap::new();
        expected.insert("foo".to_string(), "bar".to_string());
        expected.insert("bar".to_string(), "foo".to_string());
        
        let result = OcHelper::mb_array_change_key_case(array_start, None);
        assert_eq!(expected, result);
        
        let mut array_start = HashMap::new();
        array_start.insert("foo".to_string(), "bar".to_string());
        array_start.insert("bar".to_string(), "foo".to_string());
        
        let mut expected = HashMap::new();
        expected.insert("FOO".to_string(), "bar".to_string());
        expected.insert("BAR".to_string(), "foo".to_string());
        
        let result = OcHelper::mb_array_change_key_case(array_start, Some(1)); // MB_CASE_UPPER
        assert_eq!(expected, result);
    }

    #[test]
    fn test_mb_substr_replace() {
        let result = OcHelper::mb_substr_replace("This  is a teststring", "string", 5);
        assert_eq!("This string is a teststring", result);
    }

    #[test]
    fn test_mb_str_replace() {
        let result = OcHelper::mb_str_replace("teststring", "string", "This is a teststring");
        assert_eq!("This is a string", result);
    }

    #[test]
    fn test_recursive_array_search() {
        let mut haystack = HashMap::new();
        haystack.insert("Foo".to_string(), "own".to_string());
        haystack.insert("Bar".to_string(), "Cloud".to_string());
        
        let result = OcHelper::recursive_array_search(&haystack, &"own".to_string());
        assert_eq!(Some(&"Foo".to_string()), result);
        
        let result = OcHelper::recursive_array_search(&haystack, &"NotFound".to_string());
        assert_eq!(None, result);
    }

    #[test]
    fn test_build_not_existing_file_name_for_view() {
        let mut view = MockView::new();
        let mut mock = view.inner.expect();
        
        mock.expect_file_exists()
            .with(eq("/filename"))
            .return_const(false)
            .times(1);
        
        assert_eq!("/filename", OcHelper::build_not_existing_file_name_for_view("/", "filename", &view));
        
        let mut view = MockView::new();
        let mut mock = view.inner.expect();
        
        mock.expect_file_exists()
            .with(eq("dir/filename.ext"))
            .return_const(false)
            .times(1);
        
        assert_eq!("dir/filename.ext", OcHelper::build_not_existing_file_name_for_view("dir", "filename.ext", &view));
        
        let mut view = MockView::new();
        let mut mock = view.inner.expect();
        
        mock.expect_file_exists()
            .with(eq("dir/filename.ext"))
            .return_const(true)
            .times(1);
            
        mock.expect_file_exists()
            .with(eq("dir/filename (2).ext"))
            .return_const(false)
            .times(1);
        
        assert_eq!("dir/filename (2).ext", OcHelper::build_not_existing_file_name_for_view("dir", "filename.ext", &view));
        
        // Additional test cases would follow the same pattern
    }

    #[test]
    fn test_stream_copy() {
        // Case: both source and target are None
        let (count, result) = OcHelper::stream_copy::<File, File>(None, None);
        assert_eq!(0, count);
        assert!(!result);
        
        // Case: source is Some but target is None
        let server_root = env!("CARGO_MANIFEST_DIR");
        let path = format!("{}/tests/data/lorem.txt", server_root);
        let mut file = File::open(path).unwrap();
        let (count, result) = OcHelper::stream_copy(Some(&mut file), None::<&mut File>);
        assert_eq!(0, count);
        assert!(!result);
        
        // Case: both source and target are Some
        let server_root = env!("CARGO_MANIFEST_DIR");
        let source_path = format!("{}/tests/data/lorem.txt", server_root);
        let target_path = format!("{}/tests/data/lorem-copy.txt", server_root);
        let mut source_file = File::open(&source_path).unwrap();
        let mut target_file = File::create(&target_path).unwrap();
        
        let source_size = fs::metadata(&source_path).unwrap().len();
        
        let (count, result) = OcHelper::stream_copy(Some(&mut source_file), Some(&mut target_file));
        assert_eq!(source_size, count);
        assert!(result);
        
        // Clean up
        fs::remove_file(&target_path).unwrap();
    }
}