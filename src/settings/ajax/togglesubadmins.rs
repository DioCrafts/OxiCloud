use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use oc_server::{
    auth::check_admin_user,
    csrf::check_request_token,
    json_response::JsonResponse,
    subadmin::{is_subadmin_of_group, delete_subadmin, create_subadmin},
};

#[derive(Deserialize)]
pub struct ToggleSubadminRequest {
    username: String,
    group: String,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    status: &'static str,
}

pub async fn toggle_subadmins(
    req: web::Json<ToggleSubadminRequest>,
    session: web::ReqData<oc_server::session::Session>,
) -> impl Responder {
    // Check admin permissions
    if let Err(e) = check_admin_user(&session) {
        return HttpResponse::Forbidden().json(JsonResponse::error(&e.to_string()));
    }

    // Check CSRF token
    if let Err(e) = check_request_token(&session) {
        return HttpResponse::BadRequest().json(JsonResponse::error(&e.to_string()));
    }

    let username = &req.username;
    let group = &req.group;

    // Toggle group
    let result = if is_subadmin_of_group(username, group).await {
        delete_subadmin(username, group).await
    } else {
        create_subadmin(username, group).await
    };

    match result {
        Ok(_) => HttpResponse::Ok().json(SuccessResponse { status: "success" }),
        Err(e) => HttpResponse::InternalServerError().json(JsonResponse::error(&e.to_string())),
    }
}