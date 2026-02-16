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
                <div class="list-header recent-header">
                    <div></div>
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

            for (const item of recentItems) {
                this._renderRecentItem(item, filesGrid, filesListView);
            }

            window.ui.updateFileIcons();
        } catch (error) {
            console.error('Error displaying recent files:', error);
            if (window.ui && window.ui.showNotification) {
                window.ui.showNotification('Error', 'Error loading recent files');
            }
        }
    },

    // ───────────────────── renderer ─────────────────────

    _renderRecentItem(item, filesGrid, filesListView) {
        const name = item.item_name || item.item_id || 'Unknown';
        const itemId = item.item_id;
        const folderId = item.parent_id || '';
        const mimeType = item.item_mime_type || 'application/octet-stream';
        const isFolder = item.item_type === 'folder';

        // Build a minimal file object for click handlers
        const fileObj = {
            id: itemId,
            name,
            folder_id: folderId,
            mime_type: mimeType,
            size: item.item_size || 0
        };

        // Use pre-computed display fields from the enriched API response
        const iconClass = item.icon_class || (isFolder ? 'fas fa-folder' : 'fas fa-file');
        const iconSpecialClass = item.icon_special_class || (isFolder ? 'folder-icon' : '');
        const typeLabel = item.category
            ? (window.i18n ? window.i18n.t(`files.file_types.${item.category.toLowerCase()}`) || item.category : item.category)
            : (isFolder
                ? (window.i18n ? window.i18n.t('files.file_types.folder') : 'Folder')
                : (window.i18n ? window.i18n.t('files.file_types.document') : 'Document'));

        const fileSize = isFolder ? '--' : (item.size_formatted || (window.formatFileSize ? window.formatFileSize(item.item_size || 0) : '0 B'));
        const formattedDate = window.formatDateTime(item.accessed_at);

        // Click handler
        const onClick = () => {
            if (isFolder) {
                window.app.currentPath = itemId;
                window.ui.updateBreadcrumb(name);
                window.loadFiles();
            } else {
                // Re-record access
                document.dispatchEvent(new CustomEvent('file-accessed', { detail: { file: fileObj } }));
                if (window.ui && window.ui.isViewableFile(fileObj) && window.inlineViewer) {
                    window.inlineViewer.openFile(fileObj);
                } else if (window.fileOps) {
                    window.fileOps.downloadFile(itemId, name);
                }
            }
        };

        // Context menu handler
        const onContextMenu = (e) => {
            e.preventDefault();
            if (isFolder) {
                window.app.contextMenuTargetFolder = { id: itemId, name, parent_id: folderId };
                const cm = document.getElementById('folder-context-menu');
                cm.style.left = `${e.pageX}px`;
                cm.style.top = `${e.pageY}px`;
                cm.style.display = 'block';
            } else {
                window.app.contextMenuTargetFile = { id: itemId, name, folder_id: folderId };
                const cm = document.getElementById('file-context-menu');
                cm.style.left = `${e.pageX}px`;
                cm.style.top = `${e.pageY}px`;
                cm.style.display = 'block';
            }
        };

        // --- grid element ---
        const gridEl = document.createElement('div');
        gridEl.className = `file-card recent-item`;
        if (isFolder) {
            gridEl.dataset.folderId = itemId;
            gridEl.dataset.folderName = name;
        } else {
            gridEl.dataset.fileId = itemId;
            gridEl.dataset.fileName = name;
            gridEl.dataset.folderId = folderId;
        }
        gridEl.innerHTML = `
            <div class="recent-indicator"><i class="fas fa-clock"></i></div>
            <div class="file-icon ${iconSpecialClass}"><i class="${iconClass}"></i></div>
            <div class="file-name">${escapeHtml(name)}</div>
            <div class="file-info">Accessed ${formattedDate.split(' ')[0]}</div>
        `;
        gridEl.addEventListener('click', onClick);
        gridEl.addEventListener('contextmenu', onContextMenu);
        filesGrid.appendChild(gridEl);

        // --- list element ---
        const listEl = document.createElement('div');
        listEl.className = `file-item recent-item`;
        if (isFolder) {
            listEl.dataset.folderId = itemId;
            listEl.dataset.folderName = name;
        } else {
            listEl.dataset.fileId = itemId;
            listEl.dataset.fileName = name;
            listEl.dataset.folderId = folderId;
        }
        listEl.innerHTML = `
            <div class="recent-indicator"><i class="fas fa-clock"></i></div>
            <div class="name-cell">
                <div class="file-icon ${iconSpecialClass}"><i class="${iconClass}"></i></div>
                <span>${escapeHtml(name)}</span>
            </div>
            <div class="type-cell">${escapeHtml(typeLabel)}</div>
            <div class="size-cell">${fileSize}</div>
            <div class="date-cell">${formattedDate}</div>
        `;
        listEl.addEventListener('click', onClick);
        listEl.addEventListener('contextmenu', onContextMenu);
        filesListView.appendChild(listEl);
    }
};

// Expose globally
window.recent = recent;
