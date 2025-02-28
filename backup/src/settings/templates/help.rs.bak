use actix_web::{web, HttpResponse, Result};
use askama::Template;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "help.html")]
struct HelpTemplate {
    admin: bool,
    style1: String,
    style2: String,
    url1: String,
    url2: String,
    url: String,
}

#[derive(Deserialize)]
pub struct HelpParams {
    admin: Option<bool>,
    style1: Option<String>,
    style2: Option<String>,
    url1: Option<String>,
    url2: Option<String>,
    url: String,
}

pub async fn render_help(query: web::Query<HelpParams>) -> Result<HttpResponse> {
    let admin = query.admin.unwrap_or(false);
    let style1 = query.style1.clone().unwrap_or_default();
    let style2 = query.style2.clone().unwrap_or_default();
    let url1 = query.url1.clone().unwrap_or_default();
    let url2 = query.url2.clone().unwrap_or_default();
    let url = query.url.clone();

    let template = HelpTemplate {
        admin,
        style1,
        style2,
        url1,
        url2,
        url,
    };

    let html = template.render().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Template error: {}", e))
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}