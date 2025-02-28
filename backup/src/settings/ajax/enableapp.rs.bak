use actix_web::{web, Error, HttpResponse};
use log::error;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::app::{App, AppError};
use crate::auth::{check_admin_user, check_request_token};

#[derive(Deserialize)]
pub struct EnableAppForm {
    appid: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    data: ErrorData,
}

#[derive(Serialize)]
struct ErrorData {
    message: String,
}

#[derive(Error, Debug)]
pub enum EnableAppError {
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("App error: {0}")]
    AppError(#[from] AppError),
}

/// Enable an app via AJAX request
///
/// Requires admin privileges and a valid request token
pub async fn enable_app(
    form: web::Form<EnableAppForm>,
    app_service: web::Data<App>,
) -> Result<HttpResponse, Error> {
    // Check if user is admin
    check_admin_user()?;
    
    // Verify CSRF token
    check_request_token()?;

    let clean_app_id = app_service.clean_app_id(&form.appid);
    
    match app_service.enable(&clean_app_id).await {
        Ok(_) => {
            // Return success response
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "status": "success"
            })))
        }
        Err(e) => {
            // Log error
            error!("Failed to enable app: {}", e);
            
            // Return error response
            Ok(HttpResponse::BadRequest().json(ErrorResponse {
                data: ErrorData {
                    message: e.to_string(),
                },
            }))
        }
    }
}