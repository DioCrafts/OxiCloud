use axum::{
    body::Body,
    http::{HeaderName, Request, StatusCode, header},
    response::Response,
};
use quick_xml::{
    Writer,
    events::{BytesEnd, BytesStart, Event},
};
use std::sync::Arc;

use crate::application::ports::trash_ports::TrashUseCase;
use crate::common::di::AppState;
use crate::interfaces::errors::AppError;
use crate::interfaces::middleware::auth::{AuthUser, CurrentUser};
use crate::interfaces::nextcloud::webdav_handler::{
    format_oc_id, resolve_file_id, resolve_folder_id, write_text_element,
};

const HEADER_DAV: HeaderName = HeaderName::from_static("dav");

/// Dispatch Nextcloud WebDAV trashbin request to the appropriate handler.
///
/// `subpath` is everything after `/remote.php/dav/trashbin/{user}/`.
pub async fn handle_nc_trashbin(
    state: Arc<AppState>,
    req: Request<Body>,
    user: AuthUser,
    subpath: String,
) -> Result<Response<Body>, AppError> {
    let method = req.method().clone();
    let subpath_trimmed = subpath.trim_matches('/');

    match method.as_str() {
        "OPTIONS" => handle_options(),
        "PROPFIND" if subpath_trimmed == "trash" || subpath_trimmed.is_empty() => {
            handle_propfind(state, &user).await
        }
        "MOVE" if subpath_trimmed.starts_with("trash/") => {
            handle_restore(state, &user, subpath_trimmed).await
        }
        "DELETE" if subpath_trimmed == "trash" || subpath_trimmed.is_empty() => {
            handle_empty_trash(state, &user).await
        }
        "DELETE" if subpath_trimmed.starts_with("trash/") => {
            handle_delete_permanent(state, &user, subpath_trimmed).await
        }
        _ => Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .unwrap()),
    }
}

// ──────────────────── OPTIONS ────────────────────

fn handle_options() -> Result<Response<Body>, AppError> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(HEADER_DAV, "1, 2, 3")
        .header(header::ALLOW, "OPTIONS, PROPFIND, MOVE, DELETE")
        .body(Body::empty())
        .unwrap())
}

// ──────────────────── PROPFIND (list trash) ────────────────────

async fn handle_propfind(
    state: Arc<AppState>,
    user: &CurrentUser,
) -> Result<Response<Body>, AppError> {
    let trash_svc = state
        .trash_service
        .as_ref()
        .ok_or_else(|| AppError::internal_error("Trash service not available"))?;

    let items = trash_svc
        .get_trash_items(user.id)
        .await
        .map_err(|e| AppError::internal_error(format!("Failed to list trash: {}", e)))?;

    let nc = state.nextcloud.as_ref();
    let file_id_svc = nc.map(|n| &n.file_ids);

    let mut buf = Vec::new();
    write_trashbin_multistatus(&mut buf, &items, &user.username, file_id_svc)
        .await
        .map_err(|e| AppError::internal_error(format!("XML generation failed: {}", e)))?;

    Ok(Response::builder()
        .status(StatusCode::MULTI_STATUS)
        .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
        .body(Body::from(buf))
        .unwrap())
}

// ──────────────────── MOVE (restore) ────────────────────

async fn handle_restore(
    state: Arc<AppState>,
    user: &CurrentUser,
    subpath: &str,
) -> Result<Response<Body>, AppError> {
    let id = extract_trash_id(subpath)?;

    let trash_svc = state
        .trash_service
        .as_ref()
        .ok_or_else(|| AppError::internal_error("Trash service not available"))?;

    trash_svc
        .restore_item(&id, user.id)
        .await
        .map_err(|e| AppError::internal_error(format!("Failed to restore item: {}", e)))?;

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::empty())
        .unwrap())
}

// ──────────────────── DELETE (empty trash) ────────────────────

async fn handle_empty_trash(
    state: Arc<AppState>,
    user: &CurrentUser,
) -> Result<Response<Body>, AppError> {
    let trash_svc = state
        .trash_service
        .as_ref()
        .ok_or_else(|| AppError::internal_error("Trash service not available"))?;

    trash_svc
        .empty_trash(user.id)
        .await
        .map_err(|e| AppError::internal_error(format!("Failed to empty trash: {}", e)))?;

    Ok(Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Body::empty())
        .unwrap())
}

// ──────────────────── DELETE (single item) ────────────────────

async fn handle_delete_permanent(
    state: Arc<AppState>,
    user: &CurrentUser,
    subpath: &str,
) -> Result<Response<Body>, AppError> {
    let id = extract_trash_id(subpath)?;

    let trash_svc = state
        .trash_service
        .as_ref()
        .ok_or_else(|| AppError::internal_error("Trash service not available"))?;

    trash_svc
        .delete_permanently(&id, user.id)
        .await
        .map_err(|e| {
            AppError::internal_error(format!("Failed to permanently delete item: {}", e))
        })?;

    Ok(Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Body::empty())
        .unwrap())
}

// ────────────── Helpers ──────────────

/// Extract the item ID from a trashbin subpath like `trash/{id}`.
fn extract_trash_id(subpath: &str) -> Result<String, AppError> {
    // subpath is already trimmed, e.g. "trash/some-uuid"
    subpath
        .strip_prefix("trash/")
        .map(|s| s.trim_matches('/').to_string())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| AppError::bad_request("Missing trash item ID in path"))
}

/// Infer MIME content type from filename extension.
fn mime_from_name(name: &str) -> String {
    mime_guess::from_path(name)
        .first_or_octet_stream()
        .to_string()
}

/// Strip the "My Folder - {username}/" prefix from an original path to produce
/// the Nextcloud-relative original location.
fn strip_home_prefix<'a>(original_path: &'a str, username: &str) -> &'a str {
    let prefix = format!("My Folder - {}/", username);
    original_path.strip_prefix(&prefix).unwrap_or(original_path)
}

// ────────────── Trashbin PROPFIND XML Generation ──────────────

use crate::application::dtos::trash_dto::TrashedItemDto;
use crate::application::services::nextcloud_file_id_service::NextcloudFileIdService;

/// Generate a complete Nextcloud-compatible multistatus XML response for the trashbin.
async fn write_trashbin_multistatus<W: std::io::Write>(
    writer: W,
    items: &[TrashedItemDto],
    username: &str,
    file_id_svc: Option<&Arc<NextcloudFileIdService>>,
) -> Result<(), String> {
    let mut xml = Writer::new(writer);

    // Root element with all required namespaces.
    let mut ms = BytesStart::new("d:multistatus");
    ms.push_attribute(("xmlns:d", "DAV:"));
    ms.push_attribute(("xmlns:oc", "http://owncloud.org/ns"));
    ms.push_attribute(("xmlns:nc", "http://nextcloud.org/ns"));
    xml.write_event(Event::Start(ms))
        .map_err(|e| e.to_string())?;

    // Root container entry for the trash collection itself.
    write_trash_root_response(&mut xml, username)?;

    // Individual trashed items.
    for item in items {
        write_trash_item_response(&mut xml, item, username, file_id_svc).await?;
    }

    xml.write_event(Event::End(BytesEnd::new("d:multistatus")))
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Write the root collection response entry for the trash folder.
fn write_trash_root_response<W: std::io::Write>(
    xml: &mut Writer<W>,
    username: &str,
) -> Result<(), String> {
    xml.write_event(Event::Start(BytesStart::new("d:response")))
        .map_err(|e| e.to_string())?;

    let href = format!("/remote.php/dav/trashbin/{}/trash/", username);
    write_text_element(xml, "d:href", &href)?;

    xml.write_event(Event::Start(BytesStart::new("d:propstat")))
        .map_err(|e| e.to_string())?;
    xml.write_event(Event::Start(BytesStart::new("d:prop")))
        .map_err(|e| e.to_string())?;

    // resourcetype = collection
    xml.write_event(Event::Start(BytesStart::new("d:resourcetype")))
        .map_err(|e| e.to_string())?;
    xml.write_event(Event::Empty(BytesStart::new("d:collection")))
        .map_err(|e| e.to_string())?;
    xml.write_event(Event::End(BytesEnd::new("d:resourcetype")))
        .map_err(|e| e.to_string())?;

    xml.write_event(Event::End(BytesEnd::new("d:prop")))
        .map_err(|e| e.to_string())?;
    write_text_element(xml, "d:status", "HTTP/1.1 200 OK")?;
    xml.write_event(Event::End(BytesEnd::new("d:propstat")))
        .map_err(|e| e.to_string())?;

    xml.write_event(Event::End(BytesEnd::new("d:response")))
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Write a single trashed item as a `<d:response>` element.
async fn write_trash_item_response<W: std::io::Write>(
    xml: &mut Writer<W>,
    item: &TrashedItemDto,
    username: &str,
    file_id_svc: Option<&Arc<NextcloudFileIdService>>,
) -> Result<(), String> {
    xml.write_event(Event::Start(BytesStart::new("d:response")))
        .map_err(|e| e.to_string())?;

    // href
    let href = format!("/remote.php/dav/trashbin/{}/trash/{}", username, item.id);
    write_text_element(xml, "d:href", &href)?;

    xml.write_event(Event::Start(BytesStart::new("d:propstat")))
        .map_err(|e| e.to_string())?;
    xml.write_event(Event::Start(BytesStart::new("d:prop")))
        .map_err(|e| e.to_string())?;

    // d:displayname
    write_text_element(xml, "d:displayname", &item.name)?;

    // d:getlastmodified
    write_text_element(xml, "d:getlastmodified", &item.trashed_at.to_rfc2822())?;

    // d:getetag
    write_text_element(xml, "d:getetag", &format!("\"{}\"", item.original_id))?;

    // d:resourcetype
    if item.item_type == "folder" {
        xml.write_event(Event::Start(BytesStart::new("d:resourcetype")))
            .map_err(|e| e.to_string())?;
        xml.write_event(Event::Empty(BytesStart::new("d:collection")))
            .map_err(|e| e.to_string())?;
        xml.write_event(Event::End(BytesEnd::new("d:resourcetype")))
            .map_err(|e| e.to_string())?;
    } else {
        xml.write_event(Event::Empty(BytesStart::new("d:resourcetype")))
            .map_err(|e| e.to_string())?;
    }

    // d:getcontenttype
    let content_type = if item.item_type == "folder" {
        "httpd/unix-directory".to_string()
    } else {
        mime_from_name(&item.name)
    };
    write_text_element(xml, "d:getcontenttype", &content_type)?;

    // d:getcontentlength
    write_text_element(xml, "d:getcontentlength", "0")?;

    // oc:fileid and oc:id — resolve numeric ID via file_id service
    let file_id = if item.item_type == "folder" {
        resolve_folder_id(file_id_svc, &item.original_id).await
    } else {
        resolve_file_id(file_id_svc, &item.original_id).await
    };
    if let Some(id) = file_id {
        write_text_element(xml, "oc:fileid", &id.to_string())?;
        let oc_id = format_oc_id(id, file_id_svc);
        write_text_element(xml, "oc:id", &oc_id)?;
    }

    // nc:trashbin-filename
    write_text_element(xml, "nc:trashbin-filename", &item.name)?;

    // nc:trashbin-original-location
    let original_location = strip_home_prefix(&item.original_path, username);
    write_text_element(xml, "nc:trashbin-original-location", original_location)?;

    // nc:trashbin-deletion-time
    write_text_element(
        xml,
        "nc:trashbin-deletion-time",
        &item.trashed_at.timestamp().to_string(),
    )?;

    // oc:permissions — empty in trash
    write_text_element(xml, "oc:permissions", "")?;

    // oc:size
    write_text_element(xml, "oc:size", "0")?;

    xml.write_event(Event::End(BytesEnd::new("d:prop")))
        .map_err(|e| e.to_string())?;
    write_text_element(xml, "d:status", "HTTP/1.1 200 OK")?;
    xml.write_event(Event::End(BytesEnd::new("d:propstat")))
        .map_err(|e| e.to_string())?;

    xml.write_event(Event::End(BytesEnd::new("d:response")))
        .map_err(|e| e.to_string())?;

    Ok(())
}
