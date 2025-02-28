// ownCloud
//
// @author Frank Karlitschek
// @copyright 2012 Frank Karlitschek frank@owncloud.org
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

use actix_web::{web, App, HttpResponse, HttpServer, Result};
use thiserror::Error;

mod base;
use crate::base::{OC, OC_OCS, OC_Request, OC_Response};

#[derive(Error, Debug)]
enum RouterError {
    #[error("Resource not found")]
    ResourceNotFound,
    #[error("Method not allowed")]
    MethodNotAllowed,
}

async fn handle_request() -> Result<HttpResponse> {
    let raw_path = OC_Request::get_raw_path_info();
    let path = format!("/ocs{}", raw_path);
    
    match OC::get_router().match_path(&path) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => match err {
            RouterError::ResourceNotFound => {
                OC_OCS::not_found();
                Ok(HttpResponse::NotFound().finish())
            },
            RouterError::MethodNotAllowed => {
                OC_Response::set_status(405);
                Ok(HttpResponse::MethodNotAllowed().finish())
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/v1", web::get().to(handle_request))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}