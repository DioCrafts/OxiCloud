/**
 * OxiCloud - Multi-Select & Batch Actions Module
 * Adds checkboxes to both grid and list views, a batch action bar,
 * and batch delete / move / download operations.
 */

const multiSelect = {
    /** Currently selected items: { id, name, type: 'file'|'folder', parentId } */
    _selected: new Map(),

    /** Last clicked index for Shift-range selection */
    _lastClickedIndex: -1,

    /** Whether the batch bar is currently visible */
    _barVisible: false,

    // ── Public API ──────────────────────────────────────────

    /** Number of selected items */
    get count() { return this._selected.size; },

    /** All selected items as an array */
    get items() { return Array.from(this._selected.values()); },

    /** True when at least one item is selected */
    get hasSelection() { return this._selected.size > 0; },

    /** Get selected files only */
    get files() { return this.items.filter(i => i.type === 'file'); },

    /** Get selected folders only */
    get folders() { return this.items.filter(i => i.type === 'folder'); },

    // ── Selection state management ──────────────────────────

    /**
     * Toggle an item in the selection.
     * @param {string} id
     * @param {string} name
     * @param {'file'|'folder'} type
     * @param {string} parentId  parent / folder id
     * @returns {boolean} new selected state
     */
    toggle(id, name, type, parentId) {
        if (this._selected.has(id)) {
            this._selected.delete(id);
            return false;
        }
        this._selected.set(id, { id, name, type, parentId });
        return true;
    },

    /** Select a single item (add if not present) */
    select(id, name, type, parentId) {
        this._selected.set(id, { id, name, type, parentId });
    },

    /** Deselect a single item */
    deselect(id) {
        this._selected.delete(id);
    },

    /** Clear the whole selection */
    clear() {
        this._selected.clear();
        this._lastClickedIndex = -1;
        // Remove visual state from DOM
        document.querySelectorAll('.file-card.selected, .file-item.selected').forEach(el => {
            el.classList.remove('selected');
        });
        // Uncheck all item checkboxes
        document.querySelectorAll('.item-checkbox').forEach(cb => cb.checked = false);
        this._syncUI();
    },

    /** Select all visible items */
    selectAll() {
        this._selectAllInContainer('files-grid', '.file-card');
        this._selectAllInContainer('files-list-view', '.file-item');
        this._syncUI();
    },

    /** Deselect/select all toggle */
    toggleAll() {
        const allItems = this._getAllVisibleItems();
        if (this._selected.size === allItems.length && allItems.length > 0) {
            this.clear();
        } else {
            this.selectAll();
        }
    },

    // ── DOM helpers ─────────────────────────────────────────

    /** Gather info from a DOM element and add to selection */
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
        const gridItems = [...document.querySelectorAll('#files-grid .file-card')];
        const listItems = [...document.querySelectorAll('#files-list-view .file-item')];
        // Only return items from the currently visible view
        const grid = document.getElementById('files-grid');
        if (grid && grid.style.display !== 'none') return gridItems;
        return listItems;
    },

    /** Extract item info from a DOM element */
    _extractInfo(el) {
        if (el.dataset.folderId && el.dataset.folderName !== undefined) {
            return {
                id: el.dataset.folderId,
                name: el.dataset.folderName,
                type: 'folder',
                parentId: el.dataset.parentId || ''
            };
        }
        if (el.dataset.fileId) {
            return {
                id: el.dataset.fileId,
                name: el.dataset.fileName,
                type: 'file',
                parentId: el.dataset.folderId || ''
            };
        }
        return null;
    },

    // ── Click handler (shared by grid + list) ───────────────

    /**
     * Handle a checkbox/selection click on an item element.
     * Supports Shift-click for range selection.
     */
    handleItemClick(el, event) {
        const items = this._getAllVisibleItems();
        const index = items.indexOf(el);

        // Also find the matching element in the other view
        const info = this._extractInfo(el);
        if (!info) return;

        const selectorOther = info.type === 'folder'
            ? `[data-folder-id="${info.id}"]`
            : `[data-file-id="${info.id}"]`;
        const otherEl = [...document.querySelectorAll(selectorOther)]
            .find(e => e !== el);

        if (event && event.shiftKey && this._lastClickedIndex >= 0 && index >= 0) {
            // Range selection
            const start = Math.min(this._lastClickedIndex, index);
            const end   = Math.max(this._lastClickedIndex, index);
            for (let i = start; i <= end; i++) {
                this._selectElement(items[i]);
                // Mirror to other view
                const iInfo = this._extractInfo(items[i]);
                if (iInfo) {
                    const sel = iInfo.type === 'folder'
                        ? `[data-folder-id="${iInfo.id}"]`
                        : `[data-file-id="${iInfo.id}"]`;
                    document.querySelectorAll(sel).forEach(e => e.classList.add('selected'));
                }
            }
        } else {
            // Normal toggle
            const nowSelected = this.toggle(info.id, info.name, info.type, info.parentId);
            el.classList.toggle('selected', nowSelected);
            if (otherEl) otherEl.classList.toggle('selected', nowSelected);
        }

        this._lastClickedIndex = index;
        this._syncUI();
    },

    // ── Batch action bar ────────────────────────────────────

    /** Create the batch action bar if it doesn't exist */
    _ensureBar() {
        if (document.getElementById('batch-action-bar')) return;

        const bar = document.createElement('div');
        bar.id = 'batch-action-bar';
        bar.className = 'batch-action-bar';
        bar.innerHTML = `
            <div class="batch-bar-left">
                <button class="batch-bar-close" id="batch-bar-close" title="Cancel selection">
                    <i class="fas fa-times"></i>
                </button>
                <span class="batch-bar-count" id="batch-bar-count">0 selected</span>
            </div>
            <div class="batch-bar-actions">
                <button class="batch-btn" id="batch-download" title="Download">
                    <i class="fas fa-download"></i>
                    <span data-i18n="actions.download">Download</span>
                </button>
                <button class="batch-btn" id="batch-move" title="Move">
                    <i class="fas fa-arrows-alt"></i>
                    <span data-i18n="actions.move">Move</span>
                </button>
                <button class="batch-btn batch-btn-danger" id="batch-delete" title="Delete">
                    <i class="fas fa-trash-alt"></i>
                    <span data-i18n="actions.delete">Delete</span>
                </button>
            </div>
        `;

        // Insert before the files-container (inside main-content)
        const filesContainer = document.querySelector('.files-container');
        if (filesContainer && filesContainer.parentNode) {
            filesContainer.parentNode.insertBefore(bar, filesContainer);
        } else {
            document.body.appendChild(bar);
        }

        // Wire up events
        document.getElementById('batch-bar-close').addEventListener('click', () => this.clear());
        document.getElementById('batch-delete').addEventListener('click', () => this.batchDelete());
        document.getElementById('batch-move').addEventListener('click', () => this.batchMove());
        document.getElementById('batch-download').addEventListener('click', () => this.batchDownload());
    },

    /** Show/hide the bar and update the count */
    _syncUI() {
        this._ensureBar();
        const bar = document.getElementById('batch-action-bar');
        const count = document.getElementById('batch-bar-count');

        if (this._selected.size > 0) {
            bar.classList.add('visible');
            this._barVisible = true;
            const n = this._selected.size;
            const itemsText = n === 1
                ? (window.i18n ? window.i18n.t('batch.one_selected') : '1 item selected')
                : (window.i18n ? window.i18n.t('batch.n_selected', { count: n }) : `${n} items selected`);
            count.textContent = itemsText;
        } else {
            bar.classList.remove('visible');
            this._barVisible = false;
        }

        // Update select-all checkbox state
        this._syncSelectAllCheckbox();

        // Sync individual list-view checkboxes
        this._syncItemCheckboxes();
    },

    /** Sync individual item checkboxes with selection state */
    _syncItemCheckboxes() {
        document.querySelectorAll('.file-item').forEach(el => {
            const cb = el.querySelector('.item-checkbox');
            if (cb) {
                cb.checked = el.classList.contains('selected');
            }
        });
    },

    _syncSelectAllCheckbox() {
        const cb = document.getElementById('select-all-checkbox');
        if (!cb) return;
        const all = this._getAllVisibleItems();
        if (all.length === 0) {
            cb.checked = false;
            cb.indeterminate = false;
        } else if (this._selected.size === all.length) {
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
            ? (window.i18n
                ? window.i18n.t('dialogs.confirm_delete_file', { name: items[0].name })
                : `Are you sure you want to move "${items[0].name}" to trash?`)
            : (window.i18n
                ? window.i18n.t('batch.confirm_delete', { count: n })
                : `Are you sure you want to move ${n} items to trash?`);

        const confirmed = await showConfirmDialog({
            title: window.i18n ? window.i18n.t('dialogs.confirm_delete') : 'Move to trash',
            message: msg,
            confirmText: window.i18n ? window.i18n.t('actions.delete') : 'Delete',
        });
        if (!confirmed) return;

        const fileIds = items.filter(i => i.type === 'file').map(i => i.id);
        const folderIds = items.filter(i => i.type === 'folder').map(i => i.id);

        try {
            const response = await fetch('/api/batch/trash', {
                method: 'POST',
                headers: { ...getAuthHeaders(), 'Content-Type': 'application/json' },
                body: JSON.stringify({ file_ids: fileIds, folder_ids: folderIds })
            });

            const data = await response.json();
            const success = data.stats?.successful || 0;
            const errors = data.stats?.failed || 0;

            this.clear();
            window.loadFiles();

            if (errors > 0) {
                const failedNames = (data.failed || []).map(f => f.id).join(', ');
                window.ui.showNotification('Batch delete',
                    `${success} moved to trash, ${errors} failed`);
            } else {
                window.ui.showNotification('Moved to trash',
                    `${success} item${success !== 1 ? 's' : ''} moved to trash`);
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

        // Set a special batch mode flag
        window.app.moveDialogMode = 'batch';
        window.app.batchMoveItems = items;

        // Reset selection
        window.app.selectedTargetFolderId = "";

        // Update dialog title
        const dialog = document.getElementById('move-file-dialog');
        const dialogHeader = dialog.querySelector('.rename-dialog-header');
        const n = items.length;
        const titleText = window.i18n
            ? window.i18n.t('batch.move_title', { count: n })
            : `Move ${n} item${n !== 1 ? 's' : ''}`;
        dialogHeader.innerHTML = `<i class="fas fa-arrows-alt" style="color:#ff5e3a"></i> <span>${titleText}</span>`;

        // Load folders, excluding selected folder IDs
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
            const fileIds = items.filter(i => i.type === 'file').map(i => i.id);
            const folderIds = items.filter(i => i.type === 'folder').map(i => i.id);

            const response = await fetch('/api/batch/download', {
                method: 'POST',
                headers: { ...getAuthHeaders(), 'Content-Type': 'application/json' },
                body: JSON.stringify({ file_ids: fileIds, folder_ids: folderIds })
            });

            if (!response.ok) {
                throw new Error(`Server returned ${response.status}`);
            }

            const blob = await response.blob();
            const url = URL.createObjectURL(blob);
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

    // ── Initialization ──────────────────────────────────────

    init() {
        // Inject the select-all checkbox into the list header
        this._injectListHeaderCheckbox();

        // Override the deselect-on-empty-area handler to also clear our state
        this._hookGlobalDeselect();

        // Hook into the move dialog confirm to handle batch mode
        // (handled in contextMenus.js — moveDialogMode === 'batch')

        // Keyboard shortcut: Ctrl+A to select all, Escape to clear
        document.addEventListener('keydown', (e) => {
            // Don't trigger when inside an input/textarea/modal
            if (e.target.closest('input, textarea, [contenteditable], .rename-dialog, .share-dialog, .confirm-dialog')) return;

            if ((e.ctrlKey || e.metaKey) && e.key === 'a') {
                // Only when in file view (not favorites, trash etc.)
                const grid = document.getElementById('files-grid');
                if (grid && grid.closest('.files-container')) {
                    e.preventDefault();
                    this.selectAll();
                }
            }

            if (e.key === 'Escape' && this.hasSelection) {
                this.clear();
            }

            if (e.key === 'Delete' && this.hasSelection) {
                this.batchDelete();
            }
        });
    },

    /** Inject a checkbox into the list-header (or wire existing one) */
    _injectListHeaderCheckbox() {
        const cb = document.getElementById('select-all-checkbox');
        if (!cb) return;

        cb.addEventListener('change', () => {
            this.toggleAll();
        });
    },

    /** Override global click deselect to also clear our internal state */
    _hookGlobalDeselect() {
        document.addEventListener('click', (e) => {
            // Don't deselect if clicking on batch bar, context menu, modal, or any file item
            if (e.target.closest('.file-card, .file-item, .context-menu, .batch-action-bar, .about-modal, .rename-dialog, .share-dialog, .confirm-dialog, .modal-overlay, input, button')) return;

            if (this.hasSelection) {
                this.clear();
            }
        });
    }
};

// Expose globally
window.multiSelect = multiSelect;