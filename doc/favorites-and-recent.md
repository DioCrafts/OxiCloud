# 13 - Favorites and Recent Items

Two per-user item tracking features:

- **Favorites** -- users mark files and folders for quick access.
- **Recent Items** -- automatically tracks recently accessed files and folders.

Both require PostgreSQL and are only available when a database connection is configured.

---

## Favorites

### Architecture

| Layer | Component | File |
|---|---|---|
| Application Port | **FavoritesUseCase**, **FavoritesRepositoryPort** | `src/application/ports/favorites_ports.rs` |
| Application Service | **FavoritesService** | `src/application/services/favorites_service.rs` |
| Application DTO | **FavoriteItemDto** | `src/application/dtos/favorites_dto.rs` |
| Infrastructure | **FavoritesPgRepository** | `src/infrastructure/repositories/pg/favorites_pg_repository.rs` |
| Interfaces | `favorites_handler` (free functions) | `src/interfaces/api/handlers/favorites_handler.rs` |

### DTO

```rust
pub struct FavoriteItemDto {
    pub id: String,
    pub user_id: String,
    pub item_id: String,
    pub item_type: String,          // "file" | "folder"
    pub created_at: DateTime<Utc>,
}
```

### REST API

All routes under `/api/favorites`, require authentication. User ID comes from the JWT token.

| Method | Path | Handler | Description |
|---|---|---|---|
| `GET` | `/api/favorites/` | `get_favorites` | List all favorites for current user |
| `POST` | `/api/favorites/{item_type}/{item_id}` | `add_favorite` | Add a file or folder to favorites |
| `DELETE` | `/api/favorites/{item_type}/{item_id}` | `remove_favorite` | Remove from favorites |

- `item_type` must be `"file"` or `"folder"` (validated by service)
- Adding a duplicate is idempotent (`ON CONFLICT DO NOTHING`)
- Results ordered by `created_at DESC`

### Database Schema

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

Indexes: `user_id`, `item_id`, `item_type`, `created_at`, composite `(user_id, item_type)`.

### Example

```bash
# Add file to favorites
curl -X POST -H "Authorization: Bearer $TOKEN" \
  "https://oxicloud.example.com/api/favorites/file/abc-123"

# List favorites
curl -H "Authorization: Bearer $TOKEN" \
  "https://oxicloud.example.com/api/favorites/"

# Remove from favorites
curl -X DELETE -H "Authorization: Bearer $TOKEN" \
  "https://oxicloud.example.com/api/favorites/folder/def-456"
```

---

## Recent Items

### Architecture

| Layer | Component | File |
|---|---|---|
| Application Port | **RecentItemsUseCase**, **RecentItemsRepositoryPort** | `src/application/ports/recent_ports.rs` |
| Application Service | **RecentService** | `src/application/services/recent_service.rs` |
| Application DTO | **RecentItemDto** | `src/application/dtos/recent_dto.rs` |
| Infrastructure | **RecentItemsPgRepository** | `src/infrastructure/repositories/pg/recent_items_pg_repository.rs` |
| Interfaces | `recent_handler` (free functions) | `src/interfaces/api/handlers/recent_handler.rs` |

### DTO

```rust
pub struct RecentItemDto {
    pub id: String,
    pub user_id: String,
    pub item_id: String,
    pub item_type: String,          // "file" | "folder"
    pub accessed_at: DateTime<Utc>,
}
```

### REST API

All routes under `/api/recent`, require authentication.

| Method | Path | Handler | Description |
|---|---|---|---|
| `GET` | `/api/recent/` | `get_recent_items` | List recent items (optional `?limit=N`) |
| `POST` | `/api/recent/{item_type}/{item_id}` | `record_item_access` | Record an access (upsert) |
| `DELETE` | `/api/recent/{item_type}/{item_id}` | `remove_from_recent` | Remove specific item |
| `DELETE` | `/api/recent/clear` | `clear_recent_items` | Clear all recent items |

### Behavior

- **Max items per user**: 50 (configured in DI, clamped to 1-100)
- **Upsert**: re-accessing an item updates its `accessed_at` timestamp
- **Auto-prune**: after recording access, old items beyond the limit are automatically pruned
- **Ordering**: results ordered by `accessed_at DESC`
- **Limit parameter**: `?limit=N` caps results (defaults to and cannot exceed **max_recent_items**)

### Database Schema

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

Indexes: `user_id`, `item_id`, `item_type`, `accessed_at`, composite `(user_id, accessed_at DESC)`.

### Example

```bash
# Record file access
curl -X POST -H "Authorization: Bearer $TOKEN" \
  "https://oxicloud.example.com/api/recent/file/abc-123"

# Get recent items (last 10)
curl -H "Authorization: Bearer $TOKEN" \
  "https://oxicloud.example.com/api/recent/?limit=10"

# Clear history
curl -X DELETE -H "Authorization: Bearer $TOKEN" \
  "https://oxicloud.example.com/api/recent/clear"
```

## DI Wiring

Both services require PostgreSQL:

```rust
// Favorites
let repo = Arc::new(FavoritesPgRepository::new(db_pool.clone()));
let favorites_service = Arc::new(FavoritesService::new(repo));

// Recent
let repo = Arc::new(RecentItemsPgRepository::new(db_pool.clone()));
let recent_service = Arc::new(RecentService::new(repo, 50)); // max 50 items
```

Stored as `Option<Arc<...>>` in **AppState** -- only available when DB is connected.
