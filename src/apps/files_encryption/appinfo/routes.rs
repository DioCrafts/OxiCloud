// Copyright (c) 2013, Tom Needham <tom@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or later.
// See the COPYING-README file.

use rocket::{get, routes};
use rocket_contrib::json::Json;
use serde_json::{json, Value};
use std::sync::Arc;

use crate::capabilities::EncryptionCapabilities;
use crate::auth::UserAuth;

/// Module for encryption capabilities
pub mod capabilities {
    use serde::{Serialize, Deserialize};
    use serde_json::{json, Value};

    #[derive(Debug, Clone)]
    pub struct EncryptionCapabilities;

    impl EncryptionCapabilities {
        pub fn get_capabilities(&self) -> Value {
            json!({
                "encryption": {
                    // Información específica de capacidades de cifrado
                    "enabled": true,
                    // Añadir otros campos según sea necesario
                }
            })
        }
    }
}

/// Initialize routes for the encryption module
pub fn configure_routes(user_auth: Arc<UserAuth>) -> Vec<rocket::Route> {
    routes![get_capabilities]
}

/// Register API endpoint to retrieve encryption capabilities
#[get("/cloud/capabilities")]
fn get_capabilities(user_auth: rocket::State<Arc<UserAuth>>) -> Result<Json<Value>, rocket::http::Status> {
    // Verificar autenticación del usuario
    if !user_auth.is_authenticated() {
        return Err(rocket::http::Status::Unauthorized);
    }

    // Obtener capacidades de cifrado
    let encryption_capabilities = EncryptionCapabilities;
    let capabilities = encryption_capabilities.get_capabilities();
    
    Ok(Json(capabilities))
}