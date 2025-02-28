// # TimeFactory Module
//
// Originally from ownCloud - App Framework
// Author: Bernhard Posselt
// Copyright: 2012 Bernhard Posselt nukeawhale@gmail.com
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::time::{SystemTime, UNIX_EPOCH};

/// Provides time related functionality, making it easier to mock time() calls in tests
#[derive(Default, Clone, Debug)]
pub struct TimeFactory;

impl TimeFactory {
    /// Creates a new TimeFactory instance
    pub fn new() -> Self {
        Self
    }

    /// Returns the current Unix timestamp
    ///
    /// # Returns
    /// The current Unix timestamp as u64
    pub fn get_time(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time is before Unix epoch")
            .as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_time_returns_timestamp() {
        let factory = TimeFactory::new();
        let time = factory.get_time();
        assert!(time > 0);
    }
}