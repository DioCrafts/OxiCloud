// Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::collections::HashMap;
use log::{error, fatal};
use thiserror::Error;

/// Error types for database operations
#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("SQL error: {0}")]
    SqlError(String),
}

/// This handles the way we use to write queries, into something that can be
/// handled by the database abstraction layer.
pub struct Adapter {
    conn: Connection,
}

impl Adapter {
    /// Create a new adapter with the given connection
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    /// Get the ID of the last insert statement
    ///
    /// # Arguments
    ///
    /// * `table` - Table name
    ///
    /// # Returns
    ///
    /// The ID of the last insert statement
    pub fn last_insert_id(&self, table: &str) -> i64 {
        self.conn.real_last_insert_id(table)
    }

    /// Modify the statement so the database can handle it
    ///
    /// # Arguments
    ///
    /// * `statement` - SQL statement that needs to be changed
    ///
    /// # Returns
    ///
    /// The modified statement
    pub fn fixup_statement(&self, statement: &str) -> String {
        statement.to_string()
    }

    /// Insert the input values when they do not exist yet
    ///
    /// # Arguments
    ///
    /// * `table` - Table name
    /// * `input` - Key-value pairs to insert
    ///
    /// # Returns
    ///
    /// The count of inserted rows or an error
    pub fn insert_if_not_exist(&self, table: &str, input: HashMap<String, String>) -> Result<i64, DbError> {
        let keys: Vec<&String> = input.keys().collect();
        let values_count = input.len();
        
        let placeholders = vec!["?"; values_count].join(",");
        
        let mut query = format!(
            "INSERT INTO `{}` (`{}`) SELECT {} FROM `{}` WHERE ",
            table,
            keys.iter().map(|k| format!("`{}`", k)).collect::<Vec<String>>().join("`,`"),
            placeholders,
            table
        );
        
        for key in &keys {
            query.push_str(&format!("`{}` = ? AND ", key));
        }
        
        // Remove the trailing " AND "
        query.truncate(query.len() - 5);
        query.push_str(" HAVING COUNT(*) = 0");
        
        // Prepare the parameter values - we need each value twice
        let mut inserts = Vec::with_capacity(values_count * 2);
        for key in &keys {
            if let Some(value) = input.get(*key) {
                inserts.push(value.clone());
            }
        }
        inserts.extend(inserts.clone());
        
        match self.conn.execute_update(&query, &inserts) {
            Ok(count) => Ok(count),
            Err(e) => {
                let entry = format!("DB Error: \"{}\"\n", e);
                let entry = format!("{}Offending command was: {}\n", entry, query);
                
                fatal!("{}", entry);
                error!("DB error: {}", entry);
                
                // In Rust we return an error rather than calling a template printer
                Err(DbError::DatabaseError(entry))
            }
        }
    }
}

// We need to define or import the Connection type to make the code compile
// This would be defined elsewhere in the actual codebase
pub struct Connection {
    // Connection implementation details would go here
}

impl Connection {
    pub fn real_last_insert_id(&self, _table: &str) -> i64 {
        // Implementation would go here
        0
    }
    
    pub fn execute_update(&self, _query: &str, _params: &[String]) -> Result<i64, String> {
        // Implementation would go here
        Ok(0)
    }
}