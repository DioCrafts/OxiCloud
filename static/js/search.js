/**
 * OxiCloud - Search Module
 * This file handles search functionality for files and folders
 */

const search = {
    /**
     * Perform a basic search using query string
     * @param {string} query - Search query
     * @param {Object} options - Additional search options
     * @returns {Promise<Object>} - Search results
     */
    async searchFiles(query, options = {}) {
        try {
            // Prepare search parameters
            const params = new URLSearchParams();
            params.append('query', query);
            
            // Add optional parameters
            if (options.folder_id) params.append('folder_id', options.folder_id);
            if (options.recursive !== undefined) params.append('recursive', options.recursive);
            if (options.file_types) params.append('type', options.file_types);
            if (options.min_size) params.append('min_size', options.min_size);
            if (options.max_size) params.append('max_size', options.max_size);
            if (options.created_after) params.append('created_after', options.created_after);
            if (options.created_before) params.append('created_before', options.created_before);
            if (options.modified_after) params.append('modified_after', options.modified_after);
            if (options.modified_before) params.append('modified_before', options.modified_before);
            if (options.limit) params.append('limit', options.limit);
            if (options.offset) params.append('offset', options.offset);

            // Create search URL
            const url = `/api/search?${params.toString()}`;
            console.log(`Performing search with URL: ${url}`);
            
            // Perform the search request
            const response = await fetch(url);
            
            if (response.ok) {
                return await response.json();
            } else {
                let errorText = '';
                try {
                    const errorJson = await response.json();
                    errorText = errorJson.error || response.statusText;
                } catch (e) {
                    errorText = response.statusText;
                }
                
                console.error(`Search error: ${errorText}`);
                throw new Error(`Search failed: ${errorText}`);
            }
        } catch (error) {
            console.error('Error performing search:', error);
            window.ui.showNotification('Error', 'Error performing search');
            return { files: [], folders: [], total_count: 0 };
        }
    },

    /**
     * Perform advanced search with multiple criteria
     * @param {Object} criteria - Search criteria
     * @returns {Promise<Object>} - Search results
     */
    async advancedSearch(criteria) {
        try {
            console.log('Performing advanced search with criteria:', criteria);
            
            // Use POST endpoint for advanced search
            const response = await fetch('/api/search', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(criteria)
            });
            
            if (response.ok) {
                return await response.json();
            } else {
                let errorText = '';
                try {
                    const errorJson = await response.json();
                    errorText = errorJson.error || response.statusText;
                } catch (e) {
                    errorText = response.statusText;
                }
                
                console.error(`Advanced search error: ${errorText}`);
                throw new Error(`Advanced search failed: ${errorText}`);
            }
        } catch (error) {
            console.error('Error performing advanced search:', error);
            window.ui.showNotification('Error', 'Error performing advanced search');
            return { files: [], folders: [], total_count: 0 };
        }
    },

    /**
     * Display search results in the UI
     * @param {Object} results - Search results object with files and folders arrays
     */
    displaySearchResults(results) {
        // Get the files grid and list view elements
        const filesGrid = document.getElementById('files-grid');
        const filesListView = document.getElementById('files-list-view');
        
        // Clear existing content
        filesGrid.innerHTML = '';
        filesListView.innerHTML = `
            <div class="list-header">
                <div data-i18n="files.name">Name</div>
                <div data-i18n="files.type">Type</div>
                <div data-i18n="files.size">Size</div>
                <div data-i18n="files.modified">Modified</div>
            </div>
        `;
        
        // Add search results header
        const searchHeader = document.createElement('div');
        searchHeader.className = 'search-results-header';
        searchHeader.innerHTML = `
            <h3>Search results (${results.total_count || (results.files.length + results.folders.length)})</h3>
            <button class="btn btn-secondary" id="clear-search-btn">
                <i class="fas fa-times"></i> Clear search
            </button>
        `;
        filesGrid.appendChild(searchHeader);
        
        // Add event listener to clear search button
        const clearSearchBtn = document.getElementById('clear-search-btn');
        if (clearSearchBtn) {
            clearSearchBtn.addEventListener('click', () => {
                // Clear search input
                document.querySelector('.search-container input').value = '';
                
                // Load regular files view
                window.app.currentPath = '';
                window.ui.updateBreadcrumb('');
                window.loadFiles();
            });
        }
        
        // If no results, show empty state
        if (results.files.length === 0 && results.folders.length === 0) {
            const emptyState = document.createElement('div');
            emptyState.className = 'empty-state';
            emptyState.innerHTML = `
                <i class="fas fa-search" style="font-size: 48px; color: #ddd; margin-bottom: 16px;"></i>
                <p>No results found for this search</p>
            `;
            filesGrid.appendChild(emptyState);
            return;
        }
        
        // Process folders
        results.folders.forEach(folder => {
            window.ui.addFolderToView(folder);
        });
        
        // Process files
        results.files.forEach(file => {
            window.ui.addFileToView(file);
        });
        
        // Update file icons
        window.ui.updateFileIcons();
    },
    
    /**
     * Clear the search cache on the server
     * @returns {Promise<boolean>} - Success status
     */
    async clearSearchCache() {
        try {
            const response = await fetch('/api/search/cache', {
                method: 'DELETE'
            });
            
            if (response.ok) {
                window.ui.showNotification('Cache cleared', 'Search cache cleared successfully');
                return true;
            } else {
                window.ui.showNotification('Error', 'Error clearing search cache');
                return false;
            }
        } catch (error) {
            console.error('Error clearing search cache:', error);
            window.ui.showNotification('Error', 'Error clearing search cache');
            return false;
        }
    }
};

// Expose the search module globally
window.search = search;