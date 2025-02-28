use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use log::{error};
use anyhow::{Result, Context};

// Configuración equivalente al RUNTIME_NOAPPS
const RUNTIME_NOAPPS: bool = true;

/// Estructura para almacenar la configuración
struct Config {
    installed: bool,
}

impl Config {
    fn get_value(key: &str) -> Option<bool> {
        // Simulación de OC_Config::getValue
        match key {
            "installed" => Some(true), // Valor de ejemplo, en producción se obtendría de la configuración real
            _ => None,
        }
    }
}

/// Utilidades del sistema
struct Util;

impl Util {
    fn get_version() -> Vec<u32> {
        // Simulación de OC_Util::getVersion()
        vec![10, 0, 0] // Ejemplo: versión 10.0.0
    }

    fn get_version_string() -> String {
        // Simulación de OC_Util::getVersionString()
        "10.0.0".to_string()
    }

    fn get_edition_string() -> String {
        // Simulación de OC_Util::getEditionString()
        "Community".to_string()
    }
}

/// Respuesta de estado del servidor
#[derive(Serialize, Deserialize)]
struct StatusResponse {
    installed: String,
    version: String,
    versionstring: String,
    edition: String,
}

/// Manejo de respuestas HTTP
struct Response;

impl Response {
    const STATUS_INTERNAL_SERVER_ERROR: u16 = 500;
    
    fn set_status(status: u16) -> HttpResponse {
        match status {
            Self::STATUS_INTERNAL_SERVER_ERROR => HttpResponse::InternalServerError().finish(),
            _ => HttpResponse::Ok().finish(),
        }
    }
}

/// Utilidades de registro
struct Logger;

impl Logger {
    const FATAL: u8 = 4;
    
    fn write_log(category: &str, message: &str, level: u8) {
        if level == Self::FATAL {
            error!("[{}] {}", category, message);
        }
    }
}

#[get("/status")]
async fn status() -> impl Responder {
    match get_status_data() {
        Ok(status_data) => {
            // Determinar si estamos en CLI o modo web
            let is_cli = env::var("CLI").unwrap_or_else(|_| "false".to_string()) == "true";
            
            if is_cli {
                // En modo CLI, devolver texto plano
                HttpResponse::Ok()
                    .content_type("text/plain")
                    .body(format!("{:?}", status_data))
            } else {
                // En modo web, devolver JSON
                HttpResponse::Ok()
                    .content_type("application/json")
                    .json(status_data)
            }
        },
        Err(e) => {
            Logger::write_log("remote", &e.to_string(), Logger::FATAL);
            Response::set_status(Response::STATUS_INTERNAL_SERVER_ERROR)
        }
    }
}

/// Obtiene los datos de estado del servidor
fn get_status_data() -> Result<StatusResponse> {
    // Simulamos que cargamos la configuración base
    // Esto sería equivalente a require_once 'lib/base.php';
    
    let installed = if Config::get_value("installed").unwrap_or(false) {
        "true".to_string()
    } else {
        "false".to_string()
    };
    
    let version = Util::get_version().iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(".");
    
    Ok(StatusResponse {
        installed,
        version,
        versionstring: Util::get_version_string(),
        edition: Util::get_edition_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    HttpServer::new(|| {
        App::new()
            .service(status)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}