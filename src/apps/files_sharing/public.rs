use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use actix_files::NamedFile;
use actix_session::{Session, CookieSession};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::path::Path;
use chrono::{DateTime, Utc};
use handlebars::Handlebars;
use futures::future::{self, Future};
use bcrypt::{hash, verify};
use mime_guess::from_path;
use std::collections::HashMap;
use serde_json::json;
use log::{error, debug};
use humansize::{FileSize, file_size_opts};
use std::sync::Arc;

// Estructuras principales
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileInfo {
    name: String,
    path: String,
    directory: String,
    #[serde(rename = "type")]
    file_type: String,
    size: u64,
    mtime: i64,
    date: String,
    basename: Option<String>,
    extension: Option<String>,
    is_preview_available: bool,
    permissions: u8,
    icon: String,
    etag: String,
    mimetype: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ShareItem {
    id: String,
    uid_owner: String,
    file_owner: String,
    item_type: String,
    file_source: String,
    file_target: String,
    share_type: i32,
    share_with: Option<String>,
    token: String,
    permissions: u8,
}

struct AppState {
    db: Arc<Database>,
    filesystem: Arc<FileSystem>,
    config: Arc<Config>,
    templates: Arc<Handlebars<'static>>,
    preview_manager: Arc<PreviewManager>,
}

// Servicio de base de datos
struct Database {
    // Campo que representaría una conexión a base de datos
}

impl Database {
    fn get_share_by_token(&self, token: &str) -> Option<ShareItem> {
        // Simulación de la consulta a la base de datos
        todo!("Implementar la consulta real a la base de datos")
    }
    
    fn resolve_reshare(&self, link_item: &ShareItem) -> Option<ShareItem> {
        // Simulación para resolver un reshare
        todo!("Implementar la resolución real de reshares")
    }
}

// Servicio de sistema de archivos
struct FileSystem {
    // Campos para el sistema de archivos
}

impl FileSystem {
    fn tear_down_fs(&self) {
        // Limpiar el sistema de archivos actual
    }
    
    fn setup_fs(&self, user: &str) {
        // Configurar el sistema de archivos para un usuario
    }
    
    fn get_path(&self, file_source: &str) -> Option<String> {
        // Obtener la ruta de un archivo por su ID
        todo!("Implementar la obtención real de la ruta")
    }
    
    fn is_readable(&self, path: &str) -> bool {
        // Comprobar si una ruta es legible
        todo!("Implementar la comprobación real de permisos")
    }
    
    fn normalize_path(&self, path: &str) -> String {
        // Normalizar una ruta
        todo!("Implementar la normalización real de rutas")
    }
    
    fn is_dir(&self, path: &str) -> bool {
        // Comprobar si una ruta es un directorio
        todo!("Implementar la comprobación real de directorio")
    }
    
    fn get_directory_content(&self, path: &str) -> Vec<FileInfo> {
        // Obtener el contenido de un directorio
        todo!("Implementar la obtención real del contenido del directorio")
    }
    
    fn get_mime_type(&self, path: &str) -> String {
        // Obtener el tipo MIME de un archivo
        todo!("Implementar la obtención real del tipo MIME")
    }
}

// Configuración
struct Config {
    // Campos de configuración
}

impl Config {
    fn get_app_config(&self, app: &str, key: &str, default: &str) -> String {
        // Obtener configuración de una aplicación
        todo!("Implementar la obtención real de configuración de aplicación")
    }
    
    fn get_system_value<T>(&self, key: &str, default: T) -> T {
        // Obtener valor de configuración del sistema
        todo!("Implementar la obtención real de valores del sistema")
    }
    
    fn get_password_salt(&self) -> String {
        // Obtener la sal para contraseñas
        todo!("Implementar la obtención real de la sal para contraseñas")
    }
}

// Gestor de vistas previas
struct PreviewManager {
    // Campos del gestor de vistas previas
}

impl PreviewManager {
    fn is_mime_supported(&self, mimetype: &str) -> bool {
        // Comprobar si un tipo MIME es compatible con vistas previas
        todo!("Implementar la comprobación real de soporte de tipos MIME")
    }
    
    fn public_preview_icon(&self, relative_path: &str, sharing_token: &str) -> String {
        // Generar un icono de vista previa pública
        todo!("Implementar la generación real de iconos de vista previa")
    }
    
    fn mimetype_icon(&self, mimetype: &str) -> String {
        // Generar un icono para un tipo MIME
        todo!("Implementar la generación real de iconos de tipo MIME")
    }
}

// Funciones auxiliares
fn file_cmp(a: &FileInfo, b: &FileInfo) -> Ordering {
    if a.file_type == "dir" && b.file_type != "dir" {
        Ordering::Less
    } else if a.file_type != "dir" && b.file_type == "dir" {
        Ordering::Greater
    } else {
        a.name.to_lowercase().cmp(&b.name.to_lowercase())
    }
}

fn determine_icon(file: &FileInfo, sharing_root: &str, sharing_token: &str, preview_manager: &PreviewManager) -> String {
    // Para carpetas simplemente reutilizamos la lógica de archivos
    if file.file_type == "dir" {
        // Implementar el equivalente a \OCA\Files\Helper::determineIcon
        return format!("folder-icon");
    }

    let relative_path = file.path.strip_prefix("/files").unwrap_or(&file.path);
    let relative_path = relative_path.strip_prefix(sharing_root).unwrap_or(relative_path);
    
    if file.is_preview_available {
        return format!("{}&c={}", preview_manager.public_preview_icon(relative_path, sharing_token), file.etag);
    }
    
    preview_manager.mimetype_icon(&file.mimetype)
}

fn max_upload_filesize(path: &str, config: &Config) -> u64 {
    // Implementar el equivalente a OCP\Util::maxUploadFilesize
    todo!("Implementar el cálculo real del tamaño máximo de carga")
}

fn human_filesize(size: u64) -> String {
    // Implementar el equivalente a OCP\Util::humanFileSize
    size.file_size(file_size_opts::CONVENTIONAL).unwrap_or_else(|_| "0 B".to_string())
}

fn format_date(timestamp: i64) -> String {
    // Implementar el equivalente a OCP\Util::formatDate
    let dt = DateTime::<Utc>::from_timestamp(timestamp, 0).unwrap();
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

// Rutas HTTP
#[get("/public/files")]
async fn public_files(
    req: web::HttpRequest, 
    query: web::Query<HashMap<String, String>>,
    post_data: Option<web::Form<HashMap<String, String>>>,
    session: Session,
    state: web::Data<AppState>
) -> impl Responder {
    // Cargar otras aplicaciones para vistas previas de archivos
    // OC_App::loadApps() equivalente

    // Comprobar si se permiten enlaces compartidos
    if state.config.get_app_config("core", "shareapi_allow_links", "yes") != "yes" {
        return HttpResponse::NotFound()
            .content_type("text/html")
            .body(render_404_page(&state.templates));
    }

    // Procesar el token compartido
    let token = match query.get("t") {
        Some(t) => t,
        None => {
            debug!("No token provided");
            return HttpResponse::NotFound()
                .content_type("text/html")
                .body(render_404_page(&state.templates));
        }
    };

    // Obtener el elemento compartido por token
    let link_item = match state.db.get_share_by_token(token) {
        Some(item) => item,
        None => {
            debug!("could not resolve linkItem");
            return HttpResponse::NotFound()
                .content_type("text/html")
                .body(render_error_page(&state.templates));
        }
    };

    // Comprobar si el enlace es válido
    if link_item.uid_owner.is_empty() {
        debug!("Invalid share: no owner");
        return HttpResponse::NotFound()
            .content_type("text/html")
            .body(render_404_page(&state.templates));
    }

    // Obtener tipo y fuente del archivo
    let file_type = &link_item.item_type;
    let file_source = &link_item.file_source;
    let share_owner = &link_item.uid_owner;

    // Resolver reshare
    let root_link_item = state.db.resolve_reshare(&link_item).unwrap_or_else(|| link_item.clone());
    let file_owner = &root_link_item.file_owner;

    // Configurar sistema de archivos
    let mut path = None;
    if !file_owner.is_empty() {
        state.filesystem.tear_down_fs();
        state.filesystem.setup_fs(file_owner);
        path = state.filesystem.get_path(&file_source);
    }

    let path = match path {
        Some(p) => p,
        None => {
            debug!("could not resolve path");
            return HttpResponse::NotFound()
                .content_type("text/html")
                .body(render_error_page(&state.templates));
        }
    };

    // Comprobar tipo de elemento
    if link_item.item_type.is_empty() {
        error!("No item type set for share id: {}", link_item.id);
        return HttpResponse::NotFound()
            .content_type("text/html")
            .body(render_404_page(&state.templates));
    }

    // Autenticación con contraseña si es necesario
    if let Some(share_with) = &link_item.share_with {
        let url = format!("/public/files?t={}", token);
        let url = if let Some(file) = query.get("file") {
            format!("{}&file={}", url, file)
        } else if let Some(dir) = query.get("dir") {
            format!("{}&dir={}", url, dir)
        } else {
            url
        };

        if let Some(form_data) = &post_data {
            if let Some(password) = form_data.get("password") {
                if link_item.share_type == 3 { // OCP\Share::SHARE_TYPE_LINK
                    // Verificar contraseña
                    let password_salt = state.config.get_password_salt();
                    let verified = verify(format!("{}{}", password, password_salt), share_with).unwrap_or(false);
                    
                    if !verified {
                        // Contraseña incorrecta
                        return HttpResponse::Ok()
                            .content_type("text/html")
                            .body(render_authenticate_page(&state.templates, &url, true));
                    } else {
                        // Guardar ID en sesión para futuras solicitudes
                        session.insert("public_link_authenticated", &link_item.id).unwrap();
                    }
                } else {
                    error!("Unknown share type {} for share id {}", link_item.share_type, link_item.id);
                    return HttpResponse::NotFound()
                        .content_type("text/html")
                        .body(render_404_page(&state.templates));
                }
            }
        } else {
            // Comprobar si el ID está en la sesión
            match session.get::<String>("public_link_authenticated") {
                Ok(Some(authenticated_id)) if authenticated_id == link_item.id => {
                    // Autenticado, continuar
                },
                _ => {
                    // Solicitar contraseña
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(render_authenticate_page(&state.templates, &url, false));
                }
            }
        }
    }

    // Procesar la ruta solicitada
    let base_path = path.clone();
    let get_path = if let Some(req_path) = query.get("path") {
        if state.filesystem.is_readable(&format!("{}{}", base_path, req_path)) {
            state.filesystem.normalize_path(req_path)
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    
    let path = format!("{}{}", path, get_path);
    let dir = Path::new(&path).parent().unwrap_or_else(|| Path::new("/")).to_string_lossy().to_string();
    let file = Path::new(&path).file_name().unwrap_or_default().to_string_lossy().to_string();

    // Descargar archivo(s)
    if query.get("download").is_some() {
        if let Some(files) = query.get("files") {
            // Descargar archivos seleccionados
            let files_list: Vec<String> = serde_json::from_str(files).unwrap_or_else(|_| vec![files.clone()]);
            
            // Implementar equivalente a OC_Files::get()
            return HttpResponse::Ok()
                .content_type("application/octet-stream")
                .body("file_content"); // Contenido real del archivo
        } else {
            // Descargar un solo archivo
            // Implementar equivalente a OC_Files::get()
            return HttpResponse::Ok()
                .content_type("application/octet-stream")
                .body("file_content"); // Contenido real del archivo
        }
    }

    // Renderizar vista de archivos compartidos
    // Añadir scripts y estilos necesarios
    let max_upload_filesize = max_upload_filesize(&path, &state.config);
    
    let mut context = HashMap::new();
    context.insert("uidOwner", share_owner.clone());
    context.insert("displayName", format!("User Display Name")); // Obtener el nombre real del usuario
    context.insert("filename", file);
    context.insert("directory_path", link_item.file_target.clone());
    context.insert("mimetype", state.filesystem.get_mime_type(&path));
    context.insert("fileTarget", Path::new(&link_item.file_target).file_name().unwrap_or_default().to_string_lossy().to_string());
    context.insert("dirToken", link_item.token.clone());
    context.insert("disableSharing", "true".to_string());
    
    // Comprobar si se permite la carga pública
    let mut allow_public_upload_enabled = (link_item.permissions & 4) > 0; // OCP\PERMISSION_CREATE = 4
    
    // Deshabilitar carga si el cifrado está activado
    if state.config.get_app_config("files_encryption", "enabled", "no") == "yes" {
        allow_public_upload_enabled = false;
    }
    
    // Deshabilitar carga si está desactivada globalmente
    if state.config.get_app_config("core", "shareapi_allow_public_upload", "yes") == "no" {
        allow_public_upload_enabled = false;
    }
    
    // Deshabilitar carga si no es una carpeta
    if link_item.item_type != "folder" {
        allow_public_upload_enabled = false;
    }
    
    context.insert("allowPublicUploadEnabled", allow_public_upload_enabled.to_string());
    context.insert("uploadMaxFilesize", max_upload_filesize.to_string());
    context.insert("uploadMaxHumanFilesize", human_filesize(max_upload_filesize));
    
    // Construir URL con identificadores
    let url_link_identifiers = format!(
        "{}{}{}", 
        if !token.is_empty() { format!("&t={}", token) } else { String::new() },
        if let Some(dir_param) = query.get("dir") { format!("&dir={}", dir_param) } else { String::new() },
        if let Some(file_param) = query.get("file") { format!("&file={}", file_param) } else { String::new() }
    );
    
    // Mostrar lista de archivos o vista de archivo individual
    if state.filesystem.is_dir(&path) {
        // Vista de directorio
        context.insert("dir", get_path);
        
        // Obtener lista de archivos
        let mut files = state.filesystem.get_directory_content(&path);
        let root_length = base_path.len() + 1;
        let mut total_size = 0;
        
        for file_info in &mut files {
            total_size += file_info.size;
            file_info.date = format_date(file_info.mtime);
            
            if file_info.file_type == "file" {
                let path = Path::new(&file_info.name);
                file_info.basename = path.file_stem().map(|s| s.to_string_lossy().to_string());
                
                if let Some(ext) = path.extension() {
                    file_info.extension = Some(format!(".{}", ext.to_string_lossy()));
                } else {
                    file_info.extension = Some(String::new());
                }
                
                file_info.is_preview_available = state.preview_manager.is_mime_supported(&file_info.mimetype);
            }
            
            file_info.directory = get_path.clone();
            file_info.permissions = 1; // OCP\PERMISSION_READ = 1
            file_info.icon = determine_icon(file_info, &base_path, token, &state.preview_manager);
        }
        
        files.sort_by(file_cmp);
        
        // Crear migas de pan
        let mut breadcrumb = Vec::new();
        let mut path_to_here = String::new();
        
        for segment in get_path.split('/') {
            if !segment.is_empty() {
                path_to_here = format!("{}/{}", path_to_here, segment);
                breadcrumb.push(json!({
                    "dir": path_to_here,
                    "name": segment
                }));
            }
        }
        
        // Comprobar si se permite la descarga en ZIP
        let max_input_file_size = state.config.get_system_value("maxZipInputSize", 838860800u64); // 800 MB
        let allow_zip = state.config.get_system_value("allowZipDownload", true) && 
                        (max_input_file_size == 0 || total_size <= max_input_file_size);
        
        context.insert("allowZipDownload", allow_zip.to_string());
        context.insert("downloadURL", format!(
            "/public/files{}{}{}",
            url_link_identifiers,
            "&download&path=",
            urlencoding::encode(&get_path)
        ));
        
        // Renderizar la plantilla de carpeta
        let folder_template = render_folder_template(
            &files,
            &breadcrumb,
            &get_path,
            max_upload_filesize,
            &format!("/public/files{}",&url_link_identifiers),
            &state.templates
        );
        
        context.insert("folder", folder_template);
    } else {
        // Vista de archivo individual
        context.insert("dir", dir);
        
        // URL de descarga
        if file_type == "file" {
            context.insert("downloadURL", format!("/public/files{}&download", url_link_identifiers));
        } else {
            context.insert("downloadURL", format!(
                "/public/files{}{}{}",
                url_link_identifiers,
                "&download&path=",
                urlencoding::encode(&get_path)
            ));
        }
    }
    
    // Renderizar la plantilla pública
    HttpResponse::Ok()
        .content_type("text/html")
        .body(render_public_template(&context, &state.templates))
}

// Funciones de renderización de plantillas
fn render_404_page(templates: &Handlebars) -> String {
    templates.render("404", &json!({})).unwrap_or_else(|_| "404 Not Found".to_string())
}

fn render_error_page(templates: &Handlebars) -> String {
    let error_template = templates.render("part.404", &json!({})).unwrap_or_else(|_| "Error".to_string());
    templates.render("404", &json!({ "content": error_template })).unwrap_or_else(|_| "404 Not Found".to_string())
}

fn render_authenticate_page(templates: &Handlebars, url: &str, wrong_pw: bool) -> String {
    templates.render("authenticate", &json!({
        "URL": url,
        "wrongpw": wrong_pw
    })).unwrap_or_else(|_| "Authentication Required".to_string())
}

fn render_public_template(context: &HashMap<String, String>, templates: &Handlebars) -> String {
    templates.render("public", &json!(context)).unwrap_or_else(|_| "Error rendering template".to_string())
}

fn render_folder_template(
    files: &[FileInfo],
    breadcrumb: &[serde_json::Value],
    dir: &str,
    max_upload_filesize: u64,
    base_url: &str,
    templates: &Handlebars
) -> String {
    // Renderizar lista de archivos
    let file_list_template = templates.render("part.list", &json!({
        "files": files,
        "baseURL": format!("{}&path=", base_url),
        "downloadURL": format!("{}&download&path=", base_url),
        "isPublic": true,
        "sharingtoken": "", // Obtener del contexto
        "sharingroot": ""   // Obtener del contexto
    })).unwrap_or_default();
    
    // Renderizar migas de pan
    let breadcrumb_template = templates.render("part.breadcrumb", &json!({
        "breadcrumb": breadcrumb,
        "baseURL": format!("{}&path=", base_url)
    })).unwrap_or_default();
    
    // Renderizar carpeta
    templates.render("index", &json!({
        "fileList": file_list_template,
        "breadcrumb": breadcrumb_template,
        "dir": dir,
        "isCreatable": false,
        "permissions": 1, // OCP\PERMISSION_READ = 1
        "isPublic": true,
        "publicUploadEnabled": "no",
        "files": files,
        "uploadMaxFilesize": max_upload_filesize,
        "uploadMaxHumanFilesize": human_filesize(max_upload_filesize),
        "allowZipDownload": true,
        "usedSpacePercent": 0
    })).unwrap_or_else(|_| "Error rendering folder".to_string())
}

// Punto de entrada principal
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inicializar estado de la aplicación
    let app_state = web::Data::new(AppState {
        db: Arc::new(Database {}),
        filesystem: Arc::new(FileSystem {}),
        config: Arc::new(Config {}),
        templates: Arc::new({
            let mut handlebars = Handlebars::new();
            // Registrar plantillas
            handlebars.register_template_string("404", "404 Not Found").unwrap();
            handlebars.register_template_string("part.404", "Error content").unwrap();
            handlebars.register_template_string("authenticate", "Authentication Required").unwrap();
            handlebars.register_template_string("public", "Public Share").unwrap();
            handlebars.register_template_string("part.list", "File List").unwrap();
            handlebars.register_template_string("part.breadcrumb", "Breadcrumb").unwrap();
            handlebars.register_template_string("index", "Folder View").unwrap();
            handlebars
        }),
        preview_manager: Arc::new(PreviewManager {}),
    });

    // Iniciar servidor HTTP
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(public_files)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}