// Copyright (c) 2013, Tom Needham <tom@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or later.
// See the COPYING-README file.

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use async_trait::async_trait;

#[async_trait]
pub trait Capabilities {
    async fn get_capabilities(&self) -> Result<impl Responder, Box<dyn std::error::Error>>;
}

pub struct FilesVersionsCapabilities;

#[async_trait]
impl Capabilities for FilesVersionsCapabilities {
    async fn get_capabilities(&self) -> Result<impl Responder, Box<dyn std::error::Error>> {
        // Implementation of capabilities retrieval
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "files_versions": {
                // Add capabilities data here
            }
        })))
    }
}

async fn preview_handler() -> Result<impl Responder, Box<dyn std::error::Error>> {
    // Equivalent to including preview.php
    // Implementation of preview generation logic
    let image_data = std::fs::read("path/to/preview.png")?;
    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(image_data))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/cloud/capabilities")
            .route(web::get().to(|data: web::Data<FilesVersionsCapabilities>| async move {
                data.get_capabilities().await.unwrap_or_else(|_| {
                    HttpResponse::InternalServerError().finish()
                })
            }))
    );
    
    cfg.service(
        web::resource("/preview.png")
            .route(web::get().to(preview_handler))
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(FilesVersionsCapabilities {}))
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}