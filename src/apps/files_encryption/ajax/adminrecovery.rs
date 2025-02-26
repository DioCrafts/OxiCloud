//! Script to handle admin settings for encrypted key recovery
//! 
//! Copyright (c) 2013, Sam Tuke <samtuke@owncloud.com>
//! This file is licensed under the Affero General Public License version 3 or later.
//! See the COPYING-README file.

use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::encryption::helper;
use crate::l10n::L10n;
use crate::apps::AppConfig;
use crate::auth::check_admin_user;
use crate::apps::check_app_enabled;

#[derive(Error, Debug)]
pub enum AdminRecoveryError {
    #[error("Authentication failed")]
    AuthError,
    #[error("App not enabled")]
    AppNotEnabledError,
    #[error("Invalid request")]
    InvalidRequestError,
    #[error("Recovery operation failed")]
    RecoveryOperationError,
}

#[derive(Deserialize)]
pub struct AdminRecoveryRequest {
    admin_enable_recovery: String,
    recovery_password: String,
}

#[derive(Serialize)]
pub struct JsonResponse {
    data: ResponseData,
}

#[derive(Serialize)]
pub struct ResponseData {
    message: String,
}

/// Handle admin settings for encrypted key recovery
pub async fn admin_recovery(
    req: web::Json<AdminRecoveryRequest>,
    app_config: web::Data<AppConfig>,
    l10n: web::Data<L10n>,
) -> Result<HttpResponse, AdminRecoveryError> {
    // Check admin permissions and app status
    check_admin_user().map_err(|_| AdminRecoveryError::AuthError)?;
    check_app_enabled("files_encryption").map_err(|_| AdminRecoveryError::AppNotEnabledError)?;
    
    let recovery_key_id = app_config.get_value("files_encryption", "recoveryKeyId")
        .unwrap_or_default();
    
    let result = if req.admin_enable_recovery == "1" {
        // Enable recovery admin
        match helper::admin_enable_recovery(&recovery_key_id, &req.recovery_password).await {
            Ok(true) => {
                let response = JsonResponse {
                    data: ResponseData {
                        message: l10n.t("Recovery key successfully enabled"),
                    },
                };
                Ok(HttpResponse::Ok().json(response))
            },
            _ => {
                let response = JsonResponse {
                    data: ResponseData {
                        message: l10n.t("Could not enable recovery key. Please check your recovery key password!"),
                    },
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        }
    } else if req.admin_enable_recovery == "0" {
        // Disable recovery admin
        match helper::admin_disable_recovery(&req.recovery_password).await {
            Ok(true) => {
                let response = JsonResponse {
                    data: ResponseData {
                        message: l10n.t("Recovery key successfully disabled"),
                    },
                };
                Ok(HttpResponse::Ok().json(response))
            },
            _ => {
                let response = JsonResponse {
                    data: ResponseData {
                        message: l10n.t("Could not disable recovery key. Please check your recovery key password!"),
                    },
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        }
    } else {
        Err(AdminRecoveryError::InvalidRequestError)
    };
    
    result
}