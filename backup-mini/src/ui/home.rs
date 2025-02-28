//! UI para la página de inicio
use actix_web::{web, HttpResponse, Responder, HttpRequest};
// Eliminamos la importación no utilizada: use tera::Context;

use crate::{
    core::{
        files::calculate_user_storage,
        users::get_user_permissions,
    },
    utils::auth::extract_user_from_session,
    AppState,
};

use super::{render_template, PageContext};

/// Configuración de rutas para la UI de inicio
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").to(home_page));
}

/// Datos para la página de inicio
#[derive(serde::Serialize)]
struct HomePageData {
    used_space: i64,
    quota: Option<i64>,
    usage_percent: f64,
    file_count: i32,
}

/// Página de inicio
async fn home_page(
    state: web::Data<AppState>,
    req: HttpRequest,
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
    
    // Calcular estadísticas de almacenamiento
    let used_space = match calculate_user_storage(&state.db_pool, user.id).await {
        Ok(size) => size,
        Err(_) => 0,
    };
    
    // Obtener cuota de usuario
    let permissions = match get_user_permissions(&state.db_pool, user.id).await {
        Ok(perms) => perms,
        Err(_) => {
            // Si no se puede obtener permisos, redirigir a files
            return HttpResponse::Found()
                .append_header(("Location", "/files"))
                .finish();
        }
    };
    
    // Contar archivos
    let file_count = sqlx::query!(
        r#"
        SELECT COUNT(*) as count FROM files
        WHERE user_id = $1
        "#,
        user.id
    )
    .fetch_one(&state.db_pool)
    .await
    .map(|row| row.count.unwrap_or(0) as i32)
    .unwrap_or(0);
    
    // Calcular porcentaje de uso
    let usage_percent = if let Some(quota) = permissions.storage_quota {
        if quota > 0 {
            (used_space as f64 / quota as f64) * 100.0
        } else {
            0.0
        }
    } else {
        0.0 // Sin cuota = 0%
    };
    
    // Crear contexto para la plantilla
    let ctx = PageContext::new("Inicio", HomePageData {
        used_space,
        quota: permissions.storage_quota,
        usage_percent,
        file_count,
    })
    .with_user(Some(user))
    .into_context();
    
    // Manejar el Result que devuelve render_template
    match render_template("home/index.html", &ctx, &state) {
        Ok(response) => response,
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Para el error en la línea 283 (si está en otra función)
// Reemplaza
// render_template("files/details.html", &ctx, &state).expect("REASON")
// por
// match render_template("files/details.html", &ctx, &state) {
//     Ok(response) => response,
//     Err(_) => HttpResponse::InternalServerError().finish(),
// }