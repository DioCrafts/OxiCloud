use std::collections::HashMap;
use mockall::{automock, mock, predicate::*};
use rstest::*;

/**
 * ownCloud - App Framework
 *
 * @author Bernhard Posselt
 * @copyright 2012 Bernhard Posselt nukeawhale@gmail.com
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU AFFERO GENERAL PUBLIC LICENSE for more details.
 *
 * You should have received a copy of the GNU Affero General Public
 * License along with this library.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

// Define the traits and structs needed for our test

#[automock]
pub trait Controller {}

#[automock]
pub trait Dispatcher {
    fn dispatch(&self, controller: &dyn Controller, method: &str) -> (Option<()>, HashMap<String, String>, Option<String>);
}

pub struct DIContainer {
    app_name: String,
    services: HashMap<String, Box<dyn std::any::Any>>,
}

impl DIContainer {
    pub fn new(app_name: &str, _services: HashMap<String, String>) -> Self {
        Self {
            app_name: app_name.to_string(),
            services: HashMap::new(),
        }
    }

    pub fn register<T: 'static>(&mut self, name: &str, service: T) {
        self.services.insert(name.to_string(), Box::new(service));
    }

    pub fn get<T: 'static>(&self, name: &str) -> Option<&T> {
        self.services.get(name).and_then(|s| s.downcast_ref::<T>())
    }
}

pub struct App;

impl App {
    pub fn main(controller_name: &str, controller_method: &str, container: &DIContainer) -> String {
        let controller = container.get::<Box<dyn Controller>>(controller_name).expect("Controller not found");
        let dispatcher = container.get::<Box<dyn Dispatcher>>("Dispatcher").expect("Dispatcher not found");
        
        let (_, _, output) = dispatcher.dispatch(controller.as_ref(), controller_method);
        
        // In a real implementation, headers would be processed here
        
        output.unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct AppTest {
        container: DIContainer,
        controller: MockController,
        dispatcher: MockDispatcher,
        headers: HashMap<String, String>,
        output: String,
        controller_name: String,
        controller_method: String,
    }
    
    impl AppTest {
        fn setup() -> Self {
            let mut test = AppTest {
                container: DIContainer::new("test", HashMap::new()),
                controller: MockController::new(),
                dispatcher: MockDispatcher::new(),
                headers: [("key".to_string(), "value".to_string())].iter().cloned().collect(),
                output: "hi".to_string(),
                controller_name: "Controller".to_string(),
                controller_method: "method".to_string(),
            };
            
            test.container.register(&test.controller_name, Box::new(test.controller.clone()) as Box<dyn Controller>);
            test.container.register("Dispatcher", Box::new(test.dispatcher.clone()) as Box<dyn Dispatcher>);
            test.container.register("urlParams", HashMap::<String, String>::new());
            
            test
        }
    }
    
    #[test]
    fn test_controller_name_and_method_are_being_passed() {
        let mut test = AppTest::setup();
        
        let return_value = (None, HashMap::new(), None);
        test.dispatcher.expect_dispatch()
            .with(predicate::always(), predicate::eq("method"))
            .times(1)
            .return_const(return_value);
        
        let output = App::main(&test.controller_name, &test.controller_method, &test.container);
        
        assert_eq!(output, "");
    }
    
    /*
    FIXME: this complains about shit headers which are already sent because
    of the content length. Would be cool if someone could fix this
    
    #[test]
    fn test_output_is_printed() {
        let mut test = AppTest::setup();
        
        let mut headers = HashMap::new();
        let return_value = (None, headers, Some(test.output.clone()));
        test.dispatcher.expect_dispatch()
            .with(predicate::always(), predicate::eq("method"))
            .times(1)
            .return_const(return_value);
        
        let output = App::main(&test.controller_name, &test.controller_method, &test.container);
        
        assert_eq!(output, test.output);
    }
    */
    
    // FIXME: if someone manages to test the headers output, I'd be grateful
}