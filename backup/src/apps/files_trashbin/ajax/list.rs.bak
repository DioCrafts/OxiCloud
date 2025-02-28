use actix_web::{web, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use std::path::Path;

// Import your application-specific modules
use crate::files_trashbin::helper;
use crate::auth::middleware::ensure_logged_in;
use crate::util;

#[derive(Deserialize)]
pub struct ListParams {
    dir: Option<String>,
    breadcrumb: Option<bool>,
}

#[derive(Serialize)]
struct ListResponse {
    data: ListData,
}

#[derive(Serialize, Default)]
struct ListData {
    #[serde(skip_serializing_if = "Option::is_none")]
    breadcrumb: Option<String>,
    files: String,
}

// Main handler function
pub async fn list(
    params: web::Query<ListParams>,
    tera: web::Data<Tera>,
    _: ensure_logged_in::EnsureLoggedIn,
) -> Result<impl Responder> {
    // only need filesystem apps
    // RUNTIME_APPTYPES is handled through middleware in Rust

    // Load the files
    let dir = params.dir.clone().unwrap_or_else(|| String::from(""));
    let do_breadcrumb = params.breadcrumb.unwrap_or(false);
    let mut data = ListData::default();

    // Make breadcrumb
    if do_breadcrumb {
        let breadcrumb = helper::make_breadcrumb(&dir)?;

        let mut context = Context::new();
        context.insert("breadcrumb", &breadcrumb);
        context.insert("baseURL", &format!("{}?dir=", util::link_to("files_trashbin", "index.php")));
        context.insert("home", &util::link_to("files", "index.php"));

        data.breadcrumb = Some(tera.render("files_trashbin/part.breadcrumb", &context)?);
    }

    // make filelist
    let files = helper::get_trash_files(&dir)
        .ok_or_else(|| actix_web::error::ErrorNotFound("Files not found"))?;

    let dirlisting = !dir.is_empty() && dir != "/";

    let encoded_dir = util::encode_path(&dir);
    let base_url = format!("{}?dir={}", util::link_to("files_trashbin", "index.php"), encoded_dir);
    
    let mut context = Context::new();
    context.insert("files", &files);
    context.insert("baseURL", &base_url);
    context.insert("downloadURL", &util::link_to_route("download", &[("file", "/")]));
    context.insert("dirlisting", &dirlisting);
    context.insert("disableDownloadActions", &true);
    
    data.files = tera.render("files_trashbin/part.list", &context)?;

    Ok(HttpResponse::Ok().json(ListResponse { data }))
}