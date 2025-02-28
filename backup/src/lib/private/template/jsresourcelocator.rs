// Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use crate::template::resource_locator::ResourceLocator;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JsError {
    #[error("js file not found: script:{0}")]
    JsFileNotFound(String),
}

pub struct JsResourceLocator {
    resource_locator: ResourceLocator,
}

impl JsResourceLocator {
    pub fn new(resource_locator: ResourceLocator) -> Self {
        Self { resource_locator }
    }

    pub fn do_find(&self, script: &str) -> Result<(), JsError> {
        let theme_dir = format!("themes/{}/", self.resource_locator.theme());
        
        if script.starts_with("3rdparty") && 
            self.resource_locator.append_if_exist(&self.resource_locator.thirdpartyroot(), &format!("{}.js", script)) ||
            self.resource_locator.append_if_exist(&self.resource_locator.serverroot(), &format!("{}apps/{}{}.js", theme_dir, script, self.resource_locator.form_factor())) ||
            self.resource_locator.append_if_exist(&self.resource_locator.serverroot(), &format!("{}apps/{}.js", theme_dir, script)) ||
            self.resource_locator.append_if_exist(&self.resource_locator.serverroot(), &format!("{}{}{}.js", theme_dir, script, self.resource_locator.form_factor())) ||
            self.resource_locator.append_if_exist(&self.resource_locator.serverroot(), &format!("{}{}.js", theme_dir, script)) ||
            self.resource_locator.append_if_exist(&self.resource_locator.serverroot(), &format!("{}{}.js", script, self.resource_locator.form_factor())) ||
            self.resource_locator.append_if_exist(&self.resource_locator.serverroot(), &format!("{}.js", script)) ||
            self.resource_locator.append_if_exist(&self.resource_locator.serverroot(), &format!("{}core/{}{}.js", theme_dir, script, self.resource_locator.form_factor())) ||
            self.resource_locator.append_if_exist(&self.resource_locator.serverroot(), &format!("{}core/{}.js", theme_dir, script)) ||
            self.resource_locator.append_if_exist(&self.resource_locator.serverroot(), &format!("core/{}{}.js", script, self.resource_locator.form_factor())) ||
            self.resource_locator.append_if_exist(&self.resource_locator.serverroot(), &format!("core/{}.js", script))
        {
            return Ok(());
        }

        if let Some(slash_pos) = script.find('/') {
            let app = &script[0..slash_pos];
            let script_part = &script[slash_pos+1..];
            
            let app_path = crate::app::get_app_path(app);
            let app_url = crate::app::get_app_web_path(app);
            
            if self.resource_locator.append_if_exist(&app_path, &format!("{}{}.js", script_part, self.resource_locator.form_factor()), &app_url) ||
               self.resource_locator.append_if_exist(&app_path, &format!("{}.js", script_part), &app_url)
            {
                return Ok(());
            }
        }

        Err(JsError::JsFileNotFound(script.to_string()))
    }

    pub fn do_find_theme(&self, _script: &str) {
        // Implementation is empty in the original
    }
}