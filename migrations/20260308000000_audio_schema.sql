-- ============================================================
-- Audio Schema for Music Library
-- Playlists and audio metadata storage
-- ============================================================

-- Audio schema for music library
CREATE SCHEMA IF NOT EXISTS audio;

-- Playlists
CREATE TABLE IF NOT EXISTS audio.playlists (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    description TEXT,
    owner_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    is_public BOOLEAN NOT NULL DEFAULT FALSE,
    cover_file_id UUID REFERENCES storage.files(id) ON DELETE SET NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_playlists_owner_id ON audio.playlists(owner_id);
CREATE INDEX IF NOT EXISTS idx_playlists_is_public ON audio.playlists(is_public) WHERE is_public = TRUE;

COMMENT ON TABLE audio.playlists IS 'User playlists for organizing music tracks';

-- Playlist items (tracks in a playlist)
CREATE TABLE IF NOT EXISTS audio.playlist_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    playlist_id UUID NOT NULL REFERENCES audio.playlists(id) ON DELETE CASCADE,
    file_id UUID NOT NULL REFERENCES storage.files(id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    added_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(playlist_id, file_id)
);

CREATE INDEX IF NOT EXISTS idx_playlist_items_playlist_id ON audio.playlist_items(playlist_id);
CREATE INDEX IF NOT EXISTS idx_playlist_items_file_id ON audio.playlist_items(file_id);
CREATE INDEX IF NOT EXISTS idx_playlist_items_position ON audio.playlist_items(playlist_id, position);

COMMENT ON TABLE audio.playlist_items IS 'Tracks belonging to playlists';

-- Audio metadata for files (extracted from audio files)
CREATE TABLE IF NOT EXISTS audio.file_metadata (
    file_id UUID PRIMARY KEY REFERENCES storage.files(id) ON DELETE CASCADE,
    title TEXT,
    artist TEXT,
    album TEXT,
    album_artist TEXT,
    genre TEXT,
    track_number INTEGER,
    disc_number INTEGER,
    year INTEGER,
    duration_secs INTEGER NOT NULL DEFAULT 0,
    bitrate INTEGER,
    sample_rate INTEGER,
    channels SMALLINT,
    format TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_audio_metadata_artist ON audio.file_metadata(artist);
CREATE INDEX IF NOT EXISTS idx_audio_metadata_album ON audio.file_metadata(album);
CREATE INDEX IF NOT EXISTS idx_audio_metadata_genre ON audio.file_metadata(genre);
CREATE INDEX IF NOT EXISTS idx_audio_metadata_year ON audio.file_metadata(year);

COMMENT ON TABLE audio.file_metadata IS 'Audio metadata extracted from music files (ID3, Vorbis tags)';

-- Playlist sharing (similar to calendar/address book sharing)
CREATE TABLE IF NOT EXISTS audio.playlist_shares (
    id SERIAL PRIMARY KEY,
    playlist_id UUID NOT NULL REFERENCES audio.playlists(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    can_write BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(playlist_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_playlist_shares_playlist_id ON audio.playlist_shares(playlist_id);
CREATE INDEX IF NOT EXISTS idx_playlist_shares_user_id ON audio.playlist_shares(user_id);

COMMENT ON TABLE audio.playlist_shares IS 'Playlist sharing permissions between users';
