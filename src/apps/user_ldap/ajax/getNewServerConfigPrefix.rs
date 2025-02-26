use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use std::error::Error;

/// ownCloud - user_ldap
///
/// @author Arthur Schiwon
/// @copyright 2013 Arthur Schiwon blizzz@owncloud.com
///
/// This library is free software; you can redistribute it and/or
/// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
/// License as published by the Free Software Foundation; either
/// version 3 of the License, or any later version.
///
/// This library is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
///
/// You should have received a copy of the GNU Affero General Public
/// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

pub async fn get_new_server_config_prefix(
    user_info: web::ReqData<crate::auth::UserInfo>,
    app_service: web::Data<crate::services::AppService>,
) -> impl Responder {
    // Check user and app status
    if !user_info.is_admin() {
        return HttpResponse::Forbidden().json(json!({
            "status": "error",
            "message": "Admin privileges required"
        }));
    }

    if !app_service.is_app_enabled("user_ldap") {
        return HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "App user_ldap is not enabled"
        }));
    }

    match generate_new_prefix() {
        Ok(config_prefix) => {
            HttpResponse::Ok().json(json!({
                "status": "success",
                "data": {
                    "configPrefix": config_prefix
                }
            }))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": e.to_string()
            }))
        }
    }
}

fn generate_new_prefix() -> Result<String, Box<dyn Error>> {
    let mut server_connections = crate::user_ldap::helper::get_server_configuration_prefixes()?;
    server_connections.sort();
    
    let last_key = server_connections.pop().unwrap_or_else(|| "s00".to_string());
    let last_num = last_key.trim_start_matches('s').parse::<i32>().unwrap_or(0);
    let new_num = last_num + 1;
    let new_key = format!("s{:02}", new_num);
    
    Ok(new_key)
}

// Helper module stub (to be implemented in the actual project)
pub mod helper {
    use std::error::Error;

    pub fn get_server_configuration_prefixes() -> Result<Vec<String>, Box<dyn Error>> {
        // Actual implementation would retrieve server configuration prefixes
        // This is just a stub
        Ok(Vec::new())
    }
}