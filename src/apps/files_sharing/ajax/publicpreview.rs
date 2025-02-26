/*
 * Copyright (c) 2013 Georg Ehrke georg@ownCloud.com
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use actix_web::{error, get, web, HttpResponse, Result};
use log::{debug, warn};
use percent_encoding::percent_decode_str;
use std::path::Path;

#[derive(serde::Deserialize)]
struct PublicPreviewParams {
    file: Option<String>,
    x: Option<i32>,
    y: Option<i32>,
    scalingup: Option<bool>,
    t: Option<String>,
}

#[get("/publicpreview")]
async fn public_preview(
    query: web::Query<PublicPreviewParams>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    if !app_state.is_app_enabled("files_sharing") {
        return Ok(HttpResponse::Ok().finish());
    }

    let file = match &query.file {
        Some(f) => percent_decode_str(f)
            .decode_utf8()
            .map_err(|_| error::ErrorBadRequest("Invalid file parameter"))?
            .to_string(),
        None => String::new(),
    };

    let max_x = query.x.unwrap_or(36);
    let max_y = query.y.unwrap_or(36);
    let scaling_up = query.scalingup.unwrap_or(true);
    let token = match &query.t {
        Some(t) => t,
        None => {
            debug!("core-preview: No token parameter was passed");
            return Err(error::ErrorBadRequest("No token parameter was passed"));
        }
    };

    let linked_item = match app_state
        .share_manager
        .get_share_by_token(token)
        .await
    {
        Ok(item) => {
            if item.item_type != "file" && item.item_type != "folder" {
                debug!("core-preview: Passed token parameter is not valid");
                return Err(error::ErrorNotFound("Invalid share token"));
            }
            item
        }
        Err(_) => {
            debug!("core-preview: Passed token parameter is not valid");
            return Err(error::ErrorNotFound("Invalid share token"));
        }
    };

    if linked_item.uid_owner.is_empty() || linked_item.file_source == 0 {
        warn!(
            "core-preview: Passed token seems to be valid, but it does not contain all necessary information. (\"{}\")",
            token
        );
        return Err(error::ErrorInternalServerError("Invalid share data"));
    }

    let user_id = &linked_item.uid_owner;
    app_state.filesystem.setup_fs(user_id).await?;

    let path_id = linked_item.file_source;
    let path = app_state.filesystem.get_path(path_id).await?;
    let path_info = app_state.filesystem.get_file_info(&path).await?;
    let shared_file: String;

    if linked_item.item_type == "folder" {
        if !app_state.filesystem.is_valid_path(&file).await? {
            warn!(
                "core-preview: Passed filename is not valid, might be malicious (file:\"{}\";ip:\"{}\")",
                file,
                web::HttpRequest::connection_info(web::HttpRequest::new()).peer_addr().unwrap_or("unknown")
            );
            return Err(error::ErrorBadRequest("Invalid file path"));
        }
        shared_file = app_state.filesystem.normalize_path(&file).await?;
    } else {
        // item_type is "file"
        let parent = path_info.parent;
        let path_temp = app_state.filesystem.get_path(parent).await?;
        path = path_temp;
        shared_file = path_info.name;
    }

    let path = app_state.filesystem.normalize_path(&path, false).await?;
    let path = path.trim_start_matches('/');

    if max_x == 0 || max_y == 0 {
        debug!("core-preview: x and/or y set to 0");
        return Err(error::ErrorBadRequest("x and/or y set to 0"));
    }

    let root = format!("files/{}", path);

    match app_state
        .preview_manager
        .create_preview(
            user_id,
            &root,
            &shared_file,
            max_x,
            max_y,
            scaling_up,
        )
        .await
    {
        Ok(preview_data) => Ok(HttpResponse::Ok()
            .content_type(preview_data.mime_type)
            .body(preview_data.data)),
        Err(e) => {
            debug!("core: {}", e);
            Err(error::ErrorInternalServerError("Preview generation failed"))
        }
    }
}

// Required structures for the implementation

#[derive(Clone)]
pub struct AppState {
    share_manager: ShareManager,
    filesystem: FileSystem,
    preview_manager: PreviewManager,
}

impl AppState {
    pub fn is_app_enabled(&self, app_name: &str) -> bool {
        // Implementation would check if an app is enabled
        true // Placeholder
    }
}

#[derive(Clone)]
pub struct ShareManager {
    // Fields would go here
}

impl ShareManager {
    pub async fn get_share_by_token(&self, token: &str) -> Result<ShareItem, ShareError> {
        // Implementation would fetch share information by token
        Ok(ShareItem::default()) // Placeholder
    }
}

#[derive(Debug)]
pub enum ShareError {
    NotFound,
    // Other error types
}

#[derive(Default, Clone)]
pub struct ShareItem {
    pub item_type: String,
    pub uid_owner: String,
    pub file_source: i64,
    // Other fields
}

#[derive(Clone)]
pub struct FileSystem {
    // Fields would go here
}

impl FileSystem {
    pub async fn setup_fs(&self, user_id: &str) -> Result<(), FileSystemError> {
        // Implementation would set up filesystem for user
        Ok(()) // Placeholder
    }

    pub async fn get_path(&self, path_id: i64) -> Result<String, FileSystemError> {
        // Implementation would get path from ID
        Ok(String::new()) // Placeholder
    }

    pub async fn get_file_info(&self, path: &str) -> Result<FileInfo, FileSystemError> {
        // Implementation would get file info
        Ok(FileInfo::default()) // Placeholder
    }

    pub async fn is_valid_path(&self, path: &str) -> Result<bool, FileSystemError> {
        // Implementation would validate path
        Ok(true) // Placeholder
    }

    pub async fn normalize_path(&self, path: &str, keep_leading_slash: bool) -> Result<String, FileSystemError> {
        // Implementation would normalize path
        Ok(String::new()) // Placeholder
    }
}

#[derive(Debug)]
pub enum FileSystemError {
    NotFound,
    InvalidPath,
    // Other error types
}

#[derive(Default)]
pub struct FileInfo {
    pub parent: i64,
    pub name: String,
    // Other fields
}

#[derive(Clone)]
pub struct PreviewManager {
    // Fields would go here
}

impl PreviewManager {
    pub async fn create_preview(
        &self,
        user_id: &str,
        root: &str,
        file: &str,
        max_x: i32,
        max_y: i32,
        scaling_up: bool,
    ) -> Result<PreviewData, PreviewError> {
        // Implementation would generate preview
        Ok(PreviewData {
            data: vec![],
            mime_type: "image/png".to_string(),
        }) // Placeholder
    }
}

pub struct PreviewData {
    pub data: Vec<u8>,
    pub mime_type: String,
}

#[derive(Debug)]
pub enum PreviewError {
    GenerationFailed(String),
    // Other error types
}

impl std::fmt::Display for PreviewError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PreviewError::GenerationFailed(msg) => write!(f, "Preview generation failed: {}", msg),
        }
    }
}

impl std::error::Error for PreviewError {}