use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::apps::AppManager;
use crate::auth::middleware::AdminUser;
use crate::core::installer::Installer;
use crate::l10n::L10n;
use crate::utils::json::{JsonResponse, JsonError};

#[derive(Deserialize)]
pub struct UpdateAppRequest {
    appid: String,
}

#[derive(Serialize)]
struct UpdateAppResponse {
    appid: String,
}

/// Updates an app with the given app ID
pub async fn update_app(
    req: web::Json<UpdateAppRequest>,
    _: AdminUser, // middleware ensures this is an admin user
    app_manager: web::Data<AppManager>,
    installer: web::Data<Installer>,
    l10n: web::Data<L10n>,
) -> impl Responder {
    // Clean the app ID
    let appid = app_manager.clean_app_id(&req.appid);
    
    // Try to update the app
    match installer.update_app(&appid).await {
        Ok(_) => {
            // Success response
            HttpResponse::Ok().json(JsonResponse::success(UpdateAppResponse { 
                appid 
            }))
        }
        Err(_) => {
            // Error response
            let error_message = l10n.get("settings").t("Couldn't update app.");
            HttpResponse::InternalServerError().json(JsonError::new(error_message))
        }
    }
}