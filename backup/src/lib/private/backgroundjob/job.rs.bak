// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::fmt::Debug;

/// Trait for job lists that can track job execution
pub trait JobList {
    /// Records that a job was just run
    fn set_last_run(&self, job: &dyn JobTrait);
}

/// The argument type used for jobs
pub type JobArgument = Option<serde_json::Value>;

/// Core trait defining a background job
pub trait JobTrait: Send + Sync {
    /// Run the job with the given argument
    fn run(&self, argument: JobArgument);
    
    /// Get the job's unique identifier
    fn id(&self) -> Option<i64>;
    
    /// Set the job's unique identifier
    fn set_id(&mut self, id: i64);
    
    /// Get the timestamp of the last run
    fn last_run(&self) -> Option<i64>;
    
    /// Set the timestamp of the last run
    fn set_last_run(&mut self, last_run: i64);
    
    /// Get the job's argument
    fn argument(&self) -> JobArgument;
    
    /// Set the job's argument
    fn set_argument(&mut self, argument: JobArgument);
    
    /// Execute the job and update the job list
    fn execute(&self, job_list: &dyn JobList) {
        job_list.set_last_run(self);
        self.run(self.argument());
    }
}

/// Base implementation for background jobs
pub struct Job {
    id: Option<i64>,
    last_run: Option<i64>,
    argument: JobArgument,
}

impl Default for Job {
    fn default() -> Self {
        Self {
            id: None,
            last_run: None,
            argument: None,
        }
    }
}

impl Job {
    /// Create a new job instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new job with the specified argument
    pub fn with_argument(argument: JobArgument) -> Self {
        Self {
            id: None,
            last_run: None,
            argument,
        }
    }
}

impl JobTrait for Job {
    fn run(&self, _argument: JobArgument) {
        // Base implementation does nothing, should be overridden by derived types
    }

    fn id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }

    fn last_run(&self) -> Option<i64> {
        self.last_run
    }

    fn set_last_run(&mut self, last_run: i64) {
        self.last_run = Some(last_run);
    }

    fn argument(&self) -> JobArgument {
        self.argument.clone()
    }

    fn set_argument(&mut self, argument: JobArgument) {
        self.argument = argument;
    }
}