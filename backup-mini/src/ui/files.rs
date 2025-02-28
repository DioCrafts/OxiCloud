//! UI para gestión de archivos

use actix_web::{web, HttpResponse, Responder, HttpRequest, http::header};
use log::error;
use serde::{Serialize, Deserialize};
use std::path::Path;
use uuid::Uuid;

use crate::{
    core::{
        files::{FileInfo, list_files, get_file_by_id},
    },
    utils::auth::extract_user_from_session,
    AppState,
};

use super::{render_template, PageContext, FlashMessage};

/// Configuración de rutas para la UI de archivos
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/files")
            .route(web::get().to(files_page))
            .route(web::post().to(upload_file)),
    )
    .service(
        web::resource("/files/{file_id}")
            .route(web::get().to(file_details))
            .route(web::delete().to(delete_file)),
    )
    .service(web::resource("/files/{file_id}/download").to(download_file))
    .service(web::resource("/files/mkdir").to(create_directory));
}

/// Datos para la página de archivos
#[derive(Serialize)]
struct FilesPageData {
    current_path: String,
    parent_path: Option<String>,
    files: Vec<FileInfo>,
}

/// Datos para crear un directorio
#[derive(Deserialize)]
struct CreateDirForm {
    path: String,
    name: String,
}

/// Página principal de archivos
async fn files_page(
    state: web::Data<AppState>,
    req: HttpRequest,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    // Obtener usuario de la sesión
    let user = match extract_user_from_session(&state.db_pool, &req).await {
        Ok(user) => user,
        Err(_) => {
            // Redirigir a login si no está autenticado
            return HttpResponse::Found()
                .append_header(("Location", "/login"))
                .finish();
        }
    };
    
    // Obtener path de la consulta o usar raíz
    let path = query.get("path").cloned().unwrap_or_else(|| "/".to_string());
    
    // Listar archivos en la ruta
    let files = match list_files(&state.db_pool, user.id, &path).await {
        Ok(files) => files,
        Err(e) => {
            error!("Error al listar archivos: {}", e);
            
            let ctx = PageContext::new("Archivos", FilesPageData {
                current_path: path.clone(),
                parent_path: get_parent_path(&path),
                files: Vec::new(),
            })
            .with_user(Some(user))
            .with_flash(FlashMessage::error("Error al listar archivos"))
            .into_context();
            
            return match render_template("files/index.html", &ctx, &state) {
                Ok(response) => response,
                Err(_) => HttpResponse::InternalServerError()
                    .body("Error al renderizar la plantilla"),
            };
        }
    };
    
    // Crear contexto para la plantilla
    let ctx = PageContext::new("Archivos", FilesPageData {
        current_path: path.clone(),
        parent_path: get_parent_path(&path),
        files,
    })
    .with_user(Some(user))
    .into_context();
    
    match render_template("files/index.html", &ctx, &state) {
        Ok(response) => response,
        Err(_) => HttpResponse::InternalServerError().body("Error al renderizar la plantilla"),
    }
}

/// Obtiene la ruta padre de una ruta
fn get_parent_path(path: &str) -> Option<String> {
    if path == "/" {
        return None;
    }
    
    let path = path.trim_end_matches('/');
    match path.rfind('/') {
        Some(pos) => {
            let parent = &path[..=pos];
            if parent == "/" {
                Some("/".to_string())
            } else {
                Some(parent.to_string())
            }
        }
        None => Some("/".to_string()),
    }
}

/// Subir un archivo
async fn upload_file(
    state: web::Data<AppState>,
    req: HttpRequest,
    _payload: web::Payload,
) -> impl Responder {
    // Obtener usuario de la sesión
    let _user = match extract_user_from_session(&state.db_pool, &req).await {
        Ok(user) => user,
        Err(_) => {
            // Redirigir a login si no está autenticado
            return HttpResponse::Found()
                .append_header(("Location", "/login"))
                .finish();
        }
    };
    
    // Este es un placeholder. Necesitarás importar actix-multipart
    // y reescribir esta función para manejar la subida de archivos correctamente.
    let path = "/".to_string();
    
    HttpResponse::Found()
        .append_header(("Location", format!("/files?path={}", path)))
        .finish()
}

/// Página de detalles de un archivo
async fn file_details(
    state: web::Data<AppState>,
    req: HttpRequest,
    file_id: web::Path<Uuid>,
) -> impl Responder {
    // Obtener usuario de la sesión
    let user = match extract_user_from_session(&state.db_pool, &req).await {
        Ok(user) => user,
        Err(_) => {
            // Redirigir a login si no está autenticado
            return HttpResponse::Found()
                .append_header(("Location", "/login"))
                .finish();
        }
    };
    
    // Obtener información del archivo
    let file_id = file_id.into_inner();
    let file = match get_file_by_id(&state.db_pool, file_id).await {
        Ok(Some(file)) if file.user_id == user.id => file,
        Ok(Some(_)) => {
            // Archivo pertenece a otro usuario
            return HttpResponse::Found()
                .append_header(("Location", "/files?error=permission"))
                .finish();
        }
        Ok(None) => {
            // Archivo no encontrado
            return HttpResponse::Found()
                .append_header(("Location", "/files?error=notfound"))
                .finish();
        }
        Err(e) => {
            error!("Error al obtener archivo: {}", e);
            return HttpResponse::Found()
                .append_header(("Location", "/files?error=database"))
                .finish();
        }
    };
    
    // Crear contexto para la plantilla
    let ctx = PageContext::new(format!("Archivo: {}", file.filename), file)
        .with_user(Some(user))
        .into_context();
    
    match render_template("files/details.html", &ctx, &state) {
        Ok(response) => response,
        Err(_) => HttpResponse::InternalServerError().body("Error al renderizar la plantilla"),
    }
}

/// Descargar un archivo
async fn download_file(
    state: web::Data<AppState>,
    req: HttpRequest,
    file_id: web::Path<Uuid>,
) -> impl Responder {
    // Obtener usuario de la sesión
    let user = match extract_user_from_session(&state.db_pool, &req).await {
        Ok(user) => user,
        Err(_) => {
            // Redirigir a login si no está autenticado
            return HttpResponse::Found()
                .append_header(("Location", "/login"))
                .finish();
        }
    };
    
    // Obtener información del archivo
    let file_id = file_id.into_inner();
    let file = match get_file_by_id(&state.db_pool, file_id).await {
        Ok(Some(file)) if file.user_id == user.id => file,
        Ok(Some(_)) => {
            // Archivo pertenece a otro usuario
            return HttpResponse::Found()
                .append_header(("Location", "/files?error=permission"))
                .finish();
        }
        Ok(None) => {
            // Archivo no encontrado
            return HttpResponse::Found()
                .append_header(("Location", "/files?error=notfound"))
                .finish();
        }
        Err(e) => {
            error!("Error al obtener archivo: {}", e);
            return HttpResponse::Found()
                .append_header(("Location", "/files?error=database"))
                .finish();
        }
    };
    
    if file.is_directory {
        // No se pueden descargar directorios
        return HttpResponse::Found()
            .append_header(("Location", format!("/files?path={}", file.path)))
            .finish();
    }
    
    // Ruta física del archivo
    let file_path = format!("{}/files/{}", state.storage_path, file_id);
    
    // Verificar que el archivo existe
    if !Path::new(&file_path).exists() {
        error!("Archivo físico no encontrado: {}", file_path);
        return HttpResponse::Found()
            .append_header(("Location", "/files?error=filenotfound"))
            .finish();
    }
    
    // Leer el archivo
    let file_bytes = match tokio::fs::read(&file_path).await {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Error al leer archivo: {}", e);
            return HttpResponse::Found()
                .append_header(("Location", "/files?error=fileread"))
                .finish();
        }
    };
    
    // Construir respuesta con el archivo
    HttpResponse::Ok()
        .content_type(file.mime_type)
        .append_header((
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", file.filename),
        ))
        .body(file_bytes)
}

/// Eliminar un archivo
async fn delete_file(
    state: web::Data<AppState>,
    req: HttpRequest,
    file_id: web::Path<Uuid>,
) -> impl Responder {
    // Obtener usuario de la sesión
    let user = match extract_user_from_session(&state.db_pool, &req).await {
        Ok(user) => user,
        Err(_) => {
            // Redirigir a login si no está autenticado
            return HttpResponse::Found()
                .append_header(("Location", "/login"))
                .finish();
        }
    };
    
    // Obtener información del archivo
    let file_id = file_id.into_inner();
    let file = match get_file_by_id(&state.db_pool, file_id).await {
        Ok(Some(file)) if file.user_id == user.id => file,
        Ok(Some(_)) => {
            // Archivo pertenece a otro usuario
            return HttpResponse::Found()
                .append_header(("Location", "/files?error=permission"))
                .finish();
        }
        Ok(None) => {
            // Archivo no encontrado
            return HttpResponse::Found()
                .append_header(("Location", "/files?error=notfound"))
                .finish();
        }
        Err(e) => {
            error!("Error al obtener archivo: {}", e);
            return HttpResponse::Found()
                .append_header(("Location", "/files?error=database"))
                .finish();
        }
    };
    
    // Eliminar el archivo
    match crate::core::files::delete_file(&state.db_pool, file_id, user.id).await {
        Ok(_) => {
            // Redirigir de vuelta a la carpeta
            HttpResponse::Found()
                .append_header(("Location", format!("/files?path={}&success=deleted", file.path)))
                .finish()
        }
        Err(e) => {
            error!("Error al eliminar archivo: {}", e);
            HttpResponse::Found()
                .append_header(("Location", format!("/files?path={}&error=deletefailed", file.path)))
                .finish()
        }
    }
}

/// Crear un directorio nuevo
async fn create_directory(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Form<CreateDirForm>,
) -> impl Responder {
    // Obtener usuario de la sesión
    let user = match extract_user_from_session(&state.db_pool, &req).await {
        Ok(user) => user,
        Err(_) => {
            // Redirigir a login si no está autenticado
            return HttpResponse::Found()
                .append_header(("Location", "/login"))
                .finish();
        }
    };
    
    // Validar nombre de directorio
    if form.name.trim().is_empty() || form.name.contains('/') {
        return HttpResponse::Found()
            .append_header(("Location", format!("/files?path={}&error=invalidname", form.path)))
            .finish();
    }
    
    // Crear directorio
    match crate::core::files::create_directory(&state.db_pool, user.id, &form.path, &form.name).await {
        Ok(_) => {
            // Redirigir de vuelta a la carpeta
            HttpResponse::Found()
                .append_header(("Location", format!("/files?path={}&success=dircreated", form.path)))
                .finish()
        }
        Err(e) => {
            error!("Error al crear directorio: {}", e);
            HttpResponse::Found()
                .append_header(("Location", format!("/files?path={}&error=mkdirfailed", form.path)))
                .finish()
        }
    }
}