# WOPI (Office Document Editing)

OxiCloud integrates with **Collabora Online** and **OnlyOffice** via the WOPI protocol, letting users edit documents, spreadsheets, and presentations directly in the browser.

## How It Works

1. User opens a document (`.docx`, `.xlsx`, `.pptx`, `.odt`, etc.)
2. OxiCloud generates a WOPI access token and redirects to the editor
3. The editor fetches the file from OxiCloud via WOPI endpoints
4. Edits are saved back via `PutFile`

## Host / Client Flow

OxiCloud acts as the **WOPI host** and Collabora or OnlyOffice acts as the **WOPI client**.

1. OxiCloud loads the discovery document from the configured WOPI client
2. The frontend opens a host page that embeds the editor in an iframe
3. The iframe URL includes `WOPISrc`, which points back to OxiCloud's `/wopi/files/*` endpoints
4. The editor calls back into OxiCloud with the WOPI access token to read, lock, and save the file

## Configuration

```bash
OXICLOUD_WOPI_ENABLED=true
OXICLOUD_WOPI_DISCOVERY_URL="http://collabora:9980/hosting/discovery"
```

| Variable | Default | Description |
|---|---|---|
| `OXICLOUD_WOPI_ENABLED` | `false` | Enable WOPI integration |
| `OXICLOUD_WOPI_DISCOVERY_URL` | — | Editor's WOPI discovery URL |
| `OXICLOUD_WOPI_SECRET` | (JWT secret) | Token signing key |
| `OXICLOUD_WOPI_TOKEN_TTL_SECS` | `86400` | Access token lifetime |
| `OXICLOUD_WOPI_LOCK_TTL_SECS` | `1800` | Lock expiration |

## Docker Compose with Collabora

```yaml
services:
  collabora:
    image: collabora/code:latest
    environment:
      - domain=oxicloud\\.example\\.com
      - extra_params=--o:ssl.enable=false
    ports:
      - "9980:9980"
    cap_add:
      - MKNOD

  oxicloud:
    environment:
      OXICLOUD_WOPI_ENABLED: "true"
      OXICLOUD_WOPI_DISCOVERY_URL: "http://collabora:9980/hosting/discovery"
```

## Docker Compose with OnlyOffice

```yaml
services:
  onlyoffice:
    image: onlyoffice/documentserver:latest
    environment:
      - JWT_ENABLED=false
    ports:
      - "8443:443"

  oxicloud:
    environment:
      OXICLOUD_WOPI_ENABLED: "true"
      OXICLOUD_WOPI_DISCOVERY_URL: "http://onlyoffice/hosting/discovery"
```

## WOPI Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/wopi/files/{id}` | CheckFileInfo — file metadata |
| GET | `/wopi/files/{id}/contents` | GetFile — download file content |
| POST | `/wopi/files/{id}/contents` | PutFile — save edited content |
| POST | `/wopi/files/{id}` | Lock / Unlock / RefreshLock / Rename / Delete / Save As |

### Common `X-WOPI-Override` operations

| Override | Purpose |
| --- | --- |
| `LOCK` | Acquire or refresh an editor lock |
| `UNLOCK` | Release a lock |
| `REFRESH_LOCK` | Extend the current lock |
| `PUT_RELATIVE` | Save as a related file |
| `RENAME_FILE` | Rename from inside the editor |
| `DELETE` | Delete from the editor when supported |

## Notes

- `CheckFileInfo` is required for every WOPI action
- `PutFile` uploads the full file content back to OxiCloud
- lock conflicts return `409 Conflict` with the current lock value
- OxiCloud uses query parameter `?access_token=` authentication for WOPI callbacks instead of the standard JWT middleware

## Supported Formats

Any format supported by your Collabora or OnlyOffice installation, typically:
- Documents: `.docx`, `.odt`, `.doc`, `.rtf`
- Spreadsheets: `.xlsx`, `.ods`, `.xls`, `.csv`
- Presentations: `.pptx`, `.odp`, `.ppt`
