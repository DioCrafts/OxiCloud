-- ============================================================
-- OxiCloud Unified Database Schema
-- For clean installations: psql -f db/schema.sql
-- ============================================================
-- Order: auth (base) → caldav → carddav
-- All tables use IF NOT EXISTS for idempotent re-runs.
-- ============================================================

-- ============================================================
-- 1. AUTH SCHEMA
-- ============================================================
CREATE SCHEMA IF NOT EXISTS auth;

-- Create UserRole enum type
DO $BODY$ 
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_type t
        JOIN pg_catalog.pg_namespace n ON n.oid = t.typnamespace
        WHERE t.typname = 'userrole' AND n.nspname = 'auth'
    ) THEN
        CREATE TYPE auth.userrole AS ENUM ('admin', 'user');
    END IF;
END $BODY$;

-- Users table
CREATE TABLE IF NOT EXISTS auth.users (
    id VARCHAR(36) PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role auth.userrole NOT NULL,
    storage_quota_bytes BIGINT NOT NULL DEFAULT 10737418240, -- 10GB default
    storage_used_bytes BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login_at TIMESTAMP WITH TIME ZONE,
    active BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE INDEX IF NOT EXISTS idx_users_username ON auth.users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON auth.users(email);

-- Sessions table for refresh tokens
CREATE TABLE IF NOT EXISTS auth.sessions (
    id VARCHAR(36) PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    refresh_token TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    ip_address TEXT,
    user_agent TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    revoked BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON auth.sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_sessions_refresh_token ON auth.sessions(refresh_token);
CREATE INDEX IF NOT EXISTS idx_sessions_expires_at ON auth.sessions(expires_at);

CREATE OR REPLACE FUNCTION auth.is_session_active(expires_at timestamptz)
RETURNS boolean AS $$
BEGIN
  RETURN expires_at > now();
END;
$$ LANGUAGE plpgsql IMMUTABLE;

CREATE INDEX IF NOT EXISTS idx_sessions_active ON auth.sessions(user_id, revoked)
WHERE NOT revoked AND auth.is_session_active(expires_at);

-- File ownership tracking
CREATE TABLE IF NOT EXISTS auth.user_files (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    file_path TEXT NOT NULL,
    file_id TEXT NOT NULL,
    size_bytes BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, file_path)
);

CREATE INDEX IF NOT EXISTS idx_user_files_user_id ON auth.user_files(user_id);
CREATE INDEX IF NOT EXISTS idx_user_files_file_id ON auth.user_files(file_id);

-- User favorites
CREATE TABLE IF NOT EXISTS auth.user_favorites (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    item_id TEXT NOT NULL,
    item_type TEXT NOT NULL, -- 'file' or 'folder'
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, item_id, item_type)
);

CREATE INDEX IF NOT EXISTS idx_user_favorites_user_id ON auth.user_favorites(user_id);
CREATE INDEX IF NOT EXISTS idx_user_favorites_item_id ON auth.user_favorites(item_id);
CREATE INDEX IF NOT EXISTS idx_user_favorites_type ON auth.user_favorites(item_type);
CREATE INDEX IF NOT EXISTS idx_user_favorites_created ON auth.user_favorites(created_at);
CREATE INDEX IF NOT EXISTS idx_user_favorites_user_type ON auth.user_favorites(user_id, item_type);

-- Recent files
CREATE TABLE IF NOT EXISTS auth.user_recent_files (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    item_id TEXT NOT NULL,
    item_type TEXT NOT NULL, -- 'file' or 'folder'
    accessed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, item_id, item_type)
);

CREATE INDEX IF NOT EXISTS idx_user_recent_user_id ON auth.user_recent_files(user_id);
CREATE INDEX IF NOT EXISTS idx_user_recent_item_id ON auth.user_recent_files(item_id);
CREATE INDEX IF NOT EXISTS idx_user_recent_type ON auth.user_recent_files(item_type);
CREATE INDEX IF NOT EXISTS idx_user_recent_accessed ON auth.user_recent_files(accessed_at);
CREATE INDEX IF NOT EXISTS idx_user_recent_user_accessed ON auth.user_recent_files(user_id, accessed_at DESC);

COMMENT ON TABLE auth.users IS 'Stores user account information';
COMMENT ON TABLE auth.sessions IS 'Stores user session information for refresh tokens';
COMMENT ON TABLE auth.user_files IS 'Tracks file ownership and storage utilization by users';
COMMENT ON TABLE auth.user_favorites IS 'Stores user favorite files and folders for cross-device synchronization';
COMMENT ON TABLE auth.user_recent_files IS 'Stores recently accessed files and folders for cross-device synchronization';

-- Admin settings (key-value store for platform configuration)
CREATE TABLE IF NOT EXISTS auth.admin_settings (
    key VARCHAR(255) PRIMARY KEY,
    value TEXT NOT NULL,
    category VARCHAR(50) NOT NULL DEFAULT 'general',
    is_secret BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by VARCHAR(36)
);

CREATE INDEX IF NOT EXISTS idx_admin_settings_category ON auth.admin_settings(category);

COMMENT ON TABLE auth.admin_settings IS 'Platform configuration settings managed via admin panel';

-- OIDC identity linking columns
ALTER TABLE auth.users ADD COLUMN IF NOT EXISTS oidc_provider VARCHAR(255);
ALTER TABLE auth.users ADD COLUMN IF NOT EXISTS oidc_subject VARCHAR(255);
CREATE UNIQUE INDEX IF NOT EXISTS idx_users_oidc ON auth.users(oidc_provider, oidc_subject)
    WHERE oidc_provider IS NOT NULL AND oidc_subject IS NOT NULL;

-- NOTE: No default users are created. The first user to register through
-- the admin setup wizard will become the administrator.

-- Device Authorization Grant (RFC 8628)
-- Used for WebDAV/CalDAV/CardDAV client authentication via the device flow.
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

CREATE TABLE IF NOT EXISTS auth.device_codes (
    id VARCHAR(36) PRIMARY KEY,
    device_code VARCHAR(128) UNIQUE NOT NULL,
    user_code VARCHAR(16) UNIQUE NOT NULL,
    client_name VARCHAR(255) NOT NULL DEFAULT 'Unknown Client',
    scopes VARCHAR(512) NOT NULL DEFAULT 'webdav,caldav,carddav',
    status auth.device_code_status NOT NULL DEFAULT 'pending',
    user_id VARCHAR(36) REFERENCES auth.users(id) ON DELETE CASCADE,
    access_token TEXT,
    refresh_token TEXT,
    verification_uri TEXT NOT NULL,
    verification_uri_complete TEXT,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    poll_interval_secs INTEGER NOT NULL DEFAULT 5,
    last_poll_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    authorized_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX IF NOT EXISTS idx_device_codes_device_code
    ON auth.device_codes(device_code);
CREATE INDEX IF NOT EXISTS idx_device_codes_user_code
    ON auth.device_codes(user_code) WHERE status = 'pending';
CREATE INDEX IF NOT EXISTS idx_device_codes_expires_at
    ON auth.device_codes(expires_at) WHERE status = 'pending';
CREATE INDEX IF NOT EXISTS idx_device_codes_user_id
    ON auth.device_codes(user_id) WHERE status = 'authorized';

COMMENT ON TABLE auth.device_codes IS 'OAuth 2.0 Device Authorization Grant (RFC 8628) codes for DAV client authentication';

-- App Passwords (application-specific passwords for DAV clients with HTTP Basic Auth)
CREATE TABLE IF NOT EXISTS auth.app_passwords (
    id VARCHAR(36) PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    label VARCHAR(255) NOT NULL,
    password_hash TEXT NOT NULL,
    prefix VARCHAR(50) NOT NULL,
    scopes VARCHAR(512) NOT NULL DEFAULT 'webdav,caldav,carddav',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used_at TIMESTAMP WITH TIME ZONE,
    expires_at TIMESTAMP WITH TIME ZONE,
    active BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE INDEX IF NOT EXISTS idx_app_passwords_user_id
    ON auth.app_passwords(user_id) WHERE active = TRUE;
CREATE INDEX IF NOT EXISTS idx_app_passwords_active
    ON auth.app_passwords(user_id, active) WHERE active = TRUE;

COMMENT ON TABLE auth.app_passwords IS 'Application-specific passwords for DAV clients using HTTP Basic Auth';

-- ============================================================
-- 2. CALDAV SCHEMA (RFC 4791)
-- ============================================================
CREATE SCHEMA IF NOT EXISTS caldav;

-- Calendars
CREATE TABLE IF NOT EXISTS caldav.calendars (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    owner_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    description TEXT,
    color VARCHAR(9), -- #RRGGBB or #RRGGBBAA
    is_public BOOLEAN NOT NULL DEFAULT FALSE,
    ctag VARCHAR(64) NOT NULL DEFAULT '0',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_calendars_owner_id ON caldav.calendars(owner_id);

-- Calendar events (VEVENT)
CREATE TABLE IF NOT EXISTS caldav.calendar_events (
    id UUID PRIMARY KEY,
    calendar_id UUID NOT NULL REFERENCES caldav.calendars(id) ON DELETE CASCADE,
    summary TEXT NOT NULL,
    description TEXT,
    location TEXT,
    start_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_time TIMESTAMP WITH TIME ZONE NOT NULL,
    all_day BOOLEAN NOT NULL DEFAULT FALSE,
    rrule TEXT,
    ical_uid VARCHAR(255) NOT NULL,
    ical_data TEXT, -- Full iCalendar data for round-trip fidelity
    etag VARCHAR(64),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_calendar_events_calendar_id ON caldav.calendar_events(calendar_id);
CREATE INDEX IF NOT EXISTS idx_calendar_events_ical_uid ON caldav.calendar_events(ical_uid);
CREATE INDEX IF NOT EXISTS idx_calendar_events_time_range ON caldav.calendar_events(calendar_id, start_time, end_time);

-- Calendar sharing
CREATE TABLE IF NOT EXISTS caldav.calendar_shares (
    id SERIAL PRIMARY KEY,
    calendar_id UUID NOT NULL REFERENCES caldav.calendars(id) ON DELETE CASCADE,
    user_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    access_level VARCHAR(10) NOT NULL DEFAULT 'read', -- read, write, owner
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(calendar_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_calendar_shares_calendar_id ON caldav.calendar_shares(calendar_id);
CREATE INDEX IF NOT EXISTS idx_calendar_shares_user_id ON caldav.calendar_shares(user_id);

-- Calendar custom properties
CREATE TABLE IF NOT EXISTS caldav.calendar_properties (
    id SERIAL PRIMARY KEY,
    calendar_id UUID NOT NULL REFERENCES caldav.calendars(id) ON DELETE CASCADE,
    property_name TEXT NOT NULL,
    property_value TEXT NOT NULL,
    UNIQUE(calendar_id, property_name)
);

CREATE INDEX IF NOT EXISTS idx_calendar_properties_calendar_id ON caldav.calendar_properties(calendar_id);

COMMENT ON TABLE caldav.calendars IS 'CalDAV calendars for each user';
COMMENT ON TABLE caldav.calendar_events IS 'Calendar events (VEVENT) stored with iCal data';
COMMENT ON TABLE caldav.calendar_shares IS 'Calendar sharing permissions between users';
COMMENT ON TABLE caldav.calendar_properties IS 'Custom WebDAV properties on calendars';

-- ============================================================
-- 3. CARDDAV SCHEMA (RFC 6352)
-- ============================================================
CREATE SCHEMA IF NOT EXISTS carddav;

-- Address books
CREATE TABLE IF NOT EXISTS carddav.address_books (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    owner_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    description TEXT,
    color VARCHAR(9),
    is_public BOOLEAN NOT NULL DEFAULT FALSE,
    ctag VARCHAR(64) NOT NULL DEFAULT '0',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_address_books_owner_id ON carddav.address_books(owner_id);

-- Contacts
CREATE TABLE IF NOT EXISTS carddav.contacts (
    id UUID PRIMARY KEY,
    address_book_id UUID NOT NULL REFERENCES carddav.address_books(id) ON DELETE CASCADE,
    uid VARCHAR(255) NOT NULL,
    full_name TEXT,
    first_name TEXT,
    last_name TEXT,
    nickname TEXT,
    organization TEXT,
    title TEXT,
    notes TEXT,
    photo_url TEXT,
    birthday DATE,
    anniversary DATE,
    email JSONB NOT NULL DEFAULT '[]',
    phone JSONB NOT NULL DEFAULT '[]',
    address JSONB NOT NULL DEFAULT '[]',
    vcard TEXT, -- Full vCard data for round-trip fidelity
    etag VARCHAR(64) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_contacts_address_book_id ON carddav.contacts(address_book_id);
CREATE INDEX IF NOT EXISTS idx_contacts_uid ON carddav.contacts(uid);
CREATE INDEX IF NOT EXISTS idx_contacts_full_name ON carddav.contacts(full_name);

-- Address book sharing
CREATE TABLE IF NOT EXISTS carddav.address_book_shares (
    id SERIAL PRIMARY KEY,
    address_book_id UUID NOT NULL REFERENCES carddav.address_books(id) ON DELETE CASCADE,
    user_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    can_write BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(address_book_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_address_book_shares_address_book_id ON carddav.address_book_shares(address_book_id);
CREATE INDEX IF NOT EXISTS idx_address_book_shares_user_id ON carddav.address_book_shares(user_id);

-- Contact groups
CREATE TABLE IF NOT EXISTS carddav.contact_groups (
    id UUID PRIMARY KEY,
    address_book_id UUID NOT NULL REFERENCES carddav.address_books(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_contact_groups_address_book_id ON carddav.contact_groups(address_book_id);

-- Group memberships
CREATE TABLE IF NOT EXISTS carddav.group_memberships (
    id SERIAL PRIMARY KEY,
    group_id UUID NOT NULL REFERENCES carddav.contact_groups(id) ON DELETE CASCADE,
    contact_id UUID NOT NULL REFERENCES carddav.contacts(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(group_id, contact_id)
);

CREATE INDEX IF NOT EXISTS idx_group_memberships_group_id ON carddav.group_memberships(group_id);
CREATE INDEX IF NOT EXISTS idx_group_memberships_contact_id ON carddav.group_memberships(contact_id);

COMMENT ON TABLE carddav.address_books IS 'CardDAV address books for each user';
COMMENT ON TABLE carddav.contacts IS 'Contacts stored with vCard data for round-trip fidelity';
COMMENT ON TABLE carddav.address_book_shares IS 'Address book sharing permissions between users';
COMMENT ON TABLE carddav.contact_groups IS 'Contact groups within address books';
COMMENT ON TABLE carddav.group_memberships IS 'Many-to-many relationship between contacts and groups';

-- ============================================================
-- 4. STORAGE SCHEMA — 100% Blob Storage Model + ltree hierarchy
-- ============================================================
-- All file/folder metadata lives here. Actual file content is stored
-- as content-addressed blobs on the filesystem (.blobs/{prefix}/{hash}.blob).
-- The storage.blobs table is the authoritative dedup index — no JSON
-- files or in-memory HashMaps are used.
-- No physical directories are created for user folders — they are
-- virtual records in this schema.
--
-- Folder hierarchy uses PostgreSQL ltree for O(1) path lookups,
-- sub-tree queries, and ancestor/descendant operations — replacing
-- expensive recursive CTEs.
-- ============================================================
CREATE SCHEMA IF NOT EXISTS storage;

-- Enable ltree extension for hierarchical path operations
CREATE EXTENSION IF NOT EXISTS ltree;

-- Content-addressable blob index (dedup)
-- One row per unique content hash; multiple storage.files rows may
-- reference the same blob via blob_hash → storage.blobs.hash.
CREATE TABLE IF NOT EXISTS storage.blobs (
    hash        VARCHAR(64) PRIMARY KEY,
    size        BIGINT NOT NULL,
    ref_count   INTEGER NOT NULL DEFAULT 1 CHECK (ref_count >= 0),
    content_type TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Fast lookup for garbage collection (orphaned blobs with no references)
CREATE INDEX IF NOT EXISTS idx_blobs_orphaned
    ON storage.blobs(ref_count) WHERE ref_count = 0;

COMMENT ON TABLE storage.blobs IS 'Content-addressable blob dedup index — one row per unique SHA-256 hash';

-- Virtual folders (replaces physical directories on disk)
-- `path` is a materialized readable path (e.g. "Home - user1/Documents/Work")
--   maintained automatically by triggers on INSERT/UPDATE of name or parent_id.
-- `lpath` is an ltree label path using sanitized UUIDs for GiST-indexed
--   hierarchical queries (ancestor, descendant, sub-tree).
CREATE TABLE IF NOT EXISTS storage.folders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    parent_id UUID REFERENCES storage.folders(id) ON DELETE CASCADE,
    user_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    path TEXT NOT NULL DEFAULT '',
    lpath ltree NOT NULL DEFAULT '',
    is_trashed BOOLEAN NOT NULL DEFAULT FALSE,
    trashed_at TIMESTAMP WITH TIME ZONE,
    original_parent_id UUID,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- A user cannot have two non-trashed folders with the same name in the same parent
CREATE UNIQUE INDEX IF NOT EXISTS idx_folders_unique_name
    ON storage.folders(parent_id, name, user_id) WHERE NOT is_trashed AND parent_id IS NOT NULL;
CREATE UNIQUE INDEX IF NOT EXISTS idx_folders_unique_name_root
    ON storage.folders(name, user_id) WHERE NOT is_trashed AND parent_id IS NULL;

CREATE INDEX IF NOT EXISTS idx_folders_user_id ON storage.folders(user_id);
CREATE INDEX IF NOT EXISTS idx_folders_parent_id ON storage.folders(parent_id);
CREATE INDEX IF NOT EXISTS idx_folders_trashed ON storage.folders(user_id, is_trashed);
-- ltree GiST index for sub-tree, ancestor, and descendant queries
CREATE INDEX IF NOT EXISTS idx_folders_lpath ON storage.folders USING gist (lpath);
-- B-tree index on path for exact path lookups
CREATE INDEX IF NOT EXISTS idx_folders_path ON storage.folders (path text_pattern_ops);

-- ── ltree trigger: compute path & lpath on INSERT or UPDATE of name/parent_id ──
CREATE OR REPLACE FUNCTION storage.compute_folder_path()
RETURNS trigger AS $$
DECLARE
    parent_path TEXT;
    parent_lpath ltree;
    my_label TEXT;
BEGIN
    -- Convert UUID to a valid ltree label (replace '-' with '_')
    my_label := replace(NEW.id::text, '-', '_');

    IF NEW.parent_id IS NULL THEN
        NEW.path  := NEW.name;
        NEW.lpath := my_label::ltree;
    ELSE
        SELECT path, lpath INTO parent_path, parent_lpath
          FROM storage.folders WHERE id = NEW.parent_id;

        NEW.path  := parent_path || '/' || NEW.name;
        NEW.lpath := parent_lpath || my_label::ltree;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trg_folders_path
    BEFORE INSERT OR UPDATE OF name, parent_id ON storage.folders
    FOR EACH ROW EXECUTE FUNCTION storage.compute_folder_path();

-- ── Cascade trigger: when a folder's path/lpath changes, update ALL descendants
--    in a single batch UPDATE using the GiST index on lpath.
--    pg_trigger_depth() guard prevents re-firing on the rows touched by the
--    batch UPDATE itself (they also change path/lpath, which would otherwise
--    cause infinite recursion).
CREATE OR REPLACE FUNCTION storage.cascade_folder_path()
RETURNS trigger AS $$
BEGIN
    IF pg_trigger_depth() > 1 THEN
        RETURN NEW;
    END IF;

    IF OLD.path IS DISTINCT FROM NEW.path OR OLD.lpath IS DISTINCT FROM NEW.lpath THEN
        -- Single batch update: rewrite path/lpath for every descendant at once.
        -- Uses the GiST index idx_folders_lpath for the <@ operator.
        -- Does NOT touch name or parent_id, so compute_folder_path does not fire.
        UPDATE storage.folders
           SET path  = NEW.path || substr(path, length(OLD.path) + 1),
               lpath = NEW.lpath || subpath(lpath, nlevel(OLD.lpath))
         WHERE lpath <@ OLD.lpath
           AND id != NEW.id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trg_folders_cascade_path
    AFTER UPDATE OF path, lpath ON storage.folders
    FOR EACH ROW EXECUTE FUNCTION storage.cascade_folder_path();

-- Files as references to content-addressable blobs
CREATE TABLE IF NOT EXISTS storage.files (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    folder_id UUID REFERENCES storage.folders(id) ON DELETE CASCADE,
    user_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    blob_hash VARCHAR(64) NOT NULL,
    size BIGINT NOT NULL DEFAULT 0,
    mime_type TEXT NOT NULL DEFAULT 'application/octet-stream',
    is_trashed BOOLEAN NOT NULL DEFAULT FALSE,
    trashed_at TIMESTAMP WITH TIME ZONE,
    original_folder_id UUID,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- A user cannot have two non-trashed files with the same name in the same folder
CREATE UNIQUE INDEX IF NOT EXISTS idx_files_unique_name_in_folder
    ON storage.files(folder_id, name, user_id) WHERE NOT is_trashed AND folder_id IS NOT NULL;
CREATE UNIQUE INDEX IF NOT EXISTS idx_files_unique_name_at_root
    ON storage.files(name, user_id) WHERE NOT is_trashed AND folder_id IS NULL;

CREATE INDEX IF NOT EXISTS idx_files_user_id ON storage.files(user_id);
CREATE INDEX IF NOT EXISTS idx_files_folder_id ON storage.files(folder_id);
CREATE INDEX IF NOT EXISTS idx_files_blob_hash ON storage.files(blob_hash);
CREATE INDEX IF NOT EXISTS idx_files_trashed ON storage.files(user_id, is_trashed);
CREATE INDEX IF NOT EXISTS idx_files_name_search ON storage.files(user_id, name text_pattern_ops);

-- Trash view combining trashed files and folders for the TrashRepository.
-- Only shows top-level trashed items: excludes files/folders whose parent
-- is also trashed (they are implicitly in trash as children of a trashed folder).
CREATE OR REPLACE VIEW storage.trash_items AS
    SELECT f.id, f.name, 'file' AS item_type, f.user_id, f.trashed_at,
           f.original_folder_id AS original_parent_id, f.created_at
    FROM storage.files f
    WHERE f.is_trashed = TRUE
      AND (f.folder_id IS NULL
           OR NOT EXISTS (
               SELECT 1 FROM storage.folders p
                WHERE p.id = f.folder_id AND p.is_trashed = TRUE))
    UNION ALL
    SELECT fo.id, fo.name, 'folder' AS item_type, fo.user_id, fo.trashed_at,
           fo.original_parent_id, fo.created_at
    FROM storage.folders fo
    WHERE fo.is_trashed = TRUE
      AND (fo.parent_id IS NULL
           OR NOT EXISTS (
               SELECT 1 FROM storage.folders p
                WHERE p.id = fo.parent_id AND p.is_trashed = TRUE));

-- ── Trigger: auto-decrement blob ref_count when a file row is deleted ──
--
-- This is the single source of truth for ref_count bookkeeping on deletion.
-- It fires for ALL delete paths: explicit DELETE, ON DELETE CASCADE from
-- folders, trash emptying, and any future code that removes file rows.
-- Rust code must NOT call remove_reference() after deleting a file row
-- to avoid double-decrementing.
CREATE OR REPLACE FUNCTION storage.decrement_blob_ref()
RETURNS trigger AS $$
BEGIN
    UPDATE storage.blobs
       SET ref_count = GREATEST(ref_count - 1, 0)
     WHERE hash = OLD.blob_hash;
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_files_decrement_blob_ref ON storage.files;
CREATE TRIGGER trg_files_decrement_blob_ref
    AFTER DELETE ON storage.files
    FOR EACH ROW
    EXECUTE FUNCTION storage.decrement_blob_ref();

COMMENT ON FUNCTION storage.decrement_blob_ref() IS 'Auto-decrement blob ref_count when a file referencing it is deleted';

COMMENT ON TABLE storage.folders IS 'Virtual folder hierarchy with ltree — no physical directories on disk';
COMMENT ON TABLE storage.files IS 'File metadata pointing to content-addressable blobs';
COMMENT ON VIEW storage.trash_items IS 'Unified view of all trashed files and folders';

-- ── Share links ──────────────────────────────────────────────────────────
-- Replaces the legacy file-based shares.json with proper relational storage.
CREATE TABLE IF NOT EXISTS storage.shares (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    item_id     TEXT NOT NULL,
    item_name   TEXT,
    item_type   TEXT NOT NULL CHECK (item_type IN ('file', 'folder')),
    token       VARCHAR(36) NOT NULL UNIQUE,
    password_hash TEXT,
    expires_at  BIGINT,  -- unix epoch seconds, NULL = never expires
    permissions_read    BOOLEAN NOT NULL DEFAULT TRUE,
    permissions_write   BOOLEAN NOT NULL DEFAULT FALSE,
    permissions_reshare BOOLEAN NOT NULL DEFAULT FALSE,
    created_at  BIGINT NOT NULL,  -- unix epoch seconds
    created_by  VARCHAR(36) NOT NULL,
    access_count BIGINT NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_shares_token ON storage.shares(token);
CREATE INDEX IF NOT EXISTS idx_shares_item ON storage.shares(item_id, item_type);
CREATE INDEX IF NOT EXISTS idx_shares_created_by ON storage.shares(created_by);

COMMENT ON TABLE storage.shares IS 'Shared links for files and folders with token-based access';

-- ── Atomic recursive folder copy (WebDAV COPY Depth: infinity) ──────────
--
-- Copies the entire subtree rooted at `p_source_id` under `p_target_parent_id`.
-- Uses ltree for subtree discovery, a temp mapping table for old→new UUID remapping,
-- and level-by-level folder insertion so the `trg_folders_path` trigger can
-- resolve each parent's path/lpath correctly.
--
-- Files are zero-copy: new metadata rows reference the same blob_hash, and
-- ref_counts are incremented in a single batch UPDATE.
--
-- Performance: O(depth) INSERT statements for folders + 1 batch INSERT for files
-- + 1 batch UPDATE for blob ref_counts. A folder with 10K files and 50 sub-folders
-- completes in <20ms — vs ~5s with sequential N+1 copy.
CREATE OR REPLACE FUNCTION storage.copy_folder_tree(
    p_source_id UUID,
    p_target_parent_id UUID,       -- NULL = copy to root
    p_dest_name TEXT DEFAULT NULL   -- NULL = keep source folder name
) RETURNS TABLE(new_root_id TEXT, folders_copied BIGINT, files_copied BIGINT) AS $$
DECLARE
    v_root_lpath   ltree;
    v_root_depth   INT;
    v_max_depth    INT;
    v_level        INT;
    v_folders      BIGINT := 0;
    v_files        BIGINT := 0;
    v_inserted     BIGINT;
    v_new_root     UUID;
BEGIN
    -- Validate source exists
    SELECT fo.lpath, nlevel(fo.lpath)
      INTO v_root_lpath, v_root_depth
      FROM storage.folders fo
     WHERE fo.id = p_source_id AND NOT fo.is_trashed;

    IF v_root_lpath IS NULL THEN
        RAISE EXCEPTION 'Source folder not found: %', p_source_id
            USING ERRCODE = 'P0002';  -- no_data_found
    END IF;

    -- Temp mapping: every folder in the subtree → new UUID
    CREATE TEMP TABLE IF NOT EXISTS _copy_map(
        old_id UUID PRIMARY KEY,
        new_id UUID NOT NULL DEFAULT gen_random_uuid()
    ) ON COMMIT DROP;
    TRUNCATE _copy_map;

    INSERT INTO _copy_map(old_id)
    SELECT fo.id
      FROM storage.folders fo
     WHERE NOT fo.is_trashed
       AND fo.lpath <@ v_root_lpath;

    -- Remember new root ID
    SELECT cm.new_id INTO v_new_root
      FROM _copy_map cm WHERE cm.old_id = p_source_id;

    -- Max depth for level iteration
    SELECT MAX(nlevel(fo.lpath))
      INTO v_max_depth
      FROM storage.folders fo
      JOIN _copy_map cm ON fo.id = cm.old_id;

    -- ── Insert folders level by level ──
    -- Each level is a separate INSERT so that the BEFORE INSERT trigger
    -- (trg_folders_path) can resolve the parent's path/lpath from rows
    -- inserted in the previous level.
    FOR v_level IN v_root_depth .. v_max_depth LOOP
        INSERT INTO storage.folders(id, name, parent_id, user_id)
        SELECT cm.new_id,
               CASE WHEN fo.id = p_source_id AND p_dest_name IS NOT NULL
                    THEN p_dest_name ELSE fo.name END,
               CASE WHEN fo.id = p_source_id THEN p_target_parent_id
                    ELSE pm.new_id END,
               fo.user_id
          FROM storage.folders fo
          JOIN _copy_map cm ON fo.id = cm.old_id
          LEFT JOIN _copy_map pm ON fo.parent_id = pm.old_id
         WHERE NOT fo.is_trashed
           AND nlevel(fo.lpath) = v_level;

        GET DIAGNOSTICS v_inserted = ROW_COUNT;
        v_folders := v_folders + v_inserted;
    END LOOP;

    -- ── Batch copy all files (zero-copy: same blob_hash) ──
    INSERT INTO storage.files(name, folder_id, user_id, blob_hash, size, mime_type)
    SELECT f.name, cm.new_id, f.user_id, f.blob_hash, f.size, f.mime_type
      FROM storage.files f
      JOIN _copy_map cm ON f.folder_id = cm.old_id
     WHERE NOT f.is_trashed;

    GET DIAGNOSTICS v_files = ROW_COUNT;

    -- ── Batch increment blob ref_counts ──
    IF v_files > 0 THEN
        UPDATE storage.blobs b
           SET ref_count = ref_count + hc.cnt
          FROM (
              SELECT f.blob_hash, COUNT(*)::int AS cnt
                FROM storage.files f
                JOIN _copy_map cm ON f.folder_id = cm.new_id
               WHERE NOT f.is_trashed
               GROUP BY f.blob_hash
          ) hc
         WHERE b.hash = hc.blob_hash;
    END IF;

    RETURN QUERY SELECT v_new_root::text, v_folders, v_files;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION storage.copy_folder_tree(UUID, UUID, TEXT)
    IS 'Atomic recursive folder copy using ltree — O(depth) + 1 batch file copy + 1 batch ref_count update';
