/**
 * Copyright (c) 2012 Robin Appelman <icewind@owncloud.com>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

use super::Storage;
use crate::files::storage::DAV;

pub struct WebDAV {
    config: Option<HashMap<String, HashMap<String, serde_json::Value>>>,
    instance: Option<DAV>,
}

impl WebDAV {
    pub fn new() -> Self {
        WebDAV {
            config: None,
            instance: None,
        }
    }
}

#[async_trait::async_trait]
impl Storage for WebDAV {
    async fn set_up(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let id = Uuid::new_v4().to_string();
        
        // Load config from file
        self.config = match include_config("files_external/tests/config.php") {
            Ok(config) => Some(config),
            Err(_) => None,
        };
        
        // Check if WebDAV backend is configured
        if self.config.is_none() 
            || !self.config.as_ref().unwrap().contains_key("webdav") 
            || !self.config.as_ref().unwrap()["webdav"]["run"].as_bool().unwrap_or(false) {
            return Err("WebDAV backend not configured".into());
        }
        
        // Clone the config to modify it
        let mut webdav_config = self.config.as_ref().unwrap()["webdav"].clone();
        
        // Update the root path with the unique ID
        let root = webdav_config["root"].as_str().unwrap_or("") 
            .to_string() + "/" + &id;
        webdav_config.insert("root".to_string(), serde_json::Value::String(root));
        
        // Create DAV instance
        let dav = DAV::new(webdav_config);
        dav.mkdir(Path::new("/")).await?;
        
        self.instance = Some(dav);
        Ok(())
    }

    async fn tear_down(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(instance) = &self.instance {
            instance.rmdir(Path::new("/")).await?;
        }
        Ok(())
    }
}

fn include_config(path: &str) -> Result<HashMap<String, HashMap<String, serde_json::Value>>, Box<dyn std::error::Error>> {
    // In a real implementation, this would parse the PHP config file
    // For this example, we assume it's been converted to a JSON file that we can read
    let config_content = std::fs::read_to_string(path.replace(".php", ".json"))?;
    let config: HashMap<String, HashMap<String, serde_json::Value>> = serde_json::from_str(&config_content)?;
    Ok(config)
}