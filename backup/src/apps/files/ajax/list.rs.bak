use actix_web::{
    http::StatusCode,
    web::{self, Query},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

// Import necessary modules from the application
use crate::files::{
    helper::{self, FileInfo},
    templates::Template,
};
use crate::auth;
use crate::filesystem;
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

#[derive(Serialize)]
struct ListData {
    #[serde(skip_serializing_if = "Option::is_none")]
    breadcrumb: Option<String>,
    files: String,
    permissions: u32,
}

/// List files in a directory
///
/// This endpoint returns the files in a specific directory and 
/// optionally includes a breadcrumb navigation component.
pub async fn list(
    query: Query<ListParams>,
    session: web::Data<auth::Session>,
) -> impl Responder {
    // only need filesystem apps
    // RUNTIME_APPTYPES is handled by middleware in Rust implementation

    // Check if user is logged in
    if !session.is_logged_in() {
        return HttpResponse::Unauthorized().finish();
    }

    // Load the files
    let dir = query.dir.clone().unwrap_or_else(|| String::from(""));

    // Check if directory exists
    if !filesystem::is_dir(&format!("{}/", dir)) {
        return HttpResponse::NotFound().finish();
    }

    let base_url = util::link_to("files", "index.php")? + "?dir=";
    
    // Get directory permissions
    let permissions = helper::get_dir_permissions(&dir)?;
    
    let mut data = ListData {
        breadcrumb: None,
        files: String::new(),
        permissions,
    };

    // Make breadcrumb if requested
    if query.breadcrumb.unwrap_or(false) {
        let breadcrumb = helper::make_breadcrumb(&dir)?;
        
        let mut breadcrumb_nav = Template::new("files", "part.breadcrumb")?;
        breadcrumb_nav.assign("breadcrumb", &breadcrumb)?;
        breadcrumb_nav.assign("baseURL", &base_url)?;
        
        data.breadcrumb = Some(breadcrumb_nav.fetch_page()?);
    }

    // Make filelist
    let files = helper::get_files(&dir)?;
    
    let mut list = Template::new("files", "part.list")?;
    list.assign("files", &files)?;
    list.assign("baseURL", &base_url)?;
    
    let download_url = util::link_to_route("download", &[("file", "/")])?;
    list.assign("downloadURL", &download_url)?;
    list.assign("isPublic", &false)?;
    
    data.files = list.fetch_page()?;
    
    // Return success response with data
    HttpResponse::Ok().json(json!({
        "status": "success",
        "data": data
    }))
}