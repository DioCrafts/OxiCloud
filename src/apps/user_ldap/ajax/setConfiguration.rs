//! ownCloud - user_ldap
//!
//! This module handles LDAP configuration settings via REST API.
//!
//! @author Arthur Schiwon
//! @copyright 2013 Arthur Schiwon blizzz@owncloud.com
//!
//! This library is free software; you can redistribute it and/or
//! modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
//! License as published by the Free Software Foundation; either
//! version 3 of the License, or any later version.
//!
//! This library is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//!
//! You should have received a copy of the GNU Affero General Public
//! License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use std::collections::HashMap;

use crate::user_ldap::lib::{Connection, LDAP};
use crate::core::auth::check_admin_user;
use crate::core::app::check_app_enabled;
use crate::core::csrf::verify_csrf_token;

#[derive(Deserialize)]
struct LdapConfigRequest {
    ldap_serverconfig_chooser: String,
    #[serde(flatten)]
    config_params: HashMap<String, String>,
}

/// Handle LDAP configuration request
///
/// This endpoint saves LDAP configuration settings
#[post("/apps/user_ldap/ajax/setConfiguration")]
async fn set_configuration(
    req: web::Json<LdapConfigRequest>,
    session: web::Data<crate::core::session::Session>,
) -> Result<HttpResponse, crate::core::error::Error> {
    // Check user and app status
    check_admin_user(&session)?;
    check_app_enabled("user_ldap")?;
    verify_csrf_token(&req)?;

    let prefix = &req.ldap_serverconfig_chooser;
    let ldap_wrapper = LDAP::new();
    let mut connection = Connection::new(ldap_wrapper, prefix.to_string());
    
    connection.set_configuration(&req.config_params)?;
    connection.save_configuration()?;
    
    Ok(HttpResponse::Ok().json(json!({ "status": "success" })))
}

// Register the handlers with the application
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(set_configuration);
}