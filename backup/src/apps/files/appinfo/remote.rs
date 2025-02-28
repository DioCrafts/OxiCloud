/*
 * ownCloud
 *
 * @author Frank Karlitschek
 * @author Jakob Sack
 * @copyright 2012 Frank Karlitschek frank@owncloud.org
 * @copyright 2011 Jakob Sack kde@jakobsack.de
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU AFFERO GENERAL PUBLIC LICENSE for more details.
 *
 * You should have received a copy of the GNU Affero General Public
 * License along with this library.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

use std::error::Error;
use futures::executor::block_on;

mod oc {
    pub mod app;
    pub mod util;
    pub mod defaults;
    pub mod connector {
        pub mod sabre {
            pub mod auth;
            pub mod locks;
            pub mod request;
            pub mod directory;
            pub mod server;
            pub mod files_plugin;
            pub mod aborted_upload_detection_plugin;
            pub mod quota_plugin;
            pub mod maintenance_plugin;

            pub mod object_tree;
        }
    }
}

use oc::app::App;
use oc::util::Util;
use oc::defaults::Defaults;
use oc::connector::sabre::{
    auth::Auth, 
    locks::Locks, 
    request::Request, 
    directory::Directory, 
    server::Server, 
    files_plugin::FilesPlugin,
    aborted_upload_detection_plugin::AbortedUploadDetectionPlugin,
    quota_plugin::QuotaPlugin,
    maintenance_plugin::MaintenancePlugin
};
use oc::connector::sabre::object_tree::ObjectTree;

// External dependencies
mod sabre {
    pub mod dav {
        pub mod auth {
            pub struct Plugin {
                pub auth_backend: super::super::super::oc::connector::sabre::auth::Auth,
                pub realm: String,
            }
            
            impl Plugin {
                pub fn new(
                    auth_backend: super::super::super::oc::connector::sabre::auth::Auth, 
                    realm: String
                ) -> Self {
                    Self { auth_backend, realm }
                }
            }
        }
        
        pub mod locks {
            pub struct Plugin {
                pub locks_backend: super::super::super::oc::connector::sabre::locks::Locks,
            }
            
            impl Plugin {
                pub fn new(locks_backend: super::super::super::oc::connector::sabre::locks::Locks) -> Self {
                    Self { locks_backend }
                }
            }
        }
        
        pub mod browser {
            pub struct Plugin {
                pub allow_upload: bool,
            }
            
            impl Plugin {
                pub fn new(allow_upload: bool) -> Self {
                    Self { allow_upload }
                }
            }
        }
    }
}

async fn run_server() -> Result<(), Box<dyn Error>> {
    // Load needed apps
    let runtime_app_types = vec!["filesystem", "authentication", "logging"];
    App::load_apps(&runtime_app_types)?;

    Util::ob_end()?;

    // Backends
    let auth_backend = Auth::new();
    let lock_backend = Locks::new();
    let request_backend = Request::new();

    // Create ownCloud Dir
    let root_dir = Directory::new(String::from(""));
    let object_tree = ObjectTree::new(root_dir);

    // Fire up server
    let mut server = Server::new(object_tree);
    server.http_request = request_backend;
    server.set_base_uri(&get_base_uri()?)?;

    // Load plugins
    let defaults = Defaults::new();
    server.add_plugin(Box::new(sabre::dav::auth::Plugin::new(
        auth_backend,
        defaults.get_name(),
    )))?;
    
    server.add_plugin(Box::new(sabre::dav::locks::Plugin::new(lock_backend)))?;
    
    // Show something in the Browser, but no upload
    server.add_plugin(Box::new(sabre::dav::browser::Plugin::new(false)))?;
    
    server.add_plugin(Box::new(FilesPlugin::new()))?;
    server.add_plugin(Box::new(AbortedUploadDetectionPlugin::new()))?;
    server.add_plugin(Box::new(QuotaPlugin::new()))?;
    server.add_plugin(Box::new(MaintenancePlugin::new()))?;

    // And off we go!
    server.exec().await?;

    Ok(())
}

fn get_base_uri() -> Result<String, Box<dyn Error>> {
    // Implementation would depend on how baseuri was determined in the original code
    Ok(String::from("/remote.php/webdav"))
}

fn main() -> Result<(), Box<dyn Error>> {
    block_on(run_server())
}