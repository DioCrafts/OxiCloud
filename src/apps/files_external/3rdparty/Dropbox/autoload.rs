//! This module provides automatic module loading functionality.
//!
//! @package Dropbox 
//! @copyright Copyright (C) 2010 Rooftop Solutions. All rights reserved.
//! @author Evert Pot (http://www.rooftopsolutions.nl/) 
//! @license http://code.google.com/p/dropbox-php/wiki/License MIT

use std::path::{Path, PathBuf};
use std::fs;
use std::io;

/// Dropbox module autoloading functionality
pub struct DropboxAutoloader;

impl DropboxAutoloader {
    /// Creates a new autoloader instance
    pub fn new() -> Self {
        Self {}
    }

    /// Attempts to load a Dropbox class by name
    ///
    /// # Arguments
    ///
    /// * `class_name` - The name of the class to load
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the class was found and loaded
    /// * `Err(io::Error)` if the class couldn't be loaded
    pub fn load_class(&self, class_name: &str) -> io::Result<()> {
        if !class_name.starts_with("Dropbox_") {
            return Ok(());
        }

        let base_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        
        // Remove the "Dropbox_" prefix (8 chars) and replace underscores with path separators
        let module_path = class_name[8..].replace('_', "/");
        let file_path = base_dir.join(format!("{}.rs", module_path));

        // Check if the file exists
        if file_path.exists() {
            // In Rust, we don't need to explicitly include files as the
            // compiler handles modules differently. This is a stub that 
            // simulates the PHP behavior but would need to be adapted
            // to actual Rust module loading patterns.
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Could not find module file: {:?}", file_path)
            ))
        }
    }

    /// Returns all available Dropbox class files
    ///
    /// # Returns
    ///
    /// * `Vec<PathBuf>` containing paths to all available class files
    pub fn list_available_classes(&self) -> io::Result<Vec<PathBuf>> {
        let base_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut class_files = Vec::new();
        
        // Recursively scan directory for Rust files
        Self::scan_directory(&base_dir, &mut class_files)?;
        
        Ok(class_files)
    }

    // Helper function to recursively scan directories for Rust files
    fn scan_directory(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    Self::scan_directory(&path, files)?;
                } else if let Some(extension) = path.extension() {
                    if extension == "rs" {
                        files.push(path);
                    }
                }
            }
        }
        
        Ok(())
    }
}

// Create and register a global autoloader
lazy_static::lazy_static! {
    pub static ref DROPBOX_AUTOLOADER: DropboxAutoloader = DropboxAutoloader::new();
}

// This function would be called from other modules
pub fn initialize() {
    // In Rust, we don't need explicit autoloading as in PHP.
    // This is a no-op function that represents the registration.
    // The actual loading is done by the Rust compiler and module system.
}