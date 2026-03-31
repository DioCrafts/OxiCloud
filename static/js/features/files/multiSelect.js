/**
 * OxiCloud - Multi-Select & Batch Actions Module
 *
 * Adds checkboxes to grid and list views, replaces the list-view header
 * with a NextCloud-style selection bar when items are selected, and
 * provides batch delete / move / download / favorites operations.
 */

// TODO: rename into selection-bar ? 
// TODO: merge with photo part

const multiSelect = {
    /** Currently selected items: Map<id, { id, name, type, parentId }> */
    _selected: new Map(),

    /** Last clicked index for Shift-range selection */
    _lastClickedIndex: -1,

    /** Whether the selection bar is currently visible */
    _barVisible: false,

    // ── Public API ──────────────────────────────────────────

    get count()        { return this._selected.size; },
    get items()        { return Array.from(this._selected.values()); },
    get hasSelection() { return this._selected.size > 0; },
    get files()        { return this.items.filter(i => i.type === 'file'); },
    get folders()      { return this.items.filter(i => i.type === 'folder'); },

    // ── Helpers for i18n ────────────────────────────────────

    _t(key, vars) {
        if (window.i18n && typeof window.i18n.t === 'function') {
            const val = window.i18n.t(key, vars);
            // If i18n returned the key itself, it's missing → fall back
            if (val && val !== key) return val;
        }
        return null;
    },

    // ── Selection state management ──────────────────────────

    toggle(id, name, type, parentId) {
        if (this._selected.has(id)) { this._selected.delete(id); return false; }
        this._selected.set(id, { id, name, type, parentId });
        return true;
    },

    select(id, name, type, parentId) {
        this._selected.set(id, { id, name, type, parentId });
    },

    deselect(id) { this._selected.delete(id); },

    clear() {
        this._selected.clear();
        this._lastClickedIndex = -1;
        document.querySelectorAll('.file-card.selected, .file-item.selected')
            .forEach(el => el.classList.remove('selected'));
        document.querySelectorAll('.item-checkbox').forEach(cb => cb.checked = false);
        this._syncUI();
    },

    selectAll() {
        this._selectAllInContainer('files-grid', '.file-card');
        this._selectAllInContainer('files-list-view', '.file-item');
        this._syncUI();
    },

    toggleAll() {
        const allItems = this._getAllVisibleItems();
        if (this._selected.size >= allItems.length && allItems.length > 0) {
            this.clear();
        } else {
            this.selectAll();
        }
    },

    // ── DOM helpers ─────────────────────────────────────────

    _selectElement(el) {
        const info = this._extractInfo(el);
        if (info) {
            this.select(info.id, info.name, info.type, info.parentId);
            el.classList.add('selected');
        }
    },

    _selectAllInContainer(containerId, selector) {
        const container = document.getElementById(containerId);
        if (!container) return;
        container.querySelectorAll(selector).forEach(el => this._selectElement(el));
    },

    _getAllVisibleItems() {
        return [...document.querySelectorAll('.file-item, .file-card')];
        /*
        const grid = document.getElementById('files-grid');
        if (grid && grid.style.display !== 'none') {
            return [...grid.querySelectorAll('.file-card')];
        }
        return [...document.querySelectorAll('#files-list-view .file-item')];
        */
    },

    _extractInfo(el) {
        if (el.dataset.folderId && el.dataset.folderName !== undefined) {
            return { id: el.dataset.folderId, name: el.dataset.folderName, type: 'folder', parentId: el.dataset.parentId || '' };
        }
        if (el.dataset.fileId) {
            return { id: el.dataset.fileId, name: el.dataset.fileName, type: 'file', parentId: el.dataset.folderId || '' };
        }
        return null;
    },

    // ── Click handler (shared by grid + list) ───────────────

    handleItemClick(el, event) {
        const items = this._getAllVisibleItems();
        const index = items.indexOf(el);
        const info = this._extractInfo(el);
        if (!info) return;

        const selectorOther = info.type === 'folder'
            ? `[data-folder-id="${info.id}"]`
            : `[data-file-id="${info.id}"]`;
        const otherEl = [...document.querySelectorAll(selectorOther)].find(e => e !== el);

        if (event && event.shiftKey && this._lastClickedIndex >= 0 && index >= 0) {
            const start = Math.min(this._lastClickedIndex, index);
            const end   = Math.max(this._lastClickedIndex, index);
            for (let i = start; i <= end; i++) {
                this._selectElement(items[i]);
                const iInfo = this._extractInfo(items[i]);
                if (iInfo) {
                    const sel = iInfo.type === 'folder'
                        ? `[data-folder-id="${iInfo.id}"]`
                        : `[data-file-id="${iInfo.id}"]`;
                    document.querySelectorAll(sel).forEach(e => e.classList.add('selected'));
                }
            }
        } else {
            const nowSelected = this.toggle(info.id, info.name, info.type, info.parentId);
            el.classList.toggle('selected', nowSelected);
            if (otherEl) otherEl.classList.toggle('selected', nowSelected);
        }
        this._lastClickedIndex = index;
        this._syncUI();
    },

    // ── Selection bar (replaces list-header when items selected) ────

    /**
     * Build the inner HTML for the selection bar that replaces the
     * normal list-header columns (Name / Type / Size / Modified).
     */
    _buildSelectionBarHTML() {
        //FIXME: should support i18n lang change

        return `
            <div x-class="batch-bar-left" class="list-header-checkbox">
                <button class="batch-bar-close" id="batch-grid-close" title="Cancel selection">
                     <i class="fas fa-times"></i>
                </button>
                <span class="batch-bar-count" id="batch-bar-count"></span>
            </div>
            <div class="batch-selection-info">
                <div class="batch-bar-actions">
                    <button class="batch-btn" id="batch-fav" title="Add to favorites" data-i18n-title="batch.add_favorites">
                        <i class="fas fa-star"></i>
                        <span data-i18n="batch.add_favorites">Add to favorites</span>
                    </button>
                    <button class="batch-btn" id="batch-move" title="Move or copy" data-i18n-title="batch.move_copy">
                        <i class="fas fa-arrows-alt"></i>
                        <span data-i18n="batch.move_copy">Move or copy</span>
                    </button>
                    <button class="batch-btn" id="batch-download" title="Download" data-i18n-title="actions.download">
                        <i class="fas fa-download"></i>
                        <span data-i18n="actions.download">Download</span>
                    </button>
                    <button class="batch-btn batch-btn-danger" id="batch-delete" title="Delete" data-i18-title="actions.delete">
                        <i class="fas fa-trash-alt"></i>
                        <span data-i18n="actions.delete">Delete</span>
                    </button>
                </div>
            </div>
        `;
    },

    /** Main UI sync — called after every selection change */
    _syncUI() {
        const n = this._selected.size;

        const batchSelectionBar = document.getElementById('batch-selection-bar');

        if (n > 0) {
            this._barVisible = true;

            const countText = n === 1
                ? (this._t('batch.one_selected') || '1 item selected')
                : (this._t('batch.n_selected', { count: n }) || `${n} items selected`);
            document.getElementById("batch-bar-count").innerText = countText;

            batchSelectionBar.classList.add('visible');
            
            
        } else {
            this._barVisible = false;

            // Hide grid bar
            batchSelectionBar.classList.remove('visible');            
        }

        // Sync individual item checkboxes
        this._syncItemCheckboxes();
        // Sync select-all checkbox state (for non-selection-mode)
        if (!this._barVisible) this._syncSelectAllCheckbox();
    },

    /** Wire click handlers on batch action buttons (idempotent per render) */
    _wireBarButtons() {
        const del      = document.getElementById('batch-delete');
        const move     = document.getElementById('batch-move');
        const dl       = document.getElementById('batch-download');
        const fav      = document.getElementById('batch-fav');
        const closeBtn = document.getElementById('batch-grid-close');
        if (del)  del.onclick          = () => this.batchDelete();
        if (move) move.onclick         = () => this.batchMove();
        if (dl)   dl.onclick           = () => this.batchDownload();
        if (fav)  fav.onclick          = () => this.batchFavorites();
        if (closeBtn) closeBtn.onclick = () => this.clear();
    },

    _syncItemCheckboxes() {
        document.querySelectorAll('.file-item').forEach(el => {
            const cb = el.querySelector('.item-checkbox');
            if (cb) cb.checked = el.classList.contains('selected');
        });
    },

    _syncSelectAllCheckbox() {
        const cb = document.getElementById('select-all-checkbox');
        if (!cb) return;
        const all = this._getAllVisibleItems();
        if (all.length === 0) {
            cb.checked = false;
            cb.indeterminate = false;
        } else if (this._selected.size >= all.length) {
            cb.checked = true;
            cb.indeterminate = false;
        } else if (this._selected.size > 0) {
            cb.checked = false;
            cb.indeterminate = true;
        } else {
            cb.checked = false;
            cb.indeterminate = false;
        }
    },

    // ── Batch operations ────────────────────────────────────

    /** Batch delete (move to trash) */
    async batchDelete() {
        const items = this.items;
        if (items.length === 0) return;

        const n = items.length;
        const msg = n === 1
            ? (this._t('dialogs.confirm_delete_file', { name: items[0].name })
                || `Are you sure you want to move "${items[0].name}" to trash?`)
            : (this._t('batch.confirm_delete', { count: n })
                || `Are you sure you want to move ${n} items to trash?`);

        const confirmed = await showConfirmDialog({
            title: this._t('dialogs.confirm_delete') || 'Move to trash',
            message: msg,
            confirmText: this._t('actions.delete') || 'Delete',
        });
        if (!confirmed) return;

        const fileIds   = items.filter(i => i.type === 'file').map(i => i.id);
        const folderIds = items.filter(i => i.type === 'folder').map(i => i.id);

        try {
            const response = await fetch('/api/batch/trash', {
                method: 'POST',
                headers: { ...getAuthHeaders(), 'Content-Type': 'application/json' },
                body: JSON.stringify({ file_ids: fileIds, folder_ids: folderIds })
            });
            const data = await response.json();
            const success = data.stats?.successful || 0;
            const errors  = data.stats?.failed || 0;

            this.clear();
            window.loadFiles();

            if (errors > 0) {
                window.ui.showNotification('Batch delete', `${success} moved to trash, ${errors} failed`);
            } else {
                window.ui.showNotification('Moved to trash', `${success} item${success !== 1 ? 's' : ''} moved to trash`);
            }
        } catch (e) {
            console.error('Batch trash error:', e);
            window.ui.showNotification('Error', 'Could not move items to trash');
            this.clear();
            window.loadFiles();
        }
    },

    /** Batch move — reuse existing move dialog */
    async batchMove() {
        const items = this.items;
        if (items.length === 0) return;

        window.app.moveDialogMode = 'batch';
        window.app.batchMoveItems = items;
        window.app.selectedTargetFolderId = "";

        const dialog = document.getElementById('move-file-dialog');
        const dialogHeader = dialog.querySelector('.rename-dialog-header');
        const n = items.length;
        const titleText = this._t('batch.move_title', { count: n })
            || `Move ${n} item${n !== 1 ? 's' : ''}`;
        dialogHeader.innerHTML = `<i class="fas fa-arrows-alt dialog-header-icon"></i> <span>${titleText}</span>`;

        const excludeIds = items.filter(i => i.type === 'folder').map(i => i.id);
        await contextMenus.loadAllFolders(excludeIds[0] || null, 'batch');
        dialog.style.display = 'flex';
    },

    /** Batch download — downloads all selected items as a single ZIP */
    async batchDownload() {
        const items = this.items;
        if (items.length === 0) return;

        window.ui.showNotification('Preparing download', 'Creating ZIP archive...');

        try {
            const fileIds   = items.filter(i => i.type === 'file').map(i => i.id);
            const folderIds = items.filter(i => i.type === 'folder').map(i => i.id);

            const response = await fetch('/api/batch/download', {
                method: 'POST',
                headers: { ...getAuthHeaders(), 'Content-Type': 'application/json' },
                body: JSON.stringify({ file_ids: fileIds, folder_ids: folderIds })
            });

            if (!response.ok) throw new Error(`Server returned ${response.status}`);

            const blob = await response.blob();
            const url  = URL.createObjectURL(blob);
            const link = document.createElement('a');
            link.href = url;
            link.download = `oxicloud-download-${Date.now()}.zip`;
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
            URL.revokeObjectURL(url);
        } catch (e) {
            console.error('Batch download error:', e);
            window.ui.showNotification('Error', 'Could not download selected items');
        }
    },

    /** Batch add to favorites — single API call */
    async batchFavorites() {
        const items = this.items;
        if (items.length === 0 || !window.favorites) return;

        // Filter out items already in favourites
        const toAdd = items.filter(i => !window.favorites.isFavorite(i.id, i.type));
        if (toAdd.length === 0) {
            this.clear();
            window.ui.showNotification(
                this._t('favorites.add') || 'Favorites',
                'All selected items are already favorites'
            );
            return;
        }

        try {
            const response = await fetch('/api/favorites/batch', {
                method: 'POST',
                headers: { ...getAuthHeaders(), 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    items: toAdd.map(i => ({ item_id: i.id, item_type: i.type }))
                })
            });

            if (!response.ok) throw new Error(`Server returned ${response.status}`);

            const data = await response.json();
            const inserted = data.stats?.inserted || 0;

            // Replace cache directly from response (no extra GET)
            if (data.favorites && window.favorites._replaceCacheFromResponse) {
                window.favorites._replaceCacheFromResponse(data.favorites);
            } else {
                await window.favorites._fetchFromServer();
            }

            this.clear();
            if (typeof window.loadFiles === 'function') window.loadFiles();

            if (inserted > 0) {
                window.ui.showNotification(
                    this._t('favorites.add') || 'Added to favorites',
                    `${inserted} item${inserted !== 1 ? 's' : ''} added to favorites`
                );
            } else {
                window.ui.showNotification(
                    this._t('favorites.add') || 'Favorites',
                    'All selected items are already favorites'
                );
            }
        } catch (e) {
            console.error('Batch favorites error:', e);
            window.ui.showNotification('Error', 'Could not add items to favorites');
        }
    },

    // ── Initialization ──────────────────────────────────────

    init() {
        // Wire the initial select-all checkbox
        this._injectListHeaderCheckbox();

        // Global deselect on empty-area click
        this._hookGlobalDeselect();

        // Keyboard shortcuts
        document.addEventListener('keydown', (e) => {
            if (e.target.closest('input, textarea, [contenteditable], .rename-dialog, .share-dialog, .confirm-dialog')) return;

            if ((e.ctrlKey || e.metaKey) && e.key === 'a') {
                const grid = document.getElementById('files-grid');
                if (grid && grid.closest('.files-container')) {
                    e.preventDefault();
                    this.selectAll();
                }
            }
            if (e.key === 'Escape' && this.hasSelection) this.clear();
            if (e.key === 'Delete' && this.hasSelection) this.batchDelete();
        });

        const batchSelectionBar = document.getElementById('batch-selection-bar');
        batchSelectionBar.innerHTML = this._buildSelectionBarHTML();
            
        if (window.i18n && window.i18n.translateElement) {
            window.i18n.translateElement(batchSelectionBar);
        }
        this._wireBarButtons();

    },

    _injectListHeaderCheckbox() {
        const cb = document.getElementById('select-all-checkbox');
        if (!cb) return;
        cb.addEventListener('change', () => this.toggleAll());
    },

    _hookGlobalDeselect() {
        document.addEventListener('click', (e) => {
            if (window.__rubberBandJustFinished) return;
            if (e.target.closest('.file-card, .file-item, .context-menu, .batch-selection-bar, .list-header.selection-mode, .about-modal, .rename-dialog, .share-dialog, .confirm-dialog, .modal-overlay, input, button')) return;
            if (this.hasSelection) this.clear();
        });
    }
};

// Expose globally
window.multiSelect = multiSelect;