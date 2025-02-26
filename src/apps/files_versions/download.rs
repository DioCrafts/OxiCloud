//! ownCloud - Download versions directly from the versions drop-down
//!
//! @author Bjoern Schiessle
//! @copyright 2013 Bjoern Schiessle schiessle@owncloud.com
//!
//! This library is free software; you can redistribute it and/or
//! modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
//! License as published by the Free Software Foundation; either
//! version 3 of the License, or any later version.
//!
//! This library is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//!
//! You should have received a copy of the GNU Affero General Public
//! License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::io::{self, copy};
use std::fs::File;
use actix_web::{web, HttpResponse, Result, Error};
use actix_files::NamedFile;
use regex::Regex;
use mime_guess::from_path;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use crate::files_versions::storage;
use crate::core::json;
use crate::core::response;
use crate::core::util;
use crate::core::files::view::View;

/// Handler for version file downloads
pub async fn download(
    query: web::Query<DownloadQuery>,
    req: web::HttpRequest,
) -> Result<HttpResponse, Error> {
    // Check if the app is enabled
    json::check_app_enabled("files_versions")?;
    
    let file = &query.file;
    let revision = query.revision;
    
    // Get user ID and filename from the path
    let (uid, filename) = storage::get_uid_and_filename(file)?;
    
    // Construct the version file path
    let version_name = format!("/{}/files_versions/{}.v{}", uid, filename, revision);
    
    // Create the file view
    let view = View::new("/".to_string());
    
    // Get the file MIME type
    let file_path = format!("/{}/files/{}", uid, filename);
    let ftype = view.get_mime_type(&file_path)?;
    
    // Create and configure the response
    let mut builder = HttpResponse::Ok();
    builder.content_type(ftype);
    
    // Handle different user agents for Content-Disposition
    let base_filename = std::path::Path::new(&filename)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file");
    
    let encoded_filename = utf8_percent_encode(base_filename, NON_ALPHANUMERIC).to_string();
    
    let user_agent = req.headers().get("User-Agent")
        .and_then(|ua| ua.to_str().ok())
        .unwrap_or("");
    
    if Regex::new(r"MSIE").unwrap().is_match(user_agent) {
        builder.header("Content-Disposition", format!("attachment; filename=\"{}\"", encoded_filename));
    } else {
        builder.header(
            "Content-Disposition", 
            format!("attachment; filename*=UTF-8''{0}; filename=\"{0}\"", encoded_filename)
        );
    }
    
    // Disable caching
    response::disable_caching(&mut builder);
    
    // Set content length
    let file_size = view.file_size(&version_name)?;
    builder.header("Content-Length", file_size.to_string());
    
    // End output buffering (if any)
    util::ob_end()?;
    
    // Return the file
    let file_data = view.read_file(&version_name)?;
    Ok(builder.body(file_data))
}

#[derive(serde::Deserialize)]
pub struct DownloadQuery {
    file: String,
    revision: i32,
}