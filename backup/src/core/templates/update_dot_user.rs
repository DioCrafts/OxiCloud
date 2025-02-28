use actix_web::{HttpResponse, web};
use askama::Template;
use crate::i18n::Locale;

#[derive(Template)]
#[template(path = "update_user.html")]
struct UpdateUserTemplate<'a> {
    l: &'a Locale,
}

pub async fn update_user(locale: web::Data<Locale>) -> HttpResponse {
    let template = UpdateUserTemplate {
        l: &locale,
    };
    
    match template.render() {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(_) => HttpResponse::InternalServerError()
            .body("Failed to render template"),
    }
}