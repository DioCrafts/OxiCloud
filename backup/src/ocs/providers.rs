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

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::error::Error;

mod lib {
    pub mod base;
    pub mod util;
}

use crate::lib::util::Util;

async fn providers() -> impl Responder {
    let server_protocol = match Util::get_server_protocol() {
        Ok(protocol) => protocol,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get server protocol"),
    };
    
    let server_host = match Util::get_server_host() {
        Ok(host) => host,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get server host"),
    };
    
    let request_uri = match Util::get_request_uri() {
        Ok(uri) => uri,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get request URI"),
    };
    
    // Construye la URL base recortando los últimos 17 caracteres del path combinado
    let full_path = format!("{}{}", server_host, request_uri);
    let trimmed_path = match full_path.char_indices().rev().nth(16) {
        Some((idx, _)) => &full_path[..idx + 1],
        None => &full_path,
    };
    
    let url = format!("{}://{}{}", server_protocol, trimmed_path, "ocs/v1.php/");
    
    let response = format!(r#"
<providers>
<provider>
 <id>ownCloud</id>
 <location>{}</location>
 <name>ownCloud</name>
 <icon></icon>
 <termsofuse></termsofuse>
 <register></register>
 <services>
   <config ocsversion="1.7" />
   <activity ocsversion="1.7" />
   <cloud ocsversion="1.7" />
 </services>
</provider>
</providers>
"#, url);

    HttpResponse::Ok()
        .content_type("application/xml")
        .body(response)
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Inicializa la base de datos o componentes necesarios
    lib::base::init()?;
    
    HttpServer::new(|| {
        App::new()
            .route("/ocs/providers.php", web::get().to(providers))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    
    Ok(())
}