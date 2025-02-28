/**
 * Copyright (c) 2012, Tom Needham <tom@owncloud.com>
 * This file is licensed under the Affero General Public License version 3 or later.
 * See the COPYING-README file.
 */
use actix_web::{web, HttpResponse};
use askama::Template;
use serde::Deserialize;

// Define el contexto para la plantilla
#[derive(Template)]
#[template(path = "oauth-required-apps.html")]
struct OAuthRequiredAppsTemplate<'a> {
    consumer_name: &'a str,
    message: &'a str,
    required_apps: Vec<&'a str>,
    web_root: &'a str,
}

// Estructura para recibir los datos
#[derive(Deserialize)]
pub struct OAuthRequiredAppsData<'a> {
    consumer: Consumer<'a>,
    message: &'a str,
    required_apps: Vec<&'a str>,
}

#[derive(Deserialize)]
struct Consumer<'a> {
    name: &'a str,
}

// Función que maneja la generación de la página
pub async fn render_oauth_required_apps(
    data: web::Json<OAuthRequiredAppsData<'_>>,
    config: web::Data<AppConfig>,
) -> HttpResponse {
    // Sanitizar datos
    let consumer_name = sanitize_html(data.consumer.name);
    let message = sanitize_html(data.message);
    let required_apps: Vec<String> = data.required_apps
        .iter()
        .map(|app| sanitize_html(app))
        .collect();
    
    // Crear instancia de la plantilla
    let template = OAuthRequiredAppsTemplate {
        consumer_name: &consumer_name,
        message: &message,
        required_apps: required_apps.iter().map(AsRef::as_ref).collect(),
        web_root: &config.web_root,
    };

    // Renderizar y devolver respuesta
    match template.render() {
        Ok(content) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(content),
        Err(err) => {
            log::error!("Error rendering template: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Estructura de configuración de la aplicación
pub struct AppConfig {
    web_root: String,
}

// Función auxiliar para sanitizar HTML
fn sanitize_html(input: &str) -> String {
    // Implementación simplificada - en producción usaría una biblioteca de sanitización
    // como ammonia o html-escape
    input.replace('<', "&lt;").replace('>', "&gt;")
}

// La plantilla HTML correspondiente (oauth-required-apps.html):
/*
<div id="oauth-request" class="guest-container">
    <p><strong>{{ consumer_name | safe }}</strong> {{ message | safe }}</p>
    <ul>
        {% for app in required_apps %}
            <li>{{ app | safe }}</li>
        {% endfor %}
    </ul>
    <a href="{{ web_root }}" id="back-home" class="button">Back to ownCloud</a>
</div>
*/