/**
 * Copyright (c) 2013 Georg Ehrke georg@ownCloud.com
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */
use actix_web::{
    get, web, HttpRequest, HttpResponse, Responder,
    http::StatusCode,
};
use actix_web::error::ErrorInternalServerError;
use log::{debug, error};
use percent_encoding::percent_decode_str;
use std::str::FromStr;

/// Request handler for version file previews
#[get("/apps/files_versions/ajax/preview")]
async fn preview(
    req: HttpRequest,
    app_state: web::Data<AppState>,
) -> impl Responder {
    // Check if user is logged in
    if !app_state.auth_service.check_logged_in(&req).await {
        return HttpResponse::Unauthorized().finish();
    }

    // Check if files_versions app is enabled
    if !app_state.app_service.is_enabled("files_versions").await {
        return HttpResponse::NotFound().finish();
    }

    // Extract and validate query parameters
    let query = req.query_string();
    let query_params = form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .collect::<std::collections::HashMap<String, String>>();

    let file = match query_params.get("file") {
        Some(value) => {
            let decoded = percent_decode_str(value)
                .decode_utf8()
                .unwrap_or_default()
                .to_string();
            decoded
        },
        None => String::new(),
    };

    let user = query_params.get("user")
        .map(|s| s.to_string())
        .unwrap_or_default();

    let max_x = query_params.get("x")
        .and_then(|s| i32::from_str(s).ok())
        .unwrap_or(44);

    let max_y = query_params.get("y")
        .and_then(|s| i32::from_str(s).ok())
        .unwrap_or(44);

    let version = query_params.get("version")
        .map(|s| s.to_string())
        .unwrap_or_default();

    let scaling_up = query_params.get("scalingup")
        .and_then(|s| bool::from_str(s).ok())
        .unwrap_or(true);

    // Validate parameters
    if user.is_empty() {
        debug!("versions-preview: No user parameter was passed");
        return HttpResponse::BadRequest().finish();
    }

    if file.is_empty() && version.is_empty() {
        debug!("versions-preview: No file parameter was passed");
        return HttpResponse::BadRequest().finish();
    }

    if max_x == 0 || max_y == 0 {
        debug!("versions-preview: x and/or y set to 0");
        return HttpResponse::BadRequest().finish();
    }

    // Generate and serve preview
    match generate_preview(&app_state, &user, &file, &version, max_x, max_y, scaling_up).await {
        Ok(preview_data) => {
            // Return preview with appropriate content type
            let content_type = preview_data.content_type.clone();
            HttpResponse::Ok()
                .content_type(content_type)
                .body(preview_data.data)
        },
        Err(e) => {
            error!("Failed to generate preview: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

struct PreviewData {
    data: Vec<u8>,
    content_type: String,
}

async fn generate_preview(
    app_state: &web::Data<AppState>, 
    user: &str, 
    file: &str, 
    version: &str, 
    max_x: i32, 
    max_y: i32, 
    scaling_up: bool
) -> Result<PreviewData, Box<dyn std::error::Error>> {
    let preview = app_state.preview_service
        .create_preview(user, "files_versions")
        .await?;
    
    preview.set_file(&format!("{}.v{}", file, version)).await?;
    preview.set_max_x(max_x).await?;
    preview.set_max_y(max_y).await?;
    preview.set_scaling_up(scaling_up).await?;

    preview.generate_preview().await
        .map_err(|e| {
            debug!("core: {}", e);
            ErrorInternalServerError(e).into()
        })
}

// These structs would be defined elsewhere in your actual application
struct AppState {
    auth_service: AuthService,
    app_service: AppService,
    preview_service: PreviewService,
}

struct AuthService;
impl AuthService {
    async fn check_logged_in(&self, _req: &HttpRequest) -> bool {
        // Implementation would check session authentication
        true
    }
}

struct AppService;
impl AppService {
    async fn is_enabled(&self, app_name: &str) -> bool {
        // Implementation would check if the app is enabled
        app_name == "files_versions"
    }
}

struct PreviewService;
impl PreviewService {
    async fn create_preview(&self, user: &str, app: &str) -> Result<Preview, Box<dyn std::error::Error>> {
        Ok(Preview {
            user: user.to_string(),
            app: app.to_string(),
            file: String::new(),
            max_x: 0,
            max_y: 0,
            scaling_up: true,
        })
    }
}

struct Preview {
    user: String,
    app: String,
    file: String,
    max_x: i32,
    max_y: i32,
    scaling_up: bool,
}

impl Preview {
    async fn set_file(&self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation would set the file path
        Ok(())
    }

    async fn set_max_x(&self, max_x: i32) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation would set the max X dimension
        Ok(())
    }

    async fn set_max_y(&self, max_y: i32) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation would set the max Y dimension
        Ok(())
    }

    async fn set_scaling_up(&self, scaling_up: bool) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation would set scaling up parameter
        Ok(())
    }

    async fn generate_preview(&self) -> Result<PreviewData, Box<dyn std::error::Error>> {
        // Implementation would generate preview image
        Ok(PreviewData {
            data: vec![],  // Would contain actual image data
            content_type: "image/png".to_string(),
        })
    }
}