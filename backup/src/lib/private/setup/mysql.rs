use async_trait::async_trait;
use mysql_async::{prelude::*, Conn, Opts, Pool, PoolOpts};
use rand::{distributions::Alphanumeric, Rng};
use std::iter;

use crate::setup::{AbstractDatabase, DatabaseSetupException};
use crate::config;
use crate::log;
use crate::util;
use crate::db;
use crate::trans::Trans;

pub struct MySql {
    pub db_pretty_name: &'static str,
    pub db_host: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub table_prefix: String,
    pub db_definition_file: String,
    pub trans: Trans,
}

impl MySql {
    pub fn new(
        db_host: String,
        db_user: String,
        db_password: String,
        db_name: String,
        table_prefix: String,
        db_definition_file: String,
        trans: Trans,
    ) -> Self {
        Self {
            db_pretty_name: "MySQL",
            db_host,
            db_user,
            db_password,
            db_name,
            table_prefix,
            db_definition_file,
            trans,
        }
    }

    async fn create_database(&self, conn: &mut Conn) -> Result<(), DatabaseSetupException> {
        let name = &self.db_name;
        let user = &self.db_user;
        
        // We can't use DB functions here because we need to connect as the administrative user
        let query = format!("CREATE DATABASE IF NOT EXISTS `{}`", name);
        
        match conn.query_drop(&query).await {
            Ok(_) => (),
            Err(err) => {
                let entry = self.trans.t(&format!("DB Error: \"{}\"", err));
                let entry = format!("{}<br />{}", entry, self.trans.t(&format!("Offending command was: \"{}\"", query)));
                log::write("setup.mysql", &entry, log::LogLevel::Warn);
            }
        }

        // This query will fail if there aren't the right permissions, ignore the error
        let grant_query = format!("GRANT ALL PRIVILEGES ON `{}` . * TO '{}'", name, user);
        let _ = conn.query_drop(&grant_query).await;
        
        Ok(())
    }

    async fn create_db_user(&self, conn: &mut Conn) -> Result<(), DatabaseSetupException> {
        let name = &self.db_user;
        let password = &self.db_password;
        
        // We need to create 2 accounts, one for global use and one for local user.
        // If we don't specify the local one, the anonymous user would take precedence when there is one.
        let query = format!("CREATE USER '{}'@'localhost' IDENTIFIED BY '{}'", name, password);
        
        if let Err(_) = conn.query_drop(&query).await {
            return Err(DatabaseSetupException::new(
                self.trans.t(&format!("MySQL user '{}'@'localhost' exists already.", name)),
                self.trans.t(&format!("Drop this user from MySQL", name)),
            ));
        }
        
        let query = format!("CREATE USER '{}'@'%' IDENTIFIED BY '{}'", name, password);
        
        if let Err(_) = conn.query_drop(&query).await {
            return Err(DatabaseSetupException::new(
                self.trans.t(&format!("MySQL user '{}'@'%%' already exists", name)),
                self.trans.t("Drop this user from MySQL."),
            ));
        }
        
        Ok(())
    }
}

#[async_trait]
impl AbstractDatabase for MySql {
    async fn setup_database(&mut self, username: &str) -> Result<(), DatabaseSetupException> {
        // Check if the database user has admin rights
        let opts = Opts::from_url(&format!("mysql://{}:{}@{}", self.db_user, self.db_password, self.db_host))
            .map_err(|e| DatabaseSetupException::new(
                self.trans.t("MySQL username and/or password not valid"),
                self.trans.t("You need to enter either an existing account or the administrator."),
            ))?;
            
        let pool = Pool::new(opts);
        let mut conn = pool.get_conn().await.map_err(|e| DatabaseSetupException::new(
            self.trans.t("MySQL username and/or password not valid"),
            self.trans.t("You need to enter either an existing account or the administrator."),
        ))?;

        let old_user = config::get_value::<String>("dbuser", None);

        // This should be enough to check for admin rights in MySQL
        let query = format!("SELECT user FROM mysql.user WHERE user='{}'", self.db_user);
        
        let has_admin_rights = match conn.query_drop(&query).await {
            Ok(_) => true,
            Err(_) => false,
        };

        if has_admin_rights {
            // Use the admin login data for the new database user
            
            // Add prefix to the MySQL user name to prevent collisions
            let new_db_user = format!("oc_{}", username).chars().take(16).collect::<String>();
            
            if old_user.as_deref() != Some(&new_db_user) {
                // Hash the password so we don't need to store the admin config in the config file
                self.db_user = new_db_user;
                self.db_password = util::generate_random_bytes(30);

                self.create_db_user(&mut conn).await?;

                config::set_value("dbuser", &self.db_user);
                config::set_value("dbpassword", &self.db_password);
            }

            // Create the database
            self.create_database(&mut conn).await?;
        } else {
            if old_user.as_deref() != Some(&self.db_user) {
                config::set_value("dbuser", &self.db_user);
                config::set_value("dbpassword", &self.db_password);
            }

            // Create the database
            self.create_database(&mut conn).await?;
        }

        // Fill the database if needed
        let query = format!(
            "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema='{}' AND table_name = '{}'users",
            self.db_name, self.table_prefix
        );
        
        let result: Option<(i64,)> = conn.query_first(&query).await.unwrap_or(None);
        
        if result.map_or(true, |(count,)| count == 0) {
            db::create_db_from_structure(&self.db_definition_file).await?;
        }

        Ok(())
    }
}