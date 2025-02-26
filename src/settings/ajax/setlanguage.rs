use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::l10n::L10n;
use crate::user::{self, User};
use crate::preferences;
use crate::auth;
use crate::json;

#[derive(Deserialize)]
pub struct SetLanguageRequest {
    lang: Option<String>,
}

#[derive(Serialize)]
struct SuccessResponse {
    data: SuccessData,
}

#[derive(Serialize)]
struct SuccessData {
    message: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    data: ErrorData,
}

#[derive(Serialize)]
struct ErrorData {
    message: String,
}

pub async fn set_language(
    req: web::Json<SetLanguageRequest>,
    l10n: web::Data<L10n>,
    user: User,
) -> HttpResponse {
    // Verify CSRF token
    if !json::check_call() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            data: ErrorData {
                message: l10n.t("Invalid request").into(),
            },
        });
    }

    // Get data
    match &req.lang {
        Some(lang) => {
            let language_codes = L10n::find_available_languages();
            
            if language_codes.contains(lang) || lang == "en" {
                if let Err(_) = preferences::set_value(&user.get_uid(), "core", "lang", lang) {
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        data: ErrorData {
                            message: l10n.t("Failed to save language preference").into(),
                        },
                    });
                }
                
                HttpResponse::Ok().json(SuccessResponse {
                    data: SuccessData {
                        message: l10n.t("Language changed").into(),
                    },
                })
            } else {
                HttpResponse::BadRequest().json(ErrorResponse {
                    data: ErrorData {
                        message: l10n.t("Invalid request").into(),
                    },
                })
            }
        },
        None => {
            HttpResponse::BadRequest().json(ErrorResponse {
                data: ErrorData {
                    message: l10n.t("Invalid request").into(),
                },
            })
        }
    }
}

// Register this handler
pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/settings/ajax/setlanguage")
            .route(web::post().to(set_language))
            .wrap(auth::LoggedInMiddleware::new()),
    );
}