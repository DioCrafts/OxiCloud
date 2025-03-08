use std::path::PathBuf;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::UNIX_EPOCH;

use axum::{
    extract::{Multipart, Path, State},
    routing::{get, post},
    Router,
    response::{Html, IntoResponse, Redirect, Json},
    body::StreamBody,
    http::{header, StatusCode, HeaderValue},
};
use serde::{Deserialize, Serialize};
use tokio::{
    fs::{self, File},
    io::BufReader,
    sync::RwLock,
};
use tokio_util::io::ReaderStream;
use tower_http::{
    services::ServeDir,
    compression::CompressionLayer,
    trace::TraceLayer,
    cors::CorsLayer,
};
use mime_guess::from_path;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Configuración
const UPLOAD_DIR: &str = "uploads";
const CHUNK_SIZE: usize = 64 * 1024; // 64KB para streaming

// Tipos de datos para APIs
#[derive(Debug, Serialize, Deserialize, Clone)]
struct FileInfo {
    name: String,
    size: u64,
    mime_type: String,
    last_modified: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct StorageInfo {
    total_files: usize,
    total_size: u64,
    quota: u64,
}

// Estado de la aplicación
#[derive(Clone)]
struct AppState {
    file_cache: Arc<RwLock<Vec<FileInfo>>>,
    storage_info: Arc<RwLock<StorageInfo>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            file_cache: Arc::new(RwLock::new(Vec::new())),
            storage_info: Arc::new(RwLock::new(StorageInfo {
                total_files: 0,
                total_size: 0,
                quota: 1024 * 1024 * 1024, // 1GB
            })),
        }
    }
}

#[tokio::main]
async fn main() {
    // Inicializar logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Crear directorio de uploads si no existe
    fs::create_dir_all(UPLOAD_DIR)
        .await
        .expect("No se pudo crear el directorio de uploads");

    // Inicializar estado de la aplicación
    let app_state = AppState::new();
    
    // Cargar datos iniciales de archivos
    reload_file_cache(&app_state).await;

    // Crear rutas
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/api/files", get(files_api_handler))
        .route("/api/storage", get(storage_api_handler))
        .route("/upload", post(upload_handler))
        .route("/files/:filename", get(download_handler))
        .route("/delete/:filename", get(delete_handler))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(app_state)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    // Iniciar servidor
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Servidor iniciado en {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Recargar la cache de archivos
async fn reload_file_cache(app_state: &AppState) {
    let mut entries = match fs::read_dir(UPLOAD_DIR).await {
        Ok(entries) => entries,
        Err(e) => {
            tracing::error!("Error al leer directorio: {}", e);
            return;
        }
    };

    let mut files = Vec::new();
    let mut total_size = 0;

    while let Some(entry) = entries.next_entry().await.unwrap_or(None) {
        let path = entry.path();
        
        if path.is_file() {
            if let Ok(metadata) = fs::metadata(&path).await {
                let name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("desconocido")
                    .to_string();
                
                let mime_type = from_path(&path)
                    .first_or_octet_stream()
                    .to_string();
                
                let size = metadata.len();
                total_size += size;
                
                let last_modified = metadata.modified()
                    .map(|time| time.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs())
                    .unwrap_or(0);
                
                files.push(FileInfo {
                    name,
                    size,
                    mime_type,
                    last_modified,
                });
            }
        }
    }

    // Actualizar cache y stats
    let mut file_cache = app_state.file_cache.write().await;
    *file_cache = files;
    
    let mut storage_info = app_state.storage_info.write().await;
    storage_info.total_files = file_cache.len();
    storage_info.total_size = total_size;
}

// Manejador de la página principal
async fn index_handler() -> Html<String> {
    // Leer el archivo HTML
    Html(include_str!("../static/index.html").to_string())
}

// API para obtener información de archivos
async fn files_api_handler(
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let file_cache = app_state.file_cache.read().await;
    
    // Aquí puedes implementar la búsqueda cuando tengas todo funcionando
    let files = file_cache.clone();
    
    Json(files)
}

// API para obtener información de almacenamiento
async fn storage_api_handler(
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let storage_info = app_state.storage_info.read().await;
    Json((*storage_info).clone())
}

// Manejador para subir archivos
async fn upload_handler(
    State(app_state): State<AppState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    // Procesar la solicitud multipart
    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let file_name = match field.file_name() {
            Some(name) => name.to_string(),
            None => continue,
        };
        
        // Crear ruta de destino
        let path = PathBuf::from(UPLOAD_DIR).join(&file_name);
        
        // Leer el campo como bytes y guardar
        let data = match field.bytes().await {
            Ok(data) => data,
            Err(e) => {
                tracing::error!("Error al leer datos: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Error al leer datos").into_response();
            }
        };
        
        if let Err(e) = fs::write(&path, &data).await {
            tracing::error!("Error al escribir archivo: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Error al guardar archivo").into_response();
        }
        
        tracing::info!("Archivo guardado: {}", file_name);
    }
    
    // Actualizar cache de archivos
    reload_file_cache(&app_state).await;
    
    // Redireccionar a la página principal
    Redirect::to("/").into_response()
}

// Manejador para descargar archivos
async fn download_handler(
    Path(filename): Path<String>,
) -> impl IntoResponse {
    let path = PathBuf::from(UPLOAD_DIR).join(&filename);
    
    // Verificar si el archivo existe
    if !path.exists() {
        return (StatusCode::NOT_FOUND, "Archivo no encontrado").into_response();
    }
    
    // Obtener el tipo MIME del archivo
    let mime_type = from_path(&path)
        .first_or_octet_stream()
        .to_string();
    
    // Abrir el archivo
    let file = match File::open(&path).await {
        Ok(file) => file,
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    };
    
    let metadata = match file.metadata().await {
        Ok(metadata) => metadata,
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    };
    
    // Crear un stream para el archivo con un tamaño de buffer óptimo
    let reader = BufReader::with_capacity(CHUNK_SIZE, file);
    let stream = ReaderStream::new(reader);
    let body = StreamBody::new(stream);
    
    // Construir encabezados para la descarga
    let content_disposition = format!("attachment; filename=\"{}\"", filename);
    let headers = [
        (header::CONTENT_TYPE, HeaderValue::from_str(&mime_type).unwrap_or(HeaderValue::from_static("application/octet-stream"))),
        (header::CONTENT_DISPOSITION, HeaderValue::from_str(&content_disposition).unwrap_or(HeaderValue::from_static("attachment"))),
        (header::CONTENT_LENGTH, HeaderValue::from_str(&metadata.len().to_string()).unwrap_or(HeaderValue::from_static("0"))),
    ];
    
    (headers, body).into_response()
}

// Manejador para eliminar archivos
async fn delete_handler(
    State(app_state): State<AppState>,
    Path(filename): Path<String>,
) -> impl IntoResponse {
    let path = PathBuf::from(UPLOAD_DIR).join(&filename);
    
    // Verificar si el archivo existe
    if !path.exists() {
        return (StatusCode::NOT_FOUND, "Archivo no encontrado").into_response();
    }
    
    // Eliminar el archivo
    if let Err(e) = fs::remove_file(&path).await {
        tracing::error!("Error al eliminar archivo: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }
    
    // Actualizar cache
    reload_file_cache(&app_state).await;
    
    // Redireccionar a la página principal
    Redirect::to("/").into_response()
}
