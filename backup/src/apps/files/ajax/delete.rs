use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use std::path::Path;
use futures::{stream, StreamExt};

// Import necessary crates from our application
use crate::auth::{check_logged_in, check_call};
use crate::files::filesystem;
use crate::files::helpers::build_file_storage_statistics;
use crate::json::{success_response, error_response};

#[derive(Deserialize)]
pub struct DeleteRequest {
    dir: String,
    #[serde(default)]
    file: Option<String>,
    #[serde(default)]
    files: Option<String>,
}

#[derive(Serialize)]
pub struct DeleteResponse {
    dir: String,
    files: Vec<String>,
    #[serde(flatten)]
    storage_stats: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Serialize)]
pub struct DeleteErrorResponse {
    message: String,
    #[serde(flatten)]
    storage_stats: std::collections::HashMap<String, serde_json::Value>,
}

/// Handler for deleting files/directories
pub async fn delete(
    req: web::Form<DeleteRequest>,
    session: web::Data<crate::session::Session>,
) -> Result<HttpResponse, Error> {
    // Check auth status
    check_logged_in(&session)?;
    check_call(&session)?;
    
    // Get data
    let dir = req.dir.replace("\\", ""); // stripslashes equivalent
    
    // Determine which files to delete - either from 'file' or 'files' parameter
    let files_json = match &req.file {
        Some(file) => file.clone(),
        None => req.files.clone().unwrap_or_default(),
    };
    
    let files: Vec<String> = serde_json::from_str(&files_json)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid file list format"))?;
    
    let mut files_with_error = String::new();
    let mut success = true;
    
    // Now delete files
    for file in &files {
        if (dir.is_empty() && file == "Shared") || 
           !filesystem::unlink(&format!("{}/{}", dir, file)).await.unwrap_or(false) {
            files_with_error.push_str(file);
            files_with_error.push('\n');
            success = false;
        }
    }
    
    // Get array with updated storage stats after deletion
    let storage_stats = build_file_storage_statistics(&dir).await?;
    
    if success {
        Ok(success_response(&DeleteResponse {
            dir: dir.clone(),
            files: files.clone(),
            storage_stats,
        }))
    } else {
        Ok(error_response(&DeleteErrorResponse {
            message: format!("Could not delete:\n{}", files_with_error),
            storage_stats,
        }))
    }
}