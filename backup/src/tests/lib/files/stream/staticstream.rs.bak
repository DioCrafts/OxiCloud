// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::fs;
use std::path::PathBuf;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;

/// Static Stream test module
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    /// StaticStream implementation for testing
    struct StaticStream {
        source_file: PathBuf,
        source_text: String,
    }

    impl StaticStream {
        fn new() -> Self {
            let server_root = env::var("SERVER_ROOT").unwrap_or_else(|_| ".".to_string());
            let source_file = PathBuf::from(&server_root).join("tests/data/lorem.txt");
            let source_text = fs::read_to_string(&source_file)
                .expect("Failed to read source file");
            
            Self {
                source_file,
                source_text,
            }
        }

        fn clear() {
            STATIC_FILES.lock().unwrap().clear();
        }
    }

    lazy_static! {
        static ref STATIC_FILES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    }

    // Helper functions to simulate PHP file operations with static:// scheme
    fn file_put_contents(path: &str, contents: &str) -> bool {
        if path.starts_with("static://") {
            let key = path.replace("static://", "");
            STATIC_FILES.lock().unwrap().insert(key, contents.to_string());
            true
        } else {
            false
        }
    }

    fn file_get_contents(path: &str) -> Option<String> {
        if path.starts_with("static://") {
            let key = path.replace("static://", "");
            STATIC_FILES.lock().unwrap().get(&key).cloned()
        } else {
            None
        }
    }

    fn file_exists(path: &str) -> bool {
        if path.starts_with("static://") {
            let key = path.replace("static://", "");
            STATIC_FILES.lock().unwrap().contains_key(&key)
        } else {
            false
        }
    }

    fn is_file(path: &str) -> bool {
        file_exists(path)
    }

    fn is_dir(_path: &str) -> bool {
        false
    }

    fn filetype(path: &str) -> Option<String> {
        if file_exists(path) {
            Some("file".to_string())
        } else {
            None
        }
    }

    fn unlink(path: &str) -> bool {
        if path.starts_with("static://") {
            let key = path.replace("static://", "");
            STATIC_FILES.lock().unwrap().remove(&key).is_some()
        } else {
            false
        }
    }

    #[test]
    fn test_content() {
        let stream = StaticStream::new();
        file_put_contents("static://foo", &stream.source_text);
        assert_eq!(Some(stream.source_text), file_get_contents("static://foo"));
        StaticStream::clear();
    }

    #[test]
    fn test_multiple_files() {
        let stream = StaticStream::new();
        file_put_contents("static://foo", &stream.source_text);
        file_put_contents("static://bar", &stream.source_text.chars().rev().collect::<String>());
        
        assert_eq!(Some(stream.source_text), file_get_contents("static://foo"));
        assert_eq!(
            Some(stream.source_text.chars().rev().collect::<String>()), 
            file_get_contents("static://bar")
        );
        StaticStream::clear();
    }

    #[test]
    fn test_overwrite() {
        let stream = StaticStream::new();
        file_put_contents("static://foo", &stream.source_text);
        file_put_contents("static://foo", "qwerty");
        assert_eq!(Some("qwerty".to_string()), file_get_contents("static://foo"));
        StaticStream::clear();
    }

    #[test]
    fn test_is_file() {
        assert!(!is_file("static://foo"));
        let stream = StaticStream::new();
        file_put_contents("static://foo", &stream.source_text);
        assert!(is_file("static://foo"));
        StaticStream::clear();
    }

    #[test]
    fn test_is_dir() {
        assert!(!is_dir("static://foo"));
        let stream = StaticStream::new();
        file_put_contents("static://foo", &stream.source_text);
        assert!(!is_dir("static://foo"));
        StaticStream::clear();
    }

    #[test]
    fn test_file_type() {
        let stream = StaticStream::new();
        file_put_contents("static://foo", &stream.source_text);
        assert_eq!(Some("file".to_string()), filetype("static://foo"));
        StaticStream::clear();
    }

    #[test]
    fn test_unlink() {
        assert!(!file_exists("static://foo"));
        let stream = StaticStream::new();
        file_put_contents("static://foo", &stream.source_text);
        assert!(file_exists("static://foo"));
        unlink("static://foo");
        assert!(!file_exists("static://foo"));
        StaticStream::clear();
    }
}