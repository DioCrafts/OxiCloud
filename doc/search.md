# 12 - Search

File and folder search with multi-criteria filtering, recursive traversal, pagination, and in-memory result caching. Two modes: a simple `GET` with query parameters, or an advanced `POST` with a full criteria body.

Controlled by the feature flag **OXICLOUD_ENABLE_SEARCH** (default: `true`).

## Architecture

| Layer | Component | File |
|---|---|---|
| Application Port | **SearchUseCase** trait | `src/application/ports/inbound.rs` |
| Application Service | **SearchService** | `src/application/services/search_service.rs` |
| Application DTOs | **SearchCriteriaDto**, **SearchResultsDto** | `src/application/dtos/search_dto.rs` |
| Interfaces | **SearchHandler** | `src/interfaces/api/handlers/search_handler.rs` |

## Search Criteria

```rust
pub struct SearchCriteriaDto {
    pub name_contains: Option<String>,       // text to search in file/folder names
    pub file_types: Option<Vec<String>>,     // extensions, e.g. ["pdf", "jpg"]
    pub created_after: Option<u64>,          // epoch seconds
    pub created_before: Option<u64>,
    pub modified_after: Option<u64>,
    pub modified_before: Option<u64>,
    pub min_size: Option<u64>,               // bytes
    pub max_size: Option<u64>,
    pub folder_id: Option<String>,           // scope to a specific folder
    pub recursive: bool,                     // default: true
    pub limit: usize,                        // default: 100
    pub offset: usize,                       // default: 0
}
```

## Search Results

```rust
pub struct SearchResultsDto {
    pub files: Vec<FileDto>,
    pub folders: Vec<FolderDto>,
    pub total_count: Option<usize>,
    pub limit: usize,
    pub offset: usize,
    pub has_more: bool,
}
```

## REST API Endpoints

All routes under `/api/search`, require authentication.

| Method | Path | Handler | Description |
|---|---|---|---|
| `GET` | `/api/search/` | `SearchHandler::search_files_get` | Simple search via query params |
| `POST` | `/api/search/advanced` | `SearchHandler::search_files_post` | Advanced search via JSON body |
| `DELETE` | `/api/search/cache` | `SearchHandler::clear_search_cache` | Clear the result cache |

### GET Query Parameters

| Parameter | Type | Description |
|---|---|---|
| `query` | `String` | Text to search in names |
| `type` | `String` | File extension filter (comma-separated) |
| `created_after` | `u64` | Epoch seconds |
| `created_before` | `u64` | Epoch seconds |
| `modified_after` | `u64` | Epoch seconds |
| `modified_before` | `u64` | Epoch seconds |
| `min_size` | `u64` | Minimum size in bytes |
| `max_size` | `u64` | Maximum size in bytes |
| `folder_id` | `String` | Scope to folder |
| `recursive` | `bool` | Recursive search (default: true) |
| `limit` | `usize` | Max results (default: 100) |
| `offset` | `usize` | Pagination offset |

### Example

```bash
# Simple search
curl -H "Authorization: Bearer $TOKEN" \
  "https://oxicloud.example.com/api/search/?query=report&type=pdf&limit=20"

# Advanced search
curl -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name_contains":"report","file_types":["pdf","docx"],"min_size":1024,"recursive":true,"limit":50}' \
  "https://oxicloud.example.com/api/search/advanced"
```

## Implementation Details

### Search Algorithm

1. If `folder_id` is provided, start from that folder. Otherwise start from root.
2. When `recursive` is true, traverse all subfolders depth-first via `search_recursive()`.
3. Files are filtered by name match, extension, date range, and size range.
4. Folders are filtered by name match and date range.
5. Results are paginated with `offset`/`limit`.

### Caching

Results are cached in-memory using a hash of the search criteria + user ID as the key.

- **Cache TTL**: 5 minutes (300 seconds)
- **Max entries**: 1000
- **Cleanup**: background `tokio` task runs periodically, evicts expired entries
- **Manual clear**: `DELETE /api/search/cache`

## DI Wiring

```rust
// In di.rs — SearchService creation
let search_service = SearchService::new(
    repos.file_read_repository.clone(),
    repos.folder_repository.clone(),
    300,   // cache TTL seconds
    1000,  // max cache entries
);
```

Stored in `AppState.applications.search_service: Option<Arc<dyn SearchUseCase>>`.
