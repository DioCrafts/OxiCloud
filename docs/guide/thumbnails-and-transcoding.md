# Thumbnails and Transcoding

OxiCloud optimizes image delivery with two complementary features:

- WebP thumbnail generation in three sizes
- On-the-fly image transcoding for browsers that advertise WebP support

Both features use a memory cache plus a persistent disk cache and are designed to stay off the request hot path whenever possible.

## Thumbnails

### Supported sizes

| Size | Dimensions | Directory |
| --- | --- | --- |
| `icon` | 150 x 150 | `.thumbnails/icon/` |
| `preview` | 400 x 400 | `.thumbnails/preview/` |
| `large` | 800 x 800 | `.thumbnails/large/` |

### Supported inputs

`image/jpeg`, `image/jpg`, `image/png`, `image/gif`, and `image/webp`

All thumbnail outputs are stored as WebP.

### API

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/api/files/{id}/thumbnail/{size}` | Fetch a thumbnail |
| `POST` | `/api/files/upload` | Upload a file and pre-generate thumbnails for supported images |

Thumbnail responses include:

- `Content-Type: image/webp`
- `Cache-Control: public, max-age=31536000, immutable`
- `ETag: "thumb-{id}-{size}"`

### Generation flow

1. Upload succeeds through the file API
2. If the MIME type is supported, OxiCloud starts thumbnail generation in a background task
3. If a thumbnail is requested before pre-generation completes, the request can generate it lazily
4. Future requests are served from memory or disk cache

## Image Transcoding

OxiCloud can serve a smaller WebP version of uploaded JPEG, PNG, or GIF files when the client advertises WebP support in the `Accept` header.

### Rules

- Files over 5 MB skip transcoding
- Existing WebP files are not transcoded again
- SVG and BMP are not transcoded
- If the WebP output is larger than the original, OxiCloud serves the original file instead

### Storage layout

```text
<storage_path>/
  .transcoded/
    webp/
      <file_id>.webp
```

### Statistics tracked by the service

- Cache hits
- Disk hits
- Successful transcodes
- Bytes saved
- Transcode errors

## Caching

Both thumbnail and transcode services use:

- An in-memory LRU cache for hot assets
- A disk cache for persistent reuse across restarts
- Fire-and-forget background writes for cache warmup

For the broader cache model across metadata and listings, see [Caching Architecture](/architecture/caching).

## Example

```text
Client: GET /api/files/abc-123/download
Accept: image/webp, image/png, */*

Server: checks cache -> transcodes if needed -> returns the smaller asset
```

## Related Pages

- [Caching Architecture](/architecture/caching)
- [ZIP and Compression](/guide/zip-and-compression)