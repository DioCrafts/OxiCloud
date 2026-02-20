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

        const collectDroppedEntries = async (dataTransfer) => {
            const items = Array.from(dataTransfer?.items || []);
            const rootEntries = items
                .map(it => (typeof it.webkitGetAsEntry === 'function' ? it.webkitGetAsEntry() : null))
                .filter(Boolean);

            if (rootEntries.length === 0) return null;

            const out = [];

            const walkEntry = async (entry, prefix = '') => {
                if (!entry) return;

                if (entry.isFile) {
                    await new Promise((resolve) => {
                        entry.file(
                            (file) => {
                                out.push({ file, relativePath: `${prefix}${file.name}` });
                                resolve();
                            },
                            () => resolve()
                        );
                    });
                    return;
                }

                if (entry.isDirectory) {
                    const dirPrefix = `${prefix}${entry.name}/`;
                    const reader = entry.createReader();

                    while (true) {
                        const children = await new Promise((resolve) => {
                            reader.readEntries(resolve, () => resolve([]));
                        });
                        if (!children || children.length === 0) break;
                        for (const child of children) {
                            // eslint-disable-next-line no-await-in-loop
                            await walkEntry(child, dirPrefix);
                        }
                    }
                }
            };

            for (const root of rootEntries) {
                // eslint-disable-next-line no-await-in-loop
                await walkEntry(root, '');
            }

            return out;
        };

        // Dropzone events
        dropzone.addEventListener('dragover', (e) => {
            e.preventDefault();
            dropzone.classList.add('active');
        });

        dropzone.addEventListener('dragleave', () => {
            dropzone.classList.remove('active');
        });

        dropzone.addEventListener('drop', async (e) => {
            e.preventDefault();
            e.stopPropagation(); // Prevent bubbling to document's drop handler (avoids double upload)
            e._oxiHandled = true;  // Mark as handled for document-level fallback
            dropzone.classList.remove('active');
            if (e.dataTransfer.files.length > 0) {
                // First try directory-aware extraction (Finder folder drag & drop)
                const droppedEntries = await collectDroppedEntries(e.dataTransfer);
                if (droppedEntries && droppedEntries.length > 0) {
                    const hasFolderStructure = droppedEntries.some(x => x.relativePath && x.relativePath.includes('/'));
                    if (hasFolderStructure) {
                        fileOps.uploadFolderEntries(droppedEntries);
                    } else {
                        fileOps.uploadFiles(droppedEntries.map(x => x.file));
                    }
                    setTimeout(() => {
                        dropzone.style.display = 'none';
                    }, 500);
                    return;
                }

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

        document.addEventListener('drop', async (e) => {
            e.preventDefault();
            dropzone.classList.remove('active');

            // Skip if already handled by the dropzone handler (defensive against bubble leaks)
            if (e._oxiHandled) return;

            if (e.dataTransfer.files.length > 0) {
                // First try directory-aware extraction (Finder folder drag & drop)
                const droppedEntries = await collectDroppedEntries(e.dataTransfer);
                if (droppedEntries && droppedEntries.length > 0) {
                    const hasFolderStructure = droppedEntries.some(x => x.relativePath && x.relativePath.includes('/'));
                    if (hasFolderStructure) {
                        fileOps.uploadFolderEntries(droppedEntries);
                    } else {
                        fileOps.uploadFiles(droppedEntries.map(x => x.file));
                    }
                    setTimeout(() => {
                        dropzone.style.display = 'none';
                    }, 500);
                    return;
                }

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

        this._hydrateViewIfNeeded('grid');

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

        this._hydrateViewIfNeeded('list');

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
        return window.uiFileTypes.isViewableFile(file);
    },

    /**
     * Get FontAwesome icon class for a filename based on its extension.
     * Used as fallback when the backend DTO doesn't include icon_class
     * (e.g. trash items).
     */
    getIconClass(fileName) {
        return window.uiFileTypes.getIconClass(fileName);
    },

    /**
     * Get CSS special class for icon styling based on filename extension.
     * Used as fallback when the backend DTO doesn't include icon_special_class.
     */
    getIconSpecialClass(fileName) {
        return window.uiFileTypes.getIconSpecialClass(fileName);
    },

    /**
     * Show notification
     * @param {string} title - Notification title
     * @param {string} message - Notification message
     */
    showNotification(title, message) {
        window.uiNotifications.show(title, message);
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

    /** @type {Array<Object>} last rendered folder dataset */
    _lastFolders: [],

    /** @type {Array<Object>} last rendered file dataset */
    _lastFiles: [],

    /** @type {boolean} */
    _delegationReady: false,

    _getActiveView() {
        if (window.app && window.app.currentView === 'list') return 'list';
        if (window.app && window.app.currentView === 'grid') return 'grid';

        const stored = localStorage.getItem('oxicloud-view');
        return stored === 'list' ? 'list' : 'grid';
    },

    _renderFoldersToView(folders, view) {
        if (!Array.isArray(folders) || folders.length === 0) return;
        const target = view === 'list'
            ? document.getElementById('files-list-view')
            : document.getElementById('files-grid');
        if (!target) return;

        const frag = document.createDocumentFragment();
        for (const folder of folders) {
            frag.appendChild(view === 'list'
                ? this._createFolderItem(folder)
                : this._createFolderCard(folder));
        }
        target.appendChild(frag);
    },

    _renderFilesToView(files, view) {
        if (!Array.isArray(files) || files.length === 0) return;
        const target = view === 'list'
            ? document.getElementById('files-list-view')
            : document.getElementById('files-grid');
        if (!target) return;

        const frag = document.createDocumentFragment();
        for (const file of files) {
            frag.appendChild(view === 'list'
                ? this._createFileItem(file)
                : this._createFileCard(file));
        }
        target.appendChild(frag);
    },

    _upsertById(arr, item) {
        if (!Array.isArray(arr) || !item || !item.id) return;
        const idx = arr.findIndex(x => x && x.id === item.id);
        if (idx >= 0) {
            arr[idx] = item;
        } else {
            arr.push(item);
        }
    },

    _hydrateViewIfNeeded(view) {
        // Only hydrate if there is at least one rendered item in the opposite/current DOM.
        // This prevents stale cache hydration in empty-state screens.
        const hasAnyRenderedItem = !!document.querySelector('#files-grid .file-card, #files-list-view .file-item');
        if (!hasAnyRenderedItem) return;

        if (view === 'grid') {
            const grid = document.getElementById('files-grid');
            if (!grid) return;
            if (grid.children.length > 0) return;

            this._renderFoldersToView(this._lastFolders, 'grid');
            this._renderFilesToView(this._lastFiles, 'grid');
            return;
        }

        if (view === 'list') {
            const list = document.getElementById('files-list-view');
            if (!list) return;
            // list view keeps a static header row as first child
            if (list.querySelector('.file-item')) return;

            this._renderFoldersToView(this._lastFolders, 'list');
            this._renderFilesToView(this._lastFiles, 'list');
        }
    },

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

        // ── GRID: click (open / navigate; select only via checkbox) ──
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

            // Favorite star – handled by direct onclick on the button
            if (e.target.closest('.favorite-star')) return;

            // Single-click opens/navigates (selection is only via checkbox)
            const info = itemInfo(card);
            if (!info) return;

            if (info.type === 'folder') {
                navigateFolder(card);
            } else {
                openFile(info.data);
            }
        });

        // ── GRID: dblclick (navigate / open) ──────────────────────
        grid.addEventListener('dblclick', (e) => {
            // Single-click already handles open/navigate.
            // Prevent duplicate actions on double-click.
            e.preventDefault();
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
                if (window.contextMenus && typeof window.contextMenus.syncFavoriteOptionLabels === 'function') {
                    window.contextMenus.syncFavoriteOptionLabels();
                }
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
     *  Favorite star helper – attaches a direct click handler to a
     *  star <button> so the event never bubbles to the card.
     * ================================================================ */
    _bindStarClick(el) {
        const star = el.querySelector('.favorite-star');
        if (!star) return;

        star.addEventListener('click', (e) => {
            e.stopPropagation();
            e.stopImmediatePropagation();
            e.preventDefault();

            if (!window.favorites) return;

            const itemId   = star.dataset.itemId;
            const itemType = star.dataset.itemType;
            const itemName = star.dataset.itemName;

            const isActive = star.classList.contains('active');

            if (isActive) {
                this.setFavoriteVisualState(itemId, itemType, false);
                window.favorites.removeFromFavorites(itemId, itemType);
            } else {
                this.setFavoriteVisualState(itemId, itemType, true);
                window.favorites.addToFavorites(itemId, itemName, itemType);
            }

            // Keep context-menu label in sync if available
            if (window.contextMenus && typeof window.contextMenus.syncFavoriteOptionLabels === 'function') {
                window.contextMenus.syncFavoriteOptionLabels();
            }
        });
    },

    /**
     * Sync favorite visuals for a file/folder across grid and list views.
     */
    setFavoriteVisualState(itemId, itemType, isFavorite) {
        const cardSelector = itemType === 'folder'
            ? `.file-card[data-folder-id="${itemId}"]`
            : `.file-card[data-file-id="${itemId}"]`;

        const listSelector = itemType === 'folder'
            ? `.file-item[data-folder-id="${itemId}"]`
            : `.file-item[data-file-id="${itemId}"]`;

        const card = document.querySelector(cardSelector);
        const starBtn = card ? card.querySelector('.favorite-star') : null;

        if (starBtn) {
            starBtn.classList.toggle('active', !!isFavorite);

            // SVG icon path (after icons.js replacement)
            const svg = starBtn.querySelector('svg');
            const filledPath = window.OxiIcons && window.OxiIcons['star'];
            const outlinePath = window.OxiIcons && window.OxiIcons['star-outline'];
            const targetPath = isFavorite ? filledPath : outlinePath;
            if (svg && targetPath) {
                const p = svg.querySelector('path');
                if (p) p.setAttribute('d', targetPath[1]);
                svg.setAttribute('viewBox', `0 0 ${targetPath[0]} 512`);
            }

            // Fallback <i> icon (before icons.js replacement)
            const i = starBtn.querySelector('i');
            if (i) {
                i.classList.remove('fas', 'far');
                i.classList.add(isFavorite ? 'fas' : 'far');
            }
        }

        const listItem = document.querySelector(listSelector);
        if (listItem) {
            const nameCell = listItem.querySelector('.name-cell');
            if (nameCell) {
                let inlineStar = nameCell.querySelector('.favorite-star-inline');
                if (isFavorite && !inlineStar) {
                    inlineStar = document.createElement('i');
                    inlineStar.className = 'fas fa-star favorite-star-inline';
                    nameCell.appendChild(inlineStar);
                    if (window.OxiIcons && typeof window.OxiIcons.replaceIconsInElement === 'function') {
                        window.OxiIcons.replaceIconsInElement(nameCell);
                    }
                } else if (!isFavorite && inlineStar) {
                    inlineStar.remove();
                }
            }
        }
    },

    /* ================================================================
     *  Element-creation helpers
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
            <button class="favorite-star${isFav ? ' active' : ''}" data-item-id="${folder.id}" data-item-type="folder" data-item-name="${escapeHtml(folder.name)}">
                <i class="${isFav ? 'fas' : 'far'} fa-star"></i>
            </button>
            <div class="file-icon folder-icon">
                <i class="fas fa-folder"></i>
            </div>
            <div class="file-name">${escapeHtml(folder.name)}</div>
            <div class="file-info">Folder</div>
        `;

        if (window.app.currentPath !== "") {
            el.setAttribute('draggable', 'true');
        }
        this._bindStarClick(el);
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
        const iconClass = file.icon_class || this.getIconClass(file.name);
        const iconSpecialClass = file.icon_special_class || this.getIconSpecialClass(file.name);
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
            <button class="favorite-star${isFileFav ? ' active' : ''}" data-item-id="${file.id}" data-item-type="file" data-item-name="${escapeHtml(file.name)}">
                <i class="${isFileFav ? 'fas' : 'far'} fa-star"></i>
            </button>
            <div class="file-icon ${iconSpecialClass}">
                <i class="${iconClass}"></i>
            </div>
            <div class="file-name">${escapeHtml(file.name)}</div>
            <div class="file-info">Modified ${formattedDate.split(' ')[0]}</div>
        `;
        this._bindStarClick(el);
        return el;
    },

    /** Create a list row for a file */
    _createFileItem(file) {
        const iconClass = file.icon_class || this.getIconClass(file.name);
        const iconSpecialClass = file.icon_special_class || this.getIconSpecialClass(file.name);
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
        const safeFolders = Array.isArray(folders) ? folders : [];
        this._lastFolders = safeFolders.slice();

        for (const folder of safeFolders) {
            this._items.set(folder.id, folder);
        }

        this._renderFoldersToView(safeFolders, this._getActiveView());
    },

    /**
     * Render an array of files into both grid and list views
     * using DocumentFragment for minimal reflows.
     */
    renderFiles(files) {
        if (!this._delegationReady) this.initDelegation();
        const safeFiles = Array.isArray(files) ? files : [];
        this._lastFiles = safeFiles.slice();

        for (const file of safeFiles) {
            this._items.set(file.id, file);
        }

        this._renderFilesToView(safeFiles, this._getActiveView());
    },

    /* ================================================================
     *  Single-item add (backward-compatible API for post-upload, etc.)
     * ================================================================ */

    /**
     * Add a single folder to the active view.
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
        this._upsertById(this._lastFolders, folder);
        this._renderFoldersToView([folder], this._getActiveView());
    },

    /**
     * Add a single file to the active view.
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
        this._upsertById(this._lastFiles, file);
        this._renderFilesToView([file], this._getActiveView());
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

    if (window.contextMenus && typeof window.contextMenus.syncFavoriteOptionLabels === 'function') {
        window.contextMenus.syncFavoriteOptionLabels();
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
