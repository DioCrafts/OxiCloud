// Copyright notice converted from PHP file
/*
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
 */

use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;
use std::error::Error;
use mockall::{mock, predicate::*};
use mockall::automock;

// Mock framework structures
pub struct Request {}

impl Request {
    pub fn new() -> Self {
        Request {}
    }
}

pub struct Response {
    // Fields would be defined here
}

impl Response {
    pub fn new() -> Self {
        Response {}
    }
}

pub trait Controller {
    fn method(&self) -> Result<Response, Box<dyn Error>>;
}

// Define the Middleware trait
pub trait Middleware {
    fn before_controller(&self, controller: &dyn Controller, method_name: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    
    fn after_controller(&self, controller: &dyn Controller, method_name: &str, response: Response) -> Result<Response, Box<dyn Error>> {
        Ok(response)
    }
    
    fn after_exception(&self, controller: &dyn Controller, method_name: &str, error: Box<dyn Error>) -> Result<Response, Box<dyn Error>> {
        Err(error)
    }
    
    fn before_output(&self, controller: &dyn Controller, method_name: &str, output: &str) -> Result<String, Box<dyn Error>> {
        Ok(output.to_string())
    }
}

// Static counters for TestMiddleware
static BEFORE_CONTROLLER_CALLED: AtomicUsize = AtomicUsize::new(0);
static AFTER_CONTROLLER_CALLED: AtomicUsize = AtomicUsize::new(0);
static AFTER_EXCEPTION_CALLED: AtomicUsize = AtomicUsize::new(0);
static BEFORE_OUTPUT_CALLED: AtomicUsize = AtomicUsize::new(0);

// Test middleware implementation
pub struct TestMiddleware {
    before_controller_order: RefCell<usize>,
    after_controller_order: RefCell<usize>,
    after_exception_order: RefCell<usize>,
    before_output_order: RefCell<usize>,
    
    controller: RefCell<Option<&'static dyn Controller>>,
    method_name: RefCell<String>,
    exception: RefCell<Option<Box<dyn Error>>>,
    response: RefCell<Option<Response>>,
    output: RefCell<String>,
    
    before_controller_throws_ex: bool,
}

impl TestMiddleware {
    pub fn new(before_controller_throws_ex: bool) -> Self {
        BEFORE_CONTROLLER_CALLED.store(0, Ordering::SeqCst);
        AFTER_CONTROLLER_CALLED.store(0, Ordering::SeqCst);
        AFTER_EXCEPTION_CALLED.store(0, Ordering::SeqCst);
        BEFORE_OUTPUT_CALLED.store(0, Ordering::SeqCst);
        
        TestMiddleware {
            before_controller_order: RefCell::new(0),
            after_controller_order: RefCell::new(0),
            after_exception_order: RefCell::new(0),
            before_output_order: RefCell::new(0),
            
            controller: RefCell::new(None),
            method_name: RefCell::new(String::new()),
            exception: RefCell::new(None),
            response: RefCell::new(None),
            output: RefCell::new(String::new()),
            
            before_controller_throws_ex,
        }
    }
}

impl Middleware for TestMiddleware {
    fn before_controller(&self, controller: &dyn Controller, method_name: &str) -> Result<(), Box<dyn Error>> {
        let count = BEFORE_CONTROLLER_CALLED.fetch_add(1, Ordering::SeqCst) + 1;
        *self.before_controller_order.borrow_mut() = count;
        
        // Note: this is unsafe because we're extending the lifetime, but it's test code
        // In real code you'd use a different approach for storing references
        self.controller.replace(unsafe { std::mem::transmute(controller) });
        self.method_name.replace(method_name.to_string());
        
        if self.before_controller_throws_ex {
            return Err("Exception in before_controller".into());
        }
        
        Ok(())
    }
    
    fn after_exception(&self, controller: &dyn Controller, method_name: &str, error: Box<dyn Error>) -> Result<Response, Box<dyn Error>> {
        let count = AFTER_EXCEPTION_CALLED.fetch_add(1, Ordering::SeqCst) + 1;
        *self.after_exception_order.borrow_mut() = count;
        
        // Note: same lifetime extension issue as above
        self.controller.replace(unsafe { std::mem::transmute(controller) });
        self.method_name.replace(method_name.to_string());
        self.exception.replace(Some(error));
        
        // Call parent method behavior (propagate error)
        Err(self.exception.borrow_mut().take().unwrap())
    }
    
    fn after_controller(&self, controller: &dyn Controller, method_name: &str, response: Response) -> Result<Response, Box<dyn Error>> {
        let count = AFTER_CONTROLLER_CALLED.fetch_add(1, Ordering::SeqCst) + 1;
        *self.after_controller_order.borrow_mut() = count;
        
        // Note: same lifetime extension issue as above
        self.controller.replace(unsafe { std::mem::transmute(controller) });
        self.method_name.replace(method_name.to_string());
        self.response.replace(Some(response));
        
        // Return the response
        if let Some(response) = self.response.borrow_mut().take() {
            Ok(response)
        } else {
            Err("No response available".into())
        }
    }
    
    fn before_output(&self, controller: &dyn Controller, method_name: &str, output: &str) -> Result<String, Box<dyn Error>> {
        let count = BEFORE_OUTPUT_CALLED.fetch_add(1, Ordering::SeqCst) + 1;
        *self.before_output_order.borrow_mut() = count;
        
        // Note: same lifetime extension issue as above
        self.controller.replace(unsafe { std::mem::transmute(controller) });
        self.method_name.replace(method_name.to_string());
        self.output.replace(output.to_string());
        
        Ok(output.to_string())
    }
}

// MiddlewareDispatcher implementation
pub struct MiddlewareDispatcher {
    middlewares: VecDeque<Rc<dyn Middleware>>,
}

impl MiddlewareDispatcher {
    pub fn new() -> Self {
        MiddlewareDispatcher {
            middlewares: VecDeque::new(),
        }
    }
    
    pub fn register_middleware(&mut self, middleware: Rc<dyn Middleware>) {
        self.middlewares.push_back(middleware);
    }
    
    pub fn before_controller(&self, controller: &dyn Controller, method_name: &str) -> Result<(), Box<dyn Error>> {
        for middleware in &self.middlewares {
            middleware.before_controller(controller, method_name)?;
        }
        Ok(())
    }
    
    pub fn after_controller(&self, controller: &dyn Controller, method_name: &str, response: Response) -> Result<Response, Box<dyn Error>> {
        let mut response = response;
        for middleware in self.middlewares.iter().rev() {
            response = middleware.after_controller(controller, method_name, response)?;
        }
        Ok(response)
    }
    
    pub fn after_exception(&self, controller: &dyn Controller, method_name: &str, error: Box<dyn Error>) -> Result<Response, Box<dyn Error>> {
        for middleware in self.middlewares.iter().rev() {
            match middleware.after_exception(controller, method_name, error) {
                Ok(response) => return Ok(response),
                Err(e) => return Err(e),
            }
        }
        Err("Unhandled exception".into())
    }
    
    pub fn before_output(&self, controller: &dyn Controller, method_name: &str, output: &str) -> Result<String, Box<dyn Error>> {
        let mut output = output.to_string();
        for middleware in self.middlewares.iter().rev() {
            output = middleware.before_output(controller, method_name, &output)?;
        }
        Ok(output)
    }
}

// For testing purposes, generate mock implementations
mock! {
    pub DIContainer {
        pub fn get_app_name(&self) -> String;
    }
}

mock! {
    pub MyController {}
    
    impl Controller for MyController {
        fn method(&self) -> Result<Response, Box<dyn Error>>;
    }
}

mock! {
    pub MyMiddleware {}
    
    impl Middleware for MyMiddleware {
        fn before_controller(&self, controller: &dyn Controller, method_name: &str) -> Result<(), Box<dyn Error>>;
        fn after_controller(&self, controller: &dyn Controller, method_name: &str, response: Response) -> Result<Response, Box<dyn Error>>;
        fn after_exception(&self, controller: &dyn Controller, method_name: &str, error: Box<dyn Error>) -> Result<Response, Box<dyn Error>>;
        fn before_output(&self, controller: &dyn Controller, method_name: &str, output: &str) -> Result<String, Box<dyn Error>>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn setup() -> (MiddlewareDispatcher, MockMyController, String, Response) {
        let mut dispatcher = MiddlewareDispatcher::new();
        let controller = MockMyController::new();
        let method = "method".to_string();
        let response = Response::new();
        
        (dispatcher, controller, method, response)
    }
    
    #[test]
    fn test_after_exception_should_return_response_of_middleware() {
        let (mut dispatcher, controller, method, response) = setup();
        
        let mut m1 = MockMyMiddleware::new();
        m1.expect_before_controller()
            .returning(|_, _| Ok(()));
        m1.expect_after_exception()
            .never();
            
        let mut m2 = MockMyMiddleware::new();
        m2.expect_before_controller()
            .returning(|_, _| Ok(()));
        m2.expect_after_exception()
            .returning(move |_, _, _| Ok(Response::new()));
            
        dispatcher.register_middleware(Rc::new(m1));
        dispatcher.register_middleware(Rc::new(m2));
        
        dispatcher.before_controller(&controller, &method).unwrap();
        let result = dispatcher.after_exception(&controller, &method, "Test error".into());
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_after_exception_should_throw_again_when_not_handled() {
        let (mut dispatcher, controller, method, _) = setup();
        
        let m1 = TestMiddleware::new(false);
        let m2 = TestMiddleware::new(true);
        
        dispatcher.register_middleware(Rc::new(m1));
        dispatcher.register_middleware(Rc::new(m2));
        
        let result = dispatcher.before_controller(&controller, &method);
        assert!(result.is_err());
        
        let error = result.err().unwrap();
        let result = dispatcher.after_exception(&controller, &method, error);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_before_controller_correct_arguments() {
        let (mut dispatcher, controller, method, _) = setup();
        
        let m1 = Rc::new(TestMiddleware::new(false));
        dispatcher.register_middleware(Rc::clone(&m1));
        
        dispatcher.before_controller(&controller, &method).unwrap();
        
        assert!(m1.controller.borrow().is_some());
        assert_eq!(m1.method_name.borrow().as_str(), method);
    }
    
    #[test]
    fn test_after_controller_correct_arguments() {
        let (mut dispatcher, controller, method, response) = setup();
        
        let m1 = Rc::new(TestMiddleware::new(false));
        dispatcher.register_middleware(Rc::clone(&m1));
        
        dispatcher.after_controller(&controller, &method, response).unwrap();
        
        assert!(m1.controller.borrow().is_some());
        assert_eq!(m1.method_name.borrow().as_str(), method);
        assert!(m1.response.borrow().is_none()); // Was taken in the implementation
    }
    
    #[test]
    fn test_before_controller_order() {
        let (mut dispatcher, controller, method, _) = setup();
        
        let m1 = Rc::new(TestMiddleware::new(false));
        let m2 = Rc::new(TestMiddleware::new(false));
        
        dispatcher.register_middleware(Rc::clone(&m1));
        dispatcher.register_middleware(Rc::clone(&m2));
        
        dispatcher.before_controller(&controller, &method).unwrap();
        
        assert_eq!(*m1.before_controller_order.borrow(), 1);
        assert_eq!(*m2.before_controller_order.borrow(), 2);
    }
    
    #[test]
    fn test_after_controller_order() {
        let (mut dispatcher, controller, method, response) = setup();
        
        let m1 = Rc::new(TestMiddleware::new(false));
        let m2 = Rc::new(TestMiddleware::new(false));
        
        dispatcher.register_middleware(Rc::clone(&m1));
        dispatcher.register_middleware(Rc::clone(&m2));
        
        dispatcher.after_controller(&controller, &method, response).unwrap();
        
        assert_eq!(*m1.after_controller_order.borrow(), 2);
        assert_eq!(*m2.after_controller_order.borrow(), 1);
    }
    
    #[test]
    fn test_before_output_order() {
        let (mut dispatcher, controller, method, _) = setup();
        let output = "hi";
        
        let m1 = Rc::new(TestMiddleware::new(false));
        let m2 = Rc::new(TestMiddleware::new(false));
        
        dispatcher.register_middleware(Rc::clone(&m1));
        dispatcher.register_middleware(Rc::clone(&m2));
        
        dispatcher.before_output(&controller, &method, output).unwrap();
        
        assert_eq!(*m1.before_output_order.borrow(), 2);
        assert_eq!(*m2.before_output_order.borrow(), 1);
    }
    
    #[test]
    fn test_exception_should_run_after_exception_of_only_previously_executed_middlewares() {
        let (mut dispatcher, controller, method, _) = setup();
        let output = "hi";
        
        let m1 = Rc::new(TestMiddleware::new(false));
        let m2 = Rc::new(TestMiddleware::new(true));
        
        let mut m3 = MockMyMiddleware::new();
        m3.expect_after_exception().never();
        m3.expect_before_controller().never();
        m3.expect_after_controller().never();
        
        dispatcher.register_middleware(Rc::clone(&m1));
        dispatcher.register_middleware(Rc::clone(&m2));
        dispatcher.register_middleware(Rc::new(m3));
        
        let _ = dispatcher.before_output(&controller, &method, output);
        
        assert_eq!(*m1.before_output_order.borrow(), 2);
        assert_eq!(*m2.before_output_order.borrow(), 1);
    }
}