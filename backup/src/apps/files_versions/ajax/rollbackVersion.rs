use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::apps::files_versions::storage;
use crate::core::l10n::L10n;
use crate::core::app::check_app_enabled;

#[derive(Deserialize)]
pub struct RollbackParams {
    file: String,
    revision: i32,
}

#[derive(Serialize)]
struct SuccessResponse {
    data: SuccessData,
}

#[derive(Serialize)]
struct SuccessData {
    revision: i32,
    file: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    data: ErrorData,
}

#[derive(Serialize)]
struct ErrorData {
    message: String,
}

pub async fn rollback_version(
    params: web::Query<RollbackParams>,
    l10n: web::Data<Arc<L10n>>,
) -> impl Responder {
    // Check if app is enabled
    if let Err(_) = check_app_enabled("files_versions") {
        return HttpResponse::NotFound().finish();
    }

    let file = &params.file;
    let revision = params.revision;

    match storage::rollback(file, revision).await {
        Ok(_) => {
            let response = SuccessResponse {
                data: SuccessData {
                    revision,
                    file: file.clone(),
                },
            };
            HttpResponse::Ok().json(response)
        }
        Err(_) => {
            let message = l10n.t("Could not revert: %s", &[file]);
            let response = ErrorResponse {
                data: ErrorData { message },
            };
            HttpResponse::BadRequest().json(response)
        }
    }
}