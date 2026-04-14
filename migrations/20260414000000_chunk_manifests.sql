-- Content-Defined Chunking (CDC) manifests for sub-file deduplication.
--
-- Each file uploaded via CDC is split into variable-size chunks (FastCDC).
-- The manifest records the ordered list of chunk hashes that compose the file.
-- Individual chunks are stored in storage.blobs (shared across manifests).
--
-- Legacy whole-file blobs (pre-CDC) remain in storage.blobs and are
-- accessed directly when no matching manifest row exists.

CREATE TABLE IF NOT EXISTS storage.chunk_manifests (
    file_hash    VARCHAR(64) PRIMARY KEY,
    chunk_hashes TEXT[]      NOT NULL,
    chunk_sizes  BIGINT[]    NOT NULL,
    total_size   BIGINT      NOT NULL,
    chunk_count  INTEGER     NOT NULL,
    content_type TEXT,
    ref_count    INTEGER     NOT NULL DEFAULT 1 CHECK (ref_count >= 0),
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for GC: find manifests with no references.
CREATE INDEX IF NOT EXISTS idx_chunk_manifests_ref_count_zero
    ON storage.chunk_manifests (file_hash)
    WHERE ref_count = 0;
