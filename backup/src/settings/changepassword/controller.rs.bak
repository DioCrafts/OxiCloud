use actix_web::{web, Error, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// Estructuras para los datos de entrada
#[derive(Deserialize)]
pub struct PersonalPasswordData {
    #[serde(rename = "personal-password")]
    personal_password: Option<String>,
    oldpassword: String,
}

#[derive(Deserialize)]
pub struct UserPasswordData {
    username: String,
    password: Option<String>,
    recovery_password: Option<String>,
}

// Estructuras para las respuestas JSON
#[derive(Serialize)]
struct JsonResponse<T> {
    data: T,
}

#[derive(Serialize)]
struct ErrorMessage {
    message: String,
}

#[derive(Serialize)]
struct SuccessUserData {
    username: String,
}

// Estructura principal del controlador
pub struct Controller {
    user_service: Arc<dyn UserService>,
    app_service: Arc<dyn AppService>,
    filesystem_view: Arc<dyn FilesystemView>,
    app_config: Arc<dyn AppConfig>,
}

// Traits necesarios para la implementación
pub trait UserService: Send + Sync {
    fn get_user(&self) -> String;
    fn check_password(&self, username: &str, password: &str) -> bool;
    fn set_password(&self, username: &str, password: &str) -> bool;
    fn set_password_with_recovery(&self, username: &str, password: &str, recovery_password: &str) -> bool;
    fn is_admin_user(&self, username: &str) -> bool;
}

pub trait AppService: Send + Sync {
    fn load_apps(&self);
    fn is_enabled(&self, app_name: &str) -> bool;
}

pub trait L10n: Send + Sync {
    fn translate(&self, text: &str) -> String;
}

pub trait FilesystemView: Send + Sync {
    fn new(&self, path: &str) -> Self;
}

pub trait AppConfig: Send + Sync {
    fn get_value(&self, app: &str, key: &str) -> Option<String>;
}

pub trait SubAdmin: Send + Sync {
    fn is_user_accessible(&self, admin: &str, user: &str) -> bool;
}

// Estructura de utilidad para el cifrado
struct EncryptionUtil {
    view: Arc<dyn FilesystemView>,
    username: String,
}

impl EncryptionUtil {
    fn new(view: Arc<dyn FilesystemView>, username: String) -> Self {
        Self { view, username }
    }

    fn check_recovery_password(&self, recovery_password: &str) -> bool {
        // Implementación simulada
        !recovery_password.is_empty()
    }

    fn recovery_enabled_for_user(&self) -> bool {
        // Implementación simulada
        true
    }
}

impl Controller {
    pub fn new(
        user_service: Arc<dyn UserService>,
        app_service: Arc<dyn AppService>,
        filesystem_view: Arc<dyn FilesystemView>,
        app_config: Arc<dyn AppConfig>,
    ) -> Self {
        Self {
            user_service,
            app_service,
            filesystem_view,
            app_config,
        }
    }

    pub async fn change_personal_password(
        &self,
        data: web::Json<PersonalPasswordData>,
        l10n_factory: web::Data<Arc<dyn Fn() -> Box<dyn L10n>>>,
    ) -> Result<HttpResponse, Error> {
        // Check if we are an user
        // Call check and logged in check are assumed to be done by middleware
        
        // Manually load apps to ensure hooks work correctly (workaround for issue 1503)
        self.app_service.load_apps();

        let username = self.user_service.get_user();
        let password = data.personal_password.clone();
        let old_password = &data.oldpassword;

        if !self.user_service.check_password(&username, old_password) {
            let l10n = (l10n_factory.get_ref())();
            return Ok(HttpResponse::BadRequest().json(JsonResponse {
                data: ErrorMessage {
                    message: l10n.translate("Wrong password"),
                },
            }));
        }

        if let Some(password) = password {
            if self.user_service.set_password(&username, &password) {
                Ok(HttpResponse::Ok().json({}))
            } else {
                Ok(HttpResponse::InternalServerError().json({}))
            }
        } else {
            Ok(HttpResponse::InternalServerError().json({}))
        }
    }

    pub async fn change_user_password(
        &self,
        data: web::Json<UserPasswordData>,
        l10n_factory: web::Data<Arc<dyn Fn() -> Box<dyn L10n>>>,
        sub_admin: web::Data<Arc<dyn SubAdmin>>,
    ) -> Result<HttpResponse, Error> {
        // Check if we are an user
        // Call check and logged in check are assumed to be done by middleware

        // Manually load apps to ensure hooks work correctly (workaround for issue 1503)
        self.app_service.load_apps();

        let username = match &data.username {
            s if !s.is_empty() => s.clone(),
            _ => {
                let l10n = (l10n_factory.get_ref())();
                return Ok(HttpResponse::BadRequest().json(JsonResponse {
                    data: ErrorMessage {
                        message: l10n.translate("No user supplied"),
                    },
                }));
            }
        };

        let password = data.password.clone();
        let recovery_password = data.recovery_password.clone();
        let current_user = self.user_service.get_user();

        // Verificar permisos
        let user_status = if self.user_service.is_admin_user(&current_user) {
            "admin"
        } else if sub_admin.is_user_accessible(&current_user, &username) {
            "subadmin"
        } else {
            let l10n = (l10n_factory.get_ref())();
            return Ok(HttpResponse::Forbidden().json(JsonResponse {
                data: ErrorMessage {
                    message: l10n.translate("Authentication error"),
                },
            }));
        };

        if self.app_service.is_enabled("files_encryption") {
            // Manejar caso de recuperación
            let util = EncryptionUtil::new(
                self.filesystem_view.clone(),
                username.clone(),
            );
            
            let recovery_admin_enabled = self
                .app_config
                .get_value("files_encryption", "recoveryAdminEnabled")
                .unwrap_or_default();

            let mut valid_recovery_password = false;
            let recovery_password_supported = false;
            
            if recovery_admin_enabled == "1" || recovery_admin_enabled == "true" {
                if let Some(ref recovery_password) = recovery_password {
                    valid_recovery_password = util.check_recovery_password(recovery_password);
                }
                let recovery_enabled_for_user = util.recovery_enabled_for_user();
                
                if recovery_enabled_for_user {
                    if recovery_password.as_ref().map_or(true, |s| s.is_empty()) {
                        let l10n = (l10n_factory.get_ref())();
                        return Ok(HttpResponse::BadRequest().json(JsonResponse {
                            data: ErrorMessage {
                                message: l10n.translate("Please provide an admin recovery password, otherwise all user data will be lost"),
                            },
                        }));
                    } else if !valid_recovery_password {
                        let l10n = (l10n_factory.get_ref())();
                        return Ok(HttpResponse::BadRequest().json(JsonResponse {
                            data: ErrorMessage {
                                message: l10n.translate("Wrong admin recovery password. Please check the password and try again."),
                            },
                        }));
                    }
                }
            }

            // Ahora sabemos que todo está bien con respecto a la contraseña de recuperación, intentemos cambiar la contraseña
            let result = if let (Some(password), Some(recovery_password)) = (password, recovery_password) {
                self.user_service.set_password_with_recovery(&username, &password, &recovery_password)
            } else if let Some(password) = password {
                self.user_service.set_password(&username, &password)
            } else {
                false
            };

            if !result && recovery_password_supported {
                let l10n = (l10n_factory.get_ref())();
                Ok(HttpResponse::BadRequest().json(JsonResponse {
                    data: ErrorMessage {
                        message: l10n.translate("Back-end doesn't support password change, but the users encryption key was successfully updated."),
                    },
                }))
            } else if !result && !recovery_password_supported {
                let l10n = (l10n_factory.get_ref())();
                Ok(HttpResponse::InternalServerError().json(JsonResponse {
                    data: ErrorMessage {
                        message: l10n.translate("Unable to change password"),
                    },
                }))
            } else {
                Ok(HttpResponse::Ok().json(JsonResponse {
                    data: SuccessUserData { username },
                }))
            }
        } else {
            // Si el cifrado está deshabilitado, proceder
            if let Some(password) = password {
                if self.user_service.set_password(&username, &password) {
                    Ok(HttpResponse::Ok().json(JsonResponse {
                        data: SuccessUserData { username },
                    }))
                } else {
                    let l10n = (l10n_factory.get_ref())();
                    Ok(HttpResponse::InternalServerError().json(JsonResponse {
                        data: ErrorMessage {
                            message: l10n.translate("Unable to change password"),
                        },
                    }))
                }
            } else {
                let l10n = (l10n_factory.get_ref())();
                Ok(HttpResponse::BadRequest().json(JsonResponse {
                    data: ErrorMessage {
                        message: l10n.translate("No password supplied"),
                    },
                }))
            }
        }
    }
}