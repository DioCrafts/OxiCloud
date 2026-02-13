use axum::{
    Router,
    routing::{get, put, post, delete},
    extract::{State, Json, Path, Query},
    http::{StatusCode, HeaderMap, header},
    response::IntoResponse,
};

use crate::common::di::AppState;
use crate::application::dtos::settings_dto::{
    SaveOidcSettingsDto, TestOidcConnectionDto,
    UpdateUserRoleDto, UpdateUserActiveDto, UpdateUserQuotaDto,
    ListUsersQueryDto, DashboardStatsDto,
    AdminCreateUserDto, AdminResetPasswordDto,
};
use crate::interfaces::errors::AppError;

/// Admin API routes — all require admin role.
pub fn admin_routes() -> Router<AppState> {
    Router::new()
        // OIDC settings
        .route("/settings/oidc", get(get_oidc_settings))
        .route("/settings/oidc", put(save_oidc_settings))
        .route("/settings/oidc/test", post(test_oidc_connection))
        .route("/settings/general", get(get_general_settings))
        // Dashboard / stats
        .route("/dashboard", get(get_dashboard_stats))
        // User management
        .route("/users", get(list_users))
        .route("/users", post(create_user))
        .route("/users/{id}", get(get_user))
        .route("/users/{id}", delete(delete_user))
        .route("/users/{id}/role", put(update_user_role))
        .route("/users/{id}/active", put(update_user_active))
        .route("/users/{id}/quota", put(update_user_quota))
        .route("/users/{id}/password", put(reset_user_password))
        // Registration control
        .route("/settings/registration", get(get_registration_setting))
        .route("/settings/registration", put(set_registration_setting))
}

/// Validate JWT and require admin role. Returns (user_id, role).
async fn admin_guard(state: &AppState, headers: &HeaderMap) -> Result<(String, String), AppError> {
    let auth = state.auth_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Auth service not configured"))?;

    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::unauthorized("Authorization token required"))?;

    let claims = auth.token_service.validate_token(token)
        .map_err(|e| AppError::unauthorized(&format!("Invalid token: {}", e)))?;

    if claims.role != "admin" {
        return Err(AppError::new(
            StatusCode::FORBIDDEN,
            "Admin access required",
            "Forbidden",
        ));
    }

    Ok((claims.sub, claims.role))
}

/// GET /api/admin/settings/oidc — get OIDC settings for the admin panel
async fn get_oidc_settings(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    admin_guard(&state, &headers).await?;

    let svc = state.admin_settings_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Admin settings service not available"))?;

    let settings = svc.get_oidc_settings().await
        .map_err(|e| AppError::internal_error(&format!("Failed to load settings: {}", e)))?;

    Ok(Json(settings))
}

/// PUT /api/admin/settings/oidc — save OIDC settings + hot-reload
async fn save_oidc_settings(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(dto): Json<SaveOidcSettingsDto>,
) -> Result<impl IntoResponse, AppError> {
    let (user_id, _) = admin_guard(&state, &headers).await?;

    let svc = state.admin_settings_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Admin settings service not available"))?;

    svc.save_oidc_settings(dto, &user_id).await
        .map_err(|e| AppError::internal_error(&format!("Failed to save settings: {}", e)))?;

    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "OIDC settings saved and applied successfully"
    }))))
}

/// POST /api/admin/settings/oidc/test — test OIDC discovery
async fn test_oidc_connection(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(dto): Json<TestOidcConnectionDto>,
) -> Result<impl IntoResponse, AppError> {
    admin_guard(&state, &headers).await?;

    let svc = state.admin_settings_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Admin settings service not available"))?;

    let result = svc.test_oidc_connection(dto).await
        .map_err(|e| AppError::internal_error(&format!("Connection test failed: {}", e)))?;

    Ok(Json(result))
}

/// GET /api/admin/settings/general — system overview (backward compat)
async fn get_general_settings(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    admin_guard(&state, &headers).await?;

    let auth = state.auth_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Auth service not configured"))?;

    let user_count = auth.auth_application_service.count_users_efficient().await.unwrap_or(0);
    let oidc_configured = auth.auth_application_service.oidc_enabled();

    Ok(Json(serde_json::json!({
        "server_version": env!("CARGO_PKG_VERSION"),
        "auth_enabled": true,
        "total_users": user_count,
        "oidc_configured": oidc_configured,
    })))
}

// ============================================================================
// Dashboard / Stats
// ============================================================================

/// GET /api/admin/dashboard — full dashboard statistics
async fn get_dashboard_stats(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    admin_guard(&state, &headers).await?;

    let auth = state.auth_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Auth service not configured"))?;

    let auth_app = &auth.auth_application_service;

    // Get storage stats from repository (single efficient query)
    let db_pool = state.db_pool.as_ref()
        .ok_or_else(|| AppError::internal_error("Database not available"))?;

    // Use direct SQL for aggregated stats — more efficient than loading all users
    let stats_row = sqlx::query(
        r#"
        SELECT
            COUNT(*)::INT8 as total_users,
            COUNT(*) FILTER (WHERE active = true)::INT8 as active_users,
            COUNT(*) FILTER (WHERE role::text = 'admin')::INT8 as admin_users,
            COALESCE(SUM(storage_quota_bytes)::INT8, 0) as total_quota_bytes,
            COALESCE(SUM(storage_used_bytes)::INT8, 0) as total_used_bytes,
            COUNT(*) FILTER (WHERE storage_quota_bytes > 0 AND storage_used_bytes > storage_quota_bytes * 0.8)::INT8 as users_over_80,
            COUNT(*) FILTER (WHERE storage_quota_bytes > 0 AND storage_used_bytes > storage_quota_bytes)::INT8 as users_over_quota
        FROM auth.users
        "#
    )
    .fetch_one(db_pool.as_ref())
    .await
    .map_err(|e| AppError::internal_error(&format!("Database query failed: {}", e)))?;

    use sqlx::Row;
    let total_quota: i64 = stats_row.get("total_quota_bytes");
    let total_used: i64 = stats_row.get("total_used_bytes");
    let usage_percent = if total_quota > 0 {
        (total_used as f64 / total_quota as f64) * 100.0
    } else {
        0.0
    };

    let stats = DashboardStatsDto {
        server_version: env!("CARGO_PKG_VERSION").to_string(),
        auth_enabled: true,
        oidc_configured: auth_app.oidc_enabled(),
        quotas_enabled: true, // Feature flag could be checked here
        total_users: stats_row.get("total_users"),
        active_users: stats_row.get("active_users"),
        admin_users: stats_row.get("admin_users"),
        total_quota_bytes: total_quota,
        total_used_bytes: total_used,
        storage_usage_percent: (usage_percent * 100.0).round() / 100.0,
        users_over_80_percent: stats_row.get("users_over_80"),
        users_over_quota: stats_row.get("users_over_quota"),
        registration_enabled: {
            if let Some(svc) = state.admin_settings_service.as_ref() {
                svc.get_registration_enabled().await
            } else {
                true // default: enabled
            }
        },
    };

    Ok(Json(stats))
}

// ============================================================================
// User Management
// ============================================================================

/// GET /api/admin/users?limit=50&offset=0 — list all users
async fn list_users(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<ListUsersQueryDto>,
) -> Result<impl IntoResponse, AppError> {
    admin_guard(&state, &headers).await?;

    let auth = state.auth_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Auth service not configured"))?;

    let limit = query.limit.unwrap_or(100).min(500);
    let offset = query.offset.unwrap_or(0);

    let users = auth.auth_application_service.list_users(limit, offset).await
        .map_err(|e| AppError::internal_error(&format!("Failed to list users: {}", e)))?;

    let total = auth.auth_application_service.count_users_efficient().await.unwrap_or(0);

    Ok(Json(serde_json::json!({
        "users": users,
        "total": total,
        "limit": limit,
        "offset": offset,
    })))
}

/// GET /api/admin/users/:id — get single user
async fn get_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    admin_guard(&state, &headers).await?;

    let auth = state.auth_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Auth service not configured"))?;

    let user = auth.auth_application_service.get_user_admin(&id).await
        .map_err(|e| AppError::not_found(&format!("User not found: {}", e)))?;

    Ok(Json(user))
}

/// DELETE /api/admin/users/:id — delete a user
async fn delete_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let (admin_id, _) = admin_guard(&state, &headers).await?;

    // Prevent self-deletion
    if admin_id == id {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Cannot delete your own account",
            "SelfDeletion",
        ));
    }

    let auth = state.auth_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Auth service not configured"))?;

    auth.auth_application_service.delete_user_admin(&id).await
        .map_err(|e| AppError::internal_error(&format!("Failed to delete user: {}", e)))?;

    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "User deleted successfully"
    }))))
}

/// PUT /api/admin/users/:id/role — change user role
async fn update_user_role(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(dto): Json<UpdateUserRoleDto>,
) -> Result<impl IntoResponse, AppError> {
    let (admin_id, _) = admin_guard(&state, &headers).await?;

    // Prevent changing own role
    if admin_id == id {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Cannot change your own role",
            "SelfRoleChange",
        ));
    }

    let auth = state.auth_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Auth service not configured"))?;

    auth.auth_application_service.change_user_role(&id, &dto.role).await
        .map_err(|e| AppError::internal_error(&format!("Failed to change role: {}", e)))?;

    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": format!("User role updated to '{}'", dto.role)
    }))))
}

/// PUT /api/admin/users/:id/active — activate/deactivate user
async fn update_user_active(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(dto): Json<UpdateUserActiveDto>,
) -> Result<impl IntoResponse, AppError> {
    let (admin_id, _) = admin_guard(&state, &headers).await?;

    // Prevent deactivating yourself
    if admin_id == id && !dto.active {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Cannot deactivate your own account",
            "SelfDeactivation",
        ));
    }

    let auth = state.auth_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Auth service not configured"))?;

    auth.auth_application_service.set_user_active(&id, dto.active).await
        .map_err(|e| AppError::internal_error(&format!("Failed to update user status: {}", e)))?;

    let status = if dto.active { "activated" } else { "deactivated" };
    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": format!("User {}", status)
    }))))
}

/// PUT /api/admin/users/:id/quota — update user storage quota
async fn update_user_quota(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(dto): Json<UpdateUserQuotaDto>,
) -> Result<impl IntoResponse, AppError> {
    admin_guard(&state, &headers).await?;

    let auth = state.auth_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Auth service not configured"))?;

    auth.auth_application_service.update_user_quota(&id, dto.quota_bytes).await
        .map_err(|e| AppError::internal_error(&format!("Failed to update quota: {}", e)))?;

    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "User quota updated",
        "quota_bytes": dto.quota_bytes,
    }))))
}

// ============================================================================
// Admin User Creation & Password Reset
// ============================================================================

/// POST /api/admin/users — create a new user (admin only)
async fn create_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(dto): Json<AdminCreateUserDto>,
) -> Result<impl IntoResponse, AppError> {
    admin_guard(&state, &headers).await?;

    let auth = state.auth_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Auth service not configured"))?;

    let user = auth.auth_application_service.admin_create_user(dto).await
        .map_err(|e| AppError::new(
            StatusCode::BAD_REQUEST,
            &format!("Failed to create user: {}", e),
            "CreateUserFailed",
        ))?;

    Ok((StatusCode::CREATED, Json(user)))
}

/// PUT /api/admin/users/:id/password — reset a user's password (admin only)
async fn reset_user_password(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(dto): Json<AdminResetPasswordDto>,
) -> Result<impl IntoResponse, AppError> {
    admin_guard(&state, &headers).await?;

    let auth = state.auth_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Auth service not configured"))?;

    auth.auth_application_service.admin_reset_password(&id, &dto.new_password).await
        .map_err(|e| AppError::new(
            StatusCode::BAD_REQUEST,
            &format!("Failed to reset password: {}", e),
            "ResetPasswordFailed",
        ))?;

    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "Password reset successfully"
    }))))
}

// ============================================================================
// Registration Control
// ============================================================================

/// GET /api/admin/settings/registration — check if public registration is enabled
async fn get_registration_setting(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    admin_guard(&state, &headers).await?;

    let svc = state.admin_settings_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Admin settings service not available"))?;

    let val = svc.get_registration_enabled().await;

    Ok(Json(serde_json::json!({
        "registration_enabled": val,
    })))
}

/// PUT /api/admin/settings/registration — enable/disable public registration
async fn set_registration_setting(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let (admin_id, _) = admin_guard(&state, &headers).await?;

    let enabled = body.get("registration_enabled")
        .and_then(|v| v.as_bool())
        .ok_or_else(|| AppError::new(
            StatusCode::BAD_REQUEST,
            "Missing boolean field 'registration_enabled'",
            "InvalidInput",
        ))?;

    let svc = state.admin_settings_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Admin settings service not available"))?;

    svc.set_registration_enabled(enabled, &admin_id).await
        .map_err(|e| AppError::internal_error(&format!("Failed to save setting: {}", e)))?;

    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": format!("Public registration {}", if enabled { "enabled" } else { "disabled" }),
        "registration_enabled": enabled,
    }))))
}
