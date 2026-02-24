use axum::{
    Json,
    body::Body,
    extract::{Path, Query, State},
    http::{HeaderMap, Response, StatusCode, header},
    response::IntoResponse,
};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio_util::io::ReaderStream;

use crate::application::dtos::folder_dto::{CreateFolderDto, MoveFolderDto, RenameFolderDto};
use crate::application::dtos::folder_listing_dto::FolderListingDto;
use crate::application::dtos::pagination::PaginationRequestDto;
use crate::application::ports::inbound::FolderUseCase;
use crate::application::services::folder_service::FolderService;
use crate::common::di::AppState as GlobalAppState;
use crate::common::errors::ErrorKind;
use crate::interfaces::middleware::auth::AuthUser;

type AppState = Arc<FolderService>;

/// Handler for folder-related API endpoints
pub struct FolderHandler;

impl FolderHandler {
    /// Creates a new folder.
    /// When parent_id is not provided, the folder is created inside the
    /// authenticated user's home folder rather than at the storage root.
    pub async fn create_folder(
        State(service): State<AppState>,
        auth_user: AuthUser,
        Json(mut dto): Json<CreateFolderDto>,
    ) -> impl IntoResponse {
        // If no parent_id was supplied, resolve the user's home folder as
        // the default parent so the new folder is nested correctly.
        if dto.parent_id.is_none() {
            tracing::info!(
                "create_folder: parent_id is None for user '{}', resolving home folder",
                auth_user.username
            );
            match service.list_folders_for_owner(None, &auth_user.id).await {
                Ok(folders) => {
                    if let Some(home) = folders.first() {
                        tracing::info!(
                            "create_folder: resolved home folder ID '{}' for user '{}'",
                            home.id,
                            auth_user.username
                        );
                        dto.parent_id = Some(home.id.clone());
                    } else {
                        tracing::warn!(
                            "create_folder: home folder not found for user '{}', folder will be created at root",
                            auth_user.username
                        );
                    }
                }
                Err(e) => {
                    tracing::error!(
                        "create_folder: failed to list folders for home resolution: {}",
                        e
                    );
                }
            }
        }

        match service.create_folder(dto).await {
            Ok(folder) => (StatusCode::CREATED, Json(folder)).into_response(),
            Err(err) => {
                let status = match err.kind {
                    ErrorKind::AlreadyExists => StatusCode::CONFLICT,
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };

                (status, err.to_string()).into_response()
            }
        }
    }

    /// Gets a folder by ID.
    /// Validates that the authenticated user owns the folder.
    pub async fn get_folder(
        State(service): State<AppState>,
        auth_user: AuthUser,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        match service.get_folder(&id).await {
            Ok(folder) => {
                // Access check: folder must belong to the requesting user
                if let Some(ref owner) = folder.owner_id {
                    if owner != &auth_user.id {
                        tracing::warn!(
                            "get_folder: user '{}' attempted to access folder '{}' owned by '{}'",
                            auth_user.id,
                            id,
                            owner
                        );
                        return (StatusCode::NOT_FOUND, "Folder not found".to_string())
                            .into_response();
                    }
                }
                (StatusCode::OK, Json(folder)).into_response()
            }
            Err(err) => {
                let status = match err.kind {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };

                (status, err.to_string()).into_response()
            }
        }
    }

    /// Lists root folders for the authenticated user.
    /// Only returns folders owned by this user — no information disclosure.
    pub async fn list_root_folders(
        State(service): State<AppState>,
        auth_user: AuthUser,
    ) -> axum::response::Response {
        Self::list_folders_scoped(service, None, &auth_user).await
    }

    /// Lists contents of a specific folder by its ID.
    /// Scoped to the authenticated user's folders.
    pub async fn list_folder_contents(
        State(service): State<AppState>,
        auth_user: AuthUser,
        Path(id): Path<String>,
    ) -> axum::response::Response {
        Self::list_folders_scoped(service, Some(&id), &auth_user).await
    }

    /// Lists root folders with pagination support.
    pub async fn list_root_folders_paginated(
        State(service): State<AppState>,
        auth_user: AuthUser,
        _pagination: Query<PaginationRequestDto>,
    ) -> axum::response::Response {
        Self::list_folders_scoped(service, None, &auth_user).await
    }

    /// Lists contents of a specific folder with pagination.
    /// Scoped to the authenticated user — only returns folders owned by this user.
    pub async fn list_folder_contents_paginated(
        State(service): State<AppState>,
        auth_user: AuthUser,
        Path(id): Path<String>,
        pagination: Query<PaginationRequestDto>,
    ) -> axum::response::Response {
        match service
            .list_folders_for_owner_paginated(Some(&id), &auth_user.id, &pagination)
            .await
        {
            Ok(paginated_result) => (StatusCode::OK, Json(paginated_result)).into_response(),
            Err(err) => {
                let status = match err.kind {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                (
                    status,
                    Json(serde_json::json!({ "error": err.to_string() })),
                )
                    .into_response()
            }
        }
    }

    /// Internal helper: lists folders scoped to the authenticated user.
    /// Uses `list_folders_for_owner` — the DB query filters by `user_id`,
    /// so no data from other users ever leaves the database.
    async fn list_folders_scoped(
        service: AppState,
        parent_id: Option<&str>,
        auth_user: &AuthUser,
    ) -> axum::response::Response {
        match service
            .list_folders_for_owner(parent_id, &auth_user.id)
            .await
        {
            Ok(folders) => (StatusCode::OK, Json(folders)).into_response(),
            Err(err) => {
                let status = match err.kind {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                (
                    status,
                    Json(serde_json::json!({ "error": err.to_string() })),
                )
                    .into_response()
            }
        }
    }

    /// Compute a lightweight ETag from the maximum `modified_at` timestamp
    /// and item count. No body buffering required.
    fn compute_listing_etag(folders: &[crate::application::dtos::folder_dto::FolderDto], files: &[crate::application::dtos::file_dto::FileDto]) -> String {
        let max_mod = folders.iter().map(|f| f.modified_at)
            .chain(files.iter().map(|f| f.modified_at))
            .max()
            .unwrap_or(0);
        let count = folders.len() + files.len();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        max_mod.hash(&mut hasher);
        count.hash(&mut hasher);
        format!("\"{:x}\"", hasher.finish())
    }

    /// Returns both sub-folders and files for a given folder in a single
    /// response, eliminating the double-fetch the frontend used to make.
    ///
    /// Both queries run concurrently via `tokio::join!`.
    /// Supports `If-None-Match` / ETag for conditional responses (304).
    pub async fn list_folder_listing(
        State(state): State<Arc<GlobalAppState>>,
        auth_user: AuthUser,
        headers: HeaderMap,
        Path(id): Path<String>,
    ) -> axum::response::Response {
        let folder_service = &state.applications.folder_service;
        let file_service = &state.applications.file_retrieval_service;

        // Run both queries concurrently — no sequential wait.
        let (folders_result, files_result) = tokio::join!(
            folder_service.list_folders_for_owner(Some(&id), &auth_user.id),
            file_service.list_files(Some(&id))
        );

        match (folders_result, files_result) {
            (Ok(folders), Ok(files)) => {
                let etag = Self::compute_listing_etag(&folders, &files);

                // 304 Not Modified if the client already has this version
                if let Some(inm) = headers.get(header::IF_NONE_MATCH)
                    && let Ok(client_etag) = inm.to_str()
                    && client_etag == etag
                {
                    return Response::builder()
                        .status(StatusCode::NOT_MODIFIED)
                        .header(header::ETAG, &etag)
                        .body(Body::empty())
                        .unwrap()
                        .into_response();
                }

                let listing = FolderListingDto { folders, files };
                let mut resp = (StatusCode::OK, Json(listing)).into_response();
                resp.headers_mut().insert(
                    header::ETAG,
                    header::HeaderValue::from_str(&etag).unwrap(),
                );
                resp
            }
            (Err(err), _) | (_, Err(err)) => {
                let status = match err.kind {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                (
                    status,
                    Json(serde_json::json!({ "error": err.to_string() })),
                )
                    .into_response()
            }
        }
    }

    /// Renames a folder (ownership enforced by service layer)
    pub async fn rename_folder(
        State(service): State<AppState>,
        auth_user: AuthUser,
        Path(id): Path<String>,
        Json(dto): Json<RenameFolderDto>,
    ) -> impl IntoResponse {
        match service.rename_folder(&id, dto, &auth_user.id).await {
            Ok(folder) => (StatusCode::OK, Json(folder)).into_response(),
            Err(err) => {
                let status = match err.kind {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    ErrorKind::AlreadyExists => StatusCode::CONFLICT,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };

                // Return a proper JSON error response
                (
                    status,
                    Json(serde_json::json!({
                        "error": err.to_string()
                    })),
                )
                    .into_response()
            }
        }
    }

    /// Moves a folder to a new parent (ownership enforced by service layer)
    pub async fn move_folder(
        State(service): State<AppState>,
        auth_user: AuthUser,
        Path(id): Path<String>,
        Json(dto): Json<MoveFolderDto>,
    ) -> impl IntoResponse {
        match service.move_folder(&id, dto, &auth_user.id).await {
            Ok(folder) => (StatusCode::OK, Json(folder)).into_response(),
            Err(err) => {
                let status = match err.kind {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    ErrorKind::AlreadyExists => StatusCode::CONFLICT,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };

                (status, err.to_string()).into_response()
            }
        }
    }

    /// Deletes a folder (ownership enforced by service layer)
    pub async fn delete_folder(
        State(service): State<AppState>,
        auth_user: AuthUser,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        match service.delete_folder(&id, &auth_user.id).await {
            Ok(_) => StatusCode::NO_CONTENT.into_response(),
            Err(err) => {
                let status = match err.kind {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };

                (status, err.to_string()).into_response()
            }
        }
    }

    /// Deletes a folder with trash functionality (ownership enforced by service layer)
    pub async fn delete_folder_with_trash(
        State(state): State<Arc<GlobalAppState>>,
        auth_user: AuthUser,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        let user_id = &auth_user.id;
        // Check if trash service is available
        if let Some(trash_service) = &state.trash_service {
            tracing::info!("Moving folder to trash: {}", id);

            // Try to move to trash first
            match trash_service.move_to_trash(&id, "folder", user_id).await {
                Ok(_) => {
                    tracing::info!("Folder successfully moved to trash: {}", id);
                    return StatusCode::NO_CONTENT.into_response();
                }
                Err(err) => {
                    tracing::warn!(
                        "Could not move folder to trash, falling back to permanent delete: {}",
                        err
                    );
                    // Fall through to regular delete if trash fails
                }
            }
        }

        // Fallback to permanent delete if trash is unavailable or failed
        let folder_service = &state.applications.folder_service;
        match folder_service.delete_folder(&id, user_id).await {
            Ok(_) => {
                tracing::info!("Folder permanently deleted: {}", id);
                StatusCode::NO_CONTENT.into_response()
            }
            Err(err) => {
                tracing::error!("Error deleting folder: {}", err);

                let status = match err.kind {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };

                (
                    status,
                    Json(serde_json::json!({
                        "error": format!("Error deleting folder: {}", err)
                    })),
                )
                    .into_response()
            }
        }
    }

    /// Downloads a folder as a ZIP file (ownership enforced)
    pub async fn download_folder_zip(
        State(state): State<Arc<GlobalAppState>>,
        auth_user: AuthUser,
        Path(id): Path<String>,
        Query(_params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
        tracing::info!("Downloading folder as ZIP: {}", id);

        // Get folder information and verify ownership
        let folder_service = &state.applications.folder_service;

        match folder_service.get_folder(&id).await {
            Ok(folder) => {
                // Access check: folder must belong to the requesting user
                if folder.owner_id.as_deref() != Some(&auth_user.id) {
                    tracing::warn!(
                        "download_folder_zip: user '{}' attempted to download folder '{}' owned by '{:?}'",
                        auth_user.id,
                        id,
                        folder.owner_id
                    );
                    return (
                        StatusCode::NOT_FOUND,
                        Json(serde_json::json!({ "error": "Folder not found" })),
                    )
                        .into_response();
                }
                tracing::info!("Preparing ZIP for folder: {} ({})", folder.name, id);

                // Use ZIP service from DI container
                let zip_service = &state.core.zip_service;

                // Create the ZIP archive (written to a temp file, O(1) RAM)
                match zip_service.create_folder_zip(&id, &folder.name).await {
                    Ok(temp_file) => {
                        // Get the file size for Content-Length
                        let file_size = match temp_file.as_file().metadata() {
                            Ok(m) => m.len(),
                            Err(e) => {
                                tracing::error!("Error reading temp file metadata: {}", e);
                                return (
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    Json(serde_json::json!({
                                        "error": "Error creating ZIP file"
                                    })),
                                )
                                    .into_response();
                            }
                        };

                        tracing::info!(
                            "ZIP file created successfully, size: {} bytes",
                            file_size
                        );

                        // Split the NamedTempFile into the already-open std File
                        // and the TempPath (auto-deletes on drop).  This reuses
                        // the existing fd instead of opening a second one.
                        let (std_file, temp_path) = temp_file.into_parts();
                        let tokio_file = tokio::fs::File::from_std(std_file);

                        // Stream the file to the client in chunks
                        let stream = ReaderStream::new(tokio_file);
                        let body = axum::body::Body::from_stream(stream);

                        // Setup headers for download
                        let filename = format!("{}.zip", folder.name);
                        let content_disposition = format!("attachment; filename=\"{}\"", filename);

                        let mut response = Response::builder()
                            .status(StatusCode::OK)
                            .header(header::CONTENT_TYPE, "application/zip")
                            .header(header::CONTENT_DISPOSITION, content_disposition)
                            .header(header::CONTENT_LENGTH, file_size)
                            .body(body)
                            .unwrap();

                        // Keep TempPath alive in the response extensions so the
                        // file is only deleted AFTER the body stream finishes.
                        response.extensions_mut().insert(Arc::new(temp_path));

                        response.into_response()
                    }
                    Err(err) => {
                        tracing::error!("Error creating ZIP file: {}", err);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(serde_json::json!({
                                "error": format!("Error creating ZIP file: {}", err)
                            })),
                        )
                            .into_response()
                    }
                }
            }
            Err(err) => {
                tracing::error!("Folder not found: {}", err);
                let status = match err.kind {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };

                (
                    status,
                    Json(serde_json::json!({
                        "error": format!("Error finding folder: {}", err)
                    })),
                )
                    .into_response()
            }
        }
    }
}
