//! Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.

use crate::db::Adapter;
use async_trait::async_trait;

pub struct AdapterOCI8 {
    conn: Box<dyn Connection>,
}

impl AdapterOCI8 {
    pub fn new(conn: Box<dyn Connection>) -> Self {
        Self { conn }
    }

    const UNIX_TIMESTAMP_REPLACEMENT: &'static str = 
        "(cast(sys_extract_utc(systimestamp) as date) - date'1970-01-01') * 86400";
}

#[async_trait]
impl Adapter for AdapterOCI8 {
    async fn last_insert_id(&self, table: Option<&str>) -> Result<String, DatabaseError> {
        if let Some(table_name) = table {
            let suffix = "_SEQ";
            let formatted_table = format!("\"{}{}\"", table_name, suffix);
            self.conn.real_last_insert_id(Some(&formatted_table)).await
        } else {
            self.conn.real_last_insert_id(None).await
        }
    }

    fn fixup_statement(&self, statement: &str) -> String {
        let mut result = statement.replace("`", "\"");
        result = result.replace("NOW()", "CURRENT_TIMESTAMP");
        result = result.replace("UNIX_TIMESTAMP()", Self::UNIX_TIMESTAMP_REPLACEMENT);
        result
    }
}

#[async_trait]
pub trait Connection: Send + Sync {
    async fn real_last_insert_id(&self, table: Option<&str>) -> Result<String, DatabaseError>;
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Database error: {0}")]
    General(String),
    
    #[error("Connection error: {0}")]
    Connection(String),
    
    #[error("Query error: {0}")]
    Query(String),
}