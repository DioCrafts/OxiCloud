use actix_web::{web, HttpResponse, Responder, post};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::auth::check_admin_user;
use crate::app::App;
use crate::json::JsonResponse;

#[derive(Debug, Error)]
enum DisableAppError {
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("CSRF token validation failed")]
    CsrfError,
    
    #[error("Application error: {0}")]
    AppError(String),
}

#[derive(Deserialize)]
struct DisableAppRequest {
    appid: String,
}

#[derive(Serialize)]
struct SuccessResponse {
    status: &'static str,
}

#[post("/settings/ajax/disableapp")]
async fn disable_app(
    req: web::Json<DisableAppRequest>,
    session: web::ReqData<crate::session::Session>,
) -> impl Responder {
    // Check admin user permissions
    if let Err(e) = check_admin_user(&session) {
        return HttpResponse::Forbidden().json(JsonResponse::error("Not an admin user"));
    }
    
    // CSRF token validation (equivalent to OCP\JSON::callCheck())
    if !crate::json::csrf_check_token(&session) {
        return HttpResponse::Forbidden().json(JsonResponse::error("CSRF check failed"));
    }
    
    // Clean and disable the app
    let clean_app_id = App::clean_app_id(&req.appid);
    match App::disable(&clean_app_id) {
        Ok(_) => HttpResponse::Ok().json(JsonResponse::success()),
        Err(e) => HttpResponse::InternalServerError().json(JsonResponse::error(&e.to_string())),
    }
}

// Register this handler in the main application
pub fn configure(config: &mut web::ServiceConfig) {
    config.service(disable_app);
}