// SPDX-FileCopyrightText: 2012 Frank Karlitschek frank@owncloud.org
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};

/// Module for handling help page requests
pub mod help {
    use super::*;

    #[derive(Deserialize)]
    pub struct HelpQuery {
        mode: Option<String>,
    }

    #[derive(Serialize)]
    struct HelpTemplateContext {
        admin: bool,
        url: String,
        url1: String,
        url2: String,
        style1: String,
        style2: String,
    }

    /// Handler for the help page
    pub async fn handle(
        req: HttpRequest, 
        query: web::Query<HelpQuery>,
        app_state: web::Data<crate::AppState>,
    ) -> Result<HttpResponse> {
        // Check if user is logged in
        let user = app_state.auth.check_logged_in(&req)?;
        
        // Load apps
        app_state.app_manager.load_apps().await?;

        // Add style
        app_state.assets.add_style("settings", "settings");
        
        // Set active navigation entry
        app_state.navigation.set_active_entry("help");

        // Determine which help page to display
        let (url, style1, style2) = if query.mode.as_deref() == Some("admin") {
            (
                app_state.helper.link_to_absolute("core", "doc/admin/index.html"),
                String::new(),
                " pressed".to_string(),
            )
        } else {
            (
                app_state.helper.link_to_absolute("core", "doc/user/index.html"),
                " pressed".to_string(),
                String::new(),
            )
        };

        let url1 = app_state.helper.link_to_route("settings_help") + "?mode=user";
        let url2 = app_state.helper.link_to_route("settings_help") + "?mode=admin";
        
        // Check if user is admin
        let is_admin = app_state.user_manager.is_admin_user(&user);

        // Prepare template context
        let template_context = HelpTemplateContext {
            admin: is_admin,
            url,
            url1,
            url2,
            style1,
            style2,
        };

        // Render template
        let rendered_page = app_state
            .template_engine
            .render("settings", "help", "user", &template_context)?;

        // Return response
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(rendered_page))
    }
}

// Register routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/settings/help").route(web::get().to(help::handle)));
}