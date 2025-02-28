// Copyright (c) 2012 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex, Once};
use std::sync::LazyLock;

use actix_web::HttpRequest;
use async_trait::async_trait;
use regex::{Regex, RegexSet};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

use crate::helper::OcHelper;
use crate::request::OcRequest;
use crate::app::OcApp;
use crate::cache::OcCache;
use crate::response::OcResponse;
use crate::json::OcpJson;

#[derive(Error, Debug)]
pub enum RouterError {
    #[error("No action available")]
    NoActionAvailable,
    
    #[error("Not a callable action")]
    NotCallableAction,
    
    #[error("File error: {0}")]
    FileError(#[from] std::io::Error),
    
    #[error("Route not found: {0}")]
    RouteNotFound(String),
    
    #[error("Other error: {0}")]
    Other(String),
}

type Result<T> = std::result::Result<T, RouterError>;
type RouteAction = Arc<dyn Fn(HashMap<String, String>) -> Result<()> + Send + Sync>;

#[derive(Clone)]
pub struct OcRoute {
    pattern: String,
    defaults: HashMap<String, String>,
    requirements: HashMap<String, String>,
    tokens: Vec<RouteToken>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum RouteToken {
    Text(String),
    Variable { name: String, regex: String, pattern: Option<String> },
}

pub struct RequestContext {
    base_url: String,
    method: String,
    host: String,
    schema: String,
}

pub struct OcRouter {
    collections: HashMap<String, RouteCollection>,
    collection: String,
    root: Option<RouteCollection>,
    context: RequestContext,
    generator: Option<UrlGenerator>,
    routing_files: Option<HashMap<String, String>>,
    cache_key: Option<String>,
}

#[derive(Clone)]
pub struct RouteCollection {
    routes: HashMap<String, OcRoute>,
    children: HashMap<String, (RouteCollection, String)>,
}

pub struct UrlGenerator {
    routes: Arc<RouteCollection>,
    context: Arc<RequestContext>,
}

impl OcRoute {
    pub fn new(pattern: &str, defaults: HashMap<String, String>, requirements: HashMap<String, String>) -> Self {
        let tokens = Self::compile_pattern(pattern);
        Self {
            pattern: pattern.to_string(),
            defaults,
            requirements,
            tokens,
        }
    }
    
    fn compile_pattern(pattern: &str) -> Vec<RouteToken> {
        // Esta es una implementación simplificada
        // En un caso real, habría que analizar el patrón y generar tokens adecuados
        let mut tokens = Vec::new();
        
        // Añadir un token de texto simple para el patrón
        tokens.push(RouteToken::Text(pattern.to_string()));
        
        tokens
    }
    
    pub fn compile(&self) -> CompiledRoute {
        CompiledRoute {
            tokens: self.tokens.clone(),
        }
    }
    
    pub fn get_defaults(&self) -> HashMap<String, String> {
        self.defaults.clone()
    }
}

pub struct CompiledRoute {
    tokens: Vec<RouteToken>,
}

impl CompiledRoute {
    pub fn get_tokens(&self) -> &Vec<RouteToken> {
        &self.tokens
    }
}

impl RouteCollection {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            children: HashMap::new(),
        }
    }
    
    pub fn add(&mut self, name: &str, route: OcRoute) {
        self.routes.insert(name.to_string(), route);
    }
    
    pub fn add_collection(&mut self, collection: RouteCollection, prefix: &str) {
        self.children.insert(prefix.to_string(), (collection, prefix.to_string()));
    }
    
    pub fn all(&self) -> &HashMap<String, OcRoute> {
        &self.routes
    }
}

impl RequestContext {
    pub fn new(base_url: &str, method: &str, host: &str, schema: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            method: method.to_string(),
            host: host.to_string(),
            schema: schema.to_string(),
        }
    }
}

impl UrlGenerator {
    pub fn new(routes: RouteCollection, context: RequestContext) -> Self {
        Self {
            routes: Arc::new(routes),
            context: Arc::new(context),
        }
    }
    
    pub fn generate(&self, name: &str, parameters: HashMap<String, String>, absolute: bool) -> Result<String> {
        // Implementación simplificada
        // En un caso real, generaría la URL basada en la ruta y los parámetros
        if let Some(route) = self.routes.routes.get(name) {
            // Aquí iría la lógica de generación real
            let mut url = self.context.base_url.clone();
            if !url.ends_with('/') {
                url.push('/');
            }
            url.push_str(name);
            
            Ok(url)
        } else {
            Err(RouterError::RouteNotFound(name.to_string()))
        }
    }
}

impl OcRouter {
    pub fn new() -> Self {
        let base_url = OcHelper::link_to("", "index.php");
        
        let method = if !crate::OC::is_cli() {
            // Esto debería venir del entorno en un escenario real
            std::env::var("REQUEST_METHOD").unwrap_or_else(|_| "GET".to_string())
        } else {
            "GET".to_string()
        };
        
        let host = OcRequest::server_host();
        let schema = OcRequest::server_protocol();
        
        let context = RequestContext::new(&base_url, &method, &host, &schema);
        
        let mut router = Self {
            collections: HashMap::new(),
            collection: "".to_string(),
            root: None,
            context,
            generator: None,
            routing_files: None,
            cache_key: None,
        };
        
        // Inicializar la colección root
        let root = router.get_collection("root");
        router.root = Some(root);
        
        router
    }
    
    pub fn get_routing_files(&mut self) -> HashMap<String, String> {
        if self.routing_files.is_none() {
            let mut files = HashMap::new();
            for app in OcApp::get_enabled_apps() {
                let file_path = format!("{}/appinfo/routes.php", OcApp::get_app_path(&app));
                if Path::new(&file_path).exists() {
                    files.insert(app.clone(), file_path);
                }
            }
            self.routing_files = Some(files);
        }
        self.routing_files.clone().unwrap()
    }
    
    pub fn get_cache_key(&mut self) -> String {
        if self.cache_key.is_none() {
            let mut files = self.get_routing_files().values().cloned().collect::<Vec<_>>();
            files.push("settings/routes.php".to_string());
            files.push("core/routes.php".to_string());
            files.push("ocs/routes.php".to_string());
            self.cache_key = Some(OcCache::generate_cache_key_from_files(&files));
        }
        self.cache_key.clone().unwrap()
    }
    
    pub fn load_routes(&mut self) -> Result<()> {
        for (app, file) in self.get_routing_files() {
            self.use_collection(&app);
            // En Rust, no podemos simplemente require_once un archivo PHP
            // Esto es una simplificación, en la realidad necesitaríamos implementar
            // una forma de registrar rutas de forma programática
            
            let collection = self.get_collection(&app);
            if let Some(root) = &mut self.root {
                root.add_collection(collection, &format!("/apps/{}", app));
            }
        }
        
        self.use_collection("root");
        // Aquí deberíamos cargar las rutas de settings, core y ocs
        // Pero en Rust necesitaríamos un enfoque diferente
        
        let collection = self.get_collection("ocs");
        if let Some(root) = &mut self.root {
            root.add_collection(collection, "/ocs");
        }
        
        Ok(())
    }
    
    fn get_collection(&mut self, name: &str) -> RouteCollection {
        if !self.collections.contains_key(name) {
            self.collections.insert(name.to_string(), RouteCollection::new());
        }
        self.collections.get(name).unwrap().clone()
    }
    
    pub fn use_collection(&mut self, name: &str) {
        self.collection = name.to_string();
        if !self.collections.contains_key(name) {
            self.collections.insert(name.to_string(), RouteCollection::new());
        }
    }
    
    pub fn create(&mut self, name: &str, pattern: &str, defaults: HashMap<String, String>, requirements: HashMap<String, String>) -> OcRoute {
        let route = OcRoute::new(pattern, defaults, requirements);
        if let Some(collection) = self.collections.get_mut(&self.collection) {
            collection.add(name, route.clone());
        }
        route
    }
    
    pub async fn match_url(&self, url: &str) -> Result<()> {
        if let Some(root) = &self.root {
            // Implementación simplificada de UrlMatcher
            // En un caso real, necesitaríamos hacer coincidir la URL con las rutas
            
            // Simulamos encontrar parámetros
            let mut parameters = HashMap::new();
            parameters.insert("url".to_string(), url.to_string());
            
            // Verificar si hay una acción o archivo
            if let Some(action) = parameters.get("action") {
                // En Rust no podemos simplemente llamar a una acción desde una cadena
                // Necesitaríamos un registro de acciones o usar un enum/trait
                return Err(RouterError::NotCallableAction);
            } else if let Some(file) = parameters.get("file") {
                // En Rust no podemos simplemente incluir un archivo
                // Necesitaríamos un enfoque diferente
                return Err(RouterError::FileError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Cannot include file in Rust"
                )));
            } else {
                return Err(RouterError::NoActionAvailable);
            }
        }
        
        Err(RouterError::NoActionAvailable)
    }
    
    pub fn get_generator(&mut self) -> UrlGenerator {
        if self.generator.is_none() {
            if let Some(root) = &self.root {
                self.generator = Some(UrlGenerator::new(root.clone(), self.context.clone()));
            } else {
                // Inicializar root si no existe
                let root = self.get_collection("root");
                self.root = Some(root.clone());
                self.generator = Some(UrlGenerator::new(root, self.context.clone()));
            }
        }
        self.generator.as_ref().unwrap().clone()
    }
    
    pub fn generate(&mut self, name: &str, parameters: HashMap<String, String>, absolute: bool) -> Result<String> {
        self.get_generator().generate(name, parameters, absolute)
    }
    
    pub async fn js_routes() -> serde_json::Value {
        let mut router = crate::OC::get_router();
        
        let etag = router.get_cache_key();
        OcResponse::enable_caching();
        OcResponse::set_etag_header(&etag);
        
        if let Some(root) = &router.root {
            let mut routes = HashMap::new();
            for (name, route) in root.all() {
                let compiled_route = route.compile();
                let mut defaults = route.get_defaults();
                defaults.remove("action");
                
                routes.insert(name.clone(), json!({
                    "tokens": compiled_route.get_tokens(),
                    "defaults": defaults,
                }));
            }
            
            OcpJson::success(json!({ "data": routes }))
        } else {
            OcpJson::success(json!({ "data": {} }))
        }
    }
}