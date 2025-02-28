// ownCloud
//
// @author Frank Karlitschek
// @copyright 2010 Frank Karlitschek karlitschek@kde.org
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

mod lib;
mod ocp;
mod oc;

use crate::oc::{Response, Template};
use crate::ocp::Util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // No apps, yet
    let runtime_noapps = true;
    
    match run_application(runtime_noapps).await {
        Ok(_) => Ok(()),
        Err(e) => {
            Util::log_exception("index", &e);
            
            // Show the user a detailed error page
            Response::set_status(Response::STATUS_INTERNAL_SERVER_ERROR);
            Template::print_exception_error_page(&e);
            Err(e)
        }
    }
}

async fn run_application(runtime_noapps: bool) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the application
    lib::base::initialize(runtime_noapps)?;
    
    // Handle the request
    oc::OC::handle_request().await?;
    
    Ok(())
}