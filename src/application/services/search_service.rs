use async_trait::async_trait;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::application::dtos::display_helpers::{
    category_for, icon_class_for, icon_special_class_for,
};
use crate::application::dtos::file_dto::FileDto;
use crate::application::dtos::folder_dto::FolderDto;
use crate::application::dtos::search_dto::{
    SearchCriteriaDto, SearchFileResultDto, SearchFolderResultDto, SearchResultsDto,
    SearchSuggestionItem, SearchSuggestionsDto,
};
use crate::application::ports::inbound::SearchUseCase;
use crate::application::ports::outbound::FolderStoragePort;
use crate::application::ports::storage_ports::FileReadPort;
use crate::common::errors::Result;
use crate::domain::entities::folder::Folder;
use std::hash::{Hash, Hasher};

/**
 * High-performance search service implementation for files and folders.
 *
 * All search processing (filtering, scoring, sorting, categorization,
 * formatting) is performed server-side in Rust for maximum efficiency.
 * The frontend acts as a thin rendering client only.
 *
 * Features:
 * - Single-query recursive subtree search via PostgreSQL ltree
 * - Relevance scoring (exact match > starts-with > contains)
 * - Content categorization and icon mapping
 * - Multiple sort options (relevance, name, date, size)
 * - Server-side formatted file sizes
 * - Quick suggestions endpoint for autocomplete
 * - TTL-based result caching
 */
pub struct SearchService {
    /// Repository for file operations
    file_repository: Arc<dyn FileReadPort>,

    /// Repository for folder operations
    folder_repository: Arc<dyn FolderStoragePort>,

    /// Lock-free concurrent cache with automatic TTL and LRU eviction (moka).
    /// Values are `Arc<SearchResultsDto>` so cache insert/hit is a single
    /// atomic ref-count increment (~1 ns) instead of cloning thousands of Strings.
    search_cache: moka::future::Cache<u64, Arc<SearchResultsDto>>,
}

// ─── Utility functions (pure, no self — computed on the server) ─────────

/// Compute relevance score (0–100) for a name against a query.
/// Exact match = 100, starts-with = 80, contains = 50, no match = 0.
fn compute_relevance(name: &str, query: &str) -> u32 {
    let name_lower = name.to_lowercase();
    let query_lower = query.to_lowercase();

    if name_lower == query_lower {
        100
    } else if name_lower.starts_with(&query_lower) {
        80
    } else if name_lower.contains(&query_lower) {
        // Bonus for shorter names (more specific match)
        let ratio = query_lower.len() as f64 / name_lower.len() as f64;
        50 + (ratio * 20.0) as u32
    } else {
        0
    }
}

/// Format bytes into a human-readable string (e.g. "2.5 MB").
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    if bytes == 0 {
        return "0 B".to_string();
    }
    let exp = (bytes as f64).log(1024.0).floor() as usize;
    let exp = exp.min(UNITS.len() - 1);
    let value = bytes as f64 / 1024_f64.powi(exp as i32);
    if exp == 0 {
        format!("{} B", bytes)
    } else {
        format!("{:.1} {}", value, UNITS[exp])
    }
}

/// Get Font Awesome icon class for a file based on extension and MIME type.
/// Delegates to the centralised `display_helpers` so every API surface is
/// consistent.
fn get_icon_class(name: &str, mime: &str) -> String {
    icon_class_for(name, mime).to_string()
}

/// Get CSS special class for icon styling.
fn get_icon_special_class(name: &str, mime: &str) -> String {
    icon_special_class_for(name, mime).to_string()
}

/// Get category label from centralised helpers.
fn get_category(name: &str, mime: &str) -> String {
    category_for(name, mime).to_string()
}

// ─── SearchService implementation ───────────────────────────────────────

impl SearchService {
    /**
     * Creates a new instance of the search service.
     */
    pub fn new(
        file_repository: Arc<dyn FileReadPort>,
        folder_repository: Arc<dyn FolderStoragePort>,
        cache_ttl: u64,
        max_cache_size: usize,
    ) -> Self {
        let search_cache = moka::future::Cache::builder()
            .max_capacity(max_cache_size as u64)
            .time_to_live(Duration::from_secs(cache_ttl))
            .build();

        Self {
            file_repository,
            folder_repository,
            search_cache,
        }
    }

    /// Creates a cache key from the search criteria using zero-allocation hashing.
    fn create_cache_key(criteria: &SearchCriteriaDto, user_id: &str) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        criteria.hash(&mut hasher);
        user_id.hash(&mut hasher);
        hasher.finish()
    }

    /// Attempts to retrieve results from the cache.
    async fn get_from_cache(&self, key: u64) -> Option<Arc<SearchResultsDto>> {
        self.search_cache.get(&key).await
    }

    /// Stores results in the cache.
    async fn store_in_cache(&self, key: u64, results: Arc<SearchResultsDto>) {
        self.search_cache.insert(key, results).await;
    }

    /// Enrich a FileDto → SearchFileResultDto with server-computed metadata.
    fn enrich_file(file: &FileDto, query: &str) -> SearchFileResultDto {
        let relevance = if query.is_empty() {
            50
        } else {
            compute_relevance(&file.name, query)
        };

        SearchFileResultDto {
            id: file.id.clone(),
            name: file.name.clone(),
            path: file.path.clone(),
            size: file.size,
            mime_type: file.mime_type.clone(),
            folder_id: file.folder_id.clone(),
            created_at: file.created_at,
            modified_at: file.modified_at,
            relevance_score: relevance,
            size_formatted: format_bytes(file.size),
            icon_class: get_icon_class(&file.name, &file.mime_type),
            icon_special_class: get_icon_special_class(&file.name, &file.mime_type),
            category: get_category(&file.name, &file.mime_type),
        }
    }

    /// Enrich a FolderDto → SearchFolderResultDto with server-computed metadata.
    fn enrich_folder(folder: &FolderDto, query: &str) -> SearchFolderResultDto {
        let relevance = if query.is_empty() {
            50
        } else {
            compute_relevance(&folder.name, query)
        };

        SearchFolderResultDto {
            id: folder.id.clone(),
            name: folder.name.clone(),
            path: folder.path.clone(),
            parent_id: folder.parent_id.clone(),
            created_at: folder.created_at,
            modified_at: folder.modified_at,
            is_root: folder.is_root,
            relevance_score: relevance,
        }
    }

    /// Quick suggestions search — returns up to `limit` name suggestions
    /// matching the query. Pushes filtering, relevance sort and LIMIT to SQL
    /// so only a handful of rows cross the DB→app boundary.
    pub async fn suggest(
        &self,
        query: &str,
        folder_id: Option<&str>,
        limit: usize,
    ) -> Result<SearchSuggestionsDto> {
        let start = Instant::now();

        // Ask SQL for at most `limit` best-matching files and folders
        let (files, folders) = tokio::join!(
            self.file_repository
                .suggest_files_by_name(folder_id, query, limit),
            self.folder_repository
                .suggest_folders_by_name(folder_id, query, limit),
        );
        let files = files?;
        let folders = folders?;

        let mut suggestions: Vec<SearchSuggestionItem> =
            Vec::with_capacity(files.len() + folders.len());

        for file in &files {
            let file_dto = FileDto::from(file.clone());
            let score = compute_relevance(&file_dto.name, query);
            suggestions.push(SearchSuggestionItem {
                name: file_dto.name.clone(),
                item_type: "file".to_string(),
                id: file_dto.id.clone(),
                path: file_dto.path.clone(),
                icon_class: get_icon_class(&file_dto.name, &file_dto.mime_type),
                icon_special_class: get_icon_special_class(&file_dto.name, &file_dto.mime_type),
                relevance_score: score,
            });
        }

        for folder in &folders {
            let folder_dto = FolderDto::from(folder.clone());
            let score = compute_relevance(&folder_dto.name, query);
            suggestions.push(SearchSuggestionItem {
                name: folder_dto.name.clone(),
                item_type: "folder".to_string(),
                id: folder_dto.id.clone(),
                path: folder_dto.path.clone(),
                icon_class: "fas fa-folder".to_string(),
                icon_special_class: "folder-icon".to_string(),
                relevance_score: score,
            });
        }

        // Merge files + folders by relevance and truncate to the final limit
        suggestions.sort_by(|a, b| b.relevance_score.cmp(&a.relevance_score));
        suggestions.truncate(limit);

        let elapsed = start.elapsed().as_millis() as u64;
        Ok(SearchSuggestionsDto {
            suggestions,
            query_time_ms: elapsed,
        })
    }
}

// ─── SearchUseCase trait implementation ──────────────────────────────────

#[async_trait]
impl SearchUseCase for SearchService {
    /**
     * Performs a search based on the specified criteria.
     *
     * Optimization: For non-recursive searches, uses database-level pagination
     * for better performance. For recursive searches, uses the parallel approach.
     *
     * All processing happens server-side:
     * - Database-level pagination for non-recursive searches
     * - Parallel recursive traversal for recursive searches
     * - Filtering by name, type, dates, size
     * - Relevance scoring
     * - Sorting (relevance, name, date, size)
     * - Content categorization & icon mapping
     * - Human-readable size formatting
     * - Pagination
     */
    async fn search(
        &self,
        criteria: SearchCriteriaDto,
        user_id: &str,
    ) -> Result<Arc<SearchResultsDto>> {
        let start = Instant::now();

        // Try to get from cache
        let cache_key = Self::create_cache_key(&criteria, user_id);
        if let Some(cached_results) = self.get_from_cache(cache_key).await {
            return Ok(cached_results);
        }

        let query = criteria.name_contains.as_deref().unwrap_or("");

        // For non-recursive searches, use efficient database-level pagination
        // This avoids loading all files into memory
        if !criteria.recursive {
            // Use database-level pagination
            let (files, total_file_count) = self
                .file_repository
                .search_files_paginated(criteria.folder_id.as_deref(), &criteria, user_id)
                .await?;

            // Convert to DTOs and enrich with metadata
            let file_dtos: Vec<FileDto> = files.into_iter().map(FileDto::from).collect();
            let enriched_files: Vec<SearchFileResultDto> = file_dtos
                .iter()
                .map(|f| Self::enrich_file(f, query))
                .collect();

            // Get folders for this folder (non-recursive, filtered in SQL)
            let folders = self
                .folder_repository
                .search_folders(
                    criteria.folder_id.as_deref(),
                    criteria.name_contains.as_deref(),
                    user_id,
                    false,
                )
                .await?;

            let filtered_folders: Vec<FolderDto> =
                folders.into_iter().map(FolderDto::from).collect();

            // For folders, apply sorting and pagination in memory (usually fewer folders)
            let mut enriched_folders: Vec<SearchFolderResultDto> = filtered_folders
                .iter()
                .map(|f| Self::enrich_folder(f, query))
                .collect();

            // Sort folders
            match criteria.sort_by.as_str() {
                "name" => {
                    enriched_folders
                        .sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
                }
                "name_desc" => {
                    enriched_folders
                        .sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase()));
                }
                "date" => {
                    enriched_folders.sort_by(|a, b| a.modified_at.cmp(&b.modified_at));
                }
                "date_desc" => {
                    enriched_folders.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
                }
                _ => {
                    enriched_folders.sort_by(|a, b| b.relevance_score.cmp(&a.relevance_score));
                }
            }

            let folder_count = enriched_folders.len();
            let total_count = total_file_count + folder_count;

            // Combine and paginate (folders first, then files)
            let start_idx = criteria.offset.min(total_count);
            let end_idx = (criteria.offset + criteria.limit).min(total_count);

            let folder_start = start_idx.min(folder_count);
            let folder_end = end_idx.min(folder_count);
            let paginated_folders = enriched_folders[folder_start..folder_end].to_vec();

            let file_start = start_idx.saturating_sub(folder_count);
            let file_end = end_idx
                .saturating_sub(folder_count)
                .min(enriched_files.len());
            let paginated_files = enriched_files[file_start..file_end].to_vec();

            let elapsed_ms = start.elapsed().as_millis() as u64;

            let search_results = Arc::new(SearchResultsDto::new(
                paginated_files,
                paginated_folders,
                criteria.limit,
                criteria.offset,
                Some(total_count),
                elapsed_ms,
                criteria.sort_by.clone(),
            ));

            self.store_in_cache(cache_key, Arc::clone(&search_results))
                .await;
            return Ok(search_results);
        }

        // ── Recursive search via ltree (single SQL query per entity type) ──
        // Uses PostgreSQL ltree GiST index to find all files and folders
        // in the subtree in O(1) queries, replacing the O(N) spawn-per-folder
        // approach that could saturate the connection pool.
        let (found_files, total_file_count) = self
            .file_repository
            .search_files_in_subtree(criteria.folder_id.as_deref(), &criteria, user_id)
            .await?;

        // Get folders (SQL-filtered, user-scoped, recursive when applicable)
        let found_folders: Vec<Folder> = self
            .folder_repository
            .search_folders(
                criteria.folder_id.as_deref(),
                criteria.name_contains.as_deref(),
                user_id,
                true,
            )
            .await?;

        // ── Convert to DTOs and enrich with server-computed metadata ──
        let file_dtos: Vec<FileDto> = found_files.into_iter().map(FileDto::from).collect();
        let enriched_files: Vec<SearchFileResultDto> = file_dtos
            .iter()
            .map(|f| Self::enrich_file(f, query))
            .collect();

        let folder_dtos: Vec<FolderDto> = found_folders.into_iter().map(FolderDto::from).collect();
        let mut enriched_folders: Vec<SearchFolderResultDto> = folder_dtos
            .iter()
            .map(|f| Self::enrich_folder(f, query))
            .collect();

        // ── Sort folders (files already sorted by SQL ORDER BY) ──
        match criteria.sort_by.as_str() {
            "name" => {
                enriched_folders.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            }
            "name_desc" => {
                enriched_folders.sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase()));
            }
            "date" => {
                enriched_folders.sort_by(|a, b| a.modified_at.cmp(&b.modified_at));
            }
            "date_desc" => {
                enriched_folders.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
            }
            _ => {
                enriched_folders.sort_by(|a, b| b.relevance_score.cmp(&a.relevance_score));
            }
        }

        // ── Pagination (folders first, then files) ──
        let folder_count = enriched_folders.len();
        let total_count = total_file_count + folder_count;
        let start_idx = criteria.offset.min(total_count);
        let end_idx = (criteria.offset + criteria.limit).min(total_count);

        let folder_start = start_idx.min(folder_count);
        let folder_end = end_idx.min(folder_count);
        let paginated_folders = enriched_folders[folder_start..folder_end].to_vec();

        let file_start = start_idx.saturating_sub(folder_count);
        let file_end = end_idx
            .saturating_sub(folder_count)
            .min(enriched_files.len());
        let paginated_files = enriched_files[file_start..file_end].to_vec();

        let elapsed_ms = start.elapsed().as_millis() as u64;

        let search_results = Arc::new(SearchResultsDto::new(
            paginated_files,
            paginated_folders,
            criteria.limit,
            criteria.offset,
            Some(total_count),
            elapsed_ms,
            criteria.sort_by.clone(),
        ));

        // Store in cache — Arc::clone is ~1 ns (atomic increment)
        self.store_in_cache(cache_key, Arc::clone(&search_results))
            .await;

        Ok(search_results)
    }

    /// Returns quick suggestions for autocomplete.
    async fn suggest(
        &self,
        query: &str,
        folder_id: Option<&str>,
        limit: usize,
    ) -> Result<SearchSuggestionsDto> {
        self.suggest(query, folder_id, limit).await
    }

    /// Clears the search results cache.
    async fn clear_search_cache(&self) -> Result<()> {
        self.search_cache.invalidate_all();
        self.search_cache.run_pending_tasks().await;
        Ok(())
    }
}

// ─── Stub for testing ────────────────────────────────────────────────────

impl SearchService {
    /// Creates a stub version of the service for testing
    pub fn new_stub() -> impl SearchUseCase {
        struct SearchServiceStub;

        #[async_trait]
        impl SearchUseCase for SearchServiceStub {
            async fn search(
                &self,
                _criteria: SearchCriteriaDto,
                _user_id: &str,
            ) -> Result<Arc<SearchResultsDto>> {
                Ok(Arc::new(SearchResultsDto::empty()))
            }

            async fn suggest(
                &self,
                _query: &str,
                _folder_id: Option<&str>,
                _limit: usize,
            ) -> Result<SearchSuggestionsDto> {
                Ok(SearchSuggestionsDto {
                    suggestions: Vec::new(),
                    query_time_ms: 0,
                })
            }

            async fn clear_search_cache(&self) -> Result<()> {
                Ok(())
            }
        }

        SearchServiceStub
    }
}
