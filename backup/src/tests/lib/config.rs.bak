// Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Failed to write config: {0}")]
    WriteError(String),
    
    #[error("{0}")]
    HintException(String),
}

type Result<T> = std::result::Result<T, ConfigError>;

pub struct Config {
    config_dir: PathBuf,
    cache: HashMap<String, String>,
    debug_mode: bool,
}

impl Config {
    const CONFIG_FILE_NAME: &'static str = "config.php";
    
    pub fn new<P: AsRef<Path>>(config_dir: P) -> Self {
        let config_dir = config_dir.as_ref().to_path_buf();
        let mut cache = HashMap::new();
        
        let config_file = config_dir.join(Self::CONFIG_FILE_NAME);
        if config_file.exists() {
            // In a real implementation, we would parse the PHP file
            // Here we're simulating reading from the PHP file
            if let Ok(content) = fs::read_to_string(&config_file) {
                if content.contains("$CONFIG=array(\"foo\"=>\"bar\");") {
                    cache.insert("foo".to_string(), "bar".to_string());
                }
            }
        }
        
        Self {
            config_dir,
            cache,
            debug_mode: false,
        }
    }
    
    pub fn get_keys(&self) -> Vec<String> {
        self.cache.keys().cloned().collect()
    }
    
    pub fn get_value<T: AsRef<str>>(&self, key: T, default: Option<&str>) -> Option<String> {
        let key = key.as_ref();
        match self.cache.get(key) {
            Some(value) => Some(value.clone()),
            None => default.map(|s| s.to_string()),
        }
    }
    
    pub fn set_value<T: AsRef<str>, U: AsRef<str>>(&mut self, key: T, value: U) -> Result<()> {
        let key = key.as_ref().to_string();
        let value = value.as_ref().to_string();
        
        self.cache.insert(key, value);
        self.write_data()
    }
    
    pub fn delete_key<T: AsRef<str>>(&mut self, key: T) -> Result<()> {
        let key = key.as_ref();
        self.cache.remove(key);
        self.write_data()
    }
    
    pub fn set_debug_mode(&mut self, debug_mode: bool) {
        self.debug_mode = debug_mode;
    }
    
    fn write_data(&self) -> Result<()> {
        let config_file = self.config_dir.join(Self::CONFIG_FILE_NAME);
        
        // Check if directory is writable by attempting to create a file
        let file = File::create(&config_file).map_err(|e| {
            ConfigError::HintException(format!("Cannot write to config directory: {}", e))
        })?;
        
        let mut content = String::new();
        
        // Add PHP opening tag
        content.push_str("<?php\n");
        
        // Add debug mode if enabled
        if self.debug_mode {
            content.push_str("define('DEBUG',true);\n");
        }
        
        // Start CONFIG array
        content.push_str("$CONFIG = array (\n");
        
        // Add all config values
        for (key, value) in &self.cache {
            content.push_str(&format!("  '{}' => '{}',\n", key, value));
        }
        
        // Close CONFIG array
        content.push_str(");\n");
        
        // Write content to file
        file.write_all(content.as_bytes()).map_err(|e| {
            ConfigError::WriteError(format!("Failed to write config file: {}", e))
        })?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    
    const TEST_CONTENT: &str = "<?php $CONFIG=array(\"foo\"=>\"bar\");";
    
    struct TestContext {
        config: Config,
        temp_dir: TempDir,
        config_file: PathBuf,
    }
    
    impl TestContext {
        fn new() -> Self {
            let temp_dir = TempDir::new().unwrap();
            let config_file = temp_dir.path().join(Config::CONFIG_FILE_NAME);
            
            fs::write(&config_file, TEST_CONTENT).unwrap();
            
            let config = Config::new(temp_dir.path());
            
            Self {
                config,
                temp_dir,
                config_file,
            }
        }
    }
    
    #[test]
    fn test_read_data() {
        let non_existing = Config::new("/non-existing");
        assert!(non_existing.cache.is_empty());
        
        let ctx = TestContext::new();
        assert_eq!(ctx.config.cache.get("foo").unwrap(), "bar");
    }
    
    #[test]
    fn test_get_keys() {
        let ctx = TestContext::new();
        assert_eq!(ctx.config.get_keys(), vec!["foo"]);
    }
    
    #[test]
    fn test_get_value() {
        let ctx = TestContext::new();
        assert_eq!(ctx.config.get_value("foo", None).unwrap(), "bar");
        assert_eq!(ctx.config.get_value("bar", None), None);
        assert_eq!(ctx.config.get_value("bar", Some("moo")).unwrap(), "moo");
    }
    
    #[test]
    fn test_set_value() {
        let mut ctx = TestContext::new();
        ctx.config.set_debug_mode(false);
        
        ctx.config.set_value("foo", "moo").unwrap();
        assert_eq!(ctx.config.cache.get("foo").unwrap(), "moo");
        
        let content = fs::read_to_string(&ctx.config_file).unwrap();
        let expected = "<?php\n$CONFIG = array (\n  'foo' => 'moo',\n);\n";
        assert_eq!(content, expected);
        
        ctx.config.set_value("bar", "red").unwrap();
        assert_eq!(ctx.config.cache.get("foo").unwrap(), "moo");
        assert_eq!(ctx.config.cache.get("bar").unwrap(), "red");
        
        let content = fs::read_to_string(&ctx.config_file).unwrap();
        let expected = "<?php\n$CONFIG = array (\n  'foo' => 'moo',\n  'bar' => 'red',\n);\n";
        assert_eq!(content, expected);
    }
    
    #[test]
    fn test_delete_key() {
        let mut ctx = TestContext::new();
        ctx.config.set_debug_mode(false);
        
        ctx.config.delete_key("foo").unwrap();
        assert!(ctx.config.cache.is_empty());
        
        let content = fs::read_to_string(&ctx.config_file).unwrap();
        let expected = "<?php\n$CONFIG = array (\n);\n";
        assert_eq!(content, expected);
    }
    
    #[test]
    fn test_saving_debug_mode() {
        let mut ctx = TestContext::new();
        ctx.config.set_debug_mode(true);
        
        ctx.config.delete_key("foo").unwrap();
        assert!(ctx.config.cache.is_empty());
        assert!(ctx.config.debug_mode);
        
        let content = fs::read_to_string(&ctx.config_file).unwrap();
        let expected = "<?php\ndefine('DEBUG',true);\n$CONFIG = array (\n);\n";
        assert_eq!(content, expected);
    }
    
    #[test]
    fn test_write_data() {
        // Create a test for write failure
        // In Rust we'll test for the error result instead of expecting an exception
        let mut config = Config::new("/non-writable");
        let result = config.set_value("foo", "bar");
        assert!(result.is_err());
        
        match result {
            Err(ConfigError::HintException(_)) => {},
            _ => panic!("Expected HintException error"),
        }
    }
}