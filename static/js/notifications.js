/**
 * OxiCloud – Notification Bell Module
 *
 * Centralised notification system that renders items inside the bell dropdown
 * in the top-bar. Upload progress, quota errors, and general messages all
 * go through this module.
 *
 * Public API (on window.notifications):
 *   addUploadBatch(totalFiles)       → batchId
 *   updateFile(batchId, fileName, pct, status)
 *   finishBatch(batchId, successCount, totalFiles)
 *   addNotification({ icon, iconClass, title, text })
 *   clear()
 */

const notifications = (() => {
    'use strict';

    /* ── state ──────────────────────────────────────────────── */
    let _badgeCount = 0;
    let _batchSeq   = 0;
    const _batches  = {};          // batchId → { el, files:{}, totalFiles }

    /* ── DOM refs (resolved lazily) ─────────────────────────── */
    const $ = (id) => document.getElementById(id);

    /* ── bell toggle ────────────────────────────────────────── */
    function _initBell() {
        const bellBtn  = $('notif-bell-btn');
        const wrapper  = $('notif-wrapper');
        const clearBtn = $('notif-clear-btn');

        if (!bellBtn) return;

        bellBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            const open = wrapper.classList.toggle('open');
            bellBtn.classList.toggle('active', open);

            // Close user-menu if it's open
            const um = $('user-menu-wrapper');
            if (um) um.classList.remove('open');

            if (open) _clearBadge();
        });

        // Close on outside click
        document.addEventListener('click', (e) => {
            if (!wrapper.contains(e.target)) {
                wrapper.classList.remove('open');
                bellBtn.classList.remove('active');
            }
        });

        // Clear all
        if (clearBtn) {
            clearBtn.addEventListener('click', (e) => {
                e.stopPropagation();
                clear();
            });
        }
    }

    /* ── badge helpers ──────────────────────────────────────── */
    function _incrementBadge() {
        _badgeCount++;
        _renderBadge();
        _ringBell();
    }
    function _clearBadge() {
        _badgeCount = 0;
        _renderBadge();
    }
    function _renderBadge() {
        const badge = $('notif-badge');
        if (!badge) return;
        if (_badgeCount > 0) {
            badge.style.display = '';
            badge.textContent = _badgeCount > 99 ? '99+' : _badgeCount;
        } else {
            badge.style.display = 'none';
        }
    }
    function _ringBell() {
        const btn = $('notif-bell-btn');
        if (!btn) return;
        btn.classList.remove('ring');
        // Force reflow so the animation restarts
        void btn.offsetWidth;
        btn.classList.add('ring');
    }

    /* ── empty state ────────────────────────────────────────── */
    function _showEmptyIfNeeded() {
        const body  = $('notif-panel-body');
        const empty = $('notif-empty');
        if (!body || !empty) return;
        // Any real items?
        const hasItems = body.querySelector('.notif-item') !== null;
        empty.style.display = hasItems ? 'none' : '';
    }

    /* ── generic notification ───────────────────────────────── */
    function addNotification({ icon = 'fa-info-circle', iconClass = 'upload', title = '', text = '' }) {
        const body  = $('notif-panel-body');
        if (!body) return;

        const item = document.createElement('div');
        item.className = 'notif-item';
        item.innerHTML = `
            <div class="notif-item-icon ${iconClass}"><i class="fas ${icon}"></i></div>
            <div class="notif-item-body">
                <div class="notif-item-title">${_esc(title)}</div>
                <div class="notif-item-text" title="${_esc(text)}">${_esc(text)}</div>
                <div class="notif-item-time">${_timeAgo()}</div>
            </div>
        `;
        // Insert at top
        body.insertBefore(item, body.firstChild);

        // If panel is closed, bump badge
        const wrapper = $('notif-wrapper');
        if (!wrapper || !wrapper.classList.contains('open')) {
            _incrementBadge();
        }
        _showEmptyIfNeeded();
    }

    /* ── upload batch API ───────────────────────────────────── */

    /**
     * Start tracking a new upload batch.  Returns a batchId string.
     * This also auto-opens the panel so users see progress.
     */
    function addUploadBatch(totalFiles) {
        const batchId = 'batch-' + (++_batchSeq);
        const body = $('notif-panel-body');
        if (!body) return batchId;

        const item = document.createElement('div');
        item.className = 'notif-item';
        item.id = batchId;

        const uploadingText = (window.i18n && window.i18n.t) ? window.i18n.t('upload.uploading') : 'Uploading…';
        item.innerHTML = `
            <div class="notif-item-icon upload"><i class="fas fa-cloud-upload-alt"></i></div>
            <div class="notif-item-body">
                <div class="notif-item-title">${_esc(uploadingText)}</div>
                <div class="notif-upload-files" id="${batchId}-files"></div>
                <div class="notif-upload-progress">
                    <div class="notif-upload-bar"><div class="notif-upload-fill" id="${batchId}-fill"></div></div>
                    <div class="notif-upload-detail">
                        <span class="notif-upload-pct" id="${batchId}-pct">0%</span>
                        <span class="notif-upload-stats" id="${batchId}-stats">0 / ${totalFiles}</span>
                    </div>
                </div>
                <div class="notif-item-time">${_timeAgo()}</div>
            </div>
        `;

        // Insert at top
        body.insertBefore(item, body.firstChild);

        _batches[batchId] = { el: item, files: {}, totalFiles, completed: 0, successCount: 0 };
        _showEmptyIfNeeded();

        // Auto open
        const wrapper = $('notif-wrapper');
        const bellBtn = $('notif-bell-btn');
        if (wrapper && !wrapper.classList.contains('open')) {
            wrapper.classList.add('open');
            if (bellBtn) bellBtn.classList.add('active');
        }

        return batchId;
    }

    /**
     * Add / update a single file row inside a batch.
     * @param {string} batchId
     * @param {string} fileName
     * @param {number} pct       0-100
     * @param {'uploading'|'done'|'error'} status
     */
    function updateFile(batchId, fileName, pct, status) {
        const batch = _batches[batchId];
        if (!batch) return;

        const filesEl = $(batchId + '-files');
        if (!filesEl) return;

        let row = batch.files[fileName];
        if (!row) {
            row = document.createElement('div');
            row.className = 'notif-upload-file-row';
            row.style.cssText = 'display:flex;align-items:center;gap:6px;padding:2px 0;font-size:12px;';
            row.innerHTML = `
                <span class="notif-file-icon" style="width:16px;text-align:center;color:#999;flex-shrink:0;"><i class="fas fa-spinner fa-spin"></i></span>
                <span class="notif-file-name" style="flex:1;min-width:0;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;color:#64748b;" title="${_esc(fileName)}">${_esc(fileName)}</span>
                <span class="notif-file-pct" style="width:34px;text-align:right;color:#94a3b8;flex-shrink:0;">0%</span>
            `;
            filesEl.appendChild(row);
            batch.files[fileName] = row;
        }

        const iconEl = row.querySelector('.notif-file-icon');
        const pctEl  = row.querySelector('.notif-file-pct');

        pctEl.textContent = pct + '%';

        if (status === 'done') {
            iconEl.innerHTML = '<i class="fas fa-check-circle" style="color:#34c759"></i>';
            pctEl.textContent = '100%';
        } else if (status === 'error') {
            iconEl.innerHTML = '<i class="fas fa-exclamation-circle" style="color:#ff3b30"></i>';
            pctEl.textContent = 'ERR';
        }
    }

    /**
     * Mark a file as completed within a batch (updates overall bar).
     */
    function fileCompleted(batchId, success) {
        const batch = _batches[batchId];
        if (!batch) return;
        batch.completed++;
        if (success) batch.successCount++;

        const pctVal = Math.round((batch.completed / batch.totalFiles) * 100);
        const fillEl  = $(batchId + '-fill');
        const pctEl   = $(batchId + '-pct');
        const statsEl = $(batchId + '-stats');

        if (fillEl)  fillEl.style.width = pctVal + '%';
        if (pctEl)   pctEl.textContent = pctVal + '%';
        if (statsEl) statsEl.textContent = `${batch.completed} / ${batch.totalFiles}`;
    }

    /**
     * Finalise a batch – update icon and title.
     */
    function finishBatch(batchId, successCount, totalFiles) {
        const batch = _batches[batchId];
        if (!batch) return;

        const fillEl = $(batchId + '-fill');
        if (fillEl) {
            fillEl.style.width = '100%';
            fillEl.classList.add(successCount === totalFiles ? 'done' : 'error');
        }

        const titleEl = batch.el.querySelector('.notif-item-title');
        const iconEl  = batch.el.querySelector('.notif-item-icon');

        const completeText = (window.i18n && window.i18n.t)
            ? window.i18n.t('upload.complete', { count: successCount, total: totalFiles })
            : `${successCount} / ${totalFiles} uploaded`;
        if (titleEl) titleEl.textContent = completeText;

        if (iconEl) {
            if (successCount === totalFiles) {
                iconEl.className = 'notif-item-icon success';
                iconEl.innerHTML = '<i class="fas fa-check-circle"></i>';
            } else {
                iconEl.className = 'notif-item-icon error';
                iconEl.innerHTML = '<i class="fas fa-exclamation-triangle"></i>';
            }
        }

        // If the panel is closed, bump badge
        const wrapper = $('notif-wrapper');
        if (!wrapper || !wrapper.classList.contains('open')) {
            _incrementBadge();
        }
    }

    /* ── clear all ──────────────────────────────────────────── */
    function clear() {
        const body = $('notif-panel-body');
        if (!body) return;
        // Remove all notif-items
        body.querySelectorAll('.notif-item').forEach(el => el.remove());
        _clearBadge();
        _showEmptyIfNeeded();
    }

    /* ── util ───────────────────────────────────────────────── */
    function _esc(s) {
        const d = document.createElement('div');
        d.textContent = s;
        return d.innerHTML;
    }
    function _timeAgo() {
        const now = new Date();
        const h = String(now.getHours()).padStart(2, '0');
        const m = String(now.getMinutes()).padStart(2, '0');
        return `${h}:${m}`;
    }

    /* ── init on DOM ready ──────────────────────────────────── */
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', _initBell);
    } else {
        _initBell();
    }

    /* ── public API ─────────────────────────────────────────── */
    return {
        addUploadBatch,
        updateFile,
        fileCompleted,
        finishBatch,
        addNotification,
        clear,
    };
})();

window.notifications = notifications;
