/**
 * OxiCloud - Multi-Select & Batch Actions Module
 *
 * Adds checkboxes to grid and list views, replaces the list-view header
 * with a NextCloud-style selection bar when items are selected, and
 * provides batch delete / move / download / favorites operations.
 */

// TODO: rename into selection-bar ?
// TODO: merge with photo part

// @ts-check

import { loadFiles } from '../../app/filesView.js';
import { app } from '../../app/state.js';
import { showConfirmDialog, ui } from '../../app/ui.js';
import { i18n } from '../../core/i18n.js';
import { favorites } from '../library/favorites.js';
import { contextMenus } from './contextMenus.js';
import { getAuthHeaders } from './fileOperations.js';

const multiSelect = {
    /** Currently selected items: Map<id, { id, name, type, parentId }> */
    _selected: new Map(),

    /** Last clicked index for Shift-range selection */
    _lastClickedIndex: -1,

    /** Whether the selection bar is currently visible */
    _barVisible: false,

    // ── Public API ──────────────────────────────────────────

    get count() {
        return this._selected.size;
    },
    get items() {
        return Array.from(this._selected.values());
    },
    get hasSelection() {
        return this._selected.size > 0;
    },
    get files() {
        return this.items.filter((i) => i.type === 'file');
    },
    get folders() {
        return this.items.filter((i) => i.type === 'folder');
    },

    // ── Helpers for i18n ────────────────────────────────────

    _t(key, vars) {
        const val = i18n.t(key, vars);
        return val !== key ? val : null;
    },

    // ── Selection state management ──────────────────────────

    toggle(id, name, type, parentId) {
        if (this._selected.has(id)) {
            this._selected.delete(id);
            return false;
        }
        this._selected.set(id, { id, name, type, parentId });
        return true;
    },

    select(id, name, type, parentId) {
        this._selected.set(id, { id, name, type, parentId });
    },

    deselect(id) {
        this._selected.delete(id);
    },

    clear() {
        this._selected.clear();
        this._lastClickedIndex = -1;
        document.querySelectorAll('.file-item.selected').forEach((el) => {
            el.classList.remove('selected');
        });
        document.querySelectorAll('.item-checkbox').forEach((cb) => {
            cb.checked = false;
        });
        this._syncUI();
    },

    selectAll() {
        this._selectAllInContainer('files-list', '.file-item');
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

    /**
     * @typedef {Object} ItemSelection
     * @property {string[]} fileIds list of files' id
     * @property {string[]} folderIds list of folders' id
     */

    /**
     * get selection
     * @param {string} [targtFolderId] an optional targget (will be removed from selected item)
     * @return {ItemSelection}
     */
    getSelection(targtFolderId) {
        const fileIds = [];
        const folderIds = [];

        // TODO optimize & check if _selected is a better use
        document.querySelectorAll(`div.file-item.selected`).forEach((item) => {
            if (item.dataset.fileId) {
                fileIds.push(item.dataset.fileId);
            } else {
                // ignore selectedItem if this is the target
                if (targtFolderId && targtFolderId !== item.dataset.folderId) folderIds.push(item.dataset.folderId);
            }
        });

        return {
            fileIds: fileIds,
            folderIds: folderIds
        };
    },

    /**
     * @param {string} action move|copy
     * @param {BatchResult} result result of batch
     */
    showBatchResult(action, result) {
        if (action === 'copy') {
            if (result.errors > 0) {
                ui.showNotification('Batch copy', `${result.success} copied, ${result.errors} failed`);
            } else {
                ui.showNotification('Items copied', `${result.success} item${result.success !== 1 ? 's' : ''} copied successfully`);
            }
        } else {
            if (result.errors > 0) {
                ui.showNotification('Batch move', `${result.success} moved, ${result.errors} failed`);
            } else {
                ui.showNotification('Items moved', `${result.success} item${result.success !== 1 ? 's' : ''} moved successfully`);
            }
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
        container.querySelectorAll(selector).forEach((el) => {
            this._selectElement(el);
        });
    },

    _getAllVisibleItems() {
        return [...document.querySelectorAll('.file-item')];
    },

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

    handleToggleItem(el, event) {
        const items = this._getAllVisibleItems();
        const index = items.indexOf(el);
        const info = this._extractInfo(el);
        if (!info) return;

        if (event?.shiftKey && this._lastClickedIndex >= 0 && index >= 0) {
            const start = Math.min(this._lastClickedIndex, index);
            const end = Math.max(this._lastClickedIndex, index);
            for (let i = start; i <= end; i++) {
                this._selectElement(items[i]);
                const iInfo = this._extractInfo(items[i]);
                if (iInfo) {
                    const sel = iInfo.type === 'folder' ? `[data-folder-id="${iInfo.id}"]` : `[data-file-id="${iInfo.id}"]`;
                    document.querySelectorAll(sel).forEach((e) => {
                        e.classList.add('selected');
                        const checkbox = e.querySelector('input[type="checkbox"]');
                        if (checkbox) checkbox.checked = true;
                    });
                }
            }
        } else {
            const nowSelected = this.toggle(info.id, info.name, info.type, info.parentId);
            el.classList.toggle('selected', nowSelected);
            const checkbox = el.querySelector('input[type="checkbox"]');
            if (checkbox) checkbox.checked = nowSelected;
        }
        this._lastClickedIndex = index;
        this._syncUI();
        this._syncSelectAllCheckbox();
    },

    /** Main UI sync — called after every selection change */
    _syncUI() {
        const n = this._selected.size;

        const multiSelectButtons = document.getElementById('multi-select-buttons');
        const defaultButtons = document.getElementById('default-buttons');

        if (n > 0) {
            this._barVisible = true;

            const countText = n === 1 ? this._t('batch.one_selected') || '1 item selected' : this._t('batch.n_selected', { count: n }) || `${n} items selected`;
            document.getElementById('batch-bar-count').innerText = countText;

            defaultButtons?.classList.add('hidden');
            multiSelectButtons?.classList.remove('hidden');
        } else {
            this._barVisible = false;

            // Hide grid bar
            multiSelectButtons?.classList.add('hidden');
            defaultButtons?.classList.remove('hidden');
        }

        // Sync individual item checkboxes
        this._syncItemCheckboxes();
        // Sync select-all checkbox state (for non-selection-mode)
        this._syncSelectAllCheckbox();
    },

    /** Wire click handlers on batch action buttons (idempotent per render) */
    _wireBarButtons() {
        const del = document.getElementById('batch-delete');
        const move = document.getElementById('batch-move');
        const dl = document.getElementById('batch-download');
        const fav = document.getElementById('batch-fav');
        const closeBtn = document.getElementById('batch-selection-close');
        if (del) del.onclick = () => this.batchDelete();
        if (move) move.onclick = () => this.batchMove();
        if (dl) dl.onclick = () => this.batchDownload();
        if (fav) fav.onclick = () => this.batchFavorites();
        if (closeBtn) closeBtn.onclick = () => this.clear();
    },

    _syncItemCheckboxes() {
        document.querySelectorAll('.file-item').forEach((el) => {
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
        const msg =
            n === 1
                ? this._t('dialogs.confirm_delete_file', { name: items[0].name }) || `Are you sure you want to move "${items[0].name}" to trash?`
                : this._t('batch.confirm_delete', { count: n }) || `Are you sure you want to move ${n} items to trash?`;

        const confirmed = await showConfirmDialog({
            title: this._t('dialogs.confirm_delete') || 'Move to trash',
            message: msg,
            confirmText: this._t('actions.delete') || 'Delete'
        });
        if (!confirmed) return;

        const fileIds = items.filter((i) => i.type === 'file').map((i) => i.id);
        const folderIds = items.filter((i) => i.type === 'folder').map((i) => i.id);

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
            loadFiles();

            if (errors > 0) {
                ui.showNotification('Batch delete', `${success} moved to trash, ${errors} failed`);
            } else {
                ui.showNotification('Moved to trash', `${success} item${success !== 1 ? 's' : ''} moved to trash`);
            }
        } catch (e) {
            console.error('Batch trash error:', e);
            ui.showNotification('Error', 'Could not move items to trash');
            this.clear();
            loadFiles();
        }
    },

    /** Batch move — reuse existing move dialog */
    async batchMove() {
        const items = this.items;
        if (items.length === 0) return;

        app.moveDialogMode = 'batch';
        app.batchMoveItems = items;
        app.selectedTargetFolderId = '';

        const dialog = document.getElementById('move-file-dialog');
        const dialogHeader = dialog.querySelector('.rename-dialog-header');
        const n = items.length;
        const titleText = this._t('batch.move_title', { count: n }) || `Move ${n} item${n !== 1 ? 's' : ''}`;
        dialogHeader.innerHTML = `<i class="fas fa-arrows-alt dialog-header-icon"></i> <span>${titleText}</span>`;

        const excludeIds = items.filter((i) => i.type === 'folder').map((i) => i.id);
        await contextMenus.loadAllFolders(excludeIds[0] || null, 'batch');
        dialog.style.display = 'flex';
    },

    /** Batch download — downloads all selected items as a single ZIP */
    async batchDownload() {
        const items = this.items;
        if (items.length === 0) return;

        ui.showNotification('Preparing download', 'Creating ZIP archive...');

        try {
            const fileIds = items.filter((i) => i.type === 'file').map((i) => i.id);
            const folderIds = items.filter((i) => i.type === 'folder').map((i) => i.id);

            const response = await fetch('/api/batch/download', {
                method: 'POST',
                headers: { ...getAuthHeaders(), 'Content-Type': 'application/json' },
                body: JSON.stringify({ file_ids: fileIds, folder_ids: folderIds })
            });

            if (!response.ok) throw new Error(`Server returned ${response.status}`);

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
            ui.showNotification('Error', 'Could not download selected items');
        }
    },

    /** Batch add to favorites — single API call */
    async batchFavorites() {
        const items = this.items;
        if (items.length === 0 || !favorites) return;

        // Filter out items already in favourites
        const toAdd = items.filter((i) => !favorites.isFavorite(i.id, i.type));
        if (toAdd.length === 0) {
            this.clear();
            ui.showNotification(this._t('favorites.add') || 'Favorites', 'All selected items are already favorites');
            return;
        }

        try {
            const response = await fetch('/api/favorites/batch', {
                method: 'POST',
                headers: { ...getAuthHeaders(), 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    items: toAdd.map((i) => ({ item_id: i.id, item_type: i.type }))
                })
            });

            if (!response.ok) throw new Error(`Server returned ${response.status}`);

            const data = await response.json();
            const inserted = data.stats?.inserted || 0;

            // Replace cache directly from response (no extra GET)
            if (data.favorites && favorites._replaceCacheFromResponse) {
                favorites._replaceCacheFromResponse(data.favorites);
            } else {
                await favorites._fetchFromServer();
            }

            this.clear();
            loadFiles();

            if (inserted > 0) {
                ui.showNotification(this._t('favorites.add') || 'Added to favorites', `${inserted} item${inserted !== 1 ? 's' : ''} added to favorites`);
            } else {
                ui.showNotification(this._t('favorites.add') || 'Favorites', 'All selected items are already favorites');
            }
        } catch (e) {
            console.error('Batch favorites error:', e);
            ui.showNotification('Error', 'Could not add items to favorites');
        }
    },

    // ── Initialization ──────────────────────────────────────

    init() {
        // Wire the initial select-all checkbox
        this._injectListHeaderCheckbox();

        // Keyboard shortcuts
        document.addEventListener('keydown', (e) => {
            if (e.target.closest('input, textarea, [contenteditable], .rename-dialog, .share-dialog, .confirm-dialog')) return;

            const selectAllCheckbox = document.getElementById('select-all-checkbox');
            // ctrl+a cmd+a
            if ((e.ctrlKey || e.metaKey) && e.key === 'a') {
                if (selectAllCheckbox) selectAllCheckbox.checked = true;
                this.selectAll();
                e.preventDefault();
            }
            if (e.key === 'Escape' && this.hasSelection) {
                this.clear();
                if (selectAllCheckbox) selectAllCheckbox.checked = false;
            }
            if (e.key === 'Delete' && this.hasSelection) this.batchDelete();
        });

        this._wireBarButtons();
    },

    // FIXME: competition with _
    _injectListHeaderCheckbox() {
        const selectAllCheckbox = document.getElementById('select-all-checkbox');
        if (!selectAllCheckbox) return;
        selectAllCheckbox.addEventListener('change', () => this.toggleAll());
    }
};

export { multiSelect };
