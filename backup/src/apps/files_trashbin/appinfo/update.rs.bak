use std::cmp::Ordering;
use thiserror::Error;

use crate::config::Config;
use crate::db::DbConnection;

#[derive(Error, Debug)]
pub enum UpdateError {
    #[error("Failed to get app value: {0}")]
    ConfigError(String),
    
    #[error("Database operation failed: {0}")]
    DbError(String),
}

pub async fn run_updates(config: &Config, db: &DbConnection) -> Result<(), UpdateError> {
    let installed_version = config
        .get_app_value("files_trashbin", "installed_version")
        .map_err(|e| UpdateError::ConfigError(e.to_string()))?;
    
    // If installed version is less than 0.4
    if version_compare(&installed_version, "0.4") == Ordering::Less {
        // Size of the trash bin could be incorrect, remove it for all users to
        // enforce a recalculation during next usage.
        db.execute("DELETE FROM `*PREFIX*files_trashsize`")
            .await
            .map_err(|e| UpdateError::DbError(e.to_string()))?;
    }
    
    Ok(())
}

/// Compare two version strings
fn version_compare(version1: &str, version2: &str) -> Ordering {
    let v1_parts: Vec<u32> = version1
        .split('.')
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .collect();
    
    let v2_parts: Vec<u32> = version2
        .split('.')
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .collect();
    
    for (i, v1_part) in v1_parts.iter().enumerate() {
        if i >= v2_parts.len() {
            return Ordering::Greater;
        }
        
        match v1_part.cmp(&v2_parts[i]) {
            Ordering::Equal => continue,
            other => return other,
        }
    }
    
    if v1_parts.len() < v2_parts.len() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}