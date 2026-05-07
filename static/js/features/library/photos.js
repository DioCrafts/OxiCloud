/**
 * OxiCloud - Photos Timeline View
 * Photo grid grouped by day/month/year, with infinite scroll and multi-select.
 */

import { getCsrfHeaders } from '../../core/csrf.js';
import { i18n } from '../../core/i18n.js';
import { thumbnail } from '../thumbnail.js';
import { photosLightbox } from './photosLightbox.js';

/** @import {FileItem} from '../../core/types.js' */

/**
 * @typedef {'daily'|'monthly'|'yearly'} PhotoModeEnum
 */

const photosView = {
    /** @type {Array<FileItem>} All loaded photo items */
    items: [],
    /** @type {string|null} Cursor for next page */
    nextCursor: null,
    /** @type {boolean} Currently fetching */
    loading: false,
    /** @type {boolean} All items loaded */
    exhausted: false,
    /** @type {Set<string>} Selected item IDs */
    selected: new Set(),
    /** @type {IntersectionObserver|null} */
    _observer: null,
    /** @type {HTMLElement|null} */
    _container: null,
    /** @type {boolean} */
    _initialized: false,
    /** @type {PhotoModeEnum} */
    groupMode: 'monthly',
    /** @type {Map<string, string>} fileId → thumbnail URL (persists across re-renders) */
    _videoThumbCache: new Map(),
    /** @type {number} Items already rendered in the DOM */
    _renderedCount: 0,

    PAGE_SIZE: 200,

    /** Auth headers (HttpOnly cookies) */
    _headers(json = false) {
        const h = getCsrfHeaders();
        if (json) h['Content-Type'] = 'application/json';
        return h;
    },

    /** Initialize / re-initialize the photos view */
    init() {
        if (!this._container) {
            const contentArea = document.querySelector('.content-area');
            if (!contentArea) return;
            const el = document.createElement('div');
            el.id = 'photos-container';
            el.className = 'photos-container';
            contentArea.appendChild(el);
            this._container = el;
        }
        if (!this._initialized) {
            this.groupMode = /** @type {'daily'|'monthly'|'yearly'} */ (localStorage.getItem('oxicloud-photos-group')) || 'monthly';
            this._initialized = true;
        }
    },

    /** Show the photos view and load data */
    show() {
        this.init();
        if (!this._container) return;
        this._container.classList.add('active');
        this.items = [];
        this.nextCursor = null;
        this.exhausted = false;
        this.selected.clear();
        this._renderedCount = 0;
        this._container.innerHTML = '';
        this._loadPage();
    },

    /** Hide the photos view */
    hide() {
        if (this._container) {
            this._container.classList.remove('active');
        }
        this._destroyObserver();
        this._hideSelectionBar();
    },

    /** Switch grouping mode */
    /**
     *
     * @param {PhotoModeEnum} mode
     * @returns
     */
    setGroupMode(mode) {
        if (this.groupMode === mode) return;
        this.groupMode = mode;
        localStorage.setItem('oxicloud-photos-group', mode);
        this._renderedCount = 0;
        this._renderFull();
    },

    /** Fetch a page of photos from the API */
    async _loadPage() {
        if (this.loading || this.exhausted) return;
        this.loading = true;
        this._showLoading(true);
        const prevCount = this.items.length;

        try {
            let url = `/api/photos?limit=${this.PAGE_SIZE}`;
            if (this.nextCursor) {
                url += `&before=${this.nextCursor}`;
            }

            const res = await fetch(url, {
                credentials: 'include',
                headers: this._headers()
            });

            if (!res.ok) throw new Error(`HTTP ${res.status}`);

            /** @type {FileItem[]} */
            const data = await res.json();

            if (!data || data.length === 0) {
                this.exhausted = true;
            } else {
                this.items.push(...data);
                const cursor = res.headers.get('X-Next-Cursor');
                if (cursor && data.length >= this.PAGE_SIZE) {
                    this.nextCursor = cursor;
                } else {
                    this.exhausted = true;
                }
            }
        } catch (err) {
            console.error('Error loading photos:', err);
            this.exhausted = true;
        } finally {
            this.loading = false;
            this._showLoading(false);
            if (prevCount === 0) {
                this._renderFull();
            } else {
                this._appendBatch(prevCount);
            }
        }
    },

    // ── Rendering ───────────────────────────────────────────────────
    // Two render paths:
    //   _renderFull()   — full DOM rebuild (first load, group-mode change, delete)
    //   _appendBatch(n) — append-only for infinite-scroll pages (O(batch))

    /** Full DOM rebuild — first load, group-mode switch, or after deletions. */
    _renderFull() {
        if (!this._container) return;
        this._destroyObserver();

        this._container.classList.remove('photos-group-daily', 'photos-group-monthly', 'photos-group-yearly');
        this._container.classList.add(`photos-group-${this.groupMode}`);

        if (this.items.length === 0 && this.exhausted) {
            this._renderEmpty();
            return;
        }
        if (this.items.length === 0) return;

        const groups = this._groupItems(this.items);
        let html = this._renderToolbar();

        for (const [label, files] of groups) {
            html += `<div class="photos-day-header" data-group="${this._escAttr(label)}">${this._escHtml(label)}<span class="photos-day-count">${files.length}</span></div>`;
            html += '<div class="photos-grid">';
            for (const file of files) html += this._renderTile(file);
            html += '</div>';
        }

        html += '<div class="photos-sentinel"></div>';
        this._container.innerHTML = html;
        this._container.onclick = (e) => this._handleClick(e);
        this._renderedCount = this.items.length;
        this._observeSentinel();
        this._setupVideoThumbnails();
    },

    /** Append-only render for infinite scroll — inserts only the items
     *  from this.items[startIndex..] without destroying existing DOM.
     *  Complexity: O(batch) instead of O(total_items).
     * @param {number} startIndex
     */
    _appendBatch(startIndex) {
        if (!this._container) return;
        this._destroyObserver();

        const newItems = this.items.slice(startIndex);
        if (newItems.length === 0) {
            this._observeSentinel();
            return;
        }

        const newGroups = this._groupItems(newItems);
        const sentinel = this._container.querySelector('.photos-sentinel');
        if (!sentinel) {
            // Fallback: sentinel missing — full rebuild
            this._renderedCount = 0;
            this._renderFull();
            return;
        }

        for (const [label, files] of newGroups) {
            let tilesHtml = '';
            for (const file of files) tilesHtml += this._renderTile(file);

            // Does this date-group already exist in the DOM?
            const existingHeader = this._container.querySelector(`.photos-day-header[data-group="${CSS.escape(label)}"]`);

            if (existingHeader) {
                // Append tiles to existing grid and update count badge
                const grid = existingHeader.nextElementSibling;
                if (grid?.classList.contains('photos-grid')) {
                    grid.insertAdjacentHTML('beforeend', tilesHtml);
                    const countSpan = existingHeader.querySelector('.photos-day-count');
                    if (countSpan) countSpan.textContent = String(grid.children.length);
                }
            } else {
                // New group — insert header + grid before sentinel
                const sectionHtml =
                    `<div class="photos-day-header" data-group="${this._escAttr(label)}">${this._escHtml(label)}<span class="photos-day-count">${files.length}</span></div>` +
                    `<div class="photos-grid">${tilesHtml}</div>`;
                sentinel.insertAdjacentHTML('beforebegin', sectionHtml);
            }
        }

        this._renderedCount = this.items.length;
        this._observeSentinel();
        this._setupVideoThumbnails(startIndex);
    },

    /**
     * Generate HTML for a single photo/video tile
     * @param {FileItem} file
     */
    _renderTile(file) {
        const isVideo = file.mime_type?.startsWith('video/');
        const selected = this.selected.has(file.id) ? ' selected' : '';
        const cachedThumb = isVideo && this._videoThumbCache.has(file.id) ? this._videoThumbCache.get(file.id) : null;
        const thumbUrl = cachedThumb || `/api/files/${file.id}/thumbnail/preview`;
        let h = `<div class="photo-tile${selected}" data-id="${this._escAttr(file.id)}" data-mime="${this._escAttr(file.mime_type)}" data-name="${this._escAttr(file.name)}">`;
        h += `<div class="photo-check"><i class="fas fa-check"></i></div>`;
        h += `<img src="${thumbUrl}" loading="lazy" alt="${this._escAttr(file.name)}">`;
        if (isVideo) h += `<div class="video-badge"><i class="fas fa-play"></i></div>`;
        h += `</div>`;
        return h;
    },

    /** (Re-)observe the sentinel element for infinite scroll */
    _observeSentinel() {
        this._destroyObserver();
        const sentinel = this._container?.querySelector('.photos-sentinel');
        if (sentinel && !this.exhausted) {
            this._observer = new IntersectionObserver(
                (entries) => {
                    if (entries[0].isIntersecting) this._loadPage();
                },
                { rootMargin: '400px' }
            );
            this._observer.observe(sentinel);
        }
    },

    // ── Client-side video thumbnail generation ──────────────────────
    // Uses the browser's native video decoder (<video> + <canvas>) to
    // extract a frame, show it immediately, and upload to the server
    // for permanent caching.  Zero server-side dependencies (no ffmpeg).

    /** Attach error handlers to video tile images; on failure, extract a
     *  frame from the video using the browser's built-in codec. */
    /** @param {number} [startIndex=0] When > 0, only process video tiles
     *  for items[startIndex..] — avoids re-scanning the entire DOM. */
    _setupVideoThumbnails(startIndex = 0) {
        const tiles = /** @type {NodeListOf<HTMLDivElement>} */ (this._container?.querySelectorAll('.photo-tile[data-mime^="video/"]'));
        const newIds = startIndex > 0 ? new Set(this.items.slice(startIndex).map((f) => f.id)) : null;

        if (!tiles) return;

        for (const tile of tiles) {
            const fileId = tile.dataset.id;
            if (!fileId) continue;
            if (newIds && !newIds.has(fileId)) continue;
            if (this._videoThumbCache.has(fileId)) continue;

            const img = tile.querySelector('img');
            if (!img) continue;

            img.addEventListener(
                'error',
                () => {
                    this._generateVideoThumbnail(tile, img);
                },
                { once: true }
            );
        }
    },

    /**
     * Extract a frame and upload all thumbnail sizes via thumbnail.queueGenerate().
     * @param {HTMLDivElement} tile
     * @param {HTMLImageElement} img
     */
    async _generateVideoThumbnail(tile, img) {
        const fileId = tile.dataset.id;
        // TODO: remove this HACK, this is not evolutive...
        const file = /** @type {FileItem} */ ({ id: fileId, icon_special_class: 'video-icon', name: tile.dataset.name, mime_type: tile.dataset.mime });

        try {
            await thumbnail.queueGenerate(file, null, (previewDataUrl) => {
                img.src = previewDataUrl;
                this._videoThumbCache.set(fileId, previewDataUrl);
            });
            // Switch to permanent server URL so the data URL can be GC'd
            this._videoThumbCache.set(fileId, `/api/files/${fileId}/thumbnail/preview?v=1`);
        } catch {
            // Keep generic play badge on error
        }
    },

    /** Render the group mode toolbar */
    _renderToolbar() {
        const modes = [
            ['daily', i18n.t('photos.view_daily')],
            ['monthly', i18n.t('photos.view_monthly')],
            ['yearly', i18n.t('photos.view_yearly')]
        ];
        let html = '<div class="photos-toolbar"><div class="view-toggle">';
        for (const [mode, label] of modes) {
            const active = this.groupMode === mode ? ' active' : '';
            html += `<button class="toggle-btn${active}" data-group-mode="${mode}">${this._escHtml(label)}</button>`;
        }
        html += '</div></div>';
        return html;
    },

    /** Render empty state */
    _renderEmpty() {
        if (!this._container) return;
        this._container.innerHTML = `
            <div class="photos-empty">
                <i class="fas fa-images"></i>
                <p class="photos-empty-title">${i18n.t('photos.empty_state')}</p>
                <p>${i18n.t('photos.empty_hint')}</p>
            </div>`;
    },

    /**
     *  Group items by the current groupMode
     * @param {FileItem[]} items
     */
    _groupItems(items) {
        const map = new Map();
        for (const item of items) {
            const ts = (item.sort_date || item.created_at) * 1000;
            const d = new Date(ts);
            let key;
            if (this.groupMode === 'yearly') {
                key = String(d.getFullYear());
            } else if (this.groupMode === 'monthly') {
                key = d.toLocaleDateString(undefined, {
                    year: 'numeric',
                    month: 'long'
                });
            } else {
                key = d.toLocaleDateString(undefined, {
                    weekday: 'long',
                    year: 'numeric',
                    month: 'long',
                    day: 'numeric'
                });
            }
            if (!map.has(key)) map.set(key, []);
            map.get(key).push(item);
        }
        return map;
    },

    /**
     * Handle click on photo tile or toolbar
     * @param {MouseEvent} e
     */
    _handleClick(e) {
        // Handle group mode toggle
        const target = /** @type {Element} */ (e.target);
        const modeBtn = /** @type {HTMLButtonElement} */ (target.closest('[data-group-mode]'));
        if (modeBtn) {
            this.setGroupMode(/** @type {PhotoModeEnum} */ (modeBtn.dataset.groupMode));
            return;
        }

        const tile = /** @type {HTMLDivElement} */ (target.closest('.photo-tile'));
        if (!tile) return;

        const id = tile.dataset.id;
        const check = target.closest('.photo-check');

        // If clicking checkbox or in selection mode, toggle select
        if (check || this.selected.size > 0) {
            this._toggleSelect(id, tile);
            return;
        }

        // Otherwise open lightbox
        const idx = this.items.findIndex((f) => f.id === id);
        if (idx >= 0) {
            photosLightbox.open(this.items, idx);
        }
    },

    /**
     * Toggle selection of an item
     * @param {string} id
     * @param {HTMLDivElement} tile
     */
    _toggleSelect(id, tile) {
        if (this.selected.has(id)) {
            this.selected.delete(id);
            tile.classList.remove('selected');
        } else {
            this.selected.add(id);
            tile.classList.add('selected');
        }
        this._updateSelectionBar();
    },

    /** Show/update selection bar */
    _updateSelectionBar() {
        let bar = document.getElementById('photos-selection-bar');

        if (this.selected.size === 0) {
            this._hideSelectionBar();
            return;
        }

        if (!bar) {
            bar = document.createElement('div');
            bar.id = 'photos-selection-bar';
            bar.className = 'photos-selection-bar';
            document.body.appendChild(bar);
        }

        const count = this.selected.size;
        bar.innerHTML = `
            <span class="selection-count">${count} ${i18n.t('photos.items_selected')}</span>
            <button id="photos-sel-download" title="Download"><i class="fas fa-download"></i></button>
            <button id="photos-sel-delete" title="Delete"><i class="fas fa-trash"></i></button>
            <button id="photos-sel-clear" title="Clear"><i class="fas fa-times"></i></button>
        `;

        const bar_clear = /** @type {HTMLButtonElement} */ (bar.querySelector('#photos-sel-clear'));
        if (bar_clear) {
            bar_clear.onclick = () => {
                this.selected.clear();
                this._container.querySelectorAll('.photo-tile.selected').forEach((t) => {
                    t.classList.remove('selected');
                });
                this._hideSelectionBar();
            };
        }

        const bar_delete = /** @type {HTMLButtonElement} */ (bar.querySelector('#photos-sel-delete'));
        if (bar_delete) {
            bar_delete.onclick = async () => {
                if (!confirm('Delete selected items?')) return;
                for (const fid of this.selected) {
                    try {
                        await fetch(`/api/files/${fid}`, {
                            method: 'DELETE',
                            credentials: 'include',
                            headers: this._headers()
                        });
                    } catch (err) {
                        console.error('Delete failed:', fid, err);
                    }
                }
                this.items = this.items.filter((f) => !this.selected.has(f.id));
                this.selected.clear();
                this._hideSelectionBar();
                this._renderedCount = 0;
                this._renderFull();
            };
        }

        const bar_download = /** @type {HTMLButtonElement} */ (bar.querySelector('#photos-sel-download'));
        if (bar_download) {
            bar_download.onclick = async () => {
                for (const fid of this.selected) {
                    const a = document.createElement('a');
                    a.href = `/api/files/${fid}`;
                    a.download = '';
                    document.body.appendChild(a);
                    a.click();
                    a.remove();
                }
            };
        }

        bar.style.display = 'flex';
    },

    _hideSelectionBar() {
        const bar = document.getElementById('photos-selection-bar');
        if (bar) bar.style.display = 'none';
    },

    /** @param {boolean} show */
    _showLoading(show) {
        if (!this._container) return;
        let loader = this._container.querySelector('.photos-loading');
        if (show && !loader) {
            loader = document.createElement('div');
            loader.className = 'photos-loading';
            loader.innerHTML = '<i class="fas fa-spinner"></i> Loading...';
            this._container.appendChild(loader);
        } else if (!show && loader) {
            loader.remove();
        }
    },

    _destroyObserver() {
        if (this._observer) {
            this._observer.disconnect();
            this._observer = null;
        }
    },

    /** @param {any} s */
    _escHtml(s) {
        const d = document.createElement('div');
        d.textContent = s;
        return d.innerHTML;
    },

    /** @param {any} s */
    _escAttr(s) {
        return String(s || '')
            .replace(/"/g, '&quot;')
            .replace(/</g, '&lt;');
    }
};

photosLightbox.setPhotosView(photosView);

export { photosView };
