// Copyright (C) 2013 Henrik Kjölhede <hkjolhede@gmail.com>
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

use crate::files::storage::Storage;
use crate::files::storage::sftp::SFTP as SFTPStorage;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use uuid::Uuid;
use anyhow::{Result, anyhow, bail};

#[derive(Debug, Clone, Deserialize)]
struct SftpConfig {
    run: bool,
    root: String,
    // Otros campos necesarios para la configuración
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
struct Config {
    sftp: SftpConfig,
    // Otros posibles backends
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

pub struct SFTP {
    config: Option<Config>,
    instance: Option<SFTPStorage>,
}

impl SFTP {
    pub fn new() -> Self {
        SFTP {
            config: None,
            instance: None,
        }
    }

    pub async fn set_up(&mut self) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        
        // Cargar la configuración desde un archivo
        let config_path = Path::new("files_external/tests/config.json");
        let config_str = fs::read_to_string(config_path)
            .map_err(|e| anyhow!("Failed to read config file: {}", e))?;
        
        let config: Config = serde_json::from_str(&config_str)
            .map_err(|e| anyhow!("Failed to parse config: {}", e))?;
        
        if !config.sftp.run {
            bail!("SFTP backend not configured");
        }
        
        // Crear una nueva instancia con configuración actualizada
        let mut sftp_config = config.sftp.clone();
        sftp_config.root = format!("{}/{}", sftp_config.root, id);
        
        let instance = SFTPStorage::new(sftp_config.extra.clone())
            .await
            .map_err(|e| anyhow!("Failed to create SFTP storage: {}", e))?;
        
        instance.mkdir("/").await?;
        
        self.config = Some(config);
        self.instance = Some(instance);
        
        Ok(())
    }

    pub async fn tear_down(&mut self) -> Result<()> {
        if let Some(instance) = &self.instance {
            instance.rmdir("/").await?;
        }
        
        self.instance = None;
        self.config = None;
        
        Ok(())
    }
}

impl Drop for SFTP {
    fn drop(&mut self) {
        if self.instance.is_some() {
            // Intentar limpiar en caso de que el tear_down no se haya llamado
            let runtime = tokio::runtime::Runtime::new().unwrap();
            let _ = runtime.block_on(self.tear_down());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sftp() -> Result<()> {
        let mut sftp = SFTP::new();
        if let Err(e) = sftp.set_up().await {
            if e.to_string().contains("SFTP backend not configured") {
                println!("Skipping test: SFTP backend not configured");
                return Ok(());
            }
            return Err(e);
        }
        
        // Aquí irían las pruebas reales
        
        sftp.tear_down().await?;
        Ok(())
    }
}