/**
 * Copyright (c) 2013 Tom Needham <tom@owncloud.com>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub struct Capabilities;

#[derive(Debug, Serialize, Deserialize)]
pub struct OcsResult<T> {
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CapabilitiesResponse {
    pub capabilities: HashMap<String, HashMap<String, bool>>,
}

impl Capabilities {
    pub fn get_capabilities() -> OcsResult<CapabilitiesResponse> {
        let mut files_capabilities = HashMap::new();
        files_capabilities.insert("bigfilechunking".to_string(), true);
        files_capabilities.insert("undelete".to_string(), true);

        let mut capabilities = HashMap::new();
        capabilities.insert("files".to_string(), files_capabilities);

        OcsResult {
            data: CapabilitiesResponse {
                capabilities,
            }
        }
    }
}