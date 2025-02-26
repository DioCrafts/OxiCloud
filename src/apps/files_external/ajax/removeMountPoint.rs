use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct RemoveMountPointRequest {
    isPersonal: String,
    mountPoint: String,
    mountType: String,
    applicable: String,
}

/// Handler for removing a mount point
async fn remove_mount_point(
    req: web::Json<RemoveMountPointRequest>,
    session: web::Data<Session>,
) -> impl Responder {
    // Check if the app is enabled
    if !app_is_enabled("files_external") {
        return HttpResponse::BadRequest().finish();
    }

    // Validate request parameters
    let is_personal = match req.isPersonal.as_str() {
        "true" => {
            // Check if user is logged in
            if !session.is_logged_in() {
                return HttpResponse::Unauthorized().finish();
            }
            true
        }
        _ => {
            // Check if user is admin
            if !session.is_admin() {
                return HttpResponse::Forbidden().finish();
            }
            false
        }
    };

    // Call mount point removal implementation
    match mount_config::remove_mount_point(
        &req.mountPoint,
        &req.mountType,
        &req.applicable,
        is_personal,
    ) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/apps/files_external/ajax/removeMountPoint.php")
            .route(web::post().to(remove_mount_point)),
    );
}

// These would be defined elsewhere in the application
mod mount_config {
    pub fn remove_mount_point(
        mount_point: &str,
        mount_type: &str,
        applicable: &str,
        is_personal: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation of OC_Mount_Config::removeMountPoint
        Ok(())
    }
}

trait Session {
    fn is_logged_in(&self) -> bool;
    fn is_admin(&self) -> bool;
}

fn app_is_enabled(app_name: &str) -> bool {
    // Implementation to check if an app is enabled
    true
}