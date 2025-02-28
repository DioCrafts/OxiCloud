/*
 * Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use crate::session::Session;
use oc::session::Memory as OCMemory;
use uuid::Uuid;

pub struct Memory {
    instance: OCMemory,
}

impl Session for Memory {
    fn set_up(&mut self) {
        self.instance = OCMemory::new(Uuid::new_v4().to_string());
    }
}

impl Memory {
    pub fn new() -> Self {
        Self {
            instance: OCMemory::new(Uuid::new_v4().to_string()),
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}