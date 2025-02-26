use actix_web::{web, HttpResponse, Result};
use log::error;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Deserialize)]
struct UndeleteRequest {
    files: String,
    dirlisting: String,
}

#[derive(Serialize)]
struct SuccessItem {
    filename: String,
    timestamp: Option<String>,
}

#[derive(Serialize)]
struct SuccessResponse {
    data: SuccessData,
}

#[derive(Serialize)]
struct SuccessData {
    success: Vec<SuccessItem>,
}

#[derive(Serialize)]
struct ErrorResponse {
    data: ErrorData,
}

#[derive(Serialize)]
struct ErrorData {
    message: String,
    success: Vec<SuccessItem>,
    error: Vec<String>,
}

async fn undelete(
    req: web::Json<UndeleteRequest>,
    identity: web::ReqData<identity::Identity>,
    trashbin: web::Data<dyn TrashbinService>,
    l10n: web::Data<dyn L10nService>,
) -> Result<HttpResponse> {
    // Check if user is logged in
    if identity.identity().is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    // Check CSRF token (assuming it's handled by a middleware)

    let files = &req.files;
    let dirlisting = &req.dirlisting;
    let list: Vec<String> = serde_json::from_str(files).map_err(|_| {
        actix_web::error::ErrorBadRequest("Invalid JSON in files parameter")
    })?;

    let mut error = Vec::new();
    let mut success = Vec::new();

    for file in list {
        let (filename, timestamp) = if dirlisting == "0" {
            let delimiter = file.rfind(".d").unwrap_or(file.len());
            let filename = file[..delimiter].to_string();
            let timestamp = if delimiter < file.len() {
                Some(file[delimiter + 2..].to_string())
            } else {
                None
            };
            (filename, timestamp)
        } else {
            let path = Path::new(&file);
            let filename = path
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or(&file)
                .to_string();
            (filename, None)
        };

        if !trashbin.restore(&file, &filename, timestamp.as_deref()).await {
            error.push(filename.clone());
            error!("Can't restore {}", filename);
        } else {
            success.push(SuccessItem {
                filename: file.clone(),
                timestamp,
            });
        }
    }

    if !error.is_empty() {
        let filelist = error.join(", ");
        let message = l10n.translate("files_trashbin", "Couldn't restore %s", &[&filelist]);
        
        Ok(HttpResponse::BadRequest().json(ErrorResponse {
            data: ErrorData {
                message,
                success,
                error,
            },
        }))
    } else {
        Ok(HttpResponse::Ok().json(SuccessResponse {
            data: SuccessData { success },
        }))
    }
}

// These trait definitions would be in separate files
#[async_trait::async_trait]
trait TrashbinService: Send + Sync {
    async fn restore(&self, file: &str, filename: &str, timestamp: Option<&str>) -> bool;
}

trait L10nService: Send + Sync {
    fn translate(&self, app: &str, text: &str, params: &[&str]) -> String;
}