// Copyright (c) 2012 Frank Karlitschek frank@owncloud.org
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library. If not, see <http://www.gnu.org/licenses/>.

// Public interface of ownCloud for apps to use.
// Utility Class.

use std::collections::HashMap;
use std::error::Error;

/// This class provides different helper functions to make the life of a developer easier
pub struct Util;

/// Log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
    Fatal = 4,
}

impl Util {
    /// Get the current installed version of ownCloud
    pub fn get_version() -> Vec<u32> {
        // In Rust, we would use an internal function to get the version
        crate::internal::util::get_version()
    }

    /// Send an email
    pub async fn send_mail(
        to_address: &str,
        to_name: &str,
        subject: &str,
        mail_text: &str,
        from_address: &str,
        from_name: &str,
        html: bool,
        alt_body: &str,
        cc_address: &str,
        cc_name: &str,
        bcc: &str,
    ) -> Result<(), Box<dyn Error>> {
        crate::internal::mail::send(
            to_address,
            to_name,
            subject,
            mail_text,
            from_address,
            from_name,
            html,
            alt_body,
            cc_address,
            cc_name,
            bcc,
        ).await
    }

    /// Write a message in the log
    pub fn write_log(app: &str, message: &str, level: LogLevel) {
        crate::internal::log::write(app, message, level);
    }

    /// Write exception into the log. Include the stack trace if DEBUG mode is enabled
    pub fn log_exception(app: &str, ex: &dyn Error) {
        let mut message = ex.to_string();
        
        if let Some(code) = ex.source() {
            message.push_str(&format!(" [{}]", code.to_string()));
        }
        
        Self::write_log(app, &format!("Exception: {}", message), LogLevel::Fatal);
        
        if cfg!(debug_assertions) {
            // In Rust, we use the backtrace crate or std::backtrace
            if let Some(bt) = std::backtrace::Backtrace::capture().to_string() {
                let stack: Vec<&str> = bt.split('\n').collect();
                for s in stack {
                    if !s.trim().is_empty() {
                        Self::write_log(app, &format!("Exception: {}", s), LogLevel::Fatal);
                    }
                }
            }
            
            // Handle nested cause errors
            let mut cause = ex.source();
            let l10n = crate::internal::l10n::get("lib");
            
            while let Some(err) = cause {
                message = format!("{} - {} {}", message, l10n.t("Caused by:"), err);
                
                if let Some(source) = err.source() {
                    message.push_str(&format!("[{}] ", source));
                }
                
                Self::write_log(app, &format!("Exception: {}", message), LogLevel::Fatal);
                cause = err.source();
            }
        }
    }

    /// Get l10n object
    pub fn get_l10n(application: &str) -> crate::internal::L10n {
        crate::internal::l10n::get(application)
    }

    /// Add a CSS file
    pub fn add_style(application: &str, file: Option<&str>) {
        crate::internal::util::add_style(application, file);
    }

    /// Add a JavaScript file
    pub fn add_script(application: &str, file: Option<&str>) {
        crate::internal::util::add_script(application, file);
    }

    /// Add a custom element to the header
    pub fn add_header(tag: &str, attributes: HashMap<String, String>, text: &str) {
        crate::internal::util::add_header(tag, attributes, text);
    }

    /// Formats a timestamp in the "right" way
    pub fn format_date(timestamp: i64, date_only: bool) -> String {
        crate::internal::util::format_date(timestamp, date_only)
    }

    /// Check if some encrypted files are stored
    pub fn encrypted_files() -> bool {
        crate::internal::util::encrypted_files()
    }

    /// Creates an absolute url to the given app and file.
    pub fn link_to_absolute(app: &str, file: &str, args: HashMap<String, String>) -> String {
        crate::internal::helper::link_to_absolute(app, file, args)
    }

    /// Creates an absolute url for remote use.
    pub fn link_to_remote(service: &str) -> String {
        crate::internal::helper::link_to_remote(service)
    }

    /// Creates an absolute url for public use
    pub fn link_to_public(service: &str) -> String {
        crate::internal::helper::link_to_public(service)
    }

    /// Creates an url using a defined route
    pub fn link_to_route(route: &str, parameters: HashMap<String, String>) -> String {
        crate::internal::helper::link_to_route(route, parameters)
    }

    /// Creates an url to the given app and file
    pub fn link_to(app: &str, file: &str, args: HashMap<String, String>) -> String {
        crate::internal::helper::link_to(app, file, args)
    }

    /// Returns the server host, even if the website uses one or more reverse proxy
    pub fn get_server_host() -> String {
        crate::internal::request::server_host()
    }

    /// Returns the server host name without an eventual port number
    pub fn get_server_host_name() -> String {
        let host_name = Self::get_server_host();
        
        // Strip away port number (if existing)
        if let Some(colon_pos) = host_name.find(':') {
            return host_name[..colon_pos].to_string();
        }
        
        host_name
    }

    /// Returns the default email address
    pub fn get_default_email_address(user_part: &str) -> String {
        let host_name = Self::get_server_host_name();
        let host_name = crate::internal::config::get_value("mail_domain", host_name);
        let default_email_address = format!("{}@{}", user_part, host_name);

        if crate::internal::mail::validate_address(&default_email_address) {
            return default_email_address;
        }

        // In case we cannot build a valid email address from the hostname let's fallback to 'localhost.localdomain'
        format!("{}@localhost.localdomain", user_part)
    }

    /// Returns the server protocol. It respects reverse proxy servers and load balancers
    pub fn get_server_protocol() -> String {
        crate::internal::request::server_protocol()
    }

    /// Returns the request uri, even if the website uses one or more reverse proxies
    pub fn get_request_uri() -> String {
        crate::internal::request::request_uri()
    }

    /// Returns the script name, even if the website uses one or more reverse proxies
    pub fn get_script_name() -> String {
        crate::internal::request::script_name()
    }

    /// Creates path to an image
    pub fn image_path(app: &str, image: &str) -> String {
        crate::internal::helper::image_path(app, image)
    }

    /// Make a human file size (2048 to 2 kB)
    pub fn human_file_size(bytes: u64) -> String {
        crate::internal::helper::human_file_size(bytes)
    }

    /// Make a computer file size (2 kB to 2048)
    pub fn computer_file_size(str: &str) -> u64 {
        crate::internal::helper::computer_file_size(str)
    }

    /// Connects a function to a hook
    pub fn connect_hook(
        signal_class: &str,
        signal_name: &str,
        slot_class: &str,
        slot_name: &str,
    ) -> bool {
        crate::internal::hook::connect(signal_class, signal_name, slot_class, slot_name)
    }

    /// Emits a signal. To get data from the slot use references!
    pub fn emit_hook(
        signal_class: &str,
        signal_name: &str,
        params: HashMap<String, String>,
    ) -> bool {
        crate::internal::hook::emit(signal_class, signal_name, params)
    }

    /// Register an get/post call. This is important to prevent CSRF attacks
    pub fn call_register() -> String {
        crate::internal::util::call_register()
    }

    /// Check an ajax get/post call if the request token is valid. exit if not.
    pub fn call_check() {
        crate::internal::util::call_check();
    }

    /// Used to sanitize HTML
    pub fn sanitize_html<T: AsRef<str>>(value: T) -> String {
        crate::internal::util::sanitize_html(value.as_ref())
    }

    /// Public function to encode url parameters
    pub fn encode_path(component: &str) -> String {
        crate::internal::util::encode_path(component)
    }

    /// Returns an array with all keys from input lowercased or uppercased. Numbered indices are left as is.
    pub fn mb_array_change_key_case(
        input: HashMap<String, String>,
        case: u8, // MB_CASE_UPPER or MB_CASE_LOWER
        encoding: &str,
    ) -> HashMap<String, String> {
        crate::internal::helper::mb_array_change_key_case(input, case, encoding)
    }

    /// Replaces a copy of string delimited by the start and (optionally) length parameters with the string given in replacement.
    pub fn mb_substr_replace(
        string: &str,
        replacement: &str,
        start: isize,
        length: Option<isize>,
        encoding: &str,
    ) -> String {
        crate::internal::helper::mb_substr_replace(string, replacement, start, length, encoding)
    }

    /// Replace all occurrences of the search string with the replacement string
    pub fn mb_str_replace(
        search: &str,
        replace: &str,
        subject: &str,
        encoding: &str,
    ) -> (String, usize) {
        crate::internal::helper::mb_str_replace(search, replace, subject, encoding)
    }

    /// Performs a search in a nested array
    pub fn recursive_array_search<T>(
        haystack: &[HashMap<String, T>],
        needle: &str,
        index: Option<&str>,
    ) -> Option<usize> {
        crate::internal::helper::recursive_array_search(haystack, needle, index)
    }

    /// Calculates the maximum upload size respecting system settings, free space and user quota
    pub fn max_upload_filesize(dir: &str) -> u64 {
        crate::internal::helper::max_upload_filesize(dir)
    }
}