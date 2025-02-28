use actix_web::{web, HttpResponse, Result};
use askama::Template;
use serde::Deserialize;

// Template para la página de contraseña perdida
#[derive(Template)]
#[template(path = "lostpassword.html")]
struct LostPasswordTemplate {
    requested: bool,
    error: bool,
    is_encrypted: bool,
    reset_url: String,
    user_svg_path: String,
    translations: Translations,
}

// Estructura para las traducciones
struct Translations {
    reset_link_sent: String,
    request_failed: String,
    receive_link_via_email: String,
    username: String,
    files_encrypted_warning: String,
    yes_reset_password: String,
    reset_button: String,
}

// Datos del formulario
#[derive(Deserialize)]
struct LostPasswordForm {
    user: String,
    continue_reset: Option<String>,
}

// Función que renderiza la plantilla
async fn render_lostpassword_page(
    query: web::Query<LostPasswordQuery>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    // Cargar estilos (equivalente a OCP\Util::addStyle)
    // Esto sería manejado por el sistema de assets de la aplicación

    let translations = Translations {
        reset_link_sent: String::from("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator ."),
        request_failed: String::from("Request failed!<br>Did you make sure your email/username was right?"),
        receive_link_via_email: String::from("You will receive a link to reset your password via Email."),
        username: String::from("Username"),
        files_encrypted_warning: String::from("Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?"),
        yes_reset_password: String::from("Yes, I really want to reset my password now"),
        reset_button: String::from("Reset"),
    };

    let template = LostPasswordTemplate {
        requested: query.requested.unwrap_or(false),
        error: query.error.unwrap_or(false),
        is_encrypted: app_config.is_encrypted,
        reset_url: app_config.routes.lostpassword_send_email.clone(),
        user_svg_path: get_image_path("actions/user.svg"),
        translations,
    };

    let body = template.render().map_err(|e| {
        // Manejar error de renderización
        actix_web::error::ErrorInternalServerError(format!("Template error: {}", e))
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

// Estructura para los parámetros de consulta
#[derive(Deserialize)]
struct LostPasswordQuery {
    requested: Option<bool>,
    error: Option<bool>,
}

// Configuración de la aplicación
struct AppConfig {
    is_encrypted: bool,
    routes: Routes,
}

struct Routes {
    lostpassword_send_email: String,
}

// Función auxiliar para obtener rutas de imágenes
fn get_image_path(path: &str) -> String {
    format!("/core/img/{}", path)
}

// Función que maneja el envío del formulario
async fn send_password_reset_email(
    form: web::Form<LostPasswordForm>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    // Implementación del envío de correo de restablecimiento
    // ...

    // Redireccionar a la página de confirmación
    Ok(HttpResponse::SeeOther()
        .header("Location", format!("/lostpassword?requested=true"))
        .finish())
}

// Configuración de rutas
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/lostpassword")
            .route(web::get().to(render_lostpassword_page))
            .route(web::post().to(send_password_reset_email)),
    );
}