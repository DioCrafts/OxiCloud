// Share handler functionality in Rust.
//
// Based on ownCloud's share.php by Michael Gapczynski
// Copyright 2012 Michael Gapczynski mtgap@owncloud.com
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
use chrono::{DateTime, NaiveDateTime, Utc};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use axum::{
    extract::{Form, Query},
    response::Json,
    http::StatusCode,
    Router, 
    routing::{post, get},
};

#[derive(Debug, Serialize)]
struct JsonSuccess<T> {
    status: &'static str,
    data: T,
}

#[derive(Debug, Serialize)]
struct JsonError {
    status: &'static str,
    data: HashMap<&'static str, String>,
}

#[derive(Debug, Deserialize)]
struct ShareAction {
    action: String,
    item_type: String,
    item_source: String,
    share_type: Option<i32>,
    share_with: Option<String>,
    permissions: Option<i32>,
    item_source_name: Option<String>,
    date: Option<String>,
    recipient: Option<String>,
    link: Option<String>,
    file: Option<String>,
    to_address: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FetchParams {
    fetch: String,
    item_type: Option<String>,
    item_source: Option<String>,
    check_reshare: Option<String>,
    check_shares: Option<String>,
    search: Option<String>,
}

#[derive(Debug, Serialize)]
struct ShareData {
    token: String,
}

#[derive(Debug, Serialize)]
struct ShareStatus {
    reshare: Option<bool>,
    shares: Option<bool>,
}

#[derive(Debug, Serialize)]
struct ShareWithEntry {
    label: String,
    value: ShareWithValue,
}

#[derive(Debug, Serialize)]
struct ShareWithValue {
    share_type: i32,
    share_with: String,
}

// Constants
const SHARE_TYPE_USER: i32 = 0;
const SHARE_TYPE_GROUP: i32 = 1;
const SHARE_TYPE_LINK: i32 = 3;
const FORMAT_NONE: i32 = 0;
const FORMAT_STATUSES: i32 = 1;

#[async_trait]
trait UserTrait {
    async fn get_user() -> String;
    async fn get_display_name() -> String;
    async fn get_display_name_for(uid: &str) -> String;
    async fn get_display_names(search: &str, limit: i32, offset: i32) -> HashMap<String, String>;
}

#[async_trait]
trait ShareTrait {
    async fn share_item(
        item_type: &str,
        item_source: &str,
        share_type: i32,
        share_with: Option<&str>,
        permissions: i32,
        item_source_name: &str,
    ) -> Result<Option<String>, String>;
    
    async fn unshare(
        item_type: &str,
        item_source: &str,
        share_type: i32,
        share_with: Option<&str>,
    ) -> Result<bool, String>;
    
    async fn set_permissions(
        item_type: &str,
        item_source: &str,
        share_type: i32,
        share_with: &str,
        permissions: i32,
    ) -> Result<bool, String>;
    
    async fn set_expiration_date(
        item_type: &str,
        item_source: &str,
        date: &str,
    ) -> Result<bool, String>;
    
    async fn get_items_shared(
        item_type: &str,
        format: i32,
    ) -> Result<Vec<HashMap<String, String>>, String>;
    
    async fn get_item_shared_with_by_source(
        item_type: &str,
        item_source: &str,
        format: i32,
        uid: Option<&str>,
        include_all: bool,
    ) -> Result<bool, String>;
    
    async fn get_item_shared(
        item_type: &str,
        item_source: &str,
        format: i32,
        uid: Option<&str>,
        include_all: bool,
    ) -> Result<bool, String>;
    
    async fn get_item_shared_with_user(
        item_type: &str,
        item_source: &str,
        uid: &str,
    ) -> Result<Vec<HashMap<String, String>>, String>;
    
    async fn set_send_mail_status(
        item_type: &str,
        item_source: &str,
        share_type: i32,
        status: bool,
    ) -> Result<(), String>;
}

#[async_trait]
trait UtilTrait {
    async fn get_default_email_address(purpose: &str) -> String;
    async fn link_to_absolute(app: &str, file: &str, args: HashMap<&str, &str>) -> String;
    async fn send_mail(
        to: &str, 
        to_display_name: &str,
        subject: &str,
        body: &str,
        from: &str,
        from_display_name: &str,
        html: i32,
        alt_body: &str,
    ) -> Result<(), String>;
    async fn sanitize_html(html: &str) -> String;
}

#[async_trait]
trait GroupTrait {
    async fn get_groups(search: &str) -> Vec<String>;
    async fn get_user_groups(uid: &str) -> Vec<String>;
    async fn display_names_in_groups(groups: &[String], search: &str, limit: i32, offset: i32) -> HashMap<String, String>;
    async fn users_in_group(group: &str) -> Vec<String>;
}

#[async_trait]
trait ConfigTrait {
    async fn get_user_value(uid: &str, app: &str, key: &str, default: &str) -> String;
    async fn get_app_value(app: &str, key: &str, default: &str) -> String;
}

#[async_trait]
trait PreferencesTrait {
    async fn get_value(uid: &str, app: &str, key: &str, default: &str) -> String;
}

#[async_trait]
trait TemplateTrait {
    async fn new(app: &str, name: &str, render_as: &str) -> Self;
    async fn assign(&mut self, key: &str, value: &str);
    async fn assign_expiration(&mut self, key: &str, value: Option<String>);
    async fn fetch_page(&self) -> String;
}

struct Template {
    app: String,
    name: String,
    render_as: String,
    variables: HashMap<String, String>,
    expiration: Option<String>,
}

#[async_trait]
impl TemplateTrait for Template {
    async fn new(app: &str, name: &str, render_as: &str) -> Self {
        Template {
            app: app.to_string(),
            name: name.to_string(),
            render_as: render_as.to_string(),
            variables: HashMap::new(),
            expiration: None,
        }
    }
    
    async fn assign(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }
    
    async fn assign_expiration(&mut self, key: &str, value: Option<String>) {
        self.expiration = value;
    }
    
    async fn fetch_page(&self) -> String {
        // Mock implementation
        format!("Template rendered: {}/{}.{}", self.app, self.name, self.render_as)
    }
}

struct Localization;

impl Localization {
    fn get(app: &str) -> Self {
        Self
    }
    
    fn t(&self, string: &str, replacements: Vec<&str>) -> String {
        let mut result = string.to_string();
        for (i, replacement) in replacements.iter().enumerate() {
            result = result.replace(&format!("%s", i), replacement);
        }
        result
    }
}

async fn handle_share_action(form: Form<ShareAction>) -> Json<serde_json::Value> {
    let action = form.0;
    
    // Check that we have the required fields
    if action.item_type.is_empty() || action.item_source.is_empty() {
        return Json(serde_json::json!({
            "status": "error",
            "data": {
                "message": "Missing required parameters"
            }
        }));
    }
    
    match action.action.as_str() {
        "share" => handle_share(action).await,
        "unshare" => handle_unshare(action).await,
        "setPermissions" => handle_set_permissions(action).await,
        "setExpirationDate" => handle_set_expiration_date(action).await,
        "informRecipients" => handle_inform_recipients(action).await,
        "informRecipientsDisabled" => handle_inform_recipients_disabled(action).await,
        "email" => handle_email(action).await,
        _ => Json(serde_json::json!({
            "status": "error",
            "data": {
                "message": "Unknown action"
            }
        }))
    }
}

async fn handle_share(action: ShareAction) -> Json<serde_json::Value> {
    // Validate required parameters
    if action.share_type.is_none() || action.permissions.is_none() {
        return Json(serde_json::json!({
            "status": "error",
            "data": {
                "message": "Missing required parameters for share action"
            }
        }));
    }
    
    let share_type = action.share_type.unwrap();
    let share_with = match (share_type, action.share_with) {
        (SHARE_TYPE_LINK, Some(ref s)) if s.is_empty() => None,
        (_, Some(s)) => Some(s.as_str()),
        _ => None,
    };
    
    let item_source_name = action.item_source_name.unwrap_or_default();
    
    // Call the share service
    match ShareService::share_item(
        &action.item_type, 
        &action.item_source, 
        share_type, 
        share_with,
        action.permissions.unwrap(),
        &item_source_name
    ).await {
        Ok(Some(token)) => Json(serde_json::json!({
            "status": "success",
            "data": {
                "token": token
            }
        })),
        Ok(None) => Json(serde_json::json!({
            "status": "success"
        })),
        Err(e) => Json(serde_json::json!({
            "status": "error",
            "data": {
                "message": e
            }
        }))
    }
}

async fn handle_unshare(action: ShareAction) -> Json<serde_json::Value> {
    // Validate required parameters
    if action.share_type.is_none() || action.share_with.is_none() {
        return Json(serde_json::json!({
            "status": "error",
            "data": {
                "message": "Missing required parameters for unshare action"
            }
        }));
    }
    
    let share_type = action.share_type.unwrap();
    let share_with = match (share_type, action.share_with) {
        (SHARE_TYPE_LINK, Some(ref s)) if s.is_empty() => None,
        (_, Some(s)) => Some(s.as_str()),
        _ => None,
    };
    
    match ShareService::unshare(
        &action.item_type,
        &action.item_source,
        share_type,
        share_with
    ).await {
        Ok(true) => Json(serde_json::json!({
            "status": "success"
        })),
        _ => Json(serde_json::json!({
            "status": "error"
        }))
    }
}

async fn handle_set_permissions(action: ShareAction) -> Json<serde_json::Value> {
    // Validate required parameters
    if action.share_type.is_none() || action.share_with.is_none() || action.permissions.is_none() {
        return Json(serde_json::json!({
            "status": "error",
            "data": {
                "message": "Missing required parameters for setPermissions action"
            }
        }));
    }
    
    match ShareService::set_permissions(
        &action.item_type,
        &action.item_source,
        action.share_type.unwrap(),
        &action.share_with.unwrap(),
        action.permissions.unwrap()
    ).await {
        Ok(true) => Json(serde_json::json!({
            "status": "success"
        })),
        _ => Json(serde_json::json!({
            "status": "error"
        }))
    }
}

async fn handle_set_expiration_date(action: ShareAction) -> Json<serde_json::Value> {
    if action.date.is_none() {
        return Json(serde_json::json!({
            "status": "error",
            "data": {
                "message": "Missing date parameter"
            }
        }));
    }
    
    match ShareService::set_expiration_date(
        &action.item_type,
        &action.item_source,
        &action.date.unwrap()
    ).await {
        Ok(true) => Json(serde_json::json!({
            "status": "success"
        })),
        _ => Json(serde_json::json!({
            "status": "error"
        }))
    }
}

async fn handle_inform_recipients(action: ShareAction) -> Json<serde_json::Value> {
    if action.share_type.is_none() || action.recipient.is_none() {
        return Json(serde_json::json!({
            "status": "error",
            "data": {
                "message": "Missing required parameters for informRecipients action"
            }
        }));
    }
    
    let l = Localization::get("core");
    let share_type = action.share_type.unwrap();
    let item_type = &action.item_type;
    let item_source = &action.item_source;
    let recipient = action.recipient.unwrap();
    let owner_display_name = UserService::get_display_name().await;
    let from = UtilService::get_default_email_address("sharing-noreply").await;
    
    let mut no_mail = Vec::new();
    let mut recipient_list = Vec::new();
    
    if share_type == SHARE_TYPE_USER {
        recipient_list.push(recipient.clone());
    } else if share_type == SHARE_TYPE_GROUP {
        recipient_list = GroupService::users_in_group(&recipient).await;
    }
    
    // Don't send a mail to the user who shared the file
    let current_user = UserService::get_user().await;
    recipient_list.retain(|r| r != &current_user);
    
    // Send mail to all recipients with an email address
    for recipient in recipient_list {
        let email = PreferencesService::get_value(&recipient, "settings", "email", "").await;
        
        if !email.is_empty() {
            let display_name = UserService::get_display_name_for(&recipient).await;
            let items = match ShareService::get_item_shared_with_user(
                item_type,
                item_source,
                &recipient
            ).await {
                Ok(items) => items,
                Err(_) => continue,
            };
            
            if items.is_empty() {
                continue;
            }
            
            let filename = items[0].get("file_target").unwrap_or(&String::new()).trim_matches('/');
            let subject = l.t("%s shared »%s« with you", vec![&owner_display_name, filename]);
            
            let expiration = match items[0].get("expiration") {
                Some(exp) if !exp.is_empty() => {
                    // Convert to date format
                    let date = NaiveDateTime::parse_from_str(exp, "%Y-%m-%d %H:%M:%S")
                        .ok()
                        .map(|dt| format!("{}", dt.date()));
                    date
                },
                _ => None,
            };
            
            let foldername = if item_type == "folder" {
                format!("/Shared/{}", filename)
            } else {
                String::from("/Shared")
            };
            
            let mut args = HashMap::new();
            args.insert("dir", &foldername);
            let link = UtilService::link_to_absolute("files", "index.php", args).await;
            
            let mut content = Template::new("core", "mail", "").await;
            content.assign("link", &link).await;
            content.assign("user_displayname", &owner_display_name).await;
            content.assign("filename", filename).await;
            content.assign_expiration("expiration", expiration.clone()).await;
            let text = content.fetch_page().await;
            
            let mut alt_content = Template::new("core", "altmail", "").await;
            alt_content.assign("link", &link).await;
            alt_content.assign("user_displayname", &owner_display_name).await;
            alt_content.assign("filename", filename).await;
            alt_content.assign_expiration("expiration", expiration).await;
            let alt_text = alt_content.fetch_page().await;
            
            let default_from = UtilService::get_default_email_address("sharing-noreply").await;
            let from = ConfigService::get_user_value(&UserService::get_user().await, "settings", "email", &default_from).await;
            
            // Try to send email
            if let Err(_) = UtilService::send_mail(
                &email, 
                &display_name, 
                &subject, 
                &text, 
                &from, 
                &owner_display_name, 
                1, 
                &alt_text
            ).await {
                no_mail.push(display_name);
            }
        }
    }
    
    // Mark mail as sent
    let _ = ShareService::set_send_mail_status(
        item_type,
        item_source,
        share_type,
        true
    ).await;
    
    if no_mail.is_empty() {
        Json(serde_json::json!({
            "status": "success"
        }))
    } else {
        let l = Localization::get("core");
        let message = l.t("Couldn't send mail to following users: %s", vec![&no_mail.join(", ")]);
        
        Json(serde_json::json!({
            "status": "error",
            "data": {
                "message": message
            }
        }))
    }
}

async fn handle_inform_recipients_disabled(action: ShareAction) -> Json<serde_json::Value> {
    if action.share_type.is_none() {
        return Json(serde_json::json!({
            "status": "error",
            "data": {
                "message": "Missing required parameters"
            }
        }));
    }
    
    let _ = ShareService::set_send_mail_status(
        &action.item_type,
        &action.item_source,
        action.share_type.unwrap(),
        false
    ).await;
    
    Json(serde_json::json!({
        "status": "success"
    }))
}

async fn handle_email(action: ShareAction) -> Json<serde_json::Value> {
    if action.link.is_none() || action.file.is_none() || action.to_address.is_none() {
        return Json(serde_json::json!({
            "status": "error",
            "data": {
                "message": "Missing required parameters for email action"
            }
        }));
    }
    
    let user = UserService::get_user().await;
    let display_name = UserService::get_display_name().await;
    let link = action.link.unwrap();
    let file = action.file.unwrap();
    let to_address = action.to_address.unwrap();
    
    // Get localization
    let l = Localization::get("core");
    let subject = l.t("%s shared »%s« with you", vec![&display_name, &file]);
    
    let mut content = Template::new("core", "mail", "").await;
    content.assign("link", &link).await;
    content.assign("type", &action.item_type).await;
    content.assign("user_displayname", &display_name).await;
    content.assign("filename", &file).await;
    let text = content.fetch_page().await;
    
    let mut alt_content = Template::new("core", "altmail", "").await;
    alt_content.assign("link", &link).await;
    alt_content.assign("type", &action.item_type).await;
    alt_content.assign("user_displayname", &display_name).await;
    alt_content.assign("filename", &file).await;
    let alt_text = alt_content.fetch_page().await;
    
    let default_from = UtilService::get_default_email_address("sharing-noreply").await;
    let from_address = ConfigService::get_user_value(&user, "settings", "email", &default_from).await;
    
    // Send email
    match UtilService::send_mail(
        &to_address,
        &to_address,
        &subject,
        &text,
        &from_address,
        &display_name,
        1,
        &alt_text
    ).await {
        Ok(_) => Json(serde_json::json!({
            "status": "success"
        })),
        Err(e) => Json(serde_json::json!({
            "status": "error",
            "data": {
                "message": UtilService::sanitize_html(&e).await
            }
        }))
    }
}

async fn handle_fetch(query: Query<FetchParams>) -> Json<serde_json::Value> {
    match query.fetch.as_str() {
        "getItemsSharedStatuses" => handle_get_items_shared_statuses(&query).await,
        "getItem" => handle_get_item(&query).await,
        "getShareWith" => handle_get_share_with(&query).await,
        _ => Json(serde_json::json!({
            "status": "error",
            "data": {
                "message": "Unknown fetch action"
            }
        }))
    }
}

async fn handle_get_items_shared_statuses(query: &Query<FetchParams>) -> Json<serde_json::Value> {
    if let Some(ref item_type) = query.item_type {
        match ShareService::get_items_shared(item_type, FORMAT_STATUSES).await {
            Ok(statuses) => Json(serde_json::json!({
                "status": "success",
                "data": statuses
            })),
            Err(_) => Json(serde_json::json!({
                "status": "error"
            }))
        }
    } else {
        Json(serde_json::json!({
            "status": "error",
            "data": {
                "message": "Missing item_type parameter"
            }
        }))
    }
}

async fn handle_get_item(query: &Query<FetchParams>) -> Json<serde_json::Value> {
    if query.item_type.is_none() || query.item_source.is_none() || 
       query.check_reshare.is_none() || query.check_shares.is_none() {
        return Json(serde_json::json!({
            "status": "error",
            "data": {
                "message": "Missing required parameters"
            }
        }));
    }
    
    let item_type = query.item_type.as_ref().unwrap();
    let item_source = query.item_source.as_ref().unwrap();
    let check_reshare = query.check_reshare.as_ref().unwrap() == "true";
    let check_shares = query.check_shares.as_ref().unwrap() == "true";
    
    let reshare = if check_reshare {
        ShareService::get_item_shared_with_by_source(
            item_type, 
            item_source, 
            FORMAT_NONE, 
            None, 
            true
        ).await.unwrap_or(false)
    } else {
        false
    };
    
    let shares = if check_shares {
        ShareService::get_item_shared(
            item_type,
            item_source,
            FORMAT_NONE,
            None,
            true
        ).await.unwrap_or(false)
    } else {
        false
    };
    
    Json(serde_json::json!({
        "status": "success",
        "data": {
            "reshare": reshare,
            "shares": shares
        }
    }))
}

async fn handle_get_share_with(query: &Query<FetchParams>) -> Json<serde_json::Value> {
    if query.search.is_none() {
        return Json(serde_json::json!({
            "status": "error",
            "data": {
                "message": "Missing search parameter"
            }
        }));
    }
    
    let search = query.search.as_ref().unwrap();
    let share_policy = ConfigService::get_app_value("core", "shareapi_share_policy", "global").await;
    
    let mut share_with = Vec::new();
    
    // Get groups that match the search
    let mut groups = GroupService::get_groups(search).await;
    
    // Filter groups based on policy
    if share_policy == "groups_only" {
        let user_groups = GroupService::get_user_groups(&UserService::get_user().await).await;
        groups.retain(|g| user_groups.contains(g));
    }
    
    // Get users that match the search
    let mut count = 0;
    let mut users = HashMap::new();
    let mut limit = 0;
    let mut offset = 0;
    let current_user = UserService::get_user().await;
    
    while count < 15 && users.len() as i32 == limit {
        limit = 15 - count;
        
        users = if share_policy == "groups_only" {
            let user_groups = GroupService::get_user_groups(&current_user).await;
            GroupService::display_names_in_groups(&user_groups, search, limit, offset).await
        } else {
            UserService::get_display_names(search, limit, offset).await
        };
        
        offset += limit;
        
        // Add matching users to result
        for (uid, display_name) in users {
            // Skip current user and already shared users
            if uid != current_user {
                share_with.push(ShareWithEntry {
                    label: display_name,
                    value: ShareWithValue {
                        share_type: SHARE_TYPE_USER,
                        share_with: uid,
                    }
                });
                count += 1;
            }
        }
    }
    
    // Add matching groups to result (limited to 15)
    count = 0;
    for group in groups {
        if count < 15 {
            share_with.push(ShareWithEntry {
                label: group.clone(),
                value: ShareWithValue {
                    share_type: SHARE_TYPE_GROUP,
                    share_with: group,
                }
            });
            count += 1;
        } else {
            break;
        }
    }
    
    Json(serde_json::json!({
        "status": "success",
        "data": share_with
    }))
}

// Service implementation mocks
struct UserService;
struct ShareService;
struct UtilService;
struct GroupService;
struct ConfigService;
struct PreferencesService;

#[async_trait]
impl UserTrait for UserService {
    async fn get_user() -> String {
        "current_user".to_string()
    }
    
    async fn get_display_name() -> String {
        "Current User".to_string()
    }
    
    async fn get_display_name_for(uid: &str) -> String {
        format!("User {}", uid)
    }
    
    async fn get_display_names(search: &str, limit: i32, offset: i32) -> HashMap<String, String> {
        let mut result = HashMap::new();
        result.insert("user1".to_string(), "User One".to_string());
        result.insert("user2".to_string(), "User Two".to_string());
        result
    }
}

#[async_trait]
impl ShareTrait for ShareService {
    async fn share_item(
        item_type: &str,
        item_source: &str,
        share_type: i32,
        share_with: Option<&str>,
        permissions: i32,
        item_source_name: &str,
    ) -> Result<Option<String>, String> {
        // Mock implementation
        if share_type == SHARE_TYPE_LINK {
            Ok(Some("token123".to_string()))
        } else {
            Ok(None)
        }
    }
    
    async fn unshare(
        item_type: &str,
        item_source: &str,
        share_type: i32,
        share_with: Option<&str>,
    ) -> Result<bool, String> {
        // Mock implementation
        Ok(true)
    }
    
    async fn set_permissions(
        item_type: &str,
        item_source: &str,
        share_type: i32,

} // Añadido por reparador automático