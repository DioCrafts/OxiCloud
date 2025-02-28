// Copyright (c) 2012 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use actix_web::{web, HttpResponse, Responder};
use regex::Regex;
use std::path::Path;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Configure download route with regex pattern to match any file path
    cfg.route(
        "/download{file:.*}",
        web::get().to(download_file),
    );

    // Register with the capabilities API
    cfg.route(
        "/cloud/capabilities",
        web::get().to(get_capabilities),
    );
}

async fn download_file(path: web::Path<String>) -> impl Responder {
    let file_path = path.into_inner();
    
    // Include the file download logic that would have been in files/download.php
    // This is a placeholder for the actual implementation
    match include_download_logic(&file_path).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(format!("Download failed: {}", e)),
    }
}

async fn include_download_logic(file_path: &str) -> Result<HttpResponse, std::io::Error> {
    // Placeholder for the actual implementation of the download.php logic
    // In a real implementation, you would handle file reading, headers, etc.
    // TODO: Implement actual file download logic

    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .append_header(("Content-Disposition", format!("attachment; filename=\"{}\"", 
            Path::new(file_path).file_name().unwrap_or_default().to_string_lossy())))
        .body("File content would go here"))
}

async fn get_capabilities() -> impl Responder {
    // Placeholder for the OCA\Files\Capabilities::getCapabilities implementation
    let capabilities = files_capabilities::get_capabilities();
    
    HttpResponse::Ok().json(capabilities)
}

mod files_capabilities {
    use serde::Serialize;

    #[derive(Serialize)]
    struct Capabilities {
        files: FilesCapabilities,
    }

    #[derive(Serialize)]
    struct FilesCapabilities {
        // Define the actual capabilities structure based on the PHP implementation
        version: String,
        features: Vec<String>,
    }

    pub fn get_capabilities() -> Capabilities {
        Capabilities {
            files: FilesCapabilities {
                version: "1.0.0".to_string(),
                features: vec![
                    "favorites".to_string(),
                    "file_sharing".to_string(),
                ],
            },
        }
    }
}