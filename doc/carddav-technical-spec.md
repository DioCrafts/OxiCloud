# 27 - CardDAV Technical Spec

CardDAV (RFC 6352) enables contact synchronization across devices and applications. It extends WebDAV (RFC 4918) to manage address books and vCard-formatted contacts.

## Architecture

The CardDAV implementation follows the hexagonal architecture pattern:

```
┌───────────────────┐     ┌────────────────────┐     ┌────────────────────┐
│                   │     │                    │     │                    │
│  Interfaces       │     │  Application       │     │  Infrastructure    │
│  - CardDAV API    │────▶│  - Contact Service │────▶│  - Contact Repo    │
│  - Contact API    │     │  - CardDAV Adapter │     │  - PG Repository   │
│                   │     │                    │     │                    │
└───────────────────┘     └────────────────────┘     └────────────────────┘
                                    │
                                    ▼
                          ┌────────────────────┐
                          │                    │
                          │  Domain            │
                          │  - Contact Entity  │
                          │  - Address Book    │
                          │                    │
                          └────────────────────┘
```

### Components

1. **Domain Layer**
   - **Contact** entity -- name, email, phone, etc.
   - **AddressBook** entity -- a collection of contacts
   - Repository interfaces for contact management

2. **Application Layer**
   - **ContactService** -- business logic for managing contacts
   - **CardDAVAdapter** -- converts between CardDAV protocol requests/responses and domain objects

3. **Infrastructure Layer**
   - **ContactPgRepository** -- PostgreSQL implementation of contact repositories
   - **AddressBookPgRepository** -- PostgreSQL implementation of address book repositories

4. **Interface Layer**
   - REST API endpoints for contact management
   - CardDAV protocol endpoints (WebDAV extension)

## Database Schema

```sql
-- Address books table
CREATE TABLE IF NOT EXISTS carddav.address_books (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    owner_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    description TEXT,
    color VARCHAR(50),
    is_public BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(owner_id, name)
);

-- Address book sharing
CREATE TABLE IF NOT EXISTS carddav.address_book_shares (
    address_book_id UUID NOT NULL REFERENCES carddav.address_books(id) ON DELETE CASCADE,
    user_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    can_write BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY(address_book_id, user_id)
);

-- Contacts table
CREATE TABLE IF NOT EXISTS carddav.contacts (
    id UUID PRIMARY KEY,
    address_book_id UUID NOT NULL REFERENCES carddav.address_books(id) ON DELETE CASCADE,
    uid VARCHAR(255) NOT NULL,
    full_name VARCHAR(255),
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    nickname VARCHAR(255),
    email JSONB,
    phone JSONB,
    address JSONB,
    organization VARCHAR(255),
    title VARCHAR(255),
    notes TEXT,
    photo_url TEXT,
    birthday DATE,
    anniversary DATE,
    vcard TEXT NOT NULL,
    etag VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(address_book_id, uid)
);

-- Contact groups
CREATE TABLE IF NOT EXISTS carddav.contact_groups (
    id UUID PRIMARY KEY,
    address_book_id UUID NOT NULL REFERENCES carddav.address_books(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Group memberships
CREATE TABLE IF NOT EXISTS carddav.group_memberships (
    group_id UUID NOT NULL REFERENCES carddav.contact_groups(id) ON DELETE CASCADE,
    contact_id UUID NOT NULL REFERENCES carddav.contacts(id) ON DELETE CASCADE,
    PRIMARY KEY(group_id, contact_id)
);
```

## API Endpoints

### REST API

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/address-books` | List all address books |
| POST | `/api/address-books` | Create a new address book |
| GET | `/api/address-books/:id` | Get a specific address book |
| PUT | `/api/address-books/:id` | Update an address book |
| DELETE | `/api/address-books/:id` | Delete an address book |
| GET | `/api/address-books/:id/contacts` | List contacts in an address book |
| POST | `/api/address-books/:id/contacts` | Create a new contact |
| GET | `/api/address-books/:id/contacts/:contactId` | Get a specific contact |
| PUT | `/api/address-books/:id/contacts/:contactId` | Update a contact |
| DELETE | `/api/address-books/:id/contacts/:contactId` | Delete a contact |
| GET | `/api/address-books/:id/groups` | List contact groups |
| POST | `/api/address-books/:id/groups` | Create a new contact group |

### CardDAV Protocol Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| PROPFIND | `/carddav/` | List all address books |
| PROPFIND | `/carddav/:addressBookId/` | Get address book info |
| REPORT | `/carddav/:addressBookId/` | Query contacts in an address book |
| GET | `/carddav/:addressBookId/:contactId.vcf` | Get a specific contact (vCard) |
| PUT | `/carddav/:addressBookId/:contactId.vcf` | Create or update a contact |
| DELETE | `/carddav/:addressBookId/:contactId.vcf` | Delete a contact |
| MKCOL | `/carddav/:addressBookId/` | Create a new address book |
| DELETE | `/carddav/:addressBookId/` | Delete an address book |

## Data Model

### Contact Entity

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: Uuid,
    pub address_book_id: Uuid,
    pub uid: String,
    pub full_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub nickname: Option<String>,
    pub email: Vec<Email>,
    pub phone: Vec<Phone>,
    pub address: Vec<Address>,
    pub organization: Option<String>,
    pub title: Option<String>,
    pub notes: Option<String>,
    pub photo_url: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub anniversary: Option<NaiveDate>,
    pub vcard: String,
    pub etag: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    pub email: String,
    pub r#type: String, // home, work, other
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phone {
    pub number: String,
    pub r#type: String, // mobile, home, work, fax, other
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub r#type: String, // home, work, other
    pub is_primary: bool,
}
```

### AddressBook Entity

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressBook {
    pub id: Uuid,
    pub name: String,
    pub owner_id: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## Repositories

### ContactRepository Interface

```rust
#[async_trait]
pub trait ContactRepository: Send + Sync + 'static {
    async fn create_contact(&self, contact: Contact) -> ContactRepositoryResult<Contact>;
    async fn update_contact(&self, contact: Contact) -> ContactRepositoryResult<Contact>;
    async fn delete_contact(&self, id: &Uuid) -> ContactRepositoryResult<()>;
    async fn get_contact_by_id(&self, id: &Uuid) -> ContactRepositoryResult<Option<Contact>>;
    async fn get_contact_by_uid(&self, address_book_id: &Uuid, uid: &str) -> ContactRepositoryResult<Option<Contact>>;
    async fn get_contacts_by_address_book(&self, address_book_id: &Uuid) -> ContactRepositoryResult<Vec<Contact>>;
    async fn get_contacts_by_email(&self, email: &str) -> ContactRepositoryResult<Vec<Contact>>;
    async fn get_contacts_by_group(&self, group_id: &Uuid) -> ContactRepositoryResult<Vec<Contact>>;
}
```

### AddressBookRepository Interface

```rust
#[async_trait]
pub trait AddressBookRepository: Send + Sync + 'static {
    async fn create_address_book(&self, address_book: AddressBook) -> AddressBookRepositoryResult<AddressBook>;
    async fn update_address_book(&self, address_book: AddressBook) -> AddressBookRepositoryResult<AddressBook>;
    async fn delete_address_book(&self, id: &Uuid) -> AddressBookRepositoryResult<()>;
    async fn get_address_book_by_id(&self, id: &Uuid) -> AddressBookRepositoryResult<Option<AddressBook>>;
    async fn get_address_books_by_owner(&self, owner_id: &str) -> AddressBookRepositoryResult<Vec<AddressBook>>;
    async fn get_shared_address_books(&self, user_id: &str) -> AddressBookRepositoryResult<Vec<AddressBook>>;
    async fn get_public_address_books(&self) -> AddressBookRepositoryResult<Vec<AddressBook>>;
    async fn share_address_book(&self, address_book_id: &Uuid, user_id: &str, can_write: bool) -> AddressBookRepositoryResult<()>;
    async fn unshare_address_book(&self, address_book_id: &Uuid, user_id: &str) -> AddressBookRepositoryResult<()>;
    async fn get_address_book_shares(&self, address_book_id: &Uuid) -> AddressBookRepositoryResult<Vec<(String, bool)>>;
}
```

## CardDAV Protocol Features

1. **Address Book Discovery** -- clients discover available address books
2. **Address Book Collection** -- manage contacts within address books
3. **vCard Support** -- store and retrieve contacts in vCard format (3.0 and 4.0)
4. **Query Support** -- filter contacts by properties
5. **Multiget Support** -- retrieve multiple contacts in a single request
6. **Sync-Collection** -- efficient incremental synchronization

### CardDAV Adapter

The **CardDAVAdapter** handles:

1. Parsing CardDAV XML requests
2. Converting between vCard and **Contact** entities
3. Generating CardDAV XML responses
4. Supporting **PROPFIND**, **REPORT**, and other WebDAV methods
5. Implementing proper WebDAV properties for CardDAV

## Integration Points

1. **Authentication** -- reuses existing auth mechanisms
2. **WebDAV Infrastructure** -- extends the existing WebDAV implementation
3. **Database Layer** -- stores contacts in PostgreSQL
4. **User Management** -- connects contacts with user accounts

## Client Compatibility

Target clients:

- Apple Contacts
- Google Contacts
- Thunderbird
- Outlook
- Android DAVx5
- iOS native contacts app
- Evolution

See `dav-client-setup.md` for detailed connection instructions.

## Implementation Phases

### Phase 1: Core Infrastructure
- Database schema creation
- Entity definitions
- Repository interfaces
- Basic DTO and port definitions

### Phase 2: Core Business Logic
- Address book management service
- Contact management service
- vCard parsing and generation

### Phase 3: REST API
- Address book endpoints
- Contact management endpoints
- Contact group endpoints

### Phase 4: CardDAV Protocol
- **CardDAVAdapter** implementation
- WebDAV method handlers
- XML parsing and generation
- Protocol compliance testing

### Phase 5: Testing and Refinement
- Integration testing with client applications
- Performance optimization
- Edge case handling

## Security

1. **Authentication** -- proper authentication for all operations
2. **Authorization** -- verify permissions for each address book operation
3. **Data Validation** -- validate vCard input to prevent injection attacks
4. **Resource Limits** -- limits to prevent abuse
5. **Error Handling** -- appropriate error responses without leaking sensitive data

## Performance

1. **Indexing** -- proper database indexes for contact queries
2. **Caching** -- cache frequently accessed address books and contacts
3. **Pagination** -- support pagination for large address books
4. **Incremental Sync** -- efficient sync with client devices
5. **ETags** -- prevent unnecessary data transfers

## Testing Strategy

1. **Unit Tests** -- test individual components in isolation
2. **Integration Tests** -- test component interactions
3. **Protocol Compliance Tests** -- verify adherence to the CardDAV spec (RFC 6352)
4. **Client Compatibility Tests** -- test with various CardDAV clients
5. **Performance Tests** -- measure performance with large address books
