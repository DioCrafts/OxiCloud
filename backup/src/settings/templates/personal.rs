// Copyright (c) 2011, Robin Appelman <icewind1991@gmail.com>
// This file is licensed under the Affero General Public License version 3 or later.
// See the COPYING-README file.

use actix_web::{web, HttpResponse};
use askama::Template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "personal.html")]
struct PersonalTemplate {
    clients: HashMap<String, String>,
    usage: String,
    total_space: String,
    usage_relative: String,
    password_change_supported: bool,
    display_name_change_supported: bool,
    display_name: String,
    email: String,
    enable_avatars: bool,
    active_language: Language,
    common_languages: Vec<Language>,
    languages: Vec<Language>,
    forms: Vec<String>,
    enable_decrypt_all: bool,
    theme_name: String,
    version_string: String,
    channel: String,
    edition_string: String,
    short_footer: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Language {
    code: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct PersonalContext {
    l: Translator,
    clients: HashMap<String, String>,
    usage: String,
    total_space: String,
    usage_relative: String,
    password_change_supported: bool,
    display_name_change_supported: bool,
    display_name: String,
    email: String,
    enable_avatars: bool,
    active_language: Language,
    common_languages: Vec<Language>,
    languages: Vec<Language>,
    forms: Vec<String>,
    enable_decrypt_all: bool,
    theme: Theme,
}

struct Translator {
    translations: HashMap<String, String>,
}

impl Translator {
    fn new() -> Self {
        Self {
            translations: HashMap::new(),
        }
    }

    fn t(&self, key: &str) -> String {
        self.translations.get(key).cloned().unwrap_or_else(|| key.to_string())
    }

    fn t_with_args(&self, key: &str, args: Vec<String>) -> String {
        let mut result = self.t(key);
        for (i, arg) in args.iter().enumerate() {
            result = result.replace(&format!("%{}", i + 1), arg);
        }
        result
    }
}

struct Theme {
    name: String,
    short_footer: String,
}

impl Theme {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_short_footer(&self) -> &str {
        &self.short_footer
    }
}

struct OCUtil;

impl OCUtil {
    fn get_version_string() -> String {
        "10.0.0".to_string() // Example version
    }

    fn get_channel() -> String {
        "stable".to_string() // Example channel
    }

    fn get_edition_string() -> String {
        "".to_string() // Example edition string
    }
}

struct OCHelper;

impl OCHelper {
    fn link_to_remote(service: &str) -> String {
        format!("https://example.com/remote/{}", service)
    }

    fn link_to_route(route: &str) -> String {
        format!("/index.php/apps/{}", route)
    }
}

struct OCApp;

impl OCApp {
    fn is_enabled(app_name: &str) -> bool {
        // Example implementation
        match app_name {
            "firstrunwizard" => true,
            _ => false,
        }
    }
}

struct Util;

impl Util {
    fn image_path(app: &str, image: &str) -> String {
        format!("/apps/{}/img/{}", app, image)
    }
}

fn link_to_docs(section: &str) -> String {
    format!("https://docs.example.com/{}", section)
}

pub async fn render_personal_page(data: web::Data<PersonalContext>) -> HttpResponse {
    let context = PersonalTemplate {
        clients: data.clients.clone(),
        usage: data.usage.clone(),
        total_space: data.total_space.clone(),
        usage_relative: data.usage_relative.clone(),
        password_change_supported: data.password_change_supported,
        display_name_change_supported: data.display_name_change_supported,
        display_name: data.display_name.clone(),
        email: data.email.clone(),
        enable_avatars: data.enable_avatars,
        active_language: data.active_language.clone(),
        common_languages: data.common_languages.clone(),
        languages: data.languages.clone(),
        forms: data.forms.clone(),
        enable_decrypt_all: data.enable_decrypt_all,
        theme_name: data.theme.get_name().to_string(),
        version_string: OCUtil::get_version_string(),
        channel: OCUtil::get_channel(),
        edition_string: OCUtil::get_edition_string(),
        short_footer: data.theme.get_short_footer().to_string(),
    };

    match context.render() {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(e) => HttpResponse::InternalServerError().body(format!("Template error: {}", e)),
    }
}