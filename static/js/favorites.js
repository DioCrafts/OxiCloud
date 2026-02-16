/**
 * OxiCloud - Favorites Module (server-authoritative)
 *
 * Source of truth: GET /api/favorites (enriched with name/size/mime via SQL JOIN).
 * Local in-memory cache (`_cache`) keeps `isFavorite()` synchronous for the
 * rendering path so star icons can be painted without a round-trip.
 */

const favorites = {
    /** @type {Map<string, object>} key = "file:<id>" | "folder:<id>" */
    _cache: new Map(),

    /** Whether the initial fetch from the server has completed */
    _ready: false,

    // ───────────────────── helpers ─────────────────────

    _authHeaders() {
        const token = localStorage.getItem('oxicloud_token');
        const h = {};
        if (token) h['Authorization'] = `Bearer ${token}`;
        return h;
    },

    _cacheKey(id, type) {
        return `${type}:${id}`;
    },

    /**
     * Replace the entire in-memory cache from an array of FavoriteItemDto
     * objects (as returned by the batch endpoint). Avoids an extra
     * GET /api/favorites round-trip.
     */
    _replaceCacheFromResponse(items) {
        this._cache.clear();
        for (const item of items) {
            this._cache.set(this._cacheKey(item.item_id, item.item_type), item);
        }
        this._ready = true;
        console.log(`Favorites cache replaced from response: ${this._cache.size} items`);
    },

    // ───────────────────── lifecycle ─────────────────────

    /**
     * Initialise the module: fetch the full list from the server and populate
     * the in-memory cache.  Called once from app.js on startup.
     */
    async init() {
        console.log('Initializing favorites module (server-authoritative)');
        await this._fetchFromServer();
    },

    /**
     * Fetch favourites from the backend and rebuild the cache.
     */
    async _fetchFromServer() {
        try {
            const response = await fetch('/api/favorites', {
                headers: this._authHeaders()
            });

            if (!response.ok) {
                console.warn(`Favorites API returned ${response.status}`);
                this._ready = true;
                return;
            }

            const items = await response.json();
            this._cache.clear();
            for (const item of items) {
                this._cache.set(this._cacheKey(item.item_id, item.item_type), item);
            }

            this._ready = true;
            console.log(`Favorites cache loaded: ${this._cache.size} items`);
        } catch (err) {
            console.error('Error fetching favorites:', err);
            this._ready = true;
        }
    },

    // ───────────────────── public API ─────────────────────

    /**
     * Synchronous check used by ui.js to paint star icons.
     */
    isFavorite(id, type) {
        return this._cache.has(this._cacheKey(id, type));
    },

    /**
     * Add an item to favourites (server-first).
     */
    async addToFavorites(id, name, type, _parentId) {
        try {
            const response = await fetch(`/api/favorites/${type}/${id}`, {
                method: 'POST',
                headers: this._authHeaders()
            });

            if (!response.ok) {
                throw new Error(`Server returned ${response.status}`);
            }

            // Refresh cache from server to get enriched data
            await this._fetchFromServer();

            // Notify user
            if (window.ui && window.ui.showNotification) {
                window.ui.showNotification(
                    window.i18n ? window.i18n.t('favorites.added_title') : 'Added to favorites',
                    `"${name}" ${window.i18n ? window.i18n.t('favorites.added_msg') : 'added to favorites'}`
                );
            }

            // Refresh view to update star icons
            if (window.app && window.app.currentSection === 'files' && typeof window.loadFiles === 'function') {
                window.loadFiles();
            }

            return true;
        } catch (error) {
            console.error('Error adding to favorites:', error);
            return false;
        }
    },

    /**
     * Remove an item from favourites (server-first).
     */
    async removeFromFavorites(id, type) {
        try {
            // Remember name for notification before removing from cache
            const cached = this._cache.get(this._cacheKey(id, type));
            const itemName = cached?.item_name || id;

            const response = await fetch(`/api/favorites/${type}/${id}`, {
                method: 'DELETE',
                headers: this._authHeaders()
            });

            if (!response.ok) {
                throw new Error(`Server returned ${response.status}`);
            }

            // Remove from local cache
            this._cache.delete(this._cacheKey(id, type));

            if (window.ui && window.ui.showNotification) {
                window.ui.showNotification(
                    window.i18n ? window.i18n.t('favorites.removed_title') : 'Removed from favorites',
                    `"${itemName}" ${window.i18n ? window.i18n.t('favorites.removed_msg') : 'removed from favorites'}`
                );
            }

            // Refresh view to update star icons
            if (window.app && window.app.currentSection === 'files' && typeof window.loadFiles === 'function') {
                window.loadFiles();
            }

            return true;
        } catch (error) {
            console.error('Error removing from favorites:', error);
            return false;
        }
    },

    // ───────────────────── display ─────────────────────

    /**
     * Render the favourites view.  All data comes from the in-memory cache
     * (which was populated from the enriched backend response — zero extra
     * fetches).
     */
    async displayFavorites() {
        try {
            // Ensure cache is fresh
            if (!this._ready) {
                await this._fetchFromServer();
            }

            const filesGrid = document.getElementById('files-grid');
            const filesListView = document.getElementById('files-list-view');

            filesGrid.innerHTML = '';
            filesListView.innerHTML = `
                <div class="list-header">
                    <div class="list-header-checkbox"><input type="checkbox" id="select-all-checkbox" title="Select all"></div>
                    <div data-i18n="files.name">Name</div>
                    <div data-i18n="files.type">Type</div>
                    <div data-i18n="files.size">Size</div>
                    <div data-i18n="files.modified">Modified</div>
                </div>
            `;

            window.ui.updateBreadcrumb('');

            if (this._cache.size === 0) {
                const emptyState = document.createElement('div');
                emptyState.className = 'empty-state';
                emptyState.innerHTML = `
                    <i class="fas fa-star" style="font-size: 48px; color: #ddd; margin-bottom: 16px;"></i>
                    <p>${window.i18n ? window.i18n.t('favorites.empty_state') : 'No favorite items'}</p>
                    <p>${window.i18n ? window.i18n.t('favorites.empty_hint') : 'To mark as favorite, right-click on any file or folder'}</p>
                `;
                filesGrid.appendChild(emptyState);
                return;
            }

            const folders = [];
            const files = [];
            for (const item of this._cache.values()) {
                if (item.item_type === 'folder') {
                    folders.push({
                        id: item.item_id,
                        name: item.item_name || item.item_id,
                        parent_id: item.parent_id || '',
                        modified_at: item.modified_at || item.created_at
                    });
                } else {
                    files.push({
                        id: item.item_id,
                        name: item.item_name || item.item_id,
                        folder_id: item.parent_id || '',
                        mime_type: item.item_mime_type,
                        icon_class: item.icon_class,
                        icon_special_class: item.icon_special_class,
                        category: item.category,
                        size: item.item_size || 0,
                        size_formatted: item.size_formatted,
                        modified_at: item.modified_at || item.created_at
                    });
                }
            }
            if (folders.length) window.ui.renderFolders(folders);
            if (files.length) window.ui.renderFiles(files);
        } catch (error) {
            console.error('Error displaying favorites:', error);
            if (window.ui && window.ui.showNotification) {
                window.ui.showNotification('Error', 'Error loading favorite items');
            }
        }
    }
};

// Expose globally
window.favorites = favorites;
