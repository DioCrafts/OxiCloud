use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::path::Path;

async fn handle_trashbin_preview() -> impl Responder {
    // Equivalente a incluir el archivo PHP
    // La ruta se maneja de forma relativa desde el directorio actual
    let ajax_preview_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("ajax")
        .join("preview.rs");
    
    // En una aplicación real, aquí se invocaría la funcionalidad de preview.rs
    // Por ahora, devolvemos una respuesta simulada
    HttpResponse::Ok()
        .content_type("image/png")
        .body(Vec::<u8>::new()) // Contenido vacío como placeholder
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/preview.png", web::get().to(handle_trashbin_preview));
}