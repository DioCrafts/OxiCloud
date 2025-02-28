// Módulos generados automáticamente

pub mod fileglobalgc;

// Contenido fusionado desde src/lib/private/legacy/cache.rs
/**
 * Copyright (c) 2013 Thomas Tanghus (thomas@tanghus.net)
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use crate::oc::Cache;

pub struct OcCache {
    inner: Cache,
}

impl OcCache {
    pub fn new(inner: Cache) -> Self {
        Self { inner }
    }
}

impl std::ops::Deref for OcCache {
    type Target = Cache;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for OcCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<Cache> for OcCache {
    fn from(inner: Cache) -> Self {
        Self::new(inner)
    }
}