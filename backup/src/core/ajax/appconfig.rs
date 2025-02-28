/*
 * Copyright (c) 2011, Robin Appelman <icewind1991@gmail.com>
 * This file is licensed under the Affero General Public License version 3 or later.
 * See the COPYING-README file.
 */

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

// Estructuras para deserializar los parámetros de la solicitud
#[derive(Deserialize)]
struct GetValueQuery {
    app: String,
    key: String,
    #[serde(rename = "defaultValue", default)]
    default_value: Option<String>,
}

#[derive(Deserialize)]
struct SetValueForm {
    app: String,
    key: String,
    value: String,
}

#[derive(Deserialize)]
struct AppQuery {
    app: String,
}

#[derive(Deserialize)]
struct AppKeyQuery {
    app: String,
    key: String,
}

#[derive(Deserialize)]
struct AppKeyForm {
    app: String,
    key: String,
}

#[derive(Deserialize)]
struct AppForm {
    app: String,
}

// Estructura para la respuesta JSON
#[derive(Serialize)]
struct JsonResponse<T> {
    status: String,
    data: T,
}

// Handler principal
pub async fn handle_appconfig(
    query: Option<web::Query<GetValueQuery>>,
    get_app_key_query: Option<web::Query<AppKeyQuery>>,
    app_query: Option<web::Query<AppQuery>>,
    action: web::Query<AppConfigAction>,
    form_value: Option<web::Form<SetValueForm>>,
    form_key: Option<web::Form<AppKeyForm>>,
    form_app: Option<web::Form<AppForm>>,
    app_config: web::Data<AppConfig>,
) -> impl Responder {
    // Verificar que el usuario es administrador
    if !app_config.auth_service.check_admin_user().await {
        return HttpResponse::Forbidden().finish();
    }

    // Verificar token CSRF
    if !app_config.auth_service.check_csrf_token().await {
        return HttpResponse::Forbidden().finish();
    }

    let result = match action.action.as_str() {
        "getValue" => {
            if let Some(query) = query {
                app_config
                    .get_value(&query.app, &query.key, query.default_value.as_deref())
                    .await
            } else {
                return HttpResponse::BadRequest().finish();
            }
        }
        "setValue" => {
            if let Some(form) = form_value {
                app_config
                    .set_value(&form.app, &form.key, &form.value)
                    .await
            } else {
                return HttpResponse::BadRequest().finish();
            }
        }
        "getApps" => app_config.get_apps().await,
        "getKeys" => {
            if let Some(query) = app_query {
                app_config.get_keys(&query.app).await
            } else {
                return HttpResponse::BadRequest().finish();
            }
        }
        "hasKey" => {
            if let Some(query) = get_app_key_query {
                app_config.has_key(&query.app, &query.key).await
            } else {
                return HttpResponse::BadRequest().finish();
            }
        }
        "deleteKey" => {
            if let Some(form) = form_key {
                app_config.delete_key(&form.app, &form.key).await
            } else {
                return HttpResponse::BadRequest().finish();
            }
        }
        "deleteApp" => {
            if let Some(form) = form_app {
                app_config.delete_app(&form.app).await
            } else {
                return HttpResponse::BadRequest().finish();
            }
        }
        _ => return HttpResponse::BadRequest().finish(),
    };

    // Crear respuesta JSON
    let response = JsonResponse {
        status: "success".to_string(),
        data: result,
    };

    HttpResponse::Ok().json(response)
}

#[derive(Deserialize)]
struct AppConfigAction {
    action: String,
}

// Implementación de servicio AppConfig
pub struct AppConfig {
    auth_service: web::Data<AuthService>,
    // Aquí irían las dependencias necesarias, como la conexión a base de datos
}

impl AppConfig {
    pub fn new(auth_service: web::Data<AuthService>) -> Self {
        Self { auth_service }
    }

    pub async fn get_value(
        &self,
        app: &str,
        key: &str,
        default_value: Option<&str>,
    ) -> serde_json::Value {
        // Implementación para obtener valor
        // Por ahora devolvemos un valor simulado
        default_value
            .map(|v| json!(v))
            .unwrap_or_else(|| json!(null))
    }

    pub async fn set_value(&self, app: &str, key: &str, value: &str) -> serde_json::Value {
        // Implementación para establecer valor
        json!(true)
    }

    pub async fn get_apps(&self) -> serde_json::Value {
        // Implementación para obtener aplicaciones
        json!(Vec::<String>::new())
    }

    pub async fn get_keys(&self, app: &str) -> serde_json::Value {
        // Implementación para obtener claves
        json!(Vec::<String>::new())
    }

    pub async fn has_key(&self, app: &str, key: &str) -> serde_json::Value {
        // Implementación para verificar clave
        json!(false)
    }

    pub async fn delete_key(&self, app: &str, key: &str) -> serde_json::Value {
        // Implementación para eliminar clave
        json!(true)
    }

    pub async fn delete_app(&self, app: &str) -> serde_json::Value {
        // Implementación para eliminar aplicación
        json!(true)
    }
}

// Servicio de autenticación simulado
pub struct AuthService {}

impl AuthService {
    pub async fn check_admin_user(&self) -> bool {
        // Implementación real verificaría si el usuario es administrador
        true
    }

    pub async fn check_csrf_token(&self) -> bool {
        // Implementación real verificaría el token CSRF
        true
    }
}