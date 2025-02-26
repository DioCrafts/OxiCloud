use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use std::error::Error;

/// AdapterPgSql - PostgreSQL adapter implementation
///
/// Ported from the original PHP implementation:
/// Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
/// This file is licensed under the Affero General Public License version 3 or
/// later.
/// See the COPYING-README file.
pub struct AdapterPgSql {
    conn: Pool<Postgres>,
}

impl AdapterPgSql {
    pub fn new(conn: Pool<Postgres>) -> Self {
        Self { conn }
    }

    const UNIX_TIMESTAMP_REPLACEMENT: &'static str = "cast(extract(epoch from current_timestamp) as integer)";
}

#[async_trait]
impl Adapter for AdapterPgSql {
    async fn last_insert_id(&self, _table: &str) -> Result<i64, Box<dyn Error>> {
        let result = sqlx::query_scalar::<_, i64>("SELECT lastval()")
            .fetch_one(&self.conn)
            .await?;
        
        Ok(result)
    }

    fn fixup_statement(&self, statement: &str) -> String {
        let mut fixed = statement.replace("`", "\"");
        fixed = fixed.replace("UNIX_TIMESTAMP()", Self::UNIX_TIMESTAMP_REPLACEMENT);
        fixed
    }
}

#[async_trait]
pub trait Adapter {
    async fn last_insert_id(&self, table: &str) -> Result<i64, Box<dyn Error>>;
    fn fixup_statement(&self, statement: &str) -> String;
}