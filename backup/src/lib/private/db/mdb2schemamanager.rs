// Copyright (c) 2012 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use doctrine_dbal::{
    Connection, DatabasePlatform, MigrateToSql, Query, Schema, SchemaComparator, SchemaManager,
    SchemaDiff, Sequence, Table, TableDiff,
};
use oc_config::Config;

use crate::db::{mdb2_schema_reader::MDB2SchemaReader, mdb2_schema_writer};

pub const MDB2_SCHEMA_DUMP_STRUCTURE: i32 = 1;

/// Manager for handling database schema changes
pub struct MDB2SchemaManager {
    conn: Arc<Connection>,
}

impl MDB2SchemaManager {
    /// Create a new schema manager
    pub fn new(conn: Arc<Connection>) -> Self {
        Self { conn }
    }

    /// Saves database scheme to xml file
    ///
    /// # Arguments
    /// * `file` - name of file
    /// * `mode` - dump mode
    ///
    /// # Returns
    /// Result indicating success or failure
    pub async fn get_db_structure<P: AsRef<Path>>(
        &self,
        file: P,
        mode: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sm = self.conn.get_schema_manager().await?;
        mdb2_schema_writer::save_schema_to_file(file, &sm).await
    }

    /// Creates tables from XML file
    ///
    /// # Arguments
    /// * `file` - file to read structure from
    ///
    /// # Returns
    /// Result indicating success or failure
    pub async fn create_db_from_structure<P: AsRef<Path>>(
        &self,
        file: P,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let schema_reader = MDB2SchemaReader::new(
            Config::get_object(),
            self.conn.get_database_platform().await?,
        );
        let to_schema = schema_reader.load_schema_from_file(file).await?;
        self.execute_schema_change(to_schema).await
    }

    /// Update the database scheme
    ///
    /// # Arguments
    /// * `file` - file to read structure from
    /// * `generate_sql` - whether to generate SQL instead of executing it
    ///
    /// # Returns
    /// Result containing generated SQL or success indicator
    pub async fn update_db_from_structure<P: AsRef<Path>>(
        &self,
        file: P,
        generate_sql: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sm = self.conn.get_schema_manager().await?;
        let from_schema = sm.create_schema().await?;

        let schema_reader = MDB2SchemaReader::new(
            Config::get_object(),
            self.conn.get_database_platform().await?,
        );
        let to_schema = schema_reader.load_schema_from_file(file).await?;

        // Remove tables we don't know about
        let mut from_schema_clone = from_schema.clone();
        for table in from_schema.get_tables() {
            if !to_schema.has_table(table.get_name()) {
                from_schema_clone.drop_table(table.get_name());
            }
        }

        // Remove sequences we don't know about
        for sequence in from_schema.get_sequences() {
            if !to_schema.has_sequence(sequence.get_name()) {
                from_schema_clone.drop_sequence(sequence.get_name());
            }
        }

        let comparator = SchemaComparator::new();
        let mut schema_diff = comparator.compare(&from_schema_clone, &to_schema);

        let platform = self.conn.get_database_platform().await?;
        for table_diff in &mut schema_diff.changed_tables {
            table_diff.name = platform.quote_identifier(&table_diff.name);
        }

        if generate_sql {
            Ok(self.generate_change_script(&schema_diff).await?)
        } else {
            self.execute_schema_change(schema_diff).await?;
            Ok(String::new())
        }
    }

    /// Drop a table
    ///
    /// # Arguments
    /// * `table_name` - the table to drop
    pub async fn drop_table(&self, table_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let sm = self.conn.get_schema_manager().await?;
        let from_schema = sm.create_schema().await?;
        let mut to_schema = from_schema.clone();
        to_schema.drop_table(table_name);
        
        let platform = self.conn.get_database_platform().await?;
        let sql = from_schema.get_migrate_to_sql(&to_schema, &platform)?;
        self.conn.execute_query(&sql).await?;
        Ok(())
    }

    /// Remove all tables defined in a database structure xml file
    ///
    /// # Arguments
    /// * `file` - the xml file describing the tables
    pub async fn remove_db_structure<P: AsRef<Path>>(
        &self,
        file: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let schema_reader = MDB2SchemaReader::new(
            Config::get_object(),
            self.conn.get_database_platform().await?,
        );
        let from_schema = schema_reader.load_schema_from_file(file).await?;
        let mut to_schema = from_schema.clone();
        
        for table in from_schema.get_tables() {
            to_schema.drop_table(table.get_name());
        }
        
        let comparator = SchemaComparator::new();
        let schema_diff = comparator.compare(&from_schema, &to_schema);
        self.execute_schema_change(schema_diff).await?;
        Ok(())
    }

    /// Replaces the ownCloud tables with a new set
    ///
    /// # Arguments
    /// * `file` - path to the MDB2 xml db export file
    pub async fn replace_db<P: AsRef<Path>>(
        &self,
        file: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let apps = oc_app::get_all_apps().await?;
        self.conn.begin_transaction().await?;
        
        // Delete the old tables
        self.remove_db_structure(oc_server::get_server_root().join("db_structure.xml")).await?;

        for app in apps {
            let path = oc_app::get_app_path(&app)
                .join("appinfo")
                .join("database.xml");
            if path.exists() {
                self.remove_db_structure(path).await?;
            }
        }

        // Create new tables
        self.conn.commit().await?;
        Ok(())
    }

    /// Execute schema changes
    async fn execute_schema_change<S: Schema>(
        &self,
        schema: S,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        self.conn.begin_transaction().await?;
        
        let platform = self.conn.get_database_platform().await?;
        for sql in schema.to_sql(&platform)? {
            self.conn.query(&sql).await?;
        }
        
        self.conn.commit().await?;
        Ok(true)
    }

    /// Generate SQL change script from schema diff
    async fn generate_change_script<S: Schema>(
        &self,
        schema: &S,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let platform = self.conn.get_database_platform().await?;
        let sqls = schema.to_sql(&platform)?;
        
        let mut script = String::new();
        for sql in sqls {
            script.push_str(&sql);
            script.push_str(";\n");
        }
        
        Ok(script)
    }
}