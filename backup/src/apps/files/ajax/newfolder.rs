use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::auth::{check_logged_in, call_check};
use crate::filesystem::{FileInfo, FileSystem};
use crate::l10n::L10n;

#[derive(Deserialize)]
pub struct NewFolderRequest {
    dir: Option<String>,
    foldername: Option<String>,
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    data: ErrorData,
}

#[derive(Serialize)]
struct ErrorData {
    message: String,
}

#[derive(Serialize)]
struct SuccessResponse {
    success: bool,
    data: SuccessData,
}

#[derive(Serialize)]
struct SuccessData {
    id: u64,
}

/// Handles the creation of a new folder
pub async fn new_folder(
    req: web::Json<NewFolderRequest>,
    filesystem: web::Data<FileSystem>,
    l10n: web::Data<L10n>,
) -> Result<HttpResponse, Error> {
    // Init owncloud
    check_logged_in()?;
    call_check()?;

    // Get the params
    let dir = req.dir.clone().unwrap_or_default();
    let foldername = req.foldername.clone().unwrap_or_default();
    let l10n = l10n.get("files");

    // Validate folder name is not empty
    if foldername.trim().is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            data: ErrorData {
                message: l10n.t("Folder name cannot be empty."),
            },
        }));
    }

    // Validate folder name does not contain "/"
    if foldername.contains('/') {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            data: ErrorData {
                message: l10n.t("Folder name must not contain \"/\". Please choose a different name."),
            },
        }));
    }

    let target = format!("{}/{}", dir, foldername);
    
    // Check if the target already exists
    if filesystem.file_exists(&target).await {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            data: ErrorData {
                message: l10n.t_args(
                    "The name %s is already used in the folder %s. Please choose a different name.",
                    &[&foldername, &dir],
                ),
            },
        }));
    }

    // Create the directory
    if filesystem.mkdir(&target).await? {
        let path = if dir != "/" {
            format!("{}/{}", dir, foldername)
        } else {
            format!("/{}", foldername)
        };
        
        let meta = filesystem.get_file_info(&path).await?;
        let id = meta.file_id;
        
        return Ok(HttpResponse::Ok().json(SuccessResponse {
            success: true,
            data: SuccessData { id },
        }));
    }

    // If we reached here, there was an error creating the folder
    Ok(HttpResponse::InternalServerError().json(ErrorResponse {
        success: false,
        data: ErrorData {
            message: l10n.t("Error when creating the folder"),
        },
    }))
}