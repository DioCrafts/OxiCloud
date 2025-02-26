//! Copyright (c) 2013 Lukas Reschke <lukas@statuscode.ch>
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.

use actix_web::{web, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    id: String,
    // Add other fields from the original app structure
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    appid: Option<String>,
}

/// Handler for listing all apps
/// 
/// Requires admin privileges to access
pub async fn apps_custom(
    query: web::Query<QueryParams>,
    identity: web::ReqData<ocs_auth::Identity>,
) -> Result<impl Responder> {
    // Check if admin user
    if !identity.is_admin() {
        return Ok(HttpResponse::Forbidden().finish());
    }

    // List all apps
    let combined_apps = ocs_app::list_all_apps().await?;
    
    // Build response content
    let mut response_content = String::new();
    
    for app in combined_apps {
        response_content.push_str(&format!(
            "appData_{}={}\n", 
            app.id, 
            serde_json::to_string(&app)?
        ));
    }
    
    // Append app ID from query params
    if let Some(appid) = &query.appid {
        response_content.push_str(&format!(
            "var appid = {};", 
            serde_json::to_string(appid)?
        ));
    } else {
        response_content.push_str("var appid = null;");
    }
    
    // Return response with appropriate headers
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .append_header(("Cache-Control", "no-cache, must-revalidate"))
        .append_header(("Expires", "Sat, 26 Jul 1997 05:00:00 GMT"))
        .body(response_content))
}

/// Register the apps-custom handler with the application router
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/settings/js/apps-custom")
            .route(web::get().to(apps_custom))
    );
}