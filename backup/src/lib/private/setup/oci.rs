use std::env;
use std::fs;
use std::path::Path;
use thiserror::Error;

use crate::setup::abstract_database::AbstractDatabase;
use crate::config;
use crate::log;
use crate::util;
use crate::db;

#[derive(Error, Debug)]
pub enum DatabaseSetupError {
    #[error("Oracle connection could not be established: {0}")]
    ConnectionFailed(String),
    
    #[error("Oracle username and/or password not valid: {0}")]
    InvalidCredentials(String),
    
    #[error("DB Error: {0}")]
    DbError(String),
    
    #[error("{0}")]
    Other(String),
}

pub struct OCI {
    pub db_pretty_name: &'static str,
    db_tablespace: String,
    i18n: crate::l10n::Translator,
}

impl Default for OCI {
    fn default() -> Self {
        Self {
            db_pretty_name: "Oracle",
            db_tablespace: String::from("USERS"),
            i18n: crate::l10n::Translator::new(),
        }
    }
}

impl AbstractDatabase for OCI {
    fn initialize(&mut self, config: &std::collections::HashMap<String, String>) -> Result<(), DatabaseSetupError> {
        self.db_host = config.get("dbhost").cloned().unwrap_or_default();
        self.db_name = config.get("dbname").cloned().unwrap_or_default();
        self.db_user = config.get("dbuser").cloned().unwrap_or_default();
        self.db_password = config.get("dbpassword").cloned().unwrap_or_default();
        self.db_table_prefix = config.get("dbtableprefix").cloned().unwrap_or_default();
        self.db_definition_file = config.get("dbdefinitionfile").cloned().unwrap_or_default();
        
        self.db_tablespace = config.get("dbtablespace").cloned().unwrap_or_else(|| String::from("USERS"));
        config::set_value("dbtablespace", &self.db_tablespace);
        
        Ok(())
    }

    fn setup_database(&self, username: &str) -> Result<(), DatabaseSetupError> {
        let e_host = self.db_host.replace("'", "''");
        let e_dbname = self.db_name.replace("'", "''");
        
        // Check if the database user has admin right
        let easy_connect_string = if e_host.is_empty() {
            e_dbname.clone() // use dbname as easy connect name
        } else {
            format!("//{}/{}", e_host, e_dbname)
        };
        
        log::write("setup oracle", &format!("connect string: {}", easy_connect_string), log::Level::Debug);
        
        let connection = oracle::Connection::connect(&self.db_user, &self.db_password, &easy_connect_string)
            .map_err(|e| {
                let env_info = format!(
                    "Check environment: ORACLE_HOME={} ORACLE_SID={} LD_LIBRARY_PATH={} NLS_LANG={} tnsnames.ora is {}readable",
                    env::var("ORACLE_HOME").unwrap_or_default(),
                    env::var("ORACLE_SID").unwrap_or_default(),
                    env::var("LD_LIBRARY_PATH").unwrap_or_default(),
                    env::var("NLS_LANG").unwrap_or_default(),
                    if Path::new(&format!("{}/network/admin/tnsnames.ora", env::var("ORACLE_HOME").unwrap_or_default())).is_readable() { "" } else { "not " }
                );
                
                DatabaseSetupError::InvalidCredentials(env_info)
            })?;
        
        // Check for roles creation rights in oracle
        let query = "SELECT count(*) FROM user_role_privs, role_sys_privs WHERE user_role_privs.granted_role = role_sys_privs.role AND privilege = 'CREATE ROLE'";
        
        let result = connection.query_row(query, &[], |row| row.get::<_, i64>(0))
            .map_err(|e| {
                let entry = format!("DB Error: \"{}\"", e);
                let entry = format!("{}\nOffending command was: \"{}\"", entry, query);
                log::write("setup.oci", &entry, log::Level::Warn);
                DatabaseSetupError::DbError(e.to_string())
            })?;
        
        let db_user;
        let db_password;
        
        if result > 0 {
            // Use the admin login data for the new database user
            
            // Add prefix to the oracle user name to prevent collisions
            db_user = format!("oc_{}", username);
            
            // Create a new password so we don't need to store the admin config in the config file
            let mut password = util::generate_random_bytes(30);
            // Oracle passwords are treated as identifiers:
            // Must start with alphanumeric char
            // Needs to be shortened to 30 bytes, as the two " needed to escape the identifier count towards the identifier length
            db_password = password.truncate(30);
            
            self.create_db_user(&connection, &db_user, &db_password)?;
            
            config::set_value("dbuser", &db_user);
            config::set_value("dbname", &db_user);
            config::set_value("dbpassword", &db_password);
        } else {
            db_user = self.db_user.clone();
            db_password = self.db_password.clone();
            
            config::set_value("dbuser", &db_user);
            config::set_value("dbname", &self.db_name);
            config::set_value("dbpassword", &db_password);
        }
        
        // FIXME check tablespace exists: select * from user_tablespaces
        
        // The connection to dbname=oracle is not needed anymore
        drop(connection);
        
        // Connect to the oracle database (schema=$this->dbuser) and check if the schema needs to be filled
        let db_user = config::get_value("dbuser").unwrap_or_default();
        let db_password = config::get_value("dbpassword").unwrap_or_default();
        
        let easy_connect_string = if e_host.is_empty() {
            e_dbname.clone()
        } else {
            format!("//{}/{}", e_host, e_dbname)
        };
        
        let connection = oracle::Connection::connect(&db_user, &db_password, &easy_connect_string)
            .map_err(|_| {
                DatabaseSetupError::InvalidCredentials(
                    self.i18n.t("You need to enter either an existing account or the administrator.")
                )
            })?;
        
        let query = "SELECT count(*) FROM user_tables WHERE table_name = :un";
        let un = format!("{}users", self.db_table_prefix);
        
        let count = connection.query_row(query, &[(":un", &un)], |row| row.get::<_, i64>(0))
            .map_err(|e| {
                let entry = format!("DB Error: \"{}\"", e);
                let entry = format!("{}\nOffending command was: \"{}\"", entry, query);
                log::write("setup.oci", &entry, log::Level::Warn);
                DatabaseSetupError::DbError(e.to_string())
            })?;
        
        if count == 0 {
            db::create_db_from_structure(&self.db_definition_file)?;
        }
        
        Ok(())
    }
}

impl OCI {
    /// Creates a new database user
    ///
    /// @param connection The database connection
    /// @param name The username
    /// @param password The user's password
    fn create_db_user(&self, connection: &oracle::Connection, name: &str, password: &str) -> Result<(), DatabaseSetupError> {
        let query = "SELECT * FROM all_users WHERE USERNAME = :un";
        
        let rows = connection.query(query, &[(":un", name)])
            .map_err(|e| {
                let entry = format!("DB Error: \"{}\"", e);
                let entry = format!("{}\nOffending command was: \"{}\"", entry, query);
                log::write("setup.oci", &entry, log::Level::Warn);
                DatabaseSetupError::DbError(e.to_string())
            })?;
        
        if rows.count() == 0 {
            // User does not exist, let's create it
            // Password must start with alphabetic character in oracle
            let query = format!("CREATE USER {} IDENTIFIED BY \"{}\" DEFAULT TABLESPACE {}", 
                name, password, self.db_tablespace);
            
            connection.execute(&query, &[])
                .map_err(|e| {
                    let entry = format!("DB Error: \"{}\"", e);
                    let entry = format!("{}\nOffending command was: \"{}\", name: {}, password: {}", 
                        entry, query, name, password);
                    log::write("setup.oci", &entry, log::Level::Warn);
                    DatabaseSetupError::DbError(e.to_string())
                })?;
        } else {
            // Change password of the existing role
            let query = "ALTER USER :un IDENTIFIED BY :pw";
            
            connection.execute(query, &[(":un", name), (":pw", password)])
                .map_err(|e| {
                    let entry = format!("DB Error: \"{}\"", e);
                    let entry = format!("{}\nOffending command was: \"{}\"", entry, query);
                    log::write("setup.oci", &entry, log::Level::Warn);
                    DatabaseSetupError::DbError(e.to_string())
                })?;
        }
        
        // Grant necessary roles
        let query = format!("GRANT CREATE SESSION, CREATE TABLE, CREATE SEQUENCE, CREATE TRIGGER, UNLIMITED TABLESPACE TO {}", name);
        
        connection.execute(&query, &[])
            .map_err(|e| {
                let entry = format!("DB Error: \"{}\"", e);
                let entry = format!("{}\nOffending command was: \"{}\", name: {}, password: {}", 
                    entry, query, name, password);
                log::write("setup.oci", &entry, log::Level::Warn);
                DatabaseSetupError::DbError(e.to_string())
            })?;
        
        Ok(())
    }
}