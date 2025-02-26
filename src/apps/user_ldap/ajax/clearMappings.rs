//! ownCloud - user_ldap
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
//!
//! @author Arthur Schiwon
//! @copyright 2013 Arthur Schiwon blizzz@owncloud.com

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::user_ldap::lib::helper;
use crate::core::json;
use crate::core::l10n;

#[derive(Deserialize)]
struct ClearMappingRequest {
    ldap_clear_mapping: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    status: &'static str,
    data: ErrorData,
}

#[derive(Serialize)]
struct ErrorData {
    message: String,
}

#[derive(Serialize)]
struct SuccessResponse {
    status: &'static str,
}

pub async fn clear_mappings(
    request: web::Json<ClearMappingRequest>,
    session: web::Data<crate::core::session::Session>,
) -> impl Responder {
    // Check user and app status
    if !json::check_admin_user(&session) {
        return HttpResponse::Forbidden().finish();
    }
    
    if !json::check_app_enabled("user_ldap") {
        return HttpResponse::BadRequest().body("App not enabled");
    }
    
    if !json::call_check(&session) {
        return HttpResponse::BadRequest().body("CSRF check failed");
    }

    let subject = &request.ldap_clear_mapping;
    
    match helper::clear_mapping(subject) {
        true => {
            let response = SuccessResponse {
                status: "success",
            };
            HttpResponse::Ok().json(response)
        },
        false => {
            let l10n = l10n::get("user_ldap");
            let error_response = ErrorResponse {
                status: "error",
                data: ErrorData {
                    message: l10n.translate("Failed to clear the mappings."),
                },
            };
            HttpResponse::InternalServerError().json(error_response)
        }
    }
}