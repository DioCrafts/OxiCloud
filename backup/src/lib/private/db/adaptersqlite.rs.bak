/*
 * Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use crate::db::adapter::Adapter;
use log::{error, fatal};
use rusqlite::{Connection, params, Result as SqliteResult};
use std::collections::HashMap;

pub struct AdapterSqlite {
    conn: Connection,
}

impl AdapterSqlite {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn fixup_statement(&self, statement: &str) -> String {
        let mut result = statement.replace("`", "\"");
        result = result.replace("NOW()", "datetime('now')");
        result = result.replace("UNIX_TIMESTAMP()", "strftime('%s','now')");
        result
    }

    pub fn insert_if_not_exist(&self, table: &str, input: HashMap<String, String>) -> Result<usize, String> {
        // NOTE: For SQLite we have to use this clumsy approach
        // otherwise all fieldnames used must have a unique key.
        let mut query = format!("SELECT COUNT(*) FROM \"{}\" WHERE ", table);
        let mut values = Vec::new();
        
        for (key, value) in &input {
            query.push_str(&format!("\"{}\" = ? AND ", key));
            values.push(value.clone());
        }
        
        query = query[0..query.len() - 5].to_string();
        
        let mut stmt = match self.conn.prepare(&query) {
            Ok(stmt) => stmt,
            Err(e) => {
                let entry = format!("DB Error: \"{}\"\nOffending command was: {}", e.to_string(), query);
                fatal!("core", "{}", entry);
                error!("DB error: {}", entry);
                return Err(entry);
            }
        };

        let count: i64 = match stmt.query_row(params_from_slice(&values), |row| row.get(0)) {
            Ok(count) => count,
            Err(e) => {
                let entry = format!("DB Error: \"{}\"\nOffending command was: {}", e.to_string(), query);
                fatal!("core", "{}", entry);
                error!("DB error: {}", entry);
                return Err(entry);
            }
        };

        if count == 0 {
            let keys: Vec<&String> = input.keys().collect();
            let placeholders = vec!["?"; input.len()].join(",");
            let query = format!(
                "INSERT INTO \"{}\" (\"{}\")\nVALUES({})",
                table,
                keys.join("\",\""),
                placeholders
            );

            let values: Vec<&String> = input.values().collect();
            
            match self.conn.execute(&query, params_from_slice(&values)) {
                Ok(affected) => Ok(affected),
                Err(e) => {
                    let entry = format!("DB Error: \"{}\"\nOffending command was: {}", e.to_string(), query);
                    fatal!("core", "{}", entry);
                    error!("DB error: {}", entry);
                    return Err(entry);
                }
            }
        } else {
            Ok(0) // no rows updated
        }
    }
}

impl Adapter for AdapterSqlite {
    // Implement required Adapter trait methods here
}

// Helper function to convert a Vec<String> to rusqlite params
fn params_from_slice(values: &[String]) -> Vec<&dyn rusqlite::ToSql> {
    values.iter().map(|v| v as &dyn rusqlite::ToSql).collect()
}