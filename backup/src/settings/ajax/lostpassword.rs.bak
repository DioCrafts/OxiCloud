use actix_web::{web, HttpResponse, Responder};
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::l10n::L10n;
use crate::user::User;
use crate::preferences::Preferences;
use crate::auth::check_logged_in;
use crate::security::check_csrf_token;

#[derive(Deserialize)]
pub struct LostPasswordRequest {
    email: Option<String>,
}

#[derive(Serialize)]
struct JsonResponse {
    status: &'static str,
    data: JsonResponseData,
}

#[derive(Serialize)]
struct JsonResponseData {
    message: String,
}

pub async fn handle_lost_password(
    req: web::Json<LostPasswordRequest>,
    l10n: web::Data<L10n>,
    user: web::Data<User>,
    preferences: web::Data<Preferences>,
) -> impl Responder {
    // Check if user is logged in
    if let Err(response) = check_logged_in() {
        return response;
    }
    
    // Check CSRF token
    if let Err(response) = check_csrf_token() {
        return response;
    }
    
    // Get data
    if let Some(email) = &req.email {
        let email = email.trim();
        
        // Validate email
        let email_regex = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$").unwrap();
        
        if email_regex.is_match(email) {
            // Save email preference
            preferences.set_value(&user.get_user(), "settings", "email", email).await;
            
            // Return success response
            let response = JsonResponse {
                status: "success",
                data: JsonResponseData {
                    message: l10n.t("Email saved"),
                },
            };
            
            HttpResponse::Ok().json(response)
        } else {
            // Return error for invalid email
            let response = JsonResponse {
                status: "error",
                data: JsonResponseData {
                    message: l10n.t("Invalid email"),
                },
            };
            
            HttpResponse::BadRequest().json(response)
        }
    } else {
        // Return error for missing email
        let response = JsonResponse {
            status: "error",
            data: JsonResponseData {
                message: l10n.t("Invalid email"),
            },
        };
        
        HttpResponse::BadRequest().json(response)
    }
}