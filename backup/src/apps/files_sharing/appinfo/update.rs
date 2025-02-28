use std::collections::HashMap;
use std::sync::Arc;

use futures::stream::StreamExt;
use log::{error, warn};
use sqlx::{query, query_as, PgPool, Row};

use crate::config;
use crate::db;
use crate::errors::Error;
use crate::files::{FileMeta, FileType, FileView};
use crate::groups::{self, Group};
use crate::permissions::{Permission, PERMISSION_CREATE, PERMISSION_READ, PERMISSION_SHARE, PERMISSION_UPDATE};
use crate::share::{self, ShareType};
use crate::users::{self, User, UserBackend};
use crate::util;

pub async fn run_update(pool: &PgPool) -> Result<(), Error> {
    let installed_version = config::get_app_value("files_sharing", "installed_version").await?;
    
    if version_compare(&installed_version, "0.3", "lt") {
        let mut update_error = false;
        let rows = query("SELECT * FROM sharing")
            .fetch_all(pool)
            .await?;

        let mut group_shares: HashMap<String, HashMap<i64, bool>> = HashMap::new();
        
        // We need to set up user backends, otherwise creating the shares will fail with "because user does not exist"
        users::use_backend(Arc::new(UserBackend::Database));
        groups::use_backend(Arc::new(Group::Database));
        
        // Load authentication apps
        crate::app::load_apps(&["authentication"]).await?;
        
        let root_view = FileView::new("");
        
        for row in rows {
            let source: String = row.get("source");
            let meta = root_view.get_file_info(&source).await?;
            let item_source = meta.file_id;
            
            if item_source != -1 {
                let file = meta;
                let item_type = if file.mimetype == "httpd/unix-directory" {
                    FileType::Folder
                } else {
                    FileType::File
                };
                
                let permissions: u32 = row.get("permissions");
                let permissions = if permissions == 0 {
                    PERMISSION_READ | PERMISSION_SHARE
                } else {
                    let mut perms = PERMISSION_READ | PERMISSION_UPDATE | PERMISSION_SHARE;
                    if item_type == FileType::Folder {
                        perms |= PERMISSION_CREATE;
                    }
                    perms
                };
                
                let uid_shared_with: String = row.get("uid_shared_with");
                let pos = uid_shared_with.rfind('@');
                
                let (share_type, share_with) = if let Some(pos) = pos {
                    let group_name = &uid_shared_with[pos + 1..];
                    if groups::group_exists(group_name).await? {
                        let share_with = uid_shared_with[0..pos].to_string();
                        if group_shares.get(&share_with).and_then(|s| s.get(&item_source)).is_some() {
                            continue;
                        } else {
                            group_shares.entry(share_with.clone())
                                .or_insert_with(HashMap::new)
                                .insert(item_source, true);
                        }
                        (ShareType::Group, Some(share_with))
                    } else {
                        (ShareType::User, Some(uid_shared_with))
                    }
                } else if uid_shared_with == "public" {
                    (ShareType::Link, None)
                } else {
                    (ShareType::User, Some(uid_shared_with))
                };
                
                let uid_owner: String = row.get("uid_owner");
                users::set_user_id(&uid_owner).await?;
                
                // We need to setup the filesystem for the user, otherwise FileSystem::get_root will fail and break
                util::setup_fs(&uid_owner).await?;
                
                match share::share_item(
                    item_type,
                    item_source,
                    share_type,
                    share_with.as_deref(),
                    permissions,
                ).await {
                    Ok(_) => {},
                    Err(e) => {
                        update_error = true;
                        warn!(
                            "Upgrade Routine: Skipping sharing \"{}\" to \"{}\" (error is \"{}\")",
                            source,
                            share_with.unwrap_or_default(),
                            e
                        );
                    }
                }
                
                util::tear_down_fs().await?;
            }
        }
        
        users::set_user_id(None).await?;
        
        if update_error {
            error!("There were some problems upgrading the sharing of files");
        }
        
        // NOTE: Let's drop the table after more testing
        // query("DROP TABLE sharing").execute(pool).await?;
    }

    // Clean up oc_share table from files which no longer exist
    if version_compare(&installed_version, "0.3.5", "lt") {
        // Get all shares where the original file no longer exists
        let shares_found = query_as::<_, (i64,)>(
            "SELECT file_source FROM share 
             LEFT JOIN filecache ON file_source = filecache.fileid 
             WHERE filecache.fileid IS NULL AND share.item_type IN ('file', 'folder')"
        )
        .fetch_all(pool)
        .await?;

        // Delete those shares from the oc_share table
        if !shares_found.is_empty() {
            let del_array: Vec<String> = shares_found
                .iter()
                .map(|s| s.0.to_string())
                .collect();
                
            query(&format!(
                "DELETE FROM share WHERE file_source IN ({})",
                del_array.join(",")
            ))
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}

fn version_compare(a: &str, b: &str, operator: &str) -> bool {
    match operator {
        "lt" => version_lt(a, b),
        _ => false,
    }
}

fn version_lt(a: &str, b: &str) -> bool {
    let a_parts: Vec<u32> = a.split('.')
        .filter_map(|s| s.parse().ok())
        .collect();
    let b_parts: Vec<u32> = b.split('.')
        .filter_map(|s| s.parse().ok())
        .collect();

    for i in 0..std::cmp::max(a_parts.len(), b_parts.len()) {
        let a_val = a_parts.get(i).copied().unwrap_or(0);
        let b_val = b_parts.get(i).copied().unwrap_or(0);
        
        if a_val < b_val {
            return true;
        } else if a_val > b_val {
            return false;
        }
    }
    
    false
}