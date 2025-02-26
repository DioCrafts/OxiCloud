// Copyright (c) 2013 Christopher Schäpers <christopher@schaepers.it>
// Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use async_trait::async_trait;
use mockall::predicate::*;
use mockall::*;
use sqlx::{postgres::PgPool, Row};
use std::sync::Arc;

#[cfg(test)]
mod test_preferences {
    use super::*;
    use sqlx::query;

    async fn set_up_before_class(pool: &PgPool) -> Result<(), sqlx::Error> {
        let query_str = "INSERT INTO preferences VALUES($1, $2, $3, $4)";

        query(query_str)
            .bind("Someuser")
            .bind("someapp")
            .bind("somekey")
            .bind("somevalue")
            .execute(pool)
            .await?;

        query(query_str)
            .bind("Someuser")
            .bind("getusersapp")
            .bind("somekey")
            .bind("somevalue")
            .execute(pool)
            .await?;
        query(query_str)
            .bind("Anotheruser")
            .bind("getusersapp")
            .bind("somekey")
            .bind("someothervalue")
            .execute(pool)
            .await?;
        query(query_str)
            .bind("Anuser")
            .bind("getusersapp")
            .bind("somekey")
            .bind("somevalue")
            .execute(pool)
            .await?;

        query(query_str)
            .bind("Someuser")
            .bind("getappsapp")
            .bind("somekey")
            .bind("somevalue")
            .execute(pool)
            .await?;

        query(query_str)
            .bind("Someuser")
            .bind("getkeysapp")
            .bind("firstkey")
            .bind("somevalue")
            .execute(pool)
            .await?;
        query(query_str)
            .bind("Someuser")
            .bind("getkeysapp")
            .bind("anotherkey")
            .bind("somevalue")
            .execute(pool)
            .await?;
        query(query_str)
            .bind("Someuser")
            .bind("getkeysapp")
            .bind("key-tastic")
            .bind("somevalue")
            .execute(pool)
            .await?;

        query(query_str)
            .bind("Someuser")
            .bind("getvalueapp")
            .bind("key")
            .bind("a value for a key")
            .execute(pool)
            .await?;

        query(query_str)
            .bind("Deleteuser")
            .bind("deleteapp")
            .bind("deletekey")
            .bind("somevalue")
            .execute(pool)
            .await?;
        query(query_str)
            .bind("Deleteuser")
            .bind("deleteapp")
            .bind("somekey")
            .bind("somevalue")
            .execute(pool)
            .await?;
        query(query_str)
            .bind("Deleteuser")
            .bind("someapp")
            .bind("somekey")
            .bind("somevalue")
            .execute(pool)
            .await?;

        Ok(())
    }

    async fn tear_down_after_class(pool: &PgPool) -> Result<(), sqlx::Error> {
        let query_str = "DELETE FROM preferences WHERE userid = $1";

        query(query_str).bind("Someuser").execute(pool).await?;
        query(query_str).bind("Anotheruser").execute(pool).await?;
        query(query_str).bind("Anuser").execute(pool).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_get_users() -> Result<(), Box<dyn std::error::Error>> {
        let pool = setup_test_db().await?;
        
        // Collect expected results
        let expected_rows = sqlx::query("SELECT DISTINCT userid FROM preferences")
            .fetch_all(&pool)
            .await?;
        
        let mut expected: Vec<String> = Vec::new();
        for row in expected_rows {
            let userid: String = row.get("userid");
            expected.push(userid);
        }

        let preferences = Preferences::new(Arc::new(pool.clone()));
        assert_eq!(expected, preferences.get_users().await?);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_apps() -> Result<(), Box<dyn std::error::Error>> {
        let pool = setup_test_db().await?;
        
        // Collect expected results
        let expected_rows = sqlx::query("SELECT DISTINCT appid FROM preferences WHERE userid = $1")
            .bind("Someuser")
            .fetch_all(&pool)
            .await?;
        
        let mut expected: Vec<String> = Vec::new();
        for row in expected_rows {
            let appid: String = row.get("appid");
            expected.push(appid);
        }

        let preferences = Preferences::new(Arc::new(pool.clone()));
        assert_eq!(expected, preferences.get_apps("Someuser").await?);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_keys() -> Result<(), Box<dyn std::error::Error>> {
        let pool = setup_test_db().await?;
        
        // Collect expected results
        let expected_rows = sqlx::query("SELECT DISTINCT configkey FROM preferences WHERE userid = $1 AND appid = $2")
            .bind("Someuser")
            .bind("getkeysapp")
            .fetch_all(&pool)
            .await?;
        
        let mut expected: Vec<String> = Vec::new();
        for row in expected_rows {
            let configkey: String = row.get("configkey");
            expected.push(configkey);
        }

        let preferences = Preferences::new(Arc::new(pool.clone()));
        assert_eq!(expected, preferences.get_keys("Someuser", "getkeysapp").await?);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_value() -> Result<(), Box<dyn std::error::Error>> {
        let pool = setup_test_db().await?;
        let preferences = Preferences::new(Arc::new(pool.clone()));

        // Test non-existent value
        assert_eq!(None, preferences.get_value("nonexistant", "nonexistant", "nonexistant").await?);
        
        // Test default value
        assert_eq!(
            Some("default".to_string()), 
            preferences.get_value_with_default("nonexistant", "nonexistant", "nonexistant", "default".to_string()).await?
        );
        
        // Test existing value
        let expected_row = sqlx::query("SELECT configvalue FROM preferences WHERE userid = $1 AND appid = $2 AND configkey = $3")
            .bind("Someuser")
            .bind("getvalueapp")
            .bind("key")
            .fetch_one(&pool)
            .await?;
        
        let expected: String = expected_row.get("configvalue");
        assert_eq!(
            Some(expected), 
            preferences.get_value("Someuser", "getvalueapp", "key").await?
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_set_value() -> Result<(), Box<dyn std::error::Error>> {
        let pool = setup_test_db().await?;
        let preferences = Preferences::new(Arc::new(pool.clone()));

        // Set new value
        assert!(preferences.set_value("Someuser", "setvalueapp", "newkey", "newvalue").await?);
        
        // Check if value was set correctly
        let row = sqlx::query("SELECT configvalue FROM preferences WHERE userid = $1 AND appid = $2 AND configkey = $3")
            .bind("Someuser")
            .bind("setvalueapp")
            .bind("newkey")
            .fetch_one(&pool)
            .await?;
        
        let value: String = row.get("configvalue");
        assert_eq!("newvalue", value);

        // Update existing value
        assert!(preferences.set_value("Someuser", "setvalueapp", "newkey", "othervalue").await?);
        
        // Check if value was updated correctly
        let row = sqlx::query("SELECT configvalue FROM preferences WHERE userid = $1 AND appid = $2 AND configkey = $3")
            .bind("Someuser")
            .bind("setvalueapp")
            .bind("newkey")
            .fetch_one(&pool)
            .await?;
        
        let value: String = row.get("configvalue");
        assert_eq!("othervalue", value);

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_key() -> Result<(), Box<dyn std::error::Error>> {
        let pool = setup_test_db().await?;
        let preferences = Preferences::new(Arc::new(pool.clone()));

        assert!(preferences.delete_key("Deleteuser", "deleteapp", "deletekey").await?);
        
        let result = sqlx::query("SELECT configvalue FROM preferences WHERE userid = $1 AND appid = $2 AND configkey = $3")
            .bind("Deleteuser")
            .bind("deleteapp")
            .bind("deletekey")
            .fetch_optional(&pool)
            .await?;
            
        assert!(result.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_app() -> Result<(), Box<dyn std::error::Error>> {
        let pool = setup_test_db().await?;
        let preferences = Preferences::new(Arc::new(pool.clone()));

        assert!(preferences.delete_app("Deleteuser", "deleteapp").await?);
        
        let result = sqlx::query("SELECT configvalue FROM preferences WHERE userid = $1 AND appid = $2")
            .bind("Deleteuser")
            .bind("deleteapp")
            .fetch_optional(&pool)
            .await?;
            
        assert!(result.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_user() -> Result<(), Box<dyn std::error::Error>> {
        let pool = setup_test_db().await?;
        let preferences = Preferences::new(Arc::new(pool.clone()));

        assert!(preferences.delete_user("Deleteuser").await?);
        
        let result = sqlx::query("SELECT configvalue FROM preferences WHERE userid = $1")
            .bind("Deleteuser")
            .fetch_optional(&pool)
            .await?;
            
        assert!(result.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_app_from_all_users() -> Result<(), Box<dyn std::error::Error>> {
        let pool = setup_test_db().await?;
        let preferences = Preferences::new(Arc::new(pool.clone()));

        assert!(preferences.delete_app_from_all_users("someapp").await?);
        
        let result = sqlx::query("SELECT configvalue FROM preferences WHERE appid = $1")
            .bind("someapp")
            .fetch_optional(&pool)
            .await?;
            
        assert!(result.is_none());

        Ok(())
    }

    // Helper function to setup test database
    async fn setup_test_db() -> Result<PgPool, sqlx::Error> {
        let pool = PgPool::connect("postgres://user:password@localhost/testdb").await?;
        set_up_before_class(&pool).await?;
        Ok(pool)
    }
}

#[cfg(test)]
mod test_preferences_object {
    use super::*;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_get_users() -> Result<(), Box<dyn std::error::Error>> {
        let mut mock_db = MockDatabase::new();
        
        mock_db.expect_query_as()
            .with(eq("SELECT DISTINCT userid FROM preferences"))
            .returning(|_| {
                Ok(vec!["foo".to_string()])
            });
        
        let preferences = Preferences::new(Arc::new(mock_db));
        let apps = preferences.get_users().await?;
        
        assert_eq!(vec!["foo"], apps);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_get_apps() -> Result<(), Box<dyn std::error::Error>> {
        let mut mock_db = MockDatabase::new();
        
        mock_db.expect_query_as_with_params()
            .with(eq("SELECT DISTINCT appid FROM preferences WHERE userid = $1"), eq(vec!["bar".to_string()]))
            .returning(|_, _| {
                Ok(vec!["foo".to_string()])
            });
        
        let preferences = Preferences::new(Arc::new(mock_db));
        let apps = preferences.get_apps("bar").await?;
        
        assert_eq!(vec!["foo"], apps);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_get_keys() -> Result<(), Box<dyn std::error::Error>> {
        let mut mock_db = MockDatabase::new();
        
        mock_db.expect_query_as_with_params()
            .with(
                eq("SELECT configkey FROM preferences WHERE userid = $1 AND appid = $2"), 
                eq(vec!["bar".to_string(), "moo".to_string()])
            )
            .returning(|_, _| {
                Ok(vec!["foo".to_string()])
            });
        
        let preferences = Preferences::new(Arc::new(mock_db));
        let keys = preferences.get_keys("bar", "moo").await?;
        
        assert_eq!(vec!["foo"], keys);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_get_value() -> Result<(), Box<dyn std::error::Error>> {
        let mut mock_db = MockDatabase::new();
        
        mock_db.expect_query_row_with_params()
            .with(
                eq("SELECT configvalue FROM preferences WHERE userid = $1 AND appid = $2 AND configkey = $3"), 
                eq(vec!["grg".to_string(), "bar".to_string(), "red".to_string()])
            )
            .returning(|_, _| {
                Ok(Some("foo".to_string()))
            })
            .times(1);
            
        mock_db.expect_query_row_with_params()
            .with(
                eq("SELECT configvalue FROM preferences WHERE userid = $1 AND appid = $2 AND configkey = $3"), 
                eq(vec!["grg".to_string(), "bar".to_string(), "red".to_string()])
            )
            .returning(|_, _| {
                Ok(None)
            })
            .times(1);
        
        let preferences = Preferences::new(Arc::new(mock_db));
        
        let value = preferences.get_value("grg", "bar", "red").await?;
        assert_eq!(Some("foo".to_string()), value);
        
        let value = preferences.get_value_with_default("grg", "bar", "red", "def".to_string()).await?;
        assert_eq!(Some("def".to_string()), value);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_set_value() -> Result<(), Box<dyn std::error::Error>> {
        let mut mock_db = MockDatabase::new();
        
        // First call, no existing value
        mock_db.expect_exists()
            .with(
                eq("SELECT COUNT(*) FROM preferences WHERE userid = $1 AND appid = $2 AND configkey = $3"), 
                eq(vec!["grg".to_string(), "bar".to_string(), "foo".to_string()])
            )
            .returning(|_, _| {
                Ok(false)
            })
            .times(1);
            
        mock_db.expect_insert()
            .with(
                eq("preferences"),
                eq(vec![
                    ("userid".to_string(), "grg".to_string()),
                    ("appid".to_string(), "bar".to_string()),
                    ("configkey".to_string(), "foo".to_string()),
                    ("configvalue".to_string(), "v1".to_string()),
                ])
            )
            .returning(|_, _| {
                Ok(())
            })
            .times(1);
            
        // Second call, value exists
        mock_db.expect_exists()
            .with(
                eq("SELECT COUNT(*) FROM preferences WHERE userid = $1 AND appid = $2 AND configkey = $3"), 
                eq(vec!["grg".to_string(), "bar".to_string(), "foo".to_string()])
            )
            .returning(|_, _| {
                Ok(true)
            })
            .times(1);
            
        mock_db.expect_update()
            .with(
                eq("preferences"),
                eq(vec![
                    ("configvalue".to_string(), "v2".to_string()),
                ]),
                eq(vec![
                    ("userid".to_string(), "grg".to_string()),
                    ("appid".to_string(), "bar".to_string()),
                    ("configkey".to_string(), "foo".to_string()),
                ])
            )
            .returning(|_, _, _| {
                Ok(())
            })
            .times(1);
        
        let preferences = Preferences::new(Arc::new(mock_db));
        
        assert!(preferences.set_value("grg", "bar", "foo", "v1").await?);
        assert!(preferences.set_value("grg", "bar", "foo", "v2").await?);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_key() -> Result<(), Box<dyn std::error::Error>> {
        let mut mock_db = MockDatabase::new();
        
        mock_db.expect_delete()
            .with(
                eq("preferences"),
                eq(vec![
                    ("userid".to_string(), "grg".to_string()),
                    ("appid".to_string(), "bar".to_string()),
                    ("configkey".to_string(), "foo".to_string()),
                ])
            )
            .returning(|_, _| {
                Ok(())
            });
        
        let preferences = Preferences::new(Arc::new(mock_db));
        
        assert!(preferences.delete_key("grg", "bar", "foo").await?);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_app() -> Result<(), Box<dyn std::error::Error>> {
        let mut mock_db = MockDatabase::new();
        
        mock_db.expect_delete()
            .with(
                eq("preferences"),
                eq(vec![
                    ("userid".to_string(), "grg".to_string()),
                    ("appid".to_string(), "bar".to_string()),
                ])
            )
            .returning(|_, _| {
                Ok(())
            });
        
        let preferences = Preferences::new(Arc::new(mock_db));
        
        assert!(preferences.delete_app("grg", "bar").await?);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_user() -> Result<(), Box<dyn std::error::Error>> {
        let mut mock_db = MockDatabase::new();
        
        mock_db.expect_delete()
            .with(
                eq("preferences"),
                eq(vec![
                    ("userid".to_string(), "grg".to_string()),
                ])
            )
            .returning(|_, _| {
                Ok(())
            });
        
        let preferences = Preferences::new(Arc::new(mock_db));
        
        assert!(preferences.delete_user("grg").await?);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_app_from_all_users() -> Result<(), Box<dyn std::error::Error>> {
        let mut mock_db = MockDatabase::new();
        
        mock_db.expect_delete()
            .with(
                eq("preferences"),
                eq(vec![
                    ("appid".to_string(), "bar".to_string()),
                ])
            )
            .returning(|_, _| {
                Ok(())
            });
        
        let preferences = Preferences::new(Arc::new(mock_db));
        
        assert!(preferences.delete_app_from_all_users("bar").await?);
        
        Ok(())
    }
}

#[async_trait]
pub trait DatabaseInterface: Send + Sync {
    async fn query_as<T: Send>(&self, query: &str) -> Result<Vec<T>, sqlx::Error>;
    async fn query_as_with_params<T: Send>(&self, query: &str, params: Vec<String>) -> Result<Vec<T>, sqlx::Error>;
    async fn query_row_with_params(&self, query: &str, params: Vec<String>) -> Result<Option<String>, sqlx::Error>;
    async fn exists(&self, query: &str, params: Vec<String>) -> Result<bool, sqlx::Error>;
    async fn insert(&self, table: &str, values: Vec<(String, String)>) -> Result<(), sqlx::Error>;
    async fn update(&self, table: &str, values: Vec<(String, String)>, conditions: Vec<(String, String)>) -> Result<(), sqlx::Error>;
    async fn delete(&self, table: &str, conditions: Vec<(String, String)>) -> Result<(), sqlx::Error>;
}

#[cfg(test)]
mock! {
    pub Database {}
    
    #[async_trait]
    impl DatabaseInterface for Database {
        async fn query_as<T: Send>(&self, query: &str) -> Result<Vec<T>, sqlx::Error>;
        async fn query_as_with_params<T: Send>(&self, query: &str, params: Vec<String>) -> Result<Vec<T>, sqlx::Error>;
        async fn query_row_with_params(&self, query: &str, params: Vec<String>) -> Result<Option<String>, sqlx::Error>;
        async fn exists(&self, query: &str, params: Vec<String>) -> Result<bool, sqlx::Error>;
        async fn insert(&self, table: &str, values: Vec<(String, String)>) -> Result<(), sqlx::Error>;
        async fn update(&self, table: &str, values: Vec<(String, String)>, conditions: Vec<(String, String)>) -> Result<(), sqlx::Error>;
        async fn delete(&self, table: &str, conditions: Vec<(String, String)>) -> Result<(), sqlx::Error>;
    }
}

pub struct Preferences {
    db: Arc<dyn DatabaseInterface>,
}

impl Preferences {
    pub fn new(db: Arc<dyn DatabaseInterface>) -> Self {
        Self { db }
    }

    pub async fn get_users(&self) -> Result<Vec<String>, sqlx::Error> {
        self.db.query_as("SELECT DISTINCT userid FROM preferences").await
    }

    pub async fn get_apps(&self, user_id: &str) -> Result<Vec<String>, sqlx::Error> {
        self.db.query_as_with_params(
            "SELECT DISTINCT appid FROM preferences WHERE userid = $1",
            vec![user_id.to_string()],
        ).await
    }

    pub async fn get_keys(&self, user_id: &str, app_id: &str) -> Result<Vec<String>, sqlx::Error> {
        self.db.query_as_with_params(
            "SELECT configkey FROM preferences WHERE userid = $1 AND appid = $2",
            vec![user_id.to_string(), app_id.to_string()],
        ).await
    }

    pub async fn get_value(&self, user_id: &str, app_id: &str, key: &str) -> Result<Option<String>, sqlx::Error> {
        self.db.query_row_with_params(
            "SELECT configvalue FROM preferences WHERE userid = $1 AND appid = $2 AND configkey = $3",
            vec![user_id.to_string(), app_id.to_string(), key.to_string()],
        ).await
    }

    pub async fn get_value_with_default(
        &self, 
        user_id: &str, 
        app_id: &str, 
        key: &str, 
        default: String
    ) -> Result<Option<String>, sqlx::Error> {
        match self.get_value(user_id, app_id, key).await? {
            Some(value) => Ok(Some(value)),
            None => Ok(Some(default)),
        }
    }

    pub async fn set_value(&self, user_id: &str, app_id: &str, key: &str, value: &str) -> Result<bool, sqlx::Error> {
        let exists = self.db.exists(
            "SELECT COUNT(*) FROM preferences WHERE userid = $1 AND appid = $2 AND configkey = $3",
            vec![user_id.to_string(), app_id.to_string(), key.to_string()],
        ).await?;

        if exists {
            self.db.update(
                "preferences",
                vec![
                    ("configvalue".to_string(), value.to_string()),
                ],
                vec![
                    ("userid".to_string(), user_id.to_string()),
                    ("appid".to_string(), app_id.to_string()),
                    ("configkey".to_string(), key.to_string()),
                ],
            ).await?;
        } else {
            self.db.insert(
                "preferences",
                vec![
                    ("userid".to_string(), user_id.to_string()),
                    ("appid".to_string(), app_id.to_string()),
                    ("configkey".to_string(), key.to_string()),
                    ("configvalue".to_string(), value.to_string()),
                ],
            ).await?;
        }

        Ok(true)
    }

    pub async fn delete_key(&self, user_id: &str, app_id: &str, key: &str) -> Result<bool, sqlx::Error> {
        self.db.delete(
            "preferences",
            vec![
                ("userid".to_string(), user_id.to_string()),
                ("appid".to_string(), app_id.to_string()),
                ("configkey".to_string(), key.to_string()),
            ],
        ).await?;

        Ok(true)
    }

    pub async fn delete_app(&self, user_id: &str, app_id: &str) -> Result<bool, sqlx::Error> {
        self.db.delete(
            "preferences",
            vec![
                ("userid".to_string(), user_id.to_string()),
                ("appid".to_string(), app_id.to_string()),
            ],
        ).await?;

        Ok(true)
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<bool, sqlx::Error> {
        self.db.delete(
            "preferences",
            vec![
                ("userid".to_string(), user_id.to_string()),
            ],
        ).await?;

        Ok(true)
    }

    pub async fn delete_app_from_all_users(&self, app_id: &str) -> Result<bool, sqlx::Error> {
        self.db.delete(
            "preferences",
            vec![
                ("appid".to_string(), app_id.to_string()),
            ],
        ).await?;

        Ok(true)
    }
}