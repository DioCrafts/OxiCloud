//! ownCloud
//!
//! @author Michael Gapczynski
//! @copyright 2012 Michael Gapczynski mtgap@owncloud.com
//!
//! This library is free software; you can redistribute it and/or
//! modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
//! License as published by the Free Software Foundation; either
//! version 3 of the License, or any later version.
//!
//! This library is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//!
//! You should have received a copy of the GNU Affero General Public
//! License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::error::Error;
use async_trait::async_trait;

use crate::db::DB;
use crate::share::{ShareBackend, ShareBackendCollection};

pub struct ShareBackendFolder {
    file_backend: Box<dyn ShareBackend>,
}

impl ShareBackendFolder {
    pub fn new(file_backend: Box<dyn ShareBackend>) -> Self {
        Self { file_backend }
    }
}

impl ShareBackend for ShareBackendFolder {
    // Implement the ShareBackend trait methods by delegating to file_backend
    // (methods would go here)
}

#[async_trait]
impl ShareBackendCollection for ShareBackendFolder {
    async fn get_children(&self, item_source: i64) -> Result<Vec<ChildInfo>, Box<dyn Error>> {
        let mut children = Vec::new();
        let mut parents = vec![item_source];
        
        // Get mimetype ID for directories
        let db = DB::get_instance().await?;
        let query = "SELECT `id` FROM `*PREFIX*mimetypes` WHERE `mimetype` = ?";
        let params = vec!["httpd/unix-directory"];
        
        let result = db.prepare_and_execute(query, &params).await?;
        let mimetype = if let Some(row) = result.first() {
            row.get::<i64>("id")?
        } else {
            -1
        };
        
        while !parents.is_empty() {
            // Convert parents to string representation for SQL
            let parents_str = parents
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<String>>()
                .join("','");
            
            let query = format!(
                "SELECT `fileid`, `name`, `mimetype` FROM `*PREFIX*filecache` WHERE `parent` IN ('{}')", 
                parents_str
            );
            
            let result = db.execute(&query).await?;
            parents.clear();
            
            for row in result {
                let file_id = row.get::<i64>("fileid")?;
                let name = row.get::<String>("name")?;
                let file_mimetype = row.get::<i64>("mimetype")?;
                
                children.push(ChildInfo {
                    source: file_id,
                    file_path: name,
                });
                
                // If a child folder is found look inside it
                if file_mimetype == mimetype {
                    parents.push(file_id);
                }
            }
        }
        
        Ok(children)
    }
}

pub struct ChildInfo {
    pub source: i64,
    pub file_path: String,
}