//! File download handler
//!
//! Original author: Robin Appelman
//! Original copyright: 2010 Robin Appelman icewind1991@gmail.com
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

use actix_web::{error, get, web, HttpRequest, HttpResponse, Result};
use std::path::Path;
use regex::Regex;
use futures::StreamExt;
use serde::Deserialize;

use crate::auth::User;
use crate::filesystem::Filesystem;
use crate::templates::Template;
use crate::util::Util;
use crate::response::Response;

#[derive(Deserialize)]
struct DownloadParams {
    file: String,
}

#[get("/download")]
async fn download(
    req: HttpRequest,
    params: web::Query<DownloadParams>,
    user: User,
    fs: web::Data<Filesystem>,
) -> Result<HttpResponse, error::Error> {
    // Check if user is logged in
    if !user.is_logged_in() {
        return Err(error::ErrorUnauthorized("User not logged in"));
    }

    let filename = &params.file;

    if !fs.file_exists(filename).await {
        let mut tmpl = Template::new("", "404", "guest");
        tmpl.assign("file", filename);
        
        return Ok(HttpResponse::NotFound()
            .content_type("text/html")
            .body(tmpl.render()));
    }

    let ftype = fs.get_mime_type(filename).await?;
    
    let file_basename = Path::new(filename)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    
    let encoded_filename = urlencoding::encode(file_basename);
    
    let mut response_builder = HttpResponse::Ok();
    response_builder.content_type(ftype);
    
    // Handle disposition differently based on browser
    let user_agent = req.headers().get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    
    let is_msie = Regex::new(r"MSIE").unwrap().is_match(user_agent);
    
    if is_msie {
        response_builder.header(
            "Content-Disposition", 
            format!("attachment; filename=\"{}\"", encoded_filename)
        );
    } else {
        response_builder.header(
            "Content-Disposition", 
            format!("attachment; filename*=UTF-8''{0}; filename=\"{0}\"", encoded_filename)
        );
    }
    
    // Disable caching
    Response::disable_caching(&mut response_builder);
    
    // Get file size and set content length
    let file_size = fs.filesize(filename).await?;
    response_builder.header("Content-Length", file_size.to_string());
    
    // End output buffering if needed
    Util::ob_end();
    
    // Stream file content
    let stream = fs.read_file_stream(filename).await?;
    
    Ok(response_builder.streaming(stream))
}