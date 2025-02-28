/**
 * Copyright (c) 2013 Tom Needham <tom@owncloud.com>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use serde_json::{json, Value};

/// Capabilities module for files versions functionalities
pub struct Capabilities;

impl Capabilities {
    /// Returns capabilities related to file versioning
    pub fn get_capabilities() -> Result<Value, Box<dyn std::error::Error>> {
        Ok(json!({
            "capabilities": {
                "files": {
                    "versioning": true
                }
            }
        }))
    }
}