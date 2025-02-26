/**
 * Copyright (c) 2012 Thomas Tanghus <thomas@tanghus.net>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use crate::config;
use crate::l10n::L10n;
use crate::auth::check_admin_user;
use crate::ocs_client::{self, Category, Application};
use crate::app::get_enabled_apps;
use crate::helper::image_path;

#[derive(Serialize)]
struct AppData {
    name: String,
    id: String,
    active: bool,
    description: String,
    author: String,
    license: String,
    preview: String,
    internal: bool,
    internallabel: String,
    update: bool,
}

#[derive(Serialize)]
struct SuccessResponse {
    #[serde(rename = "type")]
    response_type: String,
    data: Vec<AppData>,
}

#[derive(Serialize)]
struct ErrorResponse {
    data: ErrorData,
}

#[derive(Serialize)]
struct ErrorData {
    message: String,
}

pub async fn handle_ocs_apps() -> Result<HttpResponse> {
    // Check admin privileges
    check_admin_user()?;

    let l = L10n::get("settings");

    // Check if app store is enabled
    if !config::get_value("appstoreenabled", true) {
        return Ok(HttpResponse::Ok().json(SuccessResponse {
            response_type: "external".to_string(),
            data: Vec::new(),
        }));
    }

    // Get enabled apps
    let enabled_apps = match get_enabled_apps().await {
        Some(apps) => apps,
        None => {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                data: ErrorData {
                    message: l.t("Unable to load list from App Store"),
                },
            }));
        }
    };

    let mut apps: Vec<AppData> = Vec::new();

    // Apps from external repo via OCS
    if let Some(category_names) = ocs_client::get_categories().await {
        let categories: Vec<String> = category_names.keys().cloned().collect();
        let page = 0;
        let filter = "approved";
        
        if let Some(external_apps) = ocs_client::get_applications(&categories, page, filter).await {
            for app in external_apps {
                // Show only external apps that aren't enabled yet
                let local = enabled_apps.iter().any(|a| a == &app.name);
                
                if !local {
                    let preview = if app.preview.is_empty() {
                        image_path("settings", "trans.png")
                    } else {
                        app.preview.clone()
                    };
                    
                    let label = if app.label == "recommended" {
                        "3rd Party"
                    } else {
                        "Recommended"
                    };
                    
                    apps.push(AppData {
                        name: app.name,
                        id: app.id,
                        active: false,
                        description: app.description,
                        author: app.personid,
                        license: app.license,
                        preview,
                        internal: false,
                        internallabel: label.to_string(),
                        update: false,
                    });
                }
            }
        }
    }

    Ok(HttpResponse::Ok().json(SuccessResponse {
        response_type: "external".to_string(),
        data: apps,
    }))
}