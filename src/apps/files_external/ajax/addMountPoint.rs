use actix_web::{post, web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use crate::core::json;
use crate::core::app;
use crate::core::auth;
use crate::files_external::mount_config;

#[derive(Deserialize)]
pub struct AddMountPointRequest {
    is_personal: String,
    mount_point: String,
    class: String,
    class_options: String,
    mount_type: String,
    applicable: String,
}

#[derive(Serialize)]
struct SuccessResponse {
    data: SuccessData,
}

#[derive(Serialize)]
struct SuccessData {
    message: String,
}

#[post("/apps/files_external/ajax/addMountPoint")]
pub async fn add_mount_point(
    req: web::Json<AddMountPointRequest>,
    session: web::Data<auth::Session>,
) -> Result<HttpResponse, Error> {
    // Check if the app is enabled
    app::check_app_enabled("files_external")?;
    
    // CSRF protection
    json::call_check()?;
    
    let is_personal = if req.is_personal == "true" {
        // Verify user is logged in
        auth::check_logged_in(&session)?;
        true
    } else {
        // Verify user is admin
        auth::check_admin_user(&session)?;
        false
    };
    
    // Add the mount point
    let status = mount_config::add_mount_point(
        &req.mount_point,
        &req.class,
        &req.class_options,
        &req.mount_type,
        &req.applicable,
        is_personal,
    )?;
    
    // Return success response
    let response = SuccessResponse {
        data: SuccessData {
            message: status,
        },
    };
    
    Ok(HttpResponse::Ok().json(response))
}