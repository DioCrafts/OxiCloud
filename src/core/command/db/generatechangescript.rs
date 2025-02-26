// Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches};
use std::path::PathBuf;

use crate::db::connection::Connection;
use crate::db::schema_manager::MDB2SchemaManager;

pub struct GenerateChangeScript;

impl GenerateChangeScript {
    pub fn register<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.subcommand(
            App::new("db:generate-change-script")
                .about("generates the change script from the current connected db to db_structure.xml")
                .arg(
                    Arg::with_name("schema-xml")
                        .help("the schema xml to be used as target schema")
                        .takes_value(true)
                        .required(false),
                ),
        )
    }

    pub async fn run(args: &ArgMatches<'_>, connection: Connection) -> Result<()> {
        let server_root = std::env::var("SERVER_ROOT")
            .context("SERVER_ROOT environment variable not set")?;
        
        let default_schema_path = PathBuf::from(server_root).join("db_structure.xml");
        let default_schema = default_schema_path.to_string_lossy();
        
        let schema_file = args
            .value_of("schema-xml")
            .unwrap_or(&default_schema);

        let schema_manager = MDB2SchemaManager::new(connection);
        
        match schema_manager.update_db_from_structure(schema_file, true).await {
            Ok(result) => {
                println!("{}", result);
                Ok(())
            },
            Err(e) => {
                println!("Failed to update database structure ({})", e);
                Err(e.into())
            },
        }
    }
}