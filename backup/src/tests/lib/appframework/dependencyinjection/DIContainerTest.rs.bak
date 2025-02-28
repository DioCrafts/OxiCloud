/// ownCloud - App Framework
///
/// @author Bernhard Posselt
/// @author Morris Jobke
/// @copyright 2012 Bernhard Posselt nukeawhale@gmail.com
/// @copyright 2013 Morris Jobke morris.jobke@gmail.com
///
/// This library is free software; you can redistribute it and/or
/// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
/// License as published by the Free Software Foundation; either
/// version 3 of the License, or any later version.
///
/// This library is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
///
/// You should have received a copy of the GNU Affero General Public
/// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use mockall::predicate::*;
use mockall::*;

use crate::appframework::core::api::API;
use crate::appframework::dependencyinjection::di_container::DIContainer;
use crate::appframework::http::middleware::dispatcher::MiddlewareDispatcher;
use crate::appframework::http::middleware::security::SecurityMiddleware;
use crate::appframework::http::request::Request;

mock! {
    pub API {
        fn new(app_name: &str) -> Self;
        // Add other methods as needed
    }
}

#[cfg(test)]
mod di_container_test {
    use super::*;

    struct TestContext {
        container: DIContainer,
        api: MockAPI,
    }

    impl TestContext {
        fn setup() -> Self {
            let api = MockAPI::new();
            let container = DIContainer::new("name");
            
            Self {
                container,
                api,
            }
        }
    }

    #[test]
    fn test_provides_api() {
        let ctx = TestContext::setup();
        assert!(ctx.container.has("API"));
    }

    #[test]
    fn test_provides_request() {
        let ctx = TestContext::setup();
        assert!(ctx.container.has("Request"));
    }

    #[test]
    fn test_provides_security_middleware() {
        let ctx = TestContext::setup();
        assert!(ctx.container.has("SecurityMiddleware"));
    }

    #[test]
    fn test_provides_middleware_dispatcher() {
        let ctx = TestContext::setup();
        assert!(ctx.container.has("MiddlewareDispatcher"));
    }

    #[test]
    fn test_provides_app_name() {
        let ctx = TestContext::setup();
        assert!(ctx.container.has("AppName"));
    }

    #[test]
    fn test_app_name_is_set_correctly() {
        let ctx = TestContext::setup();
        let app_name = ctx.container.get::<String>("AppName").unwrap();
        assert_eq!("name", *app_name);
    }

    #[test]
    fn test_middleware_dispatcher_includes_security_middleware() {
        let mut ctx = TestContext::setup();
        
        // Set Request in container
        let request = Request::new();
        ctx.container.register("Request", Rc::new(RefCell::new(request)));
        
        // Get security middleware and dispatcher
        let security = ctx.container.get::<Rc<SecurityMiddleware>>("SecurityMiddleware").unwrap();
        let dispatcher = ctx.container.get::<Rc<MiddlewareDispatcher>>("MiddlewareDispatcher").unwrap();
        
        // Check if security middleware is contained in dispatcher
        assert!(dispatcher.has_middleware(&*security));
    }
}