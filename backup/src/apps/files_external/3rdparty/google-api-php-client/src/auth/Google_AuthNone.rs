/*
 * Copyright 2010 Google Inc.
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
use crate::auth::Auth;
use crate::http_request::HttpRequest;
use std::sync::Arc;
use url::percent_encoding::{percent_encode, NON_ALPHANUMERIC};

/// Do-nothing authentication implementation, use this if you want to make un-authenticated calls
/// @author Chris Chabot <chabotc@google.com>
/// @author Chirag Shah <chirags@google.com>
pub struct AuthNone {
    key: Option<String>,
}

impl AuthNone {
    pub fn new(api_config: &crate::config::ApiConfig) -> Self {
        let mut auth = Self { key: None };
        
        if let Some(developer_key) = &api_config.developer_key {
            auth.set_developer_key(developer_key.clone());
        }
        
        auth
    }

    pub fn set_developer_key(&mut self, key: String) {
        self.key = Some(key);
    }
}

#[async_trait::async_trait]
impl Auth for AuthNone {
    async fn authenticate(&self, _service: &str) -> Result<(), crate::error::Error> {
        // noop
        Ok(())
    }
    
    fn set_access_token(&mut self, _access_token: &str) {
        // noop
    }
    
    fn get_access_token(&self) -> Option<String> {
        None
    }
    
    fn create_auth_url(&self, _scope: &str) -> Option<String> {
        None
    }
    
    async fn refresh_token(&mut self, _refresh_token: &str) -> Result<(), crate::error::Error> {
        // noop
        Ok(())
    }
    
    async fn revoke_token(&mut self) -> Result<(), crate::error::Error> {
        // noop
        Ok(())
    }
    
    fn sign(&self, mut request: HttpRequest) -> Result<HttpRequest, crate::error::Error> {
        if let Some(key) = &self.key {
            let url = request.get_url();
            let separator = if url.contains('?') { "&" } else { "?" };
            let encoded_key = percent_encode(key.as_bytes(), NON_ALPHANUMERIC).to_string();
            let new_url = format!("{}{}key={}", url, separator, encoded_key);
            request.set_url(&new_url);
        }
        Ok(request)
    }
}