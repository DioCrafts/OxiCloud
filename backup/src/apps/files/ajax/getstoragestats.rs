use actix_web::{web, HttpResponse, Result};
use serde::Serialize;
use std::path::PathBuf;

use crate::auth::Authentication;
use crate::files::helper;

// Define response data type
#[derive(Serialize)]
struct JsonResponse {
    status: &'static str,
    data: helper::FileStorageStatistics,
}

/// Handle the storage statistics request
///
/// Only need filesystem apps
pub async fn get_storage_stats(
    query: web::Query<GetStorageStatsParams>,
    auth: Authentication,
) -> Result<HttpResponse> {
    // Verify user is logged in
    if !auth.is_logged_in() {
        return Ok(HttpResponse::Unauthorized().json(
            serde_json::json!({"status": "error", "message": "Not logged in"})
        ));
    }

    // Get the directory from query or use default
    let dir = &query.dir.clone().unwrap_or_else(|| "/".to_string());
    
    // Build file storage statistics
    let stats = helper::build_file_storage_statistics(dir).await?;
    
    // Send back json
    Ok(HttpResponse::Ok().json(JsonResponse {
        status: "success",
        data: stats,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct GetStorageStatsParams {
    dir: Option<String>,
}