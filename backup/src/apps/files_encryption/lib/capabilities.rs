/**
 * Copyright (c) 2013 Tom Needham <tom@owncloud.com>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use serde_json::json;
use serde_json::Value;

pub struct Capabilities;

impl Capabilities {
    pub fn get_capabilities() -> Result<Value, Box<dyn std::error::Error>> {
        Ok(json!({
            "capabilities": {
                "files": {
                    "encryption": true,
                }
            }
        }))
    }
}