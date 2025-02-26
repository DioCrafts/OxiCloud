// Copyright (C) 2012 Michael Gapczynski mtgap@owncloud.com
// Copyright (C) 2013 Christian Berendt berendt@b1-systems.de
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
use std::path::Path;
use std::time::Duration;
use std::thread;
use std::fs;
use serde_json::Value;
use anyhow::{Result, bail, Context};
use async_trait::async_trait;
use rusoto_core::Region;
use rusoto_s3::{S3Client, S3, ListObjectsRequest, DeleteObjectRequest, DeleteBucketRequest};

mod storage;
use storage::Storage;

pub struct AmazonS3 {
    config: HashMap<String, Value>,
    instance: Option<Box<dyn Storage>>,
}

impl AmazonS3 {
    pub fn new() -> Self {
        AmazonS3 {
            config: HashMap::new(),
            instance: None,
        }
    }

    pub async fn set_up(&mut self) -> Result<()> {
        // Load config from file
        let config_path = Path::new("files_external/tests/config.json");
        let config_content = fs::read_to_string(config_path)
            .context("Failed to read config file")?;
        
        let config: HashMap<String, Value> = serde_json::from_str(&config_content)
            .context("Failed to parse config file")?;
        
        self.config = config;
        
        // Check if AmazonS3 is configured and should run
        if !self.config.contains_key("amazons3") {
            bail!("AmazonS3 backend not configured");
        }
        
        let amazons3_config = self.config.get("amazons3")
            .and_then(|v| v.as_object())
            .context("Invalid amazons3 config format")?;
        
        let should_run = amazons3_config.get("run")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        if !should_run {
            bail!("AmazonS3 backend not configured to run");
        }
        
        // Initialize the S3 storage instance
        // Note: This is a placeholder - actual implementation would depend on the Storage trait
        self.instance = Some(Box::new(S3Storage::new(amazons3_config.clone())));
        
        Ok(())
    }
    
    pub async fn tear_down(&mut self) -> Result<()> {
        if let Some(instance) = &self.instance {
            let connection = instance.get_connection().await?;
            let s3_client = connection.downcast_ref::<S3Client>()
                .context("Failed to get S3Client from connection")?;
            
            let bucket = self.config.get("amazons3")
                .and_then(|v| v.get("bucket"))
                .and_then(|v| v.as_str())
                .context("Failed to get bucket from config")?;
            
            // List and delete all objects in the bucket
            match list_and_delete_objects(s3_client, bucket).await {
                Ok(_) => (),
                Err(e) => eprintln!("Error clearing bucket: {}", e),
            }
            
            // Delete the bucket
            match s3_client.delete_bucket(DeleteBucketRequest {
                bucket: bucket.to_string(),
                ..Default::default()
            }).await {
                Ok(_) => (),
                Err(e) => eprintln!("Error deleting bucket: {}", e),
            }
            
            // Wait for replication to complete
            thread::sleep(Duration::from_secs(30));
        }
        
        Ok(())
    }
}

// This is a placeholder implementation for S3Storage
struct S3Storage {
    config: serde_json::Map<String, Value>,
}

impl S3Storage {
    fn new(config: serde_json::Map<String, Value>) -> Self {
        S3Storage { config }
    }
    
    async fn get_connection(&self) -> Result<Box<dyn std::any::Any>> {
        // Create and return an S3Client instance
        let region = Region::default();
        let client = S3Client::new(region);
        Ok(Box::new(client))
    }
}

#[async_trait]
impl Storage for S3Storage {
    // Implement required Storage trait methods
    // This is a placeholder - actual implementation depends on Storage trait definition
}

async fn list_and_delete_objects(client: &S3Client, bucket: &str) -> Result<()> {
    let list_req = ListObjectsRequest {
        bucket: bucket.to_string(),
        ..Default::default()
    };
    
    let objects = client.list_objects(list_req).await?;
    
    if let Some(contents) = objects.contents {
        for object in contents {
            if let Some(key) = object.key {
                let delete_req = DeleteObjectRequest {
                    bucket: bucket.to_string(),
                    key,
                    ..Default::default()
                };
                
                client.delete_object(delete_req).await?;
            }
        }
    }
    
    Ok(())
}