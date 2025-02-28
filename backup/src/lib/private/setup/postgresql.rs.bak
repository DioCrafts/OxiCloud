use crate::setup::abstract_database::AbstractDatabase;
use crate::config;
use crate::db;
use crate::util;
use crate::log;
use postgres::{Client, NoTls, Error as PgError};
use std::fmt::Write as FmtWrite;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseSetupError {
    #[error("PostgreSQL username and/or password not valid")]
    InvalidCredentials,
    #[error("DB Error: {0}")]
    DbError(#[from] PgError),
    #[error("Connection failed")]
    ConnectionFailed,
    #[error("{0}")]
    Other(String),
}

pub struct PostgreSQL {
    db_pretty_name: &'static str,
    db_host: String,
    db_user: String,
    db_password: String,
    db_name: String,
    table_prefix: String,
    db_definition_file: String,
    trans: Box<dyn Translator>,
}

impl PostgreSQL {
    pub fn new(
        db_host: String, 
        db_user: String, 
        db_password: String, 
        db_name: String,
        table_prefix: String,
        db_definition_file: String,
        trans: Box<dyn Translator>,
    ) -> Self {
        Self {
            db_pretty_name: "PostgreSQL",
            db_host,
            db_user,
            db_password,
            db_name,
            table_prefix,
            db_definition_file,
            trans,
        }
    }

    pub fn setup_database(&mut self, username: &str) -> Result<(), DatabaseSetupError> {
        // Try to connect to postgres database first
        let connection_string = format!(
            "host='{}' dbname=postgres user='{}' password='{}'",
            self.db_host, self.db_user, self.db_password
        );

        let mut admin_conn = match Client::connect(&connection_string, NoTls) {
            Ok(conn) => conn,
            Err(_) => {
                // Try if we can connect to the DB with the specified name
                let connection_string = format!(
                    "host='{}' dbname='{}' user='{}' password='{}'",
                    self.db_host, self.db_name, self.db_user, self.db_password
                );
                
                match Client::connect(&connection_string, NoTls) {
                    Ok(conn) => conn,
                    Err(_) => {
                        return Err(DatabaseSetupError::InvalidCredentials);
                    }
                }
            }
        };

        // Check for roles creation rights in postgresql
        let query = format!(
            "SELECT 1 FROM pg_roles WHERE rolcreaterole=TRUE AND rolname='{}'",
            pg_escape_string(&self.db_user)
        );
        
        let has_admin_rights = match admin_conn.query(&query, &[]) {
            Ok(result) => !result.is_empty(),
            Err(_) => false,
        };

        if has_admin_rights {
            // Use the admin login data for the new database user
            // Add prefix to the postgresql user name to prevent collisions
            self.db_user = format!("oc_{}", username);
            // Create a new password so we don't need to store the admin config in the config file
            self.db_password = util::generate_random_bytes(30);

            self.create_db_user(&mut admin_conn)?;

            config::set_value("dbuser", &self.db_user);
            config::set_value("dbpassword", &self.db_password);

            // Create the database
            self.create_database(&mut admin_conn)?;
        } else {
            config::set_value("dbuser", &self.db_user);
            config::set_value("dbpassword", &self.db_password);

            // Create the database
            self.create_database(&mut admin_conn)?;
        }

        // The connection to dbname=postgres is not needed anymore
        drop(admin_conn);

        // Connect to the ownCloud database (dbname=self.db_name) and check if it needs to be filled
        self.db_user = config::get_value("dbuser").unwrap_or_default();
        self.db_password = config::get_value("dbpassword").unwrap_or_default();

        let connection_string = format!(
            "host='{}' dbname='{}' user='{}' password='{}'",
            self.db_host, self.db_name, self.db_user, self.db_password
        );
        
        let mut conn = match Client::connect(&connection_string, NoTls) {
            Ok(conn) => conn,
            Err(_) => {
                return Err(DatabaseSetupError::InvalidCredentials);
            }
        };

        let query = format!(
            "select count(*) FROM pg_class WHERE relname='{}users' limit 1",
            self.table_prefix
        );
        
        let table_exists = match conn.query(&query, &[]) {
            Ok(result) => {
                if let Some(row) = result.get(0) {
                    let count: i64 = row.get(0);
                    count > 0
                } else {
                    false
                }
            },
            Err(_) => false,
        };

        if !table_exists {
            db::create_db_from_structure(&self.db_definition_file)?;
        }

        Ok(())
    }

    fn create_database(&self, conn: &mut Client) -> Result<(), DatabaseSetupError> {
        let e_name = pg_escape_string(&self.db_name);
        let e_user = pg_escape_string(&self.db_user);
        
        let query = format!("select datname from pg_database where datname = '{}'", e_name);
        
        let result = match conn.query(&query, &[]) {
            Ok(result) => result,
            Err(e) => {
                let mut entry = format!("DB Error: \"{}\"", e);
                write!(entry, "\nOffending command was: \"{}\"", query).unwrap();
                log::write("setup.pg", &entry, log::Level::Warn);
                return Err(DatabaseSetupError::DbError(e));
            }
        };

        if result.is_empty() {
            // The database does not exist... let's create it
            let query = format!("CREATE DATABASE \"{}\" OWNER \"{}\"", e_name, e_user);
            
            match conn.execute(&query, &[]) {
                Ok(_) => {
                    let query = format!("REVOKE ALL PRIVILEGES ON DATABASE \"{}\" FROM PUBLIC", e_name);
                    let _ = conn.execute(&query, &[]);
                },
                Err(e) => {
                    let mut entry = format!("DB Error: \"{}\"", e);
                    write!(entry, "\nOffending command was: \"{}\"", query).unwrap();
                    log::write("setup.pg", &entry, log::Level::Warn);
                    return Err(DatabaseSetupError::DbError(e));
                }
            }
        }

        Ok(())
    }

    fn create_db_user(&self, conn: &mut Client) -> Result<(), DatabaseSetupError> {
        let e_name = pg_escape_string(&self.db_user);
        let e_password = pg_escape_string(&self.db_password);
        
        let query = format!("select * from pg_roles where rolname='{}';", e_name);
        
        let result = match conn.query(&query, &[]) {
            Ok(result) => result,
            Err(e) => {
                let mut entry = format!("DB Error: \"{}\"", e);
                write!(entry, "\nOffending command was: \"{}\"", query).unwrap();
                log::write("setup.pg", &entry, log::Level::Warn);
                return Err(DatabaseSetupError::DbError(e));
            }
        };

        let query = if result.is_empty() {
            // User does not exist, let's create it
            format!("CREATE USER \"{}\" CREATEDB PASSWORD '{}';", e_name, e_password)
        } else {
            // Change password of the existing role
            format!("ALTER ROLE \"{}\" WITH PASSWORD '{}';", e_name, e_password)
        };
        
        match conn.execute(&query, &[]) {
            Ok(_) => Ok(()),
            Err(e) => {
                let mut entry = format!("DB Error: \"{}\"", e);
                write!(entry, "\nOffending command was: \"{}\"", query).unwrap();
                log::write("setup.pg", &entry, log::Level::Warn);
                Err(DatabaseSetupError::DbError(e))
            }
        }
    }
}

impl AbstractDatabase for PostgreSQL {
    fn db_pretty_name(&self) -> &str {
        self.db_pretty_name
    }
    
    // Implement other required methods from AbstractDatabase trait
}

// Helper function to escape strings for Postgres
fn pg_escape_string(s: &str) -> String {
    s.replace('\'', "''")
}

pub trait Translator {
    fn t(&self, text: &str) -> String;
    fn t_with_params(&self, text: &str, params: &[&str]) -> String;
}