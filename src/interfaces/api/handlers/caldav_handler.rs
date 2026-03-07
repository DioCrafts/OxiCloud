/**
 * CalDAV Handler Module
 *
 * This module implements the CalDAV protocol (RFC 4791) endpoints for OxiCloud.
 * It provides calendar access and management through standard CalDAV methods,
 * allowing clients like Thunderbird, Apple Calendar, and GNOME Calendar to sync.
 *
 * Supported methods:
 * - OPTIONS: Advertise CalDAV capabilities
 * - PROPFIND: List calendars and their properties
 * - REPORT: Query events (calendar-query, calendar-multiget)
 * - MKCALENDAR: Create a new calendar
 * - PUT: Create/update calendar events (.ics)
 * - GET: Retrieve calendar event data
 * - DELETE: Remove calendars or events
 * - PROPPATCH: Modify calendar properties
 */
use axum::{
    Router,
    body::{self, Body},
    http::{HeaderName, Request, StatusCode, header},
    response::Response,
};
use bytes::Buf;
use percent_encoding::percent_decode_str;
use std::fmt::Write;
use std::sync::Arc;

use crate::application::adapters::caldav_adapter::{CalDavAdapter, CalDavReportType};
use crate::application::adapters::webdav_adapter::{PropFindRequest, PropFindType};
use crate::application::dtos::calendar_dto::{
    CreateCalendarDto, CreateEventICalDto, UpdateCalendarDto,
};
use crate::application::ports::calendar_ports::CalendarUseCase;
use crate::application::services::calendar_service::CalendarService;
use crate::common::di::AppState;
use crate::interfaces::errors::AppError;
use crate::interfaces::middleware::auth::CurrentUser;

const HEADER_DAV: HeaderName = HeaderName::from_static("dav");

/// Maximum allowed request body size for CalDAV XML/iCal endpoints (1 MB).
/// Prevents OOM/DoS via unbounded body buffering.
const MAX_CALDAV_BODY: usize = 1_048_576;

/// Creates CalDAV routes with full path prefixes.
///
/// Uses `merge()` instead of `nest()` to avoid Axum's trailing-slash routing gap.
/// Registers `/caldav`, `/caldav/`, and `/caldav/{*path}` explicitly.
pub fn caldav_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/caldav/{*path}", axum::routing::any(handle_caldav_methods))
        .route("/caldav/", axum::routing::any(handle_caldav_methods_root))
        .route("/caldav", axum::routing::any(handle_caldav_methods_root))
}

/// Creates RFC 6764 well-known discovery routes.
/// These are public (no auth) and simply redirect to the CalDAV root.
pub fn well_known_routes() -> Router<Arc<AppState>> {
    Router::new().route(
        "/.well-known/caldav",
        axum::routing::any(handle_well_known_caldav),
    )
}

async fn handle_well_known_caldav() -> Response<Body> {
    Response::builder()
        .status(StatusCode::MOVED_PERMANENTLY)
        .header(header::LOCATION, "/caldav/")
        .body(Body::empty())
        .unwrap()
}

async fn handle_caldav_methods_root(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    handle_caldav_methods_inner(state, req, String::new()).await
}

async fn handle_caldav_methods(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    let uri = req.uri().clone();
    let path = extract_caldav_path(uri.path());
    reject_path_traversal(&path)?;
    handle_caldav_methods_inner(state, req, path).await
}

async fn handle_caldav_methods_inner(
    state: Arc<AppState>,
    req: Request<Body>,
    path: String,
) -> Result<Response<Body>, AppError> {
    let method = req.method().clone();

    match method.as_str() {
        "OPTIONS" => handle_options().await,
        "PROPFIND" => handle_propfind(state, req, &path).await,
        "REPORT" => handle_report(state, req, &path).await,
        "MKCALENDAR" => handle_mkcalendar(state, req, &path).await,
        "PUT" => handle_put(state, req, &path).await,
        "GET" => handle_get(state, req, &path).await,
        "DELETE" => handle_delete(state, req, &path).await,
        "PROPPATCH" => handle_proppatch(state, req, &path).await,
        _ => Err(AppError::method_not_allowed(format!(
            "Method not allowed: {}",
            method
        ))),
    }
}

/// Extract the CalDAV path from the full URI path, percent-decoding the result.
fn extract_caldav_path(uri_path: &str) -> String {
    let encoded = if let Some(pos) = uri_path.find("/caldav/") {
        let after = &uri_path[pos + 8..];
        after.trim_end_matches('/')
    } else if uri_path.ends_with("/caldav") {
        ""
    } else {
        uri_path.trim_start_matches('/').trim_end_matches('/')
    };
    percent_decode_str(encoded).decode_utf8_lossy().into_owned()
}

/// Reject paths that contain path-traversal segments (`.` or `..`).
fn reject_path_traversal(path: &str) -> Result<(), AppError> {
    for segment in path.split('/') {
        if segment == ".." || segment == "." {
            return Err(AppError::bad_request(
                "Path must not contain '.' or '..' segments",
            ));
        }
    }
    Ok(())
}

// ─── Helper: extract user from request ───────────────────────────────

fn extract_user(req: &Request<Body>) -> Result<CurrentUser, AppError> {
    req.extensions()
        .get::<Arc<CurrentUser>>()
        .map(|arc| (**arc).clone())
        .ok_or_else(|| AppError::unauthorized("Authentication required"))
}

fn get_calendar_service(state: &AppState) -> Result<&Arc<CalendarService>, AppError> {
    state.calendar_use_case.as_ref().ok_or_else(|| {
        AppError::new(
            StatusCode::NOT_IMPLEMENTED,
            "CalDAV service is not configured",
            "NotImplemented",
        )
    })
}

// ─── OPTIONS ─────────────────────────────────────────────────────────

async fn handle_options() -> Result<Response<Body>, AppError> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(HEADER_DAV, "1, 2, calendar-access")
        .header(
            header::ALLOW,
            "OPTIONS, GET, PUT, DELETE, PROPFIND, PROPPATCH, REPORT, MKCALENDAR",
        )
        .body(Body::empty())
        .unwrap())
}

// ─── PROPFIND ────────────────────────────────────────────────────────

async fn handle_propfind(
    state: Arc<AppState>,
    req: Request<Body>,
    path: &str,
) -> Result<Response<Body>, AppError> {
    let depth = req
        .headers()
        .get("Depth")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("1")
        .to_string();

    let user = extract_user(&req)?;
    let calendar_service = get_calendar_service(&state)?;

    let body_bytes = body::to_bytes(req.into_body(), MAX_CALDAV_BODY)
        .await
        .map_err(|e| AppError::bad_request(format!("Failed to read request body: {}", e)))?;

    // Parse PROPFIND request
    let propfind_request = if body_bytes.is_empty() {
        PropFindRequest {
            prop_find_type: PropFindType::AllProp,
        }
    } else {
        crate::application::adapters::webdav_adapter::WebDavAdapter::parse_propfind(
            body_bytes.reader(),
        )
        .map_err(|e| AppError::bad_request(format!("Failed to parse PROPFIND: {}", e)))?
    };

    if path.is_empty() {
        // Root CalDAV path — return discovery properties + list user's calendars
        // At depth 0, only return root entry; at depth 1+, also include calendars
        let calendars = if depth == "0" {
            vec![]
        } else {
            calendar_service
                .list_my_calendars(user.id)
                .await
                .map_err(|e| AppError::internal_error(format!("Failed to list calendars: {}", e)))?
        };

        let base_href = "/caldav/";
        let mut response_body = Vec::new();
        CalDavAdapter::generate_root_propfind_response(
            &mut response_body,
            &calendars,
            &propfind_request,
            base_href,
            &user.username,
        )
        .map_err(|e| AppError::internal_error(format!("Failed to generate XML: {}", e)))?;

        Ok(Response::builder()
            .status(StatusCode::MULTI_STATUS)
            .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
            .body(Body::from(response_body))
            .unwrap())
    } else if path.starts_with("principals/") || path == "principals" {
        // Principal resource — return user principal properties
        let username = path.strip_prefix("principals/").unwrap_or(&user.username);
        let username = if username.is_empty() {
            &user.username
        } else {
            username
        };

        let mut response_body = Vec::new();
        CalDavAdapter::generate_principal_propfind_response(
            &mut response_body,
            &propfind_request,
            username,
        )
        .map_err(|e| AppError::internal_error(format!("Failed to generate XML: {}", e)))?;

        Ok(Response::builder()
            .status(StatusCode::MULTI_STATUS)
            .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
            .body(Body::from(response_body))
            .unwrap())
    } else {
        // Path could be:
        //   {username}                        — user calendar home (from calendar-home-set)
        //   {calendar_id}                     — calendar collection
        //   {calendar_id}/{event_uid}.ics     — individual event
        //   {username}/{calendar_id}          — calendar under user home
        //   {username}/{calendar_id}/{uid}.ics — event under user home
        let parts: Vec<&str> = path.splitn(2, '/').collect();
        let first_segment = parts[0];

        if parts.len() == 1 {
            // Single path segment: try as calendar ID first, fall back to user home
            let calendar_result = calendar_service.get_calendar(first_segment, user.id).await;

            if let Ok(calendar) = calendar_result {
                // Valid calendar ID — return calendar collection
                let events = if depth != "0" {
                    calendar_service
                        .list_events(first_segment, None, None, user.id)
                        .await
                        .unwrap_or_default()
                } else {
                    vec![]
                };

                let base_href = &format!("/caldav/{}/", first_segment);
                let mut response_body = Vec::new();

                CalDavAdapter::generate_calendar_collection_propfind(
                    &mut response_body,
                    &calendar,
                    &events,
                    &propfind_request,
                    base_href,
                    &depth,
                )
                .map_err(|e| AppError::internal_error(format!("Failed to generate XML: {}", e)))?;

                Ok(Response::builder()
                    .status(StatusCode::MULTI_STATUS)
                    .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
                    .body(Body::from(response_body))
                    .unwrap())
            } else {
                // Not a calendar ID — treat as user calendar home (e.g. /caldav/{username}/)
                // List all calendars for this user
                let calendars =
                    calendar_service
                        .list_my_calendars(user.id)
                        .await
                        .map_err(|e| {
                            AppError::internal_error(format!("Failed to list calendars: {}", e))
                        })?;

                let base_href = &format!("/caldav/{}/", first_segment);
                let mut response_body = Vec::new();

                CalDavAdapter::generate_calendars_propfind_response(
                    &mut response_body,
                    &calendars,
                    &propfind_request,
                    base_href,
                )
                .map_err(|e| AppError::internal_error(format!("Failed to generate XML: {}", e)))?;

                Ok(Response::builder()
                    .status(StatusCode::MULTI_STATUS)
                    .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
                    .body(Body::from(response_body))
                    .unwrap())
            }
        } else {
            // Multi-segment path: {something}/{rest}
            let rest = parts[1];

            // Check if first_segment is a valid calendar ID
            let calendar_result = calendar_service.get_calendar(first_segment, user.id).await;

            let (calendar_id, event_path) = if calendar_result.is_ok() {
                // first_segment is a calendar ID, rest is event path
                (first_segment, rest)
            } else {
                // first_segment may be a username, rest could be {calendar_id} or
                // {calendar_id}/{event}.ics
                let sub_parts: Vec<&str> = rest.splitn(2, '/').collect();
                if sub_parts.len() == 1 {
                    // /caldav/{username}/{calendar_id}
                    // Try to get this as a calendar collection
                    let cal = calendar_service
                        .get_calendar(sub_parts[0], user.id)
                        .await
                        .map_err(|e| AppError::not_found(format!("Calendar not found: {}", e)))?;

                    let events = if depth != "0" {
                        calendar_service
                            .list_events(sub_parts[0], None, None, user.id)
                            .await
                            .unwrap_or_default()
                    } else {
                        vec![]
                    };

                    let base_href = &format!("/caldav/{}/{}/", first_segment, sub_parts[0]);
                    let mut response_body = Vec::new();

                    CalDavAdapter::generate_calendar_collection_propfind(
                        &mut response_body,
                        &cal,
                        &events,
                        &propfind_request,
                        base_href,
                        &depth,
                    )
                    .map_err(|e| {
                        AppError::internal_error(format!("Failed to generate XML: {}", e))
                    })?;

                    return Ok(Response::builder()
                        .status(StatusCode::MULTI_STATUS)
                        .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
                        .body(Body::from(response_body))
                        .unwrap());
                } else {
                    // /caldav/{username}/{calendar_id}/{event}.ics
                    (sub_parts[0], sub_parts[1])
                }
            };

            // Individual event .ics
            let ical_uid = event_path.trim_end_matches(".ics");

            let events = calendar_service
                .list_events(calendar_id, None, None, user.id)
                .await
                .map_err(|e| AppError::internal_error(format!("Failed to list events: {}", e)))?;

            let event = events
                .iter()
                .find(|e| e.ical_uid == ical_uid)
                .ok_or_else(|| AppError::not_found(format!("Event not found: {}", ical_uid)))?;

            let base_href = &format!("/caldav/{}/", calendar_id);
            let report_type = CalDavReportType::CalendarMultiget {
                hrefs: vec![format!("{}{}.ics", base_href, ical_uid)],
                props: vec![],
            };

            let mut response_body = Vec::new();
            CalDavAdapter::generate_calendar_events_response(
                &mut response_body,
                std::slice::from_ref(event),
                &report_type,
                base_href,
            )
            .map_err(|e| AppError::internal_error(format!("Failed to generate XML: {}", e)))?;

            Ok(Response::builder()
                .status(StatusCode::MULTI_STATUS)
                .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
                .body(Body::from(response_body))
                .unwrap())
        }
    }
}

// ─── REPORT ──────────────────────────────────────────────────────────

async fn handle_report(
    state: Arc<AppState>,
    req: Request<Body>,
    path: &str,
) -> Result<Response<Body>, AppError> {
    let user = extract_user(&req)?;
    let calendar_service = get_calendar_service(&state)?;

    let body_bytes = body::to_bytes(req.into_body(), MAX_CALDAV_BODY)
        .await
        .map_err(|e| AppError::bad_request(format!("Failed to read request body: {}", e)))?;

    let report = CalDavAdapter::parse_report(body_bytes.reader())
        .map_err(|e| AppError::bad_request(format!("Failed to parse REPORT: {}", e)))?;

    let calendar_id = path.split('/').next().unwrap_or(path);

    if calendar_id.is_empty() {
        return Err(AppError::bad_request("Calendar ID required in path"));
    }

    let events = match &report {
        CalDavReportType::CalendarQuery { time_range, .. } => {
            if let Some((start, end)) = time_range {
                calendar_service
                    .get_events_in_range(calendar_id, *start, *end, user.id)
                    .await
                    .map_err(|e| {
                        AppError::internal_error(format!("Failed to query events: {}", e))
                    })?
            } else {
                calendar_service
                    .list_events(calendar_id, None, None, user.id)
                    .await
                    .map_err(|e| {
                        AppError::internal_error(format!("Failed to list events: {}", e))
                    })?
            }
        }
        CalDavReportType::CalendarMultiget { hrefs, .. } => {
            let all_events = calendar_service
                .list_events(calendar_id, None, None, user.id)
                .await
                .map_err(|e| AppError::internal_error(format!("Failed to list events: {}", e)))?;

            all_events
                .into_iter()
                .filter(|evt| hrefs.iter().any(|href| href.contains(&evt.ical_uid)))
                .collect()
        }
        CalDavReportType::SyncCollection { .. } => calendar_service
            .list_events(calendar_id, None, None, user.id)
            .await
            .map_err(|e| AppError::internal_error(format!("Failed to list events: {}", e)))?,
    };

    let base_href = &format!("/caldav/{}/", calendar_id);
    let mut response_body = Vec::new();
    CalDavAdapter::generate_calendar_events_response(
        &mut response_body,
        &events,
        &report,
        base_href,
    )
    .map_err(|e| AppError::internal_error(format!("Failed to generate XML: {}", e)))?;

    Ok(Response::builder()
        .status(StatusCode::MULTI_STATUS)
        .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
        .body(Body::from(response_body))
        .unwrap())
}

// ─── MKCALENDAR ──────────────────────────────────────────────────────

async fn handle_mkcalendar(
    state: Arc<AppState>,
    req: Request<Body>,
    path: &str,
) -> Result<Response<Body>, AppError> {
    let user = extract_user(&req)?;
    let calendar_service = get_calendar_service(&state)?;

    let body_bytes = body::to_bytes(req.into_body(), MAX_CALDAV_BODY)
        .await
        .map_err(|e| AppError::bad_request(format!("Failed to read request body: {}", e)))?;

    let (name, description, color) = if body_bytes.is_empty() {
        let name = path
            .split('/')
            .next_back()
            .unwrap_or("New Calendar")
            .to_string();
        (name, None, None)
    } else {
        CalDavAdapter::parse_mkcalendar(body_bytes.reader())
            .map_err(|e| AppError::bad_request(format!("Failed to parse MKCALENDAR: {}", e)))?
    };

    let create_dto = CreateCalendarDto {
        name,
        description,
        color,
        is_public: Some(false),
    };

    calendar_service
        .create_calendar(create_dto, user.id)
        .await
        .map_err(|e| AppError::internal_error(format!("Failed to create calendar: {}", e)))?;

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::empty())
        .unwrap())
}

// ─── PUT (.ics) ──────────────────────────────────────────────────────

async fn handle_put(
    state: Arc<AppState>,
    req: Request<Body>,
    path: &str,
) -> Result<Response<Body>, AppError> {
    let user = extract_user(&req)?;
    let calendar_service = get_calendar_service(&state)?;

    let parts: Vec<&str> = path.splitn(2, '/').collect();
    if parts.len() < 2 {
        return Err(AppError::bad_request(
            "Path must be {calendar_id}/{uid}.ics",
        ));
    }

    let calendar_id = parts[0];

    let body_bytes = body::to_bytes(req.into_body(), MAX_CALDAV_BODY)
        .await
        .map_err(|e| AppError::bad_request(format!("Failed to read request body: {}", e)))?;

    let ical_data = String::from_utf8(body_bytes.to_vec())
        .map_err(|e| AppError::bad_request(format!("Invalid UTF-8 in iCalendar data: {}", e)))?;

    let ical_uid = extract_uid_from_ical(&ical_data);

    let existing = if let Some(ref uid) = ical_uid {
        let events = calendar_service
            .list_events(calendar_id, None, None, user.id)
            .await
            .unwrap_or_default();
        events.into_iter().find(|e| e.ical_uid == *uid)
    } else {
        None
    };

    if let Some(existing_event) = existing {
        // Update existing event — re-create from iCal for full fidelity
        calendar_service
            .delete_event(&existing_event.id, user.id)
            .await
            .map_err(|e| AppError::internal_error(format!("Failed to update event: {}", e)))?;

        let create_dto = CreateEventICalDto {
            calendar_id: calendar_id.to_string(),
            ical_data,
        };
        let event = calendar_service
            .create_event_from_ical(create_dto, user.id)
            .await
            .map_err(|e| AppError::internal_error(format!("Failed to recreate event: {}", e)))?;

        Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .header(header::ETAG, format!("\"{}\"", event.id))
            .body(Body::empty())
            .unwrap())
    } else {
        let create_dto = CreateEventICalDto {
            calendar_id: calendar_id.to_string(),
            ical_data,
        };

        let event = calendar_service
            .create_event_from_ical(create_dto, user.id)
            .await
            .map_err(|e| AppError::internal_error(format!("Failed to create event: {}", e)))?;

        Ok(Response::builder()
            .status(StatusCode::CREATED)
            .header(header::ETAG, format!("\"{}\"", event.id))
            .body(Body::empty())
            .unwrap())
    }
}

/// Extract UID from iCalendar data
fn extract_uid_from_ical(ical_data: &str) -> Option<String> {
    for line in ical_data.lines() {
        let trimmed = line.trim();
        if let Some(stripped) = trimmed.strip_prefix("UID:") {
            return Some(stripped.trim().to_string());
        }
    }
    None
}

// ─── GET (.ics) ──────────────────────────────────────────────────────

async fn handle_get(
    state: Arc<AppState>,
    req: Request<Body>,
    path: &str,
) -> Result<Response<Body>, AppError> {
    let user = extract_user(&req)?;
    let calendar_service = get_calendar_service(&state)?;

    let parts: Vec<&str> = path.splitn(2, '/').collect();
    let calendar_id = parts[0];

    if parts.len() < 2 {
        // GET on calendar collection
        let events = calendar_service
            .list_events(calendar_id, None, None, user.id)
            .await
            .map_err(|e| AppError::internal_error(format!("Failed to list events: {}", e)))?;

        let calendar = calendar_service
            .get_calendar(calendar_id, user.id)
            .await
            .map_err(|e| AppError::not_found(format!("Calendar not found: {}", e)))?;

        let ical = generate_full_calendar_ical(&calendar.name, &events);

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/calendar; charset=utf-8")
            .header(header::ETAG, format!("\"{}\"", calendar.id))
            .body(Body::from(ical))
            .unwrap())
    } else {
        // GET on individual event
        let event_file = parts[1];
        let ical_uid = event_file.trim_end_matches(".ics");

        let events = calendar_service
            .list_events(calendar_id, None, None, user.id)
            .await
            .map_err(|e| AppError::internal_error(format!("Failed to list events: {}", e)))?;

        let event = events
            .iter()
            .find(|e| e.ical_uid == ical_uid)
            .ok_or_else(|| AppError::not_found(format!("Event not found: {}", ical_uid)))?;

        let ical = generate_event_ical(event);

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/calendar; charset=utf-8")
            .header(header::ETAG, format!("\"{}\"", event.id))
            .body(Body::from(ical))
            .unwrap())
    }
}

fn generate_full_calendar_ical(
    calendar_name: &str,
    events: &[crate::application::dtos::calendar_dto::CalendarEventDto],
) -> String {
    // Pre-estimate: ~200 bytes header + ~320 bytes per event
    let mut buf = String::with_capacity(256 + events.len() * 320);
    let _ = write!(
        buf,
        "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//OxiCloud//NONSGML Calendar//EN\r\nX-WR-CALNAME:{}\r\n",
        calendar_name
    );
    for event in events {
        write_vevent(&mut buf, event);
    }
    buf.push_str("END:VCALENDAR\r\n");
    buf
}

fn generate_event_ical(event: &crate::application::dtos::calendar_dto::CalendarEventDto) -> String {
    let mut buf = String::with_capacity(512);
    buf.push_str("BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//OxiCloud//NONSGML Calendar//EN\r\n");
    write_vevent(&mut buf, event);
    buf.push_str("END:VCALENDAR\r\n");
    buf
}

/// Writes a VEVENT block directly into `buf` — zero intermediate allocations.
fn write_vevent(
    buf: &mut String,
    event: &crate::application::dtos::calendar_dto::CalendarEventDto,
) {
    let _ = write!(
        buf,
        "BEGIN:VEVENT\r\nUID:{}\r\nSUMMARY:{}\r\nDTSTART:{}\r\nDTEND:{}\r\n",
        event.ical_uid,
        event.summary.replace('\n', "\\n"),
        event.start_time.format("%Y%m%dT%H%M%SZ"),
        event.end_time.format("%Y%m%dT%H%M%SZ"),
    );
    if let Some(ref desc) = event.description {
        let _ = write!(buf, "DESCRIPTION:{}\r\n", desc.replace('\n', "\\n"));
    }
    if let Some(ref loc) = event.location {
        let _ = write!(buf, "LOCATION:{}\r\n", loc);
    }
    if let Some(ref rrule) = event.rrule {
        let _ = write!(buf, "RRULE:{}\r\n", rrule);
    }
    let _ = write!(
        buf,
        "DTSTAMP:{}\r\nCREATED:{}\r\nLAST-MODIFIED:{}\r\nEND:VEVENT\r\n",
        event.updated_at.format("%Y%m%dT%H%M%SZ"),
        event.created_at.format("%Y%m%dT%H%M%SZ"),
        event.updated_at.format("%Y%m%dT%H%M%SZ"),
    );
}

// ─── DELETE ──────────────────────────────────────────────────────────

async fn handle_delete(
    state: Arc<AppState>,
    req: Request<Body>,
    path: &str,
) -> Result<Response<Body>, AppError> {
    let user = extract_user(&req)?;
    let calendar_service = get_calendar_service(&state)?;

    let parts: Vec<&str> = path.splitn(2, '/').collect();
    let calendar_id = parts[0];

    if calendar_id.is_empty() {
        return Err(AppError::bad_request("Calendar ID required"));
    }

    if parts.len() < 2 {
        calendar_service
            .delete_calendar(calendar_id, user.id)
            .await
            .map_err(|e| AppError::internal_error(format!("Failed to delete calendar: {}", e)))?;
    } else {
        let event_file = parts[1];
        let ical_uid = event_file.trim_end_matches(".ics");

        let events = calendar_service
            .list_events(calendar_id, None, None, user.id)
            .await
            .map_err(|e| AppError::internal_error(format!("Failed to list events: {}", e)))?;

        let event = events
            .iter()
            .find(|e| e.ical_uid == ical_uid)
            .ok_or_else(|| AppError::not_found(format!("Event not found: {}", ical_uid)))?;

        calendar_service
            .delete_event(&event.id, user.id)
            .await
            .map_err(|e| AppError::internal_error(format!("Failed to delete event: {}", e)))?;
    }

    Ok(Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Body::empty())
        .unwrap())
}

// ─── PROPPATCH ───────────────────────────────────────────────────────

async fn handle_proppatch(
    state: Arc<AppState>,
    req: Request<Body>,
    path: &str,
) -> Result<Response<Body>, AppError> {
    let user = extract_user(&req)?;
    let calendar_service = get_calendar_service(&state)?;

    let body_bytes = body::to_bytes(req.into_body(), MAX_CALDAV_BODY)
        .await
        .map_err(|e| AppError::bad_request(format!("Failed to read request body: {}", e)))?;

    let (props_to_set, props_to_remove) =
        crate::application::adapters::webdav_adapter::WebDavAdapter::parse_proppatch(
            body_bytes.reader(),
        )
        .map_err(|e| AppError::bad_request(format!("Failed to parse PROPPATCH: {}", e)))?;

    let calendar_id = path.split('/').next().unwrap_or(path);

    if calendar_id.is_empty() {
        return Err(AppError::bad_request("Calendar ID required"));
    }

    let mut update = UpdateCalendarDto {
        name: None,
        description: None,
        color: None,
        is_public: None,
    };

    for prop in &props_to_set {
        match prop.name.name.as_str() {
            "displayname" => update.name = Some(prop.value.clone().unwrap_or_default()),
            "calendar-description" => update.description = prop.value.clone(),
            "calendar-color" => update.color = prop.value.clone(),
            _ => {}
        }
    }

    if update.name.is_some() || update.description.is_some() || update.color.is_some() {
        calendar_service
            .update_calendar(calendar_id, update, user.id)
            .await
            .map_err(|e| AppError::internal_error(format!("Failed to update calendar: {}", e)))?;
    }

    let mut results = Vec::new();
    for prop in &props_to_set {
        results.push((&prop.name, true));
    }
    for prop in &props_to_remove {
        results.push((prop, true));
    }

    let href = format!("/caldav/{}", path);
    let mut response_body = Vec::new();
    crate::application::adapters::webdav_adapter::WebDavAdapter::generate_proppatch_response(
        &mut response_body,
        &href,
        &results,
    )
    .map_err(|e| AppError::internal_error(format!("Failed to generate XML: {}", e)))?;

    Ok(Response::builder()
        .status(StatusCode::MULTI_STATUS)
        .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
        .body(Body::from(response_body))
        .unwrap())
}
