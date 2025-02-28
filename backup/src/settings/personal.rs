// Copyright (c) 2011, Robin Appelman <icewind1991@gmail.com>
// This file is licensed under the Affero General Public License version 3 or later.
// See the COPYING-README file.

use actix_web::{web, HttpResponse, Result};
use serde::Serialize;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Serialize)]
struct LanguageInfo {
    code: String,
    name: String,
}

#[derive(Serialize)]
struct TemplateData {
    usage: String,
    total_space: String,
    usage_relative: u64,
    clients: HashMap<String, String>,
    email: String,
    languages: Vec<LanguageInfo>,
    commonlanguages: Vec<LanguageInfo>,
    activelanguage: LanguageInfo,
    password_change_supported: bool,
    display_name_change_supported: bool,
    display_name: String,
    enable_decrypt_all: bool,
    enable_avatars: bool,
    forms: Vec<String>,
}

pub async fn personal_settings(
    app_state: web::Data<AppState>,
    identity: Identity,
) -> Result<HttpResponse> {
    // Check if user is logged in
    let user = identity.identity().ok_or_else(|| actix_web::error::ErrorUnauthorized("Not logged in"))?;
    
    // Load apps
    app_state.app_manager.load_apps().await?;

    // Initialize themable default strings and urls
    let defaults = app_state.defaults.clone();

    // Add scripts and styles
    app_state.util.add_script("settings", "personal").await?;
    app_state.util.add_style("settings", "settings").await?;
    app_state.util.add_script("3rdparty", "chosen/chosen.jquery.min").await?;
    app_state.util.add_style("3rdparty", "chosen").await?;
    app_state.util.add_script("files", "jquery.fileupload").await?;
    
    if app_state.config.get_value("enable_avatars", true) {
        app_state.util.add_script("3rdparty/Jcrop", "jquery.Jcrop.min").await?;
        app_state.util.add_style("3rdparty/Jcrop", "jquery.Jcrop.min").await?;
    }
    
    // Set active navigation entry
    app_state.app_manager.set_active_navigation_entry("personal").await?;

    // Get storage info
    let storage_info = app_state.helper.get_storage_info("/").await?;

    // Get user email
    let email = app_state.preferences.get_value(&user, "settings", "email", "".to_string()).await?;

    // Get user language
    let user_lang_code = app_state.preferences
        .get_value(&user, "core", "lang", app_state.l10n.find_language().await?)
        .await?;
    
    // Find available languages
    let language_codes = app_state.l10n.find_available_languages().await?;

    // Check if encryption was enabled in the past
    let enable_decrypt_all = app_state.util.encrypted_files().await?;

    // Array of common languages
    let common_lang_codes = vec![
        "en", "es", "fr", "de", "de_DE", "ja_JP", "ar", "ru", "nl", 
        "it", "pt_BR", "pt_PT", "da", "fi_FI", "nb_NO", "sv", "zh_CN", "ko"
    ];

    // Load language names
    let language_names = app_state.language_provider.get_language_codes().await?;
    
    let mut languages = Vec::new();
    let mut common_languages = vec![None; common_lang_codes.len()];
    let mut user_language = None;

    // Process languages
    for lang in language_codes {
        let l = app_state.l10n.get("settings", &lang).await?;
        let language_name = l.t("__language_name__");
        
        let ln = if !language_name.starts_with('_') {
            // Language name is in the translation file
            LanguageInfo { 
                code: lang.clone(), 
                name: language_name.to_string() 
            }
        } else if let Some(name) = language_names.get(&lang) {
            LanguageInfo { 
                code: lang.clone(), 
                name: name.clone() 
            }
        } else {
            // Fallback to language code
            LanguageInfo { 
                code: lang.clone(), 
                name: lang.clone() 
            }
        };

        // Categorize languages
        if lang == user_lang_code {
            user_language = Some(ln);
        } else if let Some(pos) = common_lang_codes.iter().position(|&code| code == lang) {
            common_languages[pos] = Some(ln);
        } else {
            languages.push(ln);
        }
    }

    // Remove None values and collect
    let mut common_languages: Vec<LanguageInfo> = common_languages.into_iter()
        .filter_map(|lang| lang)
        .collect();

    // Sort languages by name
    languages.sort_by(|a, b| a.name.cmp(&b.name));

    // Links to clients
    let mut clients = HashMap::new();
    clients.insert(
        "desktop".to_string(), 
        app_state.config.get_value("customclient_desktop", defaults.get_sync_client_url())
    );
    clients.insert(
        "android".to_string(), 
        app_state.config.get_value("customclient_android", "https://play.google.com/store/apps/details?id=com.owncloud.android".to_string())
    );
    clients.insert(
        "ios".to_string(), 
        app_state.config.get_value("customclient_ios", "https://itunes.apple.com/us/app/owncloud/id543672169?mt=8".to_string())
    );

    // Get user information
    let user_object = app_state.user_manager.get_user(&user).await?;
    let can_change_password = app_state.user_manager.can_user_change_password(&user).await?;
    let can_change_display_name = app_state.user_manager.can_user_change_display_name(&user).await?;
    let display_name = app_state.user_manager.get_display_name(&user).await?;

    // Get app forms
    let forms = app_state.app_manager.get_forms("personal").await?;

    // Create template data
    let template_data = TemplateData {
        usage: app_state.helper.human_file_size(storage_info.used),
        total_space: app_state.helper.human_file_size(storage_info.total),
        usage_relative: storage_info.relative,
        clients,
        email,
        languages,
        commonlanguages: common_languages,
        activelanguage: user_language.unwrap_or_else(|| LanguageInfo { 
            code: user_lang_code.clone(), 
            name: user_lang_code 
        }),
        password_change_supported: can_change_password,
        display_name_change_supported: can_change_display_name,
        display_name,
        enable_decrypt_all,
        enable_avatars: app_state.config.get_value("enable_avatars", true),
        forms,
    };

    // Render template
    let html = app_state.template_engine
        .render("settings/personal", &template_data)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}