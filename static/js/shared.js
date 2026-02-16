/**
 * OxiCloud - Shared Resources Page (/shared)
 * All operations go through the backend API at /api/shares.
 */

// Authentication check
function checkAuthentication() {
    const token = localStorage.getItem('oxicloud_token');
    const tokenExpiry = localStorage.getItem('oxicloud_token_expiry');
    if (!token || !tokenExpiry || new Date(tokenExpiry) < new Date()) {
        window.location.href = '/login';
    }
}

document.addEventListener('DOMContentLoaded', async () => {
    // ── i18n ──
    if (window.i18n && window.i18n.init) {
        await window.i18n.init();
        setTimeout(() => {
            if (window.i18n.translatePage) window.i18n.translatePage();
        }, 500);
    }

    function t(key, fallback) {
        return (window.i18n && window.i18n.t) ? window.i18n.t(key, fallback) : fallback;
    }

    // ── Auth headers helper ──
    function authHeaders(json = false) {
        const h = {};
        const token = localStorage.getItem('oxicloud_token');
        if (token) h['Authorization'] = `Bearer ${token}`;
        if (json) h['Content-Type'] = 'application/json';
        return h;
    }

    // ── Elements ──
    const sharedItemsList = document.getElementById('shared-items-list');
    const emptySharedState = document.getElementById('empty-shared-state');
    const filterType = document.getElementById('filter-type');
    const sortBy = document.getElementById('sort-by');
    const sharedSearch = document.getElementById('shared-search');
    const sharedSearchBtn = document.getElementById('shared-search-btn');
    const goToFilesBtn = document.getElementById('go-to-files');

    // Share dialog elements
    const shareDialog = document.getElementById('share-dialog');
    const shareDialogCloseBtn = shareDialog ? shareDialog.querySelector('.close-dialog-btn') : null;
    const shareDialogIcon = document.getElementById('share-dialog-icon');
    const shareDialogName = document.getElementById('share-dialog-name');
    const shareLinkUrl = document.getElementById('share-link-url');
    const copyLinkBtn = document.getElementById('copy-link-btn');
    const enablePassword = document.getElementById('enable-password');
    const sharePassword = document.getElementById('share-password');
    const generatePasswordBtn = document.getElementById('generate-password');
    const enableExpiration = document.getElementById('enable-expiration');
    const shareExpiration = document.getElementById('share-expiration');
    const permissionRead = document.getElementById('permission-read');
    const permissionWrite = document.getElementById('permission-write');
    const permissionReshare = document.getElementById('permission-reshare');
    const updateShareBtn = document.getElementById('update-share-btn');
    const removeShareBtn = document.getElementById('remove-share-btn');

    // Notification dialog elements
    const notificationDialog = document.getElementById('share-notification-dialog');
    const notificationCloseBtn = notificationDialog ? notificationDialog.querySelector('.close-dialog-btn') : null;
    const notifyDialogIcon = document.getElementById('notify-dialog-icon');
    const notifyDialogName = document.getElementById('notify-dialog-name');
    const notificationEmail = document.getElementById('notification-email');
    const notificationMessage = document.getElementById('notification-message');
    const sendNotificationBtn = document.getElementById('send-notification-btn');

    // Notification banner
    const notificationBanner = document.getElementById('notification-banner');
    const notificationBannerMessage = document.getElementById('notification-message');
    const closeNotificationBtn = document.getElementById('close-notification');

    // ── State ──
    let currentSharedItem = null;
    let allSharedItems = [];
    let filteredItems = [];

    // ── Init ──
    checkAuthentication();
    await loadSharedItems();

    // ── Event listeners ──
    if (filterType) filterType.addEventListener('change', filterAndSortItems);
    if (sortBy) sortBy.addEventListener('change', filterAndSortItems);
    if (sharedSearchBtn) sharedSearchBtn.addEventListener('click', filterAndSortItems);
    if (sharedSearch) sharedSearch.addEventListener('keyup', e => { if (e.key === 'Enter') filterAndSortItems(); });
    if (goToFilesBtn) goToFilesBtn.addEventListener('click', () => window.location.href = '/');

    if (shareDialogCloseBtn) shareDialogCloseBtn.addEventListener('click', closeShareDialog);
    if (copyLinkBtn) copyLinkBtn.addEventListener('click', copyShareLink);
    if (enablePassword) enablePassword.addEventListener('change', () => {
        if (sharePassword) { sharePassword.disabled = !enablePassword.checked; if (enablePassword.checked) sharePassword.focus(); }
    });
    if (generatePasswordBtn) generatePasswordBtn.addEventListener('click', generatePassword);
    if (enableExpiration) enableExpiration.addEventListener('change', () => {
        if (shareExpiration) { shareExpiration.disabled = !enableExpiration.checked; if (enableExpiration.checked) shareExpiration.focus(); }
    });
    if (updateShareBtn) updateShareBtn.addEventListener('click', updateSharedItem);
    if (removeShareBtn) removeShareBtn.addEventListener('click', removeSharedItem);

    if (notificationCloseBtn) notificationCloseBtn.addEventListener('click', closeNotificationDialog);
    if (sendNotificationBtn) sendNotificationBtn.addEventListener('click', sendNotification);
    if (closeNotificationBtn) closeNotificationBtn.addEventListener('click', () => {
        if (notificationBanner) notificationBanner.classList.remove('active');
    });

    // ── Load shares from backend ──
    async function loadSharedItems() {
        try {
            const res = await fetch('/api/shares?page=1&per_page=1000', {
                headers: authHeaders()
            });
            if (res.ok) {
                const data = await res.json();
                allSharedItems = data.items || [];
            } else {
                allSharedItems = [];
            }
        } catch (err) {
            console.error('Error loading shared items:', err);
            allSharedItems = [];
        }
        filterAndSortItems();
    }

    // ── Filter & sort ──
    function filterAndSortItems() {
        const type = filterType ? filterType.value : 'all';
        const sort = sortBy ? sortBy.value : 'date';
        const searchTerm = sharedSearch ? sharedSearch.value.toLowerCase() : '';

        filteredItems = allSharedItems.filter(item => {
            if (type !== 'all' && item.item_type !== type) return false;
            const name = (item.item_name || item.item_id || '').toLowerCase();
            return name.includes(searchTerm);
        });

        filteredItems.sort((a, b) => {
            if (sort === 'name') {
                return (a.item_name || a.item_id || '').localeCompare(b.item_name || b.item_id || '');
            } else if (sort === 'date') {
                return (b.created_at || 0) - (a.created_at || 0);
            } else if (sort === 'expiration') {
                if (!a.expires_at && !b.expires_at) return 0;
                if (!a.expires_at) return 1;
                if (!b.expires_at) return -1;
                return a.expires_at - b.expires_at;
            }
            return 0;
        });

        displaySharedItems();
    }

    // ── Display table ──
    function displaySharedItems() {
        if (!sharedItemsList) return;
        sharedItemsList.innerHTML = '';

        if (filteredItems.length === 0) {
            if (emptySharedState) emptySharedState.style.display = 'flex';
            const listContainer = document.querySelector('.shared-list-container');
            if (listContainer) listContainer.style.display = 'none';
            return;
        }

        if (emptySharedState) emptySharedState.style.display = 'none';
        const listContainer = document.querySelector('.shared-list-container');
        if (listContainer) listContainer.style.display = 'block';

        filteredItems.forEach(item => {
            const row = document.createElement('tr');
            const displayName = item.item_name || item.item_id || 'Unknown';

            // Name
            const nameCell = document.createElement('td');
            nameCell.className = 'shared-item-name';
            nameCell.innerHTML = `<span class="item-icon">${item.item_type === 'file' ? '📄' : '📁'}</span><span>${displayName}</span>`;

            // Type
            const typeCell = document.createElement('td');
            typeCell.textContent = item.item_type === 'file' ? t('shared_typeFile', 'File') : t('shared_typeFolder', 'Folder');

            // Date
            const dateCell = document.createElement('td');
            dateCell.textContent = formatDate(item.created_at);

            // Expiration
            const expirationCell = document.createElement('td');
            expirationCell.textContent = item.expires_at ? formatDate(item.expires_at) : t('shared_noExpiration', 'No expiration');

            // Permissions
            const permissionsCell = document.createElement('td');
            const perms = [];
            if (item.permissions?.read) perms.push(t('share_permissionRead', 'Read'));
            if (item.permissions?.write) perms.push(t('share_permissionWrite', 'Write'));
            if (item.permissions?.reshare) perms.push(t('share_permissionReshare', 'Reshare'));
            permissionsCell.textContent = perms.join(', ') || 'Read';

            // Password
            const passwordCell = document.createElement('td');
            passwordCell.textContent = item.has_password ? t('shared_hasPassword', 'Yes') : t('shared_noPassword', 'No');

            // Actions
            const actionsCell = document.createElement('td');
            actionsCell.className = 'shared-item-actions';

            const editBtn = document.createElement('button');
            editBtn.className = 'action-btn edit-btn';
            editBtn.innerHTML = '<span class="action-icon">✏️</span>';
            editBtn.title = t('shared_editShare', 'Edit Share');
            editBtn.addEventListener('click', () => openShareDialog(item));

            const notifyBtn = document.createElement('button');
            notifyBtn.className = 'action-btn notify-btn';
            notifyBtn.innerHTML = '<span class="action-icon">📧</span>';
            notifyBtn.title = t('shared_notifyShare', 'Notify Someone');
            notifyBtn.addEventListener('click', () => openNotificationDialog(item));

            const cpBtn = document.createElement('button');
            cpBtn.className = 'action-btn copy-btn';
            cpBtn.innerHTML = '<span class="action-icon">📋</span>';
            cpBtn.title = t('shared_copyLink', 'Copy Link');
            cpBtn.addEventListener('click', () => {
                navigator.clipboard.writeText(item.url)
                    .then(() => showNotification(t('shared_linkCopied', 'Link copied!')))
                    .catch(() => showNotification(t('shared_linkCopyFailed', 'Failed to copy link'), 'error'));
            });

            const rmBtn = document.createElement('button');
            rmBtn.className = 'action-btn remove-btn';
            rmBtn.innerHTML = '<span class="action-icon">🗑️</span>';
            rmBtn.title = t('shared_removeShare', 'Remove Share');
            rmBtn.addEventListener('click', () => { currentSharedItem = item; removeSharedItem(); });

            actionsCell.append(editBtn, notifyBtn, cpBtn, rmBtn);
            row.append(nameCell, typeCell, dateCell, expirationCell, permissionsCell, passwordCell, actionsCell);
            sharedItemsList.appendChild(row);
        });
    }

    // ── Share dialog ──
    function openShareDialog(item) {
        currentSharedItem = item;
        const dn = item.item_name || item.item_id || 'Unknown';

        if (shareDialogIcon) shareDialogIcon.textContent = item.item_type === 'file' ? '📄' : '📁';
        if (shareDialogName) shareDialogName.textContent = dn;
        if (shareLinkUrl) shareLinkUrl.value = item.url || '';

        if (permissionRead) permissionRead.checked = item.permissions?.read !== false;
        if (permissionWrite) permissionWrite.checked = !!item.permissions?.write;
        if (permissionReshare) permissionReshare.checked = !!item.permissions?.reshare;

        if (enablePassword) {
            enablePassword.checked = item.has_password;
            if (sharePassword) { sharePassword.disabled = !enablePassword.checked; sharePassword.value = ''; }
        }
        if (enableExpiration) {
            enableExpiration.checked = !!item.expires_at;
            if (shareExpiration) {
                shareExpiration.disabled = !enableExpiration.checked;
                shareExpiration.value = item.expires_at ? new Date(item.expires_at * 1000).toISOString().split('T')[0] : '';
            }
        }

        if (shareDialog) shareDialog.classList.add('active');
    }

    function closeShareDialog() {
        if (shareDialog) shareDialog.classList.remove('active');
        currentSharedItem = null;
    }

    // ── Notification dialog ──
    function openNotificationDialog(item) {
        currentSharedItem = item;
        const dn = item.item_name || item.item_id || 'Unknown';
        if (notifyDialogIcon) notifyDialogIcon.textContent = item.item_type === 'file' ? '📄' : '📁';
        if (notifyDialogName) notifyDialogName.textContent = dn;
        if (notificationEmail) notificationEmail.value = '';
        if (notificationMessage) notificationMessage.value = '';
        if (notificationDialog) notificationDialog.classList.add('active');
    }

    function closeNotificationDialog() {
        if (notificationDialog) notificationDialog.classList.remove('active');
        currentSharedItem = null;
    }

    function copyShareLink() {
        if (!shareLinkUrl) return;
        navigator.clipboard.writeText(shareLinkUrl.value)
            .then(() => showNotification(t('shared_linkCopied', 'Link copied!')))
            .catch(() => showNotification(t('shared_linkCopyFailed', 'Failed to copy link'), 'error'));
    }

    // ── Generate secure password with crypto API ──
    function generatePassword() {
        if (!sharePassword || !enablePassword) return;
        const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*';
        const array = new Uint32Array(16);
        crypto.getRandomValues(array);
        let password = '';
        for (let i = 0; i < 16; i++) {
            password += chars[array[i] % chars.length];
        }
        sharePassword.value = password;
        enablePassword.checked = true;
        sharePassword.disabled = false;
    }

    // ── Update share via API ──
    async function updateSharedItem() {
        if (!currentSharedItem) return;

        const body = {
            permissions: {
                read: permissionRead ? permissionRead.checked : true,
                write: permissionWrite ? permissionWrite.checked : false,
                reshare: permissionReshare ? permissionReshare.checked : false
            },
            password: (enablePassword && enablePassword.checked && sharePassword && sharePassword.value) ? sharePassword.value : null,
            expires_at: (enableExpiration && enableExpiration.checked && shareExpiration && shareExpiration.value)
                ? Math.floor(new Date(shareExpiration.value).getTime() / 1000)
                : null
        };

        try {
            const res = await fetch(`/api/shares/${currentSharedItem.id}`, {
                method: 'PUT',
                headers: authHeaders(true),
                body: JSON.stringify(body)
            });
            if (!res.ok) {
                const err = await res.json().catch(() => ({}));
                throw new Error(err.error || `Server error ${res.status}`);
            }
            showNotification(t('shared_itemUpdated', 'Share settings updated'));
        } catch (err) {
            console.error('Error updating share:', err);
            showNotification(err.message || 'Error updating share', 'error');
        }
        closeShareDialog();
        await loadSharedItems();
    }

    // ── Remove share via API ──
    async function removeSharedItem() {
        if (!currentSharedItem) return;
        try {
            const res = await fetch(`/api/shares/${currentSharedItem.id}`, {
                method: 'DELETE',
                headers: authHeaders()
            });
            if (!res.ok && res.status !== 204) throw new Error(`Server error ${res.status}`);
            showNotification(t('shared_itemRemoved', 'Share removed'));
        } catch (err) {
            console.error('Error removing share:', err);
            showNotification('Error removing share', 'error');
        }
        closeShareDialog();
        await loadSharedItems();
    }

    // ── Send notification (stub) ──
    function sendNotification() {
        if (!currentSharedItem) return;
        const email = notificationEmail ? notificationEmail.value.trim() : '';
        const message = notificationMessage ? notificationMessage.value.trim() : '';

        if (!email || !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
            showNotification(t('shared_invalidEmail', 'Please enter a valid email address'), 'error');
            return;
        }

        if (window.fileSharing && window.fileSharing.sendShareNotification) {
            window.fileSharing.sendShareNotification(currentSharedItem.url, email, message)
                .then(() => { closeNotificationDialog(); showNotification(t('shared_notificationSent', 'Notification sent')); })
                .catch(() => showNotification(t('shared_notificationFailed', 'Failed to send notification'), 'error'));
        }
    }

    // ── Helpers ──
    function showNotification(message, type = 'success') {
        if (notificationBannerMessage && notificationBanner) {
            notificationBannerMessage.textContent = message;
            notificationBanner.className = 'notification-banner active ' + type;
            setTimeout(() => notificationBanner.classList.remove('active'), 5000);
        } else if (window.ui && window.ui.showNotification) {
            window.ui.showNotification(message, type);
        } else {
            alert(message);
        }
    }

    function formatDate(value) {
        return window.formatDateShort ? window.formatDateShort(value) : String(value);
    }
});