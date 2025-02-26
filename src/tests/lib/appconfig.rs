// Copyright (c) 2013 Christopher Schäpers <christopher@schaepers.it>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use async_trait::async_trait;
use sqlx::{Pool, Sqlite, query, Row};
use serial_test::serial;

struct AppConfig;

impl AppConfig {
    pub async fn get_apps(db: &Pool<Sqlite>) -> Result<Vec<String>, sqlx::Error> {
        let rows = query("SELECT DISTINCT `appid` FROM `*PREFIX*appconfig`")
            .fetch_all(db)
            .await?;
        
        let mut apps = Vec::new();
        for row in rows {
            apps.push(row.get("appid"));
        }
        Ok(apps)
    }

    pub async fn get_keys(db: &Pool<Sqlite>, app_id: &str) -> Result<Vec<String>, sqlx::Error> {
        let rows = query("SELECT `configkey` FROM `*PREFIX*appconfig` WHERE `appid` = ?")
            .bind(app_id)
            .fetch_all(db)
            .await?;
        
        let mut keys = Vec::new();
        for row in rows {
            keys.push(row.get("configkey"));
        }
        Ok(keys)
    }

    pub async fn get_value(db: &Pool<Sqlite>, app_id: &str, key: &str) -> Result<Option<String>, sqlx::Error> {
        let row = query("SELECT `configvalue` FROM `*PREFIX*appconfig` WHERE `appid` = ? AND `configkey` = ?")
            .bind(app_id)
            .bind(key)
            .fetch_optional(db)
            .await?;
        
        Ok(row.map(|r| r.get("configvalue")))
    }

    pub async fn get_value_with_default(db: &Pool<Sqlite>, app_id: &str, key: &str, default: &str) -> Result<String, sqlx::Error> {
        match Self::get_value(db, app_id, key).await? {
            Some(value) => Ok(value),
            None => Ok(default.to_string()),
        }
    }

    pub async fn has_key(db: &Pool<Sqlite>, app_id: &str, key: &str) -> Result<bool, sqlx::Error> {
        let row = query("SELECT COUNT(*) as count FROM `*PREFIX*appconfig` WHERE `appid` = ? AND `configkey` = ?")
            .bind(app_id)
            .bind(key)
            .fetch_one(db)
            .await?;
        
        let count: i64 = row.get("count");
        Ok(count > 0)
    }

    pub async fn set_value(db: &Pool<Sqlite>, app_id: &str, key: &str, value: &str) -> Result<(), sqlx::Error> {
        // Check if key exists first
        if Self::has_key(db, app_id, key).await? {
            query("UPDATE `*PREFIX*appconfig` SET `configvalue` = ? WHERE `appid` = ? AND `configkey` = ?")
                .bind(value)
                .bind(app_id)
                .bind(key)
                .execute(db)
                .await?;
        } else {
            query("INSERT INTO `*PREFIX*appconfig` VALUES (?, ?, ?)")
                .bind(app_id)
                .bind(key)
                .bind(value)
                .execute(db)
                .await?;
        }
        Ok(())
    }

    pub async fn delete_key(db: &Pool<Sqlite>, app_id: &str, key: &str) -> Result<(), sqlx::Error> {
        query("DELETE FROM `*PREFIX*appconfig` WHERE `appid` = ? AND `configkey` = ?")
            .bind(app_id)
            .bind(key)
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn delete_app(db: &Pool<Sqlite>, app_id: &str) -> Result<(), sqlx::Error> {
        query("DELETE FROM `*PREFIX*appconfig` WHERE `appid` = ?")
            .bind(app_id)
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn get_values(db: &Pool<Sqlite>, app_id: Option<&str>, key: Option<&str>) -> Result<Option<std::collections::HashMap<String, String>>, sqlx::Error> {
        if app_id.is_none() && key.is_none() {
            return Ok(None);
        }

        let mut values = std::collections::HashMap::new();

        if let Some(app) = app_id {
            if key.is_some() {
                return Ok(None);
            }

            let rows = query("SELECT `configkey`, `configvalue` FROM `*PREFIX*appconfig` WHERE `appid` = ?")
                .bind(app)
                .fetch_all(db)
                .await?;
            
            for row in rows {
                values.insert(row.get("configkey"), row.get("configvalue"));
            }
        } else if let Some(k) = key {
            let rows = query("SELECT `appid`, `configvalue` FROM `*PREFIX*appconfig` WHERE `configkey` = ?")
                .bind(k)
                .fetch_all(db)
                .await?;
            
            for row in rows {
                values.insert(row.get("appid"), row.get("configvalue"));
            }
        }

        Ok(Some(values))
    }
}

#[async_trait]
trait TestFixture {
    async fn setup(&self, db: &Pool<Sqlite>) -> Result<(), sqlx::Error>;
    async fn teardown(&self, db: &Pool<Sqlite>) -> Result<(), sqlx::Error>;
}

struct AppConfigFixture;

#[async_trait]
impl TestFixture for AppConfigFixture {
    async fn setup(&self, db: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let fixtures = [
            ("testapp", "enabled", "true"),
            ("testapp", "installed_version", "1.2.3"),
            ("testapp", "depends_on", "someapp"),
            ("testapp", "deletethis", "deletethis"),
            ("testapp", "key", "value"),
            ("someapp", "key", "value"),
            ("someapp", "otherkey", "othervalue"),
            ("123456", "key", "value"),
            ("123456", "enabled", "false"),
            ("anotherapp", "key", "value"),
            ("anotherapp", "enabled", "false"),
        ];

        for (app, key, value) in fixtures.iter() {
            query("INSERT INTO `*PREFIX*appconfig` VALUES (?, ?, ?)")
                .bind(app)
                .bind(key)
                .bind(value)
                .execute(db)
                .await?;
        }

        Ok(())
    }

    async fn teardown(&self, db: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let apps = ["testapp", "someapp", "123456", "anotherapp"];
        
        for app in apps.iter() {
            query("DELETE FROM `*PREFIX*appconfig` WHERE `appid` = ?")
                .bind(app)
                .execute(db)
                .await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_db() -> Pool<Sqlite> {
        let db = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(":memory:")
            .await
            .expect("Failed to create in-memory SQLite database");

        sqlx::query("CREATE TABLE IF NOT EXISTS `*PREFIX*appconfig` (
            `appid` TEXT NOT NULL,
            `configkey` TEXT NOT NULL,
            `configvalue` TEXT NOT NULL,
            PRIMARY KEY (`appid`, `configkey`)
        )")
        .execute(&db)
        .await
        .expect("Failed to create appconfig table");

        let fixture = AppConfigFixture;
        fixture.setup(&db).await.expect("Failed to setup test data");

        db
    }

    async fn teardown_db(db: Pool<Sqlite>) {
        let fixture = AppConfigFixture;
        fixture.teardown(&db).await.expect("Failed to clean up test data");
        db.close().await;
    }

    #[tokio::test]
    #[serial]
    async fn test_get_apps() {
        let db = setup_db().await;
        
        let expected_query = sqlx::query("SELECT DISTINCT `appid` FROM `*PREFIX*appconfig`")
            .fetch_all(&db)
            .await
            .unwrap();
        
        let mut expected = Vec::new();
        for row in expected_query {
            let app_id: String = row.get("appid");
            expected.push(app_id);
        }
        
        let apps = AppConfig::get_apps(&db).await.unwrap();
        assert_eq!(expected, apps);
        
        teardown_db(db).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_get_keys() {
        let db = setup_db().await;
        
        let expected_query = sqlx::query("SELECT `configkey` FROM `*PREFIX*appconfig` WHERE `appid` = ?")
            .bind("testapp")
            .fetch_all(&db)
            .await
            .unwrap();
        
        let mut expected = Vec::new();
        for row in expected_query {
            let key: String = row.get("configkey");
            expected.push(key);
        }
        
        let keys = AppConfig::get_keys(&db, "testapp").await.unwrap();
        assert_eq!(expected, keys);
        
        teardown_db(db).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_get_value() {
        let db = setup_db().await;
        
        let expected_row = sqlx::query("SELECT `configvalue` FROM `*PREFIX*appconfig` WHERE `appid` = ? AND `configkey` = ?")
            .bind("testapp")
            .bind("installed_version")
            .fetch_one(&db)
            .await
            .unwrap();
        
        let expected: String = expected_row.get("configvalue");
        
        let value = AppConfig::get_value(&db, "testapp", "installed_version").await.unwrap();
        assert_eq!(Some(expected), value);
        
        let value = AppConfig::get_value(&db, "testapp", "nonexistant").await.unwrap();
        assert_eq!(None, value);
        
        let value = AppConfig::get_value_with_default(&db, "testapp", "nonexistant", "default").await.unwrap();
        assert_eq!("default", value);
        
        teardown_db(db).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_has_key() {
        let db = setup_db().await;
        
        let value = AppConfig::has_key(&db, "testapp", "installed_version").await.unwrap();
        assert!(value);
        
        let value = AppConfig::has_key(&db, "nonexistant", "nonexistant").await.unwrap();
        assert!(!value);
        
        teardown_db(db).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_set_value() {
        let db = setup_db().await;
        
        AppConfig::set_value(&db, "testapp", "installed_version", "1.33.7").await.unwrap();
        
        let row = sqlx::query("SELECT `configvalue` FROM `*PREFIX*appconfig` WHERE `appid` = ? AND `configkey` = ?")
            .bind("testapp")
            .bind("installed_version")
            .fetch_one(&db)
            .await
            .unwrap();
        
        let value: String = row.get("configvalue");
        assert_eq!("1.33.7", value);
        
        AppConfig::set_value(&db, "someapp", "somekey", "somevalue").await.unwrap();
        
        let row = sqlx::query("SELECT `configvalue` FROM `*PREFIX*appconfig` WHERE `appid` = ? AND `configkey` = ?")
            .bind("someapp")
            .bind("somekey")
            .fetch_one(&db)
            .await
            .unwrap();
        
        let value: String = row.get("configvalue");
        assert_eq!("somevalue", value);
        
        teardown_db(db).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_key() {
        let db = setup_db().await;
        
        AppConfig::delete_key(&db, "testapp", "deletethis").await.unwrap();
        
        let row = sqlx::query("SELECT `configvalue` FROM `*PREFIX*appconfig` WHERE `appid` = ? AND `configkey` = ?")
            .bind("testapp")
            .bind("deletethis")
            .fetch_optional(&db)
            .await
            .unwrap();
        
        assert!(row.is_none());
        
        teardown_db(db).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_app() {
        let db = setup_db().await;
        
        AppConfig::delete_app(&db, "someapp").await.unwrap();
        
        let row = sqlx::query("SELECT `configkey` FROM `*PREFIX*appconfig` WHERE `appid` = ?")
            .bind("someapp")
            .fetch_optional(&db)
            .await
            .unwrap();
        
        assert!(row.is_none());
        
        teardown_db(db).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_get_values() {
        let db = setup_db().await;
        
        let value = AppConfig::get_values(&db, Some("testapp"), Some("enabled")).await.unwrap();
        assert_eq!(None, value);
        
        let rows = sqlx::query("SELECT `configkey`, `configvalue` FROM `*PREFIX*appconfig` WHERE `appid` = ?")
            .bind("testapp")
            .fetch_all(&db)
            .await
            .unwrap();
        
        let mut expected = std::collections::HashMap::new();
        for row in rows {
            let key: String = row.get("configkey");
            let value: String = row.get("configvalue");
            expected.insert(key, value);
        }
        
        let values = AppConfig::get_values(&db, Some("testapp"), None).await.unwrap();
        assert_eq!(Some(expected), values);
        
        let rows = sqlx::query("SELECT `appid`, `configvalue` FROM `*PREFIX*appconfig` WHERE `configkey` = ?")
            .bind("enabled")
            .fetch_all(&db)
            .await
            .unwrap();
        
        let mut expected = std::collections::HashMap::new();
        for row in rows {
            let app_id: String = row.get("appid");
            let value: String = row.get("configvalue");
            expected.insert(app_id, value);
        }
        
        let values = AppConfig::get_values(&db, None, Some("enabled")).await.unwrap();
        assert_eq!(Some(expected), values);
        
        teardown_db(db).await;
    }
}