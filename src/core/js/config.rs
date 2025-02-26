//! Copyright (c) 2013 Lukas Reschke <lukas@statuscode.ch>
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.

use actix_web::{web, HttpResponse, Responder};
use chrono::Datelike;
use serde_json::json;
use std::collections::HashMap;

use crate::l10n::L10n;
use crate::app::App;
use crate::config::Config;

pub async fn generate_js_config() -> impl Responder {
    // Set the content type to Javascript and disable caching
    let mut response = HttpResponse::Ok()
        .content_type("text/javascript")
        .insert_header(("Cache-Control", "no-cache, must-revalidate"))
        .insert_header(("Expires", "Sat, 26 Jul 1997 05:00:00 GMT"));
    
    // Enable l10n support
    let l = L10n::get("core");
    
    // Get the config
    let mut apps_paths = HashMap::new();
    for app in App::get_enabled_apps().iter() {
        apps_paths.insert(app.clone(), App::get_app_web_path(app));
    }
    
    // Convert app paths to JSON and replace escaped slashes
    let apps_webroots_json = serde_json::to_string(&apps_paths)
        .unwrap_or_default()
        .replace("\\/", "/");
    
    let debug = if cfg!(debug_assertions) { "true" } else { "false" };
    let webroot = format!("\"{}\"", Config::get_webroot());
    
    let date_picker_format = serde_json::to_string(&l.l("jsdate", "jsdate"))
        .unwrap_or_default();
    
    let day_names = serde_json::to_string(&[
        l.t("Sunday"),
        l.t("Monday"),
        l.t("Tuesday"),
        l.t("Wednesday"),
        l.t("Thursday"),
        l.t("Friday"),
        l.t("Saturday"),
    ]).unwrap_or_default();
    
    let month_names = serde_json::to_string(&[
        l.t("January"),
        l.t("February"),
        l.t("March"),
        l.t("April"),
        l.t("May"),
        l.t("June"),
        l.t("July"),
        l.t("August"),
        l.t("September"),
        l.t("October"),
        l.t("November"),
        l.t("December"),
    ]).unwrap_or_default();
    
    let first_day = serde_json::to_string(&l.l("firstday", "firstday"))
        .unwrap_or_default();
    
    // Construct the JavaScript output
    let js_output = format!(
        "var oc_debug={};\n\
         var oc_webroot={};\n\
         var oc_appswebroots={};\n\
         var datepickerFormatDate={};\n\
         var dayNames={};\n\
         var monthNames={};\n\
         var firstDay={};\n",
        debug, webroot, apps_webroots_json, date_picker_format, 
        day_names, month_names, first_day
    );
    
    response.body(js_output)
}