use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::auth::{check_sub_admin_user, is_admin_user, is_user_accessible};
use crate::l10n::L10n;
use crate::middleware::csrf::verify_request_id;
use crate::user::{delete_user, get_current_user};

#[derive(Debug, Error)]
enum RemoveUserError {
    #[error("Authentication error")]
    AuthenticationError,
    #[error("Unable to delete user")]
    DeletionError,
    #[error("Invalid request")]
    InvalidRequest,
}

#[derive(Deserialize)]
struct RemoveUserRequest {
    username: String,
    request_id: String,
}

#[derive(Serialize)]
struct SuccessResponse {
    data: SuccessData,
}

#[derive(Serialize)]
struct SuccessData {
    username: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    data: ErrorData,
}

#[derive(Serialize)]
struct ErrorData {
    message: String,
}

/// Handler for removing a user from the system
pub async fn remove_user(
    form: web::Form<RemoveUserRequest>,
    l10n: web::Data<L10n>,
) -> HttpResponse {
    // Verify CSRF token
    if let Err(_) = verify_request_id(&form.request_id) {
        return HttpResponse::Forbidden().json(ErrorResponse {
            data: ErrorData {
                message: l10n.t("Invalid request"),
            },
        });
    }

    // Verify user permissions
    if let Err(err) = handle_remove_user(&form.username, &l10n).await {
        match err {
            RemoveUserError::AuthenticationError => {
                return HttpResponse::Forbidden().json(ErrorResponse {
                    data: ErrorData {
                        message: l10n.t("Authentication error"),
                    },
                });
            }
            RemoveUserError::DeletionError => {
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    data: ErrorData {
                        message: l10n.t("Unable to delete user"),
                    },
                });
            }
            RemoveUserError::InvalidRequest => {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    data: ErrorData {
                        message: l10n.t("Invalid request"),
                    },
                });
            }
        }
    }

    // Success case
    HttpResponse::Ok().json(SuccessResponse {
        data: SuccessData {
            username: form.username.clone(),
        },
    })
}

async fn handle_remove_user(username: &str, l10n: &L10n) -> Result<(), RemoveUserError> {
    // Check if the user has appropriate permissions
    check_sub_admin_user().map_err(|_| RemoveUserError::AuthenticationError)?;

    // Get current user
    let current_user = get_current_user().ok_or(RemoveUserError::AuthenticationError)?;

    // A user shouldn't be able to delete his own account
    if current_user == username {
        return Err(RemoveUserError::InvalidRequest);
    }

    // Check if current user has rights to delete the target user
    if !is_admin_user(&current_user) && !is_user_accessible(&current_user, username) {
        return Err(RemoveUserError::AuthenticationError);
    }

    // Delete the user
    delete_user(username).map_err(|_| RemoveUserError::DeletionError)?;

    Ok(())
}