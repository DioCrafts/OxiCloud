// Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::path::{Path, PathBuf};
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("Empty template name")]
    EmptyTemplateName,

    #[error("Template file not found: template:{0} formfactor:{1}")]
    TemplateNotFound(String, String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub struct TemplateFileLocator {
    form_factor: String,
    dirs: Vec<String>,
    path: Option<String>,
}

impl TemplateFileLocator {
    pub fn new(form_factor: String, dirs: Vec<String>) -> Self {
        Self {
            form_factor,
            dirs,
            path: None,
        }
    }

    pub fn find(&mut self, template: &str) -> Result<String, TemplateError> {
        if template.is_empty() {
            return Err(TemplateError::EmptyTemplateName);
        }

        for dir in &self.dirs {
            // Try with form factor first
            let file_path = format!("{}{}{}.php", dir, template, self.form_factor);
            if fs::metadata(&file_path).map(|m| m.is_file()).unwrap_or(false) {
                self.path = Some(dir.clone());
                return Ok(file_path);
            }

            // Then try without form factor
            let file_path = format!("{}{}.php", dir, template);
            if fs::metadata(&file_path).map(|m| m.is_file()).unwrap_or(false) {
                self.path = Some(dir.clone());
                return Ok(file_path);
            }
        }

        Err(TemplateError::TemplateNotFound(
            template.to_string(),
            self.form_factor.clone(),
        ))
    }

    pub fn get_path(&self) -> Option<&str> {
        self.path.as_deref()
    }
}