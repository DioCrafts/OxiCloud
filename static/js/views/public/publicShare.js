/**
 * publicShare.js — Client-side logic for the public share page (/s/{token}).
 *
 * Fetches share metadata from the API, handles password-protected shares,
 * and renders file download or folder info.
 */
(() => {
    // ── DOM refs ───────────────────────────────────────────────────
    const $loading = document.getElementById('share-loading');
    const $password = document.getElementById('share-password');
    const $expired = document.getElementById('share-expired');
    const $file = document.getElementById('share-file');
    const $folder = document.getElementById('share-folder');

    const $pwForm = document.getElementById('password-form');
    const $pwInput = document.getElementById('password-input');
    const $pwError = document.getElementById('password-error');

    const $fileName = document.getElementById('file-name');
    const $fileMeta = document.getElementById('file-meta');
    const $fileDl = document.getElementById('file-download');
    const $folderName = document.getElementById('folder-name');
    const $expiredMsg = document.getElementById('expired-message');

    // ── Extract token from URL path (/s/{token}) ──────────────────
    const pathParts = window.location.pathname.split('/');
    const tokenIdx = pathParts.indexOf('s');
    const TOKEN = tokenIdx !== -1 ? pathParts[tokenIdx + 1] : null;

    if (!TOKEN) {
        showState('expired');
        $expiredMsg.textContent = 'Invalid share link.';
        return;
    }

    // ── Helpers ────────────────────────────────────────────────────
    function showState(name) {
        [$loading, $password, $expired, $file, $folder].forEach((el) => {
            el.classList.add('hidden');
        });
        var target = {
            loading: $loading,
            password: $password,
            expired: $expired,
            file: $file,
            folder: $folder
        }[name];
        if (target) target.classList.remove('hidden');
    }

    // ── Render share data ─────────────────────────────────────────
    function renderShare(data) {
        if (data.item_type === 'folder') {
            $folderName.textContent = data.item_name || 'Shared Folder';
            showState('folder');
        } else {
            $fileName.textContent = data.item_name || 'Shared File';
            $fileMeta.textContent = data.item_name ? 'Shared file' : '';
            $fileDl.href = `/api/s/${TOKEN}/download`;
            showState('file');
        }
    }

    // ── Fetch share metadata ──────────────────────────────────────
    function fetchShare() {
        fetch(`/api/s/${encodeURIComponent(TOKEN)}`)
            .then((res) => {
                if (res.ok) return res.json();
                if (res.status === 401) {
                    return res.json().then((body) => {
                        if (body?.requiresPassword) {
                            showState('password');
                            return null;
                        }
                        throw new Error('Unauthorized');
                    });
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
                $expiredMsg.textContent = 'This share link is no longer available.';
            });
    }

    // ── Password form ─────────────────────────────────────────────
    $pwForm.addEventListener('submit', (e) => {
        e.preventDefault();
        $pwError.classList.add('hidden');

        var password = $pwInput.value;
        if (!password) return;

        fetch(`/api/s/${encodeURIComponent(TOKEN)}/verify`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ password: password })
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

    // ── Init ──────────────────────────────────────────────────────
    fetchShare();
})();
