//! # File Rename Module
//!
//! This module handles file rename operations for the Files app.
//!
//! ## Original Author
//! Morris Jobke
//!
//! ## Copyright
//! 2013 Morris Jobke morris.jobke@gmail.com
//!
//! This library is free software; you can redistribute it and/or
//! modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
//! License as published by the Free Software Foundation; either
//! version 3 of the License, or any later version.
//!
//! This library is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//! GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//!
//! You should have received a copy of the GNU Affero General Public
//! License along with this library. If not, see <http://www.gnu.org/licenses/>.

use crate::files::app::App;
use crate::filesystem::Filesystem;
use crate::json::{self, JsonResponse};
use crate::l10n::L10n;
use crate::session::Session;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RenameRequest {
    dir: String,
    file: String,
    newname: String,
}

#[derive(Serialize)]
pub struct RenameResponseData {
    #[serde(flatten)]
    data: serde_json::Value,
}

/// Handle the file rename action
///
/// This endpoint checks for user authentication, validates the request,
/// and performs the rename operation on the specified file.
pub async fn rename(
    req: HttpRequest, 
    query: web::Query<RenameRequest>,
    session: web::Data<Session>,
) -> Result<HttpResponse> {
    // Check if user is logged in
    if !session.check_logged_in(&req) {
        return Ok(json::error("Not logged in").into());
    }

    // Validate CSRF token
    if !session.check_call(&req) {
        return Ok(json::error("Invalid CSRF token").into());
    }

    // Initialize filesystem and app
    let view = Filesystem::get_view();
    let l10n = L10n::get("files");
    let files = App::new(view, l10n);

    // Perform the rename operation
    let result = files.rename(
        &query.dir,
        &query.file,
        &query.newname,
    ).await;

    // Return appropriate response based on operation success
    match result {
        Ok(data) => Ok(json::success(RenameResponseData { data }).into()),
        Err(data) => Ok(json::error(RenameResponseData { data }).into()),
    }
}