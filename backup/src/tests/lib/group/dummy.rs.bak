// ownCloud
//
// @author Robin Appelman
// @copyright 2012 Robin Appelman icewind@owncloud.com
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

use crate::group::backend::GroupBackend;
use crate::group::dummy::GroupDummy;

pub struct TestGroupDummy {
    backend: Box<dyn GroupBackend>,
}

impl TestGroupDummy {
    pub fn new() -> Self {
        Self {
            backend: Box::new(GroupDummy::new()),
        }
    }
    
    pub fn set_up(&mut self) {
        self.backend = Box::new(GroupDummy::new());
    }
}

impl Default for TestGroupDummy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_up() {
        let mut test = TestGroupDummy::new();
        test.set_up();
        // Add assertions if needed
    }
}