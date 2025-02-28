//! Gestión de activos estáticos para la UI

use actix_files::Files;
use actix_web::{web, HttpResponse, Result};

/// Configuración de rutas para activos estáticos
pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        Files::new("/static", "./static")
            .prefer_utf8(true)
            .index_file("index.html")
    );
}

/// Middleware para asegurarse de que existen las carpetas estáticas
pub async fn ensure_static_dirs() -> std::io::Result<()> {
    // Directorios a crear si no existen
    let dirs = [
        "static",
        "static/css",
        "static/js",
        "static/img",
        "static/fonts",
    ];
    
    for dir in dirs {
        if !std::path::Path::new(dir).exists() {
            std::fs::create_dir_all(dir)?;
        }
    }
    
    // Crear un archivo CSS básico si no existe
    let css_path = "static/css/main.css";
    if !std::path::Path::new(css_path).exists() {
        std::fs::write(css_path, include_str!("../../templates/static/main.css"))?;
    }
    
    // Crear un archivo JS básico si no existe
    let js_path = "static/js/main.js";
    if !std::path::Path::new(js_path).exists() {
        std::fs::write(js_path, include_str!("../../templates/static/main.js"))?;
    }
    
    Ok(())
}

/// Página de error 404
pub async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../templates/error/404.html")))
}

/// Página de error 500
pub async fn server_error() -> Result<HttpResponse> {
    Ok(HttpResponse::InternalServerError()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../templates/error/500.html")))
}