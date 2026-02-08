/**
 * OxiCloud - Shared View Component
 * Encapsulates shared files view functionality
 */

const sharedView = {
    // State
    items: [],
    filteredItems: [],
    currentItem: null,
    
    // Initialize the shared view
    init() {
        console.log('Initializing shared view component');
        this.loadItems();
    },
    
    // Show the shared view UI
    show() {
        console.log('Showing shared view component');
        this.displayUI();
        this.attachEventListeners();
        this.filterAndSortItems();
    },
    
    // Hide the shared view UI
    hide() {
        const sharedContainer = document.getElementById('shared-container');
        if (sharedContainer) {
            sharedContainer.style.display = 'none';
        }
    },
    
    // Load shared items from local storage
    loadItems() {
        try {
            this.items = JSON.parse(localStorage.getItem('oxicloud_shared_links') || '[]');
            this.filteredItems = [...this.items];
        } catch (error) {
            console.error('Error loading shared items:', error);
            this.items = [];
            this.filteredItems = [];
        }
    },
    
    // Create and display the shared view UI
    displayUI() {
        const contentArea = document.querySelector('.content-area');
        
        // Create container if it doesn't exist
        let sharedContainer = document.getElementById('shared-container');
        if (!sharedContainer) {
            sharedContainer = document.createElement('div');
            sharedContainer.id = 'shared-container';
            contentArea.appendChild(sharedContainer);
        }
        
        // Show container
        sharedContainer.style.display = 'block';
        
        // Update container
        sharedContainer.innerHTML = `
            <div class="shared-filters">
                <div class="filter-group">
                    <label for="filter-type" data-i18n="shared.filterType">Type:</label>
                    <select id="filter-type">
                        <option value="all" data-i18n="shared.filterAll">All</option>
                        <option value="file" data-i18n="shared.filterFiles">Files</option>
                        <option value="folder" data-i18n="shared.filterFolders">Folders</option>
                    </select>
                </div>
                <div class="filter-group">
                    <label for="sort-by" data-i18n="shared.sortBy">Sort by:</label>
                    <select id="sort-by">
                        <option value="name" data-i18n="shared.sortByName">Name</option>
                        <option value="date" data-i18n="shared.sortByDate">Date shared</option>
                        <option value="expiration" data-i18n="shared.sortByExpiration">Expiration</option>
                    </select>
                </div>
                <div class="search-box">
                    <input type="text" id="shared-search-filter" placeholder="Search shared items...">
                    <button id="shared-search-filter-btn" class="btn btn-primary" data-i18n="shared.search">Search</button>
                </div>
            </div>

            <div class="shared-list-container">
                <table class="shared-list">
                    <thead>
                        <tr>
                            <th data-i18n="shared.colName">Name</th>
                            <th data-i18n="shared.colType">Type</th>
                            <th data-i18n="shared.colDateShared">Date Shared</th>
                            <th data-i18n="shared.colExpiration">Expiration</th>
                            <th data-i18n="shared.colPermissions">Permissions</th>
                            <th data-i18n="shared.colPassword">Password</th>
                            <th data-i18n="shared.colActions">Actions</th>
                        </tr>
                    </thead>
                    <tbody id="shared-items-list">
                        <!-- Shared items will be loaded here dynamically -->
                    </tbody>
                </table>
            </div>

            <div id="empty-shared-state" class="empty-state">
                <div class="empty-state-icon">📂</div>
                <h3 data-i18n="shared.emptyStateTitle">No shared resources yet</h3>
                <p data-i18n="shared.emptyStateDesc">When you share files or folders, they will appear here</p>
                <button id="empty-go-to-files" class="button primary" data-i18n="shared.goToFiles">Go to Files</button>
            </div>

            <!-- Share Link Dialog (for editing existing shares) -->
            <div id="share-dialog" class="dialog">
                <div class="dialog-content">
                    <div class="dialog-header">
                        <h3 data-i18n="share.dialogTitle">Share Link</h3>
                        <button class="close-dialog-btn">&times;</button>
                    </div>
                    <div class="dialog-body">
                        <div class="share-item-info">
                            <span id="share-dialog-icon" class="item-icon">📄</span>
                            <span id="share-dialog-name" class="item-name">filename.ext</span>
                        </div>

                        <div class="share-link-section">
                            <label for="share-link-url" data-i18n="share.linkLabel">Share Link:</label>
                            <div class="share-link-container">
                                <input type="text" id="share-link-url" readonly>
                                <button id="copy-link-btn" data-i18n="share.copyLink">Copy</button>
                            </div>
                        </div>

                        <div class="share-settings">
                            <div class="share-setting">
                                <label data-i18n="share.permissions">Permissions:</label>
                                <div class="permissions-options">
                                    <label>
                                        <input type="checkbox" id="permission-read" checked>
                                        <span data-i18n="share.permissionRead">Read</span>
                                    </label>
                                    <label>
                                        <input type="checkbox" id="permission-write">
                                        <span data-i18n="share.permissionWrite">Write</span>
                                    </label>
                                    <label>
                                        <input type="checkbox" id="permission-reshare">
                                        <span data-i18n="share.permissionReshare">Reshare</span>
                                    </label>
                                </div>
                            </div>

                            <div class="share-setting">
                                <label for="share-password" data-i18n="share.password">Password Protection:</label>
                                <div class="password-setting">
                                    <input type="checkbox" id="enable-password">
                                    <input type="password" id="share-password" placeholder="Enter password" disabled>
                                    <button id="generate-password" data-i18n="share.generatePassword">Generate</button>
                                </div>
                            </div>

                            <div class="share-setting">
                                <label for="share-expiration" data-i18n="share.expiration">Expiration Date:</label>
                                <div class="expiration-setting">
                                    <input type="checkbox" id="enable-expiration">
                                    <input type="date" id="share-expiration" disabled>
                                </div>
                            </div>
                        </div>

                        <div class="share-actions">
                            <button id="update-share-btn" class="button primary" data-i18n="share.update">Update Share</button>
                            <button id="remove-share-btn" class="button danger" data-i18n="share.remove">Remove Share</button>
                        </div>
                    </div>
                </div>
            </div>

            <!-- Email Notification Dialog -->
            <div id="share-notification-dialog" class="dialog">
                <div class="dialog-content">
                    <div class="dialog-header">
                        <h3 data-i18n="share.notifyTitle">Send Notification</h3>
                        <button class="close-dialog-btn">&times;</button>
                    </div>
                    <div class="dialog-body">
                        <div class="share-item-info">
                            <span id="notify-dialog-icon" class="item-icon">📄</span>
                            <span id="notify-dialog-name" class="item-name">filename.ext</span>
                        </div>

                        <div class="notification-form">
                            <div class="form-group">
                                <label for="notification-email" data-i18n="share.notifyEmailLabel">Email Address:</label>
                                <input type="email" id="notification-email" placeholder="Enter recipient email">
                            </div>

                            <div class="form-group">
                                <label for="notification-message" data-i18n="share.notifyMessageLabel">Message (optional):</label>
                                <textarea id="notification-message" placeholder="Add a personal message" rows="3"></textarea>
                            </div>
                        </div>

                        <div class="notification-actions">
                            <button id="send-notification-btn" class="button primary" data-i18n="share.notifySend">Send Notification</button>
                        </div>
                    </div>
                </div>
            </div>
        `;
        
        // Hide other UI elements
        const filesGrid = document.getElementById('files-grid');
        const filesListView = document.getElementById('files-list-view');
        if (filesGrid) filesGrid.style.display = 'none';
        if (filesListView) filesListView.style.display = 'none';
        
        // Translate UI if i18n is loaded
        if (window.i18n && window.i18n.translatePage) {
            window.i18n.translatePage();
        }
    },
    
    // Attach event listeners to the shared view UI
    attachEventListeners() {
        const filterType = document.getElementById('filter-type');
        const sortBy = document.getElementById('sort-by');
        const searchFilter = document.getElementById('shared-search-filter');
        const searchBtn = document.getElementById('shared-search-filter-btn');
        const emptyGoToFiles = document.getElementById('empty-go-to-files');
        
        if (filterType) filterType.addEventListener('change', () => this.filterAndSortItems());
        if (sortBy) sortBy.addEventListener('change', () => this.filterAndSortItems());
        if (searchFilter) searchFilter.addEventListener('keyup', (e) => {
            if (e.key === 'Enter') this.filterAndSortItems();
        });
        if (searchBtn) searchBtn.addEventListener('click', () => this.filterAndSortItems());
        
        // Back to files button (empty state)
        if (emptyGoToFiles) emptyGoToFiles.addEventListener('click', () => window.switchToFilesView());
        
        // Share dialog buttons
        const shareDialog = document.getElementById('share-dialog');
        if (shareDialog) {
            const closeBtn = shareDialog.querySelector('.close-dialog-btn');
            const copyLinkBtn = document.getElementById('copy-link-btn');
            const enablePassword = document.getElementById('enable-password');
            const sharePassword = document.getElementById('share-password');
            const generatePasswordBtn = document.getElementById('generate-password');
            const enableExpiration = document.getElementById('enable-expiration');
            const shareExpiration = document.getElementById('share-expiration');
            const updateShareBtn = document.getElementById('update-share-btn');
            const removeShareBtn = document.getElementById('remove-share-btn');
            
            if (closeBtn) closeBtn.addEventListener('click', () => this.closeShareDialog());
            if (copyLinkBtn) copyLinkBtn.addEventListener('click', () => this.copyShareLink());
            if (enablePassword) enablePassword.addEventListener('change', () => {
                if (sharePassword) {
                    sharePassword.disabled = !enablePassword.checked;
                    if (enablePassword.checked) sharePassword.focus();
                }
            });
            if (generatePasswordBtn) generatePasswordBtn.addEventListener('click', () => this.generatePassword());
            if (enableExpiration) enableExpiration.addEventListener('change', () => {
                if (shareExpiration) {
                    shareExpiration.disabled = !enableExpiration.checked;
                    if (enableExpiration.checked) shareExpiration.focus();
                }
            });
            if (updateShareBtn) updateShareBtn.addEventListener('click', () => this.updateSharedItem());
            if (removeShareBtn) removeShareBtn.addEventListener('click', () => this.removeSharedItem());
        }
        
        // Notification dialog buttons
        const notificationDialog = document.getElementById('share-notification-dialog');
        if (notificationDialog) {
            const closeBtn = notificationDialog.querySelector('.close-dialog-btn');
            const sendBtn = document.getElementById('send-notification-btn');
            
            if (closeBtn) closeBtn.addEventListener('click', () => this.closeNotificationDialog());
            if (sendBtn) sendBtn.addEventListener('click', () => this.sendNotification());
        }
    },
    
    // Filter and sort the items based on the current settings
    filterAndSortItems() {
        const filterType = document.getElementById('filter-type');
        const sortBy = document.getElementById('sort-by');
        const searchFilter = document.getElementById('shared-search-filter');
        
        if (!filterType || !sortBy || !searchFilter) return;
        
        const type = filterType.value;
        const sort = sortBy.value;
        const searchTerm = searchFilter.value.toLowerCase();
        
        // Filter items
        this.filteredItems = this.items.filter(item => {
            // Filter by type
            if (type !== 'all' && item.type !== type) return false;
            
            // Filter by search term
            const nameMatch = item.name.toLowerCase().includes(searchTerm);
            return nameMatch;
        });
        
        // Sort items
        this.filteredItems.sort((a, b) => {
            if (sort === 'name') {
                return a.name.localeCompare(b.name);
            } else if (sort === 'date') {
                return new Date(b.created_at || b.dateShared) - new Date(a.created_at || a.dateShared);
            } else if (sort === 'expiration') {
                // Handle null expiration dates (items without expiration come last)
                if (!a.expires_at && !b.expires_at) return 0;
                if (!a.expires_at) return 1;
                if (!b.expires_at) return -1;
                return new Date(a.expires_at) - new Date(b.expires_at);
            }
            return 0;
        });
        
        // Display filtered and sorted items
        this.displaySharedItems();
    },
    
    // Display the shared items in the UI
    displaySharedItems() {
        const sharedItemsList = document.getElementById('shared-items-list');
        const emptySharedState = document.getElementById('empty-shared-state');
        const sharedListContainer = document.querySelector('.shared-list-container');
        
        if (!sharedItemsList || !emptySharedState || !sharedListContainer) return;
        
        // Clear the list
        sharedItemsList.innerHTML = '';
        
        // Show empty state if no items
        if (this.filteredItems.length === 0) {
            emptySharedState.style.display = 'flex';
            sharedListContainer.style.display = 'none';
            return;
        }
        
        // Hide empty state and show table
        emptySharedState.style.display = 'none';
        sharedListContainer.style.display = 'block';
        
        // Add items to the list
        this.filteredItems.forEach(item => {
            const row = document.createElement('tr');
            
            // Icon and name
            const nameCell = document.createElement('td');
            nameCell.className = 'shared-item-name';
            const icon = document.createElement('span');
            icon.className = 'item-icon';
            icon.textContent = item.type === 'file' ? '📄' : '📁';
            const name = document.createElement('span');
            name.textContent = item.name;
            nameCell.appendChild(icon);
            nameCell.appendChild(name);
            
            // Type
            const typeCell = document.createElement('td');
            typeCell.textContent = item.type === 'file' ? this.translate('shared_typeFile', 'File') : this.translate('shared_typeFolder', 'Folder');
            
            // Date shared
            const dateCell = document.createElement('td');
            dateCell.textContent = this.formatDate(item.created_at || item.dateShared);
            
            // Expiration
            const expirationCell = document.createElement('td');
            expirationCell.textContent = item.expires_at ? this.formatDate(item.expires_at) : this.translate('shared_noExpiration', 'No expiration');
            
            // Permissions
            const permissionsCell = document.createElement('td');
            const permissions = [];
            if (item.permissions?.read) permissions.push(this.translate('share_permissionRead', 'Read'));
            if (item.permissions?.write) permissions.push(this.translate('share_permissionWrite', 'Write'));
            if (item.permissions?.reshare) permissions.push(this.translate('share_permissionReshare', 'Reshare'));
            permissionsCell.textContent = permissions.join(', ') || 'Read';
            
            // Password
            const passwordCell = document.createElement('td');
            passwordCell.textContent = (item.password || item.password_protected) ? this.translate('shared_hasPassword', 'Yes') : this.translate('shared_noPassword', 'No');
            
            // Actions
            const actionsCell = document.createElement('td');
            actionsCell.className = 'shared-item-actions';
            
            // Edit button
            const editBtn = document.createElement('button');
            editBtn.className = 'action-btn edit-btn';
            editBtn.innerHTML = '<span class="action-icon">✏️</span>';
            editBtn.title = this.translate('shared_editShare', 'Edit Share');
            editBtn.addEventListener('click', () => this.openShareDialog(item));
            
            // Notify button
            const notifyBtn = document.createElement('button');
            notifyBtn.className = 'action-btn notify-btn';
            notifyBtn.innerHTML = '<span class="action-icon">📧</span>';
            notifyBtn.title = this.translate('shared_notifyShare', 'Notify Someone');
            notifyBtn.addEventListener('click', () => this.openNotificationDialog(item));
            
            // Copy link button
            const copyBtn = document.createElement('button');
            copyBtn.className = 'action-btn copy-btn';
            copyBtn.innerHTML = '<span class="action-icon">📋</span>';
            copyBtn.title = this.translate('shared_copyLink', 'Copy Link');
            copyBtn.addEventListener('click', () => {
                navigator.clipboard.writeText(item.url)
                    .then(() => this.showNotification(this.translate('shared_linkCopied', 'Link copied to clipboard!')))
                    .catch(err => this.showNotification(this.translate('shared_linkCopyFailed', 'Failed to copy link'), 'error'));
            });
            
            // Remove button
            const removeBtn = document.createElement('button');
            removeBtn.className = 'action-btn remove-btn';
            removeBtn.innerHTML = '<span class="action-icon">🗑️</span>';
            removeBtn.title = this.translate('shared_removeShare', 'Remove Share');
            removeBtn.addEventListener('click', () => {
                this.currentItem = item;
                this.removeSharedItem();
            });
            
            actionsCell.appendChild(editBtn);
            actionsCell.appendChild(notifyBtn);
            actionsCell.appendChild(copyBtn);
            actionsCell.appendChild(removeBtn);
            
            // Add cells to row
            row.appendChild(nameCell);
            row.appendChild(typeCell);
            row.appendChild(dateCell);
            row.appendChild(expirationCell);
            row.appendChild(permissionsCell);
            row.appendChild(passwordCell);
            row.appendChild(actionsCell);
            
            // Add row to table
            sharedItemsList.appendChild(row);
        });
    },
    
    // Open the share dialog for a shared item
    openShareDialog(item) {
        this.currentItem = item;
        const shareDialog = document.getElementById('share-dialog');
        const shareDialogIcon = document.getElementById('share-dialog-icon');
        const shareDialogName = document.getElementById('share-dialog-name');
        const shareLinkUrl = document.getElementById('share-link-url');
        const enablePassword = document.getElementById('enable-password');
        const sharePassword = document.getElementById('share-password');
        const enableExpiration = document.getElementById('enable-expiration');
        const shareExpiration = document.getElementById('share-expiration');
        const permissionRead = document.getElementById('permission-read');
        const permissionWrite = document.getElementById('permission-write');
        const permissionReshare = document.getElementById('permission-reshare');
        
        if (!shareDialog || !shareDialogIcon || !shareDialogName || !shareLinkUrl) return;
        
        // Set dialog content
        shareDialogIcon.textContent = item.type === 'file' ? '📄' : '📁';
        shareDialogName.textContent = item.name;
        shareLinkUrl.value = item.url;
        
        // Set permissions
        if (permissionRead) permissionRead.checked = item.permissions?.read !== false;
        if (permissionWrite) permissionWrite.checked = !!item.permissions?.write;
        if (permissionReshare) permissionReshare.checked = !!item.permissions?.reshare;
        
        // Set password
        if (enablePassword) {
            enablePassword.checked = !!(item.password || item.password_protected);
            if (sharePassword) {
                sharePassword.disabled = !enablePassword.checked;
                sharePassword.value = item.password || '';
            }
        }
        
        // Set expiration
        if (enableExpiration) {
            enableExpiration.checked = !!item.expires_at;
            if (shareExpiration) {
                shareExpiration.disabled = !enableExpiration.checked;
                shareExpiration.value = item.expires_at ? new Date(item.expires_at).toISOString().split('T')[0] : '';
            }
        }
        
        // Show dialog
        shareDialog.classList.add('active');
    },
    
    // Close the share dialog
    closeShareDialog() {
        const shareDialog = document.getElementById('share-dialog');
        if (shareDialog) shareDialog.classList.remove('active');
        this.currentItem = null;
    },
    
    // Open the notification dialog for a shared item
    openNotificationDialog(item) {
        this.currentItem = item;
        const notificationDialog = document.getElementById('share-notification-dialog');
        const notifyDialogIcon = document.getElementById('notify-dialog-icon');
        const notifyDialogName = document.getElementById('notify-dialog-name');
        const notificationEmail = document.getElementById('notification-email');
        const notificationMessage = document.getElementById('notification-message');
        
        if (!notificationDialog || !notifyDialogIcon || !notifyDialogName || !notificationEmail || !notificationMessage) return;
        
        // Set dialog content
        notifyDialogIcon.textContent = item.type === 'file' ? '📄' : '📁';
        notifyDialogName.textContent = item.name;
        notificationEmail.value = '';
        notificationMessage.value = '';
        
        // Show dialog
        notificationDialog.classList.add('active');
    },
    
    // Close the notification dialog
    closeNotificationDialog() {
        const notificationDialog = document.getElementById('share-notification-dialog');
        if (notificationDialog) notificationDialog.classList.remove('active');
        this.currentItem = null;
    },
    
    // Copy a share link to the clipboard
    copyShareLink() {
        const shareLinkUrl = document.getElementById('share-link-url');
        if (!shareLinkUrl) return;
        
        navigator.clipboard.writeText(shareLinkUrl.value)
            .then(() => this.showNotification(this.translate('shared_linkCopied', 'Link copied to clipboard!')))
            .catch(err => this.showNotification(this.translate('shared_linkCopyFailed', 'Failed to copy link'), 'error'));
    },
    
    // Generate a random password for a share
    generatePassword() {
        const sharePassword = document.getElementById('share-password');
        const enablePassword = document.getElementById('enable-password');
        if (!sharePassword || !enablePassword) return;
        
        const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*';
        let password = '';
        for (let i = 0; i < 12; i++) {
            password += chars.charAt(Math.floor(Math.random() * chars.length));
        }
        sharePassword.value = password;
        enablePassword.checked = true;
        sharePassword.disabled = false;
    },
    
    // Update a shared item with new settings
    updateSharedItem() {
        if (!this.currentItem) return;
        const permissionRead = document.getElementById('permission-read');
        const permissionWrite = document.getElementById('permission-write');
        const permissionReshare = document.getElementById('permission-reshare');
        const enablePassword = document.getElementById('enable-password');
        const sharePassword = document.getElementById('share-password');
        const enableExpiration = document.getElementById('enable-expiration');
        const shareExpiration = document.getElementById('share-expiration');
        
        if (!permissionRead || !permissionWrite || !permissionReshare || !enablePassword || !sharePassword || !enableExpiration || !shareExpiration) return;
        
        // Get updated settings
        const permissions = {
            read: permissionRead.checked,
            write: permissionWrite.checked,
            reshare: permissionReshare.checked
        };
        
        const password = enablePassword.checked ? sharePassword.value : null;
        const expires_at = enableExpiration.checked ? new Date(shareExpiration.value).toISOString() : null;
        
        // Update the shared link via the global function
        if (window.updateSharedLink) {
            window.updateSharedLink(this.currentItem.id, {
                permissions,
                password,
                expires_at
            });
        }
        
        // Reload items and close dialog
        this.loadItems();
        this.filterAndSortItems();
        this.closeShareDialog();
        
        // Show notification
        this.showNotification(this.translate('shared_itemUpdated', 'Share settings updated successfully'));
    },
    
    // Remove a shared item
    removeSharedItem() {
        if (!this.currentItem) return;
        
        // Remove the shared link via the global function
        if (window.removeSharedLink) {
            window.removeSharedLink(this.currentItem.id);
        }
        
        // Reload items and close dialog if open
        this.loadItems();
        this.filterAndSortItems();
        this.closeShareDialog();
        
        // Show notification
        this.showNotification(this.translate('shared_itemRemoved', 'Share removed successfully'));
    },
    
    // Send a notification for a shared item
    sendNotification() {
        if (!this.currentItem) return;
        const notificationEmail = document.getElementById('notification-email');
        const notificationMessage = document.getElementById('notification-message');
        
        if (!notificationEmail || !notificationMessage) return;
        
        const email = notificationEmail.value.trim();
        const message = notificationMessage.value.trim();
        
        // Validate email
        if (!email || !this.validateEmail(email)) {
            this.showNotification(this.translate('shared_invalidEmail', 'Please enter a valid email address'), 'error');
            return;
        }
        
        // Send notification via the global function
        if (window.sendShareNotification) {
            window.sendShareNotification(this.currentItem.id, email, message)
                .then(() => {
                    this.closeNotificationDialog();
                    this.showNotification(this.translate('shared_notificationSent', 'Notification sent successfully'));
                })
                .catch(error => {
                    this.showNotification(this.translate('shared_notificationFailed', 'Failed to send notification'), 'error');
                });
        }
    },
    
    // Show a notification
    showNotification(message, type = 'success') {
        if (window.ui && window.ui.showNotification) {
            window.ui.showNotification(message, type);
        } else {
            alert(message);
        }
    },
    
    // Validate an email address
    validateEmail(email) {
        const re = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return re.test(email);
    },
    
    // Format a date string
    formatDate(dateString) {
        if (!dateString) return 'N/A';
        const options = { year: 'numeric', month: 'short', day: 'numeric' };
        return new Date(dateString).toLocaleDateString(undefined, options);
    },
    
    // Translate a string using i18n if available
    translate(key, defaultText) {
        if (window.i18n && window.i18n.t) {
            return window.i18n.t(key, defaultText);
        }
        return defaultText;
    }
};

// Export the shared view component
window.sharedView = sharedView;