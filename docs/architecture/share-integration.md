# Share Integration

OxiCloud supports public file and folder sharing through signed share links. A share can be public, password-protected, time-limited, or scoped by permissions.

## What a Share Contains

A share record tracks:

- The shared item ID and whether it is a file or folder
- A public token used in the share URL
- Optional password protection
- Optional expiration timestamp
- Permissions for read, write, and reshare
- The creator and access count

## Public and Private Routes

### Authenticated management routes

| Method | Path | Description |
| --- | --- | --- |
| `POST` | `/api/shares/` | Create a new share |
| `GET` | `/api/shares/` | List current user's shares |
| `GET` | `/api/shares/{id}` | Fetch one share |
| `PUT` | `/api/shares/{id}` | Update permissions, password, or expiration |
| `DELETE` | `/api/shares/{id}` | Delete a share |

### Public access routes

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/api/s/{token}` | Access a shared item |
| `POST` | `/api/s/{token}/verify` | Verify a password-protected share |

## Service Responsibilities

The share service is responsible for:

- Validating that the underlying file or folder exists
- Generating unique share IDs and public tokens
- Enforcing password checks and expiration rules
- Mapping domain permissions into API DTOs
- Recording access counts

Share metadata is persisted separately from the file content itself. The shared resource still uses the normal storage model for files and folders.

## Example Workflow

### Creating a share link

1. A user selects a file or folder in the UI
2. The frontend submits a request to `/api/shares/`
3. OxiCloud validates the target and requested permissions
4. The backend generates a token and public URL
5. The share metadata is saved and returned to the caller

### Opening a share link

1. A guest opens `/api/s/{token}`
2. OxiCloud verifies the token and checks expiration
3. If the share is password protected, the client verifies the password first
4. Access is counted and the shared resource is returned according to the granted permissions

## Security Notes

- Passwords are stored as hashes, never as plaintext
- Expired shares are rejected before content access
- Permissions are checked per action, not only when the share is created

## Related Pages

- [OIDC / SSO](/config/oidc)
- [Admin Settings](/config/admin-settings)
- [Internal Architecture](/architecture/)