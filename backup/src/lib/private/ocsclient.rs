// Copyright (c) 2012 Frank Karlitschek frank@owncloud.org
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;
use chrono::prelude::*;
use serde::Deserialize;
use anyhow::{Result, anyhow};
use roxmltree;
use reqwest;
use crate::util;
use crate::config;
use crate::log;

/// Application data structure
#[derive(Debug, Clone, Default)]
pub struct AppData {
    pub id: String,
    pub name: String,
    pub label: String,
    pub version: String,
    pub type_id: String,
    pub typename: String,
    pub personid: String,
    pub license: String,
    pub detailpage: String,
    pub preview: String,
    pub preview1: String,
    pub preview2: String,
    pub preview3: String,
    pub changed: i64,
    pub description: String,
    pub score: String,
    pub downloadlink: String,
}

/// This class provides an easy way to interact with the OCS AppStore server.
pub struct OcsClient;

impl OcsClient {
    /// Get the url of the OCS AppStore server.
    ///
    /// This function returns the url of the OCS AppStore server. It's possible
    /// to set it in the config file or it will fallback to the default
    fn get_app_store_url() -> String {
        let default = if util::get_edition_string().is_empty() {
            "http://api.apps.owncloud.com/v1"
        } else {
            ""
        };
        
        config::get_value("appstoreurl", default.to_string())
    }

    /// Get the content of an OCS url call.
    ///
    /// This function calls an OCS server and returns the response.
    /// It also sets a sane timeout
    async fn get_ocs_response(url: &str) -> Result<String> {
        util::get_url_content(url).await
    }

    /// Get all the categories from the OCS server
    ///
    /// This function returns a list of all the application categories on the OCS server
    /// Returns None if config value appstoreenabled is set to false
    pub async fn get_categories() -> Result<Option<HashMap<i32, String>>> {
        if !config::get_bool_value("appstoreenabled", true) {
            return Ok(None);
        }
        
        let url = format!("{}/content/categories", Self::get_app_store_url());
        let xml = Self::get_ocs_response(&url).await?;
        
        let doc = roxmltree::Document::parse(&xml)
            .map_err(|e| anyhow!("Failed to parse XML: {}", e))?;
        
        let mut cats = HashMap::new();
        
        if let Some(data_node) = doc.descendants().find(|n| n.has_tag_name("data")) {
            for category in data_node.children().filter(|n| n.has_tag_name("category")) {
                let id = category.children()
                    .find(|n| n.has_tag_name("id"))
                    .and_then(|n| n.text())
                    .and_then(|t| t.parse::<i32>().ok())
                    .ok_or_else(|| anyhow!("Invalid category ID"))?;
                
                let name = category.children()
                    .find(|n| n.has_tag_name("name"))
                    .and_then(|n| n.text())
                    .map(|s| s.to_string())
                    .ok_or_else(|| anyhow!("Invalid category name"))?;
                
                cats.insert(id, name);
            }
        }
        
        Ok(Some(cats))
    }

    /// Get all the applications from the OCS server
    ///
    /// This function returns a list of all the applications on the OCS server
    pub async fn get_applications(
        categories: &[String], 
        page: usize, 
        filter: &str
    ) -> Result<Option<Vec<AppData>>> {
        if !config::get_bool_value("appstoreenabled", true) {
            return Ok(Some(Vec::new()));
        }
        
        let categories_string = categories.join("x");
        let version = util::get_version().join("x");
        
        let url = format!(
            "{}/content/data?categories={}&sortmode=new&page={}&pagesize=100&filter={}&version={}",
            Self::get_app_store_url(),
            urlencoding::encode(&categories_string),
            urlencoding::encode(&page.to_string()),
            urlencoding::encode(filter),
            urlencoding::encode(&version)
        );
        
        let xml = match Self::get_ocs_response(&url).await {
            Ok(xml) => xml,
            Err(_) => return Ok(None),
        };
        
        let doc = roxmltree::Document::parse(&xml)
            .map_err(|e| anyhow!("Failed to parse XML: {}", e))?;
        
        let mut apps = Vec::new();
        
        if let Some(content_node) = doc.descendants()
            .find(|n| n.has_tag_name("data"))
            .and_then(|n| n.children().find(|n| n.has_tag_name("content"))) {
            
            for app_node in content_node.children().filter(|n| n.is_element()) {
                let mut app = AppData::default();
                
                app.id = Self::get_node_text(&app_node, "id");
                app.name = Self::get_node_text(&app_node, "name");
                app.label = Self::get_node_text(&app_node, "label");
                app.version = Self::get_node_text(&app_node, "version");
                app.type_id = Self::get_node_text(&app_node, "typeid");
                app.typename = Self::get_node_text(&app_node, "typename");
                app.personid = Self::get_node_text(&app_node, "personid");
                app.license = Self::get_node_text(&app_node, "license");
                app.detailpage = Self::get_node_text(&app_node, "detailpage");
                app.preview = Self::get_node_text(&app_node, "smallpreviewpic1");
                
                let changed_str = Self::get_node_text(&app_node, "changed");
                app.changed = if !changed_str.is_empty() {
                    DateTime::parse_from_rfc3339(&changed_str)
                        .map(|dt| dt.timestamp())
                        .unwrap_or(0)
                } else {
                    0
                };
                
                app.description = Self::get_node_text(&app_node, "description");
                app.score = Self::get_node_text(&app_node, "score");
                
                apps.push(app);
            }
        }
        
        Ok(Some(apps))
    }

    /// Get an application from the OCS server
    ///
    /// This function returns application data from the OCS server
    pub async fn get_application(id: &str) -> Result<Option<AppData>> {
        if !config::get_bool_value("appstoreenabled", true) {
            return Ok(None);
        }
        
        let url = format!(
            "{}/content/data/{}",
            Self::get_app_store_url(),
            urlencoding::encode(id)
        );
        
        let xml = match Self::get_ocs_response(&url).await {
            Ok(xml) => xml,
            Err(_) => {
                log::fatal("core", "Unable to parse OCS content");
                return Ok(None);
            }
        };
        
        let doc = roxmltree::Document::parse(&xml)
            .map_err(|e| anyhow!("Failed to parse XML: {}", e))?;
        
        let content_node = match doc.descendants()
            .find(|n| n.has_tag_name("data"))
            .and_then(|n| n.children().find(|n| n.has_tag_name("content"))) {
            Some(node) => node,
            None => return Ok(None),
        };
            
        let mut app = AppData::default();
        
        app.id = Self::get_node_text(&content_node, "id");
        app.name = Self::get_node_text(&content_node, "name");
        app.version = Self::get_node_text(&content_node, "version");
        app.type_id = Self::get_node_text(&content_node, "typeid");
        app.label = Self::get_node_text(&content_node, "label");
        app.typename = Self::get_node_text(&content_node, "typename");
        app.personid = Self::get_node_text(&content_node, "personid");
        app.detailpage = Self::get_node_text(&content_node, "detailpage");
        app.preview1 = Self::get_node_text(&content_node, "smallpreviewpic1");
        app.preview2 = Self::get_node_text(&content_node, "smallpreviewpic2");
        app.preview3 = Self::get_node_text(&content_node, "smallpreviewpic3");
        
        let changed_str = Self::get_node_text(&content_node, "changed");
        app.changed = if !changed_str.is_empty() {
            DateTime::parse_from_rfc3339(&changed_str)
                .map(|dt| dt.timestamp())
                .unwrap_or(0)
        } else {
            0
        };
        
        app.description = Self::get_node_text(&content_node, "description");
        app.score = Self::get_node_text(&content_node, "score");
        
        Ok(Some(app))
    }

    /// Get the download url for an application from the OCS server
    ///
    /// This function returns a download url for an application from the OCS server
    pub async fn get_application_download(id: &str, item: &str) -> Result<Option<AppData>> {
        if !config::get_bool_value("appstoreenabled", true) {
            return Ok(None);
        }
        
        let url = format!(
            "{}/content/download/{}/{}",
            Self::get_app_store_url(),
            urlencoding::encode(id),
            urlencoding::encode(item)
        );
        
        let xml = match Self::get_ocs_response(&url).await {
            Ok(xml) => xml,
            Err(_) => {
                log::fatal("core", "Unable to parse OCS content");
                return Ok(None);
            }
        };
        
        let doc = roxmltree::Document::parse(&xml)
            .map_err(|e| anyhow!("Failed to parse XML: {}", e))?;
        
        let content_node = match doc.descendants()
            .find(|n| n.has_tag_name("data"))
            .and_then(|n| n.children().find(|n| n.has_tag_name("content"))) {
            Some(node) => node,
            None => return Ok(None),
        };
        
        let mut app = AppData::default();
        app.downloadlink = Self::get_node_text(&content_node, "downloadlink");
        
        Ok(Some(app))
    }
    
    // Helper function to extract text from XML nodes
    fn get_node_text(parent: &roxmltree::Node, tag_name: &str) -> String {
        parent.children()
            .find(|n| n.has_tag_name(tag_name))
            .and_then(|n| n.text())
            .unwrap_or("")
            .to_string()
    }
}