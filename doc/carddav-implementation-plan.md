# 28 - CardDAV Implementation Plan

CardDAV is an open protocol for syncing address books and contacts across applications and devices. This plan breaks the implementation into five phases over five weeks.

## Implementation Roadmap

### Phase 1: Core Infrastructure (Week 1)

**Database Schema** -- new migration for CardDAV tables:
- **address_books** -- stores address book collections
- **contacts** -- stores contact data
- **address_book_shares** -- sharing between users
- **contact_groups** -- organizing contacts into groups
- **group_memberships** -- associating contacts with groups

**Domain Layer** -- entity models:
- **Contact** -- core contact entity
- **AddressBook** -- collection entity
- **ContactGroup** -- grouping contacts

Repository interfaces:
- **ContactRepository** -- contact CRUD
- **AddressBookRepository** -- address book management
- **ContactGroupRepository** -- group management

**Testing** -- unit tests for entity models and repository interface contract tests.

### Phase 2: Infrastructure Layer (Week 2)

**Repository Implementations** -- PostgreSQL:
- **ContactPgRepository**
- **AddressBookPgRepository**
- **ContactGroupPgRepository**

Also implement vCard parsing/generation utilities and any data migration tools needed.

**Integration** -- update DI system to include new repositories and connect with existing auth system.

**Testing** -- repository implementation tests, vCard parsing/generation tests, integration tests with the database.

### Phase 3: Application Layer (Week 3)

**Services** -- business logic:
- **ContactService** -- contact management
- **AddressBookService** -- address book management
- **ContactGroupService** -- group management

**DTOs and Ports** -- create DTOs for contact operations, define service interface ports, implement request/response mapping.

**CardDAV Adapter** -- protocol translation adapter with vCard conversion logic and XML parsing/generation utilities.

**Testing** -- service unit tests and integration tests for the adapter.

### Phase 4: Interface Layer (Week 4)

**REST API** -- endpoints for address book operations, contact management, contact groups. Document with OpenAPI.

**CardDAV Protocol Endpoints** -- WebDAV method handlers:
- PROPFIND -- discovery and property retrieval
- REPORT -- querying contacts
- MKCOL -- creating address books
- GET/PUT/DELETE -- contact operations
- CardDAV-specific XML handling

**Integration** -- connect all layers, end-to-end testing, test with various CardDAV clients.

**Testing** -- API endpoint tests, CardDAV protocol compliance tests, client compatibility tests.

### Phase 5: Refinement and Optimization (Week 5)

**Performance** -- caching for frequently accessed resources, query optimization, efficient sync mechanisms.

**Security** -- review auth, validate I/O, add rate limiting.

**Final Testing** -- stress testing with large address books, security testing, user acceptance testing.

**Documentation** -- update API docs, user guides, client setup procedures.

## Technical Specifications

### Database Schema

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

### API Endpoints

#### REST API

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

#### CardDAV Protocol Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| PROPFIND | `/carddav/` | List all address books |
| PROPFIND | `/carddav/:addressBookId/` | Get address book information |
| REPORT | `/carddav/:addressBookId/` | Query contacts in an address book |
| GET | `/carddav/:addressBookId/:contactId.vcf` | Get a specific contact (vCard) |
| PUT | `/carddav/:addressBookId/:contactId.vcf` | Create or update a contact |
| DELETE | `/carddav/:addressBookId/:contactId.vcf` | Delete a contact |

### Dependencies

- vCard parsing/generation library (e.g., `vcard-rs` or similar)
- XML processing (for CardDAV protocol)
- Database access (PostgreSQL)
- WebDAV base functionality

## Resources Required

- Developer time: 1 full-time developer for 5 weeks
- Testing resources: multiple CardDAV clients (Apple Contacts, Thunderbird, Android)
- Server resources: test environment with PostgreSQL

## Success Criteria

1. Users can create, update, and delete address books
2. Contacts can be managed within address books
3. Address books can be shared between users
4. Standard CardDAV clients can sync with the server
5. Acceptable performance with large address books (1000+ contacts)
6. Security measures properly implemented

## Client Setup Guides

After implementation, setup guides will be created for:

- Apple Contacts (macOS/iOS)
- Thunderbird/Evolution
- Android (using DAVx5)
- Other common CardDAV clients

See `dav-client-setup.md` for general DAV client configuration.

## Future Enhancements

1. Advanced contact search
2. Contact merging for duplicate detection
3. Bulk import/export
4. Contact photo management
5. Extended fields for specialized contact info
6. Integration with external systems (e.g., LDAP directories)
