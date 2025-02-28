use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::cmp;

// Importamos módulos equivalentes a OCP y OCA
use crate::apps::files_versions::storage::Storage;
use crate::core::app::check_app_enabled;
use crate::core::error::{Error, Result};

#[derive(Deserialize)]
struct GetVersionsParams {
    source: String,
    start: usize,
}

#[derive(Serialize)]
struct VersionsResponse {
    data: VersionsData,
}

#[derive(Serialize)]
struct VersionsData {
    versions: Option<Vec<Version>>,
    end_reached: bool,
}

// Asumimos que Version es un tipo definido en otro lugar que corresponde 
// a la estructura de versiones en PHP
use crate::apps::files_versions::models::Version;

pub async fn get_versions(query: web::Query<GetVersionsParams>) -> impl Responder {
    // Verificar si la app está habilitada
    match check_app_enabled("files_versions") {
        Err(_) => return HttpResponse::NotFound().finish(),
        _ => {}
    }

    let source = &query.source;
    let start = query.start;
    
    // Obtener el uid y filename del source
    let (uid, filename) = match Storage::get_uid_and_filename(source) {
        Ok((uid, filename)) => (uid, filename),
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    // Mostrar las revisiones más recientes
    let count = 5;
    
    // Obtener versiones
    match Storage::get_versions(&uid, &filename).await {
        Ok(versions) => {
            let total_versions = versions.len();
            let end_reached = total_versions <= start + count;
            
            // Slice de versiones (similar a array_slice en PHP)
            let versions_slice = versions
                .into_iter()
                .skip(start)
                .take(count)
                .collect::<Vec<_>>();
            
            let response = VersionsResponse {
                data: VersionsData {
                    versions: Some(versions_slice),
                    end_reached,
                },
            };
            
            HttpResponse::Ok().json(response)
        },
        Err(_) => {
            let response = VersionsResponse {
                data: VersionsData {
                    versions: None,
                    end_reached: true,
                },
            };
            
            HttpResponse::Ok().json(response)
        }
    }
}

// Registración de la ruta en Actix
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/ajax/getVersions")
            .route(web::get().to(get_versions))
    );
}