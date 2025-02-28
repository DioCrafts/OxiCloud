use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::utils::json::{self, JsonResponse};
use crate::auth::{check_call, check_admin_user};
use crate::groups::GroupManager;
use crate::i18n::Translator;

#[derive(Deserialize)]
pub struct CreateGroupRequest {
    groupname: String,
}

#[derive(Serialize)]
pub struct GroupResponse {
    groupname: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
}

pub async fn create_group(
    req: web::Json<CreateGroupRequest>,
    group_manager: web::Data<GroupManager>,
    translator: web::Data<Translator>,
) -> impl Responder {
    // Verify permission and admin status
    if let Err(e) = check_call() {
        return HttpResponse::Forbidden().json(json::error_response(ErrorResponse {
            message: e.to_string(),
        }));
    }

    if let Err(e) = check_admin_user() {
        return HttpResponse::Forbidden().json(json::error_response(ErrorResponse {
            message: e.to_string(),
        }));
    }

    let groupname = &req.groupname;

    // Does the group exist?
    if group_manager.get_groups().contains(&groupname.to_string()) {
        return HttpResponse::Conflict().json(json::error_response(ErrorResponse {
            message: translator.t("Group already exists"),
        }));
    }

    // Return Success story
    match group_manager.create_group(groupname) {
        Ok(_) => HttpResponse::Created().json(json::success_response(GroupResponse {
            groupname: groupname.to_string(),
        })),
        Err(_) => HttpResponse::InternalServerError().json(json::error_response(ErrorResponse {
            message: translator.t("Unable to add group"),
        })),
    }
}