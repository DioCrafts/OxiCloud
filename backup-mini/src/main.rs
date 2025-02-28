use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use anyhow::Result;
use dotenvy::dotenv;
use log::{info, warn};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tera::Tera;

mod api;
mod core;
mod storage;
mod ui;
mod utils;

#[derive(Clone)]
pub struct AppState {
    db_pool: sqlx::PgPool,
    templates: tera::Tera,
    storage_path: String,
}

#[actix_web::main]
async fn main() -> Result<()> {
    // Inicializar sistema de logs
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    info!("Iniciando OxiCloud...");

    // Cargar variables de entorno
    dotenv().ok();

    // Configurar base de datos
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL debe estar configurada");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Ejecutar migraciones
    sqlx::migrate!("./migrations").run(&db_pool).await?;
    info!("Base de datos configurada correctamente");

    // Inicializar motor de plantillas
    let templates = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            warn!("Error al cargar plantillas: {}", e);
            Tera::default()
        }
    };

    // Definir ruta de almacenamiento
    let storage_path = env::var("STORAGE_PATH").unwrap_or_else(|_| "./data".to_string());
    
    // Asegurarse de que la ruta de almacenamiento exista
    std::fs::create_dir_all(&storage_path)?;
    info!("Directorio de almacenamiento: {}", storage_path);

    // Configurar estado compartido de la aplicación
    let app_state = AppState {
        db_pool,
        templates,
        storage_path,
    };

    // Iniciar servidor HTTP
    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    info!("Iniciando servidor en {}", bind_address);

    HttpServer::new(move || {
        App::new()
            // Estado compartido
            .app_data(web::Data::new(app_state.clone()))
            // Middlewares
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            // Archivos estáticos
            .service(fs::Files::new("/static", "./static").prefer_utf8(true))
            // API Routes
            .service(
                web::scope("/api")
                    .configure(api::auth::config)
                    .configure(api::files::config)
                    .configure(api::users::config),
            )
            // UI Routes
            .service(
                web::scope("")
                    .configure(ui::auth::config)
                    .configure(ui::files::config)
                    .configure(ui::home::config),
            )
    })
    .bind(bind_address)?
    .run()
    .await?;

    info!("Servidor finalizado");
    Ok(())
}