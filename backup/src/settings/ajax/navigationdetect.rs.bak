use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct NavigationDetectParams {
    app: String,
}

#[derive(Serialize)]
struct NavigationResponse {
    nav_ids: Vec<String>,
    nav_entries: Vec<HashMap<String, serde_json::Value>>,
}

/// Handler for navigation detection endpoint
pub async fn navigation_detect(
    query: web::Query<NavigationDetectParams>,
    auth_service: web::Data<dyn AuthService>,
    app_service: web::Data<dyn AppService>,
) -> impl Responder {
    // Check if user is admin
    if !auth_service.is_admin_user() {
        return HttpResponse::Forbidden().finish();
    }

    // Verify CSRF token
    if !auth_service.verify_request_token() {
        return HttpResponse::Forbidden().finish();
    }

    // Clean app ID
    let app = app_service.clean_app_id(&query.app);
    
    // Get navigation entries
    let navigation = match app_service.get_app_navigation_entries(&app).await {
        Ok(nav) => nav,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Extract navigation IDs
    let nav_ids: Vec<String> = navigation
        .iter()
        .filter_map(|nav| nav.get("id").and_then(|id| id.as_str().map(String::from)))
        .collect();

    // Build response
    let response = NavigationResponse {
        nav_ids,
        nav_entries: navigation,
    };

    HttpResponse::Ok().json(response)
}

// Trait for authentication service
pub trait AuthService: Send + Sync {
    fn is_admin_user(&self) -> bool;
    fn verify_request_token(&self) -> bool;
}

// Trait for app service
pub trait AppService: Send + Sync {
    fn clean_app_id(&self, app_id: &str) -> String;
    async fn get_app_navigation_entries(&self, app_id: &str) -> Result<Vec<HashMap<String, serde_json::Value>>, Box<dyn std::error::Error>>;
}