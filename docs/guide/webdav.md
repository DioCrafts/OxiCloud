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
