use axum::Json;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::sync::Arc;

use crate::application::dtos::search_dto::SearchCriteriaDto;
use crate::application::ports::inbound::SearchUseCase;
use crate::application::ports::storage_ports::StorageUsagePort;
use crate::common::di::AppState;
use crate::interfaces::middleware::auth::CurrentUser;

/// Build an OCS success response with the given statuscode and data.
fn ocs_ok(statuscode: u16, data: serde_json::Value) -> serde_json::Value {
    json!({
        "ocs": {
            "meta": { "status": "ok", "statuscode": statuscode, "message": "OK" },
            "data": data,
        }
    })
}

/// Build an OCS error response.
fn ocs_err(statuscode: u16, message: &str) -> serde_json::Value {
    json!({
        "ocs": {
            "meta": { "status": "failure", "statuscode": statuscode, "message": message },
            "data": {},
        }
    })
}

pub async fn handle_capabilities_v1(State(state): State<Arc<AppState>>) -> Response {
    let payload = capabilities_payload(&state, 1);
    tracing::info!("[NC] capabilities v1 requested, returning payload");
    Json(payload).into_response()
}

pub async fn handle_capabilities_v2(State(state): State<Arc<AppState>>) -> Response {
    let payload = capabilities_payload(&state, 2);
    tracing::info!("[NC] capabilities v2 requested, returning payload");
    Json(payload).into_response()
}

pub async fn handle_user_info(State(state): State<Arc<AppState>>, user: CurrentUser) -> Response {
    let quota: (i64, i64) = match state.storage_usage_service.as_ref() {
        Some(service) => match service.get_user_storage_info(user.id).await {
            Ok((used, total)) => (used, total),
            Err(_) => (0, 0),
        },
        None => (0, 0),
    };

    let free = quota.1.saturating_sub(quota.0);
    let relative = if quota.1 > 0 {
        (quota.0 as f64 / quota.1 as f64) * 100.0
    } else {
        0.0
    };

    Json(json!({
        "ocs": {
            "meta": { "status": "ok", "statuscode": 200, "message": "OK" },
            "data": {
                "enabled": true,
                "id": user.username,
                "display-name": user.username,
                "displayname": user.username,
                "email": user.email,
                "quota": {
                    "used": quota.0,
                    "total": quota.1,
                    "free": free,
                    "relative": relative
                }
            }
        }
    }))
    .into_response()
}

/// GET /ocs/v1.php/cloud/users/{userid}
pub async fn handle_user_provisioning_v1(
    state: State<Arc<AppState>>,
    path: Path<String>,
    user: CurrentUser,
) -> Response {
    user_provisioning_response(state, path, user, 1).await
}

/// GET /ocs/v2.php/cloud/users/{userid}
pub async fn handle_user_provisioning_v2(
    state: State<Arc<AppState>>,
    path: Path<String>,
    user: CurrentUser,
) -> Response {
    user_provisioning_response(state, path, user, 2).await
}

/// Returns user details in Nextcloud OCS provisioning API format.
/// Used by the Nextcloud mobile app to fetch the user profile screen.
async fn user_provisioning_response(
    State(state): State<Arc<AppState>>,
    Path(userid): Path<String>,
    user: CurrentUser,
    ocs_version: u8,
) -> Response {
    let statuscode = if ocs_version == 1 { 100 } else { 200 };

    // Only allow users to view their own profile, unless they are admin.
    if user.username != userid && user.role != "admin" {
        return Json(ocs_err(403, "Insufficient privileges")).into_response();
    }

    let auth_service = match state.auth_service.as_ref() {
        Some(svc) => &svc.auth_application_service,
        None => {
            return Json(ocs_err(997, "Authentication not configured")).into_response();
        }
    };

    let user_dto = match auth_service.get_user_by_username(&userid).await {
        Ok(u) => u,
        Err(_) => {
            return Json(ocs_err(404, "User not found")).into_response();
        }
    };

    // Determine groups based on role
    let groups = if user_dto.role == "admin" {
        vec!["admin", "users"]
    } else {
        vec!["users"]
    };

    // Determine backend based on auth provider
    let backend = if user_dto.auth_provider.to_lowercase().contains("oidc") {
        "OIDC"
    } else {
        "Database"
    };

    // Convert last_login_at to JS milliseconds
    let last_login = user_dto
        .last_login_at
        .map(|dt| dt.timestamp() * 1000)
        .unwrap_or(0);

    // Fetch quota from storage usage service
    let quota: (i64, i64) = match state.storage_usage_service.as_ref() {
        Some(service) => match service.get_user_storage_info(uuid::Uuid::parse_str(&user_dto.id).unwrap_or_default()).await {
            Ok((used, total)) => (used, total),
            Err(_) => (0, 0),
        },
        None => (0, 0),
    };

    let free = quota.1.saturating_sub(quota.0);
    let relative = if quota.1 > 0 {
        (quota.0 as f64 / quota.1 as f64) * 100.0
    } else {
        0.0
    };

    Json(json!({
        "ocs": {
            "meta": { "status": "ok", "statuscode": statuscode, "message": "OK" },
            "data": {
                "enabled": user_dto.active,
                "id": user_dto.username,
                "display-name": user_dto.username,
                "displayname": user_dto.username,
                "email": user_dto.email,
                "phone": "",
                "address": "",
                "website": "",
                "twitter": "",
                "groups": groups,
                "language": "en",
                "locale": "en_US",
                "backend": backend,
                "lastLogin": last_login,
                "quota": {
                    "used": quota.0,
                    "total": quota.1,
                    "free": free,
                    "relative": relative
                }
            }
        }
    }))
    .into_response()
}

pub async fn handle_revoke_apppassword(
    State(state): State<Arc<AppState>>,
    user: CurrentUser,
    headers: axum::http::HeaderMap,
) -> Response {
    let nextcloud = match state.nextcloud.as_ref() {
        Some(nextcloud) => nextcloud,
        None => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };

    let app_password = match extract_basic_password(&headers) {
        Some(password) => password,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    if let Err(e) = nextcloud
        .app_passwords
        .revoke_by_password(user.id, &app_password)
        .await
    {
        tracing::warn!("Failed to revoke app password for {}: {}", user.id, e);
    }

    Json(ocs_ok(200, json!({}))).into_response()
}

pub async fn handle_notifications_list() -> Response {
    Json(ocs_ok(200, json!([]))).into_response()
}

pub async fn handle_notifications_push() -> Response {
    Json(ocs_ok(200, json!({}))).into_response()
}

/// GET /ocs/v2.php/apps/recommendations/api/v1/recommendations
///
/// Returns recommended files. Stub that returns an empty list.
pub async fn handle_recommendations() -> Response {
    Json(json!({
        "ocs": {
            "meta": { "status": "ok", "statuscode": 200, "message": "OK" },
            "data": []
        }
    }))
    .into_response()
}

/// GET /ocs/v2.php/apps/files_sharing/api/v1/sharees?search={query}&itemType={type}
///
/// Returns matching users for the sharing autocomplete UI.
/// Even though sharing is disabled, the Nextcloud mobile app still calls
/// this endpoint and expects a well-formed OCS response rather than a 404.
pub async fn handle_sharees_search(
    State(state): State<Arc<AppState>>,
    user: CurrentUser,
    axum::extract::Query(params): axum::extract::Query<ShareeSearchParams>,
) -> Response {
    let search = params.search.unwrap_or_default();
    if search.is_empty() {
        return sharees_response(vec![]).into_response();
    }

    let auth_service = match state.auth_service.as_ref() {
        Some(svc) => &svc.auth_application_service,
        None => return sharees_response(vec![]).into_response(),
    };

    // SQL-level ILIKE search with limit — avoids loading all users into memory.
    let users = auth_service
        .search_users(&search, 26)
        .await
        .unwrap_or_default();

    let matches: Vec<serde_json::Value> = users
        .into_iter()
        .filter(|u| u.username != user.username) // Don't suggest self
        .take(25)
        .map(|u| {
            json!({
                "label": u.username,
                "value": {
                    "shareType": 0,
                    "shareWith": u.username
                }
            })
        })
        .collect();

    sharees_response(matches).into_response()
}

#[derive(serde::Deserialize)]
pub struct ShareeSearchParams {
    search: Option<String>,
    #[serde(rename = "itemType")]
    #[allow(dead_code)]
    item_type: Option<String>,
    #[serde(rename = "perPage")]
    #[allow(dead_code)]
    per_page: Option<u32>,
}

fn sharees_response(users: Vec<serde_json::Value>) -> Json<serde_json::Value> {
    Json(json!({
        "ocs": {
            "meta": { "status": "ok", "statuscode": 200, "message": "OK" },
            "data": {
                "exact": { "users": [], "groups": [], "remotes": [] },
                "users": users,
                "groups": [],
                "remotes": []
            }
        }
    }))
}

/// GET /ocs/v2.php/search/providers
///
/// Returns the list of available Unified Search providers.
/// We only expose the "files" provider.
pub async fn handle_search_providers() -> Response {
    Json(json!({
        "ocs": {
            "meta": { "status": "ok", "statuscode": 200, "message": "OK" },
            "data": [
                {
                    "id": "files",
                    "appId": "files",
                    "name": "Files",
                    "icon": "/apps/files/img/app.svg",
                    "order": 5,
                    "filters": {},
                    "isPaginated": false
                }
            ]
        }
    }))
    .into_response()
}

/// GET /ocs/v2.php/search/providers/{provider_id}/search?term=…&limit=…&cursor=…
///
/// Executes a Unified Search query against the given provider.
/// Only the "files" provider is implemented; all others return empty results.
pub async fn handle_search(
    State(state): State<Arc<AppState>>,
    Path(provider_id): Path<String>,
    axum::extract::Query(params): axum::extract::Query<UnifiedSearchParams>,
    user: CurrentUser,
) -> Response {
    // Only the "files" provider is supported
    if provider_id != "files" {
        return empty_search_response().into_response();
    }

    let search_service = match state.applications.search_service.as_ref() {
        Some(svc) => svc,
        None => return empty_search_response().into_response(),
    };

    let term = params.term.unwrap_or_default();
    if term.is_empty() {
        return empty_search_response().into_response();
    }

    let criteria = SearchCriteriaDto {
        name_contains: Some(term),
        recursive: true,
        limit: params.limit.unwrap_or(25),
        ..SearchCriteriaDto::default()
    };

    let results = match search_service.search(criteria, user.id).await {
        Ok(r) => r,
        Err(_) => return empty_search_response().into_response(),
    };

    let file_id_svc = state.nextcloud.as_ref().map(|n| &n.file_ids);

    let mut entries: Vec<serde_json::Value> = Vec::new();

    // Map file results
    for file in &results.files {
        let display_path = file
            .path
            .strip_prefix(&format!("My Folder - {}/", user.username))
            .unwrap_or(&file.path);
        let display_path = format!("/{}", display_path);

        let numeric_id = if let Some(svc) = file_id_svc {
            svc.get_or_create_file_id(&file.id).await.ok()
        } else {
            None
        };

        let thumbnail_url = match numeric_id {
            Some(nid) => format!("/index.php/core/preview?fileId={}&x=32&y=32", nid),
            None => String::new(),
        };
        let resource_url = match numeric_id {
            Some(nid) => format!("/f/{}", nid),
            None => String::new(),
        };

        entries.push(json!({
            "thumbnailUrl": thumbnail_url,
            "title": file.name,
            "subline": display_path,
            "resourceUrl": resource_url,
            "icon": "",
            "rounded": false
        }));
    }

    // Map folder results
    for folder in &results.folders {
        let display_path = folder
            .path
            .strip_prefix(&format!("My Folder - {}/", user.username))
            .unwrap_or(&folder.path);
        let display_path = format!("/{}", display_path);

        entries.push(json!({
            "thumbnailUrl": "",
            "title": folder.name,
            "subline": display_path,
            "resourceUrl": "",
            "icon": "/apps/files/img/folder.svg",
            "rounded": false
        }));
    }

    Json(json!({
        "ocs": {
            "meta": { "status": "ok", "statuscode": 200, "message": "OK" },
            "data": {
                "name": "Files",
                "isPaginated": false,
                "entries": entries,
                "cursor": null
            }
        }
    }))
    .into_response()
}

#[derive(serde::Deserialize)]
pub struct UnifiedSearchParams {
    term: Option<String>,
    limit: Option<usize>,
    #[allow(dead_code)]
    cursor: Option<String>,
}

fn empty_search_response() -> Json<serde_json::Value> {
    Json(json!({
        "ocs": {
            "meta": { "status": "ok", "statuscode": 200, "message": "OK" },
            "data": {
                "name": "Files",
                "isPaginated": false,
                "entries": [],
                "cursor": null
            }
        }
    }))
}

fn capabilities_payload(state: &AppState, ocs_version: u8) -> serde_json::Value {
    let statuscode = if ocs_version == 1 { 100 } else { 200 };
    let base_url = state.core.config.base_url();
    let (nc_major, nc_minor, nc_micro) = state.core.config.nextcloud.emulated_version;
    let nc_version_str = state.core.config.nextcloud.version_string();

    json!({
        "ocs": {
            "meta": {
                "status": "ok",
                "statuscode": statuscode,
                "message": "OK"
            },
            "data": {
                "version": {
                    "major": nc_major,
                    "minor": nc_minor,
                    "micro": nc_micro,
                    "string": nc_version_str,
                    "edition": "",
                    "extendedSupport": false
                },
                "capabilities": {
                    "core": {
                        "pollinterval": 60,
                        "webdav-root": "remote.php/dav",
                        "reference-api": false,
                        "reference-regex": ""
                    },
                    "files": {
                        "bigfilechunking": true,
                        "favorites": true,
                        "undelete": true,
                        "versioning": false
                    },
                    "dav": {
                        "chunking": "1.0"
                    },
                    "checksums": {
                        "preferredUploadType": "SHA1",
                        "supportedTypes": ["SHA1", "MD5"]
                    },
                    "files_sharing": {
                        "api_enabled": false,
                        "public": { "enabled": false },
                        "user": { "send_mail": false },
                        "resharing": false
                    },
                    "notifications": {
                        "ocs-endpoints": ["list", "get", "delete", "delete-all"]
                    },
                    "theming": {
                        "name": "OxiCloud",
                        "url": base_url,
                        "logo": format!("{}/logo.png", base_url),
                        "color": "#0082c9",
                        "color-text": "#ffffff",
                        "color-element": "#0082c9",
                        "color-element-bright": "#0082c9",
                        "color-element-dark": "#0082c9",
                        "background": "#0082c9",
                        "background-plain": true,
                        "background-default": true,
                        "logoheader": format!("{}/logo.png", base_url),
                        "favicon": format!("{}/favicon.ico", base_url)
                    }
                }
            }
        }
    })
}

fn extract_basic_password(headers: &axum::http::HeaderMap) -> Option<String> {
    let value = headers
        .get(axum::http::header::AUTHORIZATION)?
        .to_str()
        .ok()?;
    super::basic_auth_middleware::parse_basic_auth(value).map(|(_, pass)| pass)
}
