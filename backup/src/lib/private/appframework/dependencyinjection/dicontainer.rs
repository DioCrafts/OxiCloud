// lib/private/appframework/dependencyinjection/dicontainer.rs

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

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use crate::appframework::http::Http;
use crate::appframework::http::Request;
use crate::appframework::http::Dispatcher;
use crate::appframework::core::API;
use crate::appframework::middleware::middleware_dispatcher::MiddlewareDispatcher;
use crate::appframework::middleware::security::security_middleware::SecurityMiddleware;
use crate::appframework::utility::simple_container::SimpleContainer;
use crate::appframework::utility::time_factory::TimeFactory;
use crate::public::appframework::iapi::IApi;
use crate::public::appframework::iapp_container::IAppContainer;
use crate::public::appframework::imiddle_ware::IMiddleWare;
use crate::public::appframework::middleware::Middleware;
use crate::public::iserver_container::IServerContainer;
use crate::session;
use crate::user;
use crate::util;

pub struct DIContainer {
    container: SimpleContainer,
    middleware_vec: RefCell<Vec<Rc<dyn Middleware>>>,
    app_name: String,
}

impl DIContainer {
    /// Put your class dependencies in here
    /// 
    /// # Arguments
    ///
    /// * `app_name` - the name of the app
    /// * `url_params` - URL parameters
    pub fn new(app_name: String, url_params: HashMap<String, String>) -> Self {
        let mut container = SimpleContainer::new();

        container.register_parameter("AppName".to_string(), Box::new(app_name.clone()));
        container.register_parameter("urlParams".to_string(), Box::new(url_params));
        
        // This would need to be properly initialized from global state in a real implementation
        let server = Arc::new(get_oc_server());
        container.register_parameter("ServerContainer".to_string(), Box::new(server.clone()));

        let di_container = DIContainer {
            container,
            middleware_vec: RefCell::new(Vec::new()),
            app_name,
        };
        
        di_container.register_services();
        
        di_container
    }

    fn register_services(&self) {
        let app_name = self.app_name.clone();
        
        self.container.register_shared("API", move |c: &SimpleContainer| {
            Box::new(API::new(app_name.clone()))
        });
        
        // Http
        self.container.register_shared("Request", |c: &SimpleContainer| {
            let server = c.query::<Arc<dyn IServerContainer>>("ServerContainer").unwrap();
            let url_params = c.query::<HashMap<String, String>>("urlParams").unwrap();
            server.register_parameter("urlParams".to_string(), Box::new(url_params.clone()));
            Box::new(server.get_request())
        });
        
        self.container.register_shared("Protocol", |_: &SimpleContainer| {
            // In a real implementation, we would need to get server variables from the environment
            let server_vars = HashMap::<String, String>::new();
            let protocol = server_vars.get("SERVER_PROTOCOL").cloned();
            
            if let Some(protocol) = protocol {
                Box::new(Http::new(server_vars, protocol))
            } else {
                Box::new(Http::new_default(server_vars))
            }
        });
        
        self.container.register_shared("Dispatcher", |c: &SimpleContainer| {
            let protocol = c.query::<Http>("Protocol").unwrap();
            let middleware_dispatcher = c.query::<MiddlewareDispatcher>("MiddlewareDispatcher").unwrap();
            Box::new(Dispatcher::new(protocol, middleware_dispatcher))
        });
        
        // Middleware
        let container_ref = &self.container;
        self.container.register_shared("SecurityMiddleware", move |c: &SimpleContainer| {
            let request = c.query::<Request>("Request").unwrap();
            Box::new(SecurityMiddleware::new(container_ref, request))
        });
        
        let middleware_vec = self.middleware_vec.clone();
        self.container.register_shared("MiddlewareDispatcher", move |c: &SimpleContainer| {
            let mut dispatcher = MiddlewareDispatcher::new();
            let security_middleware = c.query::<SecurityMiddleware>("SecurityMiddleware").unwrap();
            dispatcher.register_middleware(Rc::new(security_middleware));
            
            for middleware in middleware_vec.borrow().iter() {
                dispatcher.register_middleware(middleware.clone());
            }
            
            Box::new(dispatcher)
        });
        
        // Utilities
        self.container.register_shared("TimeFactory", |_: &SimpleContainer| {
            Box::new(TimeFactory::new())
        });
    }

    fn get_user_id(&self) -> Option<String> {
        session::get_user_id()
    }
}

impl IAppContainer for DIContainer {
    fn get_core_api(&self) -> Arc<dyn IApi> {
        Arc::new(self.container.query::<API>("API").unwrap())
    }
    
    fn get_server(&self) -> Arc<dyn IServerContainer> {
        self.container.query::<Arc<dyn IServerContainer>>("ServerContainer").unwrap()
    }
    
    fn register_middleware(&self, middleware: Rc<dyn Middleware>) -> bool {
        self.middleware_vec.borrow_mut().push(middleware);
        true
    }
    
    fn get_app_name(&self) -> String {
        self.app_name.clone()
    }
    
    fn is_logged_in(&self) -> bool {
        user::is_logged_in()
    }
    
    fn is_admin_user(&self) -> bool {
        if let Some(uid) = self.get_user_id() {
            user::is_admin_user(&uid)
        } else {
            false
        }
    }
    
    fn log(&self, message: &str, level: &str) {
        let log_level = match level {
            "debug" => util::DEBUG,
            "info" => util::INFO,
            "warn" => util::WARN,
            "fatal" => util::FATAL,
            _ => util::ERROR,
        };
        
        util::write_log(&self.get_app_name(), message, log_level);
    }
}

// This would be defined elsewhere in the actual implementation
fn get_oc_server() -> impl IServerContainer {
    unimplemented!("This would be implemented by obtaining the global OC server instance")
}