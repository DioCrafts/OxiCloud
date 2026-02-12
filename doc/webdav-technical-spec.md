# 21 - WebDAV Technical Spec

OxiCloud's WebDAV implementation follows RFC 4918. Clients can perform file operations over HTTP, so desktop apps, mobile clients, and any WebDAV-compatible software can treat OxiCloud as a remote filesystem.

Supported operations: file/folder browsing, uploads, downloads, create/delete/move resources, metadata retrieval and modification.

## Architecture

The WebDAV layer follows the hexagonal architecture pattern used throughout the codebase:

```
┌────────────────────────────────────────────────────────────────────┐
│                           INTERFACES                               │
│                                                                    │
│  ┌───────────────────────────────────────────────────────────┐    │
│  │                      WebDAV Handler                       │    │
│  │                                                           │    │
│  │  OPTIONS │ PROPFIND │ GET │ PUT │ DELETE │ MOVE │ COPY   │    │
│  └─────────────────────────────┬─────────────────────────────┘    │
│                                 │                                  │
└─────────────────────────────────┼──────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│                            APPLICATION                              │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                       WebDAV Adapter                        │   │
│  │                                                             │   │
│  │   XML Processing │ Protocol Translation │ DTOs Conversion   │   │
│  └──────────────────────────────┬──────────────────────────────┘   │
│                                 │                                   │
│                                 ▼                                   │
│                                                                     │
│  ┌──────────────┐  ┌───────────────┐  ┌──────────────┐  ┌───────┐  │
│  │              │  │               │  │              │  │       │  │
│  │ FileService  │  │ FolderService │  │  AuthService │  │ Other │  │
│  │              │  │               │  │              │  │       │  │
│  └──────┬───────┘  └───────┬───────┘  └──────┬───────┘  └───┬───┘  │
│         │                  │                 │              │      │
└─────────┼──────────────────┼─────────────────┼──────────────┼──────┘
          │                  │                 │              │
          ▼                  ▼                 ▼              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                             DOMAIN                                  │
│                                                                     │
│   ┌──────────┐  ┌───────────┐  ┌───────────┐  ┌────────┐  ┌──────┐ │
│   │          │  │           │  │           │  │        │  │      │ │
│   │   File   │  │  Folder   │  │   User    │  │ Share  │  │ etc. │ │
│   │          │  │           │  │           │  │        │  │      │ │
│   └──────────┘  └───────────┘  └───────────┘  └────────┘  └──────┘ │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Key Components

1. **WebDAV Handler** (`src/interfaces/api/handlers/webdav_handler.rs`) -- processes HTTP requests for WebDAV methods, maps operations to service calls, manages response formatting.

2. **WebDAV Adapter** (`src/application/adapters/webdav_adapter.rs`) -- converts between WebDAV XML and domain objects, parses PROPFIND/PROPPATCH requests, generates XML responses.

## Data Flow

A typical WebDAV request flows through:

1. Client sends a WebDAV request (e.g., PROPFIND for directory listing)
2. **webdav_handler.rs** receives and authenticates
3. Handler identifies the operation type, passes to **WebDavAdapter**
4. Adapter parses XML, converts to domain objects
5. Handler calls appropriate service methods (e.g., **folder_service.list_folders()**)
6. Domain operations execute via existing services
7. Results go back to the adapter for XML formatting
8. Handler returns the response with proper HTTP headers

## Integration with OxiCloud

### File Operations
Uses **FileService** for uploads, downloads, and management.

### Folder Operations
Uses **FolderService** for directory listing and manipulation. Maintains consistent behavior with the REST API.

### Authentication
Same auth mechanisms as the rest of OxiCloud. Supports HTTP Basic Authentication for WebDAV clients.

### Trash Integration
Integrates with the trash system for file/folder deletion. WebDAV operations use the trash feature when available.

## Request Processing

### PROPFIND (Directory Listing)

```
┌─────────┐     ┌────────────────┐     ┌─────────────────┐     ┌───────────────┐
│         │     │                │     │                 │     │               │
│ Client  │────▶│ WebDAV Handler │────▶│ WebDAV Adapter  │────▶│ FolderService │
│         │     │                │     │                 │     │               │
└─────────┘     └────────────────┘     └─────────────────┘     └───────┬───────┘
                                                                       │
┌─────────┐     ┌────────────────┐     ┌─────────────────┐     ┌───────▼───────┐
│         │     │                │     │                 │     │               │
│ Client  │◀────│ WebDAV Handler │◀────│ WebDAV Adapter  │◀────│ FileService   │
│         │     │                │     │                 │     │               │
└─────────┘     └────────────────┘     └─────────────────┘     └───────────────┘
```

1. Client sends PROPFIND with Depth header
2. Handler extracts path and depth
3. Adapter parses XML to determine requested properties
4. **FolderService** retrieves folder contents
5. **FileService** retrieves file info if needed
6. Adapter generates XML response with all properties
7. Handler returns 207 Multi-Status

### PUT (File Upload)

```
┌─────────┐     ┌────────────────┐     ┌─────────────────┐
│         │     │                │     │                 │
│ Client  │────▶│ WebDAV Handler │────▶│ FileService     │
│         │     │                │     │                 │
└─────────┘     └────────────────┘     └─────────────────┘
                                                │
┌─────────┐     ┌────────────────┐     ┌────────▼────────┐
│         │     │                │     │                 │
│ Client  │◀────│ WebDAV Handler │◀────│ Response        │
│         │     │                │     │                 │
└─────────┘     └────────────────┘     └─────────────────┘
```

1. Client sends PUT with file contents
2. Handler extracts path and parent folder info
3. **FileService** uploads the file
4. Handler returns 201 Created or 204 No Content

## Security

- **Authentication** -- same mechanisms as the REST API. Supports HTTP Basic Auth for WebDAV clients. Same permissions model applies.
- **Authorization** -- users can only access their own files through WebDAV. Shared resources keep the same permissions.
- **HTTPS** -- all WebDAV traffic should be served over HTTPS.
- **Input validation** -- all XML inputs strictly validated. Path traversal prevented by proper path normalization.

## Extension Points

1. **Property storage** -- support for custom WebDAV properties via a property database.
2. **CalDAV/CardDAV** -- the architecture allows extending to CalDAV (calendar) and CardDAV (contacts), both built on the WebDAV foundation.
3. **Advanced locking** -- full WebDAV locking for collaborative editing.

## Implementation Status

| Method    | Status    | Notes                                    |
|-----------|-----------|------------------------------------------|
| OPTIONS   | Complete  | Advertises WebDAV capabilities           |
| PROPFIND  | Complete  | Full directory listing with properties   |
| GET       | Complete  | File download fully implemented          |
| HEAD      | Complete  | Metadata retrieval implemented           |
| PUT       | Complete  | File creation and update implemented     |
| DELETE    | Complete  | Integration with trash features          |
| MKCOL     | Complete  | Directory creation implemented           |
| COPY      | Complete  | File/folder copying implemented          |
| MOVE      | Complete  | File/folder moving/renaming implemented  |
| PROPPATCH | Complete  | Property updates implemented             |
| LOCK      | Complete  | Basic locking capability implemented     |
| UNLOCK    | Complete  | Basic unlocking capability implemented   |

All WebDAV methods required by RFC 4918 are implemented. The server is compatible with all standard clients. Persistent property storage may be added later.
