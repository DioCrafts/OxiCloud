use std::sync::Arc;

use sabre_dav::{
    ServerPlugin, Server,
    exception::ServiceUnavailable,
};
use oc_lib::{Config, OC};

mod service_unavailable;

/// MaintenancePlugin for ownCloud
///
/// Checks if the system is in maintenance mode and returns
/// a 503 Service Unavailable response if it is.
pub struct MaintenancePlugin {
    /// Reference to main server object
    server: Option<Arc<Server>>,
}

impl MaintenancePlugin {
    /// Create a new MaintenancePlugin
    pub fn new() -> Self {
        Self {
            server: None,
        }
    }

    /// This method is called before any HTTP method and returns http status code 503
    /// in case the system is in maintenance mode.
    ///
    /// Returns true if the system is not in maintenance mode.
    /// Throws ServiceUnavailable exception otherwise.
    fn check_maintenance_mode(&self) -> Result<bool, ServiceUnavailable> {
        if Config::get_value("maintenance", false) {
            return Err(ServiceUnavailable::new());
        }

        if OC::check_upgrade(false) {
            return Err(ServiceUnavailable::new_with_message("Upgrade needed"));
        }

        Ok(true)
    }
}

impl ServerPlugin for MaintenancePlugin {
    /// This initializes the plugin.
    ///
    /// This function is called by Server, after
    /// add_plugin is called.
    ///
    /// This method should set up the required event subscriptions.
    fn initialize(&mut self, server: Arc<Server>) {
        self.server = Some(Arc::clone(&server));
        
        // Subscribe to the beforeMethod event with priority 10
        server.subscribe_event("beforeMethod", Box::new(move |_params| {
            // We need to clone self here to avoid ownership issues
            let plugin = self;
            match plugin.check_maintenance_mode() {
                Ok(_) => Ok(true),
                Err(e) => Err(Box::new(e) as Box<dyn std::error::Error>),
            }
        }), 10);
    }
}

impl Default for MaintenancePlugin {
    fn default() -> Self {
        Self::new()
    }
}