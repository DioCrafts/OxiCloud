/**
 * Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use crate::memcache::cache::Cache;
use crate::oc::memcache::memcached::Memcached as OCMemcached;
use uuid::Uuid;

pub struct Memcached {
    instance: Option<OCMemcached>,
}

impl Cache for Memcached {
    fn set_up(&mut self) {
        if !OCMemcached::is_available() {
            // In Rust, we'd typically use the test framework's skip functionality
            // This is a placeholder for how you might implement test skipping
            println!("Test skipped: The memcached extension is not available.");
            return;
        }

        let unique_id = Uuid::new_v4().to_string();
        self.instance = Some(OCMemcached::new(&unique_id));
    }
}

impl Default for Memcached {
    fn default() -> Self {
        Self { instance: None }
    }
}