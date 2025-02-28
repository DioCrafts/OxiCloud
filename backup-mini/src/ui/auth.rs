//! UI para autenticación de usuarios

use actix_web::{web, HttpResponse, HttpRequest, cookie::Cookie};
use serde::Deserialize;
use log::error;
use time::Duration as TimeDuration;

use crate::{
    core::users::{authenticate_user, create_user, CreateUserData},
    utils::auth::{generate_token, extract_user_from_session},
    AppState,
};

use super::{render_template, PageContext, FlashMessage};

/// Configuración de rutas para la UI de autenticación
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/login")
            .route(web::get().to(login_page))
            .route(web::post().to(login_handler)),
    )
    .service(
        web::resource("/register")
            .route(web::get().to(register_page))
            .route(web::post().to(register_handler)),
    )
    .service(web::resource("/logout").to(logout_handler));
}

/// Datos para inicio de sesión desde formulario
#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
    pub remember_me: Option<String>,
}

/// Datos para registro desde formulario
#[derive(Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_confirm: String,
    pub display_name: Option<String>,
}

/// Página de inicio de sesión
async fn login_page(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    // Verificar si el usuario ya está autenticado
    if extract_user_from_session(&state.db_pool, &req).await.is_ok() {
        return Ok(
            HttpResponse::Found()
                .append_header(("Location", "/"))
                .finish()
        );
    }

    // Crear contexto para la plantilla
    let ctx = PageContext::new("Iniciar sesión", ()).into_context();

    render_template("auth/login.html", &ctx, &state)
}

/// Manejador para el formulario de inicio de sesión
async fn login_handler(
    state: web::Data<AppState>,
    form: web::Form<LoginForm>,
) -> Result<HttpResponse, actix_web::Error> {
    // Intentar autenticar al usuario
    let user = match authenticate_user(&state.db_pool, &form.username, &form.password).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            // Credenciales inválidas
            let ctx = PageContext::new("Iniciar sesión", ())
                .with_flash(FlashMessage::error("Nombre de usuario o contraseña incorrectos"))
                .into_context();
            return render_template("auth/login.html", &ctx, &state);
        }
        Err(e) => {
            error!("Error de autenticación: {}", e);
            let ctx = PageContext::new("Iniciar sesión", ())
                .with_flash(FlashMessage::error("Error al procesar la solicitud"))
                .into_context();
            return render_template("auth/login.html", &ctx, &state);
        }
    };

    // Generar token JWT
    let token = match generate_token(user.id) {
        Ok(token) => token,
        Err(e) => {
            error!("Error al generar token: {}", e);
            let ctx = PageContext::new("Iniciar sesión", ())
                .with_flash(FlashMessage::error("Error de autenticación"))
                .into_context();
            return render_template("auth/login.html", &ctx, &state);
        }
    };

    // Crear cookie de sesión
    let cookie = Cookie::build("session", token)
        .path("/")
        .http_only(true)
        // Si remember_me está activo, establecer una duración más larga
        .max_age(if form.remember_me.is_some() {
            TimeDuration::days(30)
        } else {
            TimeDuration::hours(24)
        })
        .finish();

    Ok(
        HttpResponse::Found()
            .cookie(cookie)
            .append_header(("Location", "/"))
            .finish()
    )
}

/// Página de registro
async fn register_page(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    // Verificar si el usuario ya está autenticado
    if extract_user_from_session(&state.db_pool, &req).await.is_ok() {
        return Ok(
            HttpResponse::Found()
                .append_header(("Location", "/"))
                .finish()
        );
    }

    // Crear contexto para la plantilla
    let ctx = PageContext::new("Registrarse", ()).into_context();

    render_template("auth/register.html", &ctx, &state)
}

/// Manejador para el formulario de registro
async fn register_handler(
    state: web::Data<AppState>,
    form: web::Form<RegisterForm>,
) -> Result<HttpResponse, actix_web::Error> {
    // Validar formulario
    let mut errors = Vec::new();

    if form.username.trim().is_empty() {
        errors.push(FlashMessage::error("El nombre de usuario es obligatorio"));
    }

    if form.email.trim().is_empty() {
        errors.push(FlashMessage::error("El correo electrónico es obligatorio"));
    }

    if form.password.len() < 6 {
        errors.push(FlashMessage::error("La contraseña debe tener al menos 6 caracteres"));
    }

    if form.password != form.password_confirm {
        errors.push(FlashMessage::error("Las contraseñas no coinciden"));
    }

    if !errors.is_empty() {
        let ctx = PageContext::new("Registrarse", ())
            .with_flashes(errors)
            .into_context();
        return render_template("auth/register.html", &ctx, &state);
    }

    // Crear usuario
    let create_data = CreateUserData {
        username: form.username.clone(),
        email: form.email.clone(),
        password: form.password.clone(),
        display_name: form.display_name.clone(),
        is_admin: false, // Los usuarios registrados a través de la UI nunca son admin
    };

    let user = match create_user(&state.db_pool, create_data).await {
        Ok(user) => user,
        Err(e) => {
            error!("Error al crear usuario: {}", e);
            let msg = if e.to_string().contains("ya están en uso") {
                "El nombre de usuario o correo electrónico ya están en uso"
            } else {
                "Error al crear la cuenta"
            };
            let ctx = PageContext::new("Registrarse", ())
                .with_flash(FlashMessage::error(msg))
                .into_context();
            return render_template("auth/register.html", &ctx, &state);
        }
    };

    // Generar token JWT
    let token = match generate_token(user.id) {
        Ok(token) => token,
        Err(e) => {
            error!("Error al generar token: {}", e);
            let ctx = PageContext::new("Registrarse", ())
                .with_flash(FlashMessage::error("Error de autenticación"))
                .into_context();
            return render_template("auth/register.html", &ctx, &state);
        }
    };

    // Crear cookie de sesión
    let cookie = Cookie::build("session", token)
        .path("/")
        .http_only(true)
        .max_age(TimeDuration::hours(24))
        .finish();

    Ok(
        HttpResponse::Found()
            .cookie(cookie)
            .append_header(("Location", "/"))
            .finish()
    )
}

/// Manejador para cerrar sesión
async fn logout_handler() -> Result<HttpResponse, actix_web::Error> {
    // Eliminar cookie de sesión
    let cookie = Cookie::build("session", "")
        .path("/")
        .http_only(true)
        .max_age(TimeDuration::seconds(0))
        .finish();

    Ok(
        HttpResponse::Found()
            .cookie(cookie)
            .append_header(("Location", "/login"))
            .finish()
    )
}
