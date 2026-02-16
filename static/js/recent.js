/**
 * OxiCloud - Recent Files Module (server-authoritative)
 *
 * Source of truth: GET /api/recent (enriched with name/size/mime via SQL JOIN).
 * File-access events are forwarded to the backend with POST /api/recent/{type}/{id}.
 * No localStorage usage — the server persists and prunes recent items.
 */

const recent = {
    /** Maximum items to request from the server */
    MAX_RECENT_FILES: 20,

    // ───────────────────── helpers ─────────────────────

    _authHeaders() {
        const token = localStorage.getItem('oxicloud_token');
        const h = {};
        if (token) h['Authorization'] = `Bearer ${token}`;
        return h;
    },

    // ───────────────────── lifecycle ─────────────────────

    /**
     * Initialise the module.  Called once from app.js on startup.
     */
    init() {
        console.log('Initializing recent files module (server-authoritative)');
        this.setupEventListeners();
    },

    /**
     * Listen for file-accessed events dispatched by ui.js and forward
     * them to the backend.
     */
    setupEventListeners() {
        document.addEventListener('file-accessed', (event) => {
            if (event.detail && event.detail.file) {
                const file = event.detail.file;
                const itemType = file.item_type || 'file';
                this._recordAccess(file.id, itemType);
            }
        });
    },

    /**
     * Record an access event on the server.
     */
    async _recordAccess(itemId, itemType) {
        try {
            await fetch(`/api/recent/${itemType}/${itemId}`, {
                method: 'POST',
                headers: this._authHeaders()
            });
        } catch (err) {
            console.warn('Failed to record recent access:', err);
        }
    },

    // ───────────────────── public API ─────────────────────

    /**
     * Clear all recent items (delegates to the server).
     */
    async clearRecentFiles() {
        try {
            await fetch('/api/recent/clear', {
                method: 'DELETE',
                headers: this._authHeaders()
            });
        } catch (err) {
            console.error('Error clearing recent files:', err);
        }
    },

    /**
     * Fetch and display recent files.  Data comes directly from the
     * enriched backend response — zero extra per-item fetches.
     */
    async displayRecentFiles() {
        try {
            const response = await fetch(`/api/recent?limit=${this.MAX_RECENT_FILES}`, {
                headers: this._authHeaders()
            });

            if (!response.ok) {
                throw new Error(`Server returned ${response.status}`);
            }

            const recentItems = await response.json();

            const filesGrid = document.getElementById('files-grid');
            const filesListView = document.getElementById('files-list-view');

            filesGrid.innerHTML = '';
            filesListView.innerHTML = `
                <div class="list-header">
                    <div class="list-header-checkbox"><input type="checkbox" id="select-all-checkbox" title="Select all"></div>
                    <div data-i18n="files.name">Name</div>
                    <div data-i18n="files.type">Type</div>
                    <div data-i18n="files.size">Size</div>
                    <div data-i18n="recent.accessed">Accessed</div>
                </div>
            `;

            window.ui.updateBreadcrumb('');

            if (recentItems.length === 0) {
                const emptyState = document.createElement('div');
                emptyState.className = 'empty-state';
                emptyState.innerHTML = `
                    <i class="fas fa-clock" style="font-size: 48px; color: #ddd; margin-bottom: 16px;"></i>
                    <p>${window.i18n ? window.i18n.t('recent.empty_state') : 'No recent files'}</p>
                    <p>${window.i18n ? window.i18n.t('recent.empty_hint') : 'Files you open will appear here'}</p>
                `;
                filesGrid.appendChild(emptyState);
                return;
            }

            const folders = [];
            const files = [];
            for (const item of recentItems) {
                const isFolder = item.item_type === 'folder';
                if (isFolder) {
                    folders.push({
                        id: item.item_id,
                        name: item.item_name || item.item_id,
                        parent_id: item.parent_id || '',
                        modified_at: item.accessed_at
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
                        modified_at: item.accessed_at
                    });
                }
            }
            if (folders.length) window.ui.renderFolders(folders);
            if (files.length) window.ui.renderFiles(files);
        } catch (error) {
            console.error('Error displaying recent files:', error);
            if (window.ui && window.ui.showNotification) {
                window.ui.showNotification('Error', 'Error loading recent files');
            }
        }
    }
};

// Expose globally
window.recent = recent;
