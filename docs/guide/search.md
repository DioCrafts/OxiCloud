# Search

OxiCloud provides full-text search across your files with multiple filter options.

## Endpoint

```http
GET /api/search?q=report&type_filter=pdf,docx&recursive=true
```

## Query Parameters

| Parameter | Description |
|---|---|
| `q` | Search query (matches file name) |
| `type_filter` | Comma-separated file extensions to filter by |
| `folder_id` | Restrict search to a specific folder |
| `recursive` | `true` to search subfolders |
| `date_from` / `date_to` | Filter by modification date |
| `size_min` / `size_max` | Filter by file size (bytes) |
| `limit` | Maximum results to return |
| `offset` | Pagination offset |

## How It Works

OxiCloud stores file metadata in PostgreSQL with an `ltree` path column, enabling efficient recursive subtree queries:

```sql
SELECT * FROM storage.files
WHERE path <@ 'root.folder_id'
  AND LOWER(name) LIKE '%query%'
  AND LOWER(extension) = ANY('{pdf,docx}')
ORDER BY updated_at DESC
LIMIT 50;
```

## Frontend

The web UI includes a search bar in the toolbar. Results appear instantly with file name, path, size, and type. Clicking a result navigates to the file's location.

## Feature Flag

Search can be disabled via `OXICLOUD_ENABLE_SEARCH=false`.
