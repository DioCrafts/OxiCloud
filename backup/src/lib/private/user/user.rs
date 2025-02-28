use crate::hooks::Emitter;
use crate::user::backend::OcUserBackend;
use std::sync::Arc;

// Constants from PHP backend
const OC_USER_BACKEND_GET_DISPLAYNAME: u32 = 4; // Assuming this is the value
const OC_USER_BACKEND_SET_PASSWORD: u32 = 8;    // Assuming this is the value
const OC_USER_BACKEND_SET_DISPLAYNAME: u32 = 16; // Assuming this is the value
const OC_USER_BACKEND_GET_HOME: u32 = 32;       // Assuming this is the value

/// User class for handling user operations
pub struct User {
    /// User ID
    uid: String,
    
    /// Display name for the user
    display_name: String,
    
    /// Backend for user operations
    backend: Arc<dyn OcUserBackend>,
    
    /// Whether the user is enabled
    enabled: bool,
    
    /// Emitter for events
    emitter: Option<Arc<dyn Emitter>>,
}

impl User {
    /// Create a new user
    ///
    /// # Arguments
    /// * `uid` - User ID
    /// * `backend` - Backend for user operations
    /// * `emitter` - Optional emitter for events
    pub fn new(uid: String, backend: Arc<dyn OcUserBackend>, emitter: Option<Arc<dyn Emitter>>) -> Self {
        let display_name = if backend.implements_actions(OC_USER_BACKEND_GET_DISPLAYNAME) {
            backend.get_display_name(&uid).unwrap_or_else(|| uid.clone())
        } else {
            uid.clone()
        };
        
        // TODO: DI for OC_Preferences
        let enabled = crate::preferences::OcPreferences::get_value(&uid, "core", "enabled", "true") == "true";
        
        Self {
            uid,
            display_name,
            backend,
            enabled,
            emitter,
        }
    }

    /// Get the user ID
    pub fn get_uid(&self) -> &str {
        &self.uid
    }

    /// Get the display name for the user
    /// If no specific display name is set, it will fall back to the user ID
    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }

    /// Set the display name for the user
    ///
    /// # Arguments
    /// * `display_name` - New display name
    ///
    /// # Returns
    /// `true` if the display name was changed, `false` otherwise
    pub fn set_display_name(&mut self, display_name: String) -> bool {
        if self.can_change_display_name() {
            self.display_name = display_name.clone();
            let result = self.backend.set_display_name(&self.uid, &display_name);
            result
        } else {
            false
        }
    }

    /// Delete the user
    ///
    /// # Returns
    /// `true` if the user was deleted, `false` otherwise
    pub fn delete(&self) -> bool {
        if let Some(emitter) = &self.emitter {
            emitter.emit(rr"\OC\User", "preDelete", &[self]);
        }
        
        let result = self.backend.delete_user(&self.uid);
        
        if let Some(emitter) = &self.emitter {
            emitter.emit(rr"\OC\User", "postDelete", &[self]);
        }
        
        result
    }

    /// Set the password of the user
    ///
    /// # Arguments
    /// * `password` - New password
    /// * `recovery_password` - For the encryption app to reset encryption keys
    ///
    /// # Returns
    /// `true` if the password was set, `false` otherwise
    pub fn set_password(&self, password: &str, recovery_password: &str) -> bool {
        if let Some(emitter) = &self.emitter {
            emitter.emit(rr"\OC\User", "preSetPassword", &[self, password, recovery_password]);
        }
        
        if self.backend.implements_actions(OC_USER_BACKEND_SET_PASSWORD) {
            let result = self.backend.set_password(&self.uid, password);
            
            if let Some(emitter) = &self.emitter {
                emitter.emit(rr"\OC\User", "postSetPassword", &[self, password, recovery_password]);
            }
            
            result
        } else {
            false
        }
    }

    /// Get the user's home folder path
    pub fn get_home(&self) -> String {
        if self.backend.implements_actions(OC_USER_BACKEND_GET_HOME) {
            if let Some(home) = self.backend.get_home(&self.uid) {
                return home;
            }
        }
        
        // TODO: Switch to Config object once implemented
        let data_directory = crate::config::OcConfig::get_value(
            "datadirectory", 
            &format!("{}/data", crate::OC::server_root())
        );
        
        format!("{}/{}", data_directory, self.uid)
    }

    /// Check if the backend supports changing passwords
    pub fn can_change_password(&self) -> bool {
        self.backend.implements_actions(OC_USER_BACKEND_SET_PASSWORD)
    }

    /// Check if the backend supports changing display names
    pub fn can_change_display_name(&self) -> bool {
        self.backend.implements_actions(OC_USER_BACKEND_SET_DISPLAYNAME)
    }

    /// Check if the user is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Set the enabled status for the user
    ///
    /// # Arguments
    /// * `enabled` - Whether the user should be enabled
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        let enabled_value = if enabled { "true" } else { "false" };
        crate::preferences::OcPreferences::set_value(&self.uid, "core", "enabled", enabled_value);
    }
}