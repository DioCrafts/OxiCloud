// Copyright (C) 2012 Frank Karlitschek <frank@owncloud.org>
// Copyright (C) 2012 Tom Needham <tom@owncloud.com>
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

use std::collections::HashMap;
use crate::ocs::result::OcsResult;
use crate::util;

/// Configuration for the OCS API
pub struct OcsConfig;

impl OcsConfig {
    /// Returns configuration information about the API
    pub fn api_config(_parameters: &HashMap<String, String>) -> OcsResult<HashMap<String, String>> {
        let mut xml = HashMap::new();
        xml.insert(String::from("version"), String::from("1.7"));
        xml.insert(String::from("website"), String::from("ownCloud"));
        xml.insert(String::from("host"), util::get_server_host());
        xml.insert(String::from("contact"), String::from(""));
        xml.insert(String::from("ssl"), String::from("false"));
        
        Ok(xml)
    }
}