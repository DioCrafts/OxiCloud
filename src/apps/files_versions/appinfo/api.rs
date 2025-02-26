//! ownCloud
//!
//! @author Michael Gapczynski
//! @copyright 2012 Michael Gapczynski mtgap@owncloud.com
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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use actix_web::http::Method;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDefinition {
    pub required: bool,
    #[serde(rename = "type")]
    pub param_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointDefinition {
    pub method: String,
    pub class: String,
    pub function: String,
    pub parameters: HashMap<String, ParameterDefinition>,
}

pub fn get_api_definition() -> HashMap<String, EndpointDefinition> {
    let mut api = HashMap::new();
    
    // List endpoint
    let mut list_params = HashMap::new();
    list_params.insert(
        "file".to_string(), 
        ParameterDefinition {
            required: true,
            param_type: "string".to_string(),
        }
    );
    
    api.insert(
        "list".to_string(),
        EndpointDefinition {
            method: Method::GET.to_string(),
            class: "Storage".to_string(),
            function: "get_versions".to_string(),
            parameters: list_params,
        }
    );
    
    // Revert endpoint
    let mut revert_params = HashMap::new();
    revert_params.insert(
        "file".to_string(), 
        ParameterDefinition {
            required: true,
            param_type: "string".to_string(),
        }
    );
    revert_params.insert(
        "time".to_string(), 
        ParameterDefinition {
            required: true,
            param_type: "int".to_string(),
        }
    );
    
    api.insert(
        "revert".to_string(),
        EndpointDefinition {
            method: Method::POST.to_string(),
            class: "Storage".to_string(),
            function: "rollback".to_string(),
            parameters: revert_params,
        }
    );
    
    api
}