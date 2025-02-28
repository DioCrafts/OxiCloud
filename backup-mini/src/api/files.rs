//! API para gestión de archivos

use std::path::Path;
use actix_web::{web, HttpResponse, HttpRequest};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use futures::StreamExt;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use log::info;

use crate::core::users::User;
use crate::core::files::{FileInfo, FileMetadata, create_file_metadata};
use crate::AppState;
use super::{ApiError, ApiResponse};

/// Configuración de rutas para el módulo de archivos
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/files")
            .route("", web::get().to(list_files))
            .route("", web::post().to(upload_file))
            .route("/{file_id}", web::get().to(get_file))
            .route("/{file_id}", web::delete().to(delete_file))
            .route("/{file_id}/download", web::get().to(download_file)),
    );
}

/// Estructura para responder a peticiones de lista de archivos
#[derive(Serialize)]
struct FileListResponse {
    files: Vec<FileInfo>,
}

/// Parámetros para filtrar la lista de archivos
#[derive(Deserialize)]
struct ListFilesQuery {
    path: Option<String>,
    sort_by: Option<String>,
    order: Option<String>,
}

/// Endpoint para listar archivos
async fn list_files(
    query: web::Query<ListFilesQuery>,
    state: web::Data<AppState>,
    user: web::ReqData<User>,
) -> Result<HttpResponse, ApiError> {
    let path = query.path.clone().unwrap_or_else(|| "/".to_string());
    let user_id = user.id;
    
    // Obtener archivos del usuario en la ruta especificada
    let files = sqlx::query_as!(
        FileInfo,
        r#"
        SELECT id, user_id, filename, path, size, mime_type, is_directory, created_at, updated_at
        FROM files
        WHERE user_id = $1 AND path = $2
        ORDER BY 
            CASE WHEN $3 = 'name' AND $4 = 'asc' THEN filename END ASC,
            CASE WHEN $3 = 'name' AND $4 = 'desc' THEN filename END DESC,
            CASE WHEN $3 = 'date' AND $4 = 'asc' THEN created_at END ASC,
            CASE WHEN $3 = 'date' AND $4 = 'desc' THEN created_at END DESC,
            CASE WHEN $3 = 'size' AND $4 = 'asc' THEN size END ASC,
            CASE WHEN $3 = 'size' AND $4 = 'desc' THEN size END DESC,
            is_directory DESC, filename ASC
        "#,
        user_id,
        path,
        query.sort_by.clone().unwrap_or_else(|| "name".to_string()),
        query.order.clone().unwrap_or_else(|| "asc".to_string())
    )
    .fetch_all(&state.db_pool)
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(FileListResponse { files })))
}

/// Endpoint para obtener información de un archivo
async fn get_file(
    file_id: web::Path<Uuid>,
    state: web::Data<AppState>,
    user: web::ReqData<User>,
) -> Result<HttpResponse, ApiError> {
    let user_id = user.id;
    
    // Obtener información del archivo
    let file = sqlx::query_as!(
        FileInfo,
        r#"
        SELECT id, user_id, filename, path, size, mime_type, is_directory, created_at, updated_at
        FROM files
        WHERE id = $1 AND user_id = $2
        "#,
        file_id.into_inner(),
        user_id
    )
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| ApiError::NotFound("Archivo no encontrado".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(file)))
}

/// Implementación From para convertir PayloadError a ApiError
impl From<actix_web::error::PayloadError> for ApiError {
    fn from(err: actix_web::error::PayloadError) -> Self {
        ApiError::ServerError(format!("Error en el payload: {}", err))
    }
}

/// Implementación From para convertir std::str::Utf8Error a ApiError
impl From<std::str::Utf8Error> for ApiError {
    fn from(err: std::str::Utf8Error) -> Self {
        ApiError::BadRequest(format!("Error al procesar texto UTF-8: {}", err))
    }
}

/// Endpoint para subir un archivo
async fn upload_file(
    mut payload: web::Payload,
    state: web::Data<AppState>,
    user: web::ReqData<User>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    // Extraer información del archivo de los headers
    let content_disposition = req.headers().get("Content-Disposition")
        .ok_or_else(|| ApiError::BadRequest("Falta header Content-Disposition".into()))?
        .to_str()?;
    
    // Extraer nombre del archivo
    let filename = content_disposition
        .split(';')
        .find_map(|part| {
            let part = part.trim();
            if part.starts_with("filename=") {
                Some(part.trim_start_matches("filename=").trim_matches('"'))
            } else {
                None
            }
        })
        .ok_or_else(|| ApiError::BadRequest("Falta nombre de archivo en Content-Disposition".into()))?
        .to_string();
    
    // Extraer ruta (opcional)
    let path = req.headers().get("X-Path")
        .map(|h| h.to_str().unwrap_or("/"))
        .unwrap_or("/")
        .to_string();
    
    // Generar UUID para el archivo
    let file_id = Uuid::new_v4();
    
    // Crear ruta del archivo físico
    let file_path = format!("{}/files/{}", state.storage_path, file_id);
    let mut file = fs::File::create(&file_path).await?;

    // Escribir el archivo
    let mut size: i64 = 0;
    while let Some(chunk) = payload.next().await {
        let data = chunk?;
        size += data.len() as i64;
        file.write_all(&data).await?;
    }
    
    // Determinar tipo MIME
    let mime_type = mime_guess::from_path(&filename)
        .first_or_octet_stream()
        .to_string();
    
    // Crear metadatos en la base de datos
    let file_info = create_file_metadata(
        &state.db_pool,
        FileMetadata {
            id: Some(file_id),
            user_id: user.id,
            filename,
            path,
            size,
            mime_type,
            is_directory: false,
        }
    ).await?;
    
    info!("Archivo subido: {}", file_info.filename);
    
    Ok(HttpResponse::Created().json(ApiResponse::success(file_info)))
}

/// Endpoint para descargar un archivo
async fn download_file(
    file_id: web::Path<Uuid>,
    state: web::Data<AppState>,
    user: web::ReqData<User>,
) -> Result<HttpResponse, ApiError> {
    let user_id = user.id;
    
    // Obtener información del archivo
    let file = sqlx::query_as!(
        FileInfo,
        r#"
        SELECT id, user_id, filename, path, size, mime_type, is_directory, created_at, updated_at
        FROM files
        WHERE id = $1 AND user_id = $2
        "#,
        file_id.into_inner(),
        user_id
    )
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| ApiError::NotFound("Archivo no encontrado".into()))?;
    
    if file.is_directory {
        return Err(ApiError::BadRequest("No se puede descargar un directorio".into()));
    }
    
    // Ruta del archivo físico
    let file_path = format!("{}/files/{}", state.storage_path, file.id);
    
    // Verificar que el archivo existe
    if !Path::new(&file_path).exists() {
        return Err(ApiError::NotFound("Archivo físico no encontrado".into()));
    }
    
    // Leer el archivo
    let file_content = fs::read(&file_path).await?;
    
    Ok(HttpResponse::Ok()
        .content_type(file.mime_type)
        .append_header(("Content-Disposition", format!("attachment; filename=\"{}\"", file.filename)))
        .body(file_content))
}

/// Endpoint para eliminar un archivo
async fn delete_file(
    file_id: web::Path<Uuid>,
    state: web::Data<AppState>,
    user: web::ReqData<User>,
) -> Result<HttpResponse, ApiError> {
    let user_id = user.id;
    let file_id = file_id.into_inner();
    
    // Verificar que el archivo existe y pertenece al usuario
    let file = sqlx::query_as!(
        FileInfo,
        r#"
        SELECT id, user_id, filename, path, size, mime_type, is_directory, created_at, updated_at
        FROM files
        WHERE id = $1 AND user_id = $2
        "#,
        file_id,
        user_id
    )
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| ApiError::NotFound("Archivo no encontrado".into()))?;
    
    // Ruta del archivo físico
    let file_path = format!("{}/files/{}", state.storage_path, file.id);
    
    // Iniciar transacción para asegurar consistencia
    let mut tx = state.db_pool.begin().await?;
    
    // Eliminar metadatos de la base de datos
    sqlx::query!(
        r#"
        DELETE FROM files
        WHERE id = $1 AND user_id = $2
        "#,
        file_id,
        user_id
    )
    .execute(&mut tx)
    .await?;
    
    // Eliminar archivo físico si no es un directorio
    if !file.is_directory && Path::new(&file_path).exists() {
        fs::remove_file(&file_path).await?;
    }
    
    // Commit de la transacción
    tx.commit().await?;
    
    info!("Archivo eliminado: {}", file.filename);
    
    Ok(HttpResponse::Ok().json(ApiResponse::success("Archivo eliminado correctamente")))
}