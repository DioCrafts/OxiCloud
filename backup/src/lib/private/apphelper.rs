// Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use crate::OCP::IHelper;
use crate::OC_Util;

/// TODO: Description
pub struct AppHelper;

impl IHelper for AppHelper {
    /// Gets the content of an URL by using CURL or a fallback if it is not
    /// installed
    /// 
    /// # Arguments
    /// 
    /// * `url` - the url that should be fetched
    /// 
    /// # Returns
    /// 
    /// The content of the webpage as a Result
    fn get_url_content(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        OC_Util::get_url_content(url)
    }
}

impl AppHelper {
    pub fn new() -> Self {
        AppHelper
    }
}