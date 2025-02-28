// Copyright (C) 2012 Robin Appelman <icewind@owncloud.com>
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

use crate::user::backend::TestUserBackend;
use owncloud::user::dummy::OcUserDummy;

pub struct TestUserDummy {
    backend: Option<OcUserDummy>,
}

impl TestUserDummy {
    pub fn new() -> Self {
        Self { backend: None }
    }
}

impl TestUserBackend for TestUserDummy {
    fn set_up(&mut self) {
        self.backend = Some(OcUserDummy::new());
    }
    
    fn get_backend(&self) -> Option<&OcUserDummy> {
        self.backend.as_ref()
    }
    
    fn get_backend_mut(&mut self) -> Option<&mut OcUserDummy> {
        self.backend.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_set_up() {
        let mut test = TestUserDummy::new();
        test.set_up();
        assert!(test.get_backend().is_some());
    }
}