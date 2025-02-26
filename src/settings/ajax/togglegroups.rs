use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use futures::future::{self, Future};

#[derive(Deserialize)]
struct ToggleGroupsRequest {
    username: String,
    group: String,
}

#[derive(Serialize)]
struct SuccessResponse {
    data: SuccessResponseData,
}

#[derive(Serialize)]
struct SuccessResponseData {
    username: String,
    action: String,
    groupname: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    data: ErrorResponseData,
}

#[derive(Serialize)]
struct ErrorResponseData {
    message: String,
}

pub async fn toggle_groups(
    request: web::Json<ToggleGroupsRequest>,
    auth_service: web::Data<dyn AuthService>,
    group_service: web::Data<dyn GroupService>,
    l10n_service: web::Data<dyn L10nService>,
) -> Result<HttpResponse, Error> {
    // Check if user is subadmin
    if !auth_service.is_sub_admin_user().await {
        return Ok(HttpResponse::Forbidden().finish());
    }

    // CSRF protection
    auth_service.call_check().await?;

    let username = &request.username;
    let group = &request.group;
    let current_user = auth_service.get_user().await;

    // Check if user is trying to remove themself from admin group
    if username == &current_user && group == "admin" && auth_service.is_admin_user(username).await {
        let l = l10n_service.get("core").await;
        let error_msg = l.t("Admins can't remove themself from the admin group");
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            data: ErrorResponseData {
                message: error_msg,
            },
        }));
    }

    // Check permissions
    if !auth_service.is_admin_user(&current_user).await
        && (!auth_service.is_user_accessible(&current_user, username).await
            || !auth_service.is_group_accessible(&current_user, group).await)
    {
        let l = l10n_service.get("core").await;
        let error_msg = l.t("Authentication error");
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            data: ErrorResponseData {
                message: error_msg,
            },
        }));
    }

    // Create group if it doesn't exist
    if !group_service.group_exists(group).await {
        group_service.create_group(group).await?;
    }

    let l = l10n_service.get("settings").await;

    let mut error = l.t(&format!("Unable to add user to group {}", group));
    let mut action = "add".to_string();
    let mut success = false;

    // Toggle group
    if group_service.in_group(username, group).await {
        action = "remove".to_string();
        error = l.t(&format!("Unable to remove user from group {}", group));
        success = group_service.remove_from_group(username, group).await?;
        
        let users_in_group = group_service.users_in_group(group).await?;
        if users_in_group.is_empty() {
            group_service.delete_group(group).await?;
        }
    } else {
        success = group_service.add_to_group(username, group).await?;
    }

    // Return response
    if success {
        Ok(HttpResponse::Ok().json(SuccessResponse {
            data: SuccessResponseData {
                username: username.clone(),
                action,
                groupname: group.clone(),
            },
        }))
    } else {
        Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            data: ErrorResponseData {
                message: error,
            },
        }))
    }
}

// Trait definitions that would be implemented elsewhere
#[async_trait::async_trait]
pub trait AuthService: Send + Sync {
    async fn is_sub_admin_user(&self) -> bool;
    async fn call_check(&self) -> Result<(), Error>;
    async fn get_user(&self) -> String;
    async fn is_admin_user(&self, username: &str) -> bool;
    async fn is_user_accessible(&self, admin: &str, user: &str) -> bool;
    async fn is_group_accessible(&self, admin: &str, group: &str) -> bool;
}

#[async_trait::async_trait]
pub trait GroupService: Send + Sync {
    async fn group_exists(&self, group: &str) -> bool;
    async fn create_group(&self, group: &str) -> Result<(), Error>;
    async fn in_group(&self, username: &str, group: &str) -> bool;
    async fn add_to_group(&self, username: &str, group: &str) -> Result<bool, Error>;
    async fn remove_from_group(&self, username: &str, group: &str) -> Result<bool, Error>;
    async fn users_in_group(&self, group: &str) -> Result<Vec<String>, Error>;
    async fn delete_group(&self, group: &str) -> Result<(), Error>;
}

pub trait L10nService: Send + Sync {
    async fn get(&self, domain: &str) -> Box<dyn L10n>;
}

pub trait L10n: Send + Sync {
    fn t(&self, text: &str) -> String;
}