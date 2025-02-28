// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use crate::backgroundjob::job::{Job, JobList};
use std::time::{SystemTime, UNIX_EPOCH};

/// Abstract timed job class
///
/// Create a background job that is to be executed at an interval
pub trait TimedJob: Job {
    /// The interval in seconds between job executions
    fn interval(&self) -> u64;

    /// Set the interval for the job
    ///
    /// # Arguments
    ///
    /// * `interval` - The interval in seconds
    fn set_interval(&mut self, interval: u64);

    /// Run the job if the interval has passed
    ///
    /// # Arguments
    ///
    /// * `job_list` - The job list that manages this job
    fn execute(&self, job_list: &mut dyn JobList) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        if (now - self.last_run()) > self.interval() {
            job_list.set_last_run(self);
            self.run(&self.argument());
        }
    }
}