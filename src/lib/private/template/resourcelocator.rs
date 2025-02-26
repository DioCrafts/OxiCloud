//! Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fmt;

pub trait ResourceLocator {
    fn do_find(&mut self, resource: &str) -> Result<(), ResourceError>;
    fn do_find_theme(&mut self, resource: &str) -> Result<(), ResourceError>;
}

#[derive(Debug)]
pub struct ResourceError {
    message: String,
    form_factor: String,
    serverroot: String,
}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} formfactor:{} serverroot:{}",
            self.message, self.form_factor, self.serverroot
        )
    }
}

impl std::error::Error for ResourceError {}

pub struct BaseResourceLocator {
    theme: Option<String>,
    form_factor: String,
    mapping: HashMap<String, String>,
    serverroot: String,
    thirdpartyroot: String,
    webroot: String,
    resources: Vec<(String, String, String)>,
}

impl BaseResourceLocator {
    pub fn new(
        theme: Option<String>,
        form_factor: String,
        core_map: HashMap<String, String>,
        party_map: HashMap<String, String>,
    ) -> Self {
        let serverroot = core_map.keys().next().unwrap().to_string();
        let thirdpartyroot = party_map.keys().next().unwrap().to_string();
        
        let mut mapping = HashMap::new();
        mapping.extend(core_map.clone());
        mapping.extend(party_map);
        
        let webroot = mapping.get(&serverroot).unwrap().clone();
        
        Self {
            theme,
            form_factor,
            mapping,
            serverroot,
            thirdpartyroot,
            webroot,
            resources: Vec::new(),
        }
    }

    pub fn find(&mut self, resources: &[String]) -> Result<(), ResourceError> {
        for resource in resources {
            self.do_find(resource)?;
        }
        
        if let Some(theme) = &self.theme {
            if !theme.is_empty() {
                for resource in resources {
                    self.do_find_theme(resource)?;
                }
            }
        }
        
        Ok(())
    }

    /// Append the $file resource if exist at $root
    /// 
    /// # Arguments
    /// * `root` - path to check
    /// * `file` - the filename
    /// * `webroot` - base for path, default map $root to $webroot
    ///
    /// # Returns
    /// `true` if the file exists and was appended, `false` otherwise
    pub fn append_if_exist(&mut self, root: &str, file: &str, webroot: Option<&str>) -> bool {
        let file_path = Path::new(root).join(file);
        
        if file_path.is_file() {
            let webroot = match webroot {
                Some(w) => w.to_string(),
                None => self.mapping.get(root).cloned().unwrap_or_default(),
            };
            
            self.resources.push((root.to_string(), webroot, file.to_string()));
            return true;
        }
        
        false
    }

    pub fn get_resources(&self) -> &[(String, String, String)] {
        &self.resources
    }
    
    pub fn serverroot(&self) -> &str {
        &self.serverroot
    }
    
    pub fn form_factor(&self) -> &str {
        &self.form_factor
    }
    
    pub fn theme(&self) -> Option<&str> {
        self.theme.as_deref()
    }
    
    pub fn thirdpartyroot(&self) -> &str {
        &self.thirdpartyroot
    }
    
    pub fn webroot(&self) -> &str {
        &self.webroot
    }
}

impl ResourceLocator for BaseResourceLocator {
    fn do_find(&mut self, _resource: &str) -> Result<(), ResourceError> {
        // This implementation should be overridden by derived structs
        Err(ResourceError {
            message: "Method not implemented".to_string(),
            form_factor: self.form_factor.clone(),
            serverroot: self.serverroot.clone(),
        })
    }

    fn do_find_theme(&mut self, _resource: &str) -> Result<(), ResourceError> {
        // This implementation should be overridden by derived structs
        Err(ResourceError {
            message: "Method not implemented".to_string(),
            form_factor: self.form_factor.clone(),
            serverroot: self.serverroot.clone(),
        })
    }
}