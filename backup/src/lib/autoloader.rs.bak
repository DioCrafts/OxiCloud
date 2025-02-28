// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::time::Duration;

/// Namespace representation for OC
pub mod oc {
    pub struct CLASSPATH;
    pub struct APPSROOTS;
}

/// Memory cache interface
pub trait MemoryCache {
    fn get(&self, key: &str) -> Option<Vec<String>>;
    fn set(&self, key: &str, value: Vec<String>, ttl: u64);
}

/// Autoloader struct for dynamic class loading
pub struct Autoloader {
    use_global_class_path: bool,
    prefix_paths: HashMap<String, String>,
    class_paths: HashMap<String, String>,
    
    /// Optional low-latency memory cache for class to path mapping.
    memory_cache: Option<Box<dyn MemoryCache>>,
}

impl Autoloader {
    /// Create a new Autoloader instance
    pub fn new() -> Self {
        Self {
            use_global_class_path: true,
            prefix_paths: HashMap::new(),
            class_paths: HashMap::new(),
            memory_cache: None,
        }
    }

    /// Add a custom prefix to the autoloader
    ///
    /// # Arguments
    /// * `prefix` - The class prefix
    /// * `path` - The path to load classes from
    pub fn register_prefix(&mut self, prefix: &str, path: &str) {
        self.prefix_paths.insert(prefix.to_string(), path.to_string());
    }

    /// Add a custom classpath to the autoloader
    ///
    /// # Arguments
    /// * `class` - The class name
    /// * `path` - The path to the class file
    pub fn register_class(&mut self, class: &str, path: &str) {
        self.class_paths.insert(class.to_string(), path.to_string());
    }

    /// Disable the usage of the global classpath OC::$CLASSPATH
    pub fn disable_global_class_path(&mut self) {
        self.use_global_class_path = false;
    }

    /// Enable the usage of the global classpath OC::$CLASSPATH
    pub fn enable_global_class_path(&mut self) {
        self.use_global_class_path = true;
    }

    /// Get the possible paths for a class
    ///
    /// # Arguments
    /// * `class` - The class name
    ///
    /// # Returns
    /// * `Vec<String>` - Array of possible paths or empty if the class is not part of ownCloud
    pub fn find_class(&self, class: &str) -> Vec<String> {
        let class = class.trim_start_matches('\\').trim_end_matches('\\');
        let mut paths = Vec::new();

        if let Some(path) = self.class_paths.get(class) {
            paths.push(path.clone());
        } else if self.use_global_class_path {
            // Note: This part requires access to OC::$CLASSPATH which would need to be
            // implemented differently in a real Rust application
            
            // Mock implementation for demonstration
            /*
            if let Some(path) = OC::$CLASSPATH.get(class) {
                paths.push(path.clone());
                
                // TODO: Remove this when necessary
                // Remove "apps/" from inclusion path for smooth migration to multi app dir
                if path.starts_with("apps/") {
                    log::debug!("include path for class \"{}\" starts with \"apps/\"", class);
                    paths.push(path.replace("apps/", ""));
                }
            }
            */
        }

        if class.starts_with("OC_") {
            // First check for legacy classes if underscores are used
            let suffix = class[3..].to_lowercase().replace('_', "/");
            paths.push(format!("private/legacy/{}.php", suffix));
            paths.push(format!("private/{}.php", suffix));
        } else if class.starts_with("OC\\") {
            let suffix = class[3..].to_lowercase().replace('\\', "/");
            paths.push(format!("private/{}.php", suffix));
            paths.push(format!("{}.php", suffix));
        } else if class.starts_with("OCP\\") {
            let suffix = class[4..].to_lowercase().replace('\\', "/");
            paths.push(format!("public/{}.php", suffix));
        } else if class.starts_with("OCA\\") {
            let parts: Vec<&str> = class.splitn(3, '\\').collect();
            if parts.len() >= 3 {
                let app = parts[1].to_lowercase();
                let rest = parts[2].to_lowercase().replace('\\', "/");
                
                // Mock implementation for APPSROOTS
                /*
                for app_dir in &OC::$APPSROOTS {
                    let app_path = &app_dir.path;
                    if Path::new(&format!("{}/{}", app_path, app)).exists() {
                        paths.push(format!("{}/{}/{}.php", app_path, app, rest));
                        paths.push(format!("{}/{}/lib/{}.php", app_path, app, rest));
                    }
                }
                */
            }
        } else if class.starts_with("Test_") {
            let suffix = class[5..].to_lowercase().replace('_', "/");
            paths.push(format!("tests/lib/{}.php", suffix));
        } else if class.starts_with("Test\\") {
            let suffix = class[5..].to_lowercase().replace('\\', "/");
            paths.push(format!("tests/lib/{}.php", suffix));
        } else {
            for (prefix, dir) in &self.prefix_paths {
                if class.starts_with(prefix) {
                    let path = class.replace('\\', "/").replace('_', "/");
                    paths.push(format!("{}/{}.php", dir, path));
                }
            }
        }

        paths
    }

    /// Load the specified class
    ///
    /// # Arguments
    /// * `class` - The class name
    ///
    /// # Returns
    /// * `bool` - Whether the class was loaded successfully
    pub fn load(&self, class: &str) -> bool {
        let paths_to_require = if let Some(cache) = &self.memory_cache {
            if let Some(paths) = cache.get(class) {
                paths
            } else {
                let paths = self.resolve_include_paths(class);
                if !paths.is_empty() {
                    cache.set(class, paths.clone(), 60); // Cache for 60 seconds
                }
                paths
            }
        } else {
            self.resolve_include_paths(class)
        };

        // In PHP this would require the files
        // In Rust, we'd need a different approach for loading code dynamically
        // This is a mock implementation
        for _path in &paths_to_require {
            // require_once in PHP would go here
            // In Rust, dynamic loading would use a different mechanism
        }

        !paths_to_require.is_empty()
    }

    /// Helper function to resolve include paths
    fn resolve_include_paths(&self, class: &str) -> Vec<String> {
        let mut resolved_paths = Vec::new();
        for path in self.find_class(class) {
            // This is a simplified mock of stream_resolve_include_path
            if Path::new(&path).exists() {
                resolved_paths.push(path);
            }
        }
        resolved_paths
    }

    /// Sets the optional low-latency cache for class to path mapping.
    ///
    /// # Arguments
    /// * `memory_cache` - Instance of memory cache.
    pub fn set_memory_cache(&mut self, memory_cache: Option<Box<dyn MemoryCache>>) {
        self.memory_cache = memory_cache;
    }
}

impl Default for Autoloader {
    fn default() -> Self {
        Self::new()
    }
}