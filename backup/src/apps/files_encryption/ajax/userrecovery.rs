// Script to handle admin settings for encrypted key recovery

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

use crate::apps::files_encryption::util::Util;
use crate::core::auth::Auth;
use crate::core::filesystem::FilesystemView;
use crate::core::json::{JsonError, JsonResponse};

#[derive(Error, Debug)]
pub enum UserRecoveryError {
    #[error("User not logged in")]
    NotLoggedIn,
    #[error("App 'files_encryption' not enabled")]
    AppNotEnabled,
    #[error("Invalid recovery setting value")]
    InvalidRecoverySetting,
    #[error("Failed to set recovery for user: {0}")]
    SetRecoveryFailed(String),
    #[error("Failed to add recovery keys: {0}")]
    AddRecoveryKeysFailed(String),
    #[error("Failed to remove recovery keys: {0}")]
    RemoveRecoveryKeysFailed(String),
}

#[derive(Deserialize)]
pub struct UserRecoveryRequest {
    user_enable_recovery: String,
}

pub async fn user_recovery(
    req: web::Json<UserRecoveryRequest>,
    auth: web::Data<Arc<dyn Auth>>,
    app_manager: web::Data<Arc<dyn AppManager>>,
) -> impl Responder {
    // Check if user is logged in
    if !auth.is_logged_in().await {
        return HttpResponse::Unauthorized().json(JsonError::new("Not logged in"));
    }

    // Check if app is enabled
    if !app_manager.is_app_enabled("files_encryption").await {
        return HttpResponse::BadRequest().json(JsonError::new("App not enabled"));
    }

    let user_enable_recovery = match req.user_enable_recovery.as_str() {
        "0" => false,
        "1" => true,
        _ => {
            return HttpResponse::BadRequest().json(JsonError::new("Invalid recovery setting value"));
        }
    };

    let user_id = auth.get_user().await;
    let view = FilesystemView::new("/");
    let util = Util::new(view, user_id);

    // Save recovery preference to DB
    match util.set_recovery_for_user(user_enable_recovery).await {
        Ok(true) => {
            if user_enable_recovery {
                match util.add_recovery_keys().await {
                    Ok(_) => HttpResponse::Ok().json(JsonResponse::success()),
                    Err(e) => HttpResponse::InternalServerError().json(JsonError::new(&e.to_string())),
                }
            } else {
                match util.remove_recovery_keys().await {
                    Ok(_) => HttpResponse::Ok().json(JsonResponse::success()),
                    Err(e) => HttpResponse::InternalServerError().json(JsonError::new(&e.to_string())),
                }
            }
        }
        Ok(false) => HttpResponse::InternalServerError().json(JsonError::new("Failed to set recovery")),
        Err(e) => HttpResponse::InternalServerError().json(JsonError::new(&e.to_string())),
    }
}

// Register the route
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/apps/files_encryption/ajax/userrecovery.php")
            .route(web::post().to(user_recovery)),
    );
}