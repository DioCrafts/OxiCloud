/**
 * OxiCloud - Shared View Component
 * In-app shared files view. All operations go through the backend API.
 */

const sharedView = {
    // State
    items: [],
    filteredItems: [],
    currentItem: null,

    /** Auth header helper */
    _headers(json = false) {
        const h = {};
        const token = localStorage.getItem('oxicloud_token');
        if (token) h['Authorization'] = `Bearer ${token}`;
        if (json) h['Content-Type'] = 'application/json';
        return h;
    },

    init() {
        console.log('Initializing shared view component (API-backed)');
        this.loadItems();
    },

    show() {
        this.displayUI();
        this.attachEventListeners();
        this.loadItems().then(() => this.filterAndSortItems());
    },

    hide() {
        const c = document.getElementById('shared-container');
        if (c) c.style.display = 'none';
    },

    // Load shared items from backend API
    async loadItems() {
        try {
            const res = await fetch('/api/shares?page=1&per_page=1000', {
                headers: this._headers()
            });
            if (res.ok) {
                const data = await res.json();
                this.items = data.items || [];
            } else {
                this.items = [];
            }
        } catch (err) {
            console.error('Error loading shared items:', err);
            this.items = [];
        }
        this.filteredItems = [...this.items];
    },

    // Create and display the shared view UI
    displayUI() {
        const contentArea = document.querySelector('.content-area');

        let container = document.getElementById('shared-container');
        if (!container) {
            container = document.createElement('div');
            container.id = 'shared-container';
            container.className = 'shared-view-container';
            if (contentArea) contentArea.appendChild(container);
        }

        container.style.display = 'block';
        container.innerHTML = `
            <div class="shared-header">
                <h2 data-i18n="nav.shared">Shared Files</h2>
                <div class="shared-filters">
                    <select id="filter-type" class="shared-filter-select">
                        <option value="all" data-i18n="shared_allTypes">All types</option>
                        <option value="file" data-i18n="shared_files">Files</option>
                        <option value="folder" data-i18n="shared_folders">Folders</option>
                    </select>
                    <select id="sort-by" class="shared-filter-select">
                        <option value="date" data-i18n="shared_sortDate">Sort by date</option>
                        <option value="name" data-i18n="shared_sortName">Sort by name</option>
                        <option value="expiration" data-i18n="shared_sortExpiration">Sort by expiration</option>
                    </select>
                    <div class="shared-search-box">
                        <input type="text" id="shared-search-filter" data-i18n-placeholder="shared_searchPlaceholder" placeholder="Search...">
                        <button id="shared-search-filter-btn" class="search-btn">🔍</button>
                    </div>
                </div>
            </div>

            <div id="empty-shared-state" class="empty-state" style="display:none;">
                <div class="empty-state-icon">📤</div>
                <h3 data-i18n="shared_emptyTitle">No shared items</h3>
                <p data-i18n="shared_emptyDesc">Items you share will appear here</p>
                <button id="empty-go-to-files" class="button primary" data-i18n="shared_goToFiles">Go to Files</button>
            </div>

            <div class="shared-list-container" style="display:none;">
                <table class="shared-table">
                    <thead>
                        <tr>
                            <th data-i18n="shared_columnName">Name</th>
                            <th data-i18n="shared_columnType">Type</th>
                            <th data-i18n="shared_columnDate">Date</th>
                            <th data-i18n="shared_columnExpiration">Expiration</th>
                            <th data-i18n="shared_columnPermissions">Permissions</th>
                            <th data-i18n="shared_columnPassword">Password</th>
                            <th data-i18n="shared_columnActions">Actions</th>
                        </tr>
                    </thead>
                    <tbody id="shared-items-list"></tbody>
                </table>
            </div>

            <!-- Share Edit Dialog -->
            <div id="share-dialog" class="shared-dialog">
                <div class="shared-dialog-content">
                    <div class="shared-dialog-header">
                        <span id="share-dialog-icon">📄</span>
                        <span id="share-dialog-name">Item</span>
                        <button class="close-dialog-btn">&times;</button>
                    </div>
                    <div class="share-link-section">
                        <label data-i18n="share.linkLabel">Share Link:</label>
                        <div class="share-link-input">
                            <input type="text" id="share-link-url" readonly>
                            <button id="copy-link-btn" class="button" data-i18n="share.copyLink">Copy</button>
                        </div>
                    </div>
                    <div class="share-permissions-section">
                        <h4 data-i18n="share.permissions">Permissions</h4>
                        <label><input type="checkbox" id="permission-read" checked> <span data-i18n="share.permissionRead">Read</span></label>
                        <label><input type="checkbox" id="permission-write"> <span data-i18n="share.permissionWrite">Write</span></label>
                        <label><input type="checkbox" id="permission-reshare"> <span data-i18n="share.permissionReshare">Reshare</span></label>
                    </div>
                    <div class="share-password-section">
                        <label><input type="checkbox" id="enable-password"> <span data-i18n="share.enablePassword">Password protection</span></label>
                        <div class="password-input-group">
                            <input type="text" id="share-password" disabled placeholder="Enter password">
                            <button id="generate-password" class="button small" data-i18n="share.generatePassword">Generate</button>
                        </div>
                    </div>
                    <div class="share-expiration-section">
                        <label><input type="checkbox" id="enable-expiration"> <span data-i18n="share.enableExpiration">Set expiration</span></label>
                        <input type="date" id="share-expiration" disabled>
                    </div>
                    <div class="share-actions">
                        <button id="update-share-btn" class="button primary" data-i18n="share.update">Update</button>
                        <button id="remove-share-btn" class="button danger" data-i18n="share.remove">Remove Share</button>
                    </div>
                </div>
            </div>

            <!-- Notification Dialog -->
            <div id="share-notification-dialog" class="shared-dialog">
                <div class="shared-dialog-content">
                    <div class="shared-dialog-header">
                        <span id="notify-dialog-icon">📧</span>
                        <span id="notify-dialog-name">Item</span>
                        <button class="close-dialog-btn">&times;</button>
                    </div>
                    <div class="notification-form">
                        <div class="form-group">
                            <label data-i18n="share.notifyEmail">Email:</label>
                            <input type="email" id="notification-email" placeholder="recipient@example.com">
                        </div>
                        <div class="form-group">
                            <label data-i18n="share.notifyMessage">Message (optional):</label>
                            <textarea id="notification-message" rows="3"></textarea>
                        </div>
                    </div>
                    <div class="notification-actions">
                        <button id="send-notification-btn" class="button primary" data-i18n="share.notifySend">Send Notification</button>
                    </div>
                </div>
            </div>
        `;

        // Hide other UI elements
        const filesGrid = document.getElementById('files-grid');
        const filesListView = document.getElementById('files-list-view');
        if (filesGrid) filesGrid.style.display = 'none';
        if (filesListView) filesListView.style.display = 'none';

        if (window.i18n && window.i18n.translatePage) {
            window.i18n.translatePage();
        }
    },

    // Attach event listeners
    attachEventListeners() {
        const filterType = document.getElementById('filter-type');
        const sortBy = document.getElementById('sort-by');
        const searchFilter = document.getElementById('shared-search-filter');
        const searchBtn = document.getElementById('shared-search-filter-btn');
        const emptyGoToFiles = document.getElementById('empty-go-to-files');

        if (filterType) filterType.addEventListener('change', () => this.filterAndSortItems());
        if (sortBy) sortBy.addEventListener('change', () => this.filterAndSortItems());
        if (searchFilter) searchFilter.addEventListener('keyup', e => { if (e.key === 'Enter') this.filterAndSortItems(); });
        if (searchBtn) searchBtn.addEventListener('click', () => this.filterAndSortItems());
        if (emptyGoToFiles) emptyGoToFiles.addEventListener('click', () => window.switchToFilesView());

        // Share dialog
        const shareDialog = document.getElementById('share-dialog');
        if (shareDialog) {
            const closeBtn = shareDialog.querySelector('.close-dialog-btn');
            if (closeBtn) closeBtn.addEventListener('click', () => this.closeShareDialog());
            const copyLinkBtn = document.getElementById('copy-link-btn');
            if (copyLinkBtn) copyLinkBtn.addEventListener('click', () => this.copyShareLink());
            const enablePw = document.getElementById('enable-password');
            const pwField = document.getElementById('share-password');
            if (enablePw) enablePw.addEventListener('change', () => {
                if (pwField) { pwField.disabled = !enablePw.checked; if (enablePw.checked) pwField.focus(); }
            });
            const genPwBtn = document.getElementById('generate-password');
            if (genPwBtn) genPwBtn.addEventListener('click', () => this.generatePassword());
            const enableExp = document.getElementById('enable-expiration');
            const expField = document.getElementById('share-expiration');
            if (enableExp) enableExp.addEventListener('change', () => {
                if (expField) { expField.disabled = !enableExp.checked; if (enableExp.checked) expField.focus(); }
            });
            const updateBtn = document.getElementById('update-share-btn');
            if (updateBtn) updateBtn.addEventListener('click', () => this.updateSharedItem());
            const removeBtn = document.getElementById('remove-share-btn');
            if (removeBtn) removeBtn.addEventListener('click', () => this.removeSharedItem());
        }

        // Notification dialog
        const notifDialog = document.getElementById('share-notification-dialog');
        if (notifDialog) {
            const closeBtn = notifDialog.querySelector('.close-dialog-btn');
            if (closeBtn) closeBtn.addEventListener('click', () => this.closeNotificationDialog());
            const sendBtn = document.getElementById('send-notification-btn');
            if (sendBtn) sendBtn.addEventListener('click', () => this.sendNotification());
        }
    },

    // Filter and sort items
    filterAndSortItems() {
        const filterType = document.getElementById('filter-type');
        const sortBy = document.getElementById('sort-by');
        const searchFilter = document.getElementById('shared-search-filter');

        const type = filterType ? filterType.value : 'all';
        const sort = sortBy ? sortBy.value : 'date';
        const searchTerm = searchFilter ? searchFilter.value.toLowerCase() : '';

        this.filteredItems = this.items.filter(item => {
            if (type !== 'all' && item.item_type !== type) return false;
            const name = (item.item_name || item.item_id || '').toLowerCase();
            return name.includes(searchTerm);
        });

        this.filteredItems.sort((a, b) => {
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

        this.displaySharedItems();
    },

    // Display items in the table
    displaySharedItems() {
        const sharedItemsList = document.getElementById('shared-items-list');
        const emptyState = document.getElementById('empty-shared-state');
        const listContainer = document.querySelector('.shared-list-container');

        if (!sharedItemsList || !emptyState || !listContainer) return;
        sharedItemsList.innerHTML = '';

        if (this.filteredItems.length === 0) {
            emptyState.style.display = 'flex';
            listContainer.style.display = 'none';
            return;
        }

        emptyState.style.display = 'none';
        listContainer.style.display = 'block';

        this.filteredItems.forEach(item => {
            const row = document.createElement('tr');
            const displayName = item.item_name || item.item_id || 'Unknown';

            const nameCell = document.createElement('td');
            nameCell.className = 'shared-item-name';
            const iconSpan = document.createElement('span');
            iconSpan.className = 'item-icon';
            iconSpan.textContent = item.item_type === 'file' ? '📄' : '📁';
            const nameSpan = document.createElement('span');
            nameSpan.textContent = displayName;
            nameCell.appendChild(iconSpan);
            nameCell.appendChild(nameSpan);

            const typeCell = document.createElement('td');
            typeCell.textContent = item.item_type === 'file' ? this.translate('shared_typeFile', 'File') : this.translate('shared_typeFolder', 'Folder');

            const dateCell = document.createElement('td');
            dateCell.textContent = this.formatDate(item.created_at);

            const expCell = document.createElement('td');
            expCell.textContent = item.expires_at ? this.formatDate(item.expires_at) : this.translate('shared_noExpiration', 'No expiration');

            const permCell = document.createElement('td');
            const perms = [];
            if (item.permissions?.read) perms.push(this.translate('share_permissionRead', 'Read'));
            if (item.permissions?.write) perms.push(this.translate('share_permissionWrite', 'Write'));
            if (item.permissions?.reshare) perms.push(this.translate('share_permissionReshare', 'Reshare'));
            permCell.textContent = perms.join(', ') || 'Read';

            const pwCell = document.createElement('td');
            pwCell.textContent = item.has_password ? this.translate('shared_hasPassword', 'Yes') : this.translate('shared_noPassword', 'No');

            const actionsCell = document.createElement('td');
            actionsCell.className = 'shared-item-actions';

            const editBtn = document.createElement('button');
            editBtn.className = 'action-btn edit-btn';
            editBtn.innerHTML = '<span class="action-icon">✏️</span>';
            editBtn.title = this.translate('shared_editShare', 'Edit Share');
            editBtn.addEventListener('click', () => this.openShareDialog(item));

            const notifyBtn = document.createElement('button');
            notifyBtn.className = 'action-btn notify-btn';
            notifyBtn.innerHTML = '<span class="action-icon">📧</span>';
            notifyBtn.title = this.translate('shared_notifyShare', 'Notify Someone');
            notifyBtn.addEventListener('click', () => this.openNotificationDialog(item));

            const copyBtn = document.createElement('button');
            copyBtn.className = 'action-btn copy-btn';
            copyBtn.innerHTML = '<span class="action-icon">📋</span>';
            copyBtn.title = this.translate('shared_copyLink', 'Copy Link');
            copyBtn.addEventListener('click', () => {
                navigator.clipboard.writeText(item.url)
                    .then(() => this.showNotification(this.translate('shared_linkCopied', 'Link copied!')))
                    .catch(() => this.showNotification(this.translate('shared_linkCopyFailed', 'Failed to copy link'), 'error'));
            });

            const rmBtn = document.createElement('button');
            rmBtn.className = 'action-btn remove-btn';
            rmBtn.innerHTML = '<span class="action-icon">🗑️</span>';
            rmBtn.title = this.translate('shared_removeShare', 'Remove Share');
            rmBtn.addEventListener('click', () => { this.currentItem = item; this.removeSharedItem(); });

            actionsCell.append(editBtn, notifyBtn, copyBtn, rmBtn);
            row.append(nameCell, typeCell, dateCell, expCell, permCell, pwCell, actionsCell);
            sharedItemsList.appendChild(row);
        });
    },

    // Open share dialog
    openShareDialog(item) {
        this.currentItem = item;
        const shareDialog = document.getElementById('share-dialog');
        const dn = item.item_name || item.item_id || 'Unknown';

        const iconEl = document.getElementById('share-dialog-icon');
        const nameEl = document.getElementById('share-dialog-name');
        const urlEl = document.getElementById('share-link-url');
        const enablePw = document.getElementById('enable-password');
        const pwField = document.getElementById('share-password');
        const enableExp = document.getElementById('enable-expiration');
        const expField = document.getElementById('share-expiration');
        const permRead = document.getElementById('permission-read');
        const permWrite = document.getElementById('permission-write');
        const permReshare = document.getElementById('permission-reshare');

        if (!shareDialog) return;
        if (iconEl) iconEl.textContent = item.item_type === 'file' ? '📄' : '📁';
        if (nameEl) nameEl.textContent = dn;
        if (urlEl) urlEl.value = item.url || '';

        if (permRead) permRead.checked = item.permissions?.read !== false;
        if (permWrite) permWrite.checked = !!item.permissions?.write;
        if (permReshare) permReshare.checked = !!item.permissions?.reshare;

        if (enablePw) {
            enablePw.checked = item.has_password;
            if (pwField) { pwField.disabled = !enablePw.checked; pwField.value = ''; }
        }
        if (enableExp) {
            enableExp.checked = !!item.expires_at;
            if (expField) {
                expField.disabled = !enableExp.checked;
                expField.value = item.expires_at ? new Date(item.expires_at * 1000).toISOString().split('T')[0] : '';
            }
        }

        shareDialog.classList.add('active');
    },

    closeShareDialog() {
        const d = document.getElementById('share-dialog');
        if (d) d.classList.remove('active');
        this.currentItem = null;
    },

    openNotificationDialog(item) {
        this.currentItem = item;
        const dn = item.item_name || item.item_id || 'Unknown';
        const d = document.getElementById('share-notification-dialog');
        const iconEl = document.getElementById('notify-dialog-icon');
        const nameEl = document.getElementById('notify-dialog-name');
        const emailEl = document.getElementById('notification-email');
        const msgEl = document.getElementById('notification-message');

        if (!d) return;
        if (iconEl) iconEl.textContent = item.item_type === 'file' ? '📄' : '📁';
        if (nameEl) nameEl.textContent = dn;
        if (emailEl) emailEl.value = '';
        if (msgEl) msgEl.value = '';
        d.classList.add('active');
    },

    closeNotificationDialog() {
        const d = document.getElementById('share-notification-dialog');
        if (d) d.classList.remove('active');
        this.currentItem = null;
    },

    copyShareLink() {
        const el = document.getElementById('share-link-url');
        if (!el) return;
        navigator.clipboard.writeText(el.value)
            .then(() => this.showNotification(this.translate('shared_linkCopied', 'Link copied!')))
            .catch(() => this.showNotification(this.translate('shared_linkCopyFailed', 'Failed to copy link'), 'error'));
    },

    // Generate secure password with crypto API
    generatePassword() {
        const pwField = document.getElementById('share-password');
        const enablePw = document.getElementById('enable-password');
        if (!pwField || !enablePw) return;

        const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*';
        const array = new Uint32Array(16);
        crypto.getRandomValues(array);
        let password = '';
        for (let i = 0; i < 16; i++) {
            password += chars[array[i] % chars.length];
        }
        pwField.value = password;
        enablePw.checked = true;
        pwField.disabled = false;
    },

    // Update share via API
    async updateSharedItem() {
        if (!this.currentItem) return;

        const permRead = document.getElementById('permission-read');
        const permWrite = document.getElementById('permission-write');
        const permReshare = document.getElementById('permission-reshare');
        const enablePw = document.getElementById('enable-password');
        const pwField = document.getElementById('share-password');
        const enableExp = document.getElementById('enable-expiration');
        const expField = document.getElementById('share-expiration');

        const body = {
            permissions: {
                read: permRead ? permRead.checked : true,
                write: permWrite ? permWrite.checked : false,
                reshare: permReshare ? permReshare.checked : false
            },
            password: (enablePw && enablePw.checked && pwField && pwField.value) ? pwField.value : null,
            expires_at: (enableExp && enableExp.checked && expField && expField.value)
                ? Math.floor(new Date(expField.value).getTime() / 1000)
                : null
        };

        try {
            const res = await fetch(`/api/shares/${this.currentItem.id}`, {
                method: 'PUT',
                headers: this._headers(true),
                body: JSON.stringify(body)
            });
            if (!res.ok) {
                const err = await res.json().catch(() => ({}));
                throw new Error(err.error || `Server error ${res.status}`);
            }
            this.showNotification(this.translate('shared_itemUpdated', 'Share settings updated'));
        } catch (err) {
            console.error('Error updating share:', err);
            this.showNotification(err.message || 'Error updating share', 'error');
        }

        this.closeShareDialog();
        await this.loadItems();
        this.filterAndSortItems();
    },

    // Remove share via API
    async removeSharedItem() {
        if (!this.currentItem) return;

        try {
            const res = await fetch(`/api/shares/${this.currentItem.id}`, {
                method: 'DELETE',
                headers: this._headers()
            });
            if (!res.ok && res.status !== 204) throw new Error(`Server error ${res.status}`);
            this.showNotification(this.translate('shared_itemRemoved', 'Share removed'));
        } catch (err) {
            console.error('Error removing share:', err);
            this.showNotification('Error removing share', 'error');
        }

        this.closeShareDialog();
        await this.loadItems();
        this.filterAndSortItems();
    },

    // Send notification (stub)
    sendNotification() {
        if (!this.currentItem) return;
        const emailEl = document.getElementById('notification-email');
        const msgEl = document.getElementById('notification-message');
        const email = emailEl ? emailEl.value.trim() : '';
        const message = msgEl ? msgEl.value.trim() : '';

        if (!email || !this.validateEmail(email)) {
            this.showNotification(this.translate('shared_invalidEmail', 'Please enter a valid email address'), 'error');
            return;
        }

        if (window.fileSharing && window.fileSharing.sendShareNotification) {
            window.fileSharing.sendShareNotification(this.currentItem.url, email, message)
                .then(() => { this.closeNotificationDialog(); this.showNotification(this.translate('shared_notificationSent', 'Notification sent')); })
                .catch(() => this.showNotification(this.translate('shared_notificationFailed', 'Failed to send notification'), 'error'));
        }
    },

    showNotification(message, type = 'success') {
        if (window.ui && window.ui.showNotification) {
            window.ui.showNotification(message, type);
        } else {
            alert(message);
        }
    },

    validateEmail(email) {
        return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email);
    },

    formatDate(value) {
        return window.formatDateShort ? window.formatDateShort(value) : String(value);
    },

    translate(key, defaultText) {
        if (window.i18n && window.i18n.t) return window.i18n.t(key, defaultText);
        return defaultText;
    }
};

window.sharedView = sharedView;
