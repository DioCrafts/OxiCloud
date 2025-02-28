use log::{error, info};
use std::path::{Path, PathBuf};
use futures::StreamExt;
use sqlx::{query, Pool, Sqlite};
use std::cmp::Ordering;
use std::fs;

/// Updates for files application version 1.1.6 and earlier
pub async fn update(pool: &Pool<Sqlite>, server_root: &Path) -> anyhow::Result<()> {
    // fix webdav properties, add namespace in front of the property, update for OC4.5
    let installed_version = config::get_app_value("files", "installed_version").await?;

    if version_compare(&installed_version, "1.1.6") == Ordering::Less {
        update_webdav_properties(pool).await?;
    }

    // update from OC 3
    // try to remove remaining files.
    // Give a warning if not possible
    cleanup_old_files(server_root).await?;

    Ok(())
}

/// Adds DAV namespace to property names that don't have a namespace
async fn update_webdav_properties(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    let rows = query("SELECT propertyname, propertypath, userid FROM properties")
        .fetch_all(pool)
        .await?;

    for row in rows {
        let property_name: String = row.get("propertyname");
        let user_id: String = row.get("userid");
        let property_path: String = row.get("propertypath");

        if !property_name.starts_with('{') {
            let new_property_name = format!("{{DAV:}}{}", property_name);
            
            query("UPDATE properties SET propertyname = ? WHERE userid = ? AND propertypath = ?")
                .bind(new_property_name)
                .bind(user_id)
                .bind(property_path)
                .execute(pool)
                .await?;
        }
    }

    Ok(())
}

/// Attempts to remove old files directory structure
async fn cleanup_old_files(server_root: &Path) -> anyhow::Result<()> {
    let files_to_remove = vec![
        "ajax",
        "appinfo",
        "css",
        "js",
        "l10n",
        "templates",
        "admin.php",
        "download.php",
        "index.php",
        "settings.php"
    ];

    let files_dir = server_root.join("files");

    for file in files_to_remove {
        let file_path = files_dir.join(file);
        
        if !file_path.exists() {
            continue;
        }

        match remove_dir_recursive(&file_path) {
            Ok(_) => {},
            Err(e) => {
                // Probably not sufficient privileges, give up and give a message
                error!(
                    "Could not clean /files/ directory. Please remove everything except webdav.php from {}. Error: {}", 
                    files_dir.display(), 
                    e
                );
                break;
            }
        }
    }

    Ok(())
}

/// Recursively removes a directory or file
fn remove_dir_recursive(path: &PathBuf) -> std::io::Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

/// Compares two version strings
/// Returns Ordering::Less if version1 < version2
/// Returns Ordering::Equal if version1 == version2
/// Returns Ordering::Greater if version1 > version2
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

/// Config module for accessing app configuration
mod config {
    use sqlx::{query_scalar, Pool, Sqlite};

    pub async fn get_app_value(app: &str, key: &str) -> anyhow::Result<String> {
        // This is a simplified version - in a real app this would likely
        // query a database or config file
        let pool = get_db_pool().await?;
        
        let value = query_scalar("SELECT configvalue FROM appconfig WHERE appid = ? AND configkey = ?")
            .bind(app)
            .bind(key)
            .fetch_optional(&pool)
            .await?
            .unwrap_or_default();
        
        Ok(value)
    }

    async fn get_db_pool() -> anyhow::Result<Pool<Sqlite>> {
        // In a real application, this would be properly initialized elsewhere
        // and likely passed around rather than created on demand
        sqlx::SqlitePool::connect("sqlite:data.db").await.map_err(Into::into)
    }
}

} // Añadido por reparador automático