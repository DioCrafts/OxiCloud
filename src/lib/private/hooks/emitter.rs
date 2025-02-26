// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

/// Interface for all structs that are able to emit events
pub trait Emitter {
    /// Register a callback for a specific scope and method
    ///
    /// # Arguments
    ///
    /// * `scope` - The scope of the event
    /// * `method` - The method name of the event
    /// * `callback` - The callback to be executed when the event is emitted
    fn listen<F>(&mut self, scope: &str, method: &str, callback: F)
    where
        F: Fn() + 'static + Send + Sync;

    /// Remove a registered callback
    ///
    /// # Arguments
    ///
    /// * `scope` - Optional scope to match, if None all scopes match
    /// * `method` - Optional method to match, if None all methods match
    /// * `callback` - Optional specific callback to remove, if None all matching callbacks are removed
    fn remove_listener(&mut self, scope: Option<&str>, method: Option<&str>);
}