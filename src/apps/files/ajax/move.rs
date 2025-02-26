use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use futures::future::{self, Future};
use log::error;

#[derive(Deserialize)]
pub struct MoveRequest {
    dir: String,
    file: String,
    target: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    data: ErrorData,
}

#[derive(Serialize)]
struct ErrorData {
    message: String,
}

#[derive(Serialize)]
struct SuccessResponse {
    data: SuccessData,
}

#[derive(Serialize)]
struct SuccessData {
    dir: String,
    files: String,
}

/// Handles file move operation
pub async fn move_file(
    request: web::Json<MoveRequest>,
    identity: web::ReqData<crate::identity::Identity>,
    file_system: web::Data<crate::fs::FileSystem>,
    i18n: web::Data<crate::l10n::L10n>,
) -> Result<HttpResponse, Error> {
    // Check logged in status
    if !identity.is_logged_in() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    // Get request data
    let dir = request.dir.trim_matches('\\');
    let file = request.file.trim_matches('\\');
    let target = urlencoding::decode(&request.target.trim_matches('\\'))
        .map_err(|_| HttpResponse::BadRequest().finish())?;

    let l = i18n.get("files");

    // Check if target file already exists
    if file_system.file_exists(&format!("{}/{}", target, file)).await {
        let msg = l.t("Could not move %s - File with this name already exists", &[file]);
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            data: ErrorData { message: msg },
        }));
    }

    // Don't allow moving the "Shared" folder from root
    if dir.is_empty() && file == "Shared" {
        let msg = l.t("Could not move %s", &[file]);
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            data: ErrorData { message: msg },
        }));
    }

    // Normalize paths
    let target_file = file_system.normalize_path(&format!("{}/{}", target, file));
    let source_file = file_system.normalize_path(&format!("{}/{}", dir, file));

    // Perform rename operation
    match file_system.rename(&source_file, &target_file).await {
        Ok(_) => Ok(HttpResponse::Ok().json(SuccessResponse {
            data: SuccessData {
                dir: dir.to_string(),
                files: file.to_string(),
            },
        })),
        Err(_) => {
            let msg = l.t("Could not move %s", &[file]);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                data: ErrorData { message: msg },
            }))
        }
    }
}