// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct JobRun;

impl std::fmt::Display for JobRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Job run exception")
    }
}

impl std::error::Error for JobRun {}

pub trait JobList {
    fn add(&mut self, job: Box<dyn TimedJob>);
    fn set_last_run(&mut self, job: &dyn TimedJob, last_run: SystemTime);
}

pub trait TimedJob {
    fn set_interval(&mut self, seconds: u64);
    fn get_interval(&self) -> Duration;
    fn set_last_run(&mut self, time: SystemTime);
    fn get_last_run(&self) -> SystemTime;
    fn execute(&self, job_list: &mut dyn JobList) -> Result<(), JobRun>;
    fn run(&self, argument: Option<&str>) -> Result<(), JobRun>;
}

struct DummyJobList {
    jobs: Vec<Box<dyn TimedJob>>,
}

impl DummyJobList {
    fn new() -> Self {
        DummyJobList { jobs: vec![] }
    }
}

impl JobList for DummyJobList {
    fn add(&mut self, job: Box<dyn TimedJob>) {
        self.jobs.push(job);
    }

    fn set_last_run(&mut self, _job: &dyn TimedJob, _last_run: SystemTime) {
        // Implementation not needed for tests
    }
}

struct BaseTimedJob {
    interval: Duration,
    last_run: SystemTime,
}

impl BaseTimedJob {
    fn new() -> Self {
        BaseTimedJob {
            interval: Duration::from_secs(0),
            last_run: SystemTime::now(),
        }
    }
}

impl TimedJob for BaseTimedJob {
    fn set_interval(&mut self, seconds: u64) {
        self.interval = Duration::from_secs(seconds);
    }

    fn get_interval(&self) -> Duration {
        self.interval
    }

    fn set_last_run(&mut self, time: SystemTime) {
        self.last_run = time;
    }

    fn get_last_run(&self) -> SystemTime {
        self.last_run
    }

    fn execute(&self, job_list: &mut dyn JobList) -> Result<(), JobRun> {
        let now = SystemTime::now();
        if let Ok(duration) = now.duration_since(self.last_run) {
            if duration >= self.interval {
                job_list.set_last_run(self, now);
                return self.run(None);
            }
        }
        Ok(())
    }

    fn run(&self, _argument: Option<&str>) -> Result<(), JobRun> {
        Ok(())
    }
}

struct TestTimedJob {
    base: BaseTimedJob,
}

impl TestTimedJob {
    fn new() -> Self {
        let mut job = TestTimedJob {
            base: BaseTimedJob::new(),
        };
        job.set_interval(10);
        job
    }
}

impl TimedJob for TestTimedJob {
    fn set_interval(&mut self, seconds: u64) {
        self.base.set_interval(seconds);
    }

    fn get_interval(&self) -> Duration {
        self.base.get_interval()
    }

    fn set_last_run(&mut self, time: SystemTime) {
        self.base.set_last_run(time);
    }

    fn get_last_run(&self) -> SystemTime {
        self.base.get_last_run()
    }

    fn execute(&self, job_list: &mut dyn JobList) -> Result<(), JobRun> {
        self.base.execute(job_list)
    }

    fn run(&self, argument: Option<&str>) -> Result<(), JobRun> {
        // Throw an exception so we can detect if this function is called
        Err(JobRun)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TimedJobTest {
        job_list: DummyJobList,
        job: TestTimedJob,
    }
    
    impl TimedJobTest {
        fn setup() -> Self {
            let mut job_list = DummyJobList::new();
            let job = TestTimedJob::new();
            let boxed_job = Box::new(job);
            job_list.add(boxed_job);
            
            // Need to recreate job since we've moved it into the job_list
            let job = TestTimedJob::new();
            
            TimedJobTest { job_list, job }
        }
    }
    
    #[test]
    fn test_should_run_after_interval() {
        let mut test = TimedJobTest::setup();
        
        let past_time = SystemTime::now() - Duration::from_secs(12);
        test.job.set_last_run(past_time);
        
        let result = test.job.execute(&mut test.job_list);
        assert!(result.is_err(), "job should have run");
    }
    
    #[test]
    fn test_should_not_run_within_interval() {
        let mut test = TimedJobTest::setup();
        
        let past_time = SystemTime::now() - Duration::from_secs(5);
        test.job.set_last_run(past_time);
        
        let result = test.job.execute(&mut test.job_list);
        assert!(result.is_ok(), "job should not have run");
    }
    
    #[test]
    fn test_should_not_run_twice() {
        let mut test = TimedJobTest::setup();
        
        let past_time = SystemTime::now() - Duration::from_secs(15);
        test.job.set_last_run(past_time);
        
        let result = test.job.execute(&mut test.job_list);
        assert!(result.is_err(), "job should have run the first time");
        
        let result = test.job.execute(&mut test.job_list);
        assert!(result.is_ok(), "job should not have run the second time");
    }
}