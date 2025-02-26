use actix_web::{web, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt;
use async_trait::async_trait;
use std::sync::Arc;
use anyhow::{Result, anyhow, Context};

// Simulamos la carga de la biblioteca de Dropbox que en PHP se hacía con require_once
mod dropbox {
    use anyhow::Result;
    
    #[derive(Clone)]
    pub struct OAuthCurl {
        app_key: String,
        app_secret: String,
        token: Option<(String, String)>,
    }

    impl OAuthCurl {
        pub fn new(app_key: String, app_secret: String) -> Self {
            Self {
                app_key,
                app_secret,
                token: None,
            }
        }

        pub fn set_token(&mut self, token: String, token_secret: String) {
            self.token = Some((token, token_secret));
        }

        pub async fn get_request_token(&self) -> Result<(String, String)> {
            // Implementación real requeriría llamadas HTTP a la API de Dropbox
            // Simulamos el comportamiento del original
            Ok(("request_token_value".to_string(), "request_token_secret_value".to_string()))
        }

        pub fn get_authorize_url(&self, callback: Option<String>) -> String {
            // Simulamos el comportamiento del original
            format!("https://dropbox.com/authorize?callback={}", callback.unwrap_or_default())
        }

        pub async fn get_access_token(&self) -> Result<(String, String)> {
            // Implementación real requeriría llamadas HTTP a la API de Dropbox
            // Simulamos el comportamiento del original
            Ok(("access_token_value".to_string(), "access_token_secret_value".to_string()))
        }
    }
}

// Simulamos la funcionalidad OCP de PHP
#[async_trait]
trait OcpJsonTrait {
    async fn check_app_enabled(app: &str) -> Result<()>;
    async fn check_logged_in() -> Result<()>;
    async fn call_check() -> Result<()>;
}

struct OcpJson;

#[async_trait]
impl OcpJsonTrait for OcpJson {
    async fn check_app_enabled(app: &str) -> Result<()> {
        // Simulamos la verificación de que la app esté habilitada
        if app == "files_external" {
            Ok(())
        } else {
            Err(anyhow!("App {} is not enabled", app))
        }
    }

    async fn check_logged_in() -> Result<()> {
        // Simulamos la verificación de inicio de sesión
        Ok(())
    }

    async fn call_check() -> Result<()> {
        // Simulamos la verificación de CSRF
        Ok(())
    }
}

// Estructuras para deserializar la solicitud
#[derive(Deserialize)]
struct DropboxStep1Request {
    app_key: String,
    app_secret: String,
    step: u8,
    callback: Option<String>,
}

#[derive(Deserialize)]
struct DropboxStep2Request {
    app_key: String,
    app_secret: String,
    step: u8,
    request_token: String,
    request_token_secret: String,
}

#[derive(Deserialize)]
struct DropboxBaseRequest {
    app_key: String,
    app_secret: String,
    step: Option<u8>,
}

// Estructuras para serializar la respuesta
#[derive(Serialize)]
struct Step1SuccessData {
    url: String,
    request_token: String,
    request_token_secret: String,
}

#[derive(Serialize)]
struct Step1Response {
    data: Step1SuccessData,
}

#[derive(Serialize)]
struct Step2Response {
    access_token: String,
    access_token_secret: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    data: ErrorData,
}

#[derive(Serialize)]
struct ErrorData {
    message: String,
}

// Enum para el manejo de errores específicos
#[derive(Debug)]
enum DropboxError {
    InvalidToken,
    AppKeySecretMissing,
    RequestTokenFailed,
    AccessTokenFailed,
    InternalError(anyhow::Error),
}

impl fmt::Display for DropboxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidToken => write!(f, "Invalid token"),
            Self::AppKeySecretMissing => write!(f, "Please provide a valid Dropbox app key and secret."),
            Self::RequestTokenFailed => write!(f, "Fetching request tokens failed. Verify that your Dropbox app key and secret are correct."),
            Self::AccessTokenFailed => write!(f, "Fetching access tokens failed. Verify that your Dropbox app key and secret are correct."),
            Self::InternalError(e) => write!(f, "Internal error: {}", e),
        }
    }
}

impl ResponseError for DropboxError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            data: ErrorData {
                message: self.to_string(),
            },
        };
        
        HttpResponse::BadRequest().json(error_response)
    }
}

impl From<anyhow::Error> for DropboxError {
    fn from(err: anyhow::Error) -> Self {
        DropboxError::InternalError(err)
    }
}

// Servicio principal que maneja la solicitud de Dropbox
async fn handle_dropbox(
    form: web::Form<DropboxBaseRequest>,
    app_state: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, DropboxError> {
    // Verificar que la aplicación esté habilitada, el usuario esté autenticado, y la verificación CSRF
    OcpJson::check_app_enabled("files_external").await?;
    OcpJson::check_logged_in().await?;
    OcpJson::call_check().await?;

    // Extraer app_key y app_secret
    let app_key = &form.app_key;
    let app_secret = &form.app_secret;

    if app_key.is_empty() || app_secret.is_empty() {
        return Err(DropboxError::AppKeySecretMissing);
    }

    // Procesar según el paso (step)
    match form.step {
        Some(1) => {
            if let Ok(step1_form) = serde_json::from_value::<DropboxStep1Request>(serde_json::to_value(form.0).unwrap()) {
                let mut oauth = dropbox::OAuthCurl::new(app_key.clone(), app_secret.clone());
                
                // Obtener token de solicitud
                let (token, token_secret) = oauth
                    .get_request_token()
                    .await
                    .map_err(|_| DropboxError::RequestTokenFailed)?;
                
                // Construir URL de autorización
                let authorize_url = oauth.get_authorize_url(step1_form.callback);
                
                // Construir y enviar respuesta exitosa
                let response = Step1Response {
                    data: Step1SuccessData {
                        url: authorize_url,
                        request_token: token,
                        request_token_secret: token_secret,
                    },
                };
                
                Ok(HttpResponse::Ok().json(response))
            } else {
                Err(DropboxError::InternalError(anyhow!("Invalid request format")))
            }
        },
        Some(2) => {
            if let Ok(step2_form) = serde_json::from_value::<DropboxStep2Request>(serde_json::to_value(form.0).unwrap()) {
                let mut oauth = dropbox::OAuthCurl::new(app_key.clone(), app_secret.clone());
                
                // Establecer token de solicitud
                oauth.set_token(step2_form.request_token, step2_form.request_token_secret);
                
                // Obtener token de acceso
                let (access_token, access_token_secret) = oauth
                    .get_access_token()
                    .await
                    .map_err(|_| DropboxError::AccessTokenFailed)?;
                
                // Construir y enviar respuesta exitosa
                let response = Step2Response {
                    access_token,
                    access_token_secret,
                };
                
                Ok(HttpResponse::Ok().json(response))
            } else {
                Err(DropboxError::InternalError(anyhow!("Invalid request format")))
            }
        },
        _ => Err(DropboxError::InternalError(anyhow!("Invalid step"))),
    }
}

// Estructura para el estado de la aplicación
struct AppState {
    // Puede contener configuración, conexiones a bases de datos, etc.
}

// Configuración de las rutas
fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/apps/files_external/ajax/dropbox.php")
            .route(web::post().to(handle_dropbox))
    );
}

// Función principal
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = Arc::new(AppState {});
    
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::new(app_state.clone()))
            .configure(config_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}