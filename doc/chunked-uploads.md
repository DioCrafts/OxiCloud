# 07 - Chunked Uploads

OxiCloud implements a TUS-like chunked upload protocol for large files (≥10 MB). Files are split into chunks (default 5 MB) that can be uploaded in parallel (up to 6 concurrent), with progress tracking, optional MD5 checksums, and automatic session expiration.

## Architecture

| Layer | Component | File |
|---|---|---|
| Application Port | **ChunkedUploadPort** trait + DTOs | `src/application/ports/chunked_upload_ports.rs` |
| Infrastructure | **ChunkedUploadService** | `src/infrastructure/services/chunked_upload_service.rs` |
| Interfaces | **ChunkedUploadHandler** | `src/interfaces/api/handlers/chunked_upload_handler.rs` |

## Constants

| Constant | Value | Description |
|---|---|---|
| `CHUNKED_UPLOAD_THRESHOLD` | 10 MB | Files above this should use chunked upload |
| `DEFAULT_CHUNK_SIZE` | 5 MB | Default chunk size (minimum 1 MB) |
| `MAX_PARALLEL_CHUNKS` | 6 | Maximum concurrent chunk uploads |
| `SESSION_EXPIRATION` | 24 hours | Sessions expire after this duration |

## REST API

All routes under `/api/uploads`, authentication required.

| Method | Path | Handler | Description |
|---|---|---|---|
| `POST` | `/api/uploads` | `create_upload` | Create upload session |
| `PATCH` | `/api/uploads/{upload_id}` | `upload_chunk` | Upload a single chunk |
| `HEAD` | `/api/uploads/{upload_id}` | `get_upload_status` | Query upload progress |
| `POST` | `/api/uploads/{upload_id}/complete` | `complete_upload` | Assemble chunks → create file |
| `DELETE` | `/api/uploads/{upload_id}` | `cancel_upload` | Cancel and cleanup |

## Protocol Flow

```
1. POST /api/uploads
   Body: { "filename": "video.mp4", "total_size": 104857600, "content_type": "video/mp4" }
   Response: { "upload_id": "abc-123", "chunk_size": 5242880, "total_chunks": 20, "expires_at": 1707868800 }

2. PATCH /api/uploads/abc-123?chunk_index=0   ──┐
   PATCH /api/uploads/abc-123?chunk_index=1   ──┼── Up to 6 in parallel
   PATCH /api/uploads/abc-123?chunk_index=2   ──┘
   Body: raw chunk bytes
   Response: { "chunk_index": 0, "bytes_received": 52428800, "progress": 50.0, "is_complete": false }

3. HEAD /api/uploads/abc-123
   Response headers: Upload-Offset, Upload-Length, Upload-Progress, Upload-Chunks-Total, Upload-Chunks-Complete

4. POST /api/uploads/abc-123/complete
   Response: { "file_id": "def-456", "filename": "video.mp4", "size": 104857600, "path": "/videos" }
   Status: 201 Created
```

## Request/Response DTOs

### Create Upload Request

```rust
pub struct CreateUploadRequest {
    pub filename: String,
    pub folder_id: Option<String>,
    pub content_type: Option<String>,  // default: "application/octet-stream"
    pub total_size: u64,
    pub chunk_size: Option<usize>,     // default: 5 MB, minimum: 1 MB
}
```

### Chunk Upload Query Parameters

```rust
pub struct ChunkUploadParams {
    pub chunk_index: usize,
    pub checksum: Option<String>,      // MD5 hash (also via Content-MD5 header)
}
```

### Upload Status Response

```rust
pub struct UploadStatusResponseDto {
    pub upload_id: String,
    pub filename: String,
    pub total_size: u64,
    pub bytes_received: u64,
    pub progress: f64,                 // 0.0 - 100.0
    pub total_chunks: usize,
    pub completed_chunks: usize,
    pub pending_chunks: Vec<usize>,
    pub is_complete: bool,
}
```

### Complete Upload Response

```rust
pub struct CompleteUploadResponse {
    pub file_id: String,
    pub filename: String,
    pub size: u64,
    pub path: String,
}
```

## Custom Response Headers

### On upload_chunk (PATCH)
- `Upload-Offset`: total bytes received so far
- `Upload-Progress`: percentage complete (0-100)
- `Upload-Complete: true` (only when all chunks are uploaded)

### On get_upload_status (HEAD)
- `Upload-Offset`: bytes received
- `Upload-Length`: total expected size
- `Upload-Progress`: percentage
- `Upload-Chunks-Total`: total chunk count
- `Upload-Chunks-Complete`: completed chunk count

## Internal Storage

```
<temp_dir>/
  <upload_id>/
    chunk_000000          ← individual chunk files
    chunk_000001
    chunk_000002
    ...
    assembled             ← final assembled file (after complete)
```

## Completion Flow

1. `complete_upload()` assembles all chunks in order into a single `assembled` file
2. Reads the assembled file content
3. Delegates to **FileUploadService::upload_file()** to create the permanent file record
4. Calls `finalize_upload()` to clean up the session and temp directory
5. Returns `201 Created` with file metadata

## Session Management

- **In-memory sessions**: `HashMap<String, UploadSession>` protected by `RwLock`
- **Chunk status tracking**: each chunk has a status: `Pending` → `Uploading` → `Complete` (or `Failed`)
- **Expiration**: sessions expire after 24 hours of inactivity
- **Cleanup**: background task runs hourly to remove expired sessions and orphaned temp directories

## Client Usage Example

```bash
# 1. Create upload session
RESPONSE=$(curl -s -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"filename":"large-file.zip","total_size":52428800}' \
  "https://oxicloud.example.com/api/uploads")

UPLOAD_ID=$(echo $RESPONSE | jq -r '.upload_id')
CHUNK_SIZE=$(echo $RESPONSE | jq -r '.chunk_size')
TOTAL_CHUNKS=$(echo $RESPONSE | jq -r '.total_chunks')

# 2. Upload chunks in parallel
for i in $(seq 0 $((TOTAL_CHUNKS - 1))); do
  dd if=large-file.zip bs=$CHUNK_SIZE skip=$i count=1 2>/dev/null | \
  curl -s -X PATCH -H "Authorization: Bearer $TOKEN" \
    --data-binary @- \
    "https://oxicloud.example.com/api/uploads/$UPLOAD_ID?chunk_index=$i" &
done
wait

# 3. Complete upload
curl -X POST -H "Authorization: Bearer $TOKEN" \
  "https://oxicloud.example.com/api/uploads/$UPLOAD_ID/complete"

# 4. Check progress (optional)
curl -I -H "Authorization: Bearer $TOKEN" \
  "https://oxicloud.example.com/api/uploads/$UPLOAD_ID"
```
