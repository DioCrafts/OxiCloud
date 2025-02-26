//! Redirect Response implementation for web requests
//!
//! This module implements a response type that redirects to a different URL.

use crate::http::{Response, StatusCode};

/// Redirects to a different URL
///
/// This response type redirects the client to a different URL by setting 
/// the appropriate status code and Location header.
pub struct RedirectResponse {
    /// The URL to redirect to
    redirect_url: String,
    /// The underlying HTTP response
    response: Response,
}

impl RedirectResponse {
    /// Creates a response that redirects to a url
    ///
    /// # Arguments
    ///
    /// * `redirect_url` - The url to redirect to
    pub fn new(redirect_url: impl Into<String>) -> Self {
        let redirect_url = redirect_url.into();
        let mut response = Response::new();
        response.set_status(StatusCode::TEMPORARY_REDIRECT);
        response.add_header("Location", &redirect_url);
        
        Self {
            redirect_url,
            response,
        }
    }

    /// Returns the url to redirect to
    pub fn get_redirect_url(&self) -> &str {
        &self.redirect_url
    }
}

impl From<RedirectResponse> for Response {
    fn from(redirect: RedirectResponse) -> Self {
        redirect.response
    }
}

impl AsRef<Response> for RedirectResponse {
    fn as_ref(&self) -> &Response {
        &self.response
    }
}

impl AsMut<Response> for RedirectResponse {
    fn as_mut(&mut self) -> &mut Response {
        &mut self.response
    }
}