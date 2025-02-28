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

use async_trait::async_trait;
use std::error::Error;

mod google_auth_none;
mod google_oauth2;

pub use google_auth_none::GoogleAuthNone;
pub use google_oauth2::GoogleOAuth2;

/**
 * Trait for the Authentication in the API client
 * @author Chris Chabot <chabotc@google.com>
 *
 */
#[async_trait]
pub trait GoogleAuth {
    async fn authenticate(&self, service: &dyn Any) -> Result<(), Box<dyn Error>>;
    async fn sign(&self, request: &mut GoogleHttpRequest) -> Result<(), Box<dyn Error>>;
    fn create_auth_url(&self, scope: &str) -> String;

    fn get_access_token(&self) -> Option<String>;
    fn set_access_token(&mut self, access_token: String);
    fn set_developer_key(&mut self, developer_key: String);
    async fn refresh_token(&mut self, refresh_token: &str) -> Result<(), Box<dyn Error>>;
    async fn revoke_token(&mut self) -> Result<(), Box<dyn Error>>;
}