use actix_web::{web, HttpResponse, Result};
use google_oauth2::{Client as GoogleClient, ClientConfig, Scope};
use serde::{Deserialize, Serialize};
use std::path::PathSeparator;
use nextcloud_core::{
    app::{check_app_enabled, get_app_path},
    auth::check_logged_in,
    csrf::validate_csrf_token,
};

#[derive(Deserialize)]
struct GoogleRequest {
    client_id: String,
    client_secret: String,
    redirect: String,
    step: Option<u8>,
    code: Option<String>,
}

#[derive(Serialize)]
struct AuthUrlResponse {
    url: String,
}

#[derive(Serialize)]
struct TokenResponse {
    token: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Serialize)]
enum ResponseData {
    #[serde(rename = "data")]
    AuthUrl(AuthUrlResponse),
    #[serde(rename = "data")]
    Token(TokenResponse),
    #[serde(rename = "data")]
    Error(ErrorResponse),
}

#[derive(Serialize)]
struct JsonResponse {
    #[serde(flatten)]
    data: ResponseData,
    status: &'static str,
}

async fn handle_google_auth(
    form: web::Form<GoogleRequest>
) -> Result<HttpResponse> {
    // Check if app is enabled and user is logged in
    check_app_enabled("files_external")?;
    check_logged_in()?;
    validate_csrf_token()?;

    // Create Google client
    let config = ClientConfig::new()
        .client_id(&form.client_id)
        .client_secret(&form.client_secret)
        .redirect_uri(&form.redirect)
        .add_scope(Scope::new("https://www.googleapis.com/auth/drive"));
    
    let client = GoogleClient::new(config);

    match form.step {
        Some(1) => {
            match client.create_auth_url() {
                Ok(auth_url) => {
                    let response = JsonResponse {
                        data: ResponseData::AuthUrl(AuthUrlResponse { url: auth_url }),
                        status: "success",
                    };
                    Ok(HttpResponse::Ok().json(response))
                },
                Err(err) => {
                    let response = JsonResponse {
                        data: ResponseData::Error(ErrorResponse { 
                            message: format!("Step 1 failed. Exception: {}", err) 
                        }),
                        status: "error",
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                }
            }
        },
        Some(2) => {
            if let Some(code) = &form.code {
                match client.authenticate(code).await {
                    Ok(token) => {
                        let response = JsonResponse {
                            data: ResponseData::Token(TokenResponse { token }),
                            status: "success",
                        };
                        Ok(HttpResponse::Ok().json(response))
                    },
                    Err(err) => {
                        let response = JsonResponse {
                            data: ResponseData::Error(ErrorResponse {
                                message: format!("Step 2 failed. Exception: {}", err)
                            }),
                            status: "error",
                        };
                        Ok(HttpResponse::BadRequest().json(response))
                    }
                }
            } else {
                let response = JsonResponse {
                    data: ResponseData::Error(ErrorResponse {
                        message: "Missing code parameter for step 2".to_string()
                    }),
                    status: "error",
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        },
        _ => {
            let response = JsonResponse {
                data: ResponseData::Error(ErrorResponse {
                    message: "Invalid or missing step parameter".to_string()
                }),
                status: "error",
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}

// Configure service
pub fn configure_google_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/ajax/google.php")
            .route(web::post().to(handle_google_auth))
    );
}