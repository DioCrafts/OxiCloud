# 23 - DAV Integration

WebDAV, CalDAV, and CardDAV extend the platform to support clients and devices that speak these standard protocols. The implementation follows the existing hexagonal architecture -- each protocol gets its own adapter, service, and domain layer.

## Table of Contents

1. [Introduction](#introduction)
2. [Implementation Architecture](#implementation-architecture)
3. [WebDAV](#webdav)
4. [CalDAV](#caldav)
5. [CardDAV](#carddav)
6. [Security Considerations](#security-considerations)
7. [Testing and Compatibility](#testing-and-compatibility)

## Introduction

### WebDAV (Web Distributed Authoring and Versioning)
An HTTP extension that lets clients create, modify, move, and delete files and directories on a remote server.

### CalDAV (Calendaring Extensions to WebDAV)
A WebDAV-based protocol for accessing and managing calendar data (events and tasks).

### CardDAV (vCard Extensions to WebDAV)
Extends WebDAV to allow access and management of contact data in vCard format.

## Implementation Architecture

DAV protocols plug into the existing hexagonal architecture:

```
┌────────────────────────────────────────────────────────────────────┐
│                          INTERFACES                                │
│                                                                    │
│  ┌───────────────┐  ┌───────────────┐  ┌───────────────────────┐  │
│  │               │  │               │  │                       │  │
│  │  REST API     │  │  WebDAV API   │  │  CalDAV/CardDAV API   │  │
│  │               │  │               │  │                       │  │
│  └───────┬───────┘  └───────┬───────┘  └───────────┬───────────┘  │
│          │                  │                      │              │
└──────────┼──────────────────┼──────────────────────┼──────────────┘
           │                  │                      │
           ▼                  ▼                      ▼
┌──────────────────────────────────────────────────────────────────┐
│                          APPLICATION                             │
│                                                                  │
│  ┌───────────┐  ┌────────────┐  ┌───────────┐  ┌──────────────┐ │
│  │           │  │            │  │           │  │              │ │
│  │FileService│  │FolderService│  │CalendarSvc│  │ContactService│ │
│  │           │  │            │  │           │  │              │ │
│  └─────┬─────┘  └──────┬─────┘  └─────┬─────┘  └──────┬───────┘ │
│        │               │              │               │         │
└────────┼───────────────┼──────────────┼───────────────┼─────────┘
         │               │              │               │
         ▼               ▼              ▼               ▼
┌────────────────────────────────────────────────────────────────┐
│                          DOMAIN                                │
│                                                                │
│  ┌─────────┐  ┌──────────┐  ┌────────────┐  ┌───────────────┐ │
│  │         │  │          │  │            │  │               │ │
│  │  File   │  │  Folder  │  │  Calendar  │  │    Contact    │ │
│  │         │  │          │  │            │  │               │ │
│  └─────────┘  └──────────┘  └────────────┘  └───────────────┘ │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

### Main Components

1. **DAV Adapters** -- convert between DAV specs and internal models
2. **Application Services** -- extended to include DAV-specific functionality
3. **Domain Models** -- new entities for **Calendar** and **Contact**
4. **Repositories** -- storage implementations for calendars and contacts

## WebDAV

### Required Endpoints

| HTTP Method | Endpoint | Description |
|-------------|----------|-------------|
| OPTIONS | /webdav/{path} | Reports supported WebDAV capabilities |
| PROPFIND | /webdav/{path} | Retrieves resource properties |
| PROPPATCH | /webdav/{path} | Modifies resource properties |
| MKCOL | /webdav/{path} | Creates collections (directories) |
| GET | /webdav/{path} | Retrieves resource content |
| HEAD | /webdav/{path} | Retrieves resource metadata |
| PUT | /webdav/{path} | Creates or updates resources |
| DELETE | /webdav/{path} | Deletes resources |
| COPY | /webdav/{path} | Copies resources |
| MOVE | /webdav/{path} | Moves resources |
| LOCK | /webdav/{path} | Locks resources |
| UNLOCK | /webdav/{path} | Unlocks resources |

### Implementation

1. **WebDAV Handler**:

```rust
// src/interfaces/api/handlers/webdav_handler.rs
use std::sync::Arc;
use axum::{
    Router,
    routing::get,
    extract::{Path, State, Request, Extension},
    http::StatusCode,
    response::Response,
};
use http::{Method, header};

use crate::common::di::AppState;
use crate::interfaces::middleware::auth::CurrentUser;
use crate::application::ports::file_ports::{FileRetrievalUseCase, FileUploadUseCase};
use crate::application::ports::folder_ports::FolderUseCase;
use crate::common::errors::AppError;

pub fn webdav_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/webdav/*path", get(handle_get))
        .route_with_tsr("/webdav/*path", axum::routing::on(
            Method::OPTIONS, handle_options,
            Method::PROPFIND, handle_propfind,
            Method::PROPPATCH, handle_proppatch,
            Method::MKCOL, handle_mkcol,
            Method::PUT, handle_put,
            Method::DELETE, handle_delete,
            Method::COPY, handle_copy,
            Method::MOVE, handle_move,
            Method::LOCK, handle_lock,
            Method::UNLOCK, handle_unlock,
        ))
}

// Implement functions for each WebDAV method...
```

2. **WebDAV Adapter**:

```rust
// src/application/adapters/webdav_adapter.rs
use xml::reader::{EventReader, XmlEvent};
use xml::writer::{EventWriter, EmitterConfig, XmlEvent as WriteEvent};
use std::io::{Read, Write};
use crate::application::dtos::file_dto::FileDto;
use crate::application::dtos::folder_dto::FolderDto;

/// Converts between internal objects and WebDAV representations
pub struct WebDavAdapter;

impl WebDavAdapter {
    /// Parses a PROPFIND XML property into a request object
    pub fn parse_propfind<R: Read>(reader: R) -> Result<PropFindRequest, Error> {
        // Implementation...
    }

    /// Generates PROPFIND XML response from files and folders
    pub fn generate_propfind_response<W: Write>(
        writer: W,
        files: &[FileDto],
        folders: &[FolderDto],
        base_url: &str,
    ) -> Result<(), Error> {
        // Implementation...
    }

    // Other methods for different WebDAV operations...
}
```

## CalDAV

### Required Endpoints

| HTTP Method | Endpoint | Description |
|-------------|----------|-------------|
| PROPFIND | /caldav/{calendar} | Retrieves calendar properties |
| REPORT | /caldav/{calendar} | Queries calendar events |
| MKCALENDAR | /caldav/{calendar} | Creates a new calendar |
| PUT | /caldav/{calendar}/{event}.ics | Creates or updates an event |
| GET | /caldav/{calendar}/{event}.ics | Retrieves an event |
| DELETE | /caldav/{calendar}/{event}.ics | Deletes an event |

### Implementation

1. **Domain Entities**:

```rust
// src/domain/entities/calendar.rs
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Calendar {
    id: Uuid,
    name: String,
    owner_id: String,
    description: Option<String>,
    color: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// src/domain/entities/calendar_event.rs
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct CalendarEvent {
    id: Uuid,
    calendar_id: Uuid,
    summary: String,
    description: Option<String>,
    location: Option<String>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    all_day: bool,
    rrule: Option<String>,  // Recurrence rule
    ical_data: String,      // Full iCalendar data
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

2. **Repositories**:

```rust
// src/domain/repositories/calendar_repository.rs
use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::entities::calendar::Calendar;
use crate::common::errors::Result;

#[async_trait]
pub trait CalendarRepository: Send + Sync {
    async fn create_calendar(&self, calendar: Calendar) -> Result<Calendar>;
    async fn get_calendar_by_id(&self, id: &Uuid) -> Result<Calendar>;
    async fn get_calendars_by_owner(&self, owner_id: &str) -> Result<Vec<Calendar>>;
    async fn update_calendar(&self, calendar: Calendar) -> Result<Calendar>;
    async fn delete_calendar(&self, id: &Uuid) -> Result<()>;
}

// src/domain/repositories/calendar_event_repository.rs
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::entities::calendar_event::CalendarEvent;
use crate::common::errors::Result;

#[async_trait]
pub trait CalendarEventRepository: Send + Sync {
    async fn create_event(&self, event: CalendarEvent) -> Result<CalendarEvent>;
    async fn get_event_by_id(&self, id: &Uuid) -> Result<CalendarEvent>;
    async fn get_events_by_calendar(&self, calendar_id: &Uuid) -> Result<Vec<CalendarEvent>>;
    async fn get_events_in_timerange(
        &self,
        calendar_id: &Uuid,
        start: &DateTime<Utc>,
        end: &DateTime<Utc>
    ) -> Result<Vec<CalendarEvent>>;
    async fn update_event(&self, event: CalendarEvent) -> Result<CalendarEvent>;
    async fn delete_event(&self, id: &Uuid) -> Result<()>;
}
```

3. **CalDAV Service**:

```rust
// src/application/services/calendar_service.rs
use std::sync::Arc;
use crate::application::ports::calendar_ports::CalendarStoragePort;
use crate::application::dtos::calendar_dto::*;

pub struct CalendarService {
    storage: Arc<dyn CalendarStoragePort>,
}

impl CalendarService {
    pub fn new(storage: Arc<dyn CalendarStoragePort>) -> Self {
        Self { storage }
    }

    // Implements CalendarUseCase for calendar and event operations...
}
```

4. **CalDAV Handler**:

```rust
// src/interfaces/api/handlers/caldav_handler.rs
use std::sync::Arc;
use axum::{
    Router,
    routing::{get, put, delete},
    extract::{Path, State, Request, Extension},
    http::StatusCode,
    response::Response,
};
use http::{Method, header};

use crate::common::di::AppState;
use crate::interfaces::middleware::auth::CurrentUser;
use crate::application::services::caldav_service::CalDavService;
use crate::common::errors::AppError;

pub fn caldav_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/caldav/", get(get_calendars))
        .route("/caldav/:calendar", get(get_calendar))
        .route_with_tsr("/caldav/:calendar", axum::routing::on(
            Method::PROPFIND, handle_calendar_propfind,
            Method::REPORT, handle_calendar_report,
            Method::MKCALENDAR, handle_mkcalendar,
        ))
        .route("/caldav/:calendar/:event", get(get_event))
        .route("/caldav/:calendar/:event", put(put_event))
        .route("/caldav/:calendar/:event", delete(delete_event))
}

// Implement functions for each CalDAV method...
```

## CardDAV

### Required Endpoints

| HTTP Method | Endpoint | Description |
|-------------|----------|-------------|
| PROPFIND | /carddav/addressbooks/{addressbook} | Retrieves address book properties |
| REPORT | /carddav/addressbooks/{addressbook} | Queries contacts |
| MKCOL | /carddav/addressbooks/{addressbook} | Creates a new address book |
| PUT | /carddav/addressbooks/{addressbook}/{contact}.vcf | Creates or updates a contact |
| GET | /carddav/addressbooks/{addressbook}/{contact}.vcf | Retrieves a contact |
| DELETE | /carddav/addressbooks/{addressbook}/{contact}.vcf | Deletes a contact |

### Implementation

1. **Domain Entities**:

```rust
// Note: AddressBook and Contact are both defined in src/domain/entities/contact.rs

#[derive(Debug, Clone)]
pub struct AddressBook {
    id: Uuid,
    name: String,
    owner_id: String,
    description: Option<String>,
    color: Option<String>,
    is_public: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Contact {
    id: Uuid,
    address_book_id: Uuid,
    uid: String,
    full_name: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    nickname: Option<String>,
    email: Vec<Email>,        // Struct with email, type, is_primary
    phone: Vec<Phone>,        // Struct with number, type, is_primary
    address: Vec<Address>,    // Struct with street, city, state, postal_code, country, type, is_primary
    organization: Option<String>,
    title: Option<String>,
    notes: Option<String>,
    photo_url: Option<String>,
    birthday: Option<NaiveDate>,
    anniversary: Option<NaiveDate>,
    vcard: String,            // Full vCard data
    etag: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

2. **Repositories**:

```rust
// src/domain/repositories/address_book_repository.rs
use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::entities::address_book::AddressBook;
use crate::common::errors::Result;

#[async_trait]
pub trait AddressBookRepository: Send + Sync {
    async fn create_address_book(&self, address_book: AddressBook) -> Result<AddressBook>;
    async fn get_address_book_by_id(&self, id: &Uuid) -> Result<AddressBook>;
    async fn get_address_books_by_owner(&self, owner_id: &str) -> Result<Vec<AddressBook>>;
    async fn update_address_book(&self, address_book: AddressBook) -> Result<AddressBook>;
    async fn delete_address_book(&self, id: &Uuid) -> Result<()>;
}

// src/domain/repositories/contact_repository.rs
use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::entities::contact::Contact;
use crate::common::errors::Result;

#[async_trait]
pub trait ContactRepository: Send + Sync {
    async fn create_contact(&self, contact: Contact) -> Result<Contact>;
    async fn get_contact_by_id(&self, id: &Uuid) -> Result<Contact>;
    async fn get_contacts_by_address_book(&self, address_book_id: &Uuid) -> Result<Vec<Contact>>;
    async fn search_contacts(&self, address_book_id: &Uuid, query: &str) -> Result<Vec<Contact>>;
    async fn update_contact(&self, contact: Contact) -> Result<Contact>;
    async fn delete_contact(&self, id: &Uuid) -> Result<()>;
}
```

3. **CardDAV Service**:

```rust
// src/application/services/contact_service.rs
use std::sync::Arc;
use crate::application::dtos::contact_dto::*;
use crate::application::dtos::address_book_dto::*;

pub struct ContactService {
    // Implements AddressBookUseCase and ContactUseCase
    // Uses ContactStorageAdapter as infrastructure
}

impl ContactService {
    // Implements methods for CardDAV operations...
}
```

4. **CardDAV Handler**:

```rust
// src/interfaces/api/handlers/carddav_handler.rs
use std::sync::Arc;
use axum::{
    Router,
    routing::{get, put, delete},
    extract::{Path, State, Request, Extension},
    http::StatusCode,
    response::Response,
};
use http::{Method, header};

use crate::common::di::AppState;
use crate::interfaces::middleware::auth::CurrentUser;
use crate::application::services::carddav_service::CardDavService;
use crate::common::errors::AppError;

pub fn carddav_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/carddav/addressbooks/", get(get_address_books))
        .route("/carddav/addressbooks/:addressbook", get(get_address_book))
        .route_with_tsr("/carddav/addressbooks/:addressbook", axum::routing::on(
            Method::PROPFIND, handle_addressbook_propfind,
            Method::REPORT, handle_addressbook_report,
            Method::MKCOL, handle_mkaddressbook,
        ))
        .route("/carddav/addressbooks/:addressbook/:contact", get(get_contact))
        .route("/carddav/addressbooks/:addressbook/:contact", put(put_contact))
        .route("/carddav/addressbooks/:addressbook/:contact", delete(delete_contact))
}

// Implement functions for each CardDAV method...
```

## Database Schema

```sql
-- CalDAV schema
CREATE SCHEMA IF NOT EXISTS caldav;

CREATE TABLE IF NOT EXISTS caldav.calendars (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    owner_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    description TEXT,
    color VARCHAR(50),
    is_public BOOLEAN NOT NULL DEFAULT FALSE,
    ctag VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS caldav.calendar_events (
    id UUID PRIMARY KEY,
    calendar_id UUID NOT NULL REFERENCES caldav.calendars(id) ON DELETE CASCADE,
    summary VARCHAR(255) NOT NULL,
    description TEXT,
    location TEXT,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    all_day BOOLEAN NOT NULL DEFAULT FALSE,
    rrule TEXT,
    ical_uid VARCHAR(255) NOT NULL,
    ical_data TEXT NOT NULL,
    etag VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS caldav.calendar_shares (
    id SERIAL PRIMARY KEY,
    calendar_id UUID NOT NULL REFERENCES caldav.calendars(id) ON DELETE CASCADE,
    user_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    access_level VARCHAR(20) NOT NULL DEFAULT 'read',
    UNIQUE(calendar_id, user_id)
);

CREATE TABLE IF NOT EXISTS caldav.calendar_properties (
    id SERIAL PRIMARY KEY,
    calendar_id UUID NOT NULL REFERENCES caldav.calendars(id) ON DELETE CASCADE,
    property_name VARCHAR(255) NOT NULL,
    property_value TEXT,
    UNIQUE(calendar_id, property_name)
);

-- CardDAV schema
CREATE SCHEMA IF NOT EXISTS carddav;

CREATE TABLE IF NOT EXISTS carddav.address_books (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    owner_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    description TEXT,
    color VARCHAR(50),
    is_public BOOLEAN NOT NULL DEFAULT FALSE,
    ctag VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(owner_id, name)
);

CREATE TABLE IF NOT EXISTS carddav.contacts (
    id UUID PRIMARY KEY,
    address_book_id UUID NOT NULL REFERENCES carddav.address_books(id) ON DELETE CASCADE,
    uid VARCHAR(255) NOT NULL,
    full_name VARCHAR(255),
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    nickname VARCHAR(255),
    organization VARCHAR(255),
    title VARCHAR(255),
    notes TEXT,
    photo_url TEXT,
    birthday DATE,
    anniversary DATE,
    email JSONB,
    phone JSONB,
    address JSONB,
    vcard TEXT NOT NULL,
    etag VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(address_book_id, uid)
);

CREATE TABLE IF NOT EXISTS carddav.address_book_shares (
    id SERIAL PRIMARY KEY,
    address_book_id UUID NOT NULL REFERENCES carddav.address_books(id) ON DELETE CASCADE,
    user_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    can_write BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE(address_book_id, user_id)
);

CREATE TABLE IF NOT EXISTS carddav.contact_groups (
    id UUID PRIMARY KEY,
    address_book_id UUID NOT NULL REFERENCES carddav.address_books(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS carddav.group_memberships (
    id SERIAL PRIMARY KEY,
    group_id UUID NOT NULL REFERENCES carddav.contact_groups(id) ON DELETE CASCADE,
    contact_id UUID NOT NULL REFERENCES carddav.contacts(id) ON DELETE CASCADE,
    UNIQUE(group_id, contact_id)
);

-- Indexes for efficient lookup
CREATE INDEX IF NOT EXISTS idx_calendars_owner ON caldav.calendars(owner_id);
CREATE INDEX IF NOT EXISTS idx_calendar_events_calendar ON caldav.calendar_events(calendar_id);
CREATE INDEX IF NOT EXISTS idx_address_books_owner ON carddav.address_books(owner_id);
CREATE INDEX IF NOT EXISTS idx_contacts_address_book ON carddav.contacts(address_book_id);
CREATE INDEX IF NOT EXISTS idx_contacts_full_name ON carddav.contacts(full_name);
```

## Security Considerations

1. **Authentication**
   - Uses existing authentication
   - Supports HTTP Basic Authentication for DAV clients
   - Digest authentication can be added if needed

2. **Authorization**
   - Verify user permissions before granting resource access
   - Owner-based and shared-permission access control
   - Users can only access their own calendars and address books

3. **Attack Prevention**
   - Validate and sanitize all XML input
   - Limit maximum payload size
   - Rate limiting on DAV endpoints

## Testing and Compatibility

### Clients to Test

1. **WebDAV**
   - Windows Explorer
   - macOS Finder
   - Cyberduck
   - FileZilla (with WebDAV extension)

2. **CalDAV**
   - Apple Calendar
   - Mozilla Thunderbird (Lightning)
   - Microsoft Outlook (with CalDAV add-in)
   - Google Calendar (via sync)

3. **CardDAV**
   - Apple Contacts
   - Mozilla Thunderbird
   - Microsoft Outlook (with CardDAV add-in)
   - Google Contacts (via sync)

### Compliance Testing

- Use the **CalDAVTester** suite to verify standards conformance
- Validate RFC compliance for each protocol
- Stress tests to evaluate performance under load

### Debugging

- Detailed logging for DAV operations
- Diagnostic tools for debugging complex DAV requests
- Clear error messages for troubleshooting
