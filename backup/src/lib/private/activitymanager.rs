/**
 * Copyright (c) 2013 Thomas Müller thomas.mueller@tmit.eu
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 *
 */

use std::sync::{Arc, RwLock};
use log::error;

/// Activity interface consumer definition
pub trait IConsumer: Send + Sync {
    fn receive(
        &self,
        app: &str,
        subject: &str,
        subject_params: &[&str],
        message: &str,
        message_params: &[&str],
        file: &str,
        link: &str,
        affected_user: &str,
        activity_type: &str,
        priority: i32,
    );
}

/// Manager interface for activity handling
pub trait IManager: Send + Sync {
    fn publish_activity(
        &self,
        app: &str,
        subject: &str,
        subject_params: &[&str],
        message: &str,
        message_params: &[&str],
        file: &str,
        link: &str,
        affected_user: &str,
        activity_type: &str,
        priority: i32,
    );

    fn register_consumer<F>(&self, factory: F)
    where
        F: Fn() -> Arc<dyn IConsumer> + Send + Sync + 'static;
}

/// Implementation of the activity manager
pub struct ActivityManager {
    consumers: RwLock<Vec<Box<dyn Fn() -> Arc<dyn IConsumer> + Send + Sync>>>,
}

impl ActivityManager {
    pub fn new() -> Self {
        ActivityManager {
            consumers: RwLock::new(Vec::new()),
        }
    }
}

impl IManager for ActivityManager {
    fn publish_activity(
        &self,
        app: &str,
        subject: &str,
        subject_params: &[&str],
        message: &str,
        message_params: &[&str],
        file: &str,
        link: &str,
        affected_user: &str,
        activity_type: &str,
        priority: i32,
    ) {
        if let Ok(consumers) = self.consumers.read() {
            for consumer_factory in consumers.iter() {
                let consumer = consumer_factory();
                
                // Use a match instead of try/catch
                match std::panic::catch_unwind(|| {
                    consumer.receive(
                        app,
                        subject,
                        subject_params,
                        message,
                        message_params,
                        file,
                        link,
                        affected_user,
                        activity_type,
                        priority,
                    )
                }) {
                    Ok(_) => {}, // Successfully received
                    Err(e) => {
                        // Log the error
                        error!("Error in consumer.receive: {:?}", e);
                    }
                }
            }
        }
    }

    fn register_consumer<F>(&self, factory: F)
    where
        F: Fn() -> Arc<dyn IConsumer> + Send + Sync + 'static,
    {
        if let Ok(mut consumers) = self.consumers.write() {
            consumers.push(Box::new(factory));
        }
    }
}

impl Default for ActivityManager {
    fn default() -> Self {
        Self::new()
    }
}