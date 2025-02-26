// Copyright (c) 2012 Tom Needham tom@owncloud.com
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
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use rusqlite::{Connection, Result as SqlResult, Row, Statement, NO_PARAMS, params};
use zip::{ZipWriter, CompressionMethod, write::FileOptions};
use log::{error, fatal};
use tempfile::NamedTempFile;

type DbResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Provides methods to add and access data from the migration
pub struct MigrationContent {
    zip: Option<ZipWriter<File>>,
    db: Option<Connection>,
    tmp_files: Vec<PathBuf>,
}

impl MigrationContent {
    /// Sets up the migration content
    ///
    /// # Arguments
    /// * `zip` - ZipWriter object
    /// * `db` - Optional database connection (required for exporttype user)
    ///
    /// # Returns
    /// * `Self` - New MigrationContent instance
    pub fn new(zip: ZipWriter<File>, db: Option<Connection>) -> Self {
        MigrationContent {
            zip: Some(zip),
            db,
            tmp_files: Vec::new(),
        }
    }

    /// Prepares the database query
    ///
    /// # Arguments
    /// * `query` - SQL query to prepare
    ///
    /// # Returns
    /// * `Result<Statement, Box<dyn std::error::Error>>` - Prepared statement or error
    pub fn prepare(&mut self, query: &str) -> DbResult<Statement> {
        // Only add database to tmp_files if actually used
        if let Some(db) = &self.db {
            if let Ok(db_path) = db.path() {
                if let Some(path) = db_path {
                    let path_buf = PathBuf::from(path);
                    if !self.tmp_files.contains(&path_buf) {
                        self.tmp_files.push(path_buf);
                    }
                }
            }
        }

        // Process the query
        let processed_query = self.process_query(query);

        // Prepare the query
        match &self.db {
            Some(conn) => {
                match conn.prepare(&processed_query) {
                    Ok(stmt) => Ok(stmt),
                    Err(err) => {
                        let message = format!(
                            "DB Error: \"{}\"\nOffending command was: {}", 
                            err.to_string(), processed_query
                        );
                        error!("{}", message);
                        Err(Box::new(err))
                    }
                }
            },
            None => Err("Database connection not initialized".into()),
        }
    }

    /// Processes the database query
    ///
    /// # Arguments
    /// * `query` - Query to process
    ///
    /// # Returns
    /// * `String` - Processed query
    fn process_query(&self, query: &str) -> String {
        let mut processed = query.replace("`", "'");
        processed = processed.replace("NOW()", "datetime('now')");
        processed = processed.replace("now()", "datetime('now')");
        // Remove table prefixes
        processed = processed.replace("*PREFIX*", "");
        processed
    }

    /// Copies rows to migration.db from the main database
    ///
    /// # Arguments
    /// * `options` - HashMap of options
    ///
    /// # Returns
    /// * `Result<Vec<String>, Box<dyn std::error::Error>>` - Vector of results or error
    pub fn copy_rows(&mut self, options: &HashMap<String, String>) -> DbResult<Vec<String>> {
        if !options.contains_key("table") {
            return Err("Table option is required".into());
        }

        let mut results = Vec::new();
        let table = options.get("table").unwrap();

        // Database connection from a different source (not self.db)
        // This would need to be adapted based on your actual DB setup
        let db_conn = Connection::open("original_db.sqlite")?;

        if options.contains_key("matchval") && options.contains_key("matchcol") {
            let match_col = options.get("matchcol").unwrap();
            let match_vals = options.get("matchval").unwrap();
            
            // Split match values if they're comma-separated
            let values: Vec<&str> = match_vals.split(',').collect();
            
            for val in values {
                let sql = format!("SELECT * FROM {} WHERE {} = ?", table, match_col);
                let mut stmt = db_conn.prepare(&sql)?;
                let rows = stmt.query_map(params![val], |row| self.row_to_map(row))?;
                
                for row_result in rows {
                    let row = row_result?;
                    let new_results = self.insert_data(&row, options)?;
                    results.extend(new_results);
                }
            }
        } else {
            // Get everything
            let sql = format!("SELECT * FROM {}", table);
            let mut stmt = db_conn.prepare(&sql)?;
            let rows = stmt.query_map(NO_PARAMS, |row| self.row_to_map(row))?;
            
            for row_result in rows {
                let row = row_result?;
                let new_results = self.insert_data(&row, options)?;
                results.extend(new_results);
            }
        }

        Ok(results)
    }

    /// Helper to convert a rusqlite Row to a HashMap
    fn row_to_map(&self, row: &Row) -> rusqlite::Result<HashMap<String, String>> {
        let mut map = HashMap::new();
        let column_count = row.column_count();
        
        for i in 0..column_count {
            let column_name = row.column_name(i)?;
            let value: String = row.get(i)?;
            map.insert(column_name.to_string(), value);
        }
        
        Ok(map)
    }

    /// Saves a SQL data set into migration.db
    ///
    /// # Arguments
    /// * `data` - HashMap of column names to values
    /// * `options` - Options for the copy operation
    ///
    /// # Returns
    /// * `Result<Vec<String>, Box<dyn std::error::Error>>` - Vector of result IDs
    fn insert_data(&mut self, data: &HashMap<String, String>, options: &HashMap<String, String>) -> DbResult<Vec<String>> {
        let mut results = Vec::new();
        let table = options.get("table").unwrap();
        
        let fields: Vec<&String> = data.keys().collect();
        let values: Vec<&String> = fields.iter().map(|&k| data.get(k).unwrap()).collect();
        
        // Generate SQL
        let fields_sql = fields.iter().map(|s| format!("`{}`", s)).collect::<Vec<_>>().join(", ");
        let placeholders = vec!["?"; fields.len()].join(", ");
        let sql = format!("INSERT INTO `{}` ({}) VALUES({})", table, fields_sql, placeholders);
        
        // Prepare and execute the query
        match self.prepare(&sql) {
            Ok(mut stmt) => {
                // Convert Vec<&String> to Vec<&str> for params
                let values_str: Vec<&str> = values.iter().map(|s| s.as_str()).collect();
                
                match stmt.execute(rusqlite::params_from_iter(values_str.iter())) {
                    Ok(_) => {
                        // Determine which value to return
                        if let Some(id_col) = options.get("idcol") {
                            if let Some(id_val) = data.get(id_col) {
                                results.push(id_val.clone());
                            }
                        } else if let Some(first_val) = values.first() {
                            results.push((*first_val).clone());
                        }
                    },
                    Err(e) => return Err(Box::new(e)),
                }
            },
            Err(e) => {
                error!("Invalid SQL produced: {}", sql);
                return Err(e);
            }
        }
        
        Ok(results)
    }

    /// Adds a directory to the zip object
    ///
    /// # Arguments
    /// * `dir` - Path of the directory to add
    /// * `recursive` - Whether to add subdirectories recursively
    /// * `internal_dir` - Path of folder to add dir to in zip
    ///
    /// # Returns
    /// * `Result<(), Box<dyn std::error::Error>>` - Success or error
    pub fn add_dir<P: AsRef<Path>>(&mut self, dir: P, recursive: bool, internal_dir: &str) -> DbResult<()> {
        let dir_path = dir.as_ref();
        let dir_name = dir_path.file_name()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid directory name"))?
            .to_string_lossy();
        
        if let Some(zip) = &mut self.zip {
            // Add the directory entry
            let options = FileOptions::default()
                .compression_method(CompressionMethod::Stored)
                .unix_permissions(0o755);
            
            let full_internal_path = format!("{}{}/", internal_dir, dir_name);
            zip.add_directory(&full_internal_path, options)?;
            
            // Check if the directory exists
            if !dir_path.exists() {
                return Err(format!("Directory doesn't exist: {:?}", dir_path).into());
            }
            
            // Read directory contents
            let entries = fs::read_dir(dir_path)?;
            
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().into_owned();
                
                if name == "." || name == ".." {
                    continue;
                }
                
                if path.is_dir() && recursive {
                    self.add_dir(&path, recursive, &full_internal_path)?;
                } else if path.is_file() {
                    let options = FileOptions::default()
                        .compression_method(CompressionMethod::Deflated)
                        .unix_permissions(0o644);
                    
                    let zip_path = format!("{}{}", full_internal_path, name);
                    zip.start_file(zip_path, options)?;
                    
                    let mut file = File::open(&path)?;
                    io::copy(&mut file, zip)?;
                }
            }
            
            Ok(())
        } else {
            Err("Zip writer not initialized".into())
        }
    }

    /// Adds a file to the zip from a given string
    ///
    /// # Arguments
    /// * `data` - String of data to add
    /// * `path` - Relative path inside of the zip to save the file to
    ///
    /// # Returns
    /// * `Result<(), Box<dyn std::error::Error>>` - Success or error
    pub fn add_from_string(&mut self, data: &str, path: &str) -> DbResult<()> {
        // Create a temp file
        let mut temp_file = NamedTempFile::new()?;
        let temp_path = temp_file.path().to_path_buf();
        
        // Add to list of temporary files
        self.tmp_files.push(temp_path.clone());
        
        // Write data to the temp file
        if let Err(e) = temp_file.write_all(data.as_bytes()) {
            error!("Failed to save data to a temporary file: {}", e);
            return Err(e.into());
        }
        
        // Add file to the zip
        if let Some(zip) = &mut self.zip {
            let options = FileOptions::default()
                .compression_method(CompressionMethod::Deflated)
                .unix_permissions(0o644);
            
            zip.start_file(path, options)?;
            
            let mut file = File::open(temp_path)?;
            io::copy(&mut file, zip)?;
            
            Ok(())
        } else {
            Err("Zip writer not initialized".into())
        }
    }

    /// Closes the zip, removes temp files
    ///
    /// # Returns
    /// * `Result<(), Box<dyn std::error::Error>>` - Success or error
    pub fn finish(&mut self) -> DbResult<()> {
        if let Some(zip) = self.zip.take() {
            match zip.finish() {
                Ok(_) => {
                    self.cleanup();
                    Ok(())
                },
                Err(e) => {
                    error!("Failed to write the zip file with error: {}", e);
                    Err(e.into())
                }
            }
        } else {
            Err("Zip writer not initialized".into())
        }
    }

    /// Cleans up after the zip
    fn cleanup(&self) {
        // Delete tmp files
        for path in &self.tmp_files {
            if let Err(e) = fs::remove_file(path) {
                error!("Failed to remove temporary file {:?}: {}", path, e);
            }
        }
    }
}

impl Drop for MigrationContent {
    fn drop(&mut self) {
        self.cleanup();
    }
}