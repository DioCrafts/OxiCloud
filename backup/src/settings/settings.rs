// Copyright (c) 2011, Robin Appelman <icewind1991@gmail.com>
// This file is licensed under the Affero General Public License version 3 or later.
// See the COPYING-README file.

use crate::app::App;
use crate::template::Template;
use crate::util::Util;
use anyhow::Result;

pub async fn handle_settings() -> Result<()> {
    Util::check_logged_in()?;
    App::load_apps().await?;

    Util::add_style("settings", "settings")?;
    App::set_active_navigation_entry("settings")?;

    let mut tmpl = Template::new("settings", "settings", "user")?;
    let forms = App::get_forms("settings").await?;
    
    tmpl.assign("forms", Vec::new())?;
    for form in forms {
        tmpl.append("forms", form)?;
    }
    
    tmpl.print_page()?;
    
    Ok(())
}