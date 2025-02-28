use std::path::PathBuf;
use std::net::SocketAddr;

use axum::{
    extract::{Multipart, Path},
    routing::{get, post},
    Router,
    response::{Html, IntoResponse, Redirect},
    body::StreamBody,
    http::{header, StatusCode, HeaderValue},
};
use tokio::{
    fs::{self, File},
    io::BufReader,
};
use tokio_util::io::ReaderStream;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Directorio donde se almacenarán los archivos
const UPLOAD_DIR: &str = "uploads";

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

    // Crear rutas
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/upload", post(upload_handler))
        .route("/files/:filename", get(download_handler))
        .route("/delete/:filename", get(delete_handler))
        .nest_service("/static", ServeDir::new("static"));

    // Iniciar servidor
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Escuchando en {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Página principal: listado de archivos y formulario de subida
async fn index_handler() -> Html<String> {
    // Leer los archivos en el directorio de uploads
    let mut entries = fs::read_dir(UPLOAD_DIR).await.unwrap();
    let mut files = Vec::new();

    while let Some(entry) = entries.next_entry().await.unwrap() {
        if let Ok(file_name) = entry.file_name().into_string() {
            files.push(file_name);
        }
    }

    // Construir HTML con lista de archivos y formulario de subida
    let files_html = if files.is_empty() {
        "<p>No hay archivos.</p>".to_string()
    } else {
        let mut html = "<ul>".to_string();
        for file in files {
            html.push_str(&format!(
                r#"<li>
                    {} 
                    <a href="/files/{}" download>Descargar</a> 
                    <a href="/delete/{}" onclick="return confirm('¿Eliminar este archivo?')">Eliminar</a>
                </li>"#,
                file, file, file
            ));
        }
        html.push_str("</ul>");
        html
    };

    // Página HTML completa
    Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>OxiCloud - Almacenamiento Minimalista</title>
            <style>
                body {{ font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }}
                h1 {{ color: #333; }}
                ul {{ list-style-type: none; padding: 0; }}
                li {{ margin: 10px 0; padding: 10px; border: 1px solid #ddd; }}
                a {{ margin-left: 10px; }}
                form {{ margin: 20px 0; padding: 20px; border: 1px solid #ddd; }}
            </style>
        </head>
        <body>
            <h1>OxiCloud</h1>
            <h2>Mis Archivos</h2>
            {}
            
            <form action="/upload" method="post" enctype="multipart/form-data">
                <h2>Subir Archivos</h2>
                <input type="file" name="file" required>
                <button type="submit">Subir</button>
            </form>
        </body>
        </html>
        "#,
        files_html
    ))
}

// Manejador para subir archivos
async fn upload_handler(mut multipart: Multipart) -> impl IntoResponse {
    // Procesar la solicitud multipart
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = if let Some(file_name) = field.file_name() {
            file_name.to_string()
        } else {
            continue;
        };

        let data = field.bytes().await.unwrap();
        
        // Guardar el archivo
        let path = PathBuf::from(UPLOAD_DIR).join(&file_name);
        fs::write(&path, &data).await.unwrap();
        
        tracing::info!("Archivo guardado: {}", file_name);
    }
    
    // Redireccionar a la página principal
    Redirect::to("/").into_response()
}

// Manejador para descargar archivos
async fn download_handler(Path(filename): Path<String>) -> impl IntoResponse {
    let path = PathBuf::from(UPLOAD_DIR).join(&filename);
    
    // Verificar si el archivo existe
    if !path.exists() {
        return (StatusCode::NOT_FOUND, "Archivo no encontrado".to_string()).into_response();
    }
    
    // Abrir el archivo
    let file = match File::open(&path).await {
        Ok(file) => file,
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    };
    
    // Crear un stream para el archivo
    let reader = BufReader::new(file);
    let stream = ReaderStream::new(reader);
    let body = StreamBody::new(stream);
    
    // Construir encabezados para la descarga
    let content_disposition = format!("attachment; filename=\"{}\"", filename);
    let headers = [
        (header::CONTENT_TYPE, HeaderValue::from_static("application/octet-stream")),
        (
            header::CONTENT_DISPOSITION,
            HeaderValue::from_str(&content_disposition).unwrap(),
        ),
    ];
    
    (headers, body).into_response()
}

// Manejador para eliminar archivos
async fn delete_handler(Path(filename): Path<String>) -> impl IntoResponse {
    let path = PathBuf::from(UPLOAD_DIR).join(&filename);
    
    // Verificar si el archivo existe
    if !path.exists() {
        return (StatusCode::NOT_FOUND, "Archivo no encontrado".to_string()).into_response();
    }
    
    // Eliminar el archivo
    if let Err(e) = fs::remove_file(&path).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }
    
    // Redireccionar a la página principal
    Redirect::to("/").into_response()
}
