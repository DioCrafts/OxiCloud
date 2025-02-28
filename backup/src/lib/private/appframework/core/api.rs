// lib/private/appframework/core/api.rs

use crate::OCP::{self, AppFramework::IApi, User, Util, App, Backgroundjob};

/// This is used to wrap the owncloud static api calls into an object to make the
/// code better abstractable for use in the dependency injection container
///
/// Should you find yourself in need for more methods, simply inherit from this
/// class and add your methods
pub struct Api {
    app_name: String,
}

impl Api {
    /// constructor
    /// @param app_name the name of your application
    pub fn new(app_name: String) -> Self {
        Self { app_name }
    }
}

impl IApi for Api {
    /// Gets the userid of the current user
    /// @return the user id of the current user
    fn get_user_id(&self) -> String {
        User::get_user()
    }

    /// Adds a new javascript file
    /// @param script_name the name of the javascript in js/ without the suffix
    /// @param app_name the name of the app, defaults to the current one
    fn add_script(&self, script_name: &str, app_name: Option<&str>) {
        let app = app_name.unwrap_or(&self.app_name);
        Util::add_script(app, script_name);
    }

    /// Adds a new css file
    /// @param style_name the name of the css file in css/without the suffix
    /// @param app_name the name of the app, defaults to the current one
    fn add_style(&self, style_name: &str, app_name: Option<&str>) {
        let app = app_name.unwrap_or(&self.app_name);
        Util::add_style(app, style_name);
    }

    /// shorthand for addScript for files in the 3rdparty directory
    /// @param name the name of the file without the suffix
    fn add_3rd_party_script(&self, name: &str) {
        Util::add_script(&format!("{}/3rdparty", self.app_name), name);
    }

    /// shorthand for addStyle for files in the 3rdparty directory
    /// @param name the name of the file without the suffix
    fn add_3rd_party_style(&self, name: &str) {
        Util::add_style(&format!("{}/3rdparty", self.app_name), name);
    }

    /// Checks if an app is enabled
    /// @param app_name the name of an app
    /// @return true if app is enabled
    fn is_app_enabled(&self, app_name: &str) -> bool {
        App::is_enabled(app_name)
    }

    /// used to return and open a new eventsource
    /// @return a new open EventSource class
    fn open_event_source(&self) -> OC_EventSource {
        // TODO: use public api
        OC_EventSource::new()
    }

    /// connects a function to a hook
    /// @param signal_class class name of emitter
    /// @param signal_name name of signal
    /// @param slot_class class name of slot
    /// @param slot_name name of slot, in another word, this is the
    ///        name of the method that will be called when registered
    ///        signal is emitted.
    /// @return bool, always true
    fn connect_hook(&self, signal_class: &str, signal_name: &str, slot_class: &str, slot_name: &str) -> bool {
        Util::connect_hook(signal_class, signal_name, slot_class, slot_name)
    }

    /// Emits a signal. To get data from the slot use references!
    /// @param signal_class class name of emitter
    /// @param signal_name name of signal
    /// @param params defautl: empty vec - array with additional data
    /// @return bool, true if slots exists or false if not
    fn emit_hook(&self, signal_class: &str, signal_name: &str, params: Vec<String>) -> bool {
        Util::emit_hook(signal_class, signal_name, params)
    }

    /// clear hooks
    /// @param signal_class
    /// @param signal_name
    fn clear_hook(&self, signal_class: Option<&str>, signal_name: Option<&str>) {
        if let Some(class) = signal_class {
            OC_Hook::clear(class, signal_name);
        }
    }

    /// Register a backgroundjob task
    /// @param class_name full namespace and class name of the class
    /// @param method_name the name of the static method that should be called
    fn add_regular_task(&self, class_name: &str, method_name: &str) {
        Backgroundjob::add_regular_task(class_name, method_name);
    }

    /// Tells ownCloud to include a template in the admin overview
    /// @param main_path the path to the main php file without the php
    /// suffix, relative to your apps directory! not the template directory
    /// @param app_name the name of the app, defaults to the current one
    fn register_admin(&self, main_path: &str, app_name: Option<&str>) {
        let app = app_name.unwrap_or(&self.app_name);
        App::register_admin(app, main_path);
    }
}

// Stub for OC_EventSource - This would need to be properly implemented
pub struct OC_EventSource;

impl OC_EventSource {
    pub fn new() -> Self {
        Self
    }
}

// Stub for OC_Hook - This would need to be properly implemented
pub struct OC_Hook;

impl OC_Hook {
    pub fn clear(signal_class: &str, signal_name: Option<&str>) {
        // Implementation would go here
    }
}