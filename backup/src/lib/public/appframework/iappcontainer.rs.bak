// This module provides the IAppContainer trait for app developers.

use crate::appframework::IApi;
use crate::appframework::Middleware;
use crate::IContainer;
use crate::IServerContainer;

/// This container interface provides short cuts for app developers to access predefined app service.
///
/// # Note
/// This is the Rust equivalent of the PHP IAppContainer interface.
pub trait IAppContainer: IContainer {
    /// Used to return the appname of the set application
    ///
    /// # Returns
    /// The name of your application
    fn get_app_name(&self) -> String;

    /// Get the core API
    ///
    /// # Returns
    /// A reference to the core API implementation
    fn get_core_api(&self) -> &dyn IApi;

    /// Get the server container
    ///
    /// # Returns
    /// A reference to the server container
    fn get_server(&self) -> &dyn IServerContainer;

    /// Register a middleware
    ///
    /// # Arguments
    /// * `middleware` - The middleware to register
    ///
    /// # Returns
    /// true if registration was successful, false otherwise
    fn register_middleware(&self, middleware: Box<dyn Middleware>) -> bool;

    /// Check if a user is logged in
    ///
    /// # Returns
    /// true if a user is logged in, false otherwise
    fn is_logged_in(&self) -> bool;

    /// Check if the current user is an admin
    ///
    /// # Returns
    /// true if the current user is an admin, false otherwise
    fn is_admin_user(&self) -> bool;

    /// Log a message with a specific level
    ///
    /// # Arguments
    /// * `message` - The message to log
    /// * `level` - The log level
    fn log(&self, message: &str, level: u8);
}