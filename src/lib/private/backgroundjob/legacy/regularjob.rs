// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use crate::private::backgroundjob::Job;
use std::sync::Arc;

pub struct RegularJob;

#[async_trait::async_trait]
impl Job for RegularJob {
    async fn run(&self, argument: Arc<dyn Fn() + Send + Sync>) -> Result<(), Box<dyn std::error::Error>> {
        (argument)();
        Ok(())
    }
}