use actix_web::{web, App, HttpServer, HttpResponse, Error, middleware, http::StatusCode};
use anyhow::{Result, anyhow, Context};
use std::path::{Path, PathBuf};
use log::{error, info};
use std::env;

mod oc {
    pub mod response {
        pub const STATUS_NOT_FOUND: u16 = 404;
        pub const STATUS_INTERNAL_SERVER_ERROR: u16 = 500;
    }

    pub struct Request;
    
    impl Request {
        pub fn get_path_info() -> Option<String> {
            // En un entorno real, esto vendría de la solicitud HTTP
            // En este ejemplo, recuperamos de una variable de entorno
            std::env::var("PATH_INFO").ok()
        }
    }

    pub struct AppConfig;
    
    impl AppConfig {
        pub fn get_value(app: &str, key: &str) -> Option<String> {
            // Simulación de la consulta de configuración
            // En una implementación real, esto leería de una base de datos o archivo de configuración
            match (app, key) {
                // Valores de ejemplo
                ("core", key) if key.starts_with("remote_") => Some(String::from("core/file_handler")),
                _ => None,
            }
        }
    }

    pub struct Util;
    
    impl Util {
        pub fn check_app_enabled(app: &str) -> Result<(), anyhow::Error> {
            // Simulación de verificación de aplicación
            if app == "disabled_app" {
                return Err(anyhow!("App {} is disabled", app));
            }
            Ok(())
        }
        
        pub fn write_log(category: &str, message: &str, level: u8) {
            match level {
                0 => log::debug!("[{}] {}", category, message),
                1 => log::info!("[{}] {}", category, message),
                2 => log::warn!("[{}] {}", category, message),
                3 => log::error!("[{}] {}", category, message),
                4 => log::error!("[{}] FATAL: {}", category, message),
                _ => log::trace!("[{}] {}", category, message),
            }
        }
    }

    pub struct App;
    
    impl App {
        pub fn load_app(app: &str) -> Result<(), anyhow::Error> {
            // Simulación de carga de aplicación
            Ok(())
        }
        
        pub fn get_app_path(app: &str) -> String {
            // Simulación de ruta de aplicación
            format!("apps/{}", app)
        }
    }

    pub struct WEBROOT;
    pub struct SERVERROOT;
    
    impl WEBROOT {
        pub fn get() -> String {
            std::env::var("WEBROOT").unwrap_or_else(|_| String::from(""))
        }
    }
    
    impl SERVERROOT {
        pub fn get() -> String {
            std::env::var("SERVERROOT").unwrap_or_else(|_| String::from("/var/www/html"))
        }
    }

    pub struct Template;
    
    impl Template {
        pub fn print_exception_error_page(error: &anyhow::Error) -> HttpResponse {
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(format!("<html><body><h1>Internal Server Error</h1><p>{}</p></body></html>", 
                       error))
        }
    }
}

async fn handle_remote_request() -> Result<HttpResponse, Error> {
    // Configuración de runtime
    let runtime_noapps = true;
    
    match process_remote_request().await {
        Ok(response) => Ok(response),
        Err(err) => {
            error!("Remote request error: {}", err);
            oc::Util::write_log("remote", &err.to_string(), 4);
            Ok(oc::Template::print_exception_error_page(&err))
        }
    }
}

async fn process_remote_request() -> Result<HttpResponse, anyhow::Error> {
    // Obtener la información de la ruta
    let path_info = oc::Request::get_path_info()
        .filter(|p| !p.is_empty())
        .ok_or_else(|| anyhow!("Path info not found"))?;
    
    // Encontrar la primera barra después del primer carácter
    let pos = path_info[1..].find('/').map(|p| p + 1).unwrap_or(path_info.len());
    
    // Extraer el servicio de la ruta
    let service = &path_info[1..pos];
    
    // Obtener el archivo de configuración para este servicio
    let file = oc::AppConfig::get_value("core", &format!("remote_{}", service))
        .ok_or_else(|| anyhow!("Remote service not found: {}", service))?;
    
    // Limpiar la ruta del archivo
    let file = file.trim_start_matches('/');
    
    // Dividir la ruta en componentes de aplicación y archivo
    let parts: Vec<&str> = file.splitn(2, '/').collect();
    if parts.len() < 2 {
        return Err(anyhow!("Invalid file path format"));
    }
    
    let app = parts[0];
    let app_file = parts[1];
    
    // Construir la ruta del archivo según la aplicación
    let file_path = match app {
        "core" => {
            PathBuf::from(format!("{}/{}", oc::SERVERROOT::get(), file))
        },
        _ => {
            // Verificar y cargar la aplicación
            oc::Util::check_app_enabled(app)?;
            oc::App::load_app(app)?;
            
            // Construir la ruta basada en el sistema operativo
            if cfg!(windows) {
                PathBuf::from(format!("{}/{}", oc::App::get_app_path(app), app_file))
            } else {
                PathBuf::from(format!("/{}/{}", oc::App::get_app_path(app), app_file))
            }
        }
    };
    
    // Construir la URI base para el servicio
    let base_uri = format!("{}/remote.php/{}/", oc::WEBROOT::get(), service);
    
    // En PHP, esto cargaría el archivo. En Rust, tendríamos que implementar un mecanismo
    // para manejar estos archivos dinámicamente. Aquí simulamos ese comportamiento.
    if file_path.exists() {
        // En un escenario real, esta parte dependería de qué hace el archivo PHP cargado
        Ok(HttpResponse::Ok().body(format!("Successfully processed request for service: {}", service)))
    } else {
        Err(anyhow!("File not found: {:?}", file_path))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inicializar sistema de logs
    env_logger::init();
    
    info!("Starting remote service");
    
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .default_service(web::to(handle_remote_request))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}