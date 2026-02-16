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
                <div class="list-header favorites-header">
                    <div></div>
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

            for (const item of this._cache.values()) {
                if (item.item_type === 'folder') {
                    this._renderFolder(item, filesGrid, filesListView);
                } else {
                    this._renderFile(item, filesGrid, filesListView);
                }
            }

            window.ui.updateFileIcons();
        } catch (error) {
            console.error('Error displaying favorites:', error);
            if (window.ui && window.ui.showNotification) {
                window.ui.showNotification('Error', 'Error loading favorite items');
            }
        }
    },

    // ───────────────────── renderers ─────────────────────

    _renderFolder(item, filesGrid, filesListView) {
        const name = item.item_name || item.item_id || 'Unknown';
        const folderId = item.item_id;
        const parentId = item.parent_id || '';

        const modifiedAt = item.modified_at
            ? new Date(item.modified_at)
            : new Date(item.created_at);
        const formattedDate = modifiedAt.toLocaleDateString() + ' ' +
            modifiedAt.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });

        // --- grid element ---
        const gridEl = document.createElement('div');
        gridEl.className = 'file-card favorite-item';
        gridEl.dataset.folderId = folderId;
        gridEl.dataset.folderName = name;
        gridEl.dataset.parentId = parentId;
        gridEl.innerHTML = `
            <div class="favorite-indicator active"><i class="fas fa-star"></i></div>
            <div class="file-icon folder-icon"><i class="fas fa-folder"></i></div>
            <div class="file-name">${escapeHtml(name)}</div>
            <div class="file-info">Folder</div>
        `;
        gridEl.addEventListener('click', () => {
            window.app.currentPath = folderId;
            window.ui.updateBreadcrumb(name);
            window.loadFiles();
        });
        gridEl.addEventListener('contextmenu', (e) => {
            e.preventDefault();
            window.app.contextMenuTargetFolder = { id: folderId, name, parent_id: parentId };
            const cm = document.getElementById('folder-context-menu');
            cm.style.left = `${e.pageX}px`;
            cm.style.top = `${e.pageY}px`;
            cm.style.display = 'block';
        });
        filesGrid.appendChild(gridEl);

        // --- list element ---
        const listEl = document.createElement('div');
        listEl.className = 'file-item favorite-item';
        listEl.dataset.folderId = folderId;
        listEl.dataset.folderName = name;
        listEl.dataset.parentId = parentId;
        listEl.innerHTML = `
            <div class="favorite-indicator active"><i class="fas fa-star"></i></div>
            <div class="name-cell">
                <div class="file-icon folder-icon"><i class="fas fa-folder"></i></div>
                <span>${escapeHtml(name)}</span>
            </div>
            <div class="type-cell">${window.i18n ? window.i18n.t('files.file_types.folder') : 'Folder'}</div>
            <div class="size-cell">--</div>
            <div class="date-cell">${formattedDate}</div>
        `;
        listEl.addEventListener('click', () => {
            window.app.currentPath = folderId;
            window.ui.updateBreadcrumb(name);
            window.loadFiles();
        });
        listEl.addEventListener('contextmenu', (e) => {
            e.preventDefault();
            window.app.contextMenuTargetFolder = { id: folderId, name, parent_id: parentId };
            const cm = document.getElementById('folder-context-menu');
            cm.style.left = `${e.pageX}px`;
            cm.style.top = `${e.pageY}px`;
            cm.style.display = 'block';
        });
        filesListView.appendChild(listEl);
    },

    _renderFile(item, filesGrid, filesListView) {
        const name = item.item_name || item.item_id || 'Unknown';
        const fileId = item.item_id;
        const folderId = item.parent_id || '';
        const mimeType = item.item_mime_type || 'application/octet-stream';

        // Build a minimal file object for click handlers
        const fileObj = {
            id: fileId,
            name,
            folder_id: folderId,
            mime_type: mimeType,
            size: item.item_size || 0
        };

        // Use pre-computed display fields from the enriched API response
        const iconClass = item.icon_class || 'fas fa-file';
        const iconSpecialClass = item.icon_special_class || '';
        const typeLabel = item.category
            ? (window.i18n ? window.i18n.t(`files.file_types.${item.category.toLowerCase()}`) || item.category : item.category)
            : (window.i18n ? window.i18n.t('files.file_types.document') : 'Document');

        const fileSize = item.size_formatted || (window.formatFileSize ? window.formatFileSize(item.item_size || 0) : '0 B');
        const modifiedAt = item.modified_at
            ? new Date(item.modified_at)
            : new Date(item.created_at);
        const formattedDate = modifiedAt.toLocaleDateString() + ' ' +
            modifiedAt.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });

        // --- grid element ---
        const gridEl = document.createElement('div');
        gridEl.className = 'file-card favorite-item';
        gridEl.dataset.fileId = fileId;
        gridEl.dataset.fileName = name;
        gridEl.dataset.folderId = folderId;
        gridEl.innerHTML = `
            <div class="favorite-indicator active"><i class="fas fa-star"></i></div>
            <div class="file-icon ${iconSpecialClass}"><i class="${iconClass}"></i></div>
            <div class="file-name">${escapeHtml(name)}</div>
            <div class="file-info">Modified ${formattedDate.split(' ')[0]}</div>
        `;
        gridEl.addEventListener('click', () => {
            if (window.ui && window.ui.isViewableFile(fileObj) && window.inlineViewer) {
                window.inlineViewer.openFile(fileObj);
            } else if (window.fileOps) {
                window.fileOps.downloadFile(fileId, name);
            }
        });
        gridEl.addEventListener('contextmenu', (e) => {
            e.preventDefault();
            window.app.contextMenuTargetFile = { id: fileId, name, folder_id: folderId };
            const cm = document.getElementById('file-context-menu');
            cm.style.left = `${e.pageX}px`;
            cm.style.top = `${e.pageY}px`;
            cm.style.display = 'block';
        });
        filesGrid.appendChild(gridEl);

        // --- list element ---
        const listEl = document.createElement('div');
        listEl.className = 'file-item favorite-item';
        listEl.dataset.fileId = fileId;
        listEl.dataset.fileName = name;
        listEl.dataset.folderId = folderId;
        listEl.innerHTML = `
            <div class="favorite-indicator active"><i class="fas fa-star"></i></div>
            <div class="name-cell">
                <div class="file-icon ${iconSpecialClass}"><i class="${iconClass}"></i></div>
                <span>${escapeHtml(name)}</span>
            </div>
            <div class="type-cell">${escapeHtml(typeLabel)}</div>
            <div class="size-cell">${fileSize}</div>
            <div class="date-cell">${formattedDate}</div>
        `;
        listEl.addEventListener('click', () => {
            if (window.ui && window.ui.isViewableFile(fileObj) && window.inlineViewer) {
                window.inlineViewer.openFile(fileObj);
            } else if (window.fileOps) {
                window.fileOps.downloadFile(fileId, name);
            }
        });
        listEl.addEventListener('contextmenu', (e) => {
            e.preventDefault();
            window.app.contextMenuTargetFile = { id: fileId, name, folder_id: folderId };
            const cm = document.getElementById('file-context-menu');
            cm.style.left = `${e.pageX}px`;
            cm.style.top = `${e.pageY}px`;
            cm.style.display = 'block';
        });
        filesListView.appendChild(listEl);
    }
};

// Expose globally
window.favorites = favorites;
