use actix_web::{web, HttpResponse, Responder};
use mime::Mime;
use std::str::FromStr;

/// Returns the icon for a specific mime type
async fn mimetype_icon(query: web::Query<MimeQuery>) -> impl Responder {
    match get_mimetype_icon(&query.mime) {
        Ok(icon) => HttpResponse::Ok().body(icon),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

/// Query parameters for mimetype icon request
#[derive(serde::Deserialize)]
struct MimeQuery {
    mime: String,
}

/// Retrieves icon for the specified mime type
fn get_mimetype_icon(mime_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Parse the mime type
    let mime = Mime::from_str(mime_str)?;
    
    // Call to equivalent of OC_Helper::mimetypeIcon
    // This is a placeholder - actual implementation would depend on how 
    // the helper is structured in the Rust codebase
    let icon = crate::helpers::mimetype_icon(&mime)?;
    
    Ok(icon)
}

// In your main.rs or routes configuration:
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/apps/files/ajax/mimeicon", web::get().to(mimetype_icon));
}