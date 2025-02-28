// ownCloud - ajax frontend
//
// This module handles file downloads through Ajax requests.
//
// Copyright 2010 Robin Appelman icewind1991@gmail.com
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

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::Value;
use std::vec::Vec;

use crate::auth::user;
use crate::files::file_service;
use crate::runtime::AppType;

#[derive(Deserialize)]
struct DownloadParams {
    files: String,
    dir: String,
}

/// Initialize the runtime with only filesystem apps
fn init_runtime() -> Result<(), Box<dyn std::error::Error>> {
    let app_types = vec![AppType::Filesystem];
    crate::runtime::init_with_app_types(app_types)?;
    Ok(())
}

/// Process download request
/// 
/// This function:
/// 1. Checks if user is logged in
/// 2. Parses the requested files
/// 3. Serves the files from the specified directory
pub async fn download(
    req: HttpRequest,
    query: web::Query<DownloadParams>,
) -> impl Responder {
    // Initialize runtime
    if let Err(e) = init_runtime() {
        return HttpResponse::InternalServerError().body(format!("Failed to initialize: {}", e));
    }

    // Check if user is logged in
    if !user::is_logged_in() {
        return HttpResponse::Unauthorized().finish();
    }

    // Parse files parameter - could be a JSON array or a single file
    let files_list: Vec<String> = match serde_json::from_str::<Value>(&query.files) {
        Ok(Value::Array(arr)) => arr.iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect(),
        _ => vec![query.files.clone()], // Fallback to treating it as a single file
    };

    // Determine if this is a HEAD request
    let is_head_request = req.method() == actix_web::http::Method::HEAD;

    // Get and serve the files
    match file_service::get(&query.dir, &files_list, is_head_request) {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(format!("Error serving files: {}", e)),
    }
}

// Register the download handler with the application router
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/apps/files/ajax/download")
            .route(web::get().to(download))
            .route(web::head().to(download)),
    );
}