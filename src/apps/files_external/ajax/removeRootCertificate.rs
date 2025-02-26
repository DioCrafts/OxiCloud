use actix_web::{post, web, HttpResponse, Error};
use nextcloud_app_lib::{AppManager, UserSession, Security, FileStorage};
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
struct RemoveCertificateRequest {
    cert: String,
}

/// Removes a root certificate and creates a new certificate bundle
#[post("/apps/files_external/ajax/removeRootCertificate")]
async fn remove_root_certificate(
    app_manager: web::Data<AppManager>,
    user_session: web::Data<UserSession>,
    security: web::Data<Security>,
    form: web::Form<RemoveCertificateRequest>,
) -> Result<HttpResponse, Error> {
    // Check if the app is enabled
    app_manager.check_app_enabled("files_external")?;
    
    // Check if user is logged in
    user_session.check_logged_in()?;
    
    // Verify CSRF token
    security.verify_request_token()?;

    // Get the storage
    let view = FileStorage::get_storage("files_external")?;
    
    // Clean up the certificate path to prevent directory traversal
    let sanitized_cert = form.cert.trim_start_matches(|c| c == '/' || c == '\\' || c == '.');
    let file_path = format!("uploads/{}", sanitized_cert);
    
    // Check if file exists and remove it
    if view.file_exists(&file_path)? {
        view.unlink(&file_path)?;
        mount_config::create_certificate_bundle()?;
    }
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success"
    })))
}

mod mount_config {
    use anyhow::Result;

    pub fn create_certificate_bundle() -> Result<()> {
        // Implementation of the certificate bundle creation
        // This would be a port of the OC_Mount_Config::createCertificateBundle() functionality
        Ok(())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Main function would be implemented in the app's entry point, not in this module
    Ok(())
}