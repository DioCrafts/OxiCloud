use chrono::{DateTime, Utc, NaiveDateTime};
use rand::Rng;
use sqlx::{Pool, MySql, query, query_as, Error as SqlxError};
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

/// Copyright (c) 2012 Bart Visscher <bartv@thisnet.nl>
/// This file is licensed under the Affero General Public License version 3 or
/// later.
/// See the COPYING-README file.

struct TestDb {
    pool: Arc<Pool<MySql>>,
    schema_file: String,
    test_prefix: String,
    table1: String,
    table2: String,
    table3: String,
}

#[async_trait::async_trait]
impl test_framework::TestCase for TestDb {
    async fn set_up(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let server_root = std::env::var("SERVER_ROOT")?;
        let dbfile = format!("{}/tests/data/db_structure.xml", server_root);
        
        let mut rng = rand::thread_rng();
        let random_bytes: String = (0..4)
            .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
            .collect();
        
        let r = format!("_{}_", random_bytes);
        let content = fs::read_to_string(&dbfile)?;
        let content = content.replace("*dbprefix*", &format!("*dbprefix*{}", r));
        
        fs::write(&self.schema_file, content)?;
        
        db::create_db_from_structure(&self.pool, &self.schema_file).await?;
        
        self.test_prefix = r;
        self.table1 = format!("{}cntcts_addrsbks", self.test_prefix);
        self.table2 = format!("{}cntcts_cards", self.test_prefix);
        self.table3 = format!("{}vcategory", self.test_prefix);
        
        Ok(())
    }
    
    async fn tear_down(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        db::remove_db_structure(&self.pool, &self.schema_file).await?;
        fs::remove_file(&self.schema_file)?;
        Ok(())
    }
}

impl TestDb {
    async fn new(pool: Arc<Pool<MySql>>) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            pool,
            schema_file: "static://test_db_scheme".to_string(),
            test_prefix: String::new(),
            table1: String::new(),
            table2: String::new(),
            table3: String::new(),
        })
    }

    async fn test_quotes(&self) -> Result<(), SqlxError> {
        let sql = format!("SELECT `fullname` FROM `*PREFIX*{}` WHERE `uri` = ?", self.table2);
        let row: Option<(String,)> = query_as(&sql)
            .bind("uri_1")
            .fetch_optional(&*self.pool)
            .await?;
        
        assert!(row.is_none());
        
        let sql = format!("INSERT INTO `*PREFIX*{}` (`fullname`,`uri`) VALUES (?,?)", self.table2);
        let result = query(&sql)
            .bind("fullname test")
            .bind("uri_1")
            .execute(&*self.pool)
            .await?;
        
        assert_eq!(1, result.rows_affected());
        
        let sql = format!("SELECT `fullname`,`uri` FROM `*PREFIX*{}` WHERE `uri` = ?", self.table2);
        let rows: Vec<(String, String)> = query_as(&sql)
            .bind("uri_1")
            .fetch_all(&*self.pool)
            .await?;
        
        assert!(!rows.is_empty());
        assert_eq!("fullname test", rows[0].0);
        assert_eq!(1, rows.len());
        
        Ok(())
    }

    async fn test_now(&self) -> Result<(), SqlxError> {
        let sql = format!("INSERT INTO `*PREFIX*{}` (`fullname`,`uri`) VALUES (NOW(),?)", self.table2);
        let result = query(&sql)
            .bind("uri_2")
            .execute(&*self.pool)
            .await?;
        
        assert_eq!(1, result.rows_affected());
        
        let sql = format!("SELECT `fullname`,`uri` FROM `*PREFIX*{}` WHERE `uri` = ?", self.table2);
        let rows: Vec<(String, String)> = query_as(&sql)
            .bind("uri_2")
            .fetch_all(&*self.pool)
            .await?;
        
        assert!(!rows.is_empty());
        
        Ok(())
    }

    async fn test_unix_timestamp(&self) -> Result<(), SqlxError> {
        let sql = format!("INSERT INTO `*PREFIX*{}` (`fullname`,`uri`) VALUES (UNIX_TIMESTAMP(),?)", self.table2);
        let result = query(&sql)
            .bind("uri_3")
            .execute(&*self.pool)
            .await?;
        
        assert_eq!(1, result.rows_affected());
        
        let sql = format!("SELECT `fullname`,`uri` FROM `*PREFIX*{}` WHERE `uri` = ?", self.table2);
        let rows: Vec<(String, String)> = query_as(&sql)
            .bind("uri_3")
            .fetch_all(&*self.pool)
            .await?;
        
        assert!(!rows.is_empty());
        
        Ok(())
    }
    
    async fn test_last_insert_id(&self) -> Result<(), SqlxError> {
        let sql = format!("INSERT INTO `*PREFIX*{}` (`fullname`,`uri`) VALUES (?,?)", self.table2);
        let result1 = query(&sql)
            .bind("insertid 1")
            .bind("uri_1")
            .execute(&*self.pool)
            .await?;
            
        let id1 = db::insert_id(&self.pool, &format!("*PREFIX*{}", self.table2)).await?;
        
        // we don't know the id we should expect, so insert another row
        let result2 = query(&sql)
            .bind("insertid 2")
            .bind("uri_2")
            .execute(&*self.pool)
            .await?;
            
        let id2 = db::insert_id(&self.pool, &format!("*PREFIX*{}", self.table2)).await?;
        
        // now we can check if the two ids are in correct order
        assert!(id2 > id1);
        
        Ok(())
    }
    
    async fn test_insert_if_not_exist(&self) -> Result<(), SqlxError> {
        let category_entries = vec![
            ("test", "contact", "Family", 1),
            ("test", "contact", "Friends", 1),
            ("test", "contact", "Coworkers", 1),
            ("test", "contact", "Coworkers", 0),
            ("test", "contact", "School", 1),
        ];
        
        for entry in category_entries {
            let result = db::insert_if_not_exist(
                &self.pool,
                &format!("*PREFIX*{}", self.table3),
                &[
                    ("uid", entry.0),
                    ("type", entry.1),
                    ("category", entry.2),
                ],
            ).await?;
            
            assert_eq!(entry.3, result);
        }
        
        let sql = format!("SELECT * FROM `*PREFIX*{}`", self.table3);
        let rows: Vec<(i64, String, String, String)> = query_as(&sql)
            .fetch_all(&*self.pool)
            .await?;
            
        assert_eq!(4, rows.len());
        
        Ok(())
    }
    
    async fn test_insert_if_not_exist_dont_overwrite(&self) -> Result<(), SqlxError> {
        let fullname = "fullname test";
        let uri = "uri_1";
        let carddata = "This is a vCard";
        
        // Normal test to have same known data inserted.
        let sql = format!("INSERT INTO `*PREFIX*{}` (`fullname`, `uri`, `carddata`) VALUES (?, ?, ?)", self.table2);
        let result = query(&sql)
            .bind(fullname)
            .bind(uri)
            .bind(carddata)
            .execute(&*self.pool)
            .await?;
            
        assert_eq!(1, result.rows_affected());
        
        let sql = format!("SELECT `fullname`, `uri`, `carddata` FROM `*PREFIX*{}` WHERE `uri` = ?", self.table2);
        let rows: Vec<(String, String, String)> = query_as(&sql)
            .bind(uri)
            .fetch_all(&*self.pool)
            .await?;
            
        assert!(!rows.is_empty());
        assert_eq!(carddata, rows[0].2);
        assert_eq!(1, rows.len());
        
        // Try to insert a new row
        let result = db::insert_if_not_exist(
            &self.pool,
            &format!("*PREFIX*{}", self.table2),
            &[
                ("fullname", fullname),
                ("uri", uri),
            ],
        ).await?;
        
        assert_eq!(0, result);
        
        let sql = format!("SELECT `fullname`, `uri`, `carddata` FROM `*PREFIX*{}` WHERE `uri` = ?", self.table2);
        let rows: Vec<(String, String, String)> = query_as(&sql)
            .bind(uri)
            .fetch_all(&*self.pool)
            .await?;
            
        assert!(!rows.is_empty());
        // Test that previously inserted data isn't overwritten
        assert_eq!(carddata, rows[0].2);
        // And that a new row hasn't been inserted.
        assert_eq!(1, rows.len());
        
        Ok(())
    }
    
    async fn test_timestamp_date_format(&self) -> Result<(), Box<dyn std::error::Error>> {
        let table = format!("*PREFIX*{}timestamp", self.test_prefix);
        let column = "timestamptest";
        
        let expected_format = "%Y-%m-%d %H:%M:%S";
        let expected = Utc::now();
        let formatted_date = expected.format(expected_format).to_string();
        
        let sql = format!("INSERT INTO `{}` (`{}`) VALUES (?)", table, column);
        let result = query(&sql)
            .bind(&formatted_date)
            .execute(&*self.pool)
            .await?;
            
        assert_eq!(
            1,
            result.rows_affected(),
            "Database failed to accept dates in the format '{}'.",
            expected_format
        );
        
        let id = db::insert_id(&self.pool, &table).await?;
        let sql = format!("SELECT * FROM `{}` WHERE `id` = ?", table);
        let row: (i64, String) = query_as(&sql)
            .bind(id)
            .fetch_one(&*self.pool)
            .await?;
            
        let actual = NaiveDateTime::parse_from_str(&row.1, expected_format)?;
        let actual = DateTime::<Utc>::from_utc(actual, Utc);
        
        assert_eq!(
            expected.format(expected_format).to_string(),
            actual.format(expected_format).to_string(),
            "Failed asserting that the returned date is the same as the inserted."
        );
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_quotes() {
        let pool = setup_test_db().await;
        let test = TestDb::new(Arc::new(pool)).await.unwrap();
        test.set_up().await.unwrap();
        test.test_quotes().await.unwrap();
        test.tear_down().await.unwrap();
    }
    
    #[tokio::test]
    async fn test_now() {
        let pool = setup_test_db().await;
        let test = TestDb::new(Arc::new(pool)).await.unwrap();
        test.set_up().await.unwrap();
        test.test_now().await.unwrap();
        test.tear_down().await.unwrap();
    }
    
    #[tokio::test]
    async fn test_unix_timestamp() {
        let pool = setup_test_db().await;
        let test = TestDb::new(Arc::new(pool)).await.unwrap();
        test.set_up().await.unwrap();
        test.test_unix_timestamp().await.unwrap();
        test.tear_down().await.unwrap();
    }
    
    #[tokio::test]
    async fn test_last_insert_id() {
        let pool = setup_test_db().await;
        let test = TestDb::new(Arc::new(pool)).await.unwrap();
        test.set_up().await.unwrap();
        test.test_last_insert_id().await.unwrap();
        test.tear_down().await.unwrap();
    }
    
    #[tokio::test]
    async fn test_insert_if_not_exist() {
        let pool = setup_test_db().await;
        let test = TestDb::new(Arc::new(pool)).await.unwrap();
        test.set_up().await.unwrap();
        test.test_insert_if_not_exist().await.unwrap();
        test.tear_down().await.unwrap();
    }
    
    #[tokio::test]
    async fn test_insert_if_not_exist_dont_overwrite() {
        let pool = setup_test_db().await;
        let test = TestDb::new(Arc::new(pool)).await.unwrap();
        test.set_up().await.unwrap();
        test.test_insert_if_not_exist_dont_overwrite().await.unwrap();
        test.tear_down().await.unwrap();
    }
    
    #[tokio::test]
    async fn test_timestamp_date_format() {
        let pool = setup_test_db().await;
        let test = TestDb::new(Arc::new(pool)).await.unwrap();
        test.set_up().await.unwrap();
        test.test_timestamp_date_format().await.unwrap();
        test.tear_down().await.unwrap();
    }
    
    async fn setup_test_db() -> Pool<MySql> {
        // This would be the actual implementation to create a test database connection
        todo!("Implement actual database connection setup for tests")
    }
}