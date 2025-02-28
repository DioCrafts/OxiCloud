use std::collections::HashMap;

/**
 * Dropbox OAuth
 * 
 * @package Dropbox 
 * @copyright Copyright (C) 2010 Rooftop Solutions. All rights reserved.
 * @author Evert Pot (http://www.rooftopsolutions.nl/) 
 * @license http://code.google.com/p/dropbox-php/wiki/License MIT
 */

/// This trait is an abstract OAuth class.
///
/// It must be implemented by types who wish to provide OAuth functionality
/// using different libraries.
pub trait DropboxOAuth {
    /// Constructor equivalent
    fn new(consumer_key: String, consumer_secret: String) -> Self where Self: Sized;

    /// Fetches a secured oauth url and returns the response body. 
    /// 
    /// # Arguments
    /// * `uri` - The URI to fetch
    /// * `arguments` - Arguments to pass to the request
    /// * `method` - HTTP method to use
    /// * `http_headers` - Additional HTTP headers
    fn fetch(
        &self, 
        uri: &str, 
        arguments: HashMap<String, String>, 
        method: &str, 
        http_headers: HashMap<String, String>
    ) -> Result<String, Box<dyn std::error::Error>>;

    /// Requests the OAuth request token.
    fn get_request_token(&self) -> Result<TokenPair, Box<dyn std::error::Error>>;

    /// Requests the OAuth access tokens.
    fn get_access_token(&self) -> Result<TokenPair, Box<dyn std::error::Error>>;
}

/// Base implementation for OAuth clients
pub struct DropboxOAuthBase {
    /// After a user has authorized access, dropbox can redirect the user back
    /// to this url.
    pub authorize_callback_url: Option<String>,

    /// An OAuth request token.
    oauth_token: Option<String>,

    /// OAuth token secret
    oauth_token_secret: Option<String>,
}

/// Represents an OAuth token pair
#[derive(Debug, Clone)]
pub struct TokenPair {
    pub token: String,
    pub token_secret: String,
}

impl DropboxOAuthBase {
    /// Uri used to fetch request tokens
    pub const URI_REQUEST_TOKEN: &'static str = "https://api.dropbox.com/1/oauth/request_token";

    /// Uri used to redirect the user to for authorization.
    pub const URI_AUTHORIZE: &'static str = "https://www.dropbox.com/1/oauth/authorize";

    /// Uri used for access token
    pub const URI_ACCESS_TOKEN: &'static str = "https://api.dropbox.com/1/oauth/access_token";

    /// Creates a new DropboxOAuthBase instance
    pub fn new() -> Self {
        DropboxOAuthBase {
            authorize_callback_url: None,
            oauth_token: None,
            oauth_token_secret: None,
        }
    }

    /// Sets the request token and secret.
    ///
    /// The tokens can also be passed as a TokenPair into the first argument.
    /// 
    /// # Arguments
    /// * `token` - Either a TokenPair or a token string
    /// * `token_secret` - The token secret (used only if token is a string)
    pub fn set_token<T>(&mut self, token: T, token_secret: Option<String>) 
    where 
        T: Into<TokenSetter>,
    {
        match token.into() {
            TokenSetter::Pair(pair) => {
                self.oauth_token = Some(pair.token);
                self.oauth_token_secret = Some(pair.token_secret);
            },
            TokenSetter::Token(token_str) => {
                self.oauth_token = Some(token_str);
                if let Some(secret) = token_secret {
                    self.oauth_token_secret = Some(secret);
                }
            }
        }
    }

    /// Returns the oauth request tokens as a TokenPair.
    ///
    /// # Returns
    /// Option containing the TokenPair or None if tokens are not set
    pub fn get_token(&self) -> Option<TokenPair> {
        match (&self.oauth_token, &self.oauth_token_secret) {
            (Some(token), Some(token_secret)) => Some(TokenPair {
                token: token.clone(),
                token_secret: token_secret.clone(),
            }),
            _ => None,
        }
    }

    /// Returns the authorization url
    /// 
    /// # Arguments
    /// * `callback` - Specify a callback url to automatically redirect the user back
    ///
    /// # Returns
    /// Option containing the URL or None if tokens are not set
    pub fn get_authorize_url(&self, callback: Option<&str>) -> Option<String> {
        self.get_token().map(|token| {
            let mut uri = format!("{}?oauth_token={}", Self::URI_AUTHORIZE, token.token);
            if let Some(cb) = callback {
                uri.push_str(&format!("&oauth_callback={}", cb));
            }
            uri
        })
    }
}

/// Helper enum for token setting
pub enum TokenSetter {
    Pair(TokenPair),
    Token(String),
}

impl From<TokenPair> for TokenSetter {
    fn from(pair: TokenPair) -> Self {
        TokenSetter::Pair(pair)
    }
}

impl From<String> for TokenSetter {
    fn from(token: String) -> Self {
        TokenSetter::Token(token)
    }
}

impl From<&str> for TokenSetter {
    fn from(token: &str) -> Self {
        TokenSetter::Token(token.to_string())
    }
}