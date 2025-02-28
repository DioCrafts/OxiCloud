// ownCloud
//
// @author Frank Karlitschek
// @copyright 2012 Frank Karlitschek frank@owncloud.org
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde::Serialize;
use std::collections::HashMap;
use thiserror::Error;

mod oc_util;
mod oc_app;
mod oc_template;

use crate::oc_util::OcUtil;
use crate::oc_app::OcApp;
use crate::oc_template::OcTemplate;

#[derive(Error, Debug)]
pub enum AppsError {
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("Template error: {0}")]
    TemplateError(String),
    
    #[error("App error: {0}")]
    AppError(String),
}

#[derive(Serialize)]
struct AppsContext {
    apps: Vec<HashMap<String, serde_json::Value>>,
    appid: String,
}

pub async fn handle_apps(req: HttpRequest, query: web::Query<HashMap<String, String>>) -> Result<HttpResponse, AppsError> {
    // Check admin privileges
    OcUtil::check_admin_user(&req).map_err(|e| AppsError::AuthError(e.to_string()))?;
    
    // Load all apps
    OcApp::load_apps().await.map_err(|e| AppsError::AppError(e.to_string()))?;
    
    // Load the styles we need
    OcUtil::add_style("settings", "settings").map_err(|e| AppsError::AppError(e.to_string()))?;
    
    // Set active navigation entry
    OcApp::set_active_navigation_entry("core_apps").map_err(|e| AppsError::AppError(e.to_string()))?;
    
    // Get the combined list of all apps
    let combined_apps = OcApp::list_all_apps().await.map_err(|e| AppsError::AppError(e.to_string()))?;
    
    // Create template
    let mut tmpl = OcTemplate::new("settings", "apps", "user").map_err(|e| AppsError::TemplateError(e.to_string()))?;
    
    // Get appid from query parameters
    let appid = query.get("appid").cloned().unwrap_or_default();
    
    // Assign template variables
    let context = AppsContext {
        apps: combined_apps,
        appid,
    };
    
    // Render and return the page
    let response = tmpl.render(&context).map_err(|e| AppsError::TemplateError(e.to_string()))?;
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response))
}

// Register this handler with your router
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/settings/apps")
            .route(web::get().to(handle_apps))
    );
}