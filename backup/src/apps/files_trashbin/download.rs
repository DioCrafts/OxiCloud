// Trashbin - Download file functionality
//
// @author Bjoern Schiessle
// @copyright 2013 Bjoern Schiessle schiessle@owncloud.com
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::io::Read;
use std::path::Path;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result, HttpRequest};
use actix_files::NamedFile;
use regex::Regex;
use urlencoding::encode;
use anyhow::{bail, Context};

use crate::auth::User;
use crate::filesystem::FilesystemView;
use crate::template::Template;
use crate::response::DisableCaching;

/// Download a file from the trashbin
///
/// This endpoint handles downloading files that have been moved to the trashbin
#[get("/apps/files_trashbin/download")]
async fn download_trashbin_file(
    req: HttpRequest,
    query: web::Query<TrashbinDownloadQuery>,
    user: User,
) -> Result<impl Responder> {
    // Check if user is logged in
    if !user.is_logged_in() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let filename = &query.file;
    
    // Create view path for the trashbin files
    let view_path = format!("/{}/files_trashbin/files", user.get_username());
    let view = FilesystemView::new(&view_path);

    // Check if file exists
    if !view.file_exists(filename).await {
        let mut tmpl = Template::new("", "404", "guest");
        tmpl.assign("file", filename);
        return Ok(HttpResponse::NotFound().body(tmpl.render()));
    }

    // Get file metadata
    let ftype = view.get_mime_type(filename).await
        .context("Failed to get mime type")?;
    let filesize = view.filesize(filename).await
        .context("Failed to get file size")?;
    
    // Create response with appropriate headers
    let mut response_builder = HttpResponse::Ok();
    response_builder.content_type(ftype);
    
    // Set content disposition based on browser
    let user_agent = req.headers().get("User-Agent")
        .map(|h| h.to_str().unwrap_or(""))
        .unwrap_or("");
    
    let filename_encoded = encode(&Path::new(filename).file_name()
        .unwrap_or_default()
        .to_string_lossy());
    
    if Regex::new(r"MSIE").unwrap().is_match(user_agent) {
        response_builder.append_header(("Content-Disposition", 
            format!("attachment; filename=\"{}\"", filename_encoded)));
    } else {
        response_builder.append_header(("Content-Disposition", 
            format!("attachment; filename*=UTF-8''{0}; filename=\"{0}\"", filename_encoded)));
    }
    
    // Disable caching
    response_builder.disable_caching();
    response_builder.append_header(("Content-Length", filesize.to_string()));
    
    // Stream file content
    let file_content = view.read_file(filename).await
        .context("Failed to read file")?;
    
    Ok(response_builder.body(file_content))
}

#[derive(serde::Deserialize)]
struct TrashbinDownloadQuery {
    file: String,
}

pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg.service(download_trashbin_file);
}