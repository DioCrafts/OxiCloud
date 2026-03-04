use axum::Json;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use std::sync::Arc;

use crate::common::di::AppState;

pub async fn handle_status(State(state): State<Arc<AppState>>) -> Response {
    let (major, minor, patch) = state.core.config.nextcloud.emulated_version;
    let version_string = state.core.config.nextcloud.version_string();
    Json(json!({
        "installed": true,
        "maintenance": false,
        "needsDbUpgrade": false,
        "version": format!("{}.{}.{}.1", major, minor, patch),
        "versionstring": version_string,
        "productname": "OxiCloud",
        "edition": ""
    }))
    .into_response()
}
