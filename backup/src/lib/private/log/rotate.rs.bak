// Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::fs;
use std::path::Path;

/// This rotates the current logfile to a new name, this way the total log usage
/// will stay limited and older entries are available for a while longer.
/// For more professional log management set the 'logfile' config to a different
/// location and manage that with your own tools.
pub struct Rotate {
    max_log_size: Option<u64>,
}

impl Rotate {
    pub fn new() -> Self {
        Self {
            max_log_size: None,
        }
    }

    pub fn run<P: AsRef<Path>>(&mut self, log_file: P) -> Result<(), std::io::Error> {
        self.max_log_size = config::get_value("log_rotate_size");

        if let Some(max_size) = self.max_log_size {
            match fs::metadata(&log_file) {
                Ok(metadata) => {
                    let file_size = metadata.len();
                    if file_size >= max_size {
                        self.rotate(log_file)?;
                    }
                }
                Err(_) => {
                    // File might not exist yet, which is fine
                }
            }
        }

        Ok(())
    }

    fn rotate<P: AsRef<Path>>(&self, log_file: P) -> Result<(), std::io::Error> {
        let log_path = log_file.as_ref();
        let rotated_log_file = format!("{}.1", log_path.display());
        
        fs::rename(&log_file, &rotated_log_file)?;
        
        let msg = format!(
            "Log file \"{}\" was over {} bytes, moved to \"{}\"",
            log_path.display(),
            self.max_log_size.unwrap_or(0),
            rotated_log_file
        );
        
        log::warn!("{}", msg);
        Ok(())
    }
}

impl background_job::Job for Rotate {
    fn execute(&mut self, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(log_file) = args.first() {
            self.run(*log_file)?;
        } else {
            return Err("No log file provided".into());
        }
        Ok(())
    }
}

mod config {
    pub fn get_value(key: &str) -> Option<u64> {
        // This is a placeholder for the OC_Config::getValue functionality
        // In a real implementation, this would fetch from a config system
        match key {
            "log_rotate_size" => Some(1024 * 1024), // Default 1MB
            _ => None,
        }
    }
}

mod background_job {
    pub trait Job {
        fn execute(&mut self, args: &[&str]) -> Result<(), Box<dyn std::error::Error>>;
    }
}