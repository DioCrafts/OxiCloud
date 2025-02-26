// lib/private/appframework/middleware/security/security_middleware.rs

use crate::appframework::http::{self, RedirectResponse};
use crate::appframework::middleware::Middleware;
use crate::appframework::utility::method_annotation_reader::MethodAnnotationReader;
use crate::appframework::{IAppContainer, Http, Response, JSONResponse};
use crate::request::IRequest;
use std::sync::Arc;

/// Used to do all the authentication and checking stuff for a controller method
/// It reads out the annotations of a controller method and checks which if
/// security things should be checked and also handles errors in case a security
/// check fails
pub struct SecurityMiddleware {
    app: Arc<dyn IAppContainer>,
    request: Arc<dyn IRequest>,
}

pub struct SecurityException {
    message: String,
    code: u16,
}

impl SecurityException {
    pub fn new(message: &str, code: u16) -> Self {
        SecurityException {
            message: message.to_string(),
            code,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn code(&self) -> u16 {
        self.code
    }
}

impl std::fmt::Display for SecurityException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SecurityException: {}", self.message)
    }
}

impl std::fmt::Debug for SecurityException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SecurityException: {} (code: {})", self.message, self.code)
    }
}

impl std::error::Error for SecurityException {}

impl SecurityMiddleware {
    /// Creates a new SecurityMiddleware
    /// 
    /// # Arguments
    /// * `app` - The app container
    /// * `request` - The request object
    pub fn new(app: Arc<dyn IAppContainer>, request: Arc<dyn IRequest>) -> Self {
        SecurityMiddleware { app, request }
    }
}

impl Middleware for SecurityMiddleware {
    /// This runs all the security checks before a method call. The
    /// security checks are determined by inspecting the controller method
    /// annotations
    /// 
    /// # Arguments
    /// * `controller` - The controller instance or name
    /// * `method_name` - The name of the method
    /// 
    /// # Errors
    /// * `SecurityException` - When a security check fails
    fn before_controller<T>(&self, controller: &T, method_name: &str) -> Result<(), Box<dyn std::error::Error>>
    where
        T: ?Sized,
    {
        // get annotations from comments
        let annotation_reader = MethodAnnotationReader::new(controller, method_name);

        // this will set the current navigation entry of the app, use this only
        // for normal HTML requests and not for AJAX requests
        self.app
            .get_server()
            .get_navigation_manager()
            .set_active_entry(self.app.get_app_name());

        // security checks
        let is_public_page = annotation_reader.has_annotation("PublicPage");
        if !is_public_page {
            if !self.app.is_logged_in() {
                return Err(Box::new(SecurityException::new(
                    "Current user is not logged in",
                    Http::STATUS_UNAUTHORIZED,
                )));
            }

            if !annotation_reader.has_annotation("NoAdminRequired") {
                if !self.app.is_admin_user() {
                    return Err(Box::new(SecurityException::new(
                        "Logged in user must be an admin",
                        Http::STATUS_FORBIDDEN,
                    )));
                }
            }
        }

        if !annotation_reader.has_annotation("NoCSRFRequired") {
            if !self.request.passes_csrf_check() {
                return Err(Box::new(SecurityException::new(
                    "CSRF check failed",
                    Http::STATUS_PRECONDITION_FAILED,
                )));
            }
        }

        Ok(())
    }

    /// If an SecurityException is being caught, ajax requests return a JSON error
    /// response and non ajax requests redirect to the index
    /// 
    /// # Arguments
    /// * `controller` - The controller that is being called
    /// * `method_name` - The name of the method that will be called on the controller
    /// * `exception` - The thrown exception
    /// 
    /// # Returns
    /// * `Option<Response>` - A Response object or None in case the exception could not be handled
    fn after_exception<T, E>(
        &self,
        _controller: &T,
        _method_name: &str,
        exception: &E,
    ) -> Option<Box<dyn Response>>
    where
        T: ?Sized,
        E: std::error::Error + 'static,
    {
        if let Some(security_exception) = exception.downcast_ref::<SecurityException>() {
            let accept_header = self.request.get_header("Accept");
            
            if let Some(accept) = accept_header {
                if !accept.to_lowercase().contains("html") {
                    // Return JSON for non-HTML requests
                    let response = JSONResponse::new(
                        serde_json::json!({ "message": security_exception.message() }),
                        security_exception.code(),
                    );
                    self.app.log(security_exception.message(), "debug");
                    return Some(Box::new(response));
                }
            }
            
            // Redirect for HTML requests
            let url = self.app.get_server().get_url_generator().get_absolute_url("index.php");
            let response = RedirectResponse::new(url);
            self.app.log(security_exception.message(), "debug");
            return Some(Box::new(response));
        }

        None
    }
}