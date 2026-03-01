-- ============================================================
-- Migration 003: OAuth 2.0 Device Authorization Grant (RFC 8628)
-- ============================================================
-- Adds the device_codes table to support the Device Authorization
-- Grant flow for WebDAV/CalDAV/CardDAV client authentication.
--
-- Flow:
--   1. Client POSTs to /api/auth/device/authorize → receives device_code + user_code
--   2. User opens verification_uri in browser, authenticates, enters user_code
--   3. Client polls /api/auth/device/token with device_code
--   4. Once approved, client receives access_token + refresh_token
-- ============================================================

-- Device code status enum
DO $BODY$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_type t
        JOIN pg_catalog.pg_namespace n ON n.oid = t.typnamespace
        WHERE t.typname = 'device_code_status' AND n.nspname = 'auth'
    ) THEN
        CREATE TYPE auth.device_code_status AS ENUM (
            'pending',      -- Waiting for user to authorize
            'authorized',   -- User approved, tokens ready for polling client
            'denied',       -- User denied the request
            'expired'       -- TTL exceeded without user action
        );
    END IF;
END $BODY$;

-- Device authorization codes table
CREATE TABLE IF NOT EXISTS auth.device_codes (
    -- Unique row ID
    id VARCHAR(36) PRIMARY KEY,

    -- RFC 8628 §3.2: device_code — long opaque token sent to the client for polling
    device_code VARCHAR(128) UNIQUE NOT NULL,

    -- RFC 8628 §3.2: user_code — short human-readable code shown on the client
    -- and entered by the user on the verification page (e.g. "ABCD-1234")
    user_code VARCHAR(16) UNIQUE NOT NULL,

    -- Name/description of the client requesting access (shown to user)
    client_name VARCHAR(255) NOT NULL DEFAULT 'Unknown Client',

    -- Comma-separated scopes requested (e.g. "webdav,caldav,carddav")
    scopes VARCHAR(512) NOT NULL DEFAULT 'webdav,caldav,carddav',

    -- Current status of the device flow
    status auth.device_code_status NOT NULL DEFAULT 'pending',

    -- User who authorized the request (NULL until status = 'authorized')
    user_id VARCHAR(36) REFERENCES auth.users(id) ON DELETE CASCADE,

    -- Tokens generated after authorization (NULL until status = 'authorized')
    -- Stored encrypted/hashed depending on sensitivity
    access_token TEXT,
    refresh_token TEXT,

    -- RFC 8628 §3.2: verification_uri — full URL the user must visit
    verification_uri TEXT NOT NULL,

    -- RFC 8628 §3.2: verification_uri_complete — URL with user_code pre-filled
    verification_uri_complete TEXT,

    -- RFC 8628 §3.2: expires_in — encoded as an absolute timestamp
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,

    -- RFC 8628 §3.2: interval — minimum polling interval in seconds
    poll_interval_secs INTEGER NOT NULL DEFAULT 5,

    -- Last time the client polled (for slow_down enforcement)
    last_poll_at TIMESTAMP WITH TIME ZONE,

    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    authorized_at TIMESTAMP WITH TIME ZONE
);

-- Index for client polling by device_code (hot path)
CREATE INDEX IF NOT EXISTS idx_device_codes_device_code
    ON auth.device_codes(device_code);

-- Index for user verification page lookup by user_code
CREATE INDEX IF NOT EXISTS idx_device_codes_user_code
    ON auth.device_codes(user_code)
    WHERE status = 'pending';

-- Index for cleanup of expired entries
CREATE INDEX IF NOT EXISTS idx_device_codes_expires_at
    ON auth.device_codes(expires_at)
    WHERE status = 'pending';

-- Index for user's authorized devices
CREATE INDEX IF NOT EXISTS idx_device_codes_user_id
    ON auth.device_codes(user_id)
    WHERE status = 'authorized';

COMMENT ON TABLE auth.device_codes IS 'OAuth 2.0 Device Authorization Grant (RFC 8628) codes for DAV client authentication';
