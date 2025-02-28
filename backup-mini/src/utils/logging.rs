//! Configuración del sistema de logs

use log::{info, error, LevelFilter};
use env_logger::{Builder, Env};
use std::io::Write;
use chrono::Local;

/// Inicializa el sistema de logs
pub fn init_logging() {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "info")
        .write_style_or("LOG_STYLE", "auto");
    
    let mut builder = Builder::from_env(env);
    
    // Configurar formato de logs con timestamp
    builder.format(|buf, record| {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        writeln!(
            buf,
            "[{} {} {}:{}] {}",
            timestamp,
            record.level(),
            record.file().unwrap_or("unknown"),
            record.line().unwrap_or(0),
            record.args()
        )
    });
    
    // Establecer nivel de log predeterminado
    builder.filter_level(LevelFilter::Info);
    
    // Inicializar logger
    builder.init();
    
    info!("Sistema de logs inicializado");
}

/// Configura el manejo de pánico para logging de errores críticos
pub fn setup_panic_handler() {
    std::panic::set_hook(Box::new(|panic_info| {
        let (filename, line) = panic_info
            .location()
            .map(|loc| (loc.file(), loc.line()))
            .unwrap_or(("<unknown>", 0));
        
        let cause = panic_info
            .payload()
            .downcast_ref::<String>()
            .map(|s| s.as_str())
            .or_else(|| panic_info.payload().downcast_ref::<&str>().copied())
            .unwrap_or("<causa desconocida>");
        
        error!("PÁNICO en {}:{}: {}", filename, line, cause);
    }));
    
    info!("Manejador de pánico configurado");
}

/// Ruta para archivos de log
pub fn get_log_path() -> std::path::PathBuf {
    let log_dir = std::env::var("LOG_DIR").unwrap_or_else(|_| "logs".to_string());
    let log_dir_path = std::path::Path::new(&log_dir);
    
    // Crear directorio de logs si no existe
    if !log_dir_path.exists() {
        std::fs::create_dir_all(log_dir_path).expect("No se pudo crear el directorio de logs");
    }
    
    // Generar nombre de archivo con fecha
    let date = Local::now().format("%Y-%m-%d");
    log_dir_path.join(format!("oxicloud-{}.log", date))
}

/// Configura el logging a archivo
pub fn setup_file_logging() -> Result<(), std::io::Error> {
    let log_path = get_log_path();
    
    // Crear archivo de log
    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)?;
    
    // Configurar builder
    let mut builder = Builder::new();
    
    // Formato de log
    builder.format(|buf, record| {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        writeln!(
            buf,
            "[{} {} {}:{}] {}",
            timestamp,
            record.level(),
            record.file().unwrap_or("unknown"),
            record.line().unwrap_or(0),
            record.args()
        )
    });
    
    // Establecer nivel y destino
    builder
        .filter(None, LevelFilter::Info)
        .target(env_logger::Target::Pipe(Box::new(file)))
        .init();
    
    info!("Logging a archivo configurado: {}", log_path.display());
    Ok(())
}