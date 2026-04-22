# WebDAV

OxiCloud exposes a fully RFC 4918 compliant WebDAV interface at `/webdav/`. It works with all major file managers and sync clients.

## Base URL

```
https://your-server:8086/webdav/
```

## Authentication

HTTP Basic Authentication:

```
Authorization: Basic base64(username:password)
```

::: tip
Always use HTTPS in production — Basic auth sends credentials in every request.
:::

## Supported Methods

| Method | Description |
|--------|-------------|
| `PROPFIND` | List directory contents / get file properties |
| `GET` | Download a file |
| `PUT` | Upload a file |
| `MKCOL` | Create a folder |
| `MOVE` | Move or rename a file/folder |
| `COPY` | Copy a file/folder |
| `DELETE` | Delete a file/folder |
| `LOCK` / `UNLOCK` | File locking |

## Common Operations

### List a directory

Use `PROPFIND` with a `Depth` header:

```http
PROPFIND /webdav/projects/ HTTP/1.1
Depth: 1
Content-Type: application/xml

<?xml version="1.0" encoding="utf-8" ?>
<D:propfind xmlns:D="DAV:">
  <D:allprop/>
</D:propfind>
```

Successful directory listings return `207 Multi-Status`.

### Download a file

```http
GET /webdav/projects/document.pdf HTTP/1.1
Authorization: Basic base64(username:password)
```

### Upload or replace a file

```http
PUT /webdav/projects/document.pdf HTTP/1.1
Content-Type: application/pdf

<file bytes>
```

### Create a folder

```http
MKCOL /webdav/projects/new-folder HTTP/1.1
```

### Move or copy

```http
MOVE /webdav/old-location.pdf HTTP/1.1
Destination: https://your-server/webdav/new-location.pdf
```

```http
COPY /webdav/original.pdf HTTP/1.1
Destination: https://your-server/webdav/copy.pdf
```

### Delete a resource

```http
DELETE /webdav/projects/document.pdf HTTP/1.1
```

## Client Setup

### Windows Explorer

1. Open **This PC** → **Map network drive**
2. Enter: `https://your-server:8086/webdav/`
3. Check **Connect using different credentials**
4. Enter your OxiCloud username and password

### macOS Finder

1. **Go** → **Connect to Server** (⌘K)
2. Enter: `https://your-server:8086/webdav/`
3. Enter credentials when prompted

### Linux (Nautilus / Files)

1. Open Files → **Other Locations**
2. In the address bar, type: `davs://your-server:8086/webdav/`
3. Enter credentials

### Linux (Dolphin / KDE)

1. In the address bar, type: `webdavs://your-server:8086/webdav/`

### Command Line (curl)

```bash
# List root directory
curl -u user:pass -X PROPFIND https://your-server:8086/webdav/ \
  -H "Depth: 1"

# Download a file
curl -u user:pass https://your-server:8086/webdav/document.pdf -o document.pdf

# Upload a file
curl -u user:pass -T localfile.txt https://your-server:8086/webdav/remotefile.txt

# Create a folder
curl -u user:pass -X MKCOL https://your-server:8086/webdav/new-folder/
```

## Streaming PROPFIND

OxiCloud streams PROPFIND responses, so listing directories with thousands of files doesn't consume excessive memory.

## Integration Notes

- the WebDAV handler is only an HTTP adapter; file and folder operations still go through the same application services used by the REST API
- HTTP Basic Authentication is supported for DAV clients, while authorization rules remain the same as the rest of OxiCloud
- delete operations integrate with trash when the trash feature is enabled

## Troubleshooting

- Always use the `/webdav/` base path
- Prefer HTTPS because WebDAV uses Basic Authentication
- On Windows, make sure the `WebClient` service is enabled
- OxiCloud rejects path traversal segments such as `.` and `..` at the HTTP boundary
