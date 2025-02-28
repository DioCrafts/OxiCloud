//! Gestión de configuración para OxiCloud

use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;
use log::{info, warn};
use anyhow::{Result, Context};

/// Configuración global de la aplicación
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// Configuración de la base de datos
    pub database: DatabaseConfig,
    
    /// Configuración del servidor
    pub server: ServerConfig,
    
    /// Configuración de almacenamiento
    pub storage: StorageConfig,
    
    /// Configuración de seguridad
    pub security: SecurityConfig,
}

/// Configuración de la base de datos
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// URL de conexión a la base de datos
    pub url: String,
    
    /// Número máximo de conexiones en el pool
    pub max_connections: u32,
}

/// Configuración del servidor
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Dirección de escucha
    pub host: String,
    
    /// Puerto de escucha
    pub port: u16,
    
    /// Habilitar HTTPS
    pub use_https: bool,
    
    /// Ruta al certificado SSL (si use_https = true)
    pub cert_file: Option<String>,
    
    /// Ruta a la clave privada SSL (si use_https = true)
    pub key_file: Option<String>,
}

/// Configuración de almacenamiento
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Directorio raíz para archivos
    pub data_dir: String,
    
    /// Tamaño máximo de archivo en bytes (0 = sin límite)
    pub max_file_size: u64,
    
    /// Habilitar versionado de archivos
    pub enable_versioning: bool,
}

/// Configuración de seguridad
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Secreto para firmar tokens JWT
    pub jwt_secret: String,
    
    /// Tiempo de expiración de tokens en minutos
    pub jwt_expiry_minutes: i64,
    
    /// Número de rondas para la función de hash bcrypt
    pub password_hash_rounds: u32,
}

// Singleton para la configuración global
static CONFIG: OnceLock<Config> = OnceLock::new();

/// Obtiene la configuración global, cargándola si es necesario
pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| {
        match load_config() {
            Ok(config) => config,
            Err(e) => {
                warn!("Error al cargar configuración: {}. Usando valores por defecto.", e);
                default_config()
            }
        }
    })
}

/// Carga la configuración desde variables de entorno o archivo
fn load_config() -> Result<Config> {
    // Cargar variables de entorno
    dotenv().ok();
    
    // Intentar cargar desde archivo de configuración
    if let Ok(config_path) = env::var("CONFIG_FILE") {
        return load_config_from_file(&config_path)
            .context("Error al cargar configuración desde archivo");
    }
    
    // Cargar desde variables de entorno
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/oxicloud".to_string());
    
    let max_connections = env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "5".to_string())
        .parse::<u32>()
        .unwrap_or(5);
    
    let host = env::var("SERVER_HOST")
        .unwrap_or_else(|_| "127.0.0.1".to_string());
    
    let port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    
    let use_https = env::var("SERVER_USE_HTTPS")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);
    
    let cert_file = env::var("SERVER_CERT_FILE").ok();
    let key_file = env::var("SERVER_KEY_FILE").ok();
    
    let data_dir = env::var("STORAGE_DATA_DIR")
        .unwrap_or_else(|_| "./data".to_string());
    
    let max_file_size = env::var("STORAGE_MAX_FILE_SIZE")
        .unwrap_or_else(|_| "0".to_string())
        .parse::<u64>()
        .unwrap_or(0);
    
    let enable_versioning = env::var("STORAGE_ENABLE_VERSIONING")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);
    
    let jwt_secret = env::var("SECURITY_JWT_SECRET")
        .unwrap_or_else(|_| {
            warn!("¡ADVERTENCIA! No se ha configurado SECURITY_JWT_SECRET. Usando un valor predeterminado inseguro.");
            "oxicloud_development_secret_key".to_string()
        });
    
    let jwt_expiry_minutes = env::var("SECURITY_JWT_EXPIRY_MINUTES")
        .unwrap_or_else(|_| "60".to_string())
        .parse::<i64>()
        .unwrap_or(60);
    
    let password_hash_rounds = env::var("SECURITY_PASSWORD_HASH_ROUNDS")
        .unwrap_or_else(|_| "10".to_string())
        .parse::<u32>()
        .unwrap_or(10);
    
    Ok(Config {
        database: DatabaseConfig {
            url: database_url,
            max_connections,
        },
        server: ServerConfig {
            host,
            port,
            use_https,
            cert_file,
            key_file,
        },
        storage: StorageConfig {
            data_dir,
            max_file_size,
            enable_versioning,
        },
        security: SecurityConfig {
            jwt_secret,
            jwt_expiry_minutes,
            password_hash_rounds,
        },
    })
}

/// Carga configuración desde un archivo JSON o TOML
fn load_config_from_file(path: &str) -> Result<Config> {
    let config_path = PathBuf::from(path);
    let content = fs::read_to_string(&config_path)?;
    
    // Determinar formato por extensión
    match config_path.extension().and_then(|ext| ext.to_str()) {
        Some("json") => {
            serde_json::from_str(&content).context("Error al parsear JSON")
        },
        Some("toml") => {
            toml::from_str(&content).context("Error al parsear TOML")
        },
        _ => {
            // Intentar como JSON primero, luego TOML
            serde_json::from_str(&content)
                .or_else(|_| toml::from_str(&content))
                .context("Error al parsear configuración, formato desconocido")
        }
    }
}

/// Genera una configuración por defecto
fn default_config() -> Config {
    Config {
        database: DatabaseConfig {
            url: "postgres://postgres:postgres@localhost/oxicloud".to_string(),
            max_connections: 5,
        },
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            use_https: false,
            cert_file: None,
            key_file: None,
        },
        storage: StorageConfig {
            data_dir: "./data".to_string(),
            max_file_size: 0,
            enable_versioning: false,
        },
        security: SecurityConfig {
            jwt_secret: "oxicloud_development_secret_key".to_string(),
            jwt_expiry_minutes: 60,
            password_hash_rounds: 10,
        },
    }
}

/// Inicializa el sistema de configuración
pub fn init() -> Result<()> {
    let config = get_config();
    
    // Asegurar que existe el directorio de datos
    let data_dir = PathBuf::from(&config.storage.data_dir);
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
        info!("Directorio de datos creado: {}", data_dir.display());
    }
    
    // Crear subdirectorios necesarios
    let files_dir = data_dir.join("files");
    if !files_dir.exists() {
        fs::create_dir_all(&files_dir)?;
        info!("Directorio de archivos creado: {}", files_dir.display());
    }
    
    let temp_dir = data_dir.join("temp");
    if !temp_dir.exists() {
        fs::create_dir_all(&temp_dir)?;
        info!("Directorio temporal creado: {}", temp_dir.display());
    }
    
    Ok(())
}