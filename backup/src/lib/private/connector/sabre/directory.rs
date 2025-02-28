// Copyright © 2011 Jakob Sack kde@jakobsack.de
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
use std::io::{Read, Write};
use std::path::PathBuf;
use async_trait::async_trait;

use crate::connector::sabre::node::{Node, SabreNode};
use crate::db::{self, Database};
use crate::files::{
    filesystem::{self, FileInfo},
    chunking::FileChunking,
};
use crate::helper;
use crate::user;

#[derive(Debug)]
pub struct Directory {
    node: Node,
    property_cache: Option<HashMap<String, String>>,
    fileinfo_cache: Option<FileInfo>,
}

#[async_trait]
pub trait Collection: SabreNode {
    async fn create_file<R: Read + Send + Sync>(
        &self,
        name: &str,
        data: Option<R>,
    ) -> Result<Option<String>, SabreError>;
    
    async fn create_directory(&self, name: &str) -> Result<(), SabreError>;
    
    async fn get_child(&self, name: &str, info: Option<FileInfo>) -> Result<Box<dyn SabreNode + Send + Sync>, SabreError>;
    
    async fn get_children(&self) -> Result<Vec<Box<dyn SabreNode + Send + Sync>>, SabreError>;
    
    async fn child_exists(&self, name: &str) -> Result<bool, SabreError>;
    
    async fn delete(&self) -> Result<(), SabreError>;
}

#[async_trait]
pub trait Quota: SabreNode {
    async fn get_quota_info(&self) -> Result<(u64, u64), SabreError>;
}

#[derive(Debug, thiserror::Error)]
pub enum SabreError {
    #[error("Forbidden: {0}")]
    Forbidden(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Database error: {0}")]
    DbError(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

impl Directory {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            node: Node::new(path),
            property_cache: None,
            fileinfo_cache: None,
        }
    }
    
    pub fn set_property_cache(&mut self, cache: HashMap<String, String>) {
        self.property_cache = Some(cache);
    }
    
    pub fn set_fileinfo_cache(&mut self, info: FileInfo) {
        self.fileinfo_cache = Some(info);
    }
    
    async fn get_etag_property_for_path(&self, path: &str) -> Result<String, SabreError> {
        let info = filesystem::get_file_info(path).await
            .ok_or_else(|| SabreError::NotFound(format!("File info not found for path {}", path)))?;
        
        Ok(format!("\"{}\"", info.etag))
    }
}

#[async_trait]
impl SabreNode for Directory {
    fn get_name(&self) -> String {
        self.node.get_name()
    }
    
    fn get_path(&self) -> &str {
        self.node.get_path()
    }
    
    async fn get_properties(&self, properties: &[String]) -> Result<HashMap<String, String>, SabreError> {
        let mut props = self.node.get_properties(properties).await?;
        
        let etag_prop = Node::GETETAG_PROPERTYNAME.to_string();
        if properties.contains(&etag_prop) && !props.contains_key(&etag_prop) {
            props.insert(etag_prop, self.get_etag_property_for_path(self.get_path()).await?);
        }
        
        Ok(props)
    }
}

#[async_trait]
impl Collection for Directory {
    /// Creates a new file in the directory
    ///
    /// Data will either be supplied as a stream resource, or in certain cases
    /// as a string. Keep in mind that you may have to support either.
    ///
    /// After successful creation of the file, you may choose to return the ETag
    /// of the new file here.
    ///
    /// The returned ETag must be surrounded by double-quotes (The quotes should
    /// be part of the actual string).
    ///
    /// If you cannot accurately determine the ETag, you should not return it.
    /// If you don't store the file exactly as-is (you're transforming it
    /// somehow) you should also not return an ETag.
    ///
    /// This means that if a subsequent GET to this new file does not exactly
    /// return the same contents of what was submitted here, you are strongly
    /// recommended to omit the ETag.
    async fn create_file<R: Read + Send + Sync>(
        &self,
        name: &str,
        data: Option<R>,
    ) -> Result<Option<String>, SabreError> {
        if name == "Shared" && self.get_path().is_empty() {
            return Err(SabreError::Forbidden("Cannot create 'Shared' in root".to_string()));
        }

        // Check if this is a chunked upload
        let http_chunked = std::env::var("HTTP_OC_CHUNKED").is_ok();

        if http_chunked {
            // For chunked upload also updating an existing file is a "createFile"
            // because we create all the chunks before reassemble them to the existing file.
            let info = FileChunking::decode_name(name);
            let full_path = format!("{}/{}", self.get_path(), info.name);
            
            if !filesystem::is_creatable(self.get_path()).await && 
               !filesystem::is_updatable(&full_path).await {
                return Err(SabreError::Forbidden("Permission denied".to_string()));
            }
        } else {
            // For non-chunked upload it is enough to check if we can create a new file
            if !filesystem::is_creatable(self.get_path()).await {
                return Err(SabreError::Forbidden("Permission denied".to_string()));
            }
        }

        let path = format!("{}/{}", self.get_path(), name);
        let mut file = crate::connector::sabre::file::File::new(path);
        file.put(data).await
    }

    /// Creates a new subdirectory
    async fn create_directory(&self, name: &str) -> Result<(), SabreError> {
        if name == "Shared" && self.get_path().is_empty() {
            return Err(SabreError::Forbidden("Cannot create 'Shared' in root".to_string()));
        }

        if !filesystem::is_creatable(self.get_path()).await {
            return Err(SabreError::Forbidden("Permission denied".to_string()));
        }

        let new_path = format!("{}/{}", self.get_path(), name);
        if !filesystem::mkdir(&new_path).await {
            return Err(SabreError::Forbidden(format!("Could not create directory {}", new_path)));
        }

        Ok(())
    }

    /// Returns a specific child node, referenced by its name
    async fn get_child(&self, name: &str, info: Option<FileInfo>) -> Result<Box<dyn SabreNode + Send + Sync>, SabreError> {
        let path = format!("{}/{}", self.get_path(), name);
        let info = match info {
            Some(i) => i,
            None => filesystem::get_file_info(&path).await
                .ok_or_else(|| SabreError::NotFound(format!("File with name {} could not be located", path)))?
        };

        let node: Box<dyn SabreNode + Send + Sync> = if info.mimetype == "httpd/unix-directory" {
            let mut dir = Directory::new(path);
            dir.set_fileinfo_cache(info);
            Box::new(dir)
        } else {
            let mut file = crate::connector::sabre::file::File::new(path);
            file.set_fileinfo_cache(info);
            Box::new(file)
        };

        Ok(node)
    }

    /// Returns an array with all the child nodes
    async fn get_children(&self) -> Result<Vec<Box<dyn SabreNode + Send + Sync>>, SabreError> {
        let folder_content = filesystem::get_directory_content(self.get_path()).await;
        let mut paths = Vec::new();
        let mut properties = HashMap::new();
        
        for info in &folder_content {
            let path = format!("{}/{}", self.get_path(), info.name);
            paths.push(path.clone());
            
            let mut props = HashMap::new();
            props.insert(Node::GETETAG_PROPERTYNAME.to_string(), format!("\"{}\"", info.etag));
            properties.insert(path, props);
        }
        
        if !paths.is_empty() {
            // The number of arguments within IN conditions are limited in most databases
            // we chunk $paths into arrays of 200 items each to meet this criteria
            for chunk in paths.chunks(200) {
                let placeholders = (0..chunk.len()).map(|_| "?").collect::<Vec<_>>().join(",");
                let query = format!("SELECT * FROM `*PREFIX*properties` WHERE `userid` = ? AND `propertypath` IN ({})", placeholders);
                
                let mut params = Vec::new();
                params.push(user::get_user().await);
                params.extend(chunk.iter().cloned());
                
                let results = db::execute_query(&query, &params).await
                    .map_err(|e| SabreError::DbError(e.to_string()))?;
                
                for row in results {
                    let property_path = row.get::<String>("propertypath")
                        .ok_or_else(|| SabreError::DbError("Missing propertypath".to_string()))?;
                    let property_name = row.get::<String>("propertyname")
                        .ok_or_else(|| SabreError::DbError("Missing propertyname".to_string()))?;
                    let property_value = row.get::<String>("propertyvalue")
                        .ok_or_else(|| SabreError::DbError("Missing propertyvalue".to_string()))?;
                    
                    if property_name != Node::GETETAG_PROPERTYNAME {
                        properties.entry(property_path)
                            .or_insert_with(HashMap::new)
                            .insert(property_name, property_value);
                    }
                }
            }
        }

        let mut nodes = Vec::new();
        for info in folder_content {
            let mut node = self.get_child(&info.name, Some(info)).await?;
            
            let path = format!("{}/{}", self.get_path(), node.get_name());
            if let Some(props) = properties.get(&path) {
                if let Some(dir) = node.as_any().downcast_mut::<Directory>() {
                    dir.set_property_cache(props.clone());
                } else if let Some(file) = node.as_any().downcast_mut::<crate::connector::sabre::file::File>() {
                    file.set_property_cache(props.clone());
                }
            }
            
            nodes.push(node);
        }
        
        Ok(nodes)
    }

    /// Checks if a child exists.
    async fn child_exists(&self, name: &str) -> Result<bool, SabreError> {
        let path = format!("{}/{}", self.get_path(), name);
        Ok(filesystem::file_exists(&path).await)
    }

    /// Deletes all files in this directory, and then itself
    async fn delete(&self) -> Result<(), SabreError> {
        if self.get_path() == "Shared" {
            return Err(SabreError::Forbidden("Cannot delete Shared folder".to_string()));
        }

        if !filesystem::is_deletable(self.get_path()).await {
            return Err(SabreError::Forbidden("Permission denied".to_string()));
        }

        filesystem::rmdir(self.get_path()).await?;
        Ok(())
    }
}

#[async_trait]
impl Quota for Directory {
    /// Returns available diskspace information
    async fn get_quota_info(&self) -> Result<(u64, u64), SabreError> {
        let storage_info = helper::get_storage_info(self.get_path()).await;
        Ok((storage_info.used, storage_info.free))
    }
}