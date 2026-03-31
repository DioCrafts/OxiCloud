# 32 - Authentication

The auth system uses JWT (JSON Web Tokens) with Argon2id password hashing. Features include registration, login, role-based access control (Admin/User), token refresh, storage quotas, and file/folder ownership.

## API Endpoints

All auth endpoints live under `/api/auth`:

- **POST /api/auth/register** -- register a new user
- **POST /api/auth/login** -- login and get tokens
- **POST /api/auth/refresh** -- refresh access token
- **GET /api/auth/me** -- get current user info
- **PUT /api/auth/change-password** -- change user password
- **POST /api/auth/logout** -- logout and invalidate refresh token
- **GET /api/auth/status** -- system status (auth enabled, OIDC enabled, etc.)
- **GET /api/auth/oidc/providers** -- list available OIDC providers
- **GET /api/auth/oidc/authorize** -- generate OIDC authorization URL
- **GET /api/auth/oidc/callback** -- receive OIDC callback redirect
- **POST /api/auth/oidc/exchange** -- exchange authorization code for tokens

## Request/Response Examples

### Register

**Request:**
```json
POST /api/auth/register
{
  "username": "testuser",
  "email": "test@example.com",
  "password": "SecurePassword123"
}
```

**Response:**
```json
201 Created
{
  "userId": "d290f1ee-6c54-4b01-90e6-d701748f0851",
  "username": "testuser",
  "email": "test@example.com"
}
```

### Login

**Request:**
```json
POST /api/auth/login
{
  "username": "testuser",
  "password": "SecurePassword123"
}
```

**Response:**
```json
200 OK
{
  "accessToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refreshToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expiresIn": 3600
}
```

### Refresh Token

**Request:**
```json
POST /api/auth/refresh
{
  "refreshToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Response:**
```json
200 OK
{
  "accessToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refreshToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expiresIn": 3600
}
```

### Get Current User

**Request:**
```
GET /api/auth/me
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Response:**
```json
200 OK
{
  "id": "d290f1ee-6c54-4b01-90e6-d701748f0851",
  "username": "testuser",
  "email": "test@example.com",
  "role": "user",
  "storageQuota": 10737418240,
  "storageUsed": 1048576,
  "createdAt": "2023-01-01T12:00:00Z"
}
```

### Change Password

**Request:**
```json
PUT /api/auth/change-password
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
{
  "oldPassword": "SecurePassword123",
  "newPassword": "NewSecurePassword456"
}
```

**Response:**
```
200 OK
```

### Logout

**Request:**
```
POST /api/auth/logout
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Response:**
```
200 OK
```

## Testing

1. Start PostgreSQL and create the database:
   ```bash
   createdb oxicloud
   psql -d oxicloud -f db/schema.sql
   ```

2. Set environment variables:
   ```bash
   source test-auth-env.sh
   ```

3. Start the server:
   ```bash
   cargo run
   ```

4. Run the auth test script:
   ```bash
   ./test-auth-api.sh
   ```

## Database Schema

The auth system uses these tables in the **auth** schema:

- **auth.users** -- user info (includes **oidc_provider** and **oidc_subject** columns for OIDC users)
- **auth.sessions** -- refresh token sessions
- **auth.user_files** -- file ownership (user_id, file_path, file_id, size_bytes)
- **auth.user_favorites** -- user favorites (user_id, item_id, item_type)
- **auth.user_recent_files** -- recently accessed files (user_id, item_id, item_type, accessed_at)
- **auth.admin_settings** -- admin settings (key-value with category and secret flag)

## Implementation Details

- **Password hashing**: Argon2id with memory cost 65536 (64MB), time cost 3, parallelism 4
- **JWT secret**: configured via **OXICLOUD_JWT_SECRET** environment variable
- **Token expiry**: access token 1 hour, refresh token 30 days (configurable)
- **Database connection**: PostgreSQL with connection pooling
- **Middleware**: auth middleware for protected routes

## Security

- Passwords stored only as Argon2id hashes, never in plain text
- JWT tokens signed with a secret key
- Refresh tokens can be revoked to force logout
- Rate limiting should be applied to login attempts
- Password policy requires at least 8 characters

See `oidc-integration.md` for OIDC/SSO authentication details.

## Future Work

- Email verification for new registrations
- Password reset functionality
- Enhanced password policy
- Two-factor authentication
- OAuth integration for social logins
- Session management UI
