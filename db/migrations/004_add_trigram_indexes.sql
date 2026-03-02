-- Migration 004: Add GIN trigram indexes for ILIKE/LIKE substring search
--
-- Eliminates full table scans on text search queries by enabling
-- PostgreSQL's pg_trgm extension and creating GIN indexes with
-- gin_trgm_ops on all columns used in ILIKE / LIKE '%text%' patterns.
--
-- CONCURRENTLY is used so that no table locks are held during index
-- creation — zero downtime for existing installations.
--
-- NOTE: CREATE INDEX CONCURRENTLY cannot run inside a transaction block.
-- If using sqlx migrate, run this file manually:
--   psql -f db/migrations/004_add_trigram_indexes.sql

-- 0. Enable the pg_trgm extension (idempotent)
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- 1. Contacts — search_contacts(), get_contacts_by_email()
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_contacts_full_name_trgm
    ON carddav.contacts USING gin (full_name gin_trgm_ops);
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_contacts_first_name_trgm
    ON carddav.contacts USING gin (first_name gin_trgm_ops);
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_contacts_last_name_trgm
    ON carddav.contacts USING gin (last_name gin_trgm_ops);
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_contacts_nickname_trgm
    ON carddav.contacts USING gin (nickname gin_trgm_ops);
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_contacts_organization_trgm
    ON carddav.contacts USING gin (organization gin_trgm_ops);
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_contacts_email_text_trgm
    ON carddav.contacts USING gin ((email::text) gin_trgm_ops);
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_contacts_phone_text_trgm
    ON carddav.contacts USING gin ((phone::text) gin_trgm_ops);

-- 2. Calendar events — find_events_by_summary()
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_calendar_events_summary_trgm
    ON caldav.calendar_events USING gin (summary gin_trgm_ops);

-- 3. Files — search_files_paginated(), search_files_in_subtree(), suggest_files_by_name()
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_files_name_trgm
    ON storage.files USING gin (name gin_trgm_ops);

-- 4. Folders — search_folders(), list_descendant_folders(), suggest_folders_by_name()
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_folders_name_trgm
    ON storage.folders USING gin (name gin_trgm_ops);
