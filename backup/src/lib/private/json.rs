use actix_web::{http::header, web, HttpResponse};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};

/// JSON response helper for Nextcloud API
pub struct OcJson;

static SEND_CONTENT_TYPE_HEADER: AtomicBool = AtomicBool::new(false);

impl OcJson {
    /// Set Content-Type header to jsonrequest
    pub fn set_content_type_header(builder: &mut HttpResponse::Builder, content_type: Option<&str>) -> &mut HttpResponse::Builder {
        if !SEND_CONTENT_TYPE_HEADER.load(Ordering::Relaxed) {
            let content_type = content_type.unwrap_or("application/json");
            builder.content_type(format!("{}; charset=utf-8", content_type));
            SEND_CONTENT_TYPE_HEADER.store(true, Ordering::Relaxed);
        }
        builder
    }

    /// Check if the app is enabled, return json error if not
    pub async fn check_app_enabled(app: &str) -> Result<(), HttpResponse> {
        if !crate::apps::AppManager::is_enabled(app).await {
            let l = crate::l10n::L10n::get("lib").await;
            return Err(Self::error(ErrorData {
                message: l.t("Application is not enabled"),
                ..ErrorData::default()
            }));
        }
        Ok(())
    }

    /// Check if the user is logged in, return json error if not
    pub async fn check_logged_in() -> Result<(), HttpResponse> {
        if !crate::user::User::is_logged_in().await {
            let l = crate::l10n::L10n::get("lib").await;
            return Err(Self::error(ErrorData {
                message: l.t("Authentication error"),
                ..ErrorData::default()
            }));
        }
        Ok(())
    }

    /// Check an ajax get/post call if the request token is valid.
    pub async fn call_check() -> Result<(), HttpResponse> {
        if !crate::util::Util::is_call_registered().await {
            let l = crate::l10n::L10n::get("lib").await;
            return Err(Self::error(ErrorData {
                message: l.t("Token expired. Please reload page."),
                ..ErrorData::default()
            }));
        }
        Ok(())
    }

    /// Check if the user is an admin, return json error if not
    pub async fn check_admin_user() -> Result<(), HttpResponse> {
        if !crate::user::User::is_admin_user(crate::user::User::get_user().await).await {
            let l = crate::l10n::L10n::get("lib").await;
            return Err(Self::error(ErrorData {
                message: l.t("Authentication error"),
                ..ErrorData::default()
            }));
        }
        Ok(())
    }

    /// Check if the user is a subadmin, return json error if not
    pub async fn check_sub_admin_user() -> Result<(), HttpResponse> {
        if !crate::user::SubAdmin::is_sub_admin(crate::user::User::get_user().await).await {
            let l = crate::l10n::L10n::get("lib").await;
            return Err(Self::error(ErrorData {
                message: l.t("Authentication error"),
                ..ErrorData::default()
            }));
        }
        Ok(())
    }

    /// Send json error response
    pub fn error<T: serde::Serialize>(data: T) -> HttpResponse {
        let response = JsonResponse {
            status: "error".to_string(),
            data: Some(data),
        };
        
        let mut builder = HttpResponse::BadRequest();
        Self::set_content_type_header(&mut builder, None);
        
        // Disable mimesniffing
        builder.insert_header((header::X_CONTENT_TYPE_OPTIONS, "nosniff"));
        builder.json(response)
    }

    /// Send json success response
    pub fn success<T: serde::Serialize>(data: T) -> HttpResponse {
        let response = JsonResponse {
            status: "success".to_string(),
            data: Some(data),
        };
        
        let mut builder = HttpResponse::Ok();
        Self::set_content_type_header(&mut builder, None);
        
        // Disable mimesniffing
        builder.insert_header((header::X_CONTENT_TYPE_OPTIONS, "nosniff"));
        builder.json(response)
    }

    /// Encode and send response as JSON
    pub fn encoded_print<T: serde::Serialize>(data: T, set_content_type: bool) -> HttpResponse {
        let mut builder = HttpResponse::Ok();
        
        // Disable mimesniffing
        builder.insert_header((header::X_CONTENT_TYPE_OPTIONS, "nosniff"));
        
        if set_content_type {
            Self::set_content_type_header(&mut builder, None);
        }
        
        builder.json(data)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct ErrorData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct JsonResponse<T> {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}