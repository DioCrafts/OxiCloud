use actix_web::{web, HttpResponse, Responder, get, post, delete, Error};
use serde::{Serialize, Deserialize};
use askama::Template;
use std::collections::HashMap;

/// Copyright (c) 2011, Robin Appelman <icewind1991@gmail.com>
/// This file is licensed under the Affero General Public License version 3 or later.
/// See the COPYING-README file.

#[derive(Template)]
#[template(path = "users.html")]
struct UsersTemplate {
    groups: Vec<Group>,
    subadmingroups: Vec<String>,
    users: Vec<User>,
    recovery_admin_enabled: bool,
    is_admin: bool,
    default_quota: String,
    quota_preset: Vec<String>,
    default_quota_is_user_defined: bool,
    enable_avatars: bool,
    subadmins: Option<Vec<String>>,
    l: Translator,
}

#[derive(Serialize, Deserialize, Clone)]
struct Group {
    name: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct User {
    name: String,
    display_name: String,
    groups: Vec<String>,
    subadmin: Option<Vec<String>>,
    quota: String,
    is_quota_user_defined: bool,
}

struct Translator {
    // In a real implementation, this would contain translation logic
}

impl Translator {
    fn t(&self, key: &str) -> String {
        // Simplified implementation - in reality would look up translations
        key.to_string()
    }
}

#[get("/users")]
async fn users_page(
    app_data: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let groups = app_data.get_groups().await?;
    let users = app_data.get_users().await?;
    
    let all_groups: Vec<String> = groups.iter().map(|g| g.name.clone()).collect();
    
    // Filter out admin from subadmingroups
    let mut subadmingroups = all_groups.clone();
    subadmingroups.retain(|g| g != "admin");
    
    let template = UsersTemplate {
        groups,
        subadmingroups,
        users,
        recovery_admin_enabled: app_data.recovery_admin_enabled,
        is_admin: app_data.is_admin(),
        default_quota: app_data.default_quota.clone(),
        quota_preset: app_data.quota_preset.clone(),
        default_quota_is_user_defined: app_data.default_quota_is_user_defined,
        enable_avatars: app_data.enable_avatars,
        subadmins: app_data.subadmins.clone(),
        l: Translator {},
    };
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(template.render().unwrap()))
}

#[get("/isadmin.js")]
async fn is_admin_js(app_data: web::Data<AppState>) -> impl Responder {
    let is_admin = app_data.is_admin();
    let js = format!("var isadmin = {};", if is_admin { "true" } else { "false" });
    
    HttpResponse::Ok()
        .content_type("application/javascript")
        .body(js)
}

#[post("/users")]
async fn create_user(
    app_data: web::Data<AppState>,
    form: web::Form<CreateUserForm>,
) -> Result<impl Responder, Error> {
    // Process user creation
    app_data.create_user(&form).await?;
    Ok(HttpResponse::Ok().json({"status": "success"}))
}

#[delete("/users/{username}")]
async fn delete_user(
    app_data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let username = path.into_inner();
    app_data.delete_user(&username).await?;
    Ok(HttpResponse::Ok().json({"status": "success"}))
}

#[derive(Deserialize)]
struct CreateUserForm {
    username: String,
    password: String,
    groups: Vec<String>,
}

struct AppState {
    recovery_admin_enabled: bool,
    default_quota: String,
    quota_preset: Vec<String>,
    default_quota_is_user_defined: bool,
    enable_avatars: bool,
    subadmins: Option<Vec<String>>,
    current_user: String,
}

impl AppState {
    async fn get_groups(&self) -> Result<Vec<Group>, Error> {
        // This would interact with your database or services
        Ok(vec![
            Group { name: "admin".to_string() },
            Group { name: "users".to_string() },
        ])
    }
    
    async fn get_users(&self) -> Result<Vec<User>, Error> {
        // This would interact with your database or services
        Ok(vec![
            User {
                name: "admin".to_string(),
                display_name: "Administrator".to_string(),
                groups: vec!["admin".to_string()],
                subadmin: Some(vec!["admin".to_string()]),
                quota: "default".to_string(),
                is_quota_user_defined: false,
            }
        ])
    }
    
    fn is_admin(&self) -> bool {
        // Check if current user is admin
        true
    }
    
    async fn create_user(&self, form: &CreateUserForm) -> Result<(), Error> {
        // Create user implementation
        Ok(())
    }
    
    async fn delete_user(&self, username: &str) -> Result<(), Error> {
        // Delete user implementation
        Ok(())
    }
}

fn image_path(app: &str, path: &str) -> String {
    format!("/apps/{}/img/{}", app, path)
}

fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    
    HttpServer::new(|| {
        let app_state = web::Data::new(AppState {
            recovery_admin_enabled: true,
            default_quota: "none".to_string(),
            quota_preset: vec!["1 GB".to_string(), "5 GB".to_string(), "10 GB".to_string()],
            default_quota_is_user_defined: false,
            enable_avatars: true,
            subadmins: Some(vec![]),
            current_user: "admin".to_string(),
        });
        
        App::new()
            .app_data(app_state)
            .service(users_page)
            .service(is_admin_js)
            .service(create_user)
            .service(delete_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
}