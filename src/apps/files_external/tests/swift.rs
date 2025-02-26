/*
 * ownCloud
 *
 * @author Christian Berendt
 * @copyright 2013 Christian Berendt berendt@b1-systems.de
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU AFFERO GENERAL PUBLIC LICENSE for more details.
 *
 * You should have received a copy of the GNU Affero General Public
 * License along with this library.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::collections::HashMap;
use async_trait::async_trait;
use anyhow::{Result, bail, Context};

use crate::files::storage::Storage;
use crate::files::storage::swift::SwiftStorage;

pub struct Swift {
    config: Option<HashMap<String, HashMap<String, String>>>,
    instance: Option<SwiftStorage>,
}

impl Swift {
    pub fn new() -> Self {
        Self {
            config: None,
            instance: None,
        }
    }
    
    async fn load_config(&mut self) -> Result<()> {
        let config_path = "files_external/tests/config.rs";
        self.config = Some(include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", config_path)));
        Ok(())
    }
}

#[async_trait]
impl Storage for Swift {
    async fn set_up(&mut self) -> Result<()> {
        self.load_config().await?;
        
        let config = self.config.as_ref()
            .context("Config not loaded")?;
            
        let swift_config = config.get("swift")
            .context("Swift configuration not found")?;
            
        if swift_config.get("run").map_or(false, |v| v == "true") {
            let swift_storage = SwiftStorage::new(swift_config.clone());
            self.instance = Some(swift_storage);
            Ok(())
        } else {
            bail!("OpenStack Object Storage backend not configured");
        }
    }

    async fn tear_down(&mut self) -> Result<()> {
        if let Some(instance) = &self.instance {
            let connection = instance.get_connection()?;
            
            let swift_config = self.config.as_ref()
                .context("Config not loaded")?
                .get("swift")
                .context("Swift configuration not found")?;
                
            let bucket = swift_config.get("bucket")
                .context("Bucket not specified in configuration")?;
                
            let container = connection.container(bucket)?;
            
            let objects = container.object_list()?;
            for object in objects {
                object.delete().await?;
            }
            
            container.delete().await?;
        }
        
        Ok(())
    }
}