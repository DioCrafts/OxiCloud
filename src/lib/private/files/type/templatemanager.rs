//! Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Manager for file type templates
pub struct TemplateManager {
    templates: HashMap<String, String>,
}

impl TemplateManager {
    /// Create a new TemplateManager
    pub fn new() -> Self {
        TemplateManager {
            templates: HashMap::new(),
        }
    }

    /// Register a template for a specific mimetype
    pub fn register_template(&mut self, mimetype: String, path: String) {
        self.templates.insert(mimetype, path);
    }

    /// Get the path of the template for a mimetype
    ///
    /// # Arguments
    ///
    /// * `mimetype` - The mimetype to get the template for
    ///
    /// # Returns
    ///
    /// * `Option<&String>` - The path to the template or None if not found
    pub fn get_template_path(&self, mimetype: &str) -> Option<&String> {
        self.templates.get(mimetype)
    }

    /// Get the template content for a mimetype
    ///
    /// # Arguments
    ///
    /// * `mimetype` - The mimetype to get the template for
    ///
    /// # Returns
    ///
    /// * `String` - The template content or empty string if not found
    pub fn get_template(&self, mimetype: &str) -> String {
        if let Some(path) = self.get_template_path(mimetype) {
            fs::read_to_string(Path::new(path)).unwrap_or_default()
        } else {
            String::new()
        }
    }

    /// Get the template content for a mimetype asynchronously
    ///
    /// # Arguments
    ///
    /// * `mimetype` - The mimetype to get the template for
    ///
    /// # Returns
    ///
    /// * `Result<String, std::io::Error>` - The template content or error
    pub async fn get_template_async(&self, mimetype: &str) -> Result<String, std::io::Error> {
        if let Some(path) = self.get_template_path(mimetype) {
            tokio::fs::read_to_string(Path::new(path)).await
        } else {
            Ok(String::new())
        }
    }
}

impl Default for TemplateManager {
    fn default() -> Self {
        Self::new()
    }
}