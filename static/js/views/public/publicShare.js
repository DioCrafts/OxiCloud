import { oxiIconsInit } from '../../core/icons';

/**
 * publicShare.js — client-side logic for /s/{token}.
 * Drives the password / file / folder / expired states and renders the
 * folder gallery against /api/s/{token}/contents and friends.
 */

import { uiFileTypes } from '../../app/uiFileTypes.js';

(() => {
    // ── DOM refs ──────────────────────────────────────────────────
    const $loading = document.getElementById('share-loading');
    const $password = document.getElementById('share-password');
    const $expired = document.getElementById('share-expired');
    const $file = document.getElementById('share-file');
    const $folder = document.getElementById('share-folder');

    const $pwForm = document.getElementById('password-form');
    const $pwInput = /** @type {HTMLInputElement} */ (document.getElementById('password-input'));
    const $pwError = document.getElementById('password-error');

    const $fileName = document.getElementById('file-name');
    const $fileMeta = document.getElementById('file-meta');
    const $fileDl = /** @type {HTMLAnchorElement} */ (document.getElementById('file-download'));
    const $expiredMsg = document.getElementById('expired-message');

    // ── Token from URL path (/s/{token}) ──────────────────────────
    const pathParts = window.location.pathname.split('/');
    const tokenIdx = pathParts.indexOf('s');
    const TOKEN = tokenIdx !== -1 ? pathParts[tokenIdx + 1] : null;

    oxiIconsInit();

    if (!TOKEN) {
        showState('expired');
        if ($expiredMsg) $expiredMsg.textContent = 'Invalid share link.';
        return;
    }
    const TOKEN_ENC = encodeURIComponent(TOKEN);

    // ── State management ──────────────────────────────────────────
    const VIEW_KEY = 'oxi.share.view';
    let viewMode = 'grid';
    try {
        viewMode = localStorage.getItem(VIEW_KEY) || 'grid';
    } catch (_) {
        // localStorage unavailable
    }
    let rootDisplayName = 'Shared folder';

    /**
     * @param {'loading'|'password'|'expired'|'file'|'folder'} name
     */
    function showState(name) {
        for (const el of [$loading, $password, $expired, $file, $folder]) {
            if (el) el.classList.add('hidden');
        }
        /** @type {{ loading: HTMLElement|null, password: HTMLElement|null, expired: HTMLElement|null, file: HTMLElement|null, folder: HTMLElement|null }} */
        const map = {
            loading: $loading,
            password: $password,
            expired: $expired,
            file: $file,
            folder: $folder
        };
        const target = map[name];
        if (target) target.classList.remove('hidden');
        document.body.classList.toggle('gallery-mode', name === 'folder');
    }

    // ── Utilities ─────────────────────────────────────────────────
    /**
     * @param {string|null|undefined} s
     * @returns {string}
     */
    function escapeHtml(s) {
        return String(s == null ? '' : s).replace(
            /[&<>"']/g,
            (c) =>
                /** @type {Record<string, string>} */
                ({
                    '&': '&amp;',
                    '<': '&lt;',
                    '>': '&gt;',
                    '"': '&quot;',
                    "'": '&#39;'
                })[c] ?? c
        );
    }
    /**
     * @param {number} bytes
     * @returns {string}
     */
    function formatSize(bytes) {
        if (bytes == null || Number.isNaN(bytes)) return '';
        if (bytes < 1024) return `${bytes} B`;
        const units = ['KB', 'MB', 'GB', 'TB'];
        let v = bytes / 1024;
        let i = 0;
        while (v >= 1024 && i < units.length - 1) {
            v /= 1024;
            i++;
        }
        return `${v < 10 ? v.toFixed(1) : Math.round(v)} ${units[i]}`;
    }
    /**
     * @param {string|null|undefined} mime
     * @returns {'image'|'video'|null}
     */
    function mediaKind(mime) {
        const m = (mime || '').toLowerCase();
        if (m.startsWith('image/')) return 'image';
        if (m.startsWith('video/')) return 'video';
        return null;
    }
    // ── Render share data ─────────────────────────────────────────
    /**
     * @param {any} data
     */
    function renderShare(data) {
        if (data.item_type === 'folder') {
            rootDisplayName = data.item_name || 'Shared folder';
            initFolderGallery();
        } else {
            $fileName.textContent = data.item_name || 'Shared File';
            $fileMeta.textContent = data.item_name ? 'Shared file' : '';
            $fileDl.href = `/api/s/${TOKEN_ENC}/download`;
            showState('file');
        }
    }

    function fetchShare() {
        fetch(`/api/s/${TOKEN_ENC}`)
            .then((res) => {
                if (res.ok) return res.json();
                if (res.status === 401) {
                    return res.json().then(
                        /** @type {(body:any) => null} */ (body) => {
                            if (body?.requiresPassword) {
                                showState('password');
                                return null;
                            }
                            throw new Error('Unauthorized');
                        }
                    );
                }
                if (res.status === 410) {
                    showState('expired');
                    return null;
                }
                throw new Error(`HTTP ${res.status}`);
            })
            .then((data) => {
                if (data) renderShare(data);
            })
            .catch(() => {
                showState('expired');
                if ($expiredMsg) {
                    $expiredMsg.textContent = 'This share link is no longer available.';
                }
            });
    }

    if ($pwForm) {
        $pwForm.addEventListener('submit', (e) => {
            e.preventDefault();
            $pwError.classList.add('hidden');
            const password = $pwInput.value;
            if (!password) return;
            fetch(`/api/s/${TOKEN_ENC}/verify`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ password })
            })
                .then((res) => {
                    if (res.ok) return res.json();
                    if (res.status === 401) {
                        $pwError.textContent = 'Incorrect password. Please try again.';
                        $pwError.classList.remove('hidden');
                        return null;
                    }
                    throw new Error(`HTTP ${res.status}`);
                })
                .then((data) => {
                    if (data) renderShare(data);
                })
                .catch(() => {
                    $pwError.textContent = 'An error occurred. Please try again.';
                    $pwError.classList.remove('hidden');
                });
        });
    }

    // ── Folder gallery ────────────────────────────────────────────

    /** @type {string | null} */
    let currentFolderId = null;
    /** @type {string | null} */
    let currentFolderName = null;

    function initFolderGallery() {
        showState('folder');
        const hashFolderId = parseHashFolderId();
        if (hashFolderId) {
            currentFolderId = hashFolderId;
            currentFolderName = null;
            loadAndRender(hashFolderId);
        } else {
            currentFolderId = null;
            currentFolderName = null;
            loadAndRender(null);
        }
    }

    function parseHashFolderId() {
        const h = window.location.hash;
        if (!h) return null;
        const m = h.match(/[#&]folder=([A-Za-z0-9-]{1,64})/);
        return m ? m[1] : null;
    }

    /**
     * @param {string | null} folderId
     * @returns {string}
     */
    function listingUrl(folderId) {
        return folderId ? `/api/s/${TOKEN_ENC}/contents/${encodeURIComponent(folderId)}` : `/api/s/${TOKEN_ENC}/contents`;
    }
    /**
     * @param {string} fileId
     * @returns {string}
     */
    function fileUrl(fileId) {
        return `/api/s/${TOKEN_ENC}/file/${encodeURIComponent(fileId)}`;
    }
    /**
     * @param {string | null} folderId
     * @returns {string}
     */
    function zipUrl(folderId) {
        return folderId ? `/api/s/${TOKEN_ENC}/zip/${encodeURIComponent(folderId)}` : `/api/s/${TOKEN_ENC}/zip`;
    }

    /** @type {AbortController | null} */
    let currentLoadController = null;
    /**
     * @param {string | null} folderId
     */
    function loadAndRender(folderId) {
        if (currentLoadController) currentLoadController.abort();
        const controller = new AbortController();
        currentLoadController = controller;

        $folder.innerHTML = '<div class="gallery-loading"><div class="spinner"></div></div>';
        fetch(listingUrl(folderId), { signal: controller.signal })
            .then((res) => {
                if (res.ok) return res.json();
                if (res.status === 401) {
                    showState('password');
                    return null;
                }
                if (res.status === 410 || res.status === 404) {
                    showState('expired');
                    return null;
                }
                throw new Error(`HTTP ${res.status}`);
            })
            .then((listing) => {
                if (controller.signal.aborted || !listing) return;
                renderGallery(listing, folderId);
            })
            .catch((err) => {
                if (err.name === 'AbortError') return;
                $folder.innerHTML = '<div class="gallery-error">Failed to load contents. Try again.</div>';
                console.error('share gallery load failed:', err);
            });
    }

    /**
     * @param {any} listing
     * @param {string | null} folderId
     */
    function renderGallery(listing, folderId) {
        const isSubfolder = folderId !== null;
        const title = isSubfolder ? currentFolderName || 'Subfolder' : rootDisplayName;
        const empty = (!listing.folders || listing.folders.length === 0) && (!listing.files || listing.files.length === 0);

        const backHtml = isSubfolder ? '<a class="gallery-back" href="#" data-action="back"><i class="fas fa-arrow-left"></i> Back to share root</a>' : '';

        const headerHtml = `<header class="gallery-header"><h2 class="gallery-title">${escapeHtml(title)}</h2><div class="gallery-actions">${backHtml}<div class="gallery-view-toggle" role="group"><button type="button" data-view="grid" aria-pressed="${viewMode === 'grid'}"><i class="fas fa-th-large"></i></button><button type="button" data-view="list" aria-pressed="${viewMode === 'list'}"><i class="fas fa-bars"></i></button></div><a class="gallery-zip btn-primary" href="${escapeHtml(zipUrl(folderId))}" download><i class="fas fa-file-archive"></i> Download ZIP</a></div></header>`;

        const foldersHtml =
            listing.folders && listing.folders.length > 0
                ? `<h3 class="gallery-section-title">Folders</h3><div class="gallery-folders">${listing.folders.map(folderCardHtml).join('')}</div>`
                : '';

        const filesHtml =
            listing.files && listing.files.length > 0
                ? `<h3 class="gallery-section-title">Files</h3><div class="gallery-files">${listing.files.map((/** @type {any} */ f) => fileCardHtml(f)).join('')}</div>`
                : '';

        const emptyHtml = empty ? '<div class="gallery-empty">This folder is empty.</div>' : '';

        $folder.innerHTML = headerHtml + emptyHtml + foldersHtml + filesHtml;
        document.body.dataset.shareView = viewMode;

        wireGallery();
        wireLazyVideos();
        wireImageRetry();
    }

    /**
     * @param {any} folder
     * @returns {string}
     */
    function folderCardHtml(folder) {
        return `<a class="folder-card" href="#" data-action="open-folder" data-id="${escapeHtml(folder.id)}" data-name="${escapeHtml(folder.name)}"><i class="fas fa-folder folder-icon"></i><div class="card-body"><div class="card-name">${escapeHtml(folder.name)}</div><div class="card-meta">Subfolder</div></div></a>`;
    }

    /**
     * @param {any} file
     * @returns {string}
     */
    function fileCardHtml(file) {
        const url = fileUrl(file.id);
        const kind = mediaKind(file.mime_type);
        let thumbInner;
        if (kind === 'image') {
            thumbInner = `<img src="${escapeHtml(url)}" alt="" decoding="async" />`;
        } else if (kind === 'video') {
            thumbInner = `<video data-lazy-src="${escapeHtml(url)}" preload="metadata" muted playsinline disablepictureinpicture></video><span class="video-marker" aria-hidden="true"></span>`;
        } else {
            thumbInner = `<i class="${escapeHtml(uiFileTypes.getIconClass(file.name))} file-type-icon"></i>`;
        }
        const mediaAttrs = kind ? ` data-mediakind="${kind}" data-src="${escapeHtml(url)}"` : '';
        const dataAttrs = ` data-id="${escapeHtml(file.id)}" data-name="${escapeHtml(file.name)}" data-mime="${escapeHtml(file.mime_type || '')}"${mediaAttrs}`;
        return `<a class="file-card" href="${escapeHtml(url)}"${dataAttrs}><div class="file-thumb">${thumbInner}</div><div class="card-body"><div class="card-name">${escapeHtml(file.name)}</div><div class="card-meta">${escapeHtml(formatSize(file.size))}</div></div></a>`;
    }

    function wireGallery() {
        for (const btn of /** @type {NodeListOf<HTMLButtonElement>} */ ($folder.querySelectorAll('.gallery-view-toggle button'))) {
            btn.addEventListener('click', () => setViewMode(btn.dataset.view));
        }
        const backBtn = $folder.querySelector('[data-action="back"]');
        if (backBtn) {
            backBtn.addEventListener('click', (e) => {
                e.preventDefault();
                navigate(null, rootDisplayName);
            });
        }
        for (const card of /** @type {NodeListOf<HTMLDivElement>} */ ($folder.querySelectorAll('[data-action="open-folder"]'))) {
            card.addEventListener('click', (e) => {
                if (!(e instanceof MouseEvent)) return;
                if (e.metaKey || e.ctrlKey || e.shiftKey || e.button !== 0) return;
                e.preventDefault();
                navigate(card.dataset.id, card.dataset.name);
            });
        }
        const mediaCards = Array.from(/** @type {NodeListOf<HTMLDivElement>} */ ($folder.querySelectorAll('.file-card[data-mediakind]')));
        const items = mediaCards.map((el) => ({
            kind: el.dataset.mediakind,
            src: el.dataset.src,
            name: el.dataset.name
        }));
        mediaCards.forEach((el, i) => {
            el.addEventListener('click', (e) => {
                if (e.metaKey || e.ctrlKey || e.shiftKey || e.button !== 0) return;
                e.preventDefault();
                openLightbox(items, i);
            });
        });
    }

    /**
     * @param {string | null | undefined} v
     */
    function setViewMode(v) {
        const mode = v === 'list' ? 'list' : 'grid';
        viewMode = mode;
        try {
            localStorage.setItem(VIEW_KEY, mode);
        } catch (_) {
            // ignore
        }
        document.body.dataset.shareView = mode;
        for (const b of /** @type {NodeListOf<HTMLButtonElement>} */ ($folder.querySelectorAll('.gallery-view-toggle button'))) {
            b.setAttribute('aria-pressed', String(b.dataset.view === mode));
        }
    }

    /**
     * @param {string | null} folderId
     * @param {string | null | undefined} folderName
     */
    function navigate(folderId, folderName) {
        currentFolderId = folderId;
        currentFolderName = folderName;
        const hash = folderId ? `#folder=${encodeURIComponent(folderId)}` : '';
        history.pushState({ folderId, folderName }, '', window.location.pathname + hash);
        loadAndRender(folderId);
    }

    window.addEventListener('popstate', (e) => {
        if (!$folder || $folder.classList.contains('hidden')) return;
        const state = e.state || {};
        currentFolderId = state.folderId || parseHashFolderId();
        currentFolderName = state.folderName || null;
        loadAndRender(currentFolderId);
    });

    // ── Lazy video posters + image retry ──────────────────────────
    function wireLazyVideos() {
        const lazy = $folder.querySelectorAll('.file-thumb video[data-lazy-src]');
        if (!lazy.length) return;
        const start = (/** @type {HTMLVideoElement} */ v) => {
            v.addEventListener(
                'loadedmetadata',
                () => {
                    const t = Math.min(0.1, (v.duration || 1) * 0.1);
                    try {
                        v.currentTime = t;
                    } catch (_) {
                        // unsupported
                    }
                },
                { once: true }
            );
            v.addEventListener(
                'error',
                () => {
                    if (v.dataset.retried === '1') return;
                    v.dataset.retried = '1';
                    const original = v.dataset.lazySrc;
                    setTimeout(() => {
                        const sep = original.indexOf('?') === -1 ? '?' : '&';
                        v.src = `${original}${sep}_r=${Date.now()}`;
                    }, 250);
                },
                { once: true }
            );
            v.src = v.dataset.lazySrc;
        };
        if ('IntersectionObserver' in window) {
            const obs = new IntersectionObserver(
                (entries) => {
                    for (const e of entries) {
                        if (!e.isIntersecting) continue;
                        const target = /** @type {HTMLVideoElement} */ (e.target);
                        if (target.dataset.lazySrc && !target.src) start(target);
                        obs.unobserve(target);
                    }
                },
                { rootMargin: '300px' }
            );
            for (const v of lazy) obs.observe(v);
        } else {
            for (const v of lazy) start(/** @type {HTMLVideoElement} */ (v));
        }
    }

    function wireImageRetry() {
        for (const img of /** @type {NodeListOf<HTMLImageElement>} */ ($folder.querySelectorAll('.file-thumb img'))) {
            img.addEventListener('error', () => {
                if (img.dataset.retried === '1') return;
                img.dataset.retried = '1';
                const original = img.src;
                setTimeout(() => {
                    const sep = original.indexOf('?') === -1 ? '?' : '&';
                    img.src = `${original}${sep}_r=${Date.now()}`;
                }, 250);
            });
        }
    }

    // ── Lightbox ──────────────────────────────────────────────────
    /**
     * @typedef {{ root: HTMLElement, title: Element|null, download: HTMLAnchorElement|null, close: HTMLButtonElement|null, stage: Element|null, content: Element|null, prev: HTMLButtonElement|null, next: HTMLButtonElement|null }} LightboxRefs
     */
    /** @type {LightboxRefs | null} */
    let lb = null;
    /** @type {Array<{kind: string|undefined, src: string|undefined, name: string|undefined}>} */
    let lbItems = [];
    let lbIndex = -1;

    function ensureLightbox() {
        if (lb) return lb;
        const root = document.createElement('div');
        root.className = 'lightbox hidden';
        root.id = 'share-lightbox';
        root.setAttribute('role', 'dialog');
        root.innerHTML =
            '<div class="lb-bar">' +
            '<span class="lb-title"></span>' +
            '<a class="lb-download" href="#" download title="Download"><i class="fas fa-download"></i></a>' +
            '<button class="lb-close" type="button" title="Close"><i class="fas fa-times"></i></button>' +
            '</div>' +
            '<div class="lb-stage">' +
            '<button class="lb-nav lb-prev" type="button" aria-label="Previous"><i class="fas fa-chevron-left"></i></button>' +
            '<div class="lb-content"></div>' +
            '<button class="lb-nav lb-next" type="button" aria-label="Next"><i class="fas fa-chevron-right"></i></button>' +
            '</div>';
        document.body.appendChild(root);
        lb = {
            root,
            title: root.querySelector('.lb-title'),
            download: root.querySelector('.lb-download'),
            close: root.querySelector('.lb-close'),
            stage: root.querySelector('.lb-stage'),
            content: root.querySelector('.lb-content'),
            prev: root.querySelector('.lb-prev'),
            next: root.querySelector('.lb-next')
        };
        lb.close.addEventListener('click', closeLightbox);
        lb.prev.addEventListener('click', () => stepLightbox(-1));
        lb.next.addEventListener('click', () => stepLightbox(1));
        lb.root.addEventListener('click', (e) => {
            if (e.target === lb.root) closeLightbox();
        });
        document.addEventListener('keydown', (e) => {
            if (lb.root.classList.contains('hidden')) return;
            if (e.key === 'Escape') {
                e.preventDefault();
                closeLightbox();
            } else if (e.key === 'ArrowLeft') {
                e.preventDefault();
                stepLightbox(-1);
            } else if (e.key === 'ArrowRight') {
                e.preventDefault();
                stepLightbox(1);
            }
        });
        return lb;
    }

    /**
     * @param {Array<{kind: string|undefined, src: string|undefined, name: string|undefined}>} items
     * @param {number} index
     */
    function openLightbox(items, index) {
        ensureLightbox();
        lbItems = items;
        showLightboxItem(index);
    }
    /**
     * @param {number} i
     */
    function showLightboxItem(i) {
        if (i < 0 || i >= lbItems.length) return;
        lbIndex = i;
        const item = lbItems[i];
        lb.content.innerHTML = '';
        if (item.kind === 'image') {
            const img = document.createElement('img');
            img.src = item.src;
            img.alt = item.name || '';
            lb.content.appendChild(img);
        } else {
            const v = document.createElement('video');
            v.src = item.src;
            v.controls = true;
            v.autoplay = true;
            v.preload = 'metadata';
            lb.content.appendChild(v);
        }
        lb.title.textContent = item.name || '';
        lb.download.href = item.src;
        if (item.name) lb.download.setAttribute('download', item.name);
        lb.prev.disabled = i === 0;
        lb.next.disabled = i === lbItems.length - 1;
        lb.root.classList.remove('hidden');
        lb.root.setAttribute('aria-hidden', 'false');
    }
    /**
     * @param {number} delta
     */
    function stepLightbox(delta) {
        const next = lbIndex + delta;
        if (next >= 0 && next < lbItems.length) showLightboxItem(next);
    }
    function closeLightbox() {
        if (!lb) return;
        lb.root.classList.add('hidden');
        lb.root.setAttribute('aria-hidden', 'true');
        lb.content.innerHTML = '';
        lbIndex = -1;
    }

    // ── Init ──────────────────────────────────────────────────────
    fetchShare();
})();
