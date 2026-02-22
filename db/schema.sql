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

-- ── Cascade trigger: when a folder's path/lpath changes, update all descendants ──
CREATE OR REPLACE FUNCTION storage.cascade_folder_path()
RETURNS trigger AS $$
BEGIN
    IF OLD.path IS DISTINCT FROM NEW.path OR OLD.lpath IS DISTINCT FROM NEW.lpath THEN
        -- Update all descendant folders (recursive via trigger re-fire)
        UPDATE storage.folders
           SET parent_id = parent_id  -- no-op value change, but fires the BEFORE UPDATE trigger
         WHERE parent_id = NEW.id;
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
