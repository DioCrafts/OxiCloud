use std::any::Any;
use serde::{Serialize, Deserialize};
use serde_json;
use std::error::Error;

/// Job trait that represents a background job
pub trait Job: Send + Sync + 'static {
    fn get_id(&self) -> i64;
    fn set_id(&mut self, id: i64);
    fn get_last_run(&self) -> i64;
    fn set_last_run(&mut self, time: i64);
    fn get_argument(&self) -> Option<Box<dyn Any>>;
    fn set_argument(&mut self, argument: Option<Box<dyn Any>>);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Database connection trait abstraction
pub trait DbConnection {
    fn prepare(&self, query: &str, limit: Option<usize>) -> Box<dyn DbStatement>;
    fn last_insert_id(&self) -> i64;
}

/// Database statement trait abstraction
pub trait DbStatement {
    fn execute(&self, params: Vec<Box<dyn Any>>) -> Box<dyn DbResult>;
}

/// Database result trait abstraction
pub trait DbResult {
    fn fetch_row(&mut self) -> Option<Vec<Box<dyn Any>>>;
}

/// Configuration trait abstraction
pub trait AppConfig {
    fn get_value(&self, app: &str, key: &str, default: &str) -> String;
    fn set_value(&self, app: &str, key: &str, value: &str);
}

/// JobList manages the queue of background jobs
pub struct JobList {
    db_connection: Box<dyn DbConnection>,
    app_config: Box<dyn AppConfig>,
}

impl JobList {
    /// Create a new JobList with the given database connection and app config
    pub fn new(db_connection: Box<dyn DbConnection>, app_config: Box<dyn AppConfig>) -> Self {
        JobList {
            db_connection,
            app_config,
        }
    }

    /// Add a job to the list
    ///
    /// # Arguments
    ///
    /// * `job` - The job to add, either as a Job trait object or a string class name
    /// * `argument` - Optional argument to the job
    pub fn add<T: Serialize>(&self, job: Box<dyn Any>, argument: Option<T>) -> Result<(), Box<dyn Error>> {
        if let Some(job_obj) = job.downcast_ref::<Box<dyn Job>>() {
            let class = std::any::type_name::<Box<dyn Job>>();
            let arg_json = match &argument {
                Some(arg) => serde_json::to_string(arg)?,
                None => String::from("null"),
            };
            
            if !self.has(Box::new(class), argument.clone())? {
                let query = self.db_connection.prepare("INSERT INTO `*PREFIX*jobs`(`class`, `argument`, `last_run`) VALUES(?, ?, 0)", None);
                let params: Vec<Box<dyn Any>> = vec![
                    Box::new(class.to_string()),
                    Box::new(arg_json),
                ];
                query.execute(params);
            }
            Ok(())
        } else if let Some(class) = job.downcast_ref::<String>() {
            let arg_json = match &argument {
                Some(arg) => serde_json::to_string(arg)?,
                None => String::from("null"),
            };
            
            if !self.has(Box::new(class.clone()), argument.clone())? {
                let query = self.db_connection.prepare("INSERT INTO `*PREFIX*jobs`(`class`, `argument`, `last_run`) VALUES(?, ?, 0)", None);
                let params: Vec<Box<dyn Any>> = vec![
                    Box::new(class.clone()),
                    Box::new(arg_json),
                ];
                query.execute(params);
            }
            Ok(())
        } else {
            Err("Invalid job type".into())
        }
    }

    /// Remove a job from the list
    ///
    /// # Arguments
    ///
    /// * `job` - The job to remove, either as a Job trait object or a string class name
    /// * `argument` - Optional argument to identify the specific job
    pub fn remove<T: Serialize>(&self, job: Box<dyn Any>, argument: Option<T>) -> Result<(), Box<dyn Error>> {
        let class = if let Some(job_obj) = job.downcast_ref::<Box<dyn Job>>() {
            std::any::type_name::<Box<dyn Job>>().to_string()
        } else if let Some(class_name) = job.downcast_ref::<String>() {
            class_name.clone()
        } else {
            return Err("Invalid job type".into());
        };

        match argument {
            Some(arg) => {
                let arg_json = serde_json::to_string(&arg)?;
                let query = self.db_connection.prepare("DELETE FROM `*PREFIX*jobs` WHERE `class` = ? AND `argument` = ?", None);
                let params: Vec<Box<dyn Any>> = vec![
                    Box::new(class),
                    Box::new(arg_json),
                ];
                query.execute(params);
            }
            None => {
                let query = self.db_connection.prepare("DELETE FROM `*PREFIX*jobs` WHERE `class` = ?", None);
                let params: Vec<Box<dyn Any>> = vec![Box::new(class)];
                query.execute(params);
            }
        }
        Ok(())
    }

    /// Check if a job is in the list
    ///
    /// # Arguments
    ///
    /// * `job` - The job to check for, either as a Job trait object or a string class name
    /// * `argument` - The argument to identify the specific job
    ///
    /// # Returns
    ///
    /// `true` if the job is in the list, `false` otherwise
    pub fn has<T: Serialize>(&self, job: Box<dyn Any>, argument: Option<T>) -> Result<bool, Box<dyn Error>> {
        let class = if let Some(job_obj) = job.downcast_ref::<Box<dyn Job>>() {
            std::any::type_name::<Box<dyn Job>>().to_string()
        } else if let Some(class_name) = job.downcast_ref::<String>() {
            class_name.clone()
        } else {
            return Err("Invalid job type".into());
        };

        let arg_json = match argument {
            Some(arg) => serde_json::to_string(&arg)?,
            None => String::from("null"),
        };

        let query = self.db_connection.prepare("SELECT `id` FROM `*PREFIX*jobs` WHERE `class` = ? AND `argument` = ?", None);
        let params: Vec<Box<dyn Any>> = vec![
            Box::new(class),
            Box::new(arg_json),
        ];
        let mut result = query.execute(params);
        
        Ok(result.fetch_row().is_some())
    }

    /// Get all jobs in the list
    ///
    /// # Returns
    ///
    /// A vector of all jobs
    pub fn get_all(&self) -> Vec<Box<dyn Job>> {
        let query = self.db_connection.prepare("SELECT `id`, `class`, `last_run`, `argument` FROM `*PREFIX*jobs`", None);
        let mut result = query.execute(vec![]);
        let mut jobs = Vec::new();
        
        while let Some(row) = result.fetch_row() {
            if let Some(job) = self.build_job(row) {
                jobs.push(job);
            }
        }
        
        jobs
    }

    /// Get the next job in the list
    ///
    /// # Returns
    ///
    /// The next job or None if the list is empty
    pub fn get_next(&self) -> Option<Box<dyn Job>> {
        let last_id = self.get_last_job();
        
        let query = self.db_connection.prepare("SELECT `id`, `class`, `last_run`, `argument` FROM `*PREFIX*jobs` WHERE `id` > ? ORDER BY `id` ASC", Some(1));
        let mut result = query.execute(vec![Box::new(last_id)]);
        
        if let Some(row) = result.fetch_row() {
            self.build_job(row)
        } else {
            // Begin at the start of the queue
            let query = self.db_connection.prepare("SELECT `id`, `class`, `last_run`, `argument` FROM `*PREFIX*jobs` ORDER BY `id` ASC", Some(1));
            let mut result = query.execute(vec![]);
            
            if let Some(row) = result.fetch_row() {
                self.build_job(row)
            } else {
                None // Empty job list
            }
        }
    }

    /// Get a job by its ID
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the job
    ///
    /// # Returns
    ///
    /// The job with the given ID or None if not found
    pub fn get_by_id(&self, id: i64) -> Option<Box<dyn Job>> {
        let query = self.db_connection.prepare("SELECT `id`, `class`, `last_run`, `argument` FROM `*PREFIX*jobs` WHERE `id` = ?", None);
        let mut result = query.execute(vec![Box::new(id)]);
        
        if let Some(row) = result.fetch_row() {
            self.build_job(row)
        } else {
            None
        }
    }

    /// Build a job object from a database row
    ///
    /// # Arguments
    ///
    /// * `row` - A row from the database
    ///
    /// # Returns
    ///
    /// A job object or None if the row is invalid
    fn build_job(&self, row: Vec<Box<dyn Any>>) -> Option<Box<dyn Job>> {
        if row.len() < 4 {
            return None;
        }
        
        // Extract values from row
        let id = match row[0].downcast_ref::<i64>() {
            Some(id) => *id,
            None => return None,
        };
        
        let class = match row[1].downcast_ref::<String>() {
            Some(class) => class.clone(),
            None => return None,
        };
        
        let last_run = match row[2].downcast_ref::<i64>() {
            Some(last_run) => *last_run,
            None => return None,
        };
        
        let argument_str = match row[3].downcast_ref::<String>() {
            Some(arg) => arg.clone(),
            None => return None,
        };
        
        // Create job instance (this is a simplification, in a real implementation
        // you would need to use a factory or reflection mechanism)
        let mut job = self.create_job_instance(&class)?;
        
        job.set_id(id);
        job.set_last_run(last_run);
        
        // Parse argument if not null
        if argument_str != "null" {
            if let Ok(arg) = serde_json::from_str::<Box<dyn Any>>(&argument_str) {
                job.set_argument(Some(arg));
            }
        }
        
        Some(job)
    }

    /// Create a job instance from a class name (placeholder implementation)
    fn create_job_instance(&self, class_name: &str) -> Option<Box<dyn Job>> {
        // This is a placeholder. In a real implementation, you would need
        // to use a factory pattern or some form of reflection to instantiate
        // the correct job class based on the class name.
        None
    }

    /// Set the last run job
    ///
    /// # Arguments
    ///
    /// * `job` - The job that was last run
    pub fn set_last_job(&self, job: &dyn Job) {
        self.app_config.set_value("backgroundjob", "lastjob", &job.get_id().to_string());
    }

    /// Get the ID of the last run job
    ///
    /// # Returns
    ///
    /// The ID of the last run job
    pub fn get_last_job(&self) -> i64 {
        let last_job_str = self.app_config.get_value("backgroundjob", "lastjob", "0");
        last_job_str.parse::<i64>().unwrap_or(0)
    }

    /// Set the last run time of a job to now
    ///
    /// # Arguments
    ///
    /// * `job` - The job to update
    pub fn set_last_run(&self, job: &dyn Job) {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
            
        let query = self.db_connection.prepare("UPDATE `*PREFIX*jobs` SET `last_run` = ? WHERE `id` = ?", None);
        let params: Vec<Box<dyn Any>> = vec![
            Box::new(current_time),
            Box::new(job.get_id()),
        ];
        query.execute(params);
    }
}