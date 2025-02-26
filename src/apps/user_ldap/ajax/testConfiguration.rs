//! # Test Configuration
//!
//! Test LDAP configuration and binding.
//!
//! Originally written by Arthur Schiwon
//! Copyright 2012, 2013 Arthur Schiwon blizzz@owncloud.com
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

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use log::error;

use crate::user_ldap::lib::{Connection, LDAP};
use crate::core::{
    json::{self, JsonResponse},
    auth::check_admin_user,
    app::check_app_enabled,
    l10n::L10n,
};

#[derive(Deserialize)]
pub struct ConfigurationForm {
    #[serde(flatten)]
    config: std::collections::HashMap<String, String>,
}

/// Handler for testing LDAP configuration
pub async fn test_configuration(
    form: web::Form<ConfigurationForm>,
    l10n: web::Data<L10n>,
) -> impl Responder {
    // Check user and app status
    if let Err(response) = check_admin_user() {
        return response;
    }
    
    if let Err(response) = check_app_enabled("user_ldap") {
        return response;
    }

    let ldap_wrapper = LDAP::new();
    let mut connection = Connection::new(ldap_wrapper, String::new(), None);
    
    // Convert the config to the format expected by the connection
    let config = form.into_inner().config;
    
    if connection.set_configuration(config) {
        // Configuration is okay
        if connection.bind() {
            json::success(json::JsonData {
                message: l10n.t("The configuration is valid and the connection could be established!"),
                ..Default::default()
            })
        } else {
            json::error(json::JsonData {
                message: l10n.t("The configuration is valid, but the Bind failed. Please check the server settings and credentials."),
                ..Default::default()
            })
        }
    } else {
        error!("Invalid LDAP configuration");
        json::error(json::JsonData {
            message: l10n.t("The configuration is invalid. Please have a look at the logs for further details."),
            ..Default::default()
        })
    }
}

// Route configuration function to be called from the main application
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/apps/user_ldap/ajax/testConfiguration.php")
            .route(web::post().to(test_configuration))
    );
}