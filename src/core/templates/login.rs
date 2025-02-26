use askama::Template;
use chrono::Local;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    redirect_url: Option<String>,
    invalidcookie: bool,
    apacheauthfailed: bool,
    username: String,
    user_autofocus: bool,
    invalidpassword: bool,
    remember_login_allowed: bool,
    alt_login: Vec<AltLogin>,
    timezone_offset: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AltLogin {
    name: String,
    href: String,
}

impl LoginTemplate {
    pub fn new(
        redirect_url: Option<String>,
        invalidcookie: bool,
        apacheauthfailed: bool,
        username: String,
        user_autofocus: bool,
        invalidpassword: bool,
        remember_login_allowed: bool,
        alt_login: Vec<AltLogin>,
    ) -> Self {
        // Sanitize redirect_url if present
        let sanitized_redirect_url = redirect_url
            .and_then(|url| {
                if let Ok(parsed) = Url::parse(&url) {
                    // Perform sanitization checks on parsed URL
                    if parsed.scheme() == "http" || parsed.scheme() == "https" {
                        Some(url)
                    } else {
                        None
                    }
                } else {
                    None
                }
            });

        // Get current timezone offset
        let now = Local::now();
        let timezone_offset = now.offset().local_minus_utc() / 60;

        Self {
            redirect_url: sanitized_redirect_url,
            invalidcookie,
            apacheauthfailed,
            username,
            user_autofocus,
            invalidpassword,
            remember_login_allowed,
            alt_login,
            timezone_offset,
        }
    }

    pub fn image_path(&self, path: &str) -> String {
        format!("/assets/images/{}", path)
    }

    pub fn lost_password_route(&self) -> String {
        "/core/lostpassword".to_string()
    }

    fn translate(&self, text: &str) -> String {
        // In a real implementation, this would use a translation system
        // For now, we just return the text as is
        text.to_string()
    }
}