use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use log::error;
use std::sync::Arc;

use crate::auth;
use crate::nextcloud::{
    files_trashbin::Trashbin,
    json_response::{error_response, success_response},
    l10n::L10n,
};

#[derive(Deserialize)]
pub struct DeleteRequest {
    files: String,
    dirlisting: String,
}

#[derive(Serialize)]
struct SuccessItem {
    filename: String,
    timestamp: Option<String>,
}

pub async fn delete(
    req: web::Json<DeleteRequest>,
    session: web::ReqData<Arc<auth::Session>>,
    l10n: web::Data<L10n>,
    trashbin: web::Data<Trashbin>,
) -> impl Responder {
    // Verificar sesión
    if !session.is_logged_in() {
        return HttpResponse::Unauthorized().finish();
    }

    // Verificar token CSRF
    if !auth::check_request_token() {
        return HttpResponse::Forbidden().finish();
    }

    let files = &req.files;
    let dirlisting = &req.dirlisting;
    let list: Vec<String> = serde_json::from_str(files).unwrap_or_default();

    let mut error = Vec::new();
    let mut success = Vec::new();

    for file in list {
        let (filename, timestamp) = if dirlisting == "0" {
            if let Some(delimiter) = file.rfind(".d") {
                let filename = file[..delimiter].to_string();
                let timestamp = file[delimiter + 2..].to_string();
                (filename, Some(timestamp))
            } else {
                // En caso de que el formato no sea el esperado
                (file, None)
            }
        } else {
            (file.clone(), None)
        };

        trashbin.delete(&filename, timestamp.as_deref()).await;
        
        if !trashbin.file_exists(&filename, timestamp.as_deref()).await {
            success.push(SuccessItem {
                filename: file,
                timestamp,
            });
        } else {
            error.push(filename.clone());
            error!("can't delete {} permanently.", filename);
        }
    }

    if !error.is_empty() {
        let filelist = error.join(", ");
        let message = l10n.t("Couldn't delete %s permanently", &[&filelist]);
        
        error_response(serde_json::json!({
            "message": message,
            "success": success,
            "error": error
        }))
    } else {
        success_response(serde_json::json!({
            "success": success
        }))
    }
}