//! Copyright (c) 2013 Georg Ehrke georg@ownCloud.com
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.

use actix_web::{web, HttpResponse, ResponseError};
use serde::Deserialize;
use std::fmt;
use log::debug;

#[derive(Debug)]
struct PreviewError {
    message: String,
    status_code: actix_web::http::StatusCode,
}

impl fmt::Display for PreviewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ResponseError for PreviewError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.status_code
    }
}

#[derive(Deserialize)]
struct PreviewParams {
    file: Option<String>,
    x: Option<i32>,
    y: Option<i32>,
    scalingup: Option<bool>,
}

async fn handle_preview(
    query: web::Query<PreviewParams>,
    user_service: web::Data<dyn UserService>,
) -> Result<HttpResponse, PreviewError> {
    // Check if user is logged in
    if !user_service.is_logged_in() {
        return Err(PreviewError {
            message: "User not logged in".to_string(),
            status_code: actix_web::http::StatusCode::UNAUTHORIZED,
        });
    }

    let file = match &query.file {
        Some(f) => urlencoding::decode(f)
            .map_err(|_| PreviewError {
                message: "Failed to decode file parameter".to_string(),
                status_code: actix_web::http::StatusCode::BAD_REQUEST,
            })?
            .into_owned(),
        None => {
            debug!("core-preview: No file parameter was passed");
            return Err(PreviewError {
                message: "No file parameter was passed".to_string(),
                status_code: actix_web::http::StatusCode::BAD_REQUEST,
            });
        }
    };

    let max_x = query.x.unwrap_or(36);
    let max_y = query.y.unwrap_or(36);
    let scaling_up = query.scalingup.unwrap_or(true);

    if max_x == 0 || max_y == 0 {
        debug!("core-preview: x and/or y set to 0");
        return Err(PreviewError {
            message: "x and/or y set to 0".to_string(),
            status_code: actix_web::http::StatusCode::BAD_REQUEST,
        });
    }

    // Get current user
    let username = user_service.get_current_user().ok_or(PreviewError {
        message: "Could not determine current user".to_string(),
        status_code: actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    // Generate preview
    let preview_service = Preview::new(&username, "files");
    
    match preview_service
        .set_file(&file)
        .set_max_x(max_x)
        .set_max_y(max_y)
        .set_scaling_up(scaling_up)
        .generate()
    {
        Ok(image_data) => Ok(HttpResponse::Ok()
            .content_type("image/png")  // Assume PNG, but should be determined by the preview generator
            .body(image_data)),
        Err(e) => {
            debug!("core: {}", e);
            Err(PreviewError {
                message: format!("Failed to generate preview: {}", e),
                status_code: actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            })
        }
    }
}

// Mock interfaces for the required services
trait UserService: Send + Sync {
    fn is_logged_in(&self) -> bool;
    fn get_current_user(&self) -> Option<String>;
}

struct Preview {
    user: String,
    type_str: String,
    file: Option<String>,
    max_x: i32,
    max_y: i32,
    scaling_up: bool,
}

impl Preview {
    pub fn new(user: &str, type_str: &str) -> Self {
        Self {
            user: user.to_string(),
            type_str: type_str.to_string(),
            file: None,
            max_x: 36,
            max_y: 36,
            scaling_up: true,
        }
    }

    pub fn set_file(mut self, file: &str) -> Self {
        self.file = Some(file.to_string());
        self
    }

    pub fn set_max_x(mut self, max_x: i32) -> Self {
        self.max_x = max_x;
        self
    }

    pub fn set_max_y(mut self, max_y: i32) -> Self {
        self.max_y = max_y;
        self
    }

    pub fn set_scaling_up(mut self, scaling_up: bool) -> Self {
        self.scaling_up = scaling_up;
        self
    }

    pub fn generate(self) -> Result<Vec<u8>, String> {
        // Implementation would generate the actual preview
        // This is a mock implementation
        if self.file.is_none() {
            return Err("No file specified".to_string());
        }
        
        // In a real implementation, this would generate the image
        // and return the binary data
        Ok(vec![]) // Return empty vector as placeholder
    }
}

// Function to configure and add this handler to an Actix web app
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/preview").route(web::get().to(handle_preview)));
}