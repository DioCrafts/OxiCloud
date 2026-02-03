-- OxiCloud Authentication Database Schema

-- Create schema for auth-related tables
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

-- Create indexes for users table
CREATE INDEX IF NOT EXISTS idx_users_username ON auth.users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON auth.users(email);

-- Sessions table for refresh tokens
CREATE TABLE IF NOT EXISTS auth.sessions (
    id VARCHAR(36) PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    refresh_token TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    ip_address TEXT, -- to support IPv6
    user_agent TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    revoked BOOLEAN NOT NULL DEFAULT FALSE
);

-- Create indexes for sessions table
CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON auth.sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_sessions_refresh_token ON auth.sessions(refresh_token);
CREATE INDEX IF NOT EXISTS idx_sessions_expires_at ON auth.sessions(expires_at);

-- Create function for active sessions to use in index
CREATE OR REPLACE FUNCTION auth.is_session_active(expires_at timestamptz)
RETURNS boolean AS $$
BEGIN
  RETURN expires_at > now();
END;
$$ LANGUAGE plpgsql IMMUTABLE;

-- Create index for active sessions with IMMUTABLE function
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

-- Create indexes for user_files
CREATE INDEX IF NOT EXISTS idx_user_files_user_id ON auth.user_files(user_id);
CREATE INDEX IF NOT EXISTS idx_user_files_file_id ON auth.user_files(file_id);

-- User favorites table for cross-device synchronization
CREATE TABLE IF NOT EXISTS auth.user_favorites (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    item_id TEXT NOT NULL,
    item_type TEXT NOT NULL, -- 'file' or 'folder'
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, item_id, item_type)
);

-- Create indexes for efficient querying
CREATE INDEX IF NOT EXISTS idx_user_favorites_user_id ON auth.user_favorites(user_id);
CREATE INDEX IF NOT EXISTS idx_user_favorites_item_id ON auth.user_favorites(item_id);
CREATE INDEX IF NOT EXISTS idx_user_favorites_type ON auth.user_favorites(item_type);
CREATE INDEX IF NOT EXISTS idx_user_favorites_created ON auth.user_favorites(created_at);

-- Combined index for quick lookups by user and type
CREATE INDEX IF NOT EXISTS idx_user_favorites_user_type ON auth.user_favorites(user_id, item_type);

-- Table for recent files 
CREATE TABLE IF NOT EXISTS auth.user_recent_files (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    item_id TEXT NOT NULL,
    item_type TEXT NOT NULL, -- 'file' or 'folder'
    accessed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, item_id, item_type)
);

-- Create indexes for efficient querying
CREATE INDEX IF NOT EXISTS idx_user_recent_user_id ON auth.user_recent_files(user_id);
CREATE INDEX IF NOT EXISTS idx_user_recent_item_id ON auth.user_recent_files(item_id);
CREATE INDEX IF NOT EXISTS idx_user_recent_type ON auth.user_recent_files(item_type);
CREATE INDEX IF NOT EXISTS idx_user_recent_accessed ON auth.user_recent_files(accessed_at);

-- Combined index for quick lookups by user and accessed time (for sorting)
CREATE INDEX IF NOT EXISTS idx_user_recent_user_accessed ON auth.user_recent_files(user_id, accessed_at DESC);

COMMENT ON TABLE auth.user_recent_files IS 'Stores recently accessed files and folders for cross-device synchronization';

-- NOTE: No default users are created. The first user to register through
-- the admin setup wizard will become the administrator.

COMMENT ON TABLE auth.users IS 'Stores user account information';
COMMENT ON TABLE auth.sessions IS 'Stores user session information for refresh tokens';
COMMENT ON TABLE auth.user_files IS 'Tracks file ownership and storage utilization by users';
COMMENT ON TABLE auth.user_favorites IS 'Stores user favorite files and folders for cross-device synchronization';