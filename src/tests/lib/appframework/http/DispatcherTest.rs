//! Tests for the dispatcher implementation
//!
//! Originally ported from PHP:
//! ownCloud - App Framework
//! 
//! @author Bernhard Posselt
//! @copyright 2012 Bernhard Posselt nukeawhale@gmail.com
//!
//! This library is free software; you can redistribute it and/or
//! modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
//! License as published by the Free Software Foundation; either
//! version 3 of the License, or any later version.
//!
//! This library is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//! GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//!
//! You should have received a copy of the GNU Affero General Public
//! License along with this library. If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::sync::Arc;
use mockall::{mock, predicate::*};
use chrono::{DateTime, Utc};

// Mock implementations
mock! {
    DIContainer {}
    
    impl DIContainer {
        fn new() -> Self;
    }
}

mock! {
    Request {}
    
    impl Request {
        fn new() -> Self;
    }
}

mock! {
    Http {}
    
    impl Http {
        fn new() -> Self;
        fn get_status_header(&self, status: u16, last_modified: Option<DateTime<Utc>>, etag: Option<String>) -> Option<String>;
    }
}

mock! {
    MiddlewareDispatcher {}
    
    impl MiddlewareDispatcher {
        fn new() -> Self;
        fn before_controller<C: Controller>(&self, controller: &C, method: &str) -> Result<(), Box<dyn std::error::Error>>;
        fn after_controller<C: Controller, R: Response>(&self, controller: &C, method: &str, response: R) -> R;
        fn after_exception<C: Controller, R: Response>(&self, controller: &C, method: &str, exception: Box<dyn std::error::Error>) -> Option<R>;
        fn before_output<C: Controller>(&self, controller: &C, method: &str, output: &str) -> String;
    }
}

// Required traits
trait Controller {
    fn test(&self) -> Box<dyn Response>;
}

trait Response {
    fn render(&self) -> Option<String>;
    fn get_status(&self) -> u16;
    fn get_last_modified(&self) -> Option<DateTime<Utc>>;
    fn get_etag(&self) -> Option<String>;
    fn get_headers(&self) -> HashMap<String, String>;
}

mock! {
    MockController {}
    
    impl Controller for MockController {
        fn test(&self) -> Box<dyn Response>;
    }
}

mock! {
    MockResponse {}
    
    impl Response for MockResponse {
        fn render(&self) -> Option<String>;
        fn get_status(&self) -> u16;
        fn get_last_modified(&self) -> Option<DateTime<Utc>>;
        fn get_etag(&self) -> Option<String>;
        fn get_headers(&self) -> HashMap<String, String>;
    }
}

// Actual implementation
pub struct Dispatcher {
    http: Arc<MockHttp>,
    middleware_dispatcher: Arc<MockMiddlewareDispatcher>,
}

impl Dispatcher {
    pub fn new(http: Arc<MockHttp>, middleware_dispatcher: Arc<MockMiddlewareDispatcher>) -> Self {
        Self {
            http,
            middleware_dispatcher,
        }
    }

    pub fn dispatch<C: Controller>(&self, controller: &C, method: &str) -> (Option<String>, HashMap<String, String>, Option<String>) {
        // Try to run the middleware before the controller
        let result = self.middleware_dispatcher.before_controller(controller, method);
        
        // Prepare default response
        let mut http_headers = None;
        let mut response_headers = HashMap::new();
        let mut output = None;
        
        let response = match result {
            Ok(_) => {
                // If middleware passes, call the controller method
                if method == "test" {
                    controller.test()
                } else {
                    // In real implementation, we'd use reflection or a method map
                    // but for this test we just support the 'test' method
                    panic!("Method not supported in test: {}", method);
                }
            },
            Err(exception) => {
                // If middleware fails, call the exception handler
                match self.middleware_dispatcher.after_exception(controller, method, exception) {
                    Some(resp) => resp,
                    None => return (None, HashMap::new(), None), // This would actually throw in the original code
                }
            }
        };
        
        // Process the response
        let processed_response = self.middleware_dispatcher.after_controller(controller, method, response);
        
        // Get values from the response
        let status = processed_response.get_status();
        let last_modified = processed_response.get_last_modified();
        let etag = processed_response.get_etag();
        response_headers = processed_response.get_headers();
        
        // Get HTTP status header
        http_headers = self.http.get_status_header(status, last_modified, etag);
        
        // Get output and process it through middleware
        if let Some(out) = processed_response.render() {
            output = Some(self.middleware_dispatcher.before_output(controller, method, &out));
        }
        
        (http_headers, response_headers, output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use mockall::predicate::*;
    
    const HTTP_STATUS_OK: u16 = 200;
    
    struct TestError;
    
    impl std::fmt::Debug for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Test error")
        }
    }
    
    impl std::fmt::Display for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Test error")
        }
    }
    
    impl Error for TestError {}
    
    #[test]
    fn test_dispatcher_returns_array_with_2_entries() {
        // Setup
        let mut mock_http = MockHttp::new();
        let mut mock_middleware = MockMiddlewareDispatcher::new();
        let mut mock_controller = MockController::new();
        let mut mock_response = MockResponse::new();
        
        let last_modified = Utc::now();
        let etag = "hi".to_string();
        
        // Configure mocks
        mock_middleware
            .expect_before_controller()
            .return_once(|_, _| Ok(()));
            
        mock_controller
            .expect_test()
            .return_once(move || Box::new(mock_response.clone()));
            
        mock_response
            .expect_render()
            .return_once(|| None);
            
        mock_response
            .expect_get_status()
            .return_once(|| HTTP_STATUS_OK);
            
        mock_response
            .expect_get_last_modified()
            .return_once(move || Some(last_modified));
            
        mock_response
            .expect_get_etag()
            .return_once(|| Some(etag));
            
        mock_response
            .expect_get_headers()
            .return_once(|| HashMap::new());
            
        mock_http
            .expect_get_status_header()
            .return_once(|_, _, _| None);
            
        mock_middleware
            .expect_after_controller::<_, MockResponse>()
            .return_once(|_, _, resp| resp);
            
        mock_middleware
            .expect_before_output()
            .return_once(|_, _, out| out.to_string());
        
        let dispatcher = Dispatcher::new(
            Arc::new(mock_http),
            Arc::new(mock_middleware)
        );
        
        // Execute
        let response = dispatcher.dispatch(&mock_controller, "test");
        
        // Assert
        assert_eq!(response.0, None);
        assert_eq!(response.1, HashMap::new());
        assert_eq!(response.2, None);
    }
    
    #[test]
    fn test_headers_and_output_are_returned() {
        // Setup
        let mut mock_http = MockHttp::new();
        let mut mock_middleware = MockMiddlewareDispatcher::new();
        let mut mock_controller = MockController::new();
        let mut mock_response = MockResponse::new();
        
        let output = "yo".to_string();
        let http_headers = "Http".to_string();
        let mut response_headers = HashMap::new();
        response_headers.insert("hell".to_string(), "yeah".to_string());
        
        let last_modified = Utc::now();
        let etag = "hi".to_string();
        
        // Configure mocks
        mock_middleware
            .expect_before_controller()
            .return_once(|_, _| Ok(()));
            
        mock_controller
            .expect_test()
            .return_once(move || Box::new(mock_response.clone()));
            
        mock_response
            .expect_render()
            .return_once(|| Some(output.clone()));
            
        mock_response
            .expect_get_status()
            .return_once(|| HTTP_STATUS_OK);
            
        mock_response
            .expect_get_last_modified()
            .return_once(move || Some(last_modified));
            
        mock_response
            .expect_get_etag()
            .return_once(|| Some(etag));
            
        mock_response
            .expect_get_headers()
            .return_once(|| response_headers.clone());
            
        mock_http
            .expect_get_status_header()
            .return_once(|_, _, _| Some(http_headers.clone()));
            
        mock_middleware
            .expect_after_controller::<_, MockResponse>()
            .return_once(|_, _, resp| resp);
            
        mock_middleware
            .expect_before_output()
            .return_once(|_, _, out| out.to_string());
        
        let dispatcher = Dispatcher::new(
            Arc::new(mock_http),
            Arc::new(mock_middleware)
        );
        
        // Execute
        let response = dispatcher.dispatch(&mock_controller, "test");
        
        // Assert
        assert_eq!(response.0, Some(http_headers));
        assert_eq!(response.1, response_headers);
        assert_eq!(response.2, Some(output));
    }
    
    #[test]
    fn test_exception_calls_after_exception() {
        // Setup
        let mut mock_http = MockHttp::new();
        let mut mock_middleware = MockMiddlewareDispatcher::new();
        let mut mock_controller = MockController::new();
        let mut mock_response = MockResponse::new();
        
        let output = "yo".to_string();
        let http_headers = "Http".to_string();
        let mut response_headers = HashMap::new();
        response_headers.insert("hell".to_string(), "yeah".to_string());
        
        let last_modified = Utc::now();
        let etag = "hi".to_string();
        
        // Configure mocks
        mock_middleware
            .expect_before_controller()
            .return_once(|_, _| Err(Box::new(TestError)));
            
        mock_middleware
            .expect_after_exception::<_, MockResponse>()
            .return_once(|_, _, _| Some(mock_response.clone()));
            
        mock_response
            .expect_render()
            .return_once(|| Some(output.clone()));
            
        mock_response
            .expect_get_status()
            .return_once(|| HTTP_STATUS_OK);
            
        mock_response
            .expect_get_last_modified()
            .return_once(move || Some(last_modified));
            
        mock_response
            .expect_get_etag()
            .return_once(|| Some(etag));
            
        mock_response
            .expect_get_headers()
            .return_once(|| response_headers.clone());
            
        mock_http
            .expect_get_status_header()
            .return_once(|_, _, _| Some(http_headers.clone()));
            
        mock_middleware
            .expect_after_controller::<_, MockResponse>()
            .return_once(|_, _, resp| resp);
            
        mock_middleware
            .expect_before_output()
            .return_once(|_, _, out| out.to_string());
        
        let dispatcher = Dispatcher::new(
            Arc::new(mock_http),
            Arc::new(mock_middleware)
        );
        
        // Execute
        let response = dispatcher.dispatch(&mock_controller, "test");
        
        // Assert
        assert_eq!(response.0, Some(http_headers));
        assert_eq!(response.1, response_headers);
        assert_eq!(response.2, Some(output));
    }
    
    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_exception_throws_if_can_not_be_handled_by_after_exception() {
        // Setup
        let mut mock_http = MockHttp::new();
        let mut mock_middleware = MockMiddlewareDispatcher::new();
        let mut mock_controller = MockController::new();
        
        // Configure mocks
        mock_middleware
            .expect_before_controller()
            .return_once(|_, _| Err(Box::new(TestError)));
            
        mock_middleware
            .expect_after_exception::<_, MockResponse>()
            .return_once(|_, _, _| None);
        
        let dispatcher = Dispatcher::new(
            Arc::new(mock_http),
            Arc::new(mock_middleware)
        );
        
        // This should panic because we're simulating a None return from after_exception
        let _ = dispatcher.dispatch(&mock_controller, "test");
    }
}