/**
 * OxiCloud - Photos Lightbox
 * Full-screen image/video viewer with prev/next navigation.
 */

const photosLightbox = {
    /** @type {Array} Items array reference */
    items: [],
    /** @type {number} Current index */
    index: -1,
    /** @type {HTMLElement|null} */
    _overlay: null,
    /** @type {string|null} Current blob URL to revoke */
    _blobUrl: null,
    /** @type {Function|null} */
    _keyHandler: null,

    /** Auth headers */
    _headers() {
        return typeof getCsrfHeaders === 'function' ? { ...getCsrfHeaders() } : {};
    },

    /** Open lightbox at given index */
    open(items, index) {
        this.items = items;
        this.index = index;
        this._createOverlay();
        this._show();
        this._bindKeys();
    },

    /** Close lightbox */
    close() {
        if (this._overlay) {
            this._overlay.classList.remove('active');
            setTimeout(() => {
                if (this._overlay) {
                    this._overlay.remove();
                    this._overlay = null;
                }
            }, 200);
        }
        this._revokeBlob();
        this._unbindKeys();
    },

    /** Navigate to previous */
    prev() {
        if (this.index > 0) {
            this.index--;
            this._show();
        }
    },

    /** Navigate to next */
    next() {
        if (this.index < this.items.length - 1) {
            this.index++;
            this._show();
        }
    },

    /** Create the overlay DOM structure */
    _createOverlay() {
        if (this._overlay) this._overlay.remove();

        const el = document.createElement('div');
        el.className = 'photos-lightbox';
        el.innerHTML = `
            <div class="lightbox-info">
                <div class="lightbox-filename"></div>
                <div class="lightbox-meta"></div>
            </div>
            <button class="lightbox-close"><i class="fas fa-times"></i></button>
            <button class="lightbox-nav lightbox-prev"><i class="fas fa-chevron-left"></i></button>
            <div class="lightbox-content"></div>
            <button class="lightbox-nav lightbox-next"><i class="fas fa-chevron-right"></i></button>
            <div class="lightbox-toolbar">
                <button class="lb-download" title="Download"><i class="fas fa-download"></i></button>
                <button class="lb-favorite" title="Favorite"><i class="far fa-star"></i></button>
                <button class="lb-delete" title="Delete"><i class="fas fa-trash"></i></button>
            </div>
            <div class="lightbox-counter"></div>
        `;
        document.body.appendChild(el);
        this._overlay = el;

        // Event listeners
        el.querySelector('.lightbox-close').onclick = () => this.close();
        el.querySelector('.lightbox-prev').onclick = () => this.prev();
        el.querySelector('.lightbox-next').onclick = () => this.next();

        // Click backdrop to close
        el.addEventListener('click', (e) => {
            if (e.target === el || e.target.classList.contains('lightbox-content')) {
                this.close();
            }
        });

        // Toolbar actions
        el.querySelector('.lb-download').onclick = () => this._download();
        el.querySelector('.lb-favorite').onclick = () => this._toggleFavorite();
        el.querySelector('.lb-delete').onclick = () => this._delete();

        // Animate in
        requestAnimationFrame(() => el.classList.add('active'));
    },

    /** Display the current item */
    async _show() {
        if (!this._overlay || this.index < 0) return;

        const item = this.items[this.index];
        const content = this._overlay.querySelector('.lightbox-content');
        const filename = this._overlay.querySelector('.lightbox-filename');
        const meta = this._overlay.querySelector('.lightbox-meta');
        const counter = this._overlay.querySelector('.lightbox-counter');

        filename.textContent = item.name;
        counter.textContent = `${this.index + 1} / ${this.items.length}`;

        // Format date
        const ts = (item.sort_date || item.created_at) * 1000;
        const dateStr = new Date(ts).toLocaleDateString(undefined, {
            year: 'numeric',
            month: 'short',
            day: 'numeric',
            hour: '2-digit',
            minute: '2-digit'
        });
        meta.textContent = `${dateStr} · ${item.size_formatted || ''}`;

        // Update nav button visibility
        this._overlay.querySelector('.lightbox-prev').style.visibility = this.index > 0 ? 'visible' : 'hidden';
        this._overlay.querySelector('.lightbox-next').style.visibility = this.index < this.items.length - 1 ? 'visible' : 'hidden';

        // Load content
        this._revokeBlob();
        content.innerHTML = '<div class="photos-loading"><i class="fas fa-spinner"></i></div>';

        try {
            const isVideo = item.mime_type && item.mime_type.startsWith('video/');
            const res = await fetch(`/api/files/${item.id}`, {
                credentials: 'include',
                headers: this._headers()
            });

            if (!res.ok) throw new Error(`HTTP ${res.status}`);
            const blob = await res.blob();
            this._blobUrl = URL.createObjectURL(blob);

            if (isVideo) {
                content.innerHTML = `<video src="${this._blobUrl}" controls autoplay></video>`;
            } else {
                content.innerHTML = `<img src="${this._blobUrl}" alt="${this._escAttr(item.name)}">`;
            }
        } catch (err) {
            console.error('Lightbox load error:', err);
            content.innerHTML = '<div class="photos-loading">Failed to load</div>';
        }

        // Load EXIF metadata
        this._loadMetadata(item.id, meta, dateStr, item.size_formatted || '');
    },

    /** Load EXIF metadata for info bar */
    async _loadMetadata(fileId, metaEl, dateStr, sizeStr) {
        try {
            const res = await fetch(`/api/files/${fileId}/metadata`, {
                credentials: 'include',
                headers: this._headers()
            });
            if (res.ok) {
                const data = await res.json();
                let parts = [dateStr];
                if (sizeStr) parts.push(sizeStr);
                if (data.camera_make || data.camera_model) {
                    parts.push([data.camera_make, data.camera_model].filter(Boolean).join(' '));
                }
                if (data.width && data.height) {
                    parts.push(`${data.width}×${data.height}`);
                }
                metaEl.textContent = parts.join(' · ');
            }
        } catch (err) {
            // Non-critical, keep existing meta
        }
    },

    /** Download current item */
    _download() {
        const item = this.items[this.index];
        if (!item) return;
        const a = document.createElement('a');
        a.href = `/api/files/${item.id}`;
        a.download = item.name;
        document.body.appendChild(a);
        a.click();
        a.remove();
    },

    /** Toggle favorite on current item */
    async _toggleFavorite() {
        const item = this.items[this.index];
        if (!item || !window.favorites) return;
        try {
            await fetch(`/api/favorites/file/${item.id}`, {
                method: 'POST',
                credentials: 'include',
                headers: this._headers(true)
            });
            const btn = this._overlay.querySelector('.lb-favorite');
            if (btn) {
                btn.classList.toggle('active');
                const icon = btn.querySelector('i');
                if (icon) {
                    icon.className = btn.classList.contains('active') ? 'fas fa-star' : 'far fa-star';
                }
            }
        } catch (err) {
            console.error('Favorite toggle failed:', err);
        }
    },

    /** Delete current item */
    async _delete() {
        const item = this.items[this.index];
        if (!item) return;
        if (!confirm(`Delete ${item.name}?`)) return;

        try {
            await fetch(`/api/files/${item.id}`, {
                method: 'DELETE',
                credentials: 'include',
                headers: this._headers()
            });
            // Remove from photosView items too
            if (window.photosView) {
                window.photosView.items = window.photosView.items.filter((f) => f.id !== item.id);
            }
            this.items.splice(this.index, 1);
            if (this.items.length === 0) {
                this.close();
                if (window.photosView) window.photosView._render();
            } else {
                if (this.index >= this.items.length) this.index = this.items.length - 1;
                this._show();
                if (window.photosView) window.photosView._render();
            }
        } catch (err) {
            console.error('Delete failed:', err);
        }
    },

    /** Keyboard navigation */
    _bindKeys() {
        this._keyHandler = (e) => {
            if (e.key === 'Escape') this.close();
            else if (e.key === 'ArrowLeft') this.prev();
            else if (e.key === 'ArrowRight') this.next();
        };
        document.addEventListener('keydown', this._keyHandler);
    },

    _unbindKeys() {
        if (this._keyHandler) {
            document.removeEventListener('keydown', this._keyHandler);
            this._keyHandler = null;
        }
    },

    _revokeBlob() {
        if (this._blobUrl) {
            URL.revokeObjectURL(this._blobUrl);
            this._blobUrl = null;
        }
    },

    _escAttr(s) {
        return String(s || '')
            .replace(/"/g, '&quot;')
            .replace(/</g, '&lt;');
    }
};

window.photosLightbox = photosLightbox;
