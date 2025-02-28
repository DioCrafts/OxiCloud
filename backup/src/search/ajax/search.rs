// ownCloud
//
// @author Robin Appelman
// @copyright 2010 Robin Appelman icewind1991@gmail.com
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

use actix_web::{web, HttpResponse, Result};
use serde::Serialize;
use serde_json::json;

use crate::auth::check_logged_in;
use crate::services::{app_service, search_service};

#[derive(Serialize)]
struct SearchResponse {
    results: Vec<SearchResult>,
}

#[derive(Serialize)]
struct SearchResult {
    // Define the search result structure as needed
    // This should match the structure returned by search_service
    name: String,
    link: String,
    type_name: String,
}

/// Handler for search endpoint
///
/// This endpoint processes search queries and returns matching results
pub async fn handle_search(
    query: web::Query<SearchQuery>,
    session: web::Data<crate::session::SessionManager>,
) -> Result<HttpResponse> {
    // Check if user is logged in
    check_logged_in(&session)?;
    
    // Load all apps to ensure search providers are available
    app_service::load_apps().await?;
    
    // Check if query parameter exists and is not empty
    if let Some(q) = &query.query {
        if !q.is_empty() {
            let search_results = search_service::search(q).await?;
            Ok(HttpResponse::Ok().json(search_results))
        } else {
            Ok(HttpResponse::Ok().body("false"))
        }
    } else {
        Ok(HttpResponse::Ok().body("false"))
    }
}

#[derive(Serialize, Deserialize)]
pub struct SearchQuery {
    query: Option<String>,
}

// Configuration function to register the search handler
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/search/ajax/search.php")
            .route(web::get().to(handle_search))
    );
}