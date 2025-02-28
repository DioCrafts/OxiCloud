use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse, Result};
use futures::{StreamExt, TryStreamExt};
use log::{warn};
use openssl::x509::X509;
use std::{fs, io::Write, path::Path};

use crate::apps::files_external::utils::mount_config;
use crate::core::app::check_app_enabled;
use crate::core::auth::user::get_user;
use crate::core::files::file_proxy::FileProxy;
use crate::core::files::view::FilesView;
use crate::core::routes::settings::personal_url;

pub async fn add_root_certificate(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // Check if app is enabled
    if !check_app_enabled("files_external") {
        return Ok(HttpResponse::Found()
            .append_header(("Location", personal_url()))
            .finish());
    }

    // Process the multipart form
    let mut certificate_data = None;
    let mut filename = None;

    while let Some(item) = payload.next().await {
        let mut field = item?;
        if field.name() == "rootcert_import" {
            filename = field.content_disposition().get_filename().map(String::from);
            
            let mut data = Vec::new();
            while let Some(chunk) = field.next().await {
                let chunk = chunk?;
                data.extend_from_slice(&chunk);
            }
            
            certificate_data = Some(data);
        }
    }

    // Check if we have a filename
    let filename = match filename {
        Some(name) => name,
        None => {
            return Ok(HttpResponse::Found()
                .append_header(("Location", personal_url()))
                .finish());
        }
    };

    // Check if we have certificate data
    let data = match certificate_data {
        Some(data) => data,
        None => {
            return Ok(HttpResponse::Found()
                .append_header(("Location", personal_url()))
                .finish());
        }
    };

    // Create the view
    let current_user = get_user().unwrap_or_default();
    let view_path = format!("/{}/files_external/uploads", current_user);
    let view = FilesView::new(&view_path);

    // Create directory if it doesn't exist
    if !view.file_exists("") {
        view.mkdir("").map_err(|_| {
            actix_web::error::ErrorInternalServerError("Failed to create directory")
        })?;
    }

    // Try to validate the certificate
    let mut is_valid = X509::from_pem(&data).is_ok();

    // Maybe it was just the wrong file format, try to convert it
    if !is_valid {
        let encoded = base64::encode(&data);
        let mut formatted_data = String::new();
        
        formatted_data.push_str("-----BEGIN CERTIFICATE-----\n");
        
        for i in 0..(encoded.len() / 64) + 1 {
            let start = i * 64;
            let end = std::cmp::min(start + 64, encoded.len());
            if start < end {
                formatted_data.push_str(&encoded[start..end]);
                formatted_data.push('\n');
            }
        }
        
        formatted_data.push_str("-----END CERTIFICATE-----\n");
        
        is_valid = X509::from_pem(formatted_data.as_bytes()).is_ok();
        
        if is_valid {
            // Update data to the formatted version
            data = formatted_data.into_bytes();
        }
    }

    // Add the certificate if it could be verified
    if is_valid {
        // Disable proxy to prevent multiple file operations
        let proxy_status = FileProxy::is_enabled();
        FileProxy::set_enabled(false);
        
        // Save the certificate
        view.file_put_contents(&filename, &data).map_err(|_| {
            actix_web::error::ErrorInternalServerError("Failed to save certificate")
        })?;
        
        // Create certificate bundle
        mount_config::create_certificate_bundle();
        
        // Restore proxy status
        FileProxy::set_enabled(proxy_status);
    } else {
        warn!(
            "Couldn't import SSL root certificate ({}), allowed formats: PEM and DER",
            filename
        );
    }

    // Redirect to personal settings
    Ok(HttpResponse::Found()
        .append_header(("Location", personal_url()))
        .finish())
}