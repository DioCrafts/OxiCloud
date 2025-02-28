/**
 * Copyright (c) 2012 Robin Appelman <icewind@owncloud.com>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use std::collections::HashMap;
use uuid::Uuid;

use crate::files::storage::Storage;
use crate::files::storage::dropbox::DropboxStorage;

#[derive(Default)]
pub struct Dropbox {
    config: Option<HashMap<String, HashMap<String, String>>>,
    instance: Option<DropboxStorage>,
}

impl Dropbox {
    pub fn new() -> Self {
        Self::default()
    }
}

impl crate::test::TestCase for Dropbox {
    async fn set_up(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let id = Uuid::new_v4().to_string();
        self.config = match include!("files_external/tests/config.rs") {
            config => Some(config),
        };

        if self.config.is_none() 
            || !self.config.as_ref().unwrap().contains_key("dropbox") 
            || self.config.as_ref().unwrap()["dropbox"].get("run").map_or(false, |v| v == "false") {
            return Err("Dropbox backend not configured".into());
        }

        let mut dropbox_config = self.config.as_ref().unwrap()["dropbox"].clone();
        dropbox_config.insert(
            "root".to_string(), 
            format!("{}/{}", dropbox_config.get("root").unwrap_or(&String::new()), id)
        );
        
        self.instance = Some(DropboxStorage::new(dropbox_config));
        Ok(())
    }

    async fn tear_down(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(instance) = &self.instance {
            instance.unlink("/").await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::{StorageTestSuite, TestCase};

    #[tokio::test]
    async fn test_dropbox_storage() -> Result<(), Box<dyn std::error::Error>> {
        let mut test_case = Dropbox::new();
        
        match test_case.set_up().await {
            Ok(_) => {
                let result = StorageTestSuite::run(&mut test_case).await;
                test_case.tear_down().await?;
                result
            },
            Err(e) => {
                if e.to_string() == "Dropbox backend not configured" {
                    // Skip test
                    println!("Test skipped: {}", e);
                    Ok(())
                } else {
                    Err(e)
                }
            }
        }
    }
}