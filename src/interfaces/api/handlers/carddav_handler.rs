use axum::{
    Router,
    routing::{get, put, delete, post},
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use serde_json::json;

use crate::common::di::AppState;
use crate::interfaces::middleware::auth::AuthUser;
use crate::application::dtos::address_book_dto::{
    AddressBookDto, CreateAddressBookDto, UpdateAddressBookDto,
    ShareAddressBookDto, UnshareAddressBookDto
};
use crate::application::dtos::contact_dto::{
    ContactDto, CreateContactDto, UpdateContactDto, CreateContactVCardDto,
    ContactGroupDto, CreateContactGroupDto, UpdateContactGroupDto, GroupMembershipDto
};

// CardDAV handler implementation
pub fn carddav_routes() -> Router<AppState> {
    Router::new()
        // Address book operations
        .route("/address-books", get(list_address_books).post(create_address_book))
        .route("/address-books/:id", 
            get(get_address_book)
            .put(update_address_book)
            .delete(delete_address_book)
        )
        .route("/address-books/:id/shares", 
            get(get_address_book_shares)
        )
        .route("/address-books/:id/share", 
            post(share_address_book)
        )
        .route("/address-books/:id/unshare/:user_id", 
            delete(unshare_address_book)
        )
        
        // Contact operations
        .route("/address-books/:id/contacts", 
            get(list_contacts)
            .post(create_contact)
        )
        .route("/address-books/:id/contacts/search", 
            get(search_contacts)
        )
        .route("/address-books/:id/contacts/vcard", 
            post(create_contact_from_vcard)
        )
        .route("/address-books/:address_book_id/contacts/:contact_id", 
            get(get_contact)
            .put(update_contact)
            .delete(delete_contact)
        )
        .route("/address-books/:address_book_id/contacts/:contact_id/vcard", 
            get(get_contact_vcard)
        )
        
        // Group operations
        .route("/address-books/:id/groups", 
            get(list_groups)
            .post(create_group)
        )
        .route("/address-books/:address_book_id/groups/:group_id", 
            get(get_group)
            .put(update_group)
            .delete(delete_group)
        )
        .route("/address-books/:address_book_id/groups/:group_id/contacts", 
            get(list_contacts_in_group)
        )
        .route("/groups/:group_id/contacts/:contact_id", 
            post(add_contact_to_group)
            .delete(remove_contact_from_group)
        )
        .route("/contacts/:contact_id/groups", 
            get(list_groups_for_contact)
        )
}

// Address Book handlers
async fn list_address_books(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match &state.contact_service {
        Some(contact_service) => {
            let params = json!({
                "user_id": user_id
            });
            
            match contact_service.handle_request("list_user_address_books", params).await {
                Ok(result) => {
                    let address_books: Vec<AddressBookDto> = serde_json::from_value(result)
                        .unwrap_or_else(|_| Vec::new());
                    (StatusCode::OK, Json(address_books))
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to list address books: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn create_address_book(
    State(state): State<AppState>,
    Json(dto): Json<CreateAddressBookDto>,
) -> impl IntoResponse {
    match &state.contact_service {
        Some(contact_service) => {
            match contact_service.handle_request("create_address_book", serde_json::to_value(dto).unwrap()).await {
                Ok(result) => {
                    let address_book: AddressBookDto = serde_json::from_value(result)
                        .unwrap_or_else(|_| AddressBookDto::default());
                    (StatusCode::CREATED, Json(address_book))
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to create address book: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn get_address_book(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match &state.contact_service {
        Some(contact_service) => {
            let params = json!({
                "address_book_id": id,
                "user_id": user_id
            });
            
            match contact_service.handle_request("get_address_book", params).await {
                Ok(result) => {
                    let address_book: AddressBookDto = serde_json::from_value(result)
                        .unwrap_or_else(|_| AddressBookDto::default());
                    (StatusCode::OK, Json(address_book))
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to get address book: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn update_address_book(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
    Json(mut update): Json<UpdateAddressBookDto>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    update.user_id = user_id.to_string();
    
    match &state.contact_service {
        Some(contact_service) => {
            let mut params = serde_json::to_value(update).unwrap();
            
            // Add address_book_id to the params
            if let serde_json::Value::Object(ref mut map) = params {
                map.insert("address_book_id".to_string(), serde_json::Value::String(id));
            }
            
            match contact_service.handle_request("update_address_book", params).await {
                Ok(result) => {
                    let address_book: AddressBookDto = serde_json::from_value(result)
                        .unwrap_or_else(|_| AddressBookDto::default());
                    (StatusCode::OK, Json(address_book))
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to update address book: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn delete_address_book(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match &state.contact_service {
        Some(contact_service) => {
            let params = json!({
                "address_book_id": id,
                "user_id": user_id
            });
            
            match contact_service.handle_request("delete_address_book", params).await {
                Ok(_) => {
                    StatusCode::NO_CONTENT
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to delete address book: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn get_address_book_shares(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match &state.contact_service {
        Some(contact_service) => {
            let params = json!({
                "address_book_id": id,
                "user_id": user_id
            });
            
            match contact_service.handle_request("get_address_book_shares", params).await {
                Ok(result) => {
                    (StatusCode::OK, Json(result))
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to get address book shares: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn share_address_book(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(address_book_id): Path<String>,
    Json(mut dto): Json<ShareAddressBookDto>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    dto.address_book_id = address_book_id;
    
    match &state.contact_service {
        Some(contact_service) => {
            let mut params = serde_json::to_value(dto).unwrap();
            
            // Add user_id to the params
            if let serde_json::Value::Object(ref mut map) = params {
                map.insert("user_id".to_string(), serde_json::Value::String(user_id.to_string()));
            }
            
            match contact_service.handle_request("share_address_book", params).await {
                Ok(_) => {
                    StatusCode::NO_CONTENT
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to share address book: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn unshare_address_book(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((address_book_id, shared_with)): Path<(String, String)>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match &state.contact_service {
        Some(contact_service) => {
            let dto = UnshareAddressBookDto {
                address_book_id,
                user_id: shared_with,
            };
            
            let mut params = serde_json::to_value(dto).unwrap();
            
            // Add user_id to the params
            if let serde_json::Value::Object(ref mut map) = params {
                map.insert("user_id".to_string(), serde_json::Value::String(user_id.to_string()));
            }
            
            match contact_service.handle_request("unshare_address_book", params).await {
                Ok(_) => {
                    StatusCode::NO_CONTENT
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to unshare address book: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

// Contact handlers
async fn list_contacts(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(address_book_id): Path<String>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match &state.contact_service {
        Some(contact_service) => {
            let params = json!({
                "address_book_id": address_book_id,
                "user_id": user_id
            });
            
            match contact_service.handle_request("list_contacts", params).await {
                Ok(result) => {
                    let contacts: Vec<ContactDto> = serde_json::from_value(result)
                        .unwrap_or_else(|_| Vec::new());
                    (StatusCode::OK, Json(contacts))
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to list contacts: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn search_contacts(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(address_book_id): Path<String>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    let query = params.get("q").unwrap_or(&String::new()).to_string();
    
    match &state.contact_service {
        Some(contact_service) => {
            let params = json!({
                "address_book_id": address_book_id,
                "query": query,
                "user_id": user_id
            });
            
            match contact_service.handle_request("search_contacts", params).await {
                Ok(result) => {
                    let contacts: Vec<ContactDto> = serde_json::from_value(result)
                        .unwrap_or_else(|_| Vec::new());
                    (StatusCode::OK, Json(contacts))
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to search contacts: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn create_contact(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(address_book_id): Path<String>,
    Json(mut dto): Json<CreateContactDto>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    dto.address_book_id = address_book_id;
    dto.user_id = user_id.to_string();
    
    match &state.contact_service {
        Some(contact_service) => {
            match contact_service.handle_request("create_contact", serde_json::to_value(dto).unwrap()).await {
                Ok(result) => {
                    let contact: ContactDto = serde_json::from_value(result)
                        .unwrap_or_else(|_| ContactDto::default());
                    (StatusCode::CREATED, Json(contact))
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to create contact: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn create_contact_from_vcard(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(address_book_id): Path<String>,
    Json(mut dto): Json<CreateContactVCardDto>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    dto.address_book_id = address_book_id;
    dto.user_id = user_id.to_string();
    
    match &state.contact_service {
        Some(contact_service) => {
            match contact_service.handle_request("create_contact_from_vcard", serde_json::to_value(dto).unwrap()).await {
                Ok(result) => {
                    let contact: ContactDto = serde_json::from_value(result)
                        .unwrap_or_else(|_| ContactDto::default());
                    (StatusCode::CREATED, Json(contact))
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to create contact from vCard: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn get_contact(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((_, contact_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match &state.contact_service {
        Some(contact_service) => {
            let params = json!({
                "contact_id": contact_id,
                "user_id": user_id
            });
            
            match contact_service.handle_request("get_contact", params).await {
                Ok(result) => {
                    let contact: ContactDto = serde_json::from_value(result)
                        .unwrap_or_else(|_| ContactDto::default());
                    (StatusCode::OK, Json(contact))
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to get contact: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn update_contact(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((_, contact_id)): Path<(String, String)>,
    Json(mut update): Json<UpdateContactDto>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    update.user_id = user_id.to_string();
    
    match &state.contact_service {
        Some(contact_service) => {
            let mut params = serde_json::to_value(update).unwrap();
            
            // Add contact_id to the params
            if let serde_json::Value::Object(ref mut map) = params {
                map.insert("contact_id".to_string(), serde_json::Value::String(contact_id));
            }
            
            match contact_service.handle_request("update_contact", params).await {
                Ok(result) => {
                    let contact: ContactDto = serde_json::from_value(result)
                        .unwrap_or_else(|_| ContactDto::default());
                    (StatusCode::OK, Json(contact))
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to update contact: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn delete_contact(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((_, contact_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match &state.contact_service {
        Some(contact_service) => {
            let params = json!({
                "contact_id": contact_id,
                "user_id": user_id
            });
            
            match contact_service.handle_request("delete_contact", params).await {
                Ok(_) => {
                    StatusCode::NO_CONTENT
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to delete contact: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn get_contact_vcard(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((_, contact_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match &state.contact_service {
        Some(contact_service) => {
            let params = json!({
                "contact_id": contact_id,
                "user_id": user_id
            });
            
            match contact_service.handle_request("get_contact_vcard", params).await {
                Ok(result) => {
                    let vcard = match result {
                        serde_json::Value::String(s) => s,
                        _ => "BEGIN:VCARD\nVERSION:3.0\nEND:VCARD".to_string(),
                    };
                    
                    // Return vCard with proper content type
                    (
                        StatusCode::OK,
                        [
                            ("Content-Type", "text/vcard; charset=utf-8"),
                            ("Content-Disposition", "attachment; filename=\"contact.vcf\""),
                        ],
                        vcard
                    )
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to get contact vCard: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

// Group handlers
async fn list_groups(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(address_book_id): Path<String>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match &state.contact_service {
        Some(contact_service) => {
            let params = json!({
                "address_book_id": address_book_id,
                "user_id": user_id
            });
            
            match contact_service.handle_request("list_groups", params).await {
                Ok(result) => {
                    let groups: Vec<ContactGroupDto> = serde_json::from_value(result)
                        .unwrap_or_else(|_| Vec::new());
                    (StatusCode::OK, Json(groups))
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to list groups: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn create_group(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(address_book_id): Path<String>,
    Json(mut dto): Json<CreateContactGroupDto>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    dto.address_book_id = address_book_id;
    dto.user_id = user_id.to_string();
    
    match &state.contact_service {
        Some(contact_service) => {
            match contact_service.handle_request("create_group", serde_json::to_value(dto).unwrap()).await {
                Ok(result) => {
                    let group: ContactGroupDto = serde_json::from_value(result)
                        .unwrap_or_else(|_| ContactGroupDto::default());
                    (StatusCode::CREATED, Json(group))
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to create group: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn get_group(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((_, group_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match &state.contact_service {
        Some(contact_service) => {
            let params = json!({
                "group_id": group_id,
                "user_id": user_id
            });
            
            match contact_service.handle_request("get_group", params).await {
                Ok(result) => {
                    let group: ContactGroupDto = serde_json::from_value(result)
                        .unwrap_or_else(|_| ContactGroupDto::default());
                    (StatusCode::OK, Json(group))
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to get group: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn update_group(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((_, group_id)): Path<(String, String)>,
    Json(mut update): Json<UpdateContactGroupDto>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    update.user_id = user_id.to_string();
    
    match &state.contact_service {
        Some(contact_service) => {
            let mut params = serde_json::to_value(update).unwrap();
            
            // Add group_id to the params
            if let serde_json::Value::Object(ref mut map) = params {
                map.insert("group_id".to_string(), serde_json::Value::String(group_id));
            }
            
            match contact_service.handle_request("update_group", params).await {
                Ok(result) => {
                    let group: ContactGroupDto = serde_json::from_value(result)
                        .unwrap_or_else(|_| ContactGroupDto::default());
                    (StatusCode::OK, Json(group))
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to update group: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn delete_group(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((_, group_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match &state.contact_service {
        Some(contact_service) => {
            let params = json!({
                "group_id": group_id,
                "user_id": user_id
            });
            
            match contact_service.handle_request("delete_group", params).await {
                Ok(_) => {
                    StatusCode::NO_CONTENT
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to delete group: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn list_contacts_in_group(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((_, group_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match &state.contact_service {
        Some(contact_service) => {
            let params = json!({
                "group_id": group_id,
                "user_id": user_id
            });
            
            match contact_service.handle_request("list_contacts_in_group", params).await {
                Ok(result) => {
                    let contacts: Vec<ContactDto> = serde_json::from_value(result)
                        .unwrap_or_else(|_| Vec::new());
                    (StatusCode::OK, Json(contacts))
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to list contacts in group: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn add_contact_to_group(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((group_id, contact_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match &state.contact_service {
        Some(contact_service) => {
            let dto = GroupMembershipDto {
                group_id,
                contact_id,
            };
            
            let mut params = serde_json::to_value(dto).unwrap();
            
            // Add user_id to the params
            if let serde_json::Value::Object(ref mut map) = params {
                map.insert("user_id".to_string(), serde_json::Value::String(user_id.to_string()));
            }
            
            match contact_service.handle_request("add_contact_to_group", params).await {
                Ok(_) => {
                    StatusCode::NO_CONTENT
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to add contact to group: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn remove_contact_from_group(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((group_id, contact_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match &state.contact_service {
        Some(contact_service) => {
            let dto = GroupMembershipDto {
                group_id,
                contact_id,
            };
            
            let mut params = serde_json::to_value(dto).unwrap();
            
            // Add user_id to the params
            if let serde_json::Value::Object(ref mut map) = params {
                map.insert("user_id".to_string(), serde_json::Value::String(user_id.to_string()));
            }
            
            match contact_service.handle_request("remove_contact_from_group", params).await {
                Ok(_) => {
                    StatusCode::NO_CONTENT
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to remove contact from group: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}

async fn list_groups_for_contact(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(contact_id): Path<String>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match &state.contact_service {
        Some(contact_service) => {
            let params = json!({
                "contact_id": contact_id,
                "user_id": user_id
            });
            
            match contact_service.handle_request("list_groups_for_contact", params).await {
                Ok(result) => {
                    let groups: Vec<ContactGroupDto> = serde_json::from_value(result)
                        .unwrap_or_else(|_| Vec::new());
                    (StatusCode::OK, Json(groups))
                },
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "error": format!("Failed to list groups for contact: {}", e)
                    })))
                }
            }
        },
        None => {
            (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Contact service not available"
            })))
        }
    }
}