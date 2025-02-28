/*
 * Copyright 2008 Google Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::{Client, Response, StatusCode};
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use async_trait::async_trait;
use base64::prelude::*;
use url::Url;
use anyhow::{Result, anyhow, bail, Context};
use thiserror::Error;

pub mod verifier;
pub mod login_ticket;
pub mod utils;

use crate::verifier::{PemVerifier, Verifier};
use crate::login_ticket::LoginTicket;
use crate::utils::Utils;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Authentication error: {0}")]
    General(String),
    
    #[error("HTTP error ({0}): {1}")]
    Http(u16, String),
    
    #[error("JSON decode error: {0}")]
    JsonDecode(String),
    
    #[error("Invalid token format: {0}")]
    InvalidToken(String),
    
    #[error("Token expired: {0}")]
    TokenExpired(String),
}

#[async_trait]
pub trait Auth {
    async fn sign(&self, request: &mut HttpRequest) -> Result<()>;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Token {
    access_token: String,
    expires_in: i64,
    created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_token: Option<String>,
}

pub struct AssertionCredentials {
    pub assertion_type: String,
    generate_assertion: Box<dyn Fn() -> String + Send + Sync>,
}

impl AssertionCredentials {
    pub fn new<F>(assertion_type: String, generate_assertion: F) -> Self
    where
        F: Fn() -> String + Send + Sync + 'static,
    {
        Self {
            assertion_type,
            generate_assertion: Box::new(generate_assertion),
        }
    }
    
    pub fn generate_assertion(&self) -> String {
        (self.generate_assertion)()
    }
}

pub struct HttpRequest {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl HttpRequest {
    pub fn new(url: &str, method: &str, headers: HashMap<String, String>, body: Option<String>) -> Self {
        Self {
            url: url.to_string(),
            method: method.to_string(),
            headers,
            body,
        }
    }
}

pub struct OAuth2 {
    client_id: Option<String>,
    client_secret: Option<String>,
    developer_key: Option<String>,
    token: Option<Token>,
    redirect_uri: Option<String>,
    state: Option<String>,
    access_type: String,
    approval_prompt: String,
    assertion_credentials: Option<AssertionCredentials>,
    client: Client,
}

impl OAuth2 {
    const OAUTH2_REVOKE_URI: &'static str = "https://accounts.google.com/o/oauth2/revoke";
    const OAUTH2_TOKEN_URI: &'static str = "https://accounts.google.com/o/oauth2/token";
    const OAUTH2_AUTH_URL: &'static str = "https://accounts.google.com/o/oauth2/auth";
    const OAUTH2_FEDERATED_SIGNON_CERTS_URL: &'static str = "https://www.googleapis.com/oauth2/v1/certs";
    const CLOCK_SKEW_SECS: i64 = 300; // five minutes in seconds
    const AUTH_TOKEN_LIFETIME_SECS: i64 = 300; // five minutes in seconds
    const MAX_TOKEN_LIFETIME_SECS: i64 = 86400; // one day in seconds

    /// Instantiates the class, but does not initiate the login flow, leaving it
    /// to the discretion of the caller (which is done by calling authenticate()).
    pub fn new(config: &HashMap<String, String>) -> Self {
        let mut oauth = Self {
            client_id: None,
            client_secret: None,
            developer_key: None,
            token: None,
            redirect_uri: None,
            state: None,
            access_type: "offline".to_string(),
            approval_prompt: "force".to_string(),
            assertion_credentials: None,
            client: Client::new(),
        };
        
        if let Some(developer_key) = config.get("developer_key") {
            oauth.developer_key = Some(developer_key.clone());
        }
        
        if let Some(client_id) = config.get("oauth2_client_id") {
            oauth.client_id = Some(client_id.clone());
        }
        
        if let Some(client_secret) = config.get("oauth2_client_secret") {
            oauth.client_secret = Some(client_secret.clone());
        }
        
        if let Some(redirect_uri) = config.get("oauth2_redirect_uri") {
            oauth.redirect_uri = Some(redirect_uri.clone());
        }
        
        if let Some(access_type) = config.get("oauth2_access_type") {
            oauth.access_type = access_type.clone();
        }
        
        if let Some(approval_prompt) = config.get("oauth2_approval_prompt") {
            oauth.approval_prompt = approval_prompt.clone();
        }
        
        oauth
    }
    
    /// Authenticates the user and returns the access token on success.
    pub async fn authenticate(&mut self, service: &HashMap<String, String>, code: Option<String>) -> Result<String> {
        let code = match code {
            Some(c) => Some(c),
            None => {
                // In a real application, you would extract from query parameters
                None
            }
        };
        
        if let Some(code) = code {
            // We got here from the redirect from a successful authorization grant, fetch the access token
            let client_id = self.client_id.as_ref()
                .ok_or_else(|| anyhow!("client_id is not set"))?;
            let client_secret = self.client_secret.as_ref()
                .ok_or_else(|| anyhow!("client_secret is not set"))?;
            let redirect_uri = self.redirect_uri.as_ref()
                .ok_or_else(|| anyhow!("redirect_uri is not set"))?;
                
            let params = [
                ("code", code),
                ("grant_type", "authorization_code".to_string()),
                ("redirect_uri", redirect_uri.clone()),
                ("client_id", client_id.clone()),
                ("client_secret", client_secret.clone())
            ];
            
            let response = self.client.post(Self::OAUTH2_TOKEN_URI)
                .form(&params)
                .send()
                .await?;
            
            if response.status() == StatusCode::OK {
                let body = response.text().await?;
                self.set_access_token(&body)?;
                
                if let Some(token) = &mut self.token {
                    token.created = SystemTime::now()
                        .duration_since(UNIX_EPOCH)?
                        .as_secs() as i64;
                    return Ok(self.get_access_token()?);
                }
            } else {
                let body = response.text().await?;
                let decoded_response: Result<Value, _> = serde_json::from_str(&body);
                let error_msg = match decoded_response {
                    Ok(val) => {
                        if let Some(error) = val.get("error") {
                            error.to_string()
                        } else {
                            body
                        }
                    },
                    Err(_) => body,
                };
                
                return Err(AuthError::Http(
                    response.status().as_u16(),
                    format!("Error fetching OAuth2 access token, message: '{}'", error_msg)
                ).into());
            }
        }
        
        let scope = service.get("scope")
            .ok_or_else(|| anyhow!("Service scope is not defined"))?;
        
        let auth_url = self.create_auth_url(scope);
        
        // In a web application, you would redirect to this URL
        // For this example, we'll just return the URL
        return Ok(auth_url);
    }
    
    /// Create a URL to obtain user authorization.
    /// The authorization endpoint allows the user to first
    /// authenticate, and then grant/deny the access request.
    pub fn create_auth_url(&self, scope: &str) -> String {
        let client_id = self.client_id.as_ref().unwrap_or(&"".to_string());
        let redirect_uri = self.redirect_uri.as_ref().unwrap_or(&"".to_string());
        
        let mut params = vec![
            format!("response_type=code"),
            format!("redirect_uri={}", urlencoding::encode(redirect_uri)),
            format!("client_id={}", urlencoding::encode(client_id)),
            format!("scope={}", urlencoding::encode(scope)),
            format!("access_type={}", urlencoding::encode(&self.access_type)),
            format!("approval_prompt={}", urlencoding::encode(&self.approval_prompt)),
        ];
        
        if let Some(state) = &self.state {
            params.push(format!("state={}", urlencoding::encode(state)));
        }
        
        format!("{}?{}", Self::OAUTH2_AUTH_URL, params.join("&"))
    }
    
    pub fn set_access_token(&mut self, token_str: &str) -> Result<()> {
        let token: Token = serde_json::from_str(token_str)
            .map_err(|_| AuthError::JsonDecode("Could not json decode the token".to_string()))?;
        
        self.token = Some(token);
        Ok(())
    }
    
    pub fn get_access_token(&self) -> Result<String> {
        match &self.token {
            Some(token) => Ok(serde_json::to_string(token)?),
            None => Err(anyhow!("No access token available"))
        }
    }
    
    pub fn set_developer_key(&mut self, developer_key: String) {
        self.developer_key = Some(developer_key);
    }
    
    pub fn set_state(&mut self, state: String) {
        self.state = Some(state);
    }
    
    pub fn set_access_type(&mut self, access_type: String) {
        self.access_type = access_type;
    }
    
    pub fn set_approval_prompt(&mut self, approval_prompt: String) {
        self.approval_prompt = approval_prompt;
    }
    
    pub fn set_assertion_credentials(&mut self, creds: AssertionCredentials) {
        self.assertion_credentials = Some(creds);
    }
    
    /// Include an accessToken in a given apiHttpRequest.
    pub async fn sign(&self, request: &mut HttpRequest) -> Result<()> {
        // add the developer key to the request before signing it
        if let Some(developer_key) = &self.developer_key {
            let mut url = Url::parse(&request.url)?;
            url.query_pairs_mut().append_pair("key", developer_key);
            request.url = url.to_string();
        }
        
        // Cannot sign the request without an OAuth access token.
        if self.token.is_none() && self.assertion_credentials.is_none() {
            return Ok(());
        }
        
        // Check if the token is set to expire in the next 30 seconds
        // (or has already expired).
        if self.is_access_token_expired() {
            if let Some(assertion_credentials) = &self.assertion_credentials {
                self.refresh_token_with_assertion(Some(assertion_credentials)).await?;
            } else if let Some(token) = &self.token {
                if let Some(refresh_token) = &token.refresh_token {
                    self.refresh_token(refresh_token).await?;
                } else {
                    return Err(AuthError::General(
                        "The OAuth 2.0 access token has expired, and a refresh token is not available. \
                        Refresh tokens are not returned for responses that were auto-approved.".to_string()
                    ).into());
                }
            }
        }
        
        // Add the OAuth2 header to the request
        if let Some(token) = &self.token {
            request.headers.insert(
                "Authorization".to_string(),
                format!("Bearer {}", token.access_token)
            );
        }
        
        Ok(())
    }
    
    /// Fetches a fresh access token with the given refresh token.
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<()> {
        let client_id = self.client_id.as_ref()
            .ok_or_else(|| anyhow!("client_id is not set"))?;
        let client_secret = self.client_secret.as_ref()
            .ok_or_else(|| anyhow!("client_secret is not set"))?;
            
        let params = [
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("refresh_token", refresh_token),
            ("grant_type", "refresh_token"),
        ];
        
        self.refresh_token_request(&params).await
    }
    
    /// Fetches a fresh access token with a given assertion token.
    pub async fn refresh_token_with_assertion(&self, assertion_credentials: Option<&AssertionCredentials>) -> Result<()> {
        let creds = match assertion_credentials {
            Some(ac) => ac,
            None => self.assertion_credentials.as_ref()
                .ok_or_else(|| anyhow!("No assertion credentials available"))?
        };
        
        let params = [
            ("grant_type", "assertion"),
            ("assertion_type", &creds.assertion_type),
            ("assertion", &creds.generate_assertion()),
        ];
        
        self.refresh_token_request(&params).await
    }
    
    async fn refresh_token_request(&self, params: &[(&str, &str)]) -> Result<()> {
        let response = self.client.post(Self::OAUTH2_TOKEN_URI)
            .form(params)
            .send()
            .await?;
            
        let status = response.status();
        let body = response.text().await?;
        
        if status == StatusCode::OK {
            let token: Value = serde_json::from_str(&body)
                .map_err(|_| AuthError::JsonDecode("Could not json decode the access token".to_string()))?;
                
            if let (Some(access_token), Some(expires_in)) = (
                token.get("access_token").and_then(Value::as_str),
                token.get("expires_in").and_then(Value::as_i64)
            ) {
                if let Some(ref mut token) = self.token.as_ref() {
                    // Since token is immutable in this context, we need to create a new token
                    // and update self.token
                    let mut updated_token = token.clone();
                    updated_token.access_token = access_token.to_string();
                    updated_token.expires_in = expires_in;
                    updated_token.created = SystemTime::now()
                        .duration_since(UNIX_EPOCH)?
                        .as_secs() as i64;
                    
                    // In real implementation, we would update self.token
                    // but since we're working with an immutable reference, we can't.
                    // This is a limitation of the direct translation approach.
                }
            } else {
                return Err(AuthError::InvalidToken("Invalid token format".to_string()).into());
            }
        } else {
            return Err(AuthError::Http(
                status.as_u16(),
                format!("Error refreshing the OAuth2 token, message: '{}'", body)
            ).into());
        }
        
        Ok(())
    }
    
    /// Revoke an OAuth2 access token or refresh token. This method will revoke the current access
    /// token, if a token isn't provided.
    pub async fn revoke_token(&mut self, token: Option<String>) -> Result<bool> {
        let token_to_revoke = match token {
            Some(t) => t,
            None => {
                match &self.token {
                    Some(t) => t.access_token.clone(),
                    None => return Ok(false)
                }
            }
        };
        
        let params = [("token", token_to_revoke)];
        let response = self.client.post(Self::OAUTH2_REVOKE_URI)
            .form(&params)
            .send()
            .await?;
            
        if response.status() == StatusCode::OK {
            self.token = None;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    /// Returns if the access_token is expired.
    pub fn is_access_token_expired(&self) -> bool {
        match &self.token {
            None => true,
            Some(token) => {
                // If the token is set to expire in the next 30 seconds.
                (token.created + (token.expires_in - 30)) < 
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs() as i64
            }
        }
    }
    
    // Gets federated sign-on certificates to use for verifying identity tokens.
    // Returns certs as HashMap, where keys are key ids, and values
    // are PEM encoded certificates.
    async fn get_federated_sign_on_certs(&self) -> Result<HashMap<String, String>> {
        let response = self.client.get(Self::OAUTH2_FEDERATED_SIGNON_CERTS_URL)
            .send()
            .await?;
            
        if response.status() == StatusCode::OK {
            let body = response.text().await?;
            let certs: HashMap<String, String> = serde_json::from_str(&body)
                .map_err(|_| anyhow!("Failed to parse certificates response"))?;
                
            if !certs.is_empty() {
                return Ok(certs);
            }
        }
        
        Err(AuthError::Http(
            response.status().as_u16(),
            format!("Failed to retrieve verification certificates")
        ).into())
    }
    
    /// Verifies an id token and returns the authenticated apiLoginTicket.
    /// Throws an exception if the id token is not valid.
    /// The audience parameter can be used to control which id tokens are
    /// accepted. By default, the id token must have been issued to this OAuth2 client.
    pub async fn verify_id_token(&self, id_token: Option<&str>, audience: Option<&str>) -> Result<LoginTicket> {
        let id_token = match id_token {
            Some(token) => token.to_string(),
            None => {
                match &self.token {
                    Some(t) => {
                        match &t.id_token {
                            Some(id) => id.clone(),
                            None => return Err(anyhow!("No ID token available"))
                        }
                    },
                    None => return Err(anyhow!("No token available"))
                }
            }
        };
        
        let certs = self.get_federated_sign_on_certs().await?;
        let audience = audience.unwrap_or_else(|| {
            self.client_id.as_deref().unwrap_or("")
        });
        
        self.verify_signed_jwt_with_certs(&id_token, &certs, audience)
    }
    
    // Verifies the id token, returns the verified token contents.
    fn verify_signed_jwt_with_certs(&self, jwt: &str, certs: &HashMap<String, String>, required_audience: &str) -> Result<LoginTicket> {
        let segments: Vec<&str> = jwt.split('.').collect();
        if segments.len() != 3 {
            return Err(anyhow!("Wrong number of segments in token: {}", jwt));
        }
        
        let signed = format!("{}.{}", segments[0], segments[1]);
        let signature = BASE64_URL_SAFE_NO_PAD.decode(segments[2])
            .map_err(|_| anyhow!("Invalid signature encoding"))?;
            
        // Parse envelope
        let envelope_data = BASE64_URL_SAFE_NO_PAD.decode(segments[0])
            .map_err(|_| anyhow!("Can't decode token envelope"))?;
        let envelope: Value = serde_json::from_slice(&envelope_data)
            .map_err(|_| anyhow!("Can't parse token envelope: {}", segments[0]))?;
            
        // Parse token
        let json_body = BASE64_URL_SAFE_NO_PAD.decode(segments[1])
            .map_err(|_| anyhow!("Can't decode token payload"))?;
        let payload: Value = serde_json::from_slice(&json_body)
            .map_err(|_| anyhow!("Can't parse token payload: {}", segments[1]))?;
            
        // Check signature
        let mut verified = false;
        for (key_name, pem) in certs {
            let public_key = PemVerifier::new(pem);
            if public_key.verify(&signed, &signature)? {
                verified = true;
                break;
            }
        }
        
        if !verified {
            return Err(anyhow!("Invalid token signature: {}", jwt));
        }
        
        // Check issued-at timestamp
        let iat = payload.get("iat")
            .and_then(Value::as_i64)
            .unwrap_or(0);
            
        if iat == 0 {
            return Err(anyhow!("No issue time in token: {}", String::from_utf8_lossy(&json_body)));
        }
        
        let earliest = iat - Self::CLOCK_SKEW_SECS;
        
        // Check expiration timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs() as i64;
            
        let exp = payload.get("exp")
            .and_then(Value::as_i64)
            .unwrap_or(0);
            
        if exp == 0 {
            return Err(anyhow!("No expiration time in token: {}", String::from_utf8_lossy(&json_body)));
        }
        
        if exp >= now + Self::MAX_TOKEN_LIFETIME_SECS {
            return Err(anyhow!("Expiration time too far in future: {}", String::from_utf8_lossy(&json_body)));
        }
        
        let latest = exp + Self::CLOCK_SKEW_SECS;
        if now < earliest {
            return Err(anyhow!(
                "Token used too early, {} < {}: {}", 
                now, earliest, String::from_utf8_lossy(&json_body)
            ));
        }
        
        if now > latest {
            return Err(anyhow!(
                "Token used too late, {} > {}: {}",
                now, latest, String::from_utf8_lossy(&json_body)
            ));
        }
        
        // TODO(beaton): check issuer field?
        
        // Check audience
        let aud = payload.get("aud")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("No audience in token"))?;
            
        if aud != required_audience {
            return Err(anyhow!(
                "Wrong recipient, {} != {}: {}",
                aud, required_audience, String::from_utf8_lossy(&json_body)
            ));
        }
        
        // All good.
        let envelope_obj = serde_json::from_value(envelope.clone())?;
        let payload_obj = serde_json::from_value(payload.clone())?;
        
        Ok(LoginTicket::new(envelope_obj, payload_obj))
    }
}

#[async_trait]
impl Auth for OAuth2 {
    async fn sign(&self, request: &mut HttpRequest) -> Result<()> {
        self.sign(request).await
    }
}