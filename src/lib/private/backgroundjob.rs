//! # Background Job System
//!
//! This module handles background job configuration and management.
//!
//! Originally based on:
//! ownCloud
//!
//! @author Jakob Sack
//! @copyright 2012 Jakob Sack owncloud@jakobsack.de
//!
//! This library is free software; you can redistribute it and/or
//! modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
//! License as published by the Free Software Foundation; either
//! version 3 of the License, or any later version.
//!
//! This library is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//!
//! You should have received a copy of the GNU Affero General Public
//! License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use crate::app_config;
use std::fmt;
use thiserror::Error;

/// Represents the execution type for background jobs
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionType {
    /// No background jobs execution
    None,
    /// Execute background jobs via AJAX
    Ajax,
    /// Execute background jobs via webcron
    Webcron,
    /// Execute background jobs via system cron
    Cron,
}

impl fmt::Display for ExecutionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Ajax => write!(f, "ajax"),
            Self::Webcron => write!(f, "webcron"),
            Self::Cron => write!(f, "cron"),
        }
    }
}

impl TryFrom<&str> for ExecutionType {
    type Error = BackgroundJobError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "none" => Ok(Self::None),
            "ajax" => Ok(Self::Ajax),
            "webcron" => Ok(Self::Webcron),
            "cron" => Ok(Self::Cron),
            _ => Err(BackgroundJobError::InvalidExecutionType(value.to_string())),
        }
    }
}

/// Errors that can occur when working with background jobs
#[derive(Error, Debug)]
pub enum BackgroundJobError {
    /// The execution type is invalid
    #[error("Invalid execution type: {0}")]
    InvalidExecutionType(String),
    
    /// Error interacting with app configuration
    #[error("App config error: {0}")]
    AppConfigError(#[from] app_config::AppConfigError),
}

/// Background job management functionality
pub struct BackgroundJob;

impl BackgroundJob {
    /// Get the execution type of background jobs
    ///
    /// This method returns the type how background jobs are executed. If the user
    /// did not select something, the type is ajax.
    pub async fn get_execution_type() -> Result<ExecutionType, BackgroundJobError> {
        let value = app_config::get_value("core", "backgroundjobs_mode", "ajax").await?;
        ExecutionType::try_from(value.as_str())
    }

    /// Set the execution type of background jobs
    ///
    /// This method sets the execution type of the background jobs. Possible types
    /// are "none", "ajax", "webcron", "cron"
    pub async fn set_execution_type(execution_type: ExecutionType) -> Result<(), BackgroundJobError> {
        app_config::set_value("core", "backgroundjobs_mode", &execution_type.to_string()).await?;
        Ok(())
    }
}