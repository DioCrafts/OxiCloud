// ownCloud
//
// @author Jakob Sack
// @copyright 2012 Jakob Sack owncloud@jakobsack.de
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::fs::{self, File};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::process;
use log::{error, info};

// Equivalent to the PHP TemporaryCronClass
struct CronState {
    sent: AtomicBool,
    lockfile: String,
    keeplock: AtomicBool,
}

impl CronState {
    fn new(lockfile: String) -> Self {
        Self {
            sent: AtomicBool::new(false),
            lockfile,
            keeplock: AtomicBool::new(false),
        }
    }
}

// Equivalent to BackgroundJob execution types
enum ExecutionType {
    None,
    Cron,
    Ajax,
}

// Simplified Job and JobList representations
trait Job {
    fn execute(&self, job_list: &JobList);
}

struct JobList {
    // Fields would be defined based on the actual OC\BackgroundJob\JobList implementation
}

impl JobList {
    fn new() -> Self {
        Self {}
    }

    fn get_all(&self) -> Vec<Box<dyn Job>> {
        // Implementation would fetch all jobs
        Vec::new()
    }

    fn get_next(&self) -> Box<dyn Job> {
        // Implementation would fetch the next job
        unimplemented!("Get next job")
    }

    fn set_last_job(&self, _job: Box<dyn Job>) {
        // Implementation would update the last executed job
    }
}

// Configuration access
struct Config;

impl Config {
    fn get_value<T>(key: &str, default: T) -> T {
        // Implementation would fetch configuration values
        default
    }
}

// Helper functions
struct Helper;

impl Helper {
    fn clean_tmp_no_clean() {
        // Implementation would clean temporary files
    }
}

// JSON response handling
struct JSON;

impl JSON {
    fn error(data: serde_json::Value) {
        println!("{}", serde_json::to_string(&data).unwrap());
    }

    fn success() {
        println!("{}", serde_json::to_string(&serde_json::json!({"status": "success"})).unwrap());
    }
}

// Main OC utilities
struct OC;

impl OC {
    const CLI: bool = true; // This would be determined at runtime
    static SERVERROOT: &'static str = "/path/to/server"; // This would be determined at runtime
}

// BackgroundJob management
struct BackgroundJob;

impl BackgroundJob {
    fn get_execution_type() -> ExecutionType {
        // Implementation would determine the current execution type
        ExecutionType::Cron
    }

    fn set_execution_type(execution_type: ExecutionType) {
        // Implementation would set the execution type
        match execution_type {
            ExecutionType::Cron => {
                // Set to cron mode
            }
            _ => {}
        }
    }
}

// Utility for logging
struct Util;

impl Util {
    const FATAL: u8 = 4;

    fn write_log(app: &str, message: &str, level: u8) {
        // Implementation would log messages
        match level {
            Self::FATAL => error!("[{}] {}", app, message),
            _ => info!("[{}] {}", app, message),
        }
    }
}

// Function to handle unexpected shutdowns
fn handle_unexpected_shutdown(state: &CronState) {
    // Delete lockfile
    if !state.keeplock.load(Ordering::SeqCst) && Path::new(&state.lockfile).exists() {
        if let Err(e) = fs::remove_file(&state.lockfile) {
            error!("Failed to remove lock file: {}", e);
        }
    }

    // Say goodbye if the app did not shutdown properly
    if !state.sent.load(Ordering::SeqCst) {
        if OC::CLI {
            println!("Unexpected error!");
        } else {
            JSON::error(serde_json::json!({"data": {"message": "Unexpected error!"}}));
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the state with default values
    let data_dir = Config::get_value("datadirectory", OC::SERVERROOT.to_owned() + "/data");
    let state = Arc::new(CronState::new(format!("{}/cron.lock", data_dir)));
    
    // Clone for the shutdown handler
    let state_clone = Arc::clone(&state);
    
    // Set up the shutdown handler
    ctrlc::set_handler(move || {
        handle_unexpected_shutdown(&state_clone);
        process::exit(1);
    })?;

    // Don't do anything if ownCloud has not been installed
    if !Config::get_value::<bool>("installed", false) {
        return Ok(());
    }

    // Delete temp folder
    Helper::clean_tmp_no_clean();

    // Exit if background jobs are disabled!
    let app_mode = BackgroundJob::get_execution_type();
    if let ExecutionType::None = app_mode {
        state.sent.store(true, Ordering::SeqCst);
        if OC::CLI {
            println!("Background Jobs are disabled!");
        } else {
            JSON::error(serde_json::json!({"data": {"message": "Background jobs disabled!"}}));
        }
        process::exit(1);
    }

    if OC::CLI {
        // Create lock file first
        let lockfile = &state.lockfile;

        // We call ownCloud from the CLI (aka cron)
        if let ExecutionType::Cron = app_mode {
            // We're already in cron mode
        } else {
            // Use cron in future!
            BackgroundJob::set_execution_type(ExecutionType::Cron);
        }

        // check if backgroundjobs is still running
        if Path::new(lockfile).exists() {
            state.keeplock.store(true, Ordering::SeqCst);
            state.sent.store(true, Ordering::SeqCst);
            println!("Another instance of cron.php is still running!");
            return Ok(());
        }

        // Create a lock file
        File::create(lockfile)?;

        // Work
        let job_list = JobList::new();
        let jobs = job_list.get_all();
        for job in jobs {
            job.execute(&job_list);
        }
    } else {
        // We call cron.php from some website
        match app_mode {
            ExecutionType::Cron => {
                // Cron is cron :-P
                JSON::error(serde_json::json!({"data": {"message": "Backgroundjobs are using system cron!"}}));
            }
            _ => {
                // Work and success :-)
                let job_list = JobList::new();
                let job = job_list.get_next();
                job.execute(&job_list);
                job_list.set_last_job(job);
                JSON::success();
            }
        }
    }

    // done!
    state.sent.store(true, Ordering::SeqCst);
    
    // Clean up the lock file if it exists and we haven't decided to keep it
    if !state.keeplock.load(Ordering::SeqCst) && Path::new(&state.lockfile).exists() {
        fs::remove_file(&state.lockfile)?;
    }
    
    Ok(())
}