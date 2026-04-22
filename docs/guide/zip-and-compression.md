# ZIP and Compression

OxiCloud ships two compression-related features:

- ZIP download for folders
- Gzip compression for suitable file responses

## ZIP Download

### Endpoint

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/api/folders/{id}/download` | Download a folder as a ZIP archive |

### How it works

- The ZIP archive is built in memory
- Folder traversal uses an iterative queue rather than recursive async calls
- Cycle detection prevents loops while walking nested folders
- Entries are written with UNIX mode `0o755`
- Compression uses the `Deflated` method from the `zip` crate

### Example

```bash
curl -H "Authorization: Bearer $TOKEN" \
  "https://oxicloud.example.com/api/folders/abc-123/download" \
  -o my-folder.zip
```

## Gzip Compression

OxiCloud can compress responses when it is worth doing so.

### Compression threshold

Files below 50 KB are skipped.

### Compression levels

| Level | Value |
| --- | --- |
| `None` | `0` |
| `Fast` | `1` |
| `Default` | `6` |
| `Best` | `9` |

### Skip list

These types are not gzipped because they are already compressed or because compression provides poor returns:

- `image/*` except SVG and BMP
- `audio/*`
- `video/*`
- archive formats such as ZIP, gzip, 7z, RAR, bzip2, and XZ

### Runtime behavior

- Compression and decompression run in `spawn_blocking`
- The implementation uses `flate2`
- Optional buffer pooling reduces allocation churn under load

## Related Pages

- [Batch Operations](/guide/batch-operations)
- [Thumbnails and Transcoding](/guide/thumbnails-and-transcoding)