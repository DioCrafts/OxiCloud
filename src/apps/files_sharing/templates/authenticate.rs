use actix_web::{web, HttpResponse};
use askama::Template;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "authenticate.html")]
struct AuthenticateTemplate {
    url: String,
    wrong_pw: Option<bool>,
    l: std::sync::Arc<i18n::Translator>,
}

#[derive(Deserialize)]
pub struct AuthenticateParams {
    url: String,
    wrong_pw: Option<bool>,
}

pub async fn authenticate(
    query: web::Query<AuthenticateParams>,
    translator: web::Data<std::sync::Arc<i18n::Translator>>,
) -> HttpResponse {
    let template = AuthenticateTemplate {
        url: query.url.clone(),
        wrong_pw: query.wrong_pw,
        l: translator.into_inner(),
    };

    match template.render() {
        Ok(body) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Template file: templates/authenticate.html
/*
<form action="{{ url }}" method="post">
    <fieldset>
        {% if wrong_pw.is_none() %}
            <div class="warning-info">{{ l.t("This share is password-protected") }}</div>
        {% endif %}
        {% if wrong_pw.is_some() %}
            <div class="warning">{{ l.t("The password is wrong. Try again.") }}</div>
        {% endif %}
        <p class="infield">
            <label for="password" class="infield">{{ l.t("Password") }}</label>
            <input type="password" name="password" id="password" placeholder="" value="" autofocus />
            <input type="submit" value="" class="svg" />
        </p>
    </fieldset>
</form>
*/