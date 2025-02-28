// # LDAP User Backend Configuration Delete Endpoint
//
// This module provides functionality to delete LDAP server configurations.
//
// Originally based on the PHP file:
// apps/user_ldap/ajax/deleteConfiguration.php
//
// ## License
//
// AGPL-3.0-or-later
//
// Originally authored by Arthur Schiwon (blizzz@owncloud.com)
// Copyright 2013 Arthur Schiwon

use actix_web::{delete, web, HttpResponse};
use serde::Deserialize;
use std::sync::Arc;

use crate::app::AppState;
use crate::auth::middleware::AdminOnly;
use crate::errors::{ApiError, ApiResult};
use crate::i18n::Translator;
use crate::user_ldap::helper::Helper;

/// Data structure for the incoming DELETE request
#[derive(Deserialize)]
pub struct DeleteConfigRequest {
    ldap_serverconfig_chooser: String,
}

/// Handler for LDAP server configuration deletion
///
/// This endpoint requires admin privileges and the LDAP app to be enabled.
/// It deletes a server configuration by the provided prefix.
#[delete("/settings/ldap/configuration")]
pub async fn delete_configuration(
    state: web::Data<Arc<AppState>>,
    req: web::Json<DeleteConfigRequest>,
    _: AdminOnly,
) -> ApiResult<HttpResponse> {
    // Check if the LDAP app is enabled
    if !state.app_manager.is_app_enabled("user_ldap") {
        return Err(ApiError::AppDisabled("user_ldap".to_string()));
    }

    // Get the prefix from the request
    let prefix = &req.ldap_serverconfig_chooser;

    // Attempt to delete the server configuration
    match Helper::delete_server_configuration(prefix).await {
        Ok(true) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "success"
        }))),
        Ok(false) | Err(_) => {
            let translator = Translator::new("user_ldap");
            let message = translator.translate("Failed to delete the server configuration");
            
            Err(ApiError::InternalError(message))
        }
    }
}

/// Register the LDAP configuration endpoints with the Actix web application
pub fn register(config: &mut web::ServiceConfig) {
    config.service(delete_configuration);
}