// Copyright (c) 2011, Robin Appelman <icewind1991@gmail.com>
// This file is licensed under the Affero General Public License version 3 or later.
// See the COPYING-README file.

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
struct User {
    name: String,
    display_name: String,
    groups: Vec<String>,
    quota: String,
    is_quota_user_defined: bool,
    subadmin: Vec<String>,
}

#[derive(Debug, Serialize)]
struct Group {
    name: String,
}

#[derive(Debug, Serialize)]
struct UsersTemplate {
    users: Vec<User>,
    groups: Vec<Group>,
    isadmin: i32,
    subadmins: Option<Vec<(String, String)>>,
    numofgroups: usize,
    quota_preset: Vec<String>,
    default_quota: String,
    default_quota_is_user_defined: bool,
    recovery_admin_enabled: bool,
    enable_avatars: bool,
}

pub async fn users_page(
    app_config: web::Data<AppConfig>,
    user_service: web::Data<UserService>,
    group_service: web::Data<GroupService>,
    preferences_service: web::Data<PreferencesService>,
    app_service: web::Data<AppService>,
) -> impl Responder {
    // Check user permissions
    if !user_service.check_sub_admin_user().await {
        return HttpResponse::Forbidden().finish();
    }

    // Load apps
    app_service.load_apps().await;

    // Add scripts and styles
    app_service.add_script("settings", "users").await;
    app_service.add_script("core", "multiselect").await;
    app_service.add_script("core", "singleselect").await;
    app_service.add_script("core", "jquery.inview").await;
    app_service.add_style("settings", "settings").await;
    app_service.set_active_navigation_entry("core_users").await;

    let mut users = Vec::new();
    let mut groups = Vec::new();

    let current_user = user_service.get_user().await;
    let is_admin = user_service.is_admin_user(&current_user).await;
    let recovery_admin_enabled = app_service.is_enabled("files_encryption").await
        && app_service.get_app_config_value("files_encryption", "recoveryAdminEnabled", "false").await == "true";

    let (accessible_groups, accessible_users, subadmins) = if is_admin {
        (
            group_service.get_groups().await,
            user_service.get_display_names("", 30).await,
            Some(user_service.get_all_sub_admins().await),
        )
    } else {
        let groups = user_service.get_sub_admins_groups(&current_user).await;
        (
            groups.clone(),
            group_service.display_names_in_groups(&groups, "", 30).await,
            None,
        )
    };

    // Load preset quotas
    let quota_preset_str = app_service
        .get_app_config_value("files", "quota_preset", "1 GB, 5 GB, 10 GB")
        .await;
    let mut quota_preset: Vec<String> = quota_preset_str
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    quota_preset.retain(|preset| preset != "default" && preset != "none");

    let default_quota = app_service
        .get_app_config_value("files", "default_quota", "none")
        .await;
    let default_quota_is_user_defined = !quota_preset.contains(&default_quota) 
        && default_quota != "none" 
        && default_quota != "default";

    // Load users and quota
    for (uid, display_name) in accessible_users {
        let quota = preferences_service
            .get_value(&uid, "files", "quota", "default")
            .await;
        let is_quota_user_defined = !quota_preset.contains(&quota) 
            && quota != "none" 
            && quota != "default";

        let user_groups = group_service.get_user_groups(&uid).await;
        let subadmin_groups = user_service.get_sub_admins_groups(&uid).await;

        users.push(User {
            name: uid.clone(),
            display_name: display_name.clone(),
            groups: user_groups,
            quota,
            is_quota_user_defined,
            subadmin: subadmin_groups,
        });
    }

    for group_name in accessible_groups {
        groups.push(Group {
            name: group_name,
        });
    }

    let template = UsersTemplate {
        users,
        groups,
        isadmin: if is_admin { 1 } else { 0 },
        subadmins,
        numofgroups: accessible_groups.len(),
        quota_preset,
        default_quota,
        default_quota_is_user_defined,
        recovery_admin_enabled,
        enable_avatars: app_config.enable_avatars,
    };

    HttpResponse::Ok().json(template)
}

// These would be implemented elsewhere
struct AppConfig {
    enable_avatars: bool,
}

struct UserService;
struct GroupService;
struct PreferencesService;
struct AppService;

impl UserService {
    async fn check_sub_admin_user(&self) -> bool {
        // Implementation
        true
    }

    async fn get_user(&self) -> String {
        // Implementation
        "admin".to_string()
    }

    async fn is_admin_user(&self, user: &str) -> bool {
        // Implementation
        true
    }

    async fn get_display_names(&self, search: &str, limit: usize) -> HashMap<String, String> {
        // Implementation
        HashMap::new()
    }

    async fn get_all_sub_admins(&self) -> Vec<(String, String)> {
        // Implementation
        Vec::new()
    }

    async fn get_sub_admins_groups(&self, user: &str) -> Vec<String> {
        // Implementation
        Vec::new()
    }
}

impl GroupService {
    async fn get_groups(&self) -> Vec<String> {
        // Implementation
        Vec::new()
    }

    async fn display_names_in_groups(&self, groups: &[String], search: &str, limit: usize) -> HashMap<String, String> {
        // Implementation
        HashMap::new()
    }

    async fn get_user_groups(&self, user: &str) -> Vec<String> {
        // Implementation
        Vec::new()
    }
}

impl PreferencesService {
    async fn get_value(&self, user: &str, app: &str, key: &str, default: &str) -> String {
        // Implementation
        default.to_string()
    }
}

impl AppService {
    async fn load_apps(&self) {
        // Implementation
    }

    async fn add_script(&self, app: &str, script: &str) {
        // Implementation
    }

    async fn add_style(&self, app: &str, style: &str) {
        // Implementation
    }

    async fn set_active_navigation_entry(&self, entry: &str) {
        // Implementation
    }

    async fn is_enabled(&self, app: &str) -> bool {
        // Implementation
        false
    }

    async fn get_app_config_value(&self, app: &str, key: &str, default: &str) -> String {
        // Implementation
        default.to_string()
    }
}