use actix_web::{web, HttpResponse, Responder};
use askama::Template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "settings.html")]
struct SettingsTemplate {
    dependencies: String,
    is_admin_page: bool,
    mounts: HashMap<String, Mount>,
    backends: HashMap<String, Backend>,
    groups: Vec<String>,
    users: Vec<String>,
    user_display_names: HashMap<String, String>,
    allow_user_mounting: bool,
    certs: Vec<String>,
    request_token: String,
    l: Translator,
}

#[derive(Serialize, Deserialize, Clone)]
struct Mount {
    class: String,
    backend: String,
    status: Option<bool>,
    configuration: Option<HashMap<String, String>>,
    applicable: Option<Applicable>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Applicable {
    groups: Option<Vec<String>>,
    users: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Backend {
    backend: String,
    configuration: HashMap<String, String>,
    custom: Option<String>,
}

struct Translator {
    // En una implementación real, aquí iría un estado para manejar traducciones
}

impl Translator {
    fn t(&self, text: &str) -> String {
        // En una implementación real, aquí iría la lógica de traducción
        text.to_string()
    }
}

async fn render_settings(
    data: web::Data<AppState>,
    session: web::ReqData<Session>,
) -> impl Responder {
    let user_id = session.get_user_id().unwrap_or_default();
    let is_admin = data.user_service.is_admin(&user_id).await;

    // Recuperar datos para la plantilla
    let dependencies = if is_admin {
        data.external_storage_service.get_dependencies().await.unwrap_or_default()
    } else {
        String::new()
    };

    let mounts = data.external_storage_service.get_mounts(&user_id, is_admin).await
        .unwrap_or_default();
    
    let backends = data.external_storage_service.get_backends().await
        .unwrap_or_default();
    
    let groups = if is_admin {
        data.group_service.get_all_groups().await.unwrap_or_default()
    } else {
        Vec::new()
    };
    
    let users = if is_admin {
        data.user_service.get_all_users().await.unwrap_or_default()
    } else {
        Vec::new()
    };
    
    let user_display_names = if is_admin {
        data.user_service.get_display_names(&users).await.unwrap_or_default()
    } else {
        HashMap::new()
    };
    
    let allow_user_mounting = if is_admin {
        data.config_service.get_allow_user_mounting().await.unwrap_or(false)
    } else {
        false
    };
    
    let certs = if !is_admin {
        data.certificate_service.get_root_certs(&user_id).await.unwrap_or_default()
    } else {
        Vec::new()
    };
    
    let request_token = data.security_service.generate_request_token(&user_id).await
        .unwrap_or_default();

    let translator = Translator {}; // Inicialización simplificada
    
    let template = SettingsTemplate {
        dependencies,
        is_admin_page: is_admin,
        mounts,
        backends,
        groups,
        users,
        user_display_names,
        allow_user_mounting,
        certs,
        request_token,
        l: translator,
    };
    
    match template.render() {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(err) => {
            log::error!("Error rendering template: {}", err);
            HttpResponse::InternalServerError().body("Error rendering template")
        }
    }
}

// Estructura para contener servicios
struct AppState {
    external_storage_service: ExternalStorageService,
    user_service: UserService,
    group_service: GroupService,
    config_service: ConfigService,
    certificate_service: CertificateService,
    security_service: SecurityService,
}

// Implementaciones de servicios (simuladas para este ejemplo)
struct ExternalStorageService;
struct UserService;
struct GroupService;
struct ConfigService;
struct CertificateService;
struct SecurityService;
struct Session;

impl ExternalStorageService {
    async fn get_dependencies(&self) -> Result<String, anyhow::Error> {
        Ok(String::new())
    }
    
    async fn get_mounts(&self, _user_id: &str, _is_admin: bool) -> Result<HashMap<String, Mount>, anyhow::Error> {
        Ok(HashMap::new())
    }
    
    async fn get_backends(&self) -> Result<HashMap<String, Backend>, anyhow::Error> {
        Ok(HashMap::new())
    }
}

impl UserService {
    async fn is_admin(&self, _user_id: &str) -> bool {
        false
    }
    
    async fn get_all_users(&self) -> Result<Vec<String>, anyhow::Error> {
        Ok(Vec::new())
    }
    
    async fn get_display_names(&self, _users: &[String]) -> Result<HashMap<String, String>, anyhow::Error> {
        Ok(HashMap::new())
    }
}

impl GroupService {
    async fn get_all_groups(&self) -> Result<Vec<String>, anyhow::Error> {
        Ok(Vec::new())
    }
}

impl ConfigService {
    async fn get_allow_user_mounting(&self) -> Result<bool, anyhow::Error> {
        Ok(false)
    }
}

impl CertificateService {
    async fn get_root_certs(&self, _user_id: &str) -> Result<Vec<String>, anyhow::Error> {
        Ok(Vec::new())
    }
}

impl SecurityService {
    async fn generate_request_token(&self, _user_id: &str) -> Result<String, anyhow::Error> {
        Ok(String::new())
    }
}

impl Session {
    fn get_user_id(&self) -> Option<String> {
        None
    }
}

// Configuración de rutas
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/settings")
            .route(web::get().to(render_settings))
    );
}