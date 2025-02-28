// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::any::Any;
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use async_trait::async_trait;

#[derive(Debug)]
pub struct JobRun;

impl Display for JobRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Job was run")
    }
}

impl std::error::Error for JobRun {}

#[async_trait]
pub trait Job: Send + Sync {
    async fn execute(&self, job_list: &mut dyn JobList) -> Result<(), Box<dyn std::error::Error>>;
    fn as_any(&self) -> &dyn Any;
}

#[async_trait]
pub trait QueuedJob: Job {
    async fn run(&self, argument: Option<&dyn Any>) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
impl<T: QueuedJob + Send + Sync + 'static> Job for T {
    async fn execute(&self, job_list: &mut dyn JobList) -> Result<(), Box<dyn std::error::Error>> {
        let result = self.run(None).await;
        job_list.remove(self, None);
        result
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait JobList: Send + Sync {
    fn add(&mut self, job: Box<dyn Job>);
    fn has(&self, job: &dyn Job, argument: Option<&dyn Any>) -> bool;
    fn remove(&mut self, job: &dyn Job, argument: Option<&dyn Any>);
}

struct DummyJobList {
    jobs: HashSet<usize>,
}

impl DummyJobList {
    fn new() -> Self {
        Self {
            jobs: HashSet::new(),
        }
    }
}

impl JobList for DummyJobList {
    fn add(&mut self, job: Box<dyn Job>) {
        let ptr = job.as_any() as *const dyn Any as usize;
        self.jobs.insert(ptr);
    }

    fn has(&self, job: &dyn Job, _argument: Option<&dyn Any>) -> bool {
        let ptr = job.as_any() as *const dyn Any as usize;
        self.jobs.contains(&ptr)
    }

    fn remove(&mut self, job: &dyn Job, _argument: Option<&dyn Any>) {
        let ptr = job.as_any() as *const dyn Any as usize;
        self.jobs.remove(&ptr);
    }
}

struct TestQueuedJob;

#[async_trait]
impl QueuedJob for TestQueuedJob {
    async fn run(&self, _argument: Option<&dyn Any>) -> Result<(), Box<dyn std::error::Error>> {
        // Throw an exception so we can detect if this function is called
        Err(Box::new(JobRun))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_job_should_be_removed() {
        let mut job_list = DummyJobList::new();
        let job = Box::new(TestQueuedJob);
        
        job_list.add(job.clone());
        
        assert!(job_list.has(job.as_ref(), None));
        
        let result = job.execute(&mut job_list).await;
        
        assert!(result.is_err());
        match result {
            Err(e) if e.is::<JobRun>() => {
                assert!(!job_list.has(job.as_ref(), None));
            },
            _ => panic!("job should have been run"),
        }
    }
}