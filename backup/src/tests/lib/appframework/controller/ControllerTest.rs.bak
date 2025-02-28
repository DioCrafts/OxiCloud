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

use std::collections::HashMap;
use mockall::{automock, predicate::*};
use async_trait::async_trait;

mod app_framework {
    use super::*;
    
    pub mod http {
        use super::*;
        
        pub struct Request {
            get: HashMap<String, String>,
            post: HashMap<String, String>,
            url_params: HashMap<String, String>,
            files: HashMap<String, String>,
            env: HashMap<String, String>,
            session: HashMap<String, String>,
            method: String,
        }
        
        impl Request {
            pub fn new(
                get: HashMap<String, String>,
                post: HashMap<String, String>,
                url_params: HashMap<String, String>,
                files: HashMap<String, String>,
                env: HashMap<String, String>,
                session: HashMap<String, String>,
                method: String,
            ) -> Self {
                Self {
                    get,
                    post,
                    url_params,
                    files,
                    env,
                    session,
                    method,
                }
            }
            
            pub fn get_url_parameter(&self, key: &str) -> Option<&String> {
                self.url_params.get(key)
            }
            
            pub fn get_post_parameter(&self, key: &str) -> Option<&String> {
                self.post.get(key)
            }
            
            pub fn get_file(&self, key: &str) -> Option<&String> {
                self.files.get(key)
            }
            
            pub fn get_method(&self) -> &str {
                &self.method
            }
            
            pub fn get_env(&self, key: &str) -> Option<&String> {
                self.env.get(key)
            }
        }
        
        pub struct TemplateResponse {
            template_name: String,
            params: HashMap<String, String>,
            render_as: String,
            headers: Vec<String>,
        }
        
        impl TemplateResponse {
            pub fn new(
                template_name: String,
                params: HashMap<String, String>,
                render_as: String,
                headers: Vec<String>,
            ) -> Self {
                Self {
                    template_name,
                    params,
                    render_as,
                    headers,
                }
            }
            
            pub fn get_params(&self) -> &HashMap<String, String> {
                &self.params
            }
            
            pub fn get_headers(&self) -> &Vec<String> {
                &self.headers
            }
        }
    }
    
    pub mod dependency_injection {
        use super::*;
        
        #[automock]
        #[async_trait]
        pub trait DIContainer {
            async fn get_app_name(&self) -> String;
        }
    }
    
    pub mod controller {
        use super::*;
        use crate::app_framework::http::{Request, TemplateResponse};
        use crate::app_framework::dependency_injection::DIContainer;
        
        pub struct Controller {
            container: Box<dyn DIContainer>,
            request: http::Request,
        }
        
        impl Controller {
            pub fn new(container: Box<dyn DIContainer>, request: http::Request) -> Self {
                Self {
                    container,
                    request,
                }
            }
            
            pub fn params<T: AsRef<str>>(&self, key: T, default: T) -> String {
                let key_str = key.as_ref();
                
                // First check URL parameters
                if let Some(value) = self.request.get_url_parameter(key_str) {
                    return value.clone();
                }
                
                // Then check POST parameters
                if let Some(value) = self.request.get_post_parameter(key_str) {
                    return value.clone();
                }
                
                // Then check files
                if let Some(value) = self.request.get_file(key_str) {
                    return value.clone();
                }
                
                // Return default if not found
                default.as_ref().to_string()
            }
            
            pub fn get_uploaded_file(&self, key: &str) -> Option<String> {
                self.request.get_file(key).cloned()
            }
            
            pub fn get_params(&self) -> HashMap<String, String> {
                let mut result = HashMap::new();
                
                // Add URL parameters first
                for (key, value) in self.request.url_params.iter() {
                    result.insert(key.clone(), value.clone());
                }
                
                // Override with POST parameters
                for (key, value) in self.request.post.iter() {
                    result.insert(key.clone(), value.clone());
                }
                
                result
            }
            
            pub fn render<T: AsRef<str>>(
                &self,
                template: T,
                params: HashMap<String, String>,
                render_as: T,
                headers: Vec<String>,
            ) -> TemplateResponse {
                TemplateResponse::new(
                    template.as_ref().to_string(),
                    params,
                    render_as.as_ref().to_string(),
                    headers,
                )
            }
            
            pub fn method(&self) -> &str {
                self.request.get_method()
            }
            
            pub fn env(&self, key: &str) -> Option<String> {
                self.request.get_env(key).cloned()
            }
        }
    }
}

use app_framework::http::Request;
use app_framework::controller::Controller;
use app_framework::dependency_injection::DIContainer;

struct ChildController {
    controller: Controller,
}

impl ChildController {
    fn new(container: Box<dyn DIContainer>, request: Request) -> Self {
        Self {
            controller: Controller::new(container, request)
        }
    }
    
    // Delegate methods to the inner controller
    fn params<T: AsRef<str>>(&self, key: T, default: T) -> String {
        self.controller.params(key, default)
    }
    
    fn get_uploaded_file(&self, key: &str) -> Option<String> {
        self.controller.get_uploaded_file(key)
    }
    
    fn get_params(&self) -> HashMap<String, String> {
        self.controller.get_params()
    }
    
    fn render<T: AsRef<str>>(
        &self,
        template: T,
        params: HashMap<String, String>,
        render_as: T,
        headers: Vec<String>,
    ) -> TemplateResponse {
        self.controller.render(template, params, render_as, headers)
    }
    
    fn method(&self) -> &str {
        self.controller.method()
    }
    
    fn env(&self, key: &str) -> Option<String> {
        self.controller.env(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use std::collections::HashMap;
    use app_framework::dependency_injection::MockDIContainer;
    
    struct ControllerTest {
        controller: ChildController,
        app: MockDIContainer,
    }
    
    impl ControllerTest {
        async fn setup() -> Self {
            let mut get = HashMap::new();
            get.insert("name".to_string(), "John Q. Public".to_string());
            get.insert("nickname".to_string(), "Joey".to_string());
            
            let mut post = HashMap::new();
            post.insert("name".to_string(), "Jane Doe".to_string());
            post.insert("nickname".to_string(), "Janey".to_string());
            
            let mut url_params = HashMap::new();
            url_params.insert("name".to_string(), "Johnny Weissmüller".to_string());
            
            let mut files = HashMap::new();
            files.insert("file".to_string(), "filevalue".to_string());
            
            let mut env = HashMap::new();
            env.insert("PATH".to_string(), "daheim".to_string());
            
            let mut session = HashMap::new();
            session.insert("sezession".to_string(), "kein".to_string());
            
            let request = Request::new(
                get,
                post,
                url_params,
                files,
                env,
                session,
                "hi".to_string(),
            );
            
            let mut app = MockDIContainer::new();
            app.expect_get_app_name()
                .returning(|| "apptemplate_advanced".to_string());
            
            let controller = ChildController::new(Box::new(app.clone()), request);
            
            Self {
                controller,
                app,
            }
        }
        
        async fn test_params_get(&self) {
            assert_eq!("Johnny Weissmüller", self.controller.params("name", "Tarzan"));
        }
        
        async fn test_params_get_default(&self) {
            assert_eq!("Tarzan", self.controller.params("Ape Man", "Tarzan"));
        }
        
        async fn test_params_file(&self) {
            assert_eq!("filevalue", self.controller.params("file", "filevalue"));
        }
        
        async fn test_get_uploaded_file(&self) {
            assert_eq!(Some("filevalue".to_string()), self.controller.get_uploaded_file("file"));
        }
        
        async fn test_get_uploaded_file_default(&self) {
            assert_eq!("default", self.controller.params("files", "default"));
        }
        
        async fn test_get_params(&self) {
            let mut expected = HashMap::new();
            expected.insert("name".to_string(), "Johnny Weissmüller".to_string());
            expected.insert("nickname".to_string(), "Janey".to_string());
            
            assert_eq!(expected, self.controller.get_params());
        }
        
        async fn test_render(&self) {
            let response = self.controller.render("", HashMap::new(), "", vec![]);
            assert!(response.is_a::<TemplateResponse>());
        }
        
        async fn test_set_params(&self) {
            let mut params = HashMap::new();
            params.insert("john".to_string(), "foo".to_string());
            
            let response = self.controller.render("home", params.clone(), "", vec![]);
            assert_eq!(&params, response.get_params());
        }
        
        async fn test_render_headers(&self) {
            let headers = vec!["one".to_string(), "two".to_string()];
            let response = self.controller.render("", HashMap::new(), "", headers.clone());
            
            let response_headers = response.get_headers();
            assert!(response_headers.contains(&headers[0]));
            assert!(response_headers.contains(&headers[1]));
        }
        
        async fn test_get_request_method(&self) {
            assert_eq!("hi", self.controller.method());
        }
        
        async fn test_get_env_variable(&self) {
            assert_eq!(Some("daheim".to_string()), self.controller.env("PATH"));
        }
    }
    
    #[tokio::test]
    async fn run_all_tests() {
        let test = ControllerTest::setup().await;
        
        test.test_params_get().await;
        test.test_params_get_default().await;
        test.test_params_file().await;
        test.test_get_uploaded_file().await;
        test.test_get_uploaded_file_default().await;
        test.test_get_params().await;
        test.test_render().await;
        test.test_set_params().await;
        test.test_render_headers().await;
        test.test_get_request_method().await;
        test.test_get_env_variable().await;
    }
}