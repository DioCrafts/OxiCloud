/**
 * Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use crate::memcache::cache::Cache;
use crate::memcache::{APC, APCu};
use rand::{distributions::Alphanumeric, Rng};
use std::sync::Arc;

pub struct APCTest {
    instance: Option<Arc<APC>>,
}

impl Default for APCTest {
    fn default() -> Self {
        Self { instance: None }
    }
}

impl Cache for APCTest {
    fn set_up(&mut self) -> Result<(), String> {
        if !APC::is_available() {
            return Err("The apc extension is not available.".to_string());
        }

        if APCu::is_available() {
            return Err("The apc extension is emulated by ACPu.".to_string());
        }

        // Generate a unique ID similar to PHP's uniqid()
        let unique_id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(13)
            .map(char::from)
            .collect();

        self.instance = Some(Arc::new(APC::new(&unique_id)));
        Ok(())
    }

    fn get_instance(&self) -> Option<Arc<dyn crate::memcache::MemcacheProvider>> {
        self.instance.clone().map(|i| i as Arc<dyn crate::memcache::MemcacheProvider>)
    }
}