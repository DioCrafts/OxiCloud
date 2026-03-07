/**
 * OxiCloud - Photos Timeline View
 * Photo grid grouped by day/month/year, with infinite scroll and multi-select.
 */

const photosView = {
    /** @type {Array} All loaded photo items */
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
    /** @type {'daily'|'monthly'|'yearly'} */
    groupMode: 'monthly',
    /** @type {Map<string, string>} fileId → thumbnail URL (persists across re-renders) */
    _videoThumbCache: new Map(),
    /** @type {number} Max concurrent video thumbnail extractions */
    _maxConcurrentDecodes: 3,
    /** @type {number} Currently running video decodes */
    _activeDecodes: 0,
    /** @type {Array} Pending video decode queue */
    _decodeQueue: [],

    PAGE_SIZE: 200,

    /** Auth headers (HttpOnly cookies) */
    _headers(json = false) {
        const h = typeof getCsrfHeaders === 'function' ? { ...getCsrfHeaders() } : {};
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
            this.groupMode = localStorage.getItem('oxicloud-photos-group') || 'monthly';
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
        this._render();
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
    setGroupMode(mode) {
        if (this.groupMode === mode) return;
        this.groupMode = mode;
        localStorage.setItem('oxicloud-photos-group', mode);
        this._render();
    },

    /** Fetch a page of photos from the API */
    async _loadPage() {
        if (this.loading || this.exhausted) return;
        this.loading = true;
        this._showLoading(true);

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
            this._render();
        }
    },

    /** Render the full timeline from this.items */
    _render() {
        if (!this._container) return;
        this._destroyObserver();

        // Set group mode class on container
        this._container.classList.remove('photos-group-daily', 'photos-group-monthly', 'photos-group-yearly');
        this._container.classList.add(`photos-group-${this.groupMode}`);

        if (this.items.length === 0 && this.exhausted) {
            this._renderEmpty();
            return;
        }

        if (this.items.length === 0) return;

        // Group by selected mode
        const groups = this._groupItems(this.items);
        let html = this._renderToolbar();

        for (const [label, files] of groups) {
            html += `<div class="photos-day-header">${this._escHtml(label)}<span class="photos-day-count">${files.length}</span></div>`;
            html += '<div class="photos-grid">';
            for (const file of files) {
                const isVideo = file.mime_type && file.mime_type.startsWith('video/');
                const selected = this.selected.has(file.id) ? ' selected' : '';
                // For videos with a cached local thumb, use it directly;
                // avoids the 204 → error → re-decode cycle on re-renders.
                const cachedThumb = isVideo && this._videoThumbCache.has(file.id)
                    ? this._videoThumbCache.get(file.id)
                    : null;
                const thumbUrl = cachedThumb || `/api/files/${file.id}/thumbnail/preview`;
                html += `<div class="photo-tile${selected}" data-id="${this._escAttr(file.id)}" data-mime="${this._escAttr(file.mime_type)}">`;
                html += `<div class="photo-check"><i class="fas fa-check"></i></div>`;
                html += `<img src="${thumbUrl}" loading="lazy" alt="${this._escAttr(file.name)}">`;
                if (isVideo) {
                    html += `<div class="video-badge"><i class="fas fa-play"></i></div>`;
                }
                html += `</div>`;
            }
            html += '</div>';
        }

        // Sentinel for infinite scroll
        html += '<div class="photos-sentinel"></div>';

        this._container.innerHTML = html;

        // Attach click handlers via delegation
        this._container.onclick = (e) => this._handleClick(e);

        // Observe sentinel for infinite scroll
        const sentinel = this._container.querySelector('.photos-sentinel');
        if (sentinel && !this.exhausted) {
            this._observer = new IntersectionObserver((entries) => {
                if (entries[0].isIntersecting) {
                    this._loadPage();
                }
            }, { rootMargin: '400px' });
            this._observer.observe(sentinel);
        }

        // Set up client-side video thumbnail generation
        this._setupVideoThumbnails();
    },

    // ── Client-side video thumbnail generation ──────────────────────
    // Uses the browser's native video decoder (<video> + <canvas>) to
    // extract a frame, show it immediately, and upload to the server
    // for permanent caching.  Zero server-side dependencies (no ffmpeg).

    /** Attach error handlers to video tile images; on failure, extract a
     *  frame from the video using the browser's built-in codec. */
    _setupVideoThumbnails() {
        const tiles = this._container.querySelectorAll('.photo-tile[data-mime^="video/"]');
        for (const tile of tiles) {
            const img = tile.querySelector('img');
            if (!img) continue;
            const fileId = tile.dataset.id;

            // Skip if already cached locally (URL was set during render)
            if (this._videoThumbCache.has(fileId)) continue;

            // The server returns 204 for videos without a cached thumbnail,
            // which causes the <img> to fire 'error'.
            img.addEventListener('error', () => {
                this._enqueueVideoThumbnail(tile, img);
            }, { once: true });
        }
    },

    /** Enqueue a video thumbnail decode, respecting concurrency limit. */
    _enqueueVideoThumbnail(tile, img) {
        if (this._activeDecodes < this._maxConcurrentDecodes) {
            this._activeDecodes++;
            this._generateVideoThumbnail(tile, img);
        } else {
            this._decodeQueue.push({ tile, img });
        }
    },

    /** Process next item in the decode queue. */
    _drainDecodeQueue() {
        this._activeDecodes--;
        if (this._decodeQueue.length > 0) {
            const next = this._decodeQueue.shift();
            this._activeDecodes++;
            this._generateVideoThumbnail(next.tile, next.img);
        }
    },

    /** Extract a single frame from a video and display it as the tile
     *  thumbnail, then upload the WebP to the server for caching. */
    _generateVideoThumbnail(tile, img) {
        const fileId = tile.dataset.id;
        const video = document.createElement('video');
        video.crossOrigin = 'anonymous';
        video.preload = 'metadata';
        video.muted = true;
        // Auth is handled via HttpOnly cookie — direct URL works
        video.src = `/api/files/${fileId}`;

        video.addEventListener('loadeddata', () => {
            // Seek to 25 % of duration, clamped between 0.5 s and 5 s
            video.currentTime = Math.min(5, Math.max(0.5, video.duration * 0.25));
        }, { once: true });

        video.addEventListener('seeked', () => {
            // Pre-scale to thumbnail size in the browser — saves ~22× RAM,
            // ~15× bandwidth, and lets the server skip resize entirely.
            const MAX_THUMB = 400; // must match ThumbnailSize::Preview
            const scale = Math.min(MAX_THUMB / video.videoWidth,
                                   MAX_THUMB / video.videoHeight, 1);
            const canvas = document.createElement('canvas');
            canvas.width  = Math.round(video.videoWidth  * scale);
            canvas.height = Math.round(video.videoHeight * scale);
            const ctx = canvas.getContext('2d');
            ctx.drawImage(video, 0, 0, canvas.width, canvas.height);

            // JPEG: explicit quality control, universally supported,
            // and server stores as-is when dimensions fit (zero re-encode).
            const mimeType = 'image/jpeg';

            canvas.toBlob((blob) => {
                if (!blob) {
                    this._drainDecodeQueue();
                    return;
                }

                // Show immediately in the tile
                const url = URL.createObjectURL(blob);
                img.src = url;
                // Cache locally so re-renders are instant
                this._videoThumbCache.set(fileId, url);

                // Upload to server for permanent caching
                const token = localStorage.getItem('token')
                    || sessionStorage.getItem('token');
                const headers = { 'Content-Type': blob.type, ...getCsrfHeaders() };
                if (token) headers['Authorization'] = `Bearer ${token}`;

                fetch(`/api/files/${fileId}/thumbnail/preview`, {
                    method: 'PUT',
                    headers,
                    credentials: 'same-origin',
                    body: blob,
                }).then((resp) => {
                    if (resp.ok) {
                        // Switch from blob URL to server URL so the blob
                        // can be garbage-collected and future loads use
                        // the permanently cached WebP from the server.
                        const serverUrl = `/api/files/${fileId}/thumbnail/preview?v=1`;
                        this._videoThumbCache.set(fileId, serverUrl);
                    }
                }).catch(() => { /* best-effort */ });

                // Release video resources
                video.src = '';
                video.load();
                this._drainDecodeQueue();
            }, mimeType, 0.80);
        }, { once: true });

        // If the video can't be loaded at all, keep the generic play badge
        video.addEventListener('error', () => {
            video.src = '';
            video.load();
            this._drainDecodeQueue();
        }, { once: true });
    },

    /** Render the group mode toolbar */
    _renderToolbar() {
        const t = (k, d) => window.i18n ? window.i18n.t(k) : d;
        const modes = [
            ['daily', t('photos.view_daily', 'Day')],
            ['monthly', t('photos.view_monthly', 'Month')],
            ['yearly', t('photos.view_yearly', 'Year')]
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
        const t = (k, d) => window.i18n ? window.i18n.t(k) : d;
        this._container.innerHTML = `
            <div class="photos-empty">
                <i class="fas fa-images"></i>
                <p class="photos-empty-title">${t('photos.empty_state', 'No photos yet')}</p>
                <p>${t('photos.empty_hint', 'Upload images or videos to see them here')}</p>
            </div>`;
    },

    /** Group items by the current groupMode */
    _groupItems(items) {
        const map = new Map();
        for (const item of items) {
            const ts = (item.sort_date || item.created_at) * 1000;
            const d = new Date(ts);
            let key;
            if (this.groupMode === 'yearly') {
                key = String(d.getFullYear());
            } else if (this.groupMode === 'monthly') {
                key = d.toLocaleDateString(undefined, { year: 'numeric', month: 'long' });
            } else {
                key = d.toLocaleDateString(undefined, {
                    weekday: 'long', year: 'numeric', month: 'long', day: 'numeric'
                });
            }
            if (!map.has(key)) map.set(key, []);
            map.get(key).push(item);
        }
        return map;
    },

    /** Handle click on photo tile or toolbar */
    _handleClick(e) {
        // Handle group mode toggle
        const modeBtn = e.target.closest('[data-group-mode]');
        if (modeBtn) {
            this.setGroupMode(modeBtn.dataset.groupMode);
            return;
        }

        const tile = e.target.closest('.photo-tile');
        if (!tile) return;

        const id = tile.dataset.id;
        const check = e.target.closest('.photo-check');

        // If clicking checkbox or in selection mode, toggle select
        if (check || this.selected.size > 0) {
            this._toggleSelect(id, tile);
            return;
        }

        // Otherwise open lightbox
        const idx = this.items.findIndex(f => f.id === id);
        if (idx >= 0 && window.photosLightbox) {
            window.photosLightbox.open(this.items, idx);
        }
    },

    /** Toggle selection of an item */
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

        const t = (k, d) => window.i18n ? window.i18n.t(k) : d;
        const count = this.selected.size;
        bar.innerHTML = `
            <span class="selection-count">${count} ${t('photos.items_selected', 'selected')}</span>
            <button id="photos-sel-download" title="Download"><i class="fas fa-download"></i></button>
            <button id="photos-sel-delete" title="Delete"><i class="fas fa-trash"></i></button>
            <button id="photos-sel-clear" title="Clear"><i class="fas fa-times"></i></button>
        `;

        bar.querySelector('#photos-sel-clear').onclick = () => {
            this.selected.clear();
            this._container.querySelectorAll('.photo-tile.selected').forEach(t => t.classList.remove('selected'));
            this._hideSelectionBar();
        };

        bar.querySelector('#photos-sel-delete').onclick = async () => {
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
            this.items = this.items.filter(f => !this.selected.has(f.id));
            this.selected.clear();
            this._hideSelectionBar();
            this._render();
        };

        bar.querySelector('#photos-sel-download').onclick = async () => {
            for (const fid of this.selected) {
                const a = document.createElement('a');
                a.href = `/api/files/${fid}`;
                a.download = '';
                document.body.appendChild(a);
                a.click();
                a.remove();
            }
        };

        bar.style.display = 'flex';
    },

    _hideSelectionBar() {
        const bar = document.getElementById('photos-selection-bar');
        if (bar) bar.style.display = 'none';
    },

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

    _escHtml(s) {
        const d = document.createElement('div');
        d.textContent = s;
        return d.innerHTML;
    },

    _escAttr(s) {
        return String(s || '').replace(/"/g, '&quot;').replace(/</g, '&lt;');
    }
};

window.photosView = photosView;
