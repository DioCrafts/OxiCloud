// Copyright (C) 2013 Thomas Müller <deepdiver@owncloud.com>
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

use std::sync::Arc;

/// Consumer interface for activity events
pub trait IConsumer: Send + Sync {
    // The IConsumer trait would be defined elsewhere
}

/// Manager interface for handling activity events
pub trait IManager: Send + Sync {
    /// Publish a new activity event
    ///
    /// # Arguments
    ///
    /// * `app` - The app that is publishing the activity
    /// * `subject` - The subject of the activity
    /// * `subject_params` - Parameters for the subject
    /// * `message` - The message of the activity
    /// * `message_params` - Parameters for the message
    /// * `file` - The file related to the activity
    /// * `link` - The link related to the activity
    /// * `affected_user` - The user affected by the activity
    /// * `type_` - The type of the activity
    /// * `priority` - The priority of the activity
    fn publish_activity(
        &self,
        app: &str,
        subject: &str,
        subject_params: &[&str],
        message: &str,
        message_params: &[&str],
        file: Option<&str>,
        link: Option<&str>,
        affected_user: &str,
        type_: &str,
        priority: i32,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Register a consumer for activity events
    ///
    /// In order to improve lazy loading a closure can be registered which will be called in case
    /// activity consumers are actually requested
    ///
    /// The closure has to return an instance implementing the IConsumer trait
    ///
    /// # Arguments
    ///
    /// * `callable` - Closure that returns an IConsumer implementation
    fn register_consumer<F>(&mut self, callable: F)
    where
        F: Fn() -> Arc<dyn IConsumer> + Send + Sync + 'static;
}