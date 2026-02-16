/**
 * OxiCloud - UI Module
 * This file handles UI-related functions, view toggling, and interface interactions
 */

// UI Module
const ui = {
    /**
     * Initialize context menus and dialogs
     */
    initializeContextMenus() {
        // Folder context menu
        if (!document.getElementById('folder-context-menu')) {
            const folderMenu = document.createElement('div');
            folderMenu.className = 'context-menu';
            folderMenu.id = 'folder-context-menu';
            folderMenu.innerHTML = `
                <div class="context-menu-item" id="download-folder-option">
                    <i class="fas fa-download"></i> <span data-i18n="actions.download">Download</span>
                </div>
                <div class="context-menu-item" id="favorite-folder-option">
                    <i class="fas fa-star"></i> <span data-i18n="actions.favorite">Add to favorites</span>
                </div>
                <div class="context-menu-item" id="share-folder-option">
                    <i class="fas fa-share-alt"></i> <span data-i18n="actions.share">Share</span>
                </div>
                <div class="context-menu-separator"></div>
                <div class="context-menu-item" id="rename-folder-option">
                    <i class="fas fa-pen"></i> <span data-i18n="actions.rename">Rename</span>
                </div>
                <div class="context-menu-item" id="move-folder-option">
                    <i class="fas fa-arrows-alt"></i> <span data-i18n="actions.move">Move to...</span>
                </div>
                <div class="context-menu-separator"></div>
                <div class="context-menu-item context-menu-item-danger" id="delete-folder-option">
                    <i class="fas fa-trash-alt"></i> <span data-i18n="actions.delete">Delete</span>
                </div>
            `;
            document.body.appendChild(folderMenu);
        }

        // File context menu
        if (!document.getElementById('file-context-menu')) {
            const fileMenu = document.createElement('div');
            fileMenu.className = 'context-menu';
            fileMenu.id = 'file-context-menu';
            fileMenu.innerHTML = `
                <div class="context-menu-item" id="view-file-option">
                    <i class="fas fa-eye"></i> <span data-i18n="actions.view">View</span>
                </div>
                <div class="context-menu-item" id="download-file-option">
                    <i class="fas fa-download"></i> <span data-i18n="actions.download">Download</span>
                </div>
                <div class="context-menu-separator"></div>
                <div class="context-menu-item" id="favorite-file-option">
                    <i class="fas fa-star"></i> <span data-i18n="actions.favorite">Add to favorites</span>
                </div>
                <div class="context-menu-item" id="share-file-option">
                    <i class="fas fa-share-alt"></i> <span data-i18n="actions.share">Share</span>
                </div>
                <div class="context-menu-separator"></div>
                <div class="context-menu-item" id="rename-file-option">
                    <i class="fas fa-pen"></i> <span data-i18n="actions.rename">Rename</span>
                </div>
                <div class="context-menu-item" id="move-file-option">
                    <i class="fas fa-arrows-alt"></i> <span data-i18n="actions.move">Move to...</span>
                </div>
                <div class="context-menu-separator"></div>
                <div class="context-menu-item context-menu-item-danger" id="delete-file-option">
                    <i class="fas fa-trash-alt"></i> <span data-i18n="actions.delete">Delete</span>
                </div>
            `;
            document.body.appendChild(fileMenu);
        }

        // Rename dialog — modern
        if (!document.getElementById('rename-dialog')) {
            const renameDialog = document.createElement('div');
            renameDialog.className = 'rename-dialog';
            renameDialog.id = 'rename-dialog';
            renameDialog.innerHTML = `
                <div class="rename-dialog-content">
                    <div class="rename-dialog-header">
                        <i class="fas fa-pen" style="color:#ff5e3a"></i>
                        <span data-i18n="dialogs.rename_folder">Rename</span>
                    </div>
                    <div class="rename-dialog-body">
                        <input type="text" id="rename-input" data-i18n-placeholder="dialogs.new_name" placeholder="New name">
                    </div>
                    <div class="rename-dialog-buttons">
                        <button class="btn btn-secondary" id="rename-cancel-btn" data-i18n="actions.cancel">Cancel</button>
                        <button class="btn btn-primary" id="rename-confirm-btn" data-i18n="actions.rename">Rename</button>
                    </div>
                </div>
            `;
            document.body.appendChild(renameDialog);
        }

        // Move dialog — modern
        if (!document.getElementById('move-file-dialog')) {
            const moveDialog = document.createElement('div');
            moveDialog.className = 'rename-dialog';
            moveDialog.id = 'move-file-dialog';
            moveDialog.innerHTML = `
                <div class="rename-dialog-content">
                    <div class="rename-dialog-header">
                        <i class="fas fa-arrows-alt" style="color:#ff5e3a"></i>
                        <span data-i18n="dialogs.move_file">Move</span>
                    </div>
                    <div class="rename-dialog-body">
                        <p style="margin:0 0 12px;color:#718096;font-size:14px" data-i18n="dialogs.select_destination">Select destination folder:</p>
                        <div id="folder-select-container" style="max-height:220px;overflow-y:auto;">
                            <div class="folder-select-item selected" data-folder-id="">
                                <i class="fas fa-folder"></i> <span data-i18n="dialogs.root">Root</span>
                            </div>
                        </div>
                    </div>
                    <div class="rename-dialog-buttons">
                        <button class="btn btn-secondary" id="move-cancel-btn" data-i18n="actions.cancel">Cancel</button>
                        <button class="btn btn-primary" id="move-confirm-btn" data-i18n="actions.move_to">Move</button>
                    </div>
                </div>
            `;
            document.body.appendChild(moveDialog);
        }
        
        // Share dialog
        if (!document.getElementById('share-dialog')) {
            const shareDialog = document.createElement('div');
            shareDialog.className = 'share-dialog';
            shareDialog.id = 'share-dialog';
            shareDialog.innerHTML = `
                <div class="share-dialog-content">
                    <div class="share-dialog-header">
                        <i class="fas fa-share-alt" style="color:#ff5e3a"></i>
                        <span data-i18n="dialogs.share_file">Share file</span>
                    </div>
                    <div class="shared-item-info">
                        <strong>Item:</strong> <span id="shared-item-name"></span>
                    </div>
                    
                    <div id="existing-shares-section" style="display:none; margin: 15px 0;">
                        <h3 data-i18n="dialogs.existing_shares">Existing shared links</h3>
                        <div id="existing-shares-container"></div>
                    </div>
                    
                    <div class="share-options">
                        <h3 data-i18n="dialogs.share_options">Share options</h3>
                        
                        <div class="form-group">
                            <label for="share-password" data-i18n="dialogs.password">Password (optional):</label>
                            <input type="password" id="share-password" placeholder="Protect with password">
                        </div>
                        
                        <div class="form-group">
                            <label for="share-expiration" data-i18n="dialogs.expiration">Expiration date (optional):</label>
                            <input type="date" id="share-expiration">
                        </div>
                        
                        <div class="form-group">
                            <label data-i18n="dialogs.permissions">Permissions:</label>
                            <div class="permission-options">
                                <div class="permission-option">
                                    <input type="checkbox" id="share-permission-read" checked>
                                    <label for="share-permission-read" data-i18n="permissions.read">Read</label>
                                </div>
                                <div class="permission-option">
                                    <input type="checkbox" id="share-permission-write">
                                    <label for="share-permission-write" data-i18n="permissions.write">Write</label>
                                </div>
                                <div class="permission-option">
                                    <input type="checkbox" id="share-permission-reshare">
                                    <label for="share-permission-reshare" data-i18n="permissions.reshare">Allow sharing</label>
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    <div id="new-share-section" style="display:none; margin: 15px 0;">
                        <h3 data-i18n="dialogs.generated_link">Generated link</h3>
                        <div class="form-group">
                            <input type="text" id="generated-share-url" readonly>
                            <div class="share-link-actions">
                                <button class="btn btn-small" id="copy-share-btn">
                                    <i class="fas fa-copy"></i> <span data-i18n="actions.copy">Copy</span>
                                </button>
                                <button class="btn btn-small" id="notify-share-btn">
                                    <i class="fas fa-envelope"></i> <span data-i18n="actions.notify">Notify</span>
                                </button>
                            </div>
                        </div>
                    </div>
                    
                    <div class="share-dialog-buttons">
                        <button class="btn btn-secondary" id="share-cancel-btn" data-i18n="actions.cancel">Cancel</button>
                        <button class="btn btn-primary" id="share-confirm-btn" data-i18n="actions.share">Share</button>
                    </div>
                </div>
            `;
            document.body.appendChild(shareDialog);
            
            // Add event listeners for share dialog
            document.getElementById('share-cancel-btn').addEventListener('click', () => {
                contextMenus.closeShareDialog();
            });
            
            document.getElementById('share-confirm-btn').addEventListener('click', async () => {
                await contextMenus.createSharedLink();
            });
            
            document.getElementById('copy-share-btn').addEventListener('click', async () => {
                const shareUrl = document.getElementById('generated-share-url').value;
                await fileSharing.copyLinkToClipboard(shareUrl);
            });
            
            document.getElementById('notify-share-btn').addEventListener('click', () => {
                const shareUrl = document.getElementById('generated-share-url').value;
                contextMenus.showEmailNotificationDialog(shareUrl);
            });
        }
        
        // Notification dialog
        if (!document.getElementById('notification-dialog')) {
            const notificationDialog = document.createElement('div');
            notificationDialog.className = 'share-dialog';
            notificationDialog.id = 'notification-dialog';
            notificationDialog.innerHTML = `
                <div class="share-dialog-content">
                    <div class="share-dialog-header">
                        <i class="fas fa-envelope" style="color:#ff5e3a"></i>
                        <span data-i18n="dialogs.notify">Notify shared link</span>
                    </div>
                    
                    <p><strong>URL:</strong> <span id="notification-share-url"></span></p>
                    
                    <div class="form-group">
                        <label for="notification-email" data-i18n="dialogs.recipient">Recipient:</label>
                        <input type="email" id="notification-email" placeholder="Email address">
                    </div>
                    
                    <div class="form-group">
                        <label for="notification-message" data-i18n="dialogs.message">Message (optional):</label>
                        <textarea id="notification-message" rows="3"></textarea>
                    </div>
                    
                    <div class="share-dialog-buttons">
                        <button class="btn btn-secondary" id="notification-cancel-btn" data-i18n="actions.cancel">Cancel</button>
                        <button class="btn btn-primary" id="notification-send-btn" data-i18n="actions.send">Send</button>
                    </div>
                </div>
            `;
            document.body.appendChild(notificationDialog);
            
            // Add event listeners for notification dialog
            document.getElementById('notification-cancel-btn').addEventListener('click', () => {
                contextMenus.closeNotificationDialog();
            });
            
            document.getElementById('notification-send-btn').addEventListener('click', () => {
                contextMenus.sendShareNotification();
            });
        }

        // Assign events to menu items
        if (window.contextMenus) {
            window.contextMenus.assignMenuEvents();
        } else {
            console.warn('contextMenus module not loaded');
        }
    },

    /**
     * Set up drag and drop functionality
     */
    setupDragAndDrop() {
        const dropzone = document.getElementById('dropzone');

        // Dropzone events
        dropzone.addEventListener('dragover', (e) => {
            e.preventDefault();
            dropzone.classList.add('active');
        });

        dropzone.addEventListener('dragleave', () => {
            dropzone.classList.remove('active');
        });

        dropzone.addEventListener('drop', (e) => {
            e.preventDefault();
            e.stopPropagation(); // Prevent bubbling to document's drop handler (avoids double upload)
            e._oxiHandled = true;  // Mark as handled for document-level fallback
            dropzone.classList.remove('active');
            if (e.dataTransfer.files.length > 0) {
                // Detect folder drops: files from folder drops have webkitRelativePath set
                const hasRelativePaths = Array.from(e.dataTransfer.files).some(
                    f => f.webkitRelativePath && f.webkitRelativePath.includes('/')
                );
                if (hasRelativePaths) {
                    fileOps.uploadFolderFiles(e.dataTransfer.files);
                } else {
                    fileOps.uploadFiles(e.dataTransfer.files);
                }
            }
            setTimeout(() => {
                dropzone.style.display = 'none';
            }, 500);
        });

        // Document-wide drag and drop
        document.addEventListener('dragover', (e) => {
            e.preventDefault();
            if (e.dataTransfer.types.includes('Files')) {
                dropzone.style.display = 'block';
                dropzone.classList.add('active');
            }
        });

        document.addEventListener('dragleave', (e) => {
            if (e.clientX <= 0 || e.clientY <= 0 ||
                e.clientX >= window.innerWidth || e.clientY >= window.innerHeight) {
                dropzone.classList.remove('active');
                setTimeout(() => {
                    if (!dropzone.classList.contains('active')) {
                        dropzone.style.display = 'none';
                    }
                }, 100);
            }
        });

        document.addEventListener('drop', (e) => {
            e.preventDefault();
            dropzone.classList.remove('active');

            // Skip if already handled by the dropzone handler (defensive against bubble leaks)
            if (e._oxiHandled) return;

            if (e.dataTransfer.files.length > 0) {
                // Detect folder drops: files from folder drops have webkitRelativePath set
                const hasRelativePaths = Array.from(e.dataTransfer.files).some(
                    f => f.webkitRelativePath && f.webkitRelativePath.includes('/')
                );
                if (hasRelativePaths) {
                    fileOps.uploadFolderFiles(e.dataTransfer.files);
                } else {
                    fileOps.uploadFiles(e.dataTransfer.files);
                }
            }

            setTimeout(() => {
                dropzone.style.display = 'none';
            }, 500);
        });
    },

    /**
     * Switch to grid view
     */
    switchToGridView() {
        const filesGrid = document.getElementById('files-grid');
        const filesListView = document.getElementById('files-list-view');
        const gridViewBtn = document.getElementById('grid-view-btn');
        const listViewBtn = document.getElementById('list-view-btn');

        filesGrid.style.display = 'grid';
        filesListView.style.display = 'none';
        gridViewBtn.classList.add('active');
        listViewBtn.classList.remove('active');
        window.app.currentView = 'grid';
        localStorage.setItem('oxicloud-view', 'grid');
    },

    /**
     * Switch to list view
     */
    switchToListView() {
        const filesGrid = document.getElementById('files-grid');
        const filesListView = document.getElementById('files-list-view');
        const gridViewBtn = document.getElementById('grid-view-btn');
        const listViewBtn = document.getElementById('list-view-btn');

        filesGrid.style.display = 'none';
        filesListView.style.display = 'flex';
        gridViewBtn.classList.remove('active');
        listViewBtn.classList.add('active');
        window.app.currentView = 'list';
        localStorage.setItem('oxicloud-view', 'list');
    },

    /**
     * Update breadcrumb navigation
     * @param {string} folderName - Name of the current folder
     */
    updateBreadcrumb(folderName) {
        const breadcrumb = document.querySelector('.breadcrumb');
        breadcrumb.innerHTML = '';
        
        // Get user info to help determine home folder
        const USER_DATA_KEY = 'oxicloud_user';
        const userData = JSON.parse(localStorage.getItem(USER_DATA_KEY) || '{}');
        const username = userData.username || '';
        
        // Create the home item - for users, this is their personal folder
        const homeItem = document.createElement('span');
        homeItem.className = 'breadcrumb-item';
        
        // Helper function to safely get translation text
        const getTranslatedText = (key, defaultValue) => {
            if (!window.i18n || !window.i18n.t) return defaultValue;
            return window.i18n.t(key);
        };
        
        // First determine if the current view is the user's home folder 
        const isUserHomeFolder = username && window.app.userHomeFolderName && 
            window.app.userHomeFolderName.includes(username) && 
            folderName === window.app.userHomeFolderName;
            
        // Set appropriate text for home item
        if (isUserHomeFolder) {
            // If the current folder is the user's home folder, label it as "Home"
            homeItem.textContent = getTranslatedText('breadcrumb.home', 'Home');
        } else if (folderName && folderName.startsWith('My Folder')) {
            // If viewing a root folder but not the user's home folder, use its full name
            homeItem.textContent = folderName;
        } else {
            // Default - use "Home" label
            homeItem.textContent = getTranslatedText('breadcrumb.home', 'Home');
            
            // For searching, we might have a custom breadcrumb text
            if (folderName && folderName.startsWith('Search:')) {
                // We're in search mode - don't add click handler
                breadcrumb.appendChild(homeItem);
                return;
            }
        }
        
        // Add click handler - but only if we have a user home folder to return to
        if (window.app.userHomeFolderId) {
            homeItem.addEventListener('click', () => {
                window.app.currentPath = window.app.userHomeFolderId;
                this.updateBreadcrumb(window.app.userHomeFolderName || 'Home');
                window.loadFiles();
            });
        }
        
        breadcrumb.appendChild(homeItem);

        // If we have a subfolder, add it to the breadcrumb
        if (folderName && !folderName.startsWith('Mi Carpeta') && !folderName.startsWith('Search:')) {
            const separator = document.createElement('span');
            separator.className = 'breadcrumb-separator';
            separator.textContent = '>';
            breadcrumb.appendChild(separator);

            const folderItem = document.createElement('span');
            folderItem.className = 'breadcrumb-item';
            folderItem.textContent = folderName;
            breadcrumb.appendChild(folderItem);
        }
    },

    /**
     * Check if a file can be previewed in the viewer
     * @param {Object} file - File object with mime_type property
     * @returns {boolean}
     */
    isViewableFile(file) {
        if (!file || !file.mime_type) return false;
        if (file.mime_type.startsWith('image/')) return true;
        if (file.mime_type === 'application/pdf') return true;
        // Delegate text-viewability to the single global definition
        return window.isTextViewable ? window.isTextViewable(file.mime_type) : false;
    },

    /**
     * Get FontAwesome icon class for a filename based on its extension.
     * Used as fallback when the backend DTO doesn't include icon_class
     * (e.g. trash items).
     */
    getIconClass(fileName) {
        if (!fileName) return 'fas fa-file';
        const ext = (fileName.split('.').pop() || '').toLowerCase();
        const map = {
            pdf:'fas fa-file-pdf', doc:'fas fa-file-word', docx:'fas fa-file-word',
            txt:'fas fa-file-alt', rtf:'fas fa-file-alt', odt:'fas fa-file-alt',
            xls:'fas fa-file-excel', xlsx:'fas fa-file-excel', csv:'fas fa-file-excel', ods:'fas fa-file-excel',
            ppt:'fas fa-file-powerpoint', pptx:'fas fa-file-powerpoint', odp:'fas fa-file-powerpoint',
            jpg:'fas fa-file-image', jpeg:'fas fa-file-image', png:'fas fa-file-image',
            gif:'fas fa-file-image', svg:'fas fa-file-image', webp:'fas fa-file-image',
            bmp:'fas fa-file-image', ico:'fas fa-file-image',
            mp4:'fas fa-file-video', avi:'fas fa-file-video', mov:'fas fa-file-video',
            mkv:'fas fa-file-video', webm:'fas fa-file-video', flv:'fas fa-file-video',
            mp3:'fas fa-file-audio', wav:'fas fa-file-audio', ogg:'fas fa-file-audio',
            flac:'fas fa-file-audio', aac:'fas fa-file-audio', m4a:'fas fa-file-audio',
            zip:'fas fa-file-archive', rar:'fas fa-file-archive', '7z':'fas fa-file-archive',
            tar:'fas fa-file-archive', gz:'fas fa-file-archive',
            js:'fas fa-file-code', ts:'fas fa-file-code', py:'fas fa-file-code',
            rs:'fas fa-file-code', java:'fas fa-file-code', html:'fas fa-file-code',
            css:'fas fa-file-code', json:'fas fa-file-code', xml:'fas fa-file-code',
            sh:'fas fa-terminal', bash:'fas fa-terminal', bat:'fas fa-terminal',
            md:'fas fa-file-alt',
        };
        return map[ext] || 'fas fa-file';
    },

    /**
     * Show notification
     * @param {string} title - Notification title
     * @param {string} message - Notification message
     */
    showNotification(title, message) {
        let notification = document.querySelector('.notification');
        if (!notification) {
            notification = document.createElement('div');
            notification.className = 'notification';
            notification.innerHTML = `
                <div class="notification-title">${title}</div>
                <div class="notification-message">${message}</div>
            `;
            document.body.appendChild(notification);
        } else {
            notification.querySelector('.notification-title').textContent = title;
            notification.querySelector('.notification-message').textContent = message;
        }

        notification.style.display = 'block';

        setTimeout(() => {
            notification.style.display = 'none';
        }, 5000);
    },

    /**
     * Close folder context menu
     */
    closeContextMenu() {
        const menu = document.getElementById('folder-context-menu');
        if (menu) {
            menu.style.display = 'none';
            window.app.contextMenuTargetFolder = null;
        }
    },

    /**
     * Close file context menu
     */
    closeFileContextMenu() {
        const menu = document.getElementById('file-context-menu');
        if (menu) {
            menu.style.display = 'none';
            window.app.contextMenuTargetFile = null;
        }
    },



    /* ================================================================
     *  Data store + event delegation (replaces per-item listeners)
     * ================================================================ */

    /** @type {Map<string, Object>} item data keyed by id */
    _items: new Map(),

    /** @type {boolean} */
    _delegationReady: false,

    /**
     * Attach a fixed set of delegated event listeners to the two
     * container elements (files-grid, files-list-view).
     * Called once – idempotent.
     */
    initDelegation() {
        if (this._delegationReady) return;
        const grid = document.getElementById('files-grid');
        const list = document.getElementById('files-list-view');
        if (!grid || !list) return;
        this._delegationReady = true;

        const self = this;

        // ── helpers ────────────────────────────────────────────────
        const itemInfo = (card) => {
            if (!card) return null;
            const fileId = card.dataset.fileId;
            if (fileId) return { type: 'file', id: fileId, data: self._items.get(fileId) };
            const folderId = card.dataset.folderId;
            if (folderId) return { type: 'folder', id: folderId, data: self._items.get(folderId) };
            return null;
        };

        const openFile = (file) => {
            if (!file) return;
            if (window.recent) {
                document.dispatchEvent(new CustomEvent('file-accessed', { detail: { file } }));
            }
            if (self.isViewableFile(file)) {
                if (window.inlineViewer) window.inlineViewer.openFile(file);
                else window.fileOps.downloadFile(file.id, file.name);
            } else {
                window.fileOps.downloadFile(file.id, file.name);
            }
        };

        const navigateFolder = (card) => {
            window.app.currentPath = card.dataset.folderId;
            self.updateBreadcrumb(card.dataset.folderName);
            window.loadFiles();
        };

        const setContextTarget = (card, info) => {
            if (info.type === 'folder') {
                window.app.contextMenuTargetFolder = {
                    id: info.id,
                    name: card.dataset.folderName,
                    parent_id: card.dataset.parentId || ""
                };
            } else {
                window.app.contextMenuTargetFile = {
                    id: info.id,
                    name: card.dataset.fileName,
                    folder_id: card.dataset.folderId || ""
                };
            }
        };

        // ── GRID: click (select) ──────────────────────────────────
        grid.addEventListener('click', (e) => {
            const card = e.target.closest('.file-card');
            if (!card) return;

            if (e.target.closest('.file-card-more')) {
                e.stopPropagation();
                e.preventDefault();
                const info = itemInfo(card);
                if (!info) return;
                setContextTarget(card, info);
                const menuId = info.type === 'folder'
                    ? 'folder-context-menu' : 'file-context-menu';
                showContextMenuAtElement(
                    e.target.closest('.file-card-more'), menuId);
                return;
            }

            if (e.target.closest('.file-card-checkbox')) {
                toggleCardSelection(card, e);
                return;
            }

            // In favorites/recent view, single-click navigates/opens (like list)
            if (window.app.isFavoritesView || window.app.isRecentView) {
                const info = itemInfo(card);
                if (info) {
                    if (info.type === 'folder') navigateFolder(card);
                    else openFile(info.data);
                }
                return;
            }

            toggleCardSelection(card, e);
        });

        // ── GRID: dblclick (navigate / open) ──────────────────────
        grid.addEventListener('dblclick', (e) => {
            const card = e.target.closest('.file-card');
            if (!card) return;
            if (e.target.closest('.file-card-more') ||
                e.target.closest('.file-card-checkbox')) return;

            const info = itemInfo(card);
            if (!info) return;

            if (info.type === 'folder') {
                navigateFolder(card);
            } else {
                openFile(info.data);
            }
        });

        // ── LIST: click (navigate / open) ─────────────────────────
        list.addEventListener('click', (e) => {
            if (e.target.closest('.list-header')) return;
            const card = e.target.closest('.file-item');
            if (!card) return;

            if (e.target.closest('.list-item-checkbox') ||
                e.target.closest('.item-checkbox')) {
                toggleCardSelection(card, e);
                return;
            }

            const info = itemInfo(card);
            if (!info) return;

            if (info.type === 'folder') {
                navigateFolder(card);
            } else {
                openFile(info.data);
            }
        });

        // ── shared events on both containers ──────────────────────
        for (const container of [grid, list]) {
            const sel = container === grid ? '.file-card' : '.file-item';

            // contextmenu
            container.addEventListener('contextmenu', (e) => {
                const card = e.target.closest(sel);
                if (!card) return;
                e.preventDefault();
                const info = itemInfo(card);
                if (!info) return;
                setContextTarget(card, info);
                const menuId = info.type === 'folder'
                    ? 'folder-context-menu' : 'file-context-menu';
                const menu = document.getElementById(menuId);
                menu.style.left = `${e.pageX}px`;
                menu.style.top  = `${e.pageY}px`;
                menu.style.display = 'block';
            });

            // dragstart
            container.addEventListener('dragstart', (e) => {
                const card = e.target.closest(sel);
                if (!card) { e.preventDefault(); return; }

                // Grid items must be selected to start dragging
                if (container === grid &&
                    !card.classList.contains('selected')) {
                    e.preventDefault();
                    return;
                }

                const info = itemInfo(card);
                if (!info) { e.preventDefault(); return; }

                e.dataTransfer.setData('text/plain', info.id);
                if (info.type === 'folder') {
                    e.dataTransfer.setData(
                        'application/oxicloud-folder', 'true');
                }
                card.classList.add('dragging');
            });

            // dragend
            container.addEventListener('dragend', (e) => {
                const card = e.target.closest(sel);
                if (card) card.classList.remove('dragging');
                document.querySelectorAll('.drop-target')
                    .forEach(el => el.classList.remove('drop-target'));
            });

            // dragover – only folders are valid drop targets
            container.addEventListener('dragover', (e) => {
                const card = e.target.closest(sel);
                if (!card || card.dataset.fileId) return;
                if (!card.dataset.folderId) return;
                e.preventDefault();
                card.classList.add('drop-target');
            });

            // dragleave
            container.addEventListener('dragleave', (e) => {
                const card = e.target.closest(sel);
                if (!card || card.dataset.fileId) return;
                card.classList.remove('drop-target');
            });

            // drop – only folders accept drops
            container.addEventListener('drop', async (e) => {
                const card = e.target.closest(sel);
                if (!card || card.dataset.fileId) return;
                const targetFolderId = card.dataset.folderId;
                if (!targetFolderId) return;

                e.preventDefault();
                card.classList.remove('drop-target');

                const id = e.dataTransfer.getData('text/plain');
                const isFolder =
                    e.dataTransfer.getData('application/oxicloud-folder') === 'true';

                if (id) {
                    if (isFolder) {
                        if (id === targetFolderId) {
                            alert("You cannot move a folder to itself");
                            return;
                        }
                        await fileOps.moveFolder(id, targetFolderId);
                    } else {
                        await fileOps.moveFile(id, targetFolderId);
                    }
                }
            });
        }
    },

    /* ================================================================
     *  Pure element-creation helpers (no addEventListener)
     * ================================================================ */

    /** Create a grid card for a folder */
    _createFolderCard(folder) {
        const el = document.createElement('div');
        el.className = 'file-card';
        el.dataset.folderId  = folder.id;
        el.dataset.folderName = folder.name;
        el.dataset.parentId  = folder.parent_id || "";

        const isFav = window.favorites &&
            window.favorites.isFavorite(folder.id, 'folder');

        el.innerHTML = `
            <div class="file-card-checkbox"><i class="fas fa-check"></i></div>
            <button class="file-card-more"><i class="fas fa-ellipsis-v"></i></button>
            ${isFav ? '<div class="favorite-star active"><i class="fas fa-star"></i></div>' : ''}
            <div class="file-icon folder-icon">
                <i class="fas fa-folder"></i>
            </div>
            <div class="file-name">${escapeHtml(folder.name)}</div>
            <div class="file-info">Folder</div>
        `;

        if (window.app.currentPath !== "") {
            el.setAttribute('draggable', 'true');
        }
        return el;
    },

    /** Create a list row for a folder */
    _createFolderItem(folder) {
        const el = document.createElement('div');
        el.className = 'file-item';
        el.dataset.folderId  = folder.id;
        el.dataset.folderName = folder.name;
        el.dataset.parentId  = folder.parent_id || "";

        const isFav = window.favorites &&
            window.favorites.isFavorite(folder.id, 'folder');
        const formattedDate = window.formatDateTime(folder.modified_at);

        if (window.app.currentPath !== "") {
            el.setAttribute('draggable', 'true');
        }

        el.innerHTML = `
            <div class="list-item-checkbox"><input type="checkbox" class="item-checkbox"></div>
            <div class="name-cell">
                <div class="file-icon folder-icon">
                    <i class="fas fa-folder"></i>
                </div>
                <span>${escapeHtml(folder.name)}</span>
                ${isFav ? '<i class="fas fa-star favorite-star-inline"></i>' : ''}
            </div>
            <div class="type-cell">${window.i18n ? window.i18n.t('files.file_types.folder') : 'Folder'}</div>
            <div class="size-cell">--</div>
            <div class="date-cell">${formattedDate}</div>
        `;
        return el;
    },

    /** Create a grid card for a file */
    _createFileCard(file) {
        const iconClass = file.icon_class || 'fas fa-file';
        const iconSpecialClass = file.icon_special_class || '';
        const isFileFav = window.favorites &&
            window.favorites.isFavorite(file.id, 'file');
        const formattedDate = window.formatDateTime(file.modified_at);

        const el = document.createElement('div');
        el.className = 'file-card';
        el.dataset.fileId   = file.id;
        el.dataset.fileName = file.name;
        el.dataset.folderId = file.folder_id || "";
        el.setAttribute('draggable', 'true');

        el.innerHTML = `
            <div class="file-card-checkbox"><i class="fas fa-check"></i></div>
            <button class="file-card-more"><i class="fas fa-ellipsis-v"></i></button>
            ${isFileFav ? '<div class="favorite-star active"><i class="fas fa-star"></i></div>' : ''}
            <div class="file-icon ${iconSpecialClass}">
                <i class="${iconClass}"></i>
            </div>
            <div class="file-name">${escapeHtml(file.name)}</div>
            <div class="file-info">Modified ${formattedDate.split(' ')[0]}</div>
        `;
        return el;
    },

    /** Create a list row for a file */
    _createFileItem(file) {
        const iconClass = file.icon_class || 'fas fa-file';
        const iconSpecialClass = file.icon_special_class || '';
        const cat = file.category || '';
        const typeLabel = cat
            ? (window.i18n
                ? window.i18n.t(`files.file_types.${cat.toLowerCase()}`) || cat
                : cat)
            : (window.i18n
                ? window.i18n.t('files.file_types.document')
                : 'Document');
        const fileSize = file.size_formatted || window.formatFileSize(file.size);
        const formattedDate = window.formatDateTime(file.modified_at);
        const isFileFav = window.favorites &&
            window.favorites.isFavorite(file.id, 'file');

        const el = document.createElement('div');
        el.className = 'file-item';
        el.dataset.fileId   = file.id;
        el.dataset.fileName = file.name;
        el.dataset.folderId = file.folder_id || "";
        el.setAttribute('draggable', 'true');

        el.innerHTML = `
            <div class="list-item-checkbox"><input type="checkbox" class="item-checkbox"></div>
            <div class="name-cell">
                <div class="file-icon ${iconSpecialClass}">
                    <i class="${iconClass}"></i>
                </div>
                <span>${escapeHtml(file.name)}</span>
                ${isFileFav ? '<i class="fas fa-star favorite-star-inline"></i>' : ''}
            </div>
            <div class="type-cell">${typeLabel}</div>
            <div class="size-cell">${fileSize}</div>
            <div class="date-cell">${formattedDate}</div>
        `;
        return el;
    },

    /* ================================================================
     *  Batch rendering with DocumentFragment
     * ================================================================ */

    /**
     * Render an array of folders into both grid and list views
     * using DocumentFragment for minimal reflows.
     */
    renderFolders(folders) {
        if (!this._delegationReady) this.initDelegation();
        const gridFrag = document.createDocumentFragment();
        const listFrag = document.createDocumentFragment();

        for (const folder of folders) {
            this._items.set(folder.id, folder);
            gridFrag.appendChild(this._createFolderCard(folder));
            listFrag.appendChild(this._createFolderItem(folder));
        }

        document.getElementById('files-grid').appendChild(gridFrag);
        document.getElementById('files-list-view').appendChild(listFrag);
    },

    /**
     * Render an array of files into both grid and list views
     * using DocumentFragment for minimal reflows.
     */
    renderFiles(files) {
        if (!this._delegationReady) this.initDelegation();
        const gridFrag = document.createDocumentFragment();
        const listFrag = document.createDocumentFragment();

        for (const file of files) {
            this._items.set(file.id, file);
            gridFrag.appendChild(this._createFileCard(file));
            listFrag.appendChild(this._createFileItem(file));
        }

        document.getElementById('files-grid').appendChild(gridFrag);
        document.getElementById('files-list-view').appendChild(listFrag);
    },

    /* ================================================================
     *  Single-item add (backward-compatible API for post-upload, etc.)
     * ================================================================ */

    /**
     * Add a single folder to both views.
     * @param {Object} folder - Folder object
     */
    addFolderToView(folder) {
        if (!this._delegationReady) this.initDelegation();

        // Duplicate guard
        if (document.querySelector(`.file-card[data-folder-id="${folder.id}"]`) ||
            document.querySelector(`.file-item[data-folder-id="${folder.id}"]`)) {
            console.log(`Folder ${folder.name} (${folder.id}) already exists in the view, not duplicating`);
            return;
        }

        this._items.set(folder.id, folder);
        document.getElementById('files-grid')
            .appendChild(this._createFolderCard(folder));
        document.getElementById('files-list-view')
            .appendChild(this._createFolderItem(folder));
    },

    /**
     * Add a single file to both views.
     * @param {Object} file - File object
     */
    addFileToView(file) {
        if (!this._delegationReady) this.initDelegation();

        // Duplicate guard
        if (document.querySelector(`.file-card[data-file-id="${file.id}"]`) ||
            document.querySelector(`.file-item[data-file-id="${file.id}"]`)) {
            console.log(`File ${file.name} (${file.id}) already exists in the view, not duplicating`);
            return;
        }

        this._items.set(file.id, file);
        document.getElementById('files-grid')
            .appendChild(this._createFileCard(file));
        document.getElementById('files-list-view')
            .appendChild(this._createFileItem(file));
    }
};

// --- Global helper functions for card interactions ---

/**
 * Toggle selection state of a file/folder card.
 * Routes through the multiSelect module so batch actions know about selected items.
 */
function toggleCardSelection(card, event) {
    if (window.multiSelect) {
        window.multiSelect.handleItemClick(card, event);
    } else {
        card.classList.toggle('selected');
    }
}

/**
 * Show the context menu anchored next to a trigger element (the 3-dot button).
 */
function showContextMenuAtElement(triggerElement, menuId) {
    // Hide any open menus first
    document.querySelectorAll('.context-menu').forEach(m => m.style.display = 'none');

    const menu = document.getElementById(menuId);
    if (!menu) return;

    const rect = triggerElement.getBoundingClientRect();
    const menuWidth = 200; // approximate

    // Position below the trigger, aligned to the right edge
    let left = rect.right - menuWidth + window.scrollX;
    let top = rect.bottom + 4 + window.scrollY;

    // Keep inside viewport
    if (left < 8) left = 8;
    if (top + 300 > window.innerHeight + window.scrollY) {
        top = rect.top - 4 + window.scrollY; // flip above if no room
    }

    menu.style.left = `${left}px`;
    menu.style.top = `${top}px`;
    menu.style.display = 'block';
}

/**
 * Rubber band (lasso) selection — click + drag on empty grid area
 * to draw a rectangle and select all cards it touches.
 */
function initRubberBandSelection() {
    // Create the visual rectangle element once
    let selRect = document.getElementById('selection-rect');
    if (!selRect) {
        selRect = document.createElement('div');
        selRect.id = 'selection-rect';
        selRect.className = 'selection-rect';
        document.body.appendChild(selRect);
    }

    let active = false;
    let startX = 0, startY = 0;

    // We listen on the whole files-container (covers grid + empty space)
    const container = document.querySelector('.files-container') || document.getElementById('files-grid');
    if (!container) return;

    container.addEventListener('mousedown', (e) => {
        // Only start if clicking empty area (not on a card, button, menu, input…)
        if (e.button !== 0) return; // left click only
        if (e.target.closest('.file-card') || e.target.closest('.context-menu') ||
            e.target.closest('.upload-dropdown') || e.target.closest('button') ||
            e.target.closest('input') || e.target.closest('.breadcrumb')) return;

        active = true;
        startX = e.clientX;
        startY = e.clientY;

        selRect.style.left = `${startX}px`;
        selRect.style.top = `${startY}px`;
        selRect.style.width = '0px';
        selRect.style.height = '0px';
        selRect.style.display = 'none'; // show only after a small movement

        e.preventDefault(); // prevent text selection
    });

    document.addEventListener('mousemove', (e) => {
        if (!active) return;

        const curX = e.clientX;
        const curY = e.clientY;

        const left = Math.min(startX, curX);
        const top = Math.min(startY, curY);
        const width = Math.abs(curX - startX);
        const height = Math.abs(curY - startY);

        // Only show the rect after a small threshold to avoid flicker on click
        if (width > 5 || height > 5) {
            selRect.style.display = 'block';
        }

        selRect.style.left = `${left}px`;
        selRect.style.top = `${top}px`;
        selRect.style.width = `${width}px`;
        selRect.style.height = `${height}px`;

        // Highlight cards that intersect with the rectangle
        const rectBounds = { left, top, right: left + width, bottom: top + height };

        document.querySelectorAll('#files-grid .file-card').forEach(card => {
            const cardRect = card.getBoundingClientRect();
            const intersects =
                cardRect.left < rectBounds.right &&
                cardRect.right > rectBounds.left &&
                cardRect.top < rectBounds.bottom &&
                cardRect.bottom > rectBounds.top;

            if (intersects) {
                card.classList.add('selected');
                // Sync with multiSelect module
                if (window.multiSelect) {
                    const info = window.multiSelect._extractInfo(card);
                    if (info) window.multiSelect.select(info.id, info.name, info.type, info.parentId);
                }
            } else {
                card.classList.remove('selected');
                // Deselect from multiSelect module
                if (window.multiSelect) {
                    const info = window.multiSelect._extractInfo(card);
                    if (info) window.multiSelect.deselect(info.id);
                }
            }
        });
    });

    document.addEventListener('mouseup', () => {
        if (!active) return;
        active = false;
        selRect.style.display = 'none';
        // Update the batch bar after rubber band selection completes
        if (window.multiSelect) window.multiSelect._syncUI();
    });
}

// Initialize rubber band once DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initRubberBandSelection);
} else {
    initRubberBandSelection();
}

// Expose helpers globally
window.toggleCardSelection = toggleCardSelection;
window.showContextMenuAtElement = showContextMenuAtElement;
window.initRubberBandSelection = initRubberBandSelection;

/**
 * Show a modern confirm dialog (replaces native confirm())
 * @param {Object} options
 * @param {string} options.title - Dialog title
 * @param {string} options.message - Dialog message/body
 * @param {string} [options.confirmText='Confirmar'] - Text for confirm button
 * @param {string} [options.cancelText='Cancelar'] - Text for cancel button
 * @param {boolean} [options.danger=false] - Use danger styling (red)
 * @returns {Promise<boolean>} true if confirmed, false if cancelled
 */
function showConfirmDialog({ title, message, confirmText, cancelText, danger = true } = {}) {
    const ct = confirmText || (window.i18n ? window.i18n.t('actions.delete') : 'Delete');
    const cc = cancelText || (window.i18n ? window.i18n.t('actions.cancel') : 'Cancel');
    const t = title || (window.i18n ? window.i18n.t('dialogs.confirm_title') : 'Confirm action');

    return new Promise((resolve) => {
        // Remove any previous confirm dialog
        const prev = document.getElementById('confirm-dialog-overlay');
        if (prev) prev.remove();

        const overlay = document.createElement('div');
        overlay.id = 'confirm-dialog-overlay';
        overlay.className = 'confirm-dialog';
        overlay.innerHTML = `
            <div class="confirm-dialog-content">
                <div class="confirm-dialog-icon">
                    <i class="fas ${danger ? 'fa-exclamation-triangle' : 'fa-question-circle'}"></i>
                </div>
                <div class="confirm-dialog-title">${t}</div>
                <div class="confirm-dialog-message">${message || ''}</div>
                <div class="confirm-dialog-buttons">
                    <button class="btn btn-secondary confirm-dialog-cancel">${cc}</button>
                    <button class="btn ${danger ? 'btn-danger' : 'btn-primary'} confirm-dialog-ok">${ct}</button>
                </div>
            </div>
        `;
        document.body.appendChild(overlay);

        // Force layout then show
        requestAnimationFrame(() => { overlay.classList.add('active'); });

        const cleanup = (result) => {
            overlay.classList.remove('active');
            setTimeout(() => overlay.remove(), 200);
            resolve(result);
        };

        overlay.querySelector('.confirm-dialog-cancel').addEventListener('click', () => cleanup(false));
        overlay.querySelector('.confirm-dialog-ok').addEventListener('click', () => cleanup(true));
        overlay.addEventListener('click', (e) => { if (e.target === overlay) cleanup(false); });
    });
}
window.showConfirmDialog = showConfirmDialog;

// Expose UI module globally
window.ui = ui;
