/**
 * OxiCloud - Photos Timeline View
 * Dense photo grid grouped by day, with infinite scroll and multi-select.
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
                // Read cursor from header
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

        if (this.items.length === 0 && this.exhausted) {
            this._renderEmpty();
            return;
        }

        if (this.items.length === 0) return;

        // Group by day
        const groups = this._groupByDay(this.items);
        let html = '';

        for (const [dayLabel, files] of groups) {
            html += `<div class="photos-day-header">${this._escHtml(dayLabel)}<span class="photos-day-count">${files.length}</span></div>`;
            html += '<div class="photos-grid">';
            for (const file of files) {
                const isVideo = file.mime_type && file.mime_type.startsWith('video/');
                const selected = this.selected.has(file.id) ? ' selected' : '';
                const thumbUrl = `/api/files/${file.id}/thumbnail/preview`;
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

    /** Group items by day using sort_date */
    _groupByDay(items) {
        const map = new Map();
        for (const item of items) {
            const ts = (item.sort_date || item.created_at) * 1000;
            const d = new Date(ts);
            const key = d.toLocaleDateString(undefined, {
                weekday: 'long', year: 'numeric', month: 'long', day: 'numeric'
            });
            if (!map.has(key)) map.set(key, []);
            map.get(key).push(item);
        }
        return map;
    },

    /** Handle click on photo tile */
    _handleClick(e) {
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
            // Remove from items and re-render
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
