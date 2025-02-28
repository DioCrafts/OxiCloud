use actix_web::{web, HttpResponse, Responder};
use askama::Template;
use serde::{Deserialize, Serialize};

// Estructura para los datos del template
#[derive(Template)]
#[template(path = "admin.html")]
struct AdminTemplate {
    upload_changable: bool,
    upload_max_filesize: String,
    display_max_possible_upload_size: bool,
    max_possible_upload_size: String,
    allow_zip_download: bool,
    max_zip_input_size: String,
    requesttoken: String,
}

// Datos para el formulario
#[derive(Serialize, Deserialize)]
struct FilesAdminSettings {
    max_upload_size: Option<String>,
    allow_zip_download: Option<bool>,
    max_zip_input_size: String,
    requesttoken: String,
}

// Función que maneja la ruta para mostrar la página de admin
pub async fn admin_page(
    app_config: web::Data<crate::AppConfig>,
    identity: web::ReqData<crate::auth::Identity>,
) -> impl Responder {
    // Verificar que el usuario tiene permisos de administrador
    if !identity.is_admin() {
        return HttpResponse::Forbidden().finish();
    }

    // Obtener los ajustes actuales
    let settings = match get_files_settings(&app_config).await {
        Ok(settings) => settings,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let template = AdminTemplate {
        upload_changable: settings.upload_changable,
        upload_max_filesize: settings.upload_max_filesize,
        display_max_possible_upload_size: settings.display_max_possible_upload_size,
        max_possible_upload_size: settings.max_possible_upload_size,
        allow_zip_download: settings.allow_zip_download,
        max_zip_input_size: settings.max_zip_input_size,
        requesttoken: generate_request_token(&identity),
    };

    match template.render() {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Función que maneja el envío del formulario
pub async fn admin_save(
    app_config: web::Data<crate::AppConfig>,
    identity: web::ReqData<crate::auth::Identity>,
    form: web::Form<FilesAdminSettings>,
) -> impl Responder {
    // Verificar que el usuario tiene permisos de administrador
    if !identity.is_admin() {
        return HttpResponse::Forbidden().finish();
    }

    // Verificar el token CSRF
    if !verify_request_token(&identity, &form.requesttoken) {
        return HttpResponse::Forbidden().finish();
    }

    // Guardar los ajustes
    match save_files_settings(&app_config, &form).await {
        Ok(_) => HttpResponse::Found().header("Location", "/settings/admin").finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Funciones auxiliares
async fn get_files_settings(
    config: &crate::AppConfig,
) -> Result<AdminTemplateData, Box<dyn std::error::Error>> {
    // Implementación real obtendría estos valores de la base de datos o configuración
    Ok(AdminTemplateData {
        upload_changable: true,
        upload_max_filesize: "512MB".to_string(),
        display_max_possible_upload_size: true,
        max_possible_upload_size: "2GB".to_string(),
        allow_zip_download: true,
        max_zip_input_size: "0".to_string(),
    })
}

async fn save_files_settings(
    config: &crate::AppConfig,
    form: &FilesAdminSettings,
) -> Result<(), Box<dyn std::error::Error>> {
    // Implementación real guardaría estos valores en la base de datos o configuración
    Ok(())
}

fn generate_request_token(identity: &crate::auth::Identity) -> String {
    // Implementación real generaría un token CSRF
    "csrf_token_example".to_string()
}

fn verify_request_token(identity: &crate::auth::Identity, token: &str) -> bool {
    // Implementación real verificaría el token CSRF
    token == "csrf_token_example"
}

// Estructura para almacenar datos necesarios para la plantilla
struct AdminTemplateData {
    upload_changable: bool,
    upload_max_filesize: String,
    display_max_possible_upload_size: bool,
    max_possible_upload_size: String,
    allow_zip_download: bool,
    max_zip_input_size: String,
}

// Registrar rutas en la aplicación
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/settings/admin/files")
            .route(web::get().to(admin_page))
            .route(web::post().to(admin_save)),
    );
}