// Copyright (c) 2013 Sam Tuke <samtuke@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use actix_web::{web, HttpResponse, Result};
use tera::Tera;
use std::sync::Arc;

use crate::app_config::AppConfig;
use crate::encryption::{Session, Util};
use crate::filesystem::FilesystemView;
use crate::user::User;
use crate::util::Util as OcUtil;

pub async fn render_settings_personal(
    tera: web::Data<Tera>,
    app_config: web::Data<Arc<AppConfig>>,
    session: web::Data<Arc<Session>>,
    user: User,
) -> Result<HttpResponse> {
    // Add CSS stylesheet
    OcUtil::add_style("files_encryption", "settings-personal");

    let view = FilesystemView::new("/");
    let util = Util::new(&view, &user.get_user_id());

    let private_key_set = session.get_private_key().is_some();
    // did we tried to initialize the keys for this session?
    let initialized = session.get_initialized();

    let recovery_admin_enabled = app_config
        .get_value("files_encryption", "recoveryAdminEnabled")
        .unwrap_or_default();
    
    let recovery_enabled_for_user = util.recovery_enabled_for_user();

    if recovery_admin_enabled || !private_key_set {
        // Add JavaScript
        OcUtil::add_script("files_encryption", "settings-personal");
        OcUtil::add_script("settings", "personal");

        let mut context = tera::Context::new();
        context.insert("recoveryEnabled", &recovery_admin_enabled);
        context.insert("recoveryEnabledForUser", &recovery_enabled_for_user);
        context.insert("privateKeySet", &private_key_set);
        context.insert("initialized", &initialized);

        let rendered = tera
            .render("files_encryption/settings-personal.html", &context)
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
    } else {
        // Return empty response if no content should be shown
        Ok(HttpResponse::Ok().finish())
    }
}