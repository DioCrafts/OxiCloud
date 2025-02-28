/*
 * Copyright 2012 Google Inc.
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

use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::json;
use anyhow::{Result, Context};

// Assumed imports from other modules
use crate::utils::GoogleUtils;
use crate::auth::GoogleOAuth2;
use crate::auth::GoogleP12Signer;

/// Credentials object used for OAuth 2.0 Signed JWT assertion grants.
///
/// Author: Chirag Shah <chirags@google.com>
pub struct GoogleAssertionCredentials {
    pub service_account_name: String,
    pub scopes: String,
    pub private_key: Vec<u8>,
    pub private_key_password: String,
    pub assertion_type: String,
    pub sub: Option<String>,
    /// @deprecated
    /// [http://tools.ietf.org/html/draft-ietf-oauth-json-web-token-06](http://tools.ietf.org/html/draft-ietf-oauth-json-web-token-06)
    pub prn: Option<String>,
}

impl GoogleAssertionCredentials {
    const MAX_TOKEN_LIFETIME_SECS: u64 = 3600;

    /// Create a new GoogleAssertionCredentials instance
    ///
    /// # Arguments
    ///
    /// * `service_account_name` - The service account email
    /// * `scopes` - List of scopes or space-delimited string of scopes
    /// * `private_key` - The private key data
    /// * `private_key_password` - Password for the private key
    /// * `assertion_type` - The assertion type
    /// * `sub` - The email address of the user for which the application is requesting delegated access
    pub fn new<T: Into<String>>(
        service_account_name: T,
        scopes: impl Into<Scopes>,
        private_key: Vec<u8>,
        private_key_password: Option<T>,
        assertion_type: Option<T>,
        sub: Option<T>,
    ) -> Self {
        let scopes_str = match scopes.into() {
            Scopes::String(s) => s,
            Scopes::Vec(v) => v.join(" "),
        };

        let sub_string = sub.map(|s| s.into());
        
        Self {
            service_account_name: service_account_name.into(),
            scopes: scopes_str,
            private_key,
            private_key_password: private_key_password
                .map_or_else(|| "notasecret".to_string(), |p| p.into()),
            assertion_type: assertion_type
                .map_or_else(|| "http://oauth.net/grant_type/jwt/1.0/bearer".to_string(), |a| a.into()),
            sub: sub_string.clone(),
            prn: sub_string,
        }
    }

    /// Generate the assertion JWT
    pub fn generate_assertion(&self) -> Result<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("Failed to get current time")?
            .as_secs();

        let mut jwt_params = json!({
            "aud": GoogleOAuth2::OAUTH2_TOKEN_URI,
            "scope": self.scopes,
            "iat": now,
            "exp": now + Self::MAX_TOKEN_LIFETIME_SECS,
            "iss": self.service_account_name,
        });

        if let Some(sub) = &self.sub {
            jwt_params["sub"] = json!(sub);
        } else if let Some(prn) = &self.prn {
            jwt_params["prn"] = json!(prn);
        }

        self.make_signed_jwt(jwt_params.as_object().unwrap())
    }

    /// Creates a signed JWT.
    ///
    /// # Arguments
    ///
    /// * `payload` - The JWT payload to sign
    ///
    /// # Returns
    ///
    /// The signed JWT string
    fn make_signed_jwt(&self, payload: &serde_json::Map<String, serde_json::Value>) -> Result<String> {
        let header = json!({"typ": "JWT", "alg": "RS256"});
        
        let encoded_header = GoogleUtils::url_safe_b64_encode(&serde_json::to_string(&header)?)?;
        let encoded_payload = GoogleUtils::url_safe_b64_encode(&serde_json::to_string(payload)?)?;
        
        let signing_input = format!("{}.{}", encoded_header, encoded_payload);
        
        let signer = GoogleP12Signer::new(&self.private_key, &self.private_key_password)?;
        let signature = signer.sign(&signing_input)?;
        let encoded_signature = GoogleUtils::url_safe_b64_encode(&signature)?;
        
        Ok(format!("{}.{}.{}", encoded_header, encoded_payload, encoded_signature))
    }
}

/// Helper enum to handle both string and vector of strings for scopes
pub enum Scopes {
    String(String),
    Vec(Vec<String>),
}

impl From<String> for Scopes {
    fn from(s: String) -> Self {
        Scopes::String(s)
    }
}

impl From<&str> for Scopes {
    fn from(s: &str) -> Self {
        Scopes::String(s.to_string())
    }
}

impl From<Vec<String>> for Scopes {
    fn from(v: Vec<String>) -> Self {
        Scopes::Vec(v)
    }
}

impl<'a> From<Vec<&'a str>> for Scopes {
    fn from(v: Vec<&'a str>) -> Self {
        Scopes::Vec(v.into_iter().map(String::from).collect())
    }
}