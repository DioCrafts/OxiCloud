// Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::path::Path;

use anyhow::{anyhow, Result};

use crate::template::resource_locator::ResourceLocator;

pub struct CssResourceLocator {
    inner: ResourceLocator,
}

impl CssResourceLocator {
    pub fn new(resource_locator: ResourceLocator) -> Self {
        Self {
            inner: resource_locator,
        }
    }

    pub fn do_find(&self, style: &str) -> Result<()> {
        if style.starts_with("3rdparty")
            && self.inner.append_if_exist(&self.inner.thirdpartyroot, &format!("{}.css", style))?
            || self
                .inner
                .append_if_exist(&self.inner.serverroot, &format!("{}{}.css", style, self.inner.form_factor))?
            || self.inner.append_if_exist(&self.inner.serverroot, &format!("{}.css", style))?
            || self
                .inner
                .append_if_exist(&self.inner.serverroot, &format!("core/{}{}.css", style, self.inner.form_factor))?
            || self
                .inner
                .append_if_exist(&self.inner.serverroot, &format!("core/{}.css", style))?
        {
            return Ok(());
        }

        let app_end = match style.find('/') {
            Some(pos) => pos,
            None => return Err(anyhow!("Invalid style format: {}", style)),
        };

        let app = &style[..app_end];
        let style = &style[app_end + 1..];
        
        let app_path = crate::app::get_app_path(app)?;
        let app_url = format!("{}/index.php/apps/{}", self.inner.webroot, app);
        
        if self
            .inner
            .append_if_exist(&app_path, &format!("{}{}.css", style, self.inner.form_factor), Some(&app_url))?
            || self
                .inner
                .append_if_exist(&app_path, &format!("{}.css", style), Some(&app_url))?
        {
            return Ok(());
        }

        Err(anyhow!("css file not found: style:{}", style))
    }

    pub fn do_find_theme(&self, style: &str) -> Result<bool> {
        let theme_dir = format!("themes/{}/", self.inner.theme);
        
        Ok(self
            .inner
            .append_if_exist(
                &self.inner.serverroot,
                &format!("{}apps/{}{}.css", theme_dir, style, self.inner.form_factor),
            )?
            || self
                .inner
                .append_if_exist(&self.inner.serverroot, &format!("{}apps/{}.css", theme_dir, style))?
            || self
                .inner
                .append_if_exist(
                    &self.inner.serverroot,
                    &format!("{}{}{}.css", theme_dir, style, self.inner.form_factor),
                )?
            || self
                .inner
                .append_if_exist(&self.inner.serverroot, &format!("{}{}.css", theme_dir, style))?
            || self
                .inner
                .append_if_exist(
                    &self.inner.serverroot,
                    &format!("{}core/{}{}.css", theme_dir, style, self.inner.form_factor),
                )?
            || self
                .inner
                .append_if_exist(&self.inner.serverroot, &format!("{}core/{}.css", theme_dir, style))?)
    }
}