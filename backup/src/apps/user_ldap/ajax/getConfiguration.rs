// ownCloud - user_ldap
//
// @author Arthur Schiwon
// @copyright 2013 Arthur Schiwon blizzz@owncloud.com
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::auth::{check_admin_user, check_app_enabled, check_token};
use crate::user_ldap::lib::{Connection, LDAP};

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("User is not an admin")]
    NotAdmin,
    #[error("App is not enabled")]
    AppDisabled,
    #[error("Invalid CSRF token")]
    InvalidToken,
    #[error("LDAP connection error: {0}")]
    LdapError(String),
}

#[derive(Deserialize)]
pub struct ConfigRequest {
    ldap_serverconfig_chooser: String,
}

#[derive(Serialize)]
pub struct ConfigResponse {
    configuration: std::collections::HashMap<String, String>,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    status: &'static str,
    data: ConfigResponse,
}

/// Get LDAP configuration
///
/// This endpoint retrieves the LDAP configuration for the specified prefix.
/// Requires admin privileges and the user_ldap app to be enabled.
#[post("/apps/user_ldap/ajax/getConfiguration.php")]
pub async fn get_configuration(
    data: web::Data<AppState>,
    form: web::Form<ConfigRequest>,
    req: web::HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    // Check user and app status
    check_admin_user(&req).map_err(|_| actix_web::error::ErrorForbidden(ConfigError::NotAdmin))?;
    check_app_enabled("user_ldap").map_err(|_| actix_web::error::ErrorForbidden(ConfigError::AppDisabled))?;
    check_token(&req).map_err(|_| actix_web::error::ErrorForbidden(ConfigError::InvalidToken))?;

    let prefix = &form.ldap_serverconfig_chooser;
    
    // Create LDAP connection
    let ldap_wrapper = LDAP::new();
    let connection = Connection::new(ldap_wrapper, prefix.to_string());
    
    // Get configuration
    let configuration = connection.get_configuration()
        .map_err(|e| actix_web::error::ErrorInternalServerError(ConfigError::LdapError(e.to_string())))?;
    
    // Return success response
    let response = SuccessResponse {
        status: "success",
        data: ConfigResponse {
            configuration,
        },
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub struct AppState {
    // Application state goes here
}