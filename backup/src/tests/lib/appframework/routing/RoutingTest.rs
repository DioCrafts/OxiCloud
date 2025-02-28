use std::collections::HashMap;
use mockall::{automock, mock, predicate::*};
use anyhow::{anyhow, Result};
use reqwest::Method;

// Mocks y estructuras necesarias para el test
type RouteCallback = Box<dyn Fn() + Send + Sync>;

#[automock]
trait OCRoute {
    fn method(&self, method: Method) -> &Self;
    fn action(&self, handler: RouteActionHandler) -> &Self;
}

#[automock]
trait OCRouter {
    fn create(&mut self, name: &str, url: &str) -> MockOCRoute;
}

struct DIContainer {
    app_name: String,
}

impl DIContainer {
    fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct RouteActionHandler {
    container: DIContainer,
    controller_name: String,
    action_name: String,
}

impl RouteActionHandler {
    fn new(container: DIContainer, controller_name: &str, action_name: &str) -> Self {
        Self {
            container,
            controller_name: controller_name.to_string(),
            action_name: action_name.to_string(),
        }
    }
}

struct RouteConfig {
    container: DIContainer,
    router: Box<dyn OCRouter>,
    routes_config: HashMap<String, Vec<HashMap<String, String>>>,
}

impl RouteConfig {
    fn new(
        container: DIContainer, 
        router: Box<dyn OCRouter>, 
        routes_config: HashMap<String, Vec<HashMap<String, String>>>,
    ) -> Self {
        Self {
            container,
            router,
            routes_config,
        }
    }

    fn register(&mut self) -> Result<()> {
        // Registrar rutas simples
        if let Some(routes) = self.routes_config.get("routes") {
            for route in routes {
                let name = route.get("name").ok_or_else(|| anyhow!("Route name not specified"))?;
                
                // Verificar que el nombre contiene un '#'
                if !name.contains('#') {
                    return Err(anyhow!("Route name must contain '#' separator"));
                }
                
                let parts: Vec<&str> = name.split('#').collect();
                let controller = parts[0];
                let action = parts[1];
                
                let url = route.get("url").ok_or_else(|| anyhow!("URL not specified"))?;
                
                let verb = match route.get("verb") {
                    Some(v) => v.to_uppercase(),
                    None => "GET".to_string()
                };
                
                // Transformar nombres de controlador y acción
                let controller_name = self.create_controller_name(controller);
                let action_name = self.create_action_name(action);
                
                self.create_route(
                    &format!("{}.{}", controller, action),
                    &url,
                    Method::from_bytes(verb.as_bytes()).unwrap(),
                    &controller_name,
                    &action_name,
                )?;
            }
        }

        // Registrar recursos
        if let Some(resources) = self.routes_config.get("resources") {
            for (resource_name, config) in resources.iter().flat_map(|m| m.iter()) {
                let url = config.get("url").ok_or_else(|| anyhow!("Resource URL not specified"))?;
                
                // Transformar nombres de controlador y parámetro
                let controller_name = self.create_controller_name(resource_name);
                let param_name = self.create_param_name(resource_name);
                
                self.register_resource(resource_name, url, &controller_name, &param_name)?;
            }
        }
        
        Ok(())
    }
    
    fn create_controller_name(&self, controller: &str) -> String {
        let parts: Vec<&str> = controller.split('_').collect();
        let mut result = String::new();
        
        for part in parts {
            let mut chars = part.chars();
            if let Some(first_char) = chars.next() {
                result.push_str(&first_char.to_uppercase().to_string());
                result.push_str(&chars.collect::<String>());
            }
        }
        
        result + "Controller"
    }
    
    fn create_action_name(&self, action: &str) -> String {
        let parts: Vec<&str> = action.split('_').collect();
        let mut result = String::new();
        
        for (i, part) in parts.iter().enumerate() {
            let mut chars = part.chars();
            if let Some(first_char) = chars.next() {
                if i == 0 {
                    result.push_str(&first_char.to_lowercase().to_string());
                } else {
                    result.push_str(&first_char.to_uppercase().to_string());
                }
                result.push_str(&chars.collect::<String>());
            }
        }
        
        result
    }
    
    fn create_param_name(&self, resource: &str) -> String {
        if resource.contains('_') {
            let parts: Vec<&str> = resource.split('_').collect();
            let mut result = String::new();
            
            for (i, part) in parts.iter().enumerate() {
                if i == 0 {
                    result.push_str(part);
                } else {
                    let mut chars = part.chars();
                    if let Some(first_char) = chars.next() {
                        result.push_str(&first_char.to_uppercase().to_string());
                        result.push_str(&chars.collect::<String>());
                    }
                }
            }
            
            result + "Id"
        } else {
            resource.to_string() + "Id"
        }
    }
    
    fn create_route(
        &mut self,
        name: &str,
        url: &str,
        method: Method,
        controller_name: &str,
        action_name: &str,
    ) -> Result<()> {
        let route_name = format!("{}.{}", self.container.app_name, name);
        let mut route = self.router.create(&route_name, url);
        
        let action_handler = RouteActionHandler::new(
            DIContainer::new(&self.container.app_name),
            controller_name,
            action_name,
        );
        
        route.method(method);
        route.action(action_handler);
        
        Ok(())
    }
    
    fn register_resource(
        &mut self,
        resource_name: &str,
        url: &str,
        controller_name: &str,
        param_name: &str,
    ) -> Result<()> {
        // INDEX
        self.create_route(
            &format!("{}.index", resource_name),
            url,
            Method::GET,
            controller_name,
            "index",
        )?;
        
        // SHOW
        self.create_route(
            &format!("{}.show", resource_name),
            &format!("{}/{{{}}}", url, param_name),
            Method::GET,
            controller_name,
            "show",
        )?;
        
        // CREATE
        self.create_route(
            &format!("{}.create", resource_name),
            url,
            Method::POST,
            controller_name,
            "create",
        )?;
        
        // UPDATE
        self.create_route(
            &format!("{}.update", resource_name),
            &format!("{}/{{{}}}", url, param_name),
            Method::PUT,
            controller_name,
            "update",
        )?;
        
        // DESTROY
        self.create_route(
            &format!("{}.destroy", resource_name),
            &format!("{}/{{{}}}", url, param_name),
            Method::DELETE,
            controller_name,
            "destroy",
        )?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::{hashmap, hashset};
    use mockall::predicate::*;
    use std::panic;
    
    #[test]
    fn test_simple_route() {
        let routes = hashmap! {
            "routes".to_string() => vec![
                hashmap! {
                    "name".to_string() => "folders#open".to_string(),
                    "url".to_string() => "/folders/{folderId}/open".to_string(),
                    "verb".to_string() => "GET".to_string()
                }
            ]
        };
        
        assert_simple_route(
            routes, 
            "folders.open", 
            Method::GET, 
            "/folders/{folderId}/open", 
            "FoldersController", 
            "open"
        );
    }
    
    #[test]
    fn test_simple_route_with_missing_verb() {
        let routes = hashmap! {
            "routes".to_string() => vec![
                hashmap! {
                    "name".to_string() => "folders#open".to_string(),
                    "url".to_string() => "/folders/{folderId}/open".to_string()
                }
            ]
        };
        
        assert_simple_route(
            routes, 
            "folders.open", 
            Method::GET, 
            "/folders/{folderId}/open", 
            "FoldersController", 
            "open"
        );
    }
    
    #[test]
    fn test_simple_route_with_lowercase_verb() {
        let routes = hashmap! {
            "routes".to_string() => vec![
                hashmap! {
                    "name".to_string() => "folders#open".to_string(),
                    "url".to_string() => "/folders/{folderId}/open".to_string(),
                    "verb".to_string() => "delete".to_string()
                }
            ]
        };
        
        assert_simple_route(
            routes, 
            "folders.open", 
            Method::DELETE, 
            "/folders/{folderId}/open", 
            "FoldersController", 
            "open"
        );
    }
    
    #[test]
    fn test_simple_route_with_broken_name() {
        let routes = hashmap! {
            "routes".to_string() => vec![
                hashmap! {
                    "name".to_string() => "folders_open".to_string(),
                    "url".to_string() => "/folders/{folderId}/open".to_string(),
                    "verb".to_string() => "delete".to_string()
                }
            ]
        };
        
        let mut router = Box::new(MockOCRouter::new());
        let container = DIContainer::new("app1");
        let mut config = RouteConfig::new(container, router, routes);
        
        let result = panic::catch_unwind(|| {
            config.register().expect("Should fail with error");
        });
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_simple_route_with_under_score_names() {
        let routes = hashmap! {
            "routes".to_string() => vec![
                hashmap! {
                    "name".to_string() => "admin_folders#open_current".to_string(),
                    "url".to_string() => "/folders/{folderId}/open".to_string(),
                    "verb".to_string() => "delete".to_string()
                }
            ]
        };
        
        assert_simple_route(
            routes, 
            "admin_folders.open_current", 
            Method::DELETE, 
            "/folders/{folderId}/open", 
            "AdminFoldersController", 
            "openCurrent"
        );
    }
    
    #[test]
    fn test_resource() {
        let routes = hashmap! {
            "resources".to_string() => vec![
                hashmap! {
                    "accounts".to_string() => hashmap! {
                        "url".to_string() => "/accounts".to_string()
                    }
                }
            ]
        };
        
        assert_resource(
            routes,
            "accounts",
            "/accounts",
            "AccountsController",
            "accountId"
        );
    }
    
    #[test]
    fn test_resource_with_under_score_name() {
        let routes = hashmap! {
            "resources".to_string() => vec![
                hashmap! {
                    "admin_accounts".to_string() => hashmap! {
                        "url".to_string() => "/admin/accounts".to_string()
                    }
                }
            ]
        };
        
        assert_resource(
            routes,
            "admin_accounts",
            "/admin/accounts",
            "AdminAccountsController",
            "adminAccountId"
        );
    }
    
    fn assert_simple_route(
        routes: HashMap<String, Vec<HashMap<String, String>>>,
        name: &str,
        verb: Method,
        url: &str,
        controller_name: &str,
        action_name: &str
    ) {
        // Crear mock route
        let mut mock_route = mock_route(verb, controller_name, action_name);
        
        // Crear mock router
        let mut router = MockOCRouter::new();
        router
            .expect_create()
            .with(eq(format!("app1.{}", name)), eq(url))
            .times(1)
            .return_once(move |_, _| mock_route);
        
        // Configuración y registro de rutas
        let container = DIContainer::new("app1");
        let mut config = RouteConfig::new(container, Box::new(router), routes);
        
        config.register().expect("Failed to register routes");
    }
    
    fn assert_resource(
        yaml: HashMap<String, Vec<HashMap<String, String>>>,
        resource_name: &str,
        url: &str,
        controller_name: &str,
        param_name: &str
    ) {
        // Crear mock routes
        let mut index_route = mock_route(Method::GET, controller_name, "index");
        let mut show_route = mock_route(Method::GET, controller_name, "show");
        let mut create_route = mock_route(Method::POST, controller_name, "create");
        let mut update_route = mock_route(Method::PUT, controller_name, "update");
        let mut destroy_route = mock_route(Method::DELETE, controller_name, "destroy");
        
        let url_with_param = format!("{}/{{{}}}", url, param_name);
        
        // Crear mock router
        let mut router = MockOCRouter::new();
        router
            .expect_create()
            .with(eq(format!("app1.{}.index", resource_name)), eq(url))
            .times(1)
            .return_once(move |_, _| index_route);
            
        router
            .expect_create()
            .with(eq(format!("app1.{}.show", resource_name)), eq(&url_with_param))
            .times(1)
            .return_once(move |_, _| show_route);
            
        router
            .expect_create()
            .with(eq(format!("app1.{}.create", resource_name)), eq(url))
            .times(1)
            .return_once(move |_, _| create_route);
            
        router
            .expect_create()
            .with(eq(format!("app1.{}.update", resource_name)), eq(&url_with_param))
            .times(1)
            .return_once(move |_, _| update_route);
            
        router
            .expect_create()
            .with(eq(format!("app1.{}.destroy", resource_name)), eq(&url_with_param))
            .times(1)
            .return_once(move |_, _| destroy_route);
        
        // Configuración y registro de rutas
        let container = DIContainer::new("app1");
        let mut config = RouteConfig::new(container, Box::new(router), yaml);
        
        config.register().expect("Failed to register routes");
    }
    
    fn mock_route(verb: Method, controller_name: &str, action_name: &str) -> MockOCRoute {
        let container = DIContainer::new("app1");
        let handler = RouteActionHandler::new(container, controller_name, action_name);
        
        let mut route = MockOCRoute::new();
        route
            .expect_method()
            .with(eq(verb))
            .times(1)
            .return_const(());
            
        route
            .expect_action()
            .with(eq(handler))
            .times(1)
            .return_const(());
            
        route
    }
}

/*
#
# sample routes.yaml for ownCloud
#
# the section simple describes one route

routes:
        - name: folders#open
          url: /folders/{folderId}/open
          verb: GET
          # controller: name.split()[0]
          # action: name.split()[1]

# for a resource following actions will be generated:
# - index
# - create
# - show
# - update
# - destroy
# - new
resources:
    accounts:
        url: /accounts

    folders:
        url: /accounts/{accountId}/folders
        # actions can be used to define additional actions on the resource
        actions:
            - name: validate
              verb: GET
              on-collection: false

 * */