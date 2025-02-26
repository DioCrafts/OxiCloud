// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use crate::hooks::basic_emitter::BasicEmitter;

/// Public emitter implementation.
pub struct PublicEmitter {
    basic_emitter: BasicEmitter,
}

impl PublicEmitter {
    /// Creates a new PublicEmitter.
    pub fn new() -> Self {
        Self {
            basic_emitter: BasicEmitter::new(),
        }
    }

    /// Emit an event to all registered listeners.
    ///
    /// # Arguments
    ///
    /// * `scope` - The scope of the event
    /// * `method` - The method/name of the event
    /// * `arguments` - Optional arguments to pass to the listeners
    pub fn emit<T>(&self, scope: &str, method: &str, arguments: Option<Vec<T>>) 
    where 
        T: Clone,
    {
        self.basic_emitter.emit(scope, method, arguments);
    }
}

impl Default for PublicEmitter {
    fn default() -> Self {
        Self::new()
    }
}