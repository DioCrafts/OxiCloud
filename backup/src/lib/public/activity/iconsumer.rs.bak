// SPDX-FileCopyrightText: 2013 Thomas Müller <deepdiver@owncloud.com>
// SPDX-License-Identifier: AGPL-3.0-or-later
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

/// Public trait for activity consumers that apps should use
pub trait IConsumer {
    /// Receive an activity event
    ///
    /// # Parameters
    /// * `app` - The app generating the activity
    /// * `subject` - Subject of the activity
    /// * `subject_params` - Parameters for the subject
    /// * `message` - Message of the activity
    /// * `message_params` - Parameters for the message
    /// * `file` - File related to the activity
    /// * `link` - Link related to the activity
    /// * `affected_user` - User affected by the activity
    /// * `activity_type` - Type of the activity
    /// * `priority` - Priority of the activity
    ///
    /// # Returns
    /// Result indicating success or failure
    fn receive(
        &self,
        app: &str,
        subject: &str,
        subject_params: &[&str],
        message: &str,
        message_params: &[&str],
        file: Option<&str>,
        link: Option<&str>,
        affected_user: &str,
        activity_type: &str,
        priority: i32,
    ) -> Result<(), Box<dyn std::error::Error>>;
}