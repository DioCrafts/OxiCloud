use actix_web::{web, HttpResponse, Result};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::cmp::Ordering;
use std::collections::HashSet;

use crate::auth::check_logged_in;
use crate::filesystem::{Filesystem, File, PreviewManager};
use crate::util::format_date;
use crate::files::helper::determine_icon;

#[derive(Deserialize)]
struct RawListQuery {
    dir: Option<String>,
    mimetypes: Option<String>,
}

#[derive(Serialize)]
struct RawListResponse {
    data: Vec<FileInfo>,
}

#[derive(Serialize, Clone)]
struct FileInfo {
    #[serde(flatten)]
    file: File,
    directory: String,
    is_preview_available: bool,
    date: String,
    mimetype_icon: String,
}

pub async fn raw_list(
    query: web::Query<RawListQuery>,
    filesystem: web::Data<Filesystem>,
    preview_manager: web::Data<PreviewManager>,
) -> Result<HttpResponse> {
    // Check if user is logged in
    check_logged_in()?;

    // Load the files
    let dir = query.dir.clone().unwrap_or_else(|| "".to_string());
    
    // Parse mimetypes
    let mut mimetypes: Vec<String> = Vec::new();
    if let Some(ref mimetype_str) = query.mimetypes {
        if let Ok(parsed) = serde_json::from_str::<Vec<String>>(mimetype_str) {
            mimetypes = parsed;
        } else {
            // Handle non-array requests
            mimetypes.push(mimetype_str.clone());
        }
    }
    
    // Clean up duplicates from array
    let mimetypes: HashSet<String> = mimetypes.into_iter().collect();
    
    // make filelist
    let mut files: Vec<FileInfo> = Vec::new();
    
    // If a type other than directory is requested first load them.
    if !mimetypes.is_empty() && !mimetypes.contains("httpd/unix-directory") {
        for file in filesystem.get_directory_content(&dir, "httpd/unix-directory").await? {
            let file_info = FileInfo {
                file: file.clone(),
                directory: dir.clone(),
                is_preview_available: preview_manager.is_mime_supported(&file.mimetype),
                date: format_date(file.mtime),
                mimetype_icon: determine_icon(&file),
            };
            files.push(file_info);
        }
    }

    if !mimetypes.is_empty() {
        for mimetype in &mimetypes {
            for file in filesystem.get_directory_content(&dir, mimetype).await? {
                let file_info = FileInfo {
                    file: file.clone(),
                    directory: dir.clone(),
                    is_preview_available: preview_manager.is_mime_supported(&file.mimetype),
                    date: format_date(file.mtime),
                    mimetype_icon: determine_icon(&file),
                };
                files.push(file_info);
            }
        }
    } else {
        for file in filesystem.get_directory_content_all(&dir).await? {
            let file_info = FileInfo {
                file: file.clone(),
                directory: dir.clone(),
                is_preview_available: preview_manager.is_mime_supported(&file.mimetype),
                date: format_date(file.mtime),
                mimetype_icon: determine_icon(&file),
            };
            files.push(file_info);
        }
    }

    // Sort by name
    files.sort_by(|a, b| {
        if a.file.name == b.file.name {
            Ordering::Equal
        } else if a.file.name < b.file.name {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": files
    })))
}