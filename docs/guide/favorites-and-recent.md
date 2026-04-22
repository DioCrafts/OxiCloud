# Favorites and Recent Items

OxiCloud includes two per-user tracking features backed by PostgreSQL:

- Favorites for pinning files and folders you want to reach quickly
- Recent items for tracking the files and folders you accessed most recently

Both features are enabled when the instance has a database connection.

## Favorites

### API

All routes live under `/api/favorites` and require authentication.

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/api/favorites/` | List all favorites for the current user |
| `POST` | `/api/favorites/{item_type}/{item_id}` | Add a file or folder to favorites |
| `DELETE` | `/api/favorites/{item_type}/{item_id}` | Remove a favorite |

`item_type` must be either `file` or `folder`.

### Behavior

- Adding the same item twice is idempotent
- Results are ordered by `created_at DESC`
- User identity comes from the JWT, not the request body

### Storage model

Favorites are stored in `auth.user_favorites` with a uniqueness constraint on `(user_id, item_id, item_type)`.

```sql
CREATE TABLE IF NOT EXISTS auth.user_favorites (
    id         SERIAL PRIMARY KEY,
    user_id    VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    item_id    TEXT NOT NULL,
    item_type  TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, item_id, item_type)
);
```

## Recent Items

### API

All routes live under `/api/recent` and require authentication.

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/api/recent/` | List recent items, optionally with `?limit=N` |
| `POST` | `/api/recent/{item_type}/{item_id}` | Record an access |
| `DELETE` | `/api/recent/{item_type}/{item_id}` | Remove one item from history |
| `DELETE` | `/api/recent/clear` | Clear all recent items |

### Behavior

- Default maximum per user: 50 items
- Re-accessing an item updates its `accessed_at` timestamp
- Old items are automatically pruned after inserts
- Results are ordered by `accessed_at DESC`

### Storage model

Recent items are stored in `auth.user_recent_files`.

```sql
CREATE TABLE IF NOT EXISTS auth.user_recent_files (
    id          SERIAL PRIMARY KEY,
    user_id     VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    item_id     TEXT NOT NULL,
    item_type   TEXT NOT NULL,
    accessed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, item_id, item_type)
);
```

## Example

```bash
# Add a file to favorites
curl -X POST -H "Authorization: Bearer $TOKEN" \
  "https://oxicloud.example.com/api/favorites/file/abc-123"

# List recent items
curl -H "Authorization: Bearer $TOKEN" \
  "https://oxicloud.example.com/api/recent/?limit=10"

# Clear recent history
curl -X DELETE -H "Authorization: Bearer $TOKEN" \
  "https://oxicloud.example.com/api/recent/clear"
```

## Related Pages

- [Search](/guide/search)
- [Trash & Recycle Bin](/guide/trash)
- [Batch Operations](/guide/batch-operations)