// ownCloud API implementation
//
// Originally created by:
// - Tom Needham
// - Michael Gapczynski
// - Bart Visscher
// Copyright 2012 Tom Needham tom@owncloud.com
// Copyright 2012 Michael Gapczynski mtgap@owncloud.com
// Copyright 2012 Bart Visscher bartv@thisnet.nl
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use quick_xml::Writer as XMLWriter;
use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText};
use serde::Serialize;
use serde_json;
use async_trait::async_trait;
use actix_web::{web, HttpRequest, HttpResponse, HttpMessage};
use actix_web::http::StatusCode;
use once_cell::sync::Lazy;

/// API authentication levels
pub struct AuthLevel;

impl AuthLevel {
    pub const GUEST_AUTH: i32 = 0;
    pub const USER_AUTH: i32 = 1;
    pub const SUBADMIN_AUTH: i32 = 2;
    pub const ADMIN_AUTH: i32 = 3;
}

/// API Response Codes
pub struct ResponseCode;

impl ResponseCode {
    pub const RESPOND_UNAUTHORISED: i32 = 997;
    pub const RESPOND_SERVER_ERROR: i32 = 996;
    pub const RESPOND_NOT_FOUND: i32 = 998;
    pub const RESPOND_UNKNOWN_ERROR: i32 = 999;
}

#[derive(Debug, Clone, Serialize)]
pub struct OCSResult {
    meta: HashMap<String, serde_json::Value>,
    data: HashMap<String, serde_json::Value>,
    status_code: i32,
}

impl OCSResult {
    pub fn new(data: Option<HashMap<String, serde_json::Value>>, status_code: i32, message: Option<&str>) -> Self {
        let mut meta = HashMap::new();
        meta.insert("statuscode".to_string(), serde_json::json!(status_code));
        meta.insert("message".to_string(), serde_json::json!(message.unwrap_or("")));
        
        OCSResult {
            meta,
            data: data.unwrap_or_default(),
            status_code,
        }
    }

    pub fn succeeded(&self) -> bool {
        self.status_code == 100
    }

    pub fn get_status_code(&self) -> i32 {
        self.status_code
    }

    pub fn get_meta(&self) -> &HashMap<String, serde_json::Value> {
        &self.meta
    }

    pub fn get_data(&self) -> &HashMap<String, serde_json::Value> {
        &self.data
    }
}

#[derive(Debug, Clone)]
pub struct ActionInfo {
    app: String,
    action: Arc<dyn Fn(HashMap<String, String>) -> OCSResult + Send + Sync>,
    auth_level: i32,
}

type ActionsMap = HashMap<String, Vec<ActionInfo>>;

static ACTIONS: Lazy<Mutex<ActionsMap>> = Lazy::new(|| Mutex::new(HashMap::new()));
static mut LOGOUT_REQUIRED: bool = false;

pub struct API;

impl API {
    /// Registers an api call
    pub fn register<F>(
        method: &str, 
        url: &str, 
        action: F, 
        app: &str, 
        auth_level: i32,
        defaults: Option<HashMap<String, String>>,
        requirements: Option<HashMap<String, String>>
    ) 
    where 
        F: Fn(HashMap<String, String>) -> OCSResult + Send + Sync + 'static
    {
        let name = format!("{}{}", method.to_lowercase(), url);
        let name = name.replace(['/', '{', '}'], "_");
        
        let mut actions = ACTIONS.lock().unwrap();
        
        if !actions.contains_key(&name) {
            // In Rust, we'd use the router's API differently
            // This is a placeholder for where router configuration would happen
            actions.insert(name.clone(), Vec::new());
        }
        
        let action_info = ActionInfo {
            app: app.to_string(),
            action: Arc::new(action),
            auth_level,
        };
        
        actions.get_mut(&name).unwrap().push(action_info);
    }
    
    /// Handles an api call
    pub async fn call(req: HttpRequest, params: web::Path<HashMap<String, String>>, query: web::Query<HashMap<String, String>>) -> HttpResponse {
        let mut parameters = params.into_inner();
        
        // Add query parameters
        for (k, v) in query.into_inner() {
            parameters.insert(k, v);
        }
        
        // Handle PUT/DELETE requests
        if req.method() == "PUT" || req.method() == "DELETE" {
            let mut body = String::new();
            
            if let Ok(mut payload) = req.take_payload() {
                let mut bytes = web::BytesMut::new();
                while let Ok(Some(chunk)) = payload.next().await {
                    bytes.extend_from_slice(&chunk);
                }
                body = String::from_utf8_lossy(&bytes).to_string();
            }
            
            let mut parsed_body = HashMap::new();
            for pair in body.split('&') {
                if let Some(idx) = pair.find('=') {
                    let (key, val) = pair.split_at(idx);
                    parsed_body.insert(key.to_string(), val[1..].to_string());
                }
            }
            
            if req.method() == "PUT" {
                parameters.insert("_put".to_string(), serde_json::to_string(&parsed_body).unwrap());
            } else {
                parameters.insert("_delete".to_string(), serde_json::to_string(&parsed_body).unwrap());
            }
        }
        
        let name = parameters.get("_route").cloned().unwrap_or_default();
        
        // Process each registered action
        let mut responses = Vec::new();
        
        let actions_clone = {
            let actions = ACTIONS.lock().unwrap();
            match actions.get(&name) {
                Some(actions_for_route) => actions_for_route.clone(),
                None => Vec::new(),
            }
        };
        
        for action in actions_clone {
            // Check authentication and availability
            if !API::is_authorised(&action) {
                responses.push((
                    action.app.clone(),
                    OCSResult::new(None, ResponseCode::RESPOND_UNAUTHORISED, Some("Unauthorised")),
                ));
                continue;
            }
            
            // Run the action
            let response = (action.action)(parameters.clone());
            responses.push((action.app.clone(), response));
        }
        
        let format = parameters.get("format").map(|f| f.as_str()).unwrap_or("xml");
        
        let response = API::merge_responses(&responses);
        
        unsafe {
            if LOGOUT_REQUIRED {
                // User::logout() would be called here in actual implementation
                LOGOUT_REQUIRED = false;
            }
        }
        
        API::respond(response, format)
    }
    
    /// Merge the returned result objects into one response
    fn merge_responses(responses: &[(String, OCSResult)]) -> OCSResult {
        let mut shipped_succeeded = HashMap::new();
        let mut shipped_failed = HashMap::new();
        let mut thirdparty_succeeded = HashMap::new();
        let mut thirdparty_failed = HashMap::new();
        
        for (app, response) in responses {
            // This is a simplified version of the app checking logic
            let is_shipped = app == "core" || API::is_shipped(app);
            
            if is_shipped {
                if response.succeeded() {
                    shipped_succeeded.insert(app.clone(), response.clone());
                } else {
                    shipped_failed.insert(app.clone(), response.clone());
                }
            } else {
                if response.succeeded() {
                    thirdparty_succeeded.insert(app.clone(), response.clone());
                } else {
                    thirdparty_failed.insert(app.clone(), response.clone());
                }
            }
        }
        
        // Remove any error responses if there is one shipped response that succeeded
        if !shipped_succeeded.is_empty() {
            let mut data = HashMap::new();
            
            // Merge data from all successful responses
            for (_, response) in shipped_succeeded.iter().chain(thirdparty_succeeded.iter()) {
                for (key, value) in response.get_data() {
                    data.insert(key.clone(), value.clone());
                }
            }
            
            return OCSResult::new(Some(data), 100, None);
        } else if !shipped_failed.is_empty() {
            // Return first shipped failure
            return shipped_failed.values().next().unwrap().clone();
        } else if !thirdparty_failed.is_empty() {
            // Return first third-party failure
            return thirdparty_failed.values().next().unwrap().clone();
        } else {
            // If we got here, there are no responses at all
            return OCSResult::new(None, 100, None);
        }
    }
    
    /// Check if app is shipped with core
    fn is_shipped(app: &str) -> bool {
        // This would check if the app is part of the core distribution
        // Simplified implementation
        matches!(app, "files" | "settings" | "dav" | "federatedfilesharing")
    }
    
    /// Authenticate the api call
    fn is_authorised(action: &ActionInfo) -> bool {
        let level = action.auth_level;
        match level {
            AuthLevel::GUEST_AUTH => true,
            AuthLevel::USER_AUTH => API::login_user(),
            AuthLevel::SUBADMIN_AUTH => {
                let user = API::login_user();
                if !user {
                    false
                } else {
                    // For this example, we'll simplify the check
                    API::is_subadmin() || API::is_admin()
                }
            },
            AuthLevel::ADMIN_AUTH => {
                let user = API::login_user();
                if !user {
                    false
                } else {
                    API::is_admin()
                }
            },
            _ => false,
        }
    }
    
    /// HTTP basic auth
    fn login_user() -> bool {
        // This would be implemented based on your auth system
        // Placeholder implementation
        let auth_header = std::env::var("HTTP_AUTHORIZATION").ok();
        
        if let Some(auth) = auth_header {
            if auth.starts_with("Basic ") {
                // Decode and verify credentials
                // Set LOGOUT_REQUIRED = true on success
                unsafe { LOGOUT_REQUIRED = true; }
                
                // In a real implementation, we'd set up the user's filesystem
                return true;
            }
        }
        
        // Check for existing login
        let logged_in = API::is_logged_in();
        let ocs_api_request = std::env::var("HTTP_OCS_APIREQUEST").unwrap_or_default() == "true";
        
        if logged_in && ocs_api_request {
            // In a real implementation, we'd set up the user's filesystem
            return true;
        }
        
        false
    }
    
    /// Check if a user is logged in
    fn is_logged_in() -> bool {
        // Placeholder - would check session/token validity
        false
    }
    
    /// Check if user is admin
    fn is_admin() -> bool {
        // Placeholder - would check user privileges
        false
    }
    
    /// Check if user is subadmin
    fn is_subadmin() -> bool {
        // Placeholder - would check user privileges
        false
    }
    
    /// Respond to a call
    fn respond(result: OCSResult, format: &str) -> HttpResponse {
        let mut response = HttpResponse::Ok();
        
        // Send 401 headers if unauthorised
        if result.get_status_code() == ResponseCode::RESPOND_UNAUTHORISED {
            return HttpResponse::Unauthorized()
                .header("WWW-Authenticate", "Basic realm=\"Authorisation Required\"")
                .json(result);
        }
        
        let ocs_response = json!({
            "ocs": {
                "meta": result.get_meta(),
                "data": result.get_data(),
            }
        });
        
        if format == "json" {
            response.content_type("application/json");
            response.body(serde_json::to_string(&ocs_response).unwrap())
        } else {
            response.content_type("text/xml; charset=UTF-8");
            
            let mut xml_output = Vec::new();
            {
                let mut writer = XMLWriter::new(&mut xml_output);
                API::to_xml(&ocs_response, &mut writer).unwrap();
            }
            
            response.body(String::from_utf8(xml_output).unwrap())
        }
    }
    
    /// Convert to XML
    fn to_xml<W: Write>(data: &serde_json::Value, writer: &mut XMLWriter<W>) -> Result<(), quick_xml::Error> {
        match data {
            serde_json::Value::Object(map) => {
                for (k, v) in map {
                    if k.starts_with('@') {
                        // Handle attributes
                        // This is simplified, in a complete implementation you would
                        // attach these to the parent element
                        continue;
                    }
                    
                    let mut elem = BytesStart::new(k);
                    writer.write_event(Event::Start(elem))?;
                    API::to_xml(v, writer)?;
                    writer.write_event(Event::End(BytesEnd::new(k)))?;
                }
            },
            serde_json::Value::Array(arr) => {
                for (i, v) in arr.iter().enumerate() {
                    let elem_name = "element";
                    writer.write_event(Event::Start(BytesStart::new(elem_name)))?;
                    API::to_xml(v, writer)?;
                    writer.write_event(Event::End(BytesEnd::new(elem_name)))?;
                }
            },
            serde_json::Value::String(s) => {
                writer.write_event(Event::Text(BytesText::new(s)))?;
            },
            _ => {
                writer.write_event(Event::Text(BytesText::new(&data.to_string())))?;
            }
        }
        Ok(())
    }
}