use std::sync::Arc;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::Mutex;
use async_trait::async_trait;
use tokio::time;

use crate::common::errors::Result;
use crate::application::dtos::search_dto::{SearchCriteriaDto, SearchResultsDto};
use crate::application::dtos::file_dto::FileDto;
use crate::application::dtos::folder_dto::FolderDto;
use crate::application::ports::inbound::SearchUseCase;
use crate::application::ports::outbound::FolderStoragePort;
use crate::application::ports::storage_ports::FileReadPort;

/**
 * Search service implementation for files and folders.
 * 
 * This service implements the advanced search functionality that allows
 * users to find files and folders based on various criteria
 * such as name, type, date and size. It also includes a cache to improve
 * the performance of repeated searches.
 */
pub struct SearchService {
    /// Repository for file operations
    file_repository: Arc<dyn FileReadPort>,
    
    /// Repository for folder operations
    folder_repository: Arc<dyn FolderStoragePort>,
    
    /// Search results cache with expiration time
    search_cache: Arc<Mutex<HashMap<SearchCacheKey, CachedSearchResult>>>,
    
    /// Cache validity duration in seconds
    cache_ttl: u64,
    
    /// Maximum cache size (number of stored results)
    max_cache_size: usize,
}

/// Key for the search cache
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SearchCacheKey {
    /// Serialized representation of the search criteria
    criteria_hash: String,
    
    /// User ID (to isolate searches between users)
    user_id: String,
}

/// Cached search result with expiration time
struct CachedSearchResult {
    /// Search results
    results: SearchResultsDto,
    
    /// Time when the cache entry was created
    timestamp: Instant,
}

impl SearchService {
    /**
     * Creates a new instance of the search service.
     * 
     * @param file_repository Repository for file operations
     * @param folder_repository Repository for folder operations
     * @param cache_ttl Cache time-to-live in seconds (0 to disable)
     * @param max_cache_size Maximum cache size
     */
    pub fn new(
        file_repository: Arc<dyn FileReadPort>,
        folder_repository: Arc<dyn FolderStoragePort>,
        cache_ttl: u64,
        max_cache_size: usize,
    ) -> Self {
        let search_service = Self {
            file_repository,
            folder_repository,
            search_cache: Arc::new(Mutex::new(HashMap::new())),
            cache_ttl,
            max_cache_size,
        };
        
        // Start cache cleanup task if TTL > 0
        if cache_ttl > 0 {
            Self::start_cache_cleanup_task(search_service.search_cache.clone(), cache_ttl);
        }
        
        search_service
    }
    
    /**
     * Starts an asynchronous task to clean up expired cache entries.
     * 
     * @param cache_ref Reference to the shared cache
     * @param ttl_seconds TTL in seconds
     */
    fn start_cache_cleanup_task(
        cache_ref: Arc<Mutex<HashMap<SearchCacheKey, CachedSearchResult>>>,
        ttl_seconds: u64,
    ) {
        tokio::spawn(async move {
            let cleanup_interval = Duration::from_secs(ttl_seconds / 2);
            let ttl = Duration::from_secs(ttl_seconds);
            
            loop {
                time::sleep(cleanup_interval).await;
                
                // Acquire lock and clean up expired entries
                if let Ok(mut cache) = cache_ref.lock() {
                    let now = Instant::now();
                    
                    // Identify expired entries
                    let expired_keys: Vec<SearchCacheKey> = cache
                        .iter()
                        .filter(|(_, result)| now.duration_since(result.timestamp) > ttl)
                        .map(|(key, _)| key.clone())
                        .collect();
                    
                    // Remove expired entries
                    for key in expired_keys {
                        cache.remove(&key);
                    }
                }
            }
        });
    }
    
    /**
     * Creates a cache key from the search criteria.
     * 
     * @param criteria Search criteria
     * @param user_id User ID (to isolate cache between users)
     * @return Cache key
     */
    fn create_cache_key(&self, criteria: &SearchCriteriaDto, user_id: &str) -> SearchCacheKey {
        // Serialize criteria to generate a hash
        let criteria_str = serde_json::to_string(criteria).unwrap_or_default();
        
        SearchCacheKey {
            criteria_hash: criteria_str,
            user_id: user_id.to_string(),
        }
    }
    
    /**
     * Attempts to retrieve results from the cache.
     * 
     * @param key Cache key
     * @return Optionally, the results if they exist and have not expired
     */
    fn get_from_cache(&self, key: &SearchCacheKey) -> Option<SearchResultsDto> {
        // If TTL is 0, the cache is disabled
        if self.cache_ttl == 0 {
            return None;
        }
        
        if let Ok(cache) = self.search_cache.lock() {
            if let Some(cached_result) = cache.get(key) {
                let now = Instant::now();
                let ttl = Duration::from_secs(self.cache_ttl);
                
                // Check if the entry has expired
                if now.duration_since(cached_result.timestamp) < ttl {
                    return Some(cached_result.results.clone());
                }
            }
        }
        
        None
    }
    
    /**
     * Stores results in the cache.
     * 
     * @param key Cache key
     * @param results Results to store
     */
    fn store_in_cache(&self, key: SearchCacheKey, results: SearchResultsDto) {
        // If TTL is 0, the cache is disabled
        if self.cache_ttl == 0 {
            return;
        }
        
        if let Ok(mut cache) = self.search_cache.lock() {
            // If the cache is full, remove the oldest entry
            if cache.len() >= self.max_cache_size {
                if let Some((oldest_key, _)) = cache
                    .iter()
                    .min_by_key(|(_, result)| result.timestamp) {
                    let key_to_remove = oldest_key.clone();
                    cache.remove(&key_to_remove);
                }
            }
            
            // Store the new result
            cache.insert(key, CachedSearchResult {
                results,
                timestamp: Instant::now(),
            });
        }
    }
    
    /**
     * Filters files according to the search criteria.
     * 
     * @param files List of files to filter
     * @param criteria Search criteria
     * @return Files that match the criteria
     */
    fn filter_files(&self, files: Vec<FileDto>, criteria: &SearchCriteriaDto) -> Vec<FileDto> {
        files.into_iter()
            .filter(|file| {
                // Filter by name
                if let Some(name_query) = &criteria.name_contains {
                    if !file.name.to_lowercase().contains(&name_query.to_lowercase()) {
                        return false;
                    }
                }
                
                // Filter by file type (extension)
                if let Some(file_types) = &criteria.file_types {
                    if let Some(extension) = file.name.split('.').last() {
                        if !file_types.iter().any(|ext| ext.eq_ignore_ascii_case(extension)) {
                            return false;
                        }
                    } else {
                        // Has no extension
                        return false;
                    }
                }
                
                // Filter by creation date
                if let Some(created_after) = criteria.created_after {
                    if file.created_at < created_after {
                        return false;
                    }
                }
                
                if let Some(created_before) = criteria.created_before {
                    if file.created_at > created_before {
                        return false;
                    }
                }
                
                // Filter by modification date
                if let Some(modified_after) = criteria.modified_after {
                    if file.modified_at < modified_after {
                        return false;
                    }
                }
                
                if let Some(modified_before) = criteria.modified_before {
                    if file.modified_at > modified_before {
                        return false;
                    }
                }
                
                // Filter by size
                if let Some(min_size) = criteria.min_size {
                    if file.size < min_size {
                        return false;
                    }
                }
                
                if let Some(max_size) = criteria.max_size {
                    if file.size > max_size {
                        return false;
                    }
                }
                
                true
            })
            .collect()
    }
    
    /**
     * Filters folders according to the search criteria.
     * 
     * @param folders List of folders to filter
     * @param criteria Search criteria
     * @return Folders that match the criteria
     */
    fn filter_folders(&self, folders: Vec<FolderDto>, criteria: &SearchCriteriaDto) -> Vec<FolderDto> {
        folders.into_iter()
            .filter(|folder| {
                // Filter by name
                if let Some(name_query) = &criteria.name_contains {
                    if !folder.name.to_lowercase().contains(&name_query.to_lowercase()) {
                        return false;
                    }
                }
                
                // Filter by creation date
                if let Some(created_after) = criteria.created_after {
                    if folder.created_at < created_after {
                        return false;
                    }
                }
                
                if let Some(created_before) = criteria.created_before {
                    if folder.created_at > created_before {
                        return false;
                    }
                }
                
                // Filter by modification date
                if let Some(modified_after) = criteria.modified_after {
                    if folder.modified_at < modified_after {
                        return false;
                    }
                }
                
                if let Some(modified_before) = criteria.modified_before {
                    if folder.modified_at > modified_before {
                        return false;
                    }
                }
                
                true
            })
            .collect()
    }
    
    /**
     * Implementation of recursive search through folders.
     * 
     * @param current_folder_id ID of the current folder
     * @param criteria Search criteria
     * @param found_files Files found so far
     * @param found_folders Folders found so far
     */
    async fn search_recursive(
        &self,
        current_folder_id: Option<&str>,
        criteria: &SearchCriteriaDto,
        found_files: &mut Vec<FileDto>,
        found_folders: &mut Vec<FolderDto>,
    ) -> Result<()> {
        Box::pin(async move {
        // List files in the current folder
        let files = self.file_repository.list_files(current_folder_id).await?;
        
        // Filter files according to criteria and add them to the results
        let filtered_files = self.filter_files(
            files.into_iter().map(FileDto::from).collect(), 
            criteria
        );
        found_files.extend(filtered_files);
        
        // If the search is recursive, process subfolders
        if criteria.recursive {
            // List subfolders
            let folders = self.folder_repository.list_folders(current_folder_id).await?;
            
            // Filter folders according to criteria and add them to the results
            let filtered_folders: Vec<FolderDto> = self.filter_folders(
                folders.into_iter().map(FolderDto::from).collect(),
                criteria
            );
            
            // Add filtered folders to the results
            found_folders.extend(filtered_folders.iter().cloned());
            
            // Search recursively in each subfolder
            for folder in filtered_folders {
                self.search_recursive(
                    Some(&folder.id),
                    criteria,
                    found_files,
                    found_folders,
                ).await?;
            }
        }
        
        Ok(())
        }).await
    }
}

#[async_trait]
impl SearchUseCase for SearchService {
    /**
     * Performs a search based on the specified criteria.
     * 
     * @param criteria Search criteria
     * @return Search results
     */
    async fn search(&self, criteria: SearchCriteriaDto) -> Result<SearchResultsDto> {
        // TODO: Get user ID from the authentication context
        let user_id = "default-user";
        let cache_key = self.create_cache_key(&criteria, user_id);
        
        // Try to get results from the cache
        if let Some(cached_results) = self.get_from_cache(&cache_key) {
            return Ok(cached_results);
        }
        
        // Initialize collections for results
        let mut found_files: Vec<FileDto> = Vec::new();
        let mut found_folders: Vec<FolderDto> = Vec::new();
        
        // Perform search in the specified folder or at the root
        self.search_recursive(
            criteria.folder_id.as_deref(),
            &criteria,
            &mut found_files,
            &mut found_folders,
        ).await?;
        
        // Apply pagination
        let total_count = found_files.len() + found_folders.len();
        
        // Sort by relevance or date according to criteria
        // By default, sort by modification date (most recent first)
        found_files.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
        found_folders.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
        
        // Apply limit and offset for pagination
        let start_idx = criteria.offset.min(total_count);
        let end_idx = (criteria.offset + criteria.limit).min(total_count);
        
        let paginated_items: Vec<(bool, usize)> = (start_idx..end_idx)
            .map(|i| {
                if i < found_folders.len() {
                    (true, i) // It's a folder
                } else {
                    (false, i - found_folders.len()) // It's a file
                }
            })
            .collect();
        
        // Extract paginated items
        let mut paginated_folders = Vec::new();
        let mut paginated_files = Vec::new();
        
        for (is_folder, idx) in paginated_items {
            if is_folder {
                if idx < found_folders.len() {
                    paginated_folders.push(found_folders[idx].clone());
                }
            } else {
                if idx < found_files.len() {
                    paginated_files.push(found_files[idx].clone());
                }
            }
        }
        
        // Create results object
        let search_results = SearchResultsDto::new(
            paginated_files,
            paginated_folders,
            criteria.limit,
            criteria.offset,
            Some(total_count),
        );
        
        // Store in cache
        self.store_in_cache(cache_key, search_results.clone());
        
        Ok(search_results)
    }
    
    /**
     * Clears the search results cache.
     * 
     * @return Result indicating success
     */
    async fn clear_search_cache(&self) -> Result<()> {
        if let Ok(mut cache) = self.search_cache.lock() {
            cache.clear();
        }
        Ok(())
    }
}

// Implement the test use case (stub)
impl SearchService {
    /// Creates a stub version of the service for testing
    pub fn new_stub() -> impl SearchUseCase {
        struct SearchServiceStub;
        
        #[async_trait]
        impl SearchUseCase for SearchServiceStub {
            async fn search(&self, _criteria: SearchCriteriaDto) -> Result<SearchResultsDto> {
                Ok(SearchResultsDto::empty())
            }
            
            async fn clear_search_cache(&self) -> Result<()> {
                Ok(())
            }
        }
        
        SearchServiceStub
    }
}