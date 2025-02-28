/*
 * ownCloud
 *
 * @author Michael Gapczynski
 * @copyright 2012 Michael Gapczynski mtgap@owncloud.com
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU AFFERO GENERAL PUBLIC LICENSE for more details.
 *
 * You should have received a copy of the GNU Affero General Public
 * License along with this library.  If not, see <http://www.gnu.org/licenses/>.
 */

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::oc_json;
use crate::oc_user;
use crate::oc_group;
use crate::oc_sub_admin;
use crate::oc_preferences;

#[derive(Deserialize)]
pub struct UserlistQuery {
    offset: Option<usize>,
    limit: Option<usize>,
}

#[derive(Serialize)]
struct UserInfo {
    name: String,
    displayname: String,
    groups: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    subadmin: Option<String>,
    quota: String,
}

#[derive(Serialize)]
struct Response {
    data: Vec<UserInfo>,
}

pub async fn userlist(
    query: web::Query<UserlistQuery>,
    app_state: web::Data<crate::AppState>,
) -> impl Responder {
    // Verify permissions
    if let Err(e) = oc_json::call_check() {
        return HttpResponse::Forbidden().json(e);
    }

    if let Err(e) = oc_json::check_sub_admin_user() {
        return HttpResponse::Forbidden().json(e);
    }

    // Get parameters from query
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(10);
    
    let current_user = match oc_user::get_user() {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let mut users = Vec::new();

    if oc_user::is_admin_user(&current_user) {
        let batch = oc_user::get_display_names("", limit, offset);
        
        for (user, displayname) in batch {
            let user_groups = oc_group::get_user_groups(&user);
            let subadmin_groups = oc_sub_admin::get_sub_admins_groups(&user);
            let quota = oc_preferences::get_value(&user, "files", "quota", "default");
            
            users.push(UserInfo {
                name: user,
                displayname,
                groups: user_groups.join(", "),
                subadmin: Some(subadmin_groups.join(", ")),
                quota,
            });
        }
    } else {
        let groups = oc_sub_admin::get_sub_admins_groups(&current_user);
        let batch = oc_group::users_in_groups(&groups, "", limit, offset);
        
        for user in batch {
            let user_groups = oc_group::get_user_groups(&user);
            let displayname = oc_user::get_display_name(&user);
            let quota = oc_preferences::get_value(&user, "files", "quota", "default");
            
            users.push(UserInfo {
                name: user,
                displayname,
                groups: user_groups.join(", "),
                subadmin: None,
                quota,
            });
        }
    }

    HttpResponse::Ok().json(Response { data: users })
}