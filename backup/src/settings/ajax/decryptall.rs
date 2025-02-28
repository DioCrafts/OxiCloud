use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use log::error;

// Importamos las aplicaciones necesarias
use crate::apps::files_encryption::{Util, FilesystemView};
use crate::auth::User;

#[derive(Deserialize)]
struct DecryptRequest {
    password: String,
}

#[derive(Serialize)]
struct JsonResponse {
    data: ResponseData,
}

#[derive(Serialize)]
struct ResponseData {
    message: String,
}

/// Controlador para descifrar todos los archivos
#[post("/settings/ajax/decryptall")]
async fn decrypt_all(
    req: web::Json<DecryptRequest>,
    user: web::Data<User>,
) -> impl Responder {
    // Carga la aplicación de cifrado
    // (esto sería manejado por el sistema de aplicaciones en Rust)

    // Inicializa la aplicación de cifrado
    let params = [
        ("uid", user.get_user_id()),
        ("password", &req.password),
    ];

    let view = FilesystemView::new("/");
    let util = Util::new(&view, user.get_user_id());

    // Intenta inicializar el cifrado
    match util.init_encryption(&params) {
        Ok(_) => {
            // Intenta descifrar todos los archivos
            match util.decrypt_all() {
                Ok(true) => {
                    let response = JsonResponse {
                        data: ResponseData {
                            message: String::from("Files decrypted successfully"),
                        },
                    };
                    HttpResponse::Ok().json(response)
                },
                _ => {
                    error!("Failed to decrypt files");
                    let response = JsonResponse {
                        data: ResponseData {
                            message: String::from("Couldn't decrypt your files, please check your owncloud.log or ask your administrator"),
                        },
                    };
                    HttpResponse::InternalServerError().json(response)
                }
            }
        },
        Err(_) => {
            let response = JsonResponse {
                data: ResponseData {
                    message: String::from("Couldn't decrypt your files, check your password and try again"),
                },
            };
            HttpResponse::BadRequest().json(response)
        }
    }
}