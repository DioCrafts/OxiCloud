use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::collections::HashSet;
use thiserror::Error;

#[derive(Error, Debug)]
enum CreateUserError {
    #[error("User creation failed for {0}")]
    UserCreationFailed(String),
    
    #[error("{0}")]
    GenericError(String),
}

#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
    password: String,
    groups: Option<Vec<String>>,
}

#[derive(Serialize)]
struct CreateUserResponse {
    home_exists: bool,
    username: String,
    groups: Vec<String>,
}

#[derive(Serialize)]
struct SuccessResponse {
    data: CreateUserResponse,
}

#[derive(Serialize)]
struct ErrorResponse {
    data: ErrorData,
}

#[derive(Serialize)]
struct ErrorData {
    message: String,
}

async fn create_user(
    req: web::Json<CreateUserRequest>,
    identity: web::ResMut<Identity>,
    user_service: web::Data<UserService>,
    group_service: web::Data<GroupService>,
) -> Result<HttpResponse, Error> {
    // Check if user has permission
    json_call_check()?;
    check_sub_admin_user(&identity)?;
    
    let username = &req.username;
    let password = &req.password;
    
    // Determine which groups to assign
    let groups = if user_service.is_admin_user(identity.get_user()) {
        // Admin can assign to any groups
        req.groups.clone().unwrap_or_default()
    } else {
        // SubAdmin can only assign to accessible groups
        let mut accessible_groups = Vec::new();
        
        if let Some(requested_groups) = &req.groups {
            for group in requested_groups {
                if user_service.is_group_accessible(identity.get_user(), group) {
                    accessible_groups.push(group.clone());
                }
            }
            
            if accessible_groups.is_empty() {
                accessible_groups = user_service.get_sub_admins_groups(identity.get_user());
            }
        } else {
            accessible_groups = user_service.get_sub_admins_groups(identity.get_user());
        }
        
        accessible_groups
    };
    
    // Try to create the user
    match create_user_with_groups(username, password, &groups, &user_service, &group_service).await {
        Ok(response) => {
            Ok(HttpResponse::Ok().json(SuccessResponse { data: response }))
        },
        Err(err) => {
            let error_message = err.to_string();
            Ok(HttpResponse::BadRequest().json(ErrorResponse { 
                data: ErrorData { message: error_message } 
            }))
        }
    }
}

async fn create_user_with_groups(
    username: &str,
    password: &str,
    groups: &[String],
    user_service: &UserService,
    group_service: &GroupService,
) -> Result<CreateUserResponse, CreateUserError> {
    // Check whether the user's files home exists
    let user_directory = format!("{}/files/", user_service.get_home(username));
    let home_exists = Path::new(&user_directory).exists();
    
    // Create the user
    if !user_service.create_user(username, password).await {
        return Err(CreateUserError::UserCreationFailed(username.to_string()));
    }
    
    // Add user to groups
    for group in groups {
        if !group_service.group_exists(group).await {
            group_service.create_group(group).await?;
        }
        group_service.add_to_group(username, group).await?;
    }
    
    // Return success response
    Ok(CreateUserResponse {
        home_exists,
        username: username.to_string(),
        groups: group_service.get_user_groups(username).await,
    })
}

// These would need to be implemented based on your system's auth and permissions
fn json_call_check() -> Result<(), Error> {
    // Implement the equivalent of OCP\JSON::callCheck()
    Ok(())
}

fn check_sub_admin_user(identity: &Identity) -> Result<(), Error> {
    // Implement the equivalent of OC_JSON::checkSubAdminUser()
    Ok(())
}

// Service traits that would need to be implemented
trait Identity {
    fn get_user(&self) -> &str;
}

trait UserService {
    fn is_admin_user(&self, username: &str) -> bool;
    fn is_group_accessible(&self, username: &str, group: &str) -> bool;
    fn get_sub_admins_groups(&self, username: &str) -> Vec<String>;
    fn get_home(&self, username: &str) -> String;
    async fn create_user(&self, username: &str, password: &str) -> bool;
}

trait GroupService {
    async fn group_exists(&self, group: &str) -> bool;
    async fn create_group(&self, group: &str) -> Result<(), CreateUserError>;
    async fn add_to_group(&self, username: &str, group: &str) -> Result<(), CreateUserError>;
    async fn get_user_groups(&self, username: &str) -> Vec<String>;
}