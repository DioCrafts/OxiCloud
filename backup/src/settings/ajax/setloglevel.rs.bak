// Copyright (c) 2011, Robin Appelman <icewind1991@gmail.com>
// This file is licensed under the Affero General Public License version 3 or later.
// See the COPYING-README file.

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::config::AppConfig;
use crate::middleware::auth::ensure_admin;
use crate::middleware::csrf::verify_csrf_token;

#[derive(Deserialize)]
pub struct SetLogLevelRequest {
    level: u8,
}

#[derive(Serialize)]
pub struct ApiResponse {
    success: bool,
}

pub async fn set_log_level(
    State(config): State<Arc<AppConfig>>,
    current_user: crate::models::user::User,
    Json(payload): Json<SetLogLevelRequest>,
) -> Response {
    // Check if user is admin
    if let Err(err) = ensure_admin(&current_user) {
        return err.into_response();
    }

    // Verify CSRF token
    if let Err(err) = verify_csrf_token() {
        return err.into_response();
    }

    // Update the log level in the config
    if let Err(_) = config.set_value("loglevel", payload.level) {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    // Return success response
    Json(ApiResponse { success: true }).into_response()
}