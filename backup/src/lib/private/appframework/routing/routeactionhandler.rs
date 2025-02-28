// Copyright (c) 2013 Thomas Müller <thomas.mueller@tmit.eu>
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

use std::collections::HashMap;
use std::rc::Rc;

use crate::appframework::app;
use crate::appframework::dependency_injection::di_container::DIContainer;

/// Handler for route actions in the AppFramework
pub struct RouteActionHandler {
    controller_name: String,
    action_name: String,
    container: Rc<DIContainer>,
}

impl RouteActionHandler {
    /// Create a new RouteActionHandler with the specified container, controller and action
    pub fn new(container: Rc<DIContainer>, controller_name: &str, action_name: &str) -> Self {
        RouteActionHandler {
            controller_name: controller_name.to_string(),
            action_name: action_name.to_string(),
            container,
        }
    }

    /// Invokes the handler with the given parameters
    pub fn call(&self, params: HashMap<String, String>) {
        app::main(
            &self.controller_name,
            &self.action_name,
            params,
            Rc::clone(&self.container),
        );
    }
}

impl FnOnce<(HashMap<String, String>,)> for RouteActionHandler {
    type Output = ();

    extern "rust-call" fn call_once(self, args: (HashMap<String, String>,)) -> Self::Output {
        let params = args.0;
        app::main(
            &self.controller_name,
            &self.action_name,
            params,
            self.container,
        );
    }
}

impl FnMut<(HashMap<String, String>,)> for RouteActionHandler {
    extern "rust-call" fn call_mut(&mut self, args: (HashMap<String, String>,)) -> Self::Output {
        let params = args.0;
        app::main(
            &self.controller_name,
            &self.action_name,
            params,
            Rc::clone(&self.container),
        );
    }
}

impl Fn<(HashMap<String, String>,)> for RouteActionHandler {
    extern "rust-call" fn call(&self, args: (HashMap<String, String>,)) -> Self::Output {
        let params = args.0;
        app::main(
            &self.controller_name,
            &self.action_name,
            params,
            Rc::clone(&self.container),
        );
    }
}