/*
 * Copyright (c) 2011, Robin Appelman <icewind1991@gmail.com>
 * This file is licensed under the Affero General Public License version 3 or later.
 * See the COPYING-README file.
 */

use actix_web::{web, HttpResponse, Result};
use serde::Serialize;
use std::collections::HashMap;

// Se asume que estas estructuras existen en otros archivos
use crate::utils::{
    check_admin_user, load_apps, add_style, add_script, set_active_navigation_entry,
    is_htaccess_working, is_internet_connection_enabled, is_internet_connection_working,
    is_set_locale_working, is_web_dav_working, file_info_loaded, server_protocol,
};
use crate::config::{Config, Appconfig};
use crate::log::OwncloudLog;
use crate::template::Template;

#[derive(Debug, Serialize)]
struct AdminPageData {
    loglevel: i32,
    entries: Vec<LogEntry>,
    entriesremain: bool,
    htaccessworking: bool,
    internetconnectionworking: bool,
    islocaleworking: bool,
    iswebdavworking: bool,
    has_fileinfo: bool,
    backgroundjobs_mode: String,
    shareapienabled: String,
    isconnectedviahttps: bool,
    forcehttpsenabled: bool,
    allowlinks: String,
    allowpublicupload: String,
    allowresharing: String,
    allowmailnotification: String,
    sharepolicy: String,
    forms: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
struct LogEntry {
    // Definición de los campos de entrada de registro
    // (se adaptaría a la estructura real en tu aplicación)
    message: String,
    level: i32,
    timestamp: i64,
}

pub async fn admin_page() -> Result<HttpResponse> {
    check_admin_user()?;
    load_apps().await?;

    add_style("settings", "settings");
    add_script("settings", "admin");
    add_script("settings", "log");
    set_active_navigation_entry("admin");

    let tmpl = Template::new("settings", "admin", "user");
    let forms = get_forms("admin").await?;
    let htaccess_working = is_htaccess_working().await?;

    let entries = OwncloudLog::get_entries(3).await?;
    let entries_remain = OwncloudLog::get_entries(4).await?.len() > 3;

    let internet_connection_working = if is_internet_connection_enabled().await? {
        is_internet_connection_working().await?
    } else {
        false
    };

    // Check if connected using HTTPS
    let connected_https = server_protocol() == "https";

    let data = AdminPageData {
        loglevel: Config::get_value("loglevel", 2),
        entries,
        entriesremain: entries_remain,
        htaccessworking: htaccess_working,
        internetconnectionworking: internet_connection_working,
        islocaleworking: is_set_locale_working().await?,
        iswebdavworking: is_web_dav_working().await?,
        has_fileinfo: file_info_loaded(),
        backgroundjobs_mode: Appconfig::get_value("core", "backgroundjobs_mode", "ajax".to_string()),
        shareapienabled: Appconfig::get_value("core", "shareapi_enabled", "yes".to_string()),
        isconnectedviahttps: connected_https,
        forcehttpsenabled: Config::get_value("forcessl", false),
        allowlinks: Appconfig::get_value("core", "shareapi_allow_links", "yes".to_string()),
        allowpublicupload: Appconfig::get_value("core", "shareapi_allow_public_upload", "yes".to_string()),
        allowresharing: Appconfig::get_value("core", "shareapi_allow_resharing", "yes".to_string()),
        allowmailnotification: Appconfig::get_value("core", "shareapi_allow_mail_notification", "yes".to_string()),
        sharepolicy: Appconfig::get_value("core", "shareapi_share_policy", "global".to_string()),
        forms,
    };

    let html = tmpl.render(&data)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn get_forms(section: &str) -> Result<Vec<String>> {
    // Implementación para obtener formularios de aplicaciones
    // Esta función reemplaza a OC_App::getForms
    Ok(Vec::new()) // Implementación simplificada
}

// Función para registrar la ruta con Actix-web
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/settings/admin").route(web::get().to(admin_page)));
}