//! Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.

use anyhow::Result;
use clap::{App, ArgMatches};
use std::collections::HashMap;

pub struct Status;

impl Status {
    pub fn register<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.subcommand(
            App::new("status")
                .about("show some status information")
        )
    }

    pub async fn run(_matches: &ArgMatches<'_>) -> Result<()> {
        let config = crate::config::get_config()?;
        let installed = config.get_bool("installed").unwrap_or(false);
        
        let version = crate::util::get_version();
        let version_string = crate::util::get_version_string()?;
        let edition_string = crate::util::get_edition_string()?;
        
        let values: HashMap<&str, String> = [
            ("installed", if installed { "true".to_string() } else { "false".to_string() }),
            ("version", version.join(".")),
            ("versionstring", version_string),
            ("edition", edition_string),
        ].iter().cloned().collect();
        
        println!("{:#?}", values);
        
        Ok(())
    }
}