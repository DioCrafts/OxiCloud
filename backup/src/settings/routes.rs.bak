// Copyright (c) 2012 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use actix_web::{web, HttpResponse, Scope};
use std::sync::Arc;

pub fn configure_routes(router: &mut Router) -> Result<(), RouterError> {
    // Settings pages
    router.create("settings_help", "/settings/help")?
        .action_include("settings/help")?;
    router.create("settings_personal", "/settings/personal")?
        .action_include("settings/personal")?;
    router.create("settings_settings", "/settings")?
        .action_include("settings/settings")?;
    router.create("settings_users", "/settings/users")?
        .action_include("settings/users")?;
    router.create("settings_apps", "/settings/apps")?
        .action_include("settings/apps")?;
    router.create("settings_admin", "/settings/admin")?
        .action_include("settings/admin")?;
    
    // Settings ajax actions
    // users
    router.create("settings_ajax_userlist", "/settings/ajax/userlist")?
        .action_include("settings/ajax/userlist")?;
    router.create("settings_ajax_createuser", "/settings/ajax/createuser.php")?
        .action_include("settings/ajax/createuser")?;
    router.create("settings_ajax_removeuser", "/settings/ajax/removeuser.php")?
        .action_include("settings/ajax/removeuser")?;
    router.create("settings_ajax_setquota", "/settings/ajax/setquota.php")?
        .action_include("settings/ajax/setquota")?;
    router.create("settings_ajax_creategroup", "/settings/ajax/creategroup.php")?
        .action_include("settings/ajax/creategroup")?;
    router.create("settings_ajax_togglegroups", "/settings/ajax/togglegroups.php")?
        .action_include("settings/ajax/togglegroups")?;
    router.create("settings_ajax_togglesubadmins", "/settings/ajax/togglesubadmins.php")?
        .action_include("settings/ajax/togglesubadmins")?;
    router.create("settings_ajax_removegroup", "/settings/ajax/removegroup.php")?
        .action_include("settings/ajax/removegroup")?;
    router.create("settings_users_changepassword", "/settings/users/changepassword")?
        .post()?
        .action(Arc::new(ChangePasswordController::new()), "change_user_password")?;
    router.create("settings_ajax_changedisplayname", "/settings/ajax/changedisplayname.php")?
        .action_include("settings/ajax/changedisplayname")?;
    
    // personal
    router.create("settings_personal_changepassword", "/settings/personal/changepassword")?
        .post()?
        .action(Arc::new(ChangePasswordController::new()), "change_personal_password")?;
    router.create("settings_ajax_lostpassword", "/settings/ajax/lostpassword.php")?
        .action_include("settings/ajax/lostpassword")?;
    router.create("settings_ajax_setlanguage", "/settings/ajax/setlanguage.php")?
        .action_include("settings/ajax/setlanguage")?;
    router.create("settings_ajax_decryptall", "/settings/ajax/decryptall.php")?
        .action_include("settings/ajax/decryptall")?;
    
    // apps
    router.create("settings_ajax_apps_ocs", "/settings/ajax/apps/ocs.php")?
        .action_include("settings/ajax/apps/ocs")?;
    router.create("settings_ajax_enableapp", "/settings/ajax/enableapp.php")?
        .action_include("settings/ajax/enableapp")?;
    router.create("settings_ajax_disableapp", "/settings/ajax/disableapp.php")?
        .action_include("settings/ajax/disableapp")?;
    router.create("settings_ajax_updateapp", "/settings/ajax/updateapp.php")?
        .action_include("settings/ajax/updateapp")?;
    router.create("settings_ajax_navigationdetect", "/settings/ajax/navigationdetect.php")?
        .action_include("settings/ajax/navigationdetect")?;
    router.create("apps_custom", "/settings/js/apps-custom.js")?
        .action_include("settings/js/apps-custom")?;
    
    // admin
    router.create("settings_ajax_getlog", "/settings/ajax/getlog.php")?
        .action_include("settings/ajax/getlog")?;
    router.create("settings_ajax_setloglevel", "/settings/ajax/setloglevel.php")?
        .action_include("settings/ajax/setloglevel")?;
    router.create("settings_ajax_setsecurity", "/settings/ajax/setsecurity.php")?
        .action_include("settings/ajax/setsecurity")?;
    router.create("isadmin", "/settings/js/isadmin.js")?
        .action_include("settings/js/isadmin")?;
    
    Ok(())
}

// Estructuras necesarias para implementación

pub struct Router {
    routes: Vec<Route>,
}

pub struct Route {
    name: String,
    path: String,
    method: HttpMethod,
    handler: Option<RouteHandler>,
    include_path: Option<String>,
}

pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Any,
}

pub enum RouteHandler {
    ActionInclude(String),
    ControllerAction(Arc<dyn Controller>, String),
}

pub trait Controller: Send + Sync {
    fn handle(&self, method: &str, request: &web::HttpRequest) -> HttpResponse;
}

pub struct ChangePasswordController;

impl ChangePasswordController {
    pub fn new() -> Self {
        Self {}
    }
}

impl Controller for ChangePasswordController {
    fn handle(&self, method: &str, request: &web::HttpRequest) -> HttpResponse {
        match method {
            "change_user_password" => {
                // Implementación de cambio de contraseña para usuarios
                HttpResponse::Ok().finish()
            },
            "change_personal_password" => {
                // Implementación de cambio de contraseña personal
                HttpResponse::Ok().finish()
            },
            _ => HttpResponse::NotFound().finish(),
        }
    }
}

#[derive(Debug)]
pub enum RouterError {
    RouteNotFound(String),
    InvalidRouteConfiguration(String),
    ActionNotFound(String),
}

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn create(&mut self, name: &str, path: &str) -> Result<&mut Route, RouterError> {
        let route = Route {
            name: name.to_string(),
            path: path.to_string(),
            method: HttpMethod::Any,
            handler: None,
            include_path: None,
        };
        
        self.routes.push(route);
        
        // Retorna una referencia mutable al último elemento añadido
        Ok(self.routes.last_mut().unwrap())
    }
}

impl Route {
    pub fn action_include(&mut self, include_path: &str) -> Result<&mut Self, RouterError> {
        self.include_path = Some(include_path.to_string());
        self.handler = Some(RouteHandler::ActionInclude(include_path.to_string()));
        Ok(self)
    }

    pub fn action(&mut self, controller: Arc<dyn Controller>, method: &str) -> Result<&mut Self, RouterError> {
        self.handler = Some(RouteHandler::ControllerAction(controller, method.to_string()));
        Ok(self)
    }

    pub fn post(&mut self) -> Result<&mut Self, RouterError> {
        self.method = HttpMethod::Post;
        Ok(self)
    }
}