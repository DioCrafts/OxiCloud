# Chunked Uploads

OxiCloud exposes resumable chunked uploads under `/api/uploads`. The protocol is TUS-like in spirit, but the concrete API is OxiCloud-specific: create a session, stream chunks with `PATCH`, inspect progress with `HEAD`, then finalize the assembled file.

## Upload Flow

1. Create an upload session with `POST /api/uploads`
2. Upload each chunk with `PATCH /api/uploads/{upload_id}?chunk_index=N`
3. Optionally inspect progress with `HEAD /api/uploads/{upload_id}`
4. Finalize with `POST /api/uploads/{upload_id}/complete`
5. Cancel an in-flight upload with `DELETE /api/uploads/{upload_id}` if needed

## API Endpoints

### Create upload session

```http
POST /api/uploads
Content-Type: application/json

{
  "filename": "large-video.mp4",
  "folder_id": "folder-uuid",
  "content_type": "video/mp4",
  "total_size": 524288000,
  "chunk_size": 8388608
}
```

Typical response:

```json
{
  "upload_id": "uuid",
  "chunk_size": 8388608,
  "total_chunks": 63,
  "expires_at": 86400
}
```

### Upload a chunk

Chunks are sent as raw bytes, not multipart form uploads.

```http
PATCH /api/uploads/{upload_id}?chunk_index=0&checksum=md5-hex
Content-Type: application/octet-stream
Content-MD5: md5-hex

<binary chunk bytes>
```

Notes:

- `chunk_index` is required and zero-based
- `checksum` is optional and can also be supplied with the `Content-MD5` header
- Successful responses include progress headers such as `Upload-Offset`, `Upload-Progress`, and `Upload-Complete`

### Inspect upload status

```http
HEAD /api/uploads/{upload_id}
```

The response includes upload metadata in headers such as:

- `Upload-Offset`
- `Upload-Length`
- `Upload-Progress`
- `Upload-Chunks-Total`
- `Upload-Chunks-Complete`

### Finalize upload

```http
POST /api/uploads/{upload_id}/complete
```

Successful responses return the created file metadata:

```json
{
  "file_id": "uuid",
  "filename": "large-video.mp4",
  "size": 524288000,
  "path": "/Videos/large-video.mp4"
}
```

### Cancel upload

```http
DELETE /api/uploads/{upload_id}
```

This removes the in-progress session and temporary chunk data.

## Validation Rules

- `filename` is required
- `total_size` must be greater than zero
- `chunk_size` must be at least 1 MB when provided
- Storage quota checks can reject the session before upload starts

## Frontend Behavior

The OxiCloud web UI can switch to chunked uploads for larger files, track aggregate progress, and retry individual chunks without restarting the full transfer.
