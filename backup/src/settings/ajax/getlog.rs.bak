// Copyright (c) 2012, Robin Appelman <icewind1991@gmail.com>
// This file is licensed under the Affero General Public License version 3 or later.
// See the COPYING-README file.

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct GetLogParams {
    count: Option<usize>,
    offset: Option<usize>,
}

#[derive(Serialize)]
pub struct LogEntry {
    // Define log entry fields based on OC_Log_Owncloud::getEntries output
    // This is a placeholder and should be adjusted according to actual log structure
    app: String,
    message: String,
    level: i32,
    time: f64,
}

pub async fn get_log(
    params: web::Query<GetLogParams>,
    auth_service: web::Data<crate::auth::AuthService>,
) -> impl Responder {
    // Check admin user
    if !auth_service.is_admin_user().await {
        return HttpResponse::Forbidden().json(json!({
            "status": "error",
            "data": {
                "message": "Forbidden"
            }
        }));
    }

    let count = params.count.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);

    // Get log entries
    match get_log_entries(count, offset).await {
        Ok((entries, has_more)) => {
            HttpResponse::Ok().json(json!({
                "status": "success",
                "data": entries,
                "remain": has_more
            }))
        }
        Err(err) => {
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "data": {
                    "message": format!("Failed to get log entries: {}", err)
                }
            }))
        }
    }
}

async fn get_log_entries(count: usize, offset: usize) -> Result<(Vec<LogEntry>, bool), String> {
    // This is a placeholder for the actual log retrieval logic
    // Should be implemented according to OC_Log_Owncloud::getEntries functionality
    let log_service = crate::log::LogService::new();
    
    let entries = log_service.get_entries(count, offset)
        .await
        .map_err(|e| e.to_string())?;
    
    // Check if there are more entries
    let has_more = log_service.get_entries(1, offset + count)
        .await
        .map_err(|e| e.to_string())?
        .len() > 0;
    
    Ok((entries, has_more))
}

// This function configures the routes for the module
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/settings/ajax/getlog")
            .route(web::get().to(get_log))
    );
}