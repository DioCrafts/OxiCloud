# Chunked Uploads

OxiCloud supports TUS-like chunked uploads for large files. Uploads are parallel, resumable, and have MD5 integrity checks.

## How It Works

1. Client sends `POST /api/files/upload/init` with file metadata → receives an `upload_id`
2. Client splits the file into chunks and uploads them in parallel via `POST /api/files/upload/chunk`
3. Each chunk includes its index, MD5 hash, and the `upload_id`
4. When all chunks are uploaded, client calls `POST /api/files/upload/complete`
5. Server reassembles the file, verifies integrity, and runs deduplication

## API Endpoints

### Initialize Upload

```http
POST /api/files/upload/init
Content-Type: application/json

{
  "file_name": "large-video.mp4",
  "folder_id": "folder-uuid",
  "total_size": 524288000,
  "chunk_size": 8388608,
  "total_chunks": 63
}
```

### Upload Chunk

```http
POST /api/files/upload/chunk
Content-Type: multipart/form-data

upload_id: "uuid"
chunk_index: 0
chunk_hash: "md5-hex"
file: <binary>
```

### Complete Upload

```http
POST /api/files/upload/complete
Content-Type: application/json

{
  "upload_id": "uuid"
}
```

## Configuration

| Parameter | Default | Description |
|---|---|---|
| Max parallel chunks | 8 | Concurrent chunk uploads |
| Min size for chunking | 200 MB | Below this, single-shot upload is used |
| Chunk size | 8 MB | Default chunk size |

## Frontend Behaviour

The OxiCloud web UI automatically selects chunked upload for large files. A progress bar shows overall completion and current chunk status.
