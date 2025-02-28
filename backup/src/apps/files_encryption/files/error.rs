use actix_web::{web, HttpResponse, HttpRequest, Responder};
use actix_web::http::StatusCode;
use actix_web::web::Query;
use serde::Deserialize;
use tera::{Tera, Context};
use std::sync::Arc;

use crate::encryption::crypt::{
    ENCRYPTION_NOT_INITIALIZED_ERROR,
    ENCRYPTION_PRIVATE_KEY_NOT_VALID_ERROR,
    ENCRYPTION_NO_SHARE_KEY_FOUND,
    ENCRYPTION_UNKNOWN_ERROR,
};
use crate::config::defaults::Defaults;
use crate::l10n::L10n;
use crate::json::Json;

#[derive(Deserialize)]
pub struct ErrorParams {
    error_code: Option<i32>,
    p: Option<String>,
}

pub async fn handle_encryption_error(
    req: HttpRequest,
    query: Query<ErrorParams>,
    l10n: web::Data<Arc<L10n>>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let l = l10n.get("files_encryption");
    
    let (error_code, error_msg) = match query.error_code {
        Some(code) => {
            match code {
                ENCRYPTION_NOT_INITIALIZED_ERROR => {
                    (code, l.t("Encryption app not initialized! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialize the encryption app."))
                },
                ENCRYPTION_PRIVATE_KEY_NOT_VALID_ERROR => {
                    let theme = Defaults::new();
                    (code, l.t(&format!("Your private key is not valid! Likely your password was changed outside of {} (e.g. your corporate directory). You can update your private key password in your personal settings to recover access to your encrypted files.", theme.get_name())))
                },
                ENCRYPTION_NO_SHARE_KEY_FOUND => {
                    (code, l.t("Can not decrypt this file, probably this is a shared file. Please ask the file owner to reshare the file with you."))
                },
                _ => {
                    (code, l.t("Unknown error please check your system settings or contact your administrator"))
                }
            }
        },
        None => {
            (ENCRYPTION_UNKNOWN_ERROR, l.t("Unknown error please check your system settings or contact your administrator"))
        }
    };

    // Check if 'p' parameter is 1
    if let Some(p) = &query.p {
        if p == "1" {
            return HttpResponse::Forbidden()
                .reason(error_msg.clone())
                .finish();
        }
    }

    // Check if request is AJAX
    let is_ajax = match req.headers().get("X-Requested-With") {
        Some(header_value) => {
            if let Ok(value_str) = header_value.to_str() {
                value_str.to_lowercase() == "xmlhttprequest"
            } else {
                false
            }
        },
        None => false
    };

    if is_ajax {
        // Return JSON error for AJAX requests
        Json::error(json!({
            "data": {
                "message": error_msg
            }
        }))
    } else {
        // Render template for normal requests
        let mut context = Context::new();
        context.insert("message", &error_msg);
        context.insert("errorCode", &error_code);
        
        match tera.render("files_encryption/invalid_private_key", &context) {
            Ok(content) => HttpResponse::build(StatusCode::FORBIDDEN)
                .reason(error_msg)
                .content_type("text/html; charset=utf-8")
                .body(content),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}