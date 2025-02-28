use async_trait::async_trait;

/// Interface for configuration management
#[async_trait]
pub trait IConfig {
    /// Looks up a system wide defined value
    /// # Arguments
    /// * `key` - the key of the value, under which it was saved
    /// # Returns
    /// The saved value
    async fn get_system_value(&self, key: &str) -> String;

    /// Writes a new app wide value
    /// # Arguments
    /// * `app_name` - the app name that we want to store the value under
    /// * `key` - the key of the value, under which will be saved
    /// * `value` - the value that should be stored
    async fn set_app_value(&self, app_name: &str, key: &str, value: &str) -> Result<(), ConfigError>;

    /// Looks up an app wide defined value
    /// # Arguments
    /// * `app_name` - the app name that we stored the value under
    /// * `key` - the key of the value, under which it was saved
    /// # Returns
    /// The saved value
    async fn get_app_value(&self, app_name: &str, key: &str) -> String;

    /// Set a user defined value
    /// # Arguments
    /// * `user_id` - the user id of the user that we want to store the value under
    /// * `app_name` - the app name that we want to store the value under
    /// * `key` - the key under which the value is being stored
    /// * `value` - the value that you want to store
    async fn set_user_value(&self, user_id: &str, app_name: &str, key: &str, value: &str) -> Result<(), ConfigError>;

    /// Shortcut for getting a user defined value
    /// # Arguments
    /// * `user_id` - the user id of the user that we want to get the value for
    /// * `app_name` - the app name that we stored the value under
    /// * `key` - the key under which the value is being stored
    /// # Returns
    /// The saved value
    async fn get_user_value(&self, user_id: &str, app_name: &str, key: &str) -> String;
}

/// Error type for config operations
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Invalid configuration value: {0}")]
    InvalidValue(String),
    
    #[error("Configuration not found")]
    NotFound,
    
    #[error("Unknown config error: {0}")]
    Other(String),
}

/// Class to combine all the configuration options NextCloud offers
pub struct AllConfig {
    // Dependencies would go here
}

impl AllConfig {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IConfig for AllConfig {
    async fn get_system_value(&self, key: &str) -> String {
        // Call to the equivalent of \OCP\Config::getSystemValue
        config::get_system_value(key, "").await
    }

    async fn set_app_value(&self, app_name: &str, key: &str, value: &str) -> Result<(), ConfigError> {
        // Call to the equivalent of \OCP\Config::setAppValue
        config::set_app_value(app_name, key, value).await
    }

    async fn get_app_value(&self, app_name: &str, key: &str) -> String {
        // Call to the equivalent of \OCP\Config::getAppValue
        config::get_app_value(app_name, key, "").await
    }

    async fn set_user_value(&self, user_id: &str, app_name: &str, key: &str, value: &str) -> Result<(), ConfigError> {
        // Call to the equivalent of \OCP\Config::setUserValue
        config::set_user_value(user_id, app_name, key, value).await
    }

    async fn get_user_value(&self, user_id: &str, app_name: &str, key: &str) -> String {
        // Call to the equivalent of \OCP\Config::getUserValue
        config::get_user_value(user_id, app_name, key, "").await
    }
}

// This module would contain the actual implementation of the config functions
mod config {
    use super::ConfigError;

    pub async fn get_system_value(key: &str, default: &str) -> String {
        // Implementation would go here
        default.to_string()
    }

    pub async fn set_app_value(app_name: &str, key: &str, value: &str) -> Result<(), ConfigError> {
        // Implementation would go here
        Ok(())
    }

    pub async fn get_app_value(app_name: &str, key: &str, default: &str) -> String {
        // Implementation would go here
        default.to_string()
    }

    pub async fn set_user_value(user_id: &str, app_name: &str, key: &str, value: &str) -> Result<(), ConfigError> {
        // Implementation would go here
        Ok(())
    }

    pub async fn get_user_value(user_id: &str, app_name: &str, key: &str, default: &str) -> String {
        // Implementation would go here
        default.to_string()
    }
}