/**
 * CardDAV Handler Module
 * 
 * This module implements the CardDAV protocol (RFC 6352) endpoints for OxiCloud.
 * It provides contact/address book access and management through standard CardDAV
 * methods, allowing clients like Thunderbird, Apple Contacts, GNOME Contacts,
 * and DAVx⁵ to sync contacts.
 * 
 * Supported methods:
 * - OPTIONS: Advertise CardDAV capabilities
 * - PROPFIND: List address books and their properties
 * - REPORT: Query contacts (addressbook-query, addressbook-multiget)
 * - MKCOL (ext): Create a new address book
 * - PUT: Create/update contacts (.vcf)
 * - GET: Retrieve contact vCard data
 * - DELETE: Remove address books or contacts
 * - PROPPATCH: Modify address book properties
 */

use axum::{
    Router,
    response::Response,
    http::{StatusCode, header, HeaderName, Request},
    body::{Body, self},
};
use std::sync::Arc;
use bytes::Buf;

use crate::common::di::AppState;
use crate::application::adapters::carddav_adapter::{CardDavAdapter, CardDavReportType, contact_to_vcard};
use crate::application::adapters::webdav_adapter::{PropFindRequest, PropFindType};
use crate::application::ports::carddav_ports::{AddressBookUseCase, ContactUseCase};
use crate::application::dtos::address_book_dto::{
    CreateAddressBookDto, UpdateAddressBookDto,
};
use crate::application::dtos::contact_dto::CreateContactVCardDto;
use crate::interfaces::middleware::auth::CurrentUser;
use crate::interfaces::errors::AppError;

const HEADER_DAV: HeaderName = HeaderName::from_static("dav");

/// Creates CardDAV routes with full path prefixes.
/// 
/// Uses `merge()` instead of `nest()` to avoid Axum's trailing-slash routing gap.
/// Registers `/carddav`, `/carddav/`, and `/carddav/{*path}` explicitly.
pub fn carddav_routes() -> Router<AppState> {
    Router::new()
        .route("/carddav/{*path}", axum::routing::any(handle_carddav_methods))
        .route("/carddav/", axum::routing::any(handle_carddav_methods_root))
        .route("/carddav", axum::routing::any(handle_carddav_methods_root))
}

async fn handle_carddav_methods_root(
    axum::extract::State(state): axum::extract::State<AppState>,
    req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    handle_carddav_methods_inner(state, req, String::new()).await
}

async fn handle_carddav_methods(
    axum::extract::State(state): axum::extract::State<AppState>,
    req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    let uri = req.uri().clone();
    let path = extract_carddav_path(uri.path());
    handle_carddav_methods_inner(state, req, path).await
}

async fn handle_carddav_methods_inner(
    state: AppState,
    req: Request<Body>,
    path: String,
) -> Result<Response<Body>, AppError> {
    let state = Arc::new(state);
    let method = req.method().clone();
    
    match method.as_str() {
        "OPTIONS" => handle_options().await,
        "PROPFIND" => handle_propfind(state.clone(), req, &path).await,
        "REPORT" => handle_report(state.clone(), req, &path).await,
        "MKCOL" => handle_mkcol(state.clone(), req, &path).await,
        "PUT" => handle_put(state.clone(), req, &path).await,
        "GET" => handle_get(state.clone(), req, &path).await,
        "DELETE" => handle_delete(state.clone(), req, &path).await,
        "PROPPATCH" => handle_proppatch(state.clone(), req, &path).await,
        _ => Err(AppError::method_not_allowed(format!("Method not allowed: {}", method))),
    }
}

/// Extract the CardDAV path from the full URI path.
fn extract_carddav_path(uri_path: &str) -> String {
    if let Some(pos) = uri_path.find("/carddav/") {
        let after = &uri_path[pos + 9..];
        after.trim_end_matches('/').to_string()
    } else if uri_path.ends_with("/carddav") {
        String::new()
    } else {
        uri_path.trim_start_matches('/').trim_end_matches('/').to_string()
    }
}

// ─── Helper: extract user from request ───────────────────────────────

fn extract_user(req: &Request<Body>) -> Result<CurrentUser, AppError> {
    req.extensions()
        .get::<CurrentUser>()
        .cloned()
        .ok_or_else(|| AppError::unauthorized("Authentication required"))
}

fn get_addressbook_service(state: &AppState) -> Result<&Arc<dyn AddressBookUseCase>, AppError> {
    state.addressbook_use_case.as_ref().ok_or_else(|| {
        AppError::new(
            StatusCode::NOT_IMPLEMENTED,
            "CardDAV address book service is not configured",
            "NotImplemented",
        )
    })
}

fn get_contact_service(state: &AppState) -> Result<&Arc<dyn ContactUseCase>, AppError> {
    state.contact_use_case.as_ref().ok_or_else(|| {
        AppError::new(
            StatusCode::NOT_IMPLEMENTED,
            "CardDAV contact service is not configured",
            "NotImplemented",
        )
    })
}

// ─── OPTIONS ─────────────────────────────────────────────────────────

async fn handle_options() -> Result<Response<Body>, AppError> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(HEADER_DAV, "1, 2, 3, addressbook")
        .header(header::ALLOW, "OPTIONS, GET, PUT, DELETE, PROPFIND, PROPPATCH, REPORT, MKCOL")
        .body(Body::empty())
        .unwrap())
}

// ─── PROPFIND ────────────────────────────────────────────────────────

async fn handle_propfind(
    state: Arc<AppState>,
    req: Request<Body>,
    path: &str,
) -> Result<Response<Body>, AppError> {
    let depth = req.headers()
        .get("Depth")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("1")
        .to_string();
    
    let user = extract_user(&req)?;
    let addressbook_service = get_addressbook_service(&state)?;
    let contact_svc = get_contact_service(&state)?;
    
    let body_bytes = body::to_bytes(req.into_body(), usize::MAX)
        .await
        .map_err(|e| AppError::bad_request(format!("Failed to read request body: {}", e)))?;
    
    let propfind_request = if body_bytes.is_empty() {
        PropFindRequest { prop_find_type: PropFindType::AllProp }
    } else {
        crate::application::adapters::webdav_adapter::WebDavAdapter::parse_propfind(body_bytes.reader())
            .map_err(|e| AppError::bad_request(format!("Failed to parse PROPFIND: {}", e)))?
    };
    
    if path.is_empty() {
        // Root CardDAV path — list user's address books
        let address_books = addressbook_service.list_user_address_books(&user.id).await
            .map_err(|e| AppError::internal_error(format!("Failed to list address books: {}", e)))?;
        
        let base_href = "/carddav/";
        let mut response_body = Vec::new();
        CardDavAdapter::generate_addressbooks_propfind_response(
            &mut response_body,
            &address_books,
            &propfind_request,
            base_href,
        ).map_err(|e| AppError::internal_error(format!("Failed to generate XML: {}", e)))?;
        
        Ok(Response::builder()
            .status(StatusCode::MULTI_STATUS)
            .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
            .body(Body::from(response_body))
            .unwrap())
    } else {
        let parts: Vec<&str> = path.splitn(2, '/').collect();
        let address_book_id = parts[0];
        
        if parts.len() == 1 {
            // Address book collection
            let address_book = addressbook_service.get_address_book(address_book_id, &user.id).await
                .map_err(|e| AppError::not_found(format!("Address book not found: {}", e)))?;
            
            let contacts = if depth != "0" {
                contact_svc.list_contacts(address_book_id, &user.id).await
                    .unwrap_or_default()
            } else {
                vec![]
            };
            
            let base_href = &format!("/carddav/{}/", address_book_id);
            let mut response_body = Vec::new();
            
            CardDavAdapter::generate_addressbook_collection_propfind(
                &mut response_body,
                &address_book,
                &contacts,
                &propfind_request,
                base_href,
                &depth,
            ).map_err(|e| AppError::internal_error(format!("Failed to generate XML: {}", e)))?;
            
            Ok(Response::builder()
                .status(StatusCode::MULTI_STATUS)
                .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
                .body(Body::from(response_body))
                .unwrap())
        } else {
            // Individual contact .vcf
            let contact_file = parts[1];
            let contact_uid = contact_file.trim_end_matches(".vcf");
            
            // Look up by UID across all contacts in this address book
            let contacts = contact_svc.list_contacts(address_book_id, &user.id).await
                .map_err(|e| AppError::internal_error(format!("Failed to list contacts: {}", e)))?;
            
            let contact = contacts.iter().find(|c| c.uid == contact_uid)
                .ok_or_else(|| AppError::not_found(format!("Contact not found: {}", contact_uid)))?;
            
            // Build single-resource PROPFIND response
            let base_href = &format!("/carddav/{}/", address_book_id);
            let report = CardDavReportType::AddressbookMultiget {
                hrefs: vec![format!("{}{}.vcf", base_href, contact_uid)],
                props: vec![],
            };
            
            let mut response_body = Vec::new();
            CardDavAdapter::generate_contacts_response(
                &mut response_body,
                &[contact.clone()],
                &[(contact.uid.clone(), contact_to_vcard(contact))],
                &report,
                base_href,
            ).map_err(|e| AppError::internal_error(format!("Failed to generate XML: {}", e)))?;
            
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
    let contact_svc = get_contact_service(&state)?;
    
    let body_bytes = body::to_bytes(req.into_body(), usize::MAX)
        .await
        .map_err(|e| AppError::bad_request(format!("Failed to read request body: {}", e)))?;
    
    let report = CardDavAdapter::parse_report(body_bytes.reader())
        .map_err(|e| AppError::bad_request(format!("Failed to parse REPORT: {}", e)))?;
    
    let address_book_id = path.split('/').next().unwrap_or(path);
    
    if address_book_id.is_empty() {
        return Err(AppError::bad_request("Address book ID required in path"));
    }
    
    let contacts = match &report {
        CardDavReportType::AddressbookQuery { .. } => {
            contact_svc.list_contacts(address_book_id, &user.id).await
                .map_err(|e| AppError::internal_error(format!("Failed to list contacts: {}", e)))?
        },
        CardDavReportType::AddressbookMultiget { hrefs, .. } => {
            let all_contacts = contact_svc.list_contacts(address_book_id, &user.id).await
                .map_err(|e| AppError::internal_error(format!("Failed to list contacts: {}", e)))?;
            
            all_contacts.into_iter()
                .filter(|c| hrefs.iter().any(|href| href.contains(&c.uid)))
                .collect()
        },
        CardDavReportType::SyncCollection { .. } => {
            contact_svc.list_contacts(address_book_id, &user.id).await
                .map_err(|e| AppError::internal_error(format!("Failed to list contacts: {}", e)))?
        },
    };
    
    // Generate vCards
    let vcards: Vec<(String, String)> = contacts.iter()
        .map(|c| (c.uid.clone(), contact_to_vcard(c)))
        .collect();
    
    let base_href = &format!("/carddav/{}/", address_book_id);
    let mut response_body = Vec::new();
    CardDavAdapter::generate_contacts_response(
        &mut response_body,
        &contacts,
        &vcards,
        &report,
        base_href,
    ).map_err(|e| AppError::internal_error(format!("Failed to generate XML: {}", e)))?;
    
    Ok(Response::builder()
        .status(StatusCode::MULTI_STATUS)
        .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
        .body(Body::from(response_body))
        .unwrap())
}

// ─── MKCOL (create address book) ─────────────────────────────────────

async fn handle_mkcol(
    state: Arc<AppState>,
    req: Request<Body>,
    path: &str,
) -> Result<Response<Body>, AppError> {
    let user = extract_user(&req)?;
    let addressbook_service = get_addressbook_service(&state)?;
    
    let body_bytes = body::to_bytes(req.into_body(), usize::MAX)
        .await
        .map_err(|e| AppError::bad_request(format!("Failed to read request body: {}", e)))?;
    
    let (name, description, color) = if body_bytes.is_empty() {
        let name = path.split('/').next_back().unwrap_or("New Address Book").to_string();
        (name, None, None)
    } else {
        CardDavAdapter::parse_mkaddressbook(body_bytes.reader())
            .map_err(|e| AppError::bad_request(format!("Failed to parse MKCOL: {}", e)))?
    };
    
    let create_dto = CreateAddressBookDto {
        name,
        owner_id: user.id.clone(),
        description,
        color,
        is_public: Some(false),
    };
    
    addressbook_service.create_address_book(create_dto).await
        .map_err(|e| AppError::internal_error(format!("Failed to create address book: {}", e)))?;
    
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::empty())
        .unwrap())
}

// ─── PUT (.vcf) ──────────────────────────────────────────────────────

async fn handle_put(
    state: Arc<AppState>,
    req: Request<Body>,
    path: &str,
) -> Result<Response<Body>, AppError> {
    let user = extract_user(&req)?;
    let contact_svc = get_contact_service(&state)?;
    
    let parts: Vec<&str> = path.splitn(2, '/').collect();
    if parts.len() < 2 {
        return Err(AppError::bad_request("Path must be {address_book_id}/{uid}.vcf"));
    }
    
    let address_book_id = parts[0];
    
    let body_bytes = body::to_bytes(req.into_body(), usize::MAX)
        .await
        .map_err(|e| AppError::bad_request(format!("Failed to read request body: {}", e)))?;
    
    let vcard_data = String::from_utf8(body_bytes.to_vec())
        .map_err(|e| AppError::bad_request(format!("Invalid UTF-8 in vCard data: {}", e)))?;
    
    // Extract UID from vCard
    let vcard_uid = extract_uid_from_vcard(&vcard_data);
    
    // Check if contact already exists
    let existing = if let Some(ref uid) = vcard_uid {
        let contacts = contact_svc.list_contacts(address_book_id, &user.id).await
            .unwrap_or_default();
        contacts.into_iter().find(|c| c.uid == *uid)
    } else {
        None
    };
    
    if let Some(existing_contact) = existing {
        // Update: delete + recreate from vCard
        contact_svc.delete_contact(&existing_contact.id, &user.id).await
            .map_err(|e| AppError::internal_error(format!("Failed to update contact: {}", e)))?;
        
        let create_dto = CreateContactVCardDto {
            address_book_id: address_book_id.to_string(),
            vcard: vcard_data,
            user_id: user.id.clone(),
        };
        let contact = contact_svc.create_contact_from_vcard(create_dto).await
            .map_err(|e| AppError::internal_error(format!("Failed to recreate contact: {}", e)))?;
        
        Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .header(header::ETAG, format!("\"{}\"", contact.etag))
            .body(Body::empty())
            .unwrap())
    } else {
        let create_dto = CreateContactVCardDto {
            address_book_id: address_book_id.to_string(),
            vcard: vcard_data,
            user_id: user.id.clone(),
        };
        
        let contact = contact_svc.create_contact_from_vcard(create_dto).await
            .map_err(|e| AppError::internal_error(format!("Failed to create contact: {}", e)))?;
        
        Ok(Response::builder()
            .status(StatusCode::CREATED)
            .header(header::ETAG, format!("\"{}\"", contact.etag))
            .body(Body::empty())
            .unwrap())
    }
}

/// Extract UID from vCard data
fn extract_uid_from_vcard(vcard_data: &str) -> Option<String> {
    for line in vcard_data.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("UID:") {
            return Some(trimmed[4..].trim().to_string());
        }
    }
    None
}

// ─── GET (.vcf) ──────────────────────────────────────────────────────

async fn handle_get(
    state: Arc<AppState>,
    req: Request<Body>,
    path: &str,
) -> Result<Response<Body>, AppError> {
    let user = extract_user(&req)?;
    let contact_svc = get_contact_service(&state)?;
    
    let parts: Vec<&str> = path.splitn(2, '/').collect();
    let address_book_id = parts[0];
    
    if parts.len() < 2 {
        // GET on address book collection — return all contacts as vcf
        let contacts = contact_svc.list_contacts(address_book_id, &user.id).await
            .map_err(|e| AppError::internal_error(format!("Failed to list contacts: {}", e)))?;
        
        let mut vcf_data = String::new();
        for contact in &contacts {
            vcf_data.push_str(&contact_to_vcard(contact));
        }
        
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/vcard; charset=utf-8")
            .body(Body::from(vcf_data))
            .unwrap())
    } else {
        // GET on individual contact
        let contact_file = parts[1];
        let contact_uid = contact_file.trim_end_matches(".vcf");
        
        let contacts = contact_svc.list_contacts(address_book_id, &user.id).await
            .map_err(|e| AppError::internal_error(format!("Failed to list contacts: {}", e)))?;
        
        let contact = contacts.iter().find(|c| c.uid == contact_uid)
            .ok_or_else(|| AppError::not_found(format!("Contact not found: {}", contact_uid)))?;
        
        let vcard = contact_to_vcard(contact);
        
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/vcard; charset=utf-8")
            .header(header::ETAG, format!("\"{}\"", contact.etag))
            .body(Body::from(vcard))
            .unwrap())
    }
}

// ─── DELETE ──────────────────────────────────────────────────────────

async fn handle_delete(
    state: Arc<AppState>,
    req: Request<Body>,
    path: &str,
) -> Result<Response<Body>, AppError> {
    let user = extract_user(&req)?;
    let addressbook_service = get_addressbook_service(&state)?;
    let contact_svc = get_contact_service(&state)?;
    
    let parts: Vec<&str> = path.splitn(2, '/').collect();
    let address_book_id = parts[0];
    
    if address_book_id.is_empty() {
        return Err(AppError::bad_request("Address book ID required"));
    }
    
    if parts.len() < 2 {
        // Delete address book
        addressbook_service.delete_address_book(address_book_id, &user.id).await
            .map_err(|e| AppError::internal_error(format!("Failed to delete address book: {}", e)))?;
    } else {
        // Delete contact
        let contact_file = parts[1];
        let contact_uid = contact_file.trim_end_matches(".vcf");
        
        let contacts = contact_svc.list_contacts(address_book_id, &user.id).await
            .map_err(|e| AppError::internal_error(format!("Failed to list contacts: {}", e)))?;
        
        let contact = contacts.iter().find(|c| c.uid == contact_uid)
            .ok_or_else(|| AppError::not_found(format!("Contact not found: {}", contact_uid)))?;
        
        contact_svc.delete_contact(&contact.id, &user.id).await
            .map_err(|e| AppError::internal_error(format!("Failed to delete contact: {}", e)))?;
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
    let addressbook_service = get_addressbook_service(&state)?;
    
    let body_bytes = body::to_bytes(req.into_body(), usize::MAX)
        .await
        .map_err(|e| AppError::bad_request(format!("Failed to read request body: {}", e)))?;
    
    let (props_to_set, props_to_remove) = crate::application::adapters::webdav_adapter::WebDavAdapter::parse_proppatch(body_bytes.reader())
        .map_err(|e| AppError::bad_request(format!("Failed to parse PROPPATCH: {}", e)))?;
    
    let address_book_id = path.split('/').next().unwrap_or(path);
    
    if address_book_id.is_empty() {
        return Err(AppError::bad_request("Address book ID required"));
    }
    
    let mut update = UpdateAddressBookDto {
        name: None,
        description: None,
        color: None,
        is_public: None,
        user_id: user.id.clone(),
    };
    
    for prop in &props_to_set {
        match prop.name.name.as_str() {
            "displayname" => update.name = Some(prop.value.clone().unwrap_or_default()),
            "addressbook-description" => update.description = prop.value.clone(),
            "calendar-color" | "addressbook-color" => update.color = prop.value.clone(),
            _ => {}
        }
    }
    
    if update.name.is_some() || update.description.is_some() || update.color.is_some() {
        addressbook_service.update_address_book(address_book_id, update).await
            .map_err(|e| AppError::internal_error(format!("Failed to update address book: {}", e)))?;
    }
    
    let mut results = Vec::new();
    for prop in &props_to_set {
        results.push((&prop.name, true));
    }
    for prop in &props_to_remove {
        results.push((prop, true));
    }
    
    let href = format!("/carddav/{}", path);
    let mut response_body = Vec::new();
    crate::application::adapters::webdav_adapter::WebDavAdapter::generate_proppatch_response(
        &mut response_body,
        &href,
        &results,
    ).map_err(|e| AppError::internal_error(format!("Failed to generate XML: {}", e)))?;
    
    Ok(Response::builder()
        .status(StatusCode::MULTI_STATUS)
        .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
        .body(Body::from(response_body))
        .unwrap())
}
