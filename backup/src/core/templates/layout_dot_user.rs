Es importante señalar que no existe una traducción directa de plantillas PHP a Rust, ya que son conceptos fundamentalmente diferentes. En su lugar, voy a implementar un manejador de plantillas que renderiza esta página en Rust utilizando un enfoque similar a Handlebars o Tera:

use std::collections::HashMap;
use askama::Template;
use actix_web::{web, HttpResponse, Responder};

#[derive(Template)]
#[template(path = "layout_user.html")]
struct UserLayoutTemplate<'a> {
    user_uid: &'a str,
    request_token: &'a str,
    application: Option<&'a str>,
    title: &'a str,
    css_files: Vec<&'a str>,
    js_files: Vec<&'a str>,
    headers: Vec<Header<'a>>,
    body_id: &'a str,
    update_available: bool,
    update_version: Option<&'a str>,
    update_link: Option<&'a str>,
    theme_name: &'a str,
    logo_claim: &'a str>,
    user_display_name: &'a str,
    enable_avatars: bool,
    settings_navigation: Vec<NavigationEntry<'a>>,
    navigation: Vec<NavigationEntry<'a>>,
    is_admin: bool,
    content: &'a str,
    search_query: Option<&'a str>,
}

struct Header<'a> {
    tag: &'a str,
    attributes: HashMap<&'a str, &'a str>,
}

struct NavigationEntry<'a> {
    id: &'a str,
    href: &'a str,
    name: &'a str,
    icon: &'a str,
    active: bool,
}

async fn render_user_layout(
    data: web::Data<AppState>,
    user: web::ReqData<User>,
    req: HttpRequest,
) -> impl Responder {
    let user_uid = user.uid.as_str();
    let request_token = data.security.generate_request_token(user_uid);
    
    let theme = &data.theme;
    let l = &data.l10n;

    let user_display_name = if user.display_name.trim().is_empty() {
        &user.uid
    } else {
        &user.display_name
    };

    let is_admin = data.user_service.is_admin(&user.uid).await.unwrap_or(false);
    
    let update_info = if data.config.update_available {
        Some((
            data.config.update_version.as_deref(),
            data.config.update_link.as_deref()
        ))
    } else {
        None
    };
    
    let search_query = req
        .headers()
        .get("query")
        .and_then(|v| v.to_str().ok());

    let template = UserLayoutTemplate {
        user_uid,
        request_token: &request_token,
        application: data.application.as_deref(),
        title: theme.get_title(),
        css_files: data.css_files.iter().map(|s| s.as_str()).collect(),
        js_files: data.js_files.iter().map(|s| s.as_str()).collect(),
        headers: data.headers.iter().map(|h| Header {
            tag: h.tag.as_str(),
            attributes: h.attributes.iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect(),
        }).collect(),
        body_id: &data.body_id,
        update_available: data.config.update_available,
        update_version: update_info.as_ref().and_then(|i| i.0),
        update_link: update_info.as_ref().and_then(|i| i.1),
        theme_name: theme.get_name(),
        logo_claim: theme.get_logo_claim(),
        user_display_name,
        enable_avatars: data.config.enable_avatars,
        settings_navigation: data.settings_navigation.iter().map(|entry| NavigationEntry {
            id: entry.id.as_str(),
            href: entry.href.as_str(),
            name: entry.name.as_str(),
            icon: entry.icon.as_str(),
            active: entry.active,
        }).collect(),
        navigation: data.navigation.iter().map(|entry| NavigationEntry {
            id: entry.id.as_str(),
            href: entry.href.as_str(),
            name: entry.name.as_str(),
            icon: entry.icon.as_str(),
            active: entry.active,
        }).collect(),
        is_admin,
        content: &data.content,
        search_query,
    };

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(template.render().unwrap_or_else(|e| {
            log::error!("Template rendering error: {}", e);
            "Internal server error".to_string()
        }))
}

// La plantilla HTML se guardaría como templates/layout_user.html
// con la sintaxis correspondiente del motor de plantillas (Askama en este caso)