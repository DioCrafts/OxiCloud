/**
 * Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use std::collections::HashMap;

pub mod oc_hook {
    use std::collections::HashMap;
    
    pub fn emit(scope: &str, method: &str, arguments: &HashMap<String, String>) {
        // This would be the implementation of the OC_Hook::emit function
        // Since we don't have access to the original implementation, this is a placeholder
    }
}

pub trait Emitter {
    fn emit(&self, scope: &str, method: &str, arguments: &HashMap<String, String>);
}

pub struct BasicEmitter;

impl Emitter for BasicEmitter {
    fn emit(&self, _scope: &str, _method: &str, _arguments: &HashMap<String, String>) {
        // Implementation of BasicEmitter's emit method
        // This is a placeholder as we don't have the original implementation
    }
}

pub struct LegacyEmitter {
    basic_emitter: BasicEmitter,
}

impl LegacyEmitter {
    pub fn new() -> Self {
        Self {
            basic_emitter: BasicEmitter,
        }
    }
}

impl Emitter for LegacyEmitter {
    fn emit(&self, scope: &str, method: &str, arguments: &HashMap<String, String>) {
        oc_hook::emit(scope, method, arguments);
        self.basic_emitter.emit(scope, method, arguments);
    }
}