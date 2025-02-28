// tests/lib/backgroundjob/dummy_job_list.rs

use std::error::Error;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a job execution exception
#[derive(Debug)]
pub struct JobRun {
    message: String,
}

impl JobRun {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for JobRun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JobRun: {}", self.message)
    }
}

impl Error for JobRun {}

/// Trait representing a background job
pub trait Job: PartialEq + Send + Sync {
    fn get_id(&self) -> usize;
    fn set_argument(&mut self, argument: Option<String>);
    fn set_last_run(&mut self, timestamp: u64);
}

/**
 * DummyJobList
 *
 * In-memory job list for testing purposes
 */
pub struct DummyJobList {
    /// Collection of background jobs
    jobs: Vec<Box<dyn Job>>,
    /// Index of the last executed job
    last: usize,
}

impl DummyJobList {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            last: 0,
        }
    }

    /// Add a job to the list
    pub fn add(&mut self, mut job: Box<dyn Job>, argument: Option<String>) {
        job.set_argument(argument);
        if !self.has(&*job, None) {
            self.jobs.push(job);
        }
    }

    /// Remove a job from the list
    pub fn remove(&mut self, job: &dyn Job, _argument: Option<String>) {
        if let Some(index) = self.jobs.iter().position(|j| *j == job) {
            self.jobs.remove(index);
        }
    }

    /// Check if a job is in the list
    ///
    /// # Parameters
    /// * `job` - The job to check
    /// * `_argument` - Optional argument (not used in this implementation)
    ///
    /// # Returns
    /// `true` if the job is in the list, `false` otherwise
    pub fn has(&self, job: &dyn Job, _argument: Option<String>) -> bool {
        self.jobs.iter().any(|j| &**j == job)
    }

    /// Get all jobs in the list
    ///
    /// # Returns
    /// All jobs in the list
    pub fn get_all(&self) -> &[Box<dyn Job>] {
        &self.jobs
    }

    /// Get the next job in the list
    ///
    /// # Returns
    /// The next job in the list, or None if there are no jobs
    pub fn get_next(&self) -> Option<&Box<dyn Job>> {
        if self.jobs.is_empty() {
            return None;
        }

        let i = if self.last < (self.jobs.len() - 1) {
            self.last + 1
        } else {
            0
        };
        
        self.jobs.get(i)
    }

    /// Set the job that was last ran
    ///
    /// # Parameters
    /// * `job` - The job that was last ran
    pub fn set_last_job(&mut self, job: &dyn Job) {
        if let Some(i) = self.jobs.iter().position(|j| &**j == job) {
            self.last = i;
        } else {
            self.last = 0;
        }
    }

    /// Get a job by its ID
    ///
    /// # Parameters
    /// * `id` - The ID of the job to get
    ///
    /// # Returns
    /// The job with the given ID, or None if no job has that ID
    pub fn get_by_id(&self, id: usize) -> Option<&Box<dyn Job>> {
        self.jobs.iter().find(|job| job.get_id() == id)
    }

    /// Get the index of the last ran job
    ///
    /// # Returns
    /// The index of the last ran job
    pub fn get_last_job(&self) -> usize {
        self.last
    }

    /// Set the lastRun of a job to now
    ///
    /// # Parameters
    /// * `job` - The job to update
    pub fn set_last_run(&mut self, job: &mut dyn Job) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        job.set_last_run(now);
    }
}

impl Default for DummyJobList {
    fn default() -> Self {
        Self::new()
    }
}