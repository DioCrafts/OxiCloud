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
use crate::domain::errors::DomainError;

/**
 * High-performance search service implementation for files and folders.
 *
 * All search processing (filtering, scoring, sorting, categorization,
 * formatting) is performed server-side in Rust for maximum efficiency.
 * The frontend acts as a thin rendering client only.
 *
 * Features:
 * - Parallel recursive folder traversal using tokio tasks
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

    /// Lock-free concurrent cache with automatic TTL and LRU eviction (moka)
    search_cache: moka::sync::Cache<SearchCacheKey, SearchResultsDto>,
}

/// Key for the search cache
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SearchCacheKey {
    /// Serialized representation of the search criteria
    criteria_hash: String,

    /// User ID (to isolate searches between users)
    user_id: String,
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
        let search_cache = moka::sync::Cache::builder()
            .max_capacity(max_cache_size as u64)
            .time_to_live(Duration::from_secs(cache_ttl))
            .build();

        Self {
            file_repository,
            folder_repository,
            search_cache,
        }
    }

    /// Creates a cache key from the search criteria.
    fn create_cache_key(
        &self,
        criteria: &SearchCriteriaDto,
        user_id: &str,
    ) -> Result<SearchCacheKey> {
        let criteria_str = serde_json::to_string(criteria).map_err(|e| {
            DomainError::internal_error(
                "SearchService",
                format!("Failed to serialize criteria: {}", e),
            )
        })?;
        Ok(SearchCacheKey {
            criteria_hash: criteria_str,
            user_id: user_id.to_string(),
        })
    }

    /// Attempts to retrieve results from the cache.
    fn get_from_cache(&self, key: &SearchCacheKey) -> Option<SearchResultsDto> {
        self.search_cache.get(key)
    }

    /// Stores results in the cache.
    fn store_in_cache(&self, key: SearchCacheKey, results: SearchResultsDto) {
        self.search_cache.insert(key, results);
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

    /**
     * Parallel recursive search through folders using tokio tasks.
     *
     * Instead of searching subfolders sequentially, we spawn a task
     * per subfolder and join them all concurrently.
     */
    fn search_parallel(
        file_repo: Arc<dyn FileReadPort>,
        folder_repo: Arc<dyn FolderStoragePort>,
        current_folder_id: Option<String>,
        criteria: Arc<SearchCriteriaDto>,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(Vec<FileDto>, Vec<FolderDto>)>> + Send>,
    > {
        Box::pin(async move {
            // List files in the current folder
            let files = file_repo.list_files(current_folder_id.as_deref()).await?;

            let filtered_files: Vec<FileDto> = files
                .into_iter()
                .map(FileDto::from)
                .filter(|file| passes_file_filter(file, &criteria))
                .collect();

            let mut all_files = filtered_files;
            let mut all_folders: Vec<FolderDto> = Vec::new();

            // If recursive, process subfolders in parallel
            if criteria.recursive {
                let folders = folder_repo
                    .list_folders(current_folder_id.as_deref())
                    .await?;

                let folder_dtos: Vec<FolderDto> = folders
                    .into_iter()
                    .map(FolderDto::from)
                    .filter(|f| passes_folder_filter(f, &criteria))
                    .collect();

                all_folders.extend(folder_dtos.iter().cloned());

                // Spawn parallel tasks for each subfolder
                let mut handles = Vec::with_capacity(folder_dtos.len());
                for subfolder in &folder_dtos {
                    let fr = file_repo.clone();
                    let fdr = folder_repo.clone();
                    let crit = criteria.clone();
                    let folder_id = subfolder.id.clone();

                    handles.push(tokio::spawn(async move {
                        Self::search_parallel(fr, fdr, Some(folder_id), crit).await
                    }));
                }

                // Collect results from all parallel tasks
                for handle in handles {
                    match handle.await {
                        Ok(Ok((sub_files, sub_folders))) => {
                            all_files.extend(sub_files);
                            all_folders.extend(sub_folders);
                        }
                        Ok(Err(e)) => {
                            tracing::warn!("Parallel search subtask error: {}", e);
                        }
                        Err(e) => {
                            tracing::warn!("Parallel search task join error: {}", e);
                        }
                    }
                }
            }

            Ok((all_files, all_folders))
        })
    }

    /// Quick suggestions search — returns up to `limit` name suggestions
    /// matching the query prefix. Uses cache-friendly shallow search.
    pub async fn suggest(
        &self,
        query: &str,
        folder_id: Option<&str>,
        limit: usize,
    ) -> Result<SearchSuggestionsDto> {
        let start = Instant::now();
        let query_lower = query.to_lowercase();

        let mut suggestions: Vec<SearchSuggestionItem> = Vec::new();

        // List files in the folder
        let files = self.file_repository.list_files(folder_id).await?;
        for file in files {
            let file_dto = FileDto::from(file);
            if file_dto.name.to_lowercase().contains(&query_lower) {
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
            if suggestions.len() >= limit * 2 {
                break; // Collect enough candidates
            }
        }

        // List folders
        let folders = self.folder_repository.list_folders(folder_id).await?;
        for folder in folders {
            let folder_dto = FolderDto::from(folder);
            if folder_dto.name.to_lowercase().contains(&query_lower) {
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
        }

        // Sort by relevance and truncate
        suggestions.sort_by(|a, b| b.relevance_score.cmp(&a.relevance_score));
        suggestions.truncate(limit);

        let elapsed = start.elapsed().as_millis() as u64;
        Ok(SearchSuggestionsDto {
            suggestions,
            query_time_ms: elapsed,
        })
    }
}

// ─── Standalone filter functions for use in parallel tasks ──────────────

/// Check if a file passes all filter criteria (standalone, no &self needed).
fn passes_file_filter(file: &FileDto, criteria: &SearchCriteriaDto) -> bool {
    if let Some(name_query) = &criteria.name_contains
        && !file
            .name
            .to_lowercase()
            .contains(&name_query.to_lowercase())
    {
        return false;
    }
    if let Some(file_types) = &criteria.file_types {
        if let Some(extension) = file.name.split('.').next_back() {
            if !file_types
                .iter()
                .any(|ext| ext.eq_ignore_ascii_case(extension))
            {
                return false;
            }
        } else {
            return false;
        }
    }
    if let Some(v) = criteria.created_after {
        if file.created_at < v {
            return false;
        }
    }
    if let Some(v) = criteria.created_before {
        if file.created_at > v {
            return false;
        }
    }
    if let Some(v) = criteria.modified_after {
        if file.modified_at < v {
            return false;
        }
    }
    if let Some(v) = criteria.modified_before {
        if file.modified_at > v {
            return false;
        }
    }
    if let Some(v) = criteria.min_size {
        if file.size < v {
            return false;
        }
    }
    if let Some(v) = criteria.max_size {
        if file.size > v {
            return false;
        }
    }
    true
}

/// Check if a folder passes all filter criteria (standalone).
fn passes_folder_filter(folder: &FolderDto, criteria: &SearchCriteriaDto) -> bool {
    if let Some(name_query) = &criteria.name_contains
        && !folder
            .name
            .to_lowercase()
            .contains(&name_query.to_lowercase())
    {
        return false;
    }
    if let Some(v) = criteria.created_after {
        if folder.created_at < v {
            return false;
        }
    }
    if let Some(v) = criteria.created_before {
        if folder.created_at > v {
            return false;
        }
    }
    if let Some(v) = criteria.modified_after {
        if folder.modified_at < v {
            return false;
        }
    }
    if let Some(v) = criteria.modified_before {
        if folder.modified_at > v {
            return false;
        }
    }
    true
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
    async fn search(&self, criteria: SearchCriteriaDto) -> Result<SearchResultsDto> {
        let start = Instant::now();

        // TODO: Get user ID from the authentication context
        let user_id = "default-user";

        // Try to get from cache
        let cache_key = self.create_cache_key(&criteria, user_id).ok();
        if let Some(ref key) = cache_key {
            if let Some(cached_results) = self.get_from_cache(key) {
                return Ok(cached_results);
            }
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

            // Get folders for this folder (non-recursive)
            let folders = self
                .folder_repository
                .list_folders(criteria.folder_id.as_deref())
                .await?;

            // Filter folders if name criteria present
            let filtered_folders: Vec<FolderDto> = if let Some(name_query) = &criteria.name_contains
            {
                let query_lower = name_query.to_lowercase();
                folders
                    .into_iter()
                    .map(FolderDto::from)
                    .filter(|f| {
                        let folder_name_lower = f.name.to_lowercase();
                        folder_name_lower.contains(&query_lower)
                    })
                    .collect()
            } else {
                folders.into_iter().map(FolderDto::from).collect()
            };

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

            let mut paginated_folders = Vec::new();
            let mut paginated_files = Vec::new();

            for i in start_idx..end_idx {
                if i < folder_count {
                    paginated_folders.push(enriched_folders[i].clone());
                } else {
                    let file_idx = i - folder_count;
                    if file_idx < enriched_files.len() {
                        paginated_files.push(enriched_files[file_idx].clone());
                    }
                }
            }

            let elapsed_ms = start.elapsed().as_millis() as u64;

            let search_results = SearchResultsDto::new(
                paginated_files,
                paginated_folders,
                criteria.limit,
                criteria.offset,
                Some(total_count),
                elapsed_ms,
                criteria.sort_by.clone(),
            );

            if let Some(key) = cache_key {
                self.store_in_cache(key, search_results.clone());
            }
            return Ok(search_results);
        }

        // ── Recursive search (fallback to original parallel approach) ──
        // For recursive searches, we need to traverse all subfolders
        // This is less efficient but necessary for recursive functionality
        let criteria_arc = Arc::new(criteria.clone());
        let (found_files, found_folders): (Vec<FileDto>, Vec<FolderDto>) = Self::search_parallel(
            self.file_repository.clone(),
            self.folder_repository.clone(),
            criteria.folder_id.clone(),
            criteria_arc,
        )
        .await?;

        // ── Enrich results with server-computed metadata ──
        let mut enriched_files: Vec<SearchFileResultDto> = found_files
            .iter()
            .map(|f| Self::enrich_file(f, query))
            .collect();

        let mut enriched_folders: Vec<SearchFolderResultDto> = found_folders
            .iter()
            .map(|f| Self::enrich_folder(f, query))
            .collect();

        // ── Sort based on criteria.sort_by ──
        match criteria.sort_by.as_str() {
            "name" => {
                enriched_files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
                enriched_folders.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            }
            "name_desc" => {
                enriched_files.sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase()));
                enriched_folders.sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase()));
            }
            "date" => {
                enriched_files.sort_by(|a, b| a.modified_at.cmp(&b.modified_at));
                enriched_folders.sort_by(|a, b| a.modified_at.cmp(&b.modified_at));
            }
            "date_desc" => {
                enriched_files.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
                enriched_folders.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
            }
            "size" => {
                enriched_files.sort_by(|a, b| a.size.cmp(&b.size));
            }
            "size_desc" => {
                enriched_files.sort_by(|a, b| b.size.cmp(&a.size));
            }
            _ => {
                // "relevance" (default) — highest relevance first, tie-break by date desc
                enriched_files.sort_by(|a, b| {
                    b.relevance_score
                        .cmp(&a.relevance_score)
                        .then_with(|| b.modified_at.cmp(&a.modified_at))
                });
                enriched_folders.sort_by(|a, b| {
                    b.relevance_score
                        .cmp(&a.relevance_score)
                        .then_with(|| b.modified_at.cmp(&a.modified_at))
                });
            }
        }

        // ── Pagination ──
        let total_count = enriched_files.len() + enriched_folders.len();
        let start_idx = criteria.offset.min(total_count);
        let end_idx = (criteria.offset + criteria.limit).min(total_count);

        let paginated_items: Vec<(bool, usize)> = (start_idx..end_idx)
            .map(|i| {
                if i < enriched_folders.len() {
                    (true, i) // folder
                } else {
                    (false, i - enriched_folders.len()) // file
                }
            })
            .collect();

        let mut paginated_folders = Vec::new();
        let mut paginated_files = Vec::new();

        for (is_folder, idx) in paginated_items {
            if is_folder {
                if idx < enriched_folders.len() {
                    paginated_folders.push(enriched_folders[idx].clone());
                }
            } else if idx < enriched_files.len() {
                paginated_files.push(enriched_files[idx].clone());
            }
        }

        let elapsed_ms = start.elapsed().as_millis() as u64;

        let search_results = SearchResultsDto::new(
            paginated_files,
            paginated_folders,
            criteria.limit,
            criteria.offset,
            Some(total_count),
            elapsed_ms,
            criteria.sort_by.clone(),
        );

        // Store in cache
        if let Some(key) = cache_key {
            self.store_in_cache(key, search_results.clone());
        }

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
            async fn search(&self, _criteria: SearchCriteriaDto) -> Result<SearchResultsDto> {
                Ok(SearchResultsDto::empty())
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
