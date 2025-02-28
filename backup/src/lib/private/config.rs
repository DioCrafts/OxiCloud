/*
 * ownCloud
 *
 * @author Frank Karlitschek
 * @author Jakob Sack
 * @copyright 2012 Frank Karlitschek frank@owncloud.org
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU AFFERO GENERAL PUBLIC LICENSE for more details.
 *
 * You should have received a copy of the GNU Affero General Public
 * License along with this library.  If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * An example of config.rs
 *
 *
 * let config = {
 *     "database": "mysql",
 *     "firstrun": false,
 *     "pi": 3.14
 * };
 *
 */

use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use anyhow::{anyhow, Context, Result};
use glob::glob;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use thiserror::Error;

/// Custom error type for config operations
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Can't write into config directory: {0}")]
    WriteError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    
    #[error("Parse error: {0}")]
    ParseError(String),
}

/// This struct is responsible for reading and writing config.php, the very basic
/// configuration file of ownCloud.
pub struct Config {
    /// associative array key => value
    cache: HashMap<String, Value>,
    config_dir: PathBuf,
    config_filename: PathBuf,
    debug_mode: bool,
}

impl Config {
    /// Creates a new Config instance
    ///
    /// # Arguments
    ///
    /// * `config_dir` - Path to the config directory
    pub fn new<P: AsRef<Path>>(config_dir: P) -> Result<Self> {
        let config_dir = config_dir.as_ref().to_path_buf();
        let config_filename = config_dir.join("config.php");
        
        let mut config = Self {
            cache: HashMap::new(),
            config_dir,
            config_filename,
            debug_mode: false,
        };
        
        config.read_data()?;
        config.set_debug_mode(std::env::var("DEBUG").is_ok());
        
        Ok(config)
    }
    
    /// Sets the debug mode
    ///
    /// # Arguments
    ///
    /// * `enable` - Whether to enable debug mode
    pub fn set_debug_mode(&mut self, enable: bool) {
        self.debug_mode = enable;
    }
    
    /// Lists all available config keys
    ///
    /// This function returns all keys saved in config. Please note that it
    /// does not return the values.
    pub fn get_keys(&self) -> Vec<String> {
        self.cache.keys().cloned().collect()
    }
    
    /// Gets a value from config
    ///
    /// # Arguments
    ///
    /// * `key` - The key to get
    /// * `default` - The default value to return if the key doesn't exist
    ///
    /// This function gets the value from config. If it does not exist,
    /// `default` will be returned.
    pub fn get_value<T>(&self, key: &str, default: Option<T>) -> Option<T> 
    where
        T: for<'de> Deserialize<'de>,
    {
        match self.cache.get(key) {
            Some(value) => serde_json::from_value(value.clone()).ok(),
            None => default,
        }
    }
    
    /// Sets a value
    ///
    /// # Arguments
    ///
    /// * `key` - The key to set
    /// * `value` - The value to set
    ///
    /// This function sets the value and writes the config.
    pub fn set_value<T>(&mut self, key: &str, value: T) -> Result<()> 
    where
        T: Serialize,
    {
        // Add change
        let json_value = serde_json::to_value(value)?;
        self.cache.insert(key.to_string(), json_value);
        
        // Write changes
        self.write_data()
    }
    
    /// Removes a key from the config
    ///
    /// # Arguments
    ///
    /// * `key` - The key to remove
    ///
    /// This function removes a key from the config.
    pub fn delete_key(&mut self, key: &str) -> Result<()> {
        if self.cache.remove(key).is_some() {
            // Write changes
            self.write_data()?;
        }
        Ok(())
    }
    
    /// Loads the config files
    ///
    /// Reads the config files and saves them to the cache
    fn read_data(&mut self) -> Result<()> {
        // Default config
        let mut config_files = vec![self.config_filename.clone()];
        
        // Add all files in the config dir ending with config.php
        let pattern = self.config_dir.join("*.config.php");
        let pattern_str = pattern.to_string_lossy();
        let mut extra = glob(&pattern_str)
            .context("Failed to read glob pattern")?
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        
        extra.sort();
        config_files.extend(extra);
        
        // Include file and merge config
        for file in config_files {
            if !file.exists() {
                continue;
            }
            
            let mut content = String::new();
            let mut file = match File::open(&file) {
                Ok(f) => f,
                Err(_) => continue, // Skip if can't open
            };
            
            if file.read_to_string(&mut content).is_err() {
                continue; // Skip if can't read
            }
            
            // Extremely simplified PHP parsing - in a real implementation you would need
            // a proper PHP parser or ensure the config is in a parseable format (like JSON)
            if let Some(config) = self.extract_php_array(&content) {
                self.cache.extend(config);
            }
        }
        
        Ok(())
    }
    
    /// Writes the config file
    ///
    /// Saves the config to the config file.
    fn write_data(&self) -> Result<()> {
        // Create a php file ...
        let mut content = String::from("<?php\n");
        if self.debug_mode {
            content.push_str("define('DEBUG',true);\n");
        }
        content.push_str("$CONFIG = ");
        
        // Convert HashMap to a PHP-style array string
        // This is simplified and would need a proper implementation
        let php_array = self.to_php_array(&self.cache);
        content.push_str(&php_array);
        content.push_str(";\n");
        
        // Write the file
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.config_filename)
            .map_err(|e| {
                ConfigError::WriteError(format!(
                    "This can usually be fixed by giving the webserver write access to the config directory. Error: {}", 
                    e
                ))
            })?;
        
        file.write_all(content.as_bytes())
            .map_err(|e| ConfigError::WriteError(e.to_string()))?;
        
        // Prevent others from reading the config
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = file.metadata()?;
            let mut perms = metadata.permissions();
            perms.set_mode(0o640);
            fs::set_permissions(&self.config_filename, perms)?;
        }
        
        // Clear opcode cache
        // In Rust, we don't need this, but we would call any related cache clearing code here
        
        Ok(())
    }
    
    /// Extract PHP array from PHP code
    ///
    /// This is a very simplified version that won't work for all PHP configs.
    /// In a real implementation, you would need a proper PHP parser.
    fn extract_php_array(&self, php_code: &str) -> Option<HashMap<String, Value>> {
        // This is a placeholder. In a real implementation, you would need to parse PHP code.
        // For a proper solution, consider using a PHP parser or ensuring the config is in a parseable format.
        let mut result = HashMap::new();
        
        // Very basic extraction logic - this is just a placeholder
        if let Some(config_start) = php_code.find("$CONFIG = ") {
            if let Some(config_end) = php_code[config_start..].find(";") {
                let config_str = &php_code[config_start + 10..config_start + config_end];
                // Here you would parse the PHP array syntax
                // This is just a placeholder for demonstration
                
                // In a real implementation, add proper parsing of PHP array syntax
                // or consider using a different format for configuration
            }
        }
        
        Some(result)
    }
    
    /// Convert a HashMap to a PHP array string
    ///
    /// This is a simplified version and won't work for all data types.
    fn to_php_array(&self, map: &HashMap<String, Value>) -> String {
        let mut result = String::from("array(\n");
        
        for (key, value) in map {
            result.push_str(&format!("    \"{}\" => ", key));
            
            match value {
                Value::Null => result.push_str("null"),
                Value::Bool(b) => result.push_str(if *b { "true" } else { "false" }),
                Value::Number(n) => result.push_str(&n.to_string()),
                Value::String(s) => result.push_str(&format!("\"{}\"", s.replace("\"", "\\\""))),
                Value::Array(arr) => {
                    result.push_str("array(");
                    for (i, item) in arr.iter().enumerate() {
                        if i > 0 {
                            result.push_str(", ");
                        }
                        // Recursive handling of nested values would go here
                        // This is simplified
                        result.push_str(&format!("{}", item));
                    }
                    result.push_str(")");
                },
                Value::Object(obj) => {
                    let mut inner_map = HashMap::new();
                    for (k, v) in obj {
                        inner_map.insert(k.clone(), v.clone());
                    }
                    result.push_str(&self.to_php_array(&inner_map));
                },
            }
            
            result.push_str(",\n");
        }
        
        result.push_str(")");
        result
    }
}