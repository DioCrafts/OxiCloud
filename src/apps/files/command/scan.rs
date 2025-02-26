//! Copyright (c) 2013 Thomas Müller <thomas.mueller@tmit.eu>
//! Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.

use async_trait::async_trait;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::sync::Arc;

pub struct Scan {
    user_manager: Arc<dyn UserManager>,
}

#[async_trait]
pub trait UserManager: Send + Sync {
    async fn search(&self, query: &str) -> Vec<User>;
}

#[async_trait]
pub trait Command: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    
    fn configure<'a, 'b>(&self, app: App<'a, 'b>) -> App<'a, 'b>;
    
    async fn execute(&self, matches: &ArgMatches<'_>, output: &mut dyn Output) -> Result<(), CommandError>;
}

pub trait Output: Send + Sync {
    fn writeln(&self, message: &str);
}

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("User not found: {0}")]
    UserNotFound(String),
    
    #[error("Scan error: {0}")]
    ScanError(String),
    
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

pub struct User {
    uid: String,
}

impl User {
    pub fn get_uid(&self) -> &str {
        &self.uid
    }
}

#[async_trait]
pub trait Scanner: Send + Sync {
    fn on_scan_file<F>(&mut self, callback: F)
    where
        F: FnMut(&str) + Send + 'static;
    
    fn on_scan_folder<F>(&mut self, callback: F)
    where
        F: FnMut(&str) + Send + 'static;
    
    async fn scan(&mut self, path: &str) -> Result<(), CommandError>;
}

impl Scan {
    pub fn new(user_manager: Arc<dyn UserManager>) -> Self {
        Self { user_manager }
    }

    async fn scan_files(&self, user: &str, output: &dyn Output) -> Result<(), CommandError> {
        let mut scanner = FilesScanner::new(user);
        
        scanner.on_scan_file(|path| {
            output.writeln(&format!("Scanning {}", path));
        });
        
        scanner.on_scan_folder(|path| {
            output.writeln(&format!("Scanning {}", path));
        });
        
        scanner.scan("").await
    }
}

struct FilesScanner {
    user: String,
    scan_file_callback: Option<Box<dyn FnMut(&str) + Send>>,
    scan_folder_callback: Option<Box<dyn FnMut(&str) + Send>>,
}

impl FilesScanner {
    fn new(user: &str) -> Self {
        Self {
            user: user.to_string(),
            scan_file_callback: None,
            scan_folder_callback: None,
        }
    }
}

#[async_trait]
impl Scanner for FilesScanner {
    fn on_scan_file<F>(&mut self, callback: F)
    where
        F: FnMut(&str) + Send + 'static,
    {
        self.scan_file_callback = Some(Box::new(callback));
    }
    
    fn on_scan_folder<F>(&mut self, callback: F)
    where
        F: FnMut(&str) + Send + 'static,
    {
        self.scan_folder_callback = Some(Box::new(callback));
    }
    
    async fn scan(&mut self, path: &str) -> Result<(), CommandError> {
        // Implementation would go here
        // For demonstration purposes, we'll just call the callbacks
        if let Some(ref mut callback) = self.scan_folder_callback {
            callback(path);
        }
        
        Ok(())
    }
}

#[async_trait]
impl Command for Scan {
    fn name(&self) -> &'static str {
        "files:scan"
    }
    
    fn description(&self) -> &'static str {
        "rescan filesystem"
    }
    
    fn configure<'a, 'b>(&self, app: App<'a, 'b>) -> App<'a, 'b> {
        app.about(self.description())
            .arg(
                Arg::with_name("user_id")
                    .help("will rescan all files of the given user(s)")
                    .multiple(true),
            )
            .arg(
                Arg::with_name("all")
                    .long("all")
                    .help("will rescan all files of all known users")
                    .takes_value(false),
            )
    }
    
    async fn execute(&self, matches: &ArgMatches<'_>, output: &mut dyn Output) -> Result<(), CommandError> {
        let users: Vec<String> = if matches.is_present("all") {
            let users = self.user_manager.search("").await;
            users.into_iter().map(|u| u.get_uid().to_string()).collect()
        } else {
            matches
                .values_of("user_id")
                .map(|values| values.map(String::from).collect())
                .unwrap_or_default()
        };

        for user in users {
            self.scan_files(&user, output).await?;
        }
        
        Ok(())
    }
}