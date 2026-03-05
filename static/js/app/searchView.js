/**
 * Search view orchestration logic
 */

async function performSearch(query, sortBy) {
    const app = window.app;

    console.log(`Performing search for: "${query}" (sort: ${sortBy || 'relevance'})`);

    try {
        app.isSearchMode = true;
        window.ui.updateBreadcrumb(`Search: "${query}"`);

        const filesGrid = document.getElementById('files-grid');
        if (filesGrid) {
            filesGrid.innerHTML = `
                <div class="search-results-header">
                    <h3><i class="fas fa-spinner fa-spin" style="margin-right:8px;"></i> Searching for "${query}"...</h3>
                </div>
            `;
        }

        const options = {
            recursive: true,
            limit: 100,
            sort_by: sortBy || 'relevance'
        };

        if (!app.isTrashView) {
            options.folder_id = app.currentPath;

            if (!options.folder_id || options.folder_id === '') {
                await window.resolveHomeFolder();
                options.folder_id = app.currentPath;
            }
        }

        const searchResults = await window.search.searchFiles(query, options);
        window.search.displaySearchResults(searchResults);

    } catch (error) {
        console.error('Search error:', error);
        window.ui.showNotification('Error', 'Error performing search');
    }
}

document.addEventListener('search-resort', (e) => {
    const searchInput = document.querySelector('.search-container input');
    if (searchInput && searchInput.value.trim()) {
        performSearch(searchInput.value.trim(), e.detail.sort_by);
    }
});

window.performSearch = performSearch;
