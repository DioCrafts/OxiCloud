/**
 * Copyright (c) 2013 Georg Ehrke georg@ownCloud.com
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::path::PathBuf;
use log::{debug, error};

#[derive(Deserialize)]
pub struct PreviewParams {
    file: Option<String>,
    x: Option<u32>,
    y: Option<u32>,
    scalingup: Option<bool>,
}

pub async fn handle_preview(
    query: web::Query<PreviewParams>,
    app_state: web::Data<crate::AppState>,
) -> impl Responder {
    // Check if user is logged in
    if !app_state.auth_service.is_logged_in() {
        return HttpResponse::Unauthorized().finish();
    }

    // Check if the trashbin app is enabled
    if !app_state.app_service.is_enabled("files_trashbin") {
        return HttpResponse::NotFound().finish();
    }

    // Extract parameters with defaults
    let file = match &query.file {
        Some(f) => percent_encoding::percent_decode_str(f)
            .decode_utf8()
            .unwrap_or_default()
            .to_string(),
        None => {
            debug!("core-preview: No file parameter was passed");
            return HttpResponse::BadRequest().finish();
        }
    };

    let max_x = query.x.unwrap_or(44);
    let max_y = query.y.unwrap_or(44);
    let scaling_up = query.scalingup.unwrap_or(true);

    if file.is_empty() {
        debug!("core-preview: No file parameter was passed");
        return HttpResponse::BadRequest().finish();
    }

    if max_x == 0 || max_y == 0 {
        debug!("core-preview: x and/or y set to 0");
        return HttpResponse::BadRequest().finish();
    }

    // Get current user
    let current_user = app_state.auth_service.get_current_user();
    
    match generate_preview(&current_user, &file, max_x, max_y, scaling_up, &app_state).await {
        Ok(preview_data) => {
            // Determine content type based on the preview_data
            let content_type = determine_content_type(&preview_data);
            
            HttpResponse::Ok()
                .content_type(content_type)
                .body(preview_data)
        }
        Err(e) => {
            error!("core: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn generate_preview(
    user: &str,
    file: &str,
    max_x: u32,
    max_y: u32,
    scaling_up: bool,
    app_state: &crate::AppState,
) -> Result<Vec<u8>, String> {
    let preview = app_state.preview_service.create_preview(user, "files_trashbin/files")?;
    
    preview.set_file(file)?;
    preview.set_max_x(max_x)?;
    preview.set_max_y(max_y)?;
    preview.set_scaling_up(scaling_up)?;

    preview.generate_preview()
}

fn determine_content_type(preview_data: &[u8]) -> &str {
    // Logic to determine content type based on preview data
    // This is a simplification, actual implementation would check file headers
    "image/png"
}

// This function would be registered in the main app configuration
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/apps/files_trashbin/ajax/preview")
            .route(web::get().to(handle_preview)),
    );
}