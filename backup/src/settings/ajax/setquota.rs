use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::errors::AppError;
use crate::helpers::file_size::{computer_file_size, human_file_size};
use crate::l10n::L10n;
use crate::preferences::{Preferences, AppConfig};
use crate::user::{User, SubAdmin};

/**
 * Copyright (c) 2012, Robin Appelman <icewind1991@gmail.com>
 * This file is licensed under the Affero General Public License version 3 or later.
 * See the COPYING-README file.
 */

#[derive(Deserialize)]
pub struct SetQuotaRequest {
    username: Option<String>,
    quota: String,
}

#[derive(Serialize)]
pub struct SetQuotaResponse {
    data: SetQuotaData,
}

#[derive(Serialize)]
pub struct SetQuotaData {
    username: String,
    quota: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    data: ErrorData,
}

#[derive(Serialize)]
pub struct ErrorData {
    message: String,
}

pub async fn set_quota(
    req: web::Json<SetQuotaRequest>,
    user_service: web::Data<Arc<dyn User>>,
    sub_admin_service: web::Data<Arc<dyn SubAdmin>>,
    preferences: web::Data<Arc<dyn Preferences>>,
    app_config: web::Data<Arc<dyn AppConfig>>,
    l10n: web::Data<Arc<dyn L10n>>,
) -> Result<impl Responder, AppError> {
    // Check if the request is from a subadmin user
    if !user_service.is_sub_admin_user().await? {
        return Err(AppError::Unauthorized("Not a subadmin user".to_string()));
    }
    
    // Check if the request is valid
    if !user_service.call_check().await? {
        return Err(AppError::Unauthorized("Invalid call check".to_string()));
    }

    let username = req.username.clone().unwrap_or_default();
    let current_user = user_service.get_user().await?;

    if (username.is_empty() && !user_service.is_admin_user(&current_user).await?)
        || (!user_service.is_admin_user(&current_user).await?
            && !sub_admin_service.is_user_accessible(&current_user, &username).await?)
    {
        let message = l10n.get("core", "Authentication error").await?;
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            data: ErrorData { message },
        }));
    }

    // Make sure the quota is in the expected format
    let mut quota = req.quota.clone();
    if quota != "none" && quota != "default" {
        let bytes = computer_file_size(&quota)?;
        quota = human_file_size(bytes)?;
    }

    // Set the quota
    if !username.is_empty() {
        // Set quota for specific user
        preferences.set_value(&username, "files", "quota", &quota).await?;
    } else {
        // Set the default quota when no username is specified
        if quota == "default" {
            // 'default' as default quota makes no sense
            quota = "none".to_string();
        }
        app_config.set_value("files", "default_quota", &quota).await?;
    }

    Ok(HttpResponse::Ok().json(SetQuotaResponse {
        data: SetQuotaData {
            username,
            quota,
        },
    }))
}