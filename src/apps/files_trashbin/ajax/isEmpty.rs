use actix_web::{error, post, web, Error, HttpResponse, Responder};
use serde::Serialize;
use trashbin::Trashbin;
use users::User;

/// Response structure for trash bin status check
#[derive(Serialize)]
struct TrashStatusResponse {
    data: TrashStatusData,
}

#[derive(Serialize)]
struct TrashStatusData {
    is_empty: bool,
}

/// Check if trash bin is empty to re-enable the deleted files button if needed
#[post("/is_empty")]
async fn is_empty(
    user: web::ReqData<User>,
    trashbin: web::Data<Trashbin>,
    csrf_token: web::Header<String>,
) -> Result<impl Responder, Error> {
    // Check CSRF token
    if !csrf_token_valid(&csrf_token) {
        return Err(error::ErrorForbidden("Invalid CSRF token"));
    }

    // Check if trash bin is empty for current user
    let trash_status = trashbin.is_empty(user.id()).await
        .map_err(|e| error::ErrorInternalServerError(format!("Failed to check trash: {}", e)))?;

    // Return success response with status
    let response = TrashStatusResponse {
        data: TrashStatusData {
            is_empty: trash_status,
        },
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Validates the CSRF token
fn csrf_token_valid(token: &web::Header<String>) -> bool {
    // Implementation of CSRF token validation would go here
    // This would need to be implemented according to your application's security requirements
    true // Placeholder
}

// Register the endpoint with your application
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(is_empty);
}