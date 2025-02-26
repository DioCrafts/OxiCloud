//! Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.

use crate::backgroundjob::job::Job;
use crate::backgroundjob::job_list::JobList;
use async_trait::async_trait;
use std::fmt::Debug;
use std::marker::Send;

/// Create a background job that is to be executed once
///
/// This trait represents a job that should be run only once and then removed
/// from the job list automatically.
#[async_trait]
pub trait QueuedJob: Job {
    /// Run the job, then remove it from the joblist
    ///
    /// This method executes the job and then removes it from the job list
    /// to ensure it won't be run again.
    async fn execute<T: JobList + Send + Sync>(&self, job_list: &T) -> Result<(), Box<dyn std::error::Error>> {
        job_list.remove(self).await?;
        self.run().await?;
        Ok(())
    }
}