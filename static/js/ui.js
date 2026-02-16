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
            dropzone.classList.remove('active');
            if (e.dataTransfer.files.length > 0) {
                fileOps.uploadFiles(e.dataTransfer.files);
            }
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

            if (e.dataTransfer.files.length > 0) {
                fileOps.uploadFiles(e.dataTransfer.files);
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
        if (file.mime_type.startsWith('text/')) return true;
        const textTypes = [
            'application/json', 'application/xml', 'application/javascript',
            'application/x-sh', 'application/x-yaml', 'application/toml',
            'application/x-toml', 'application/sql',
        ];
        return textTypes.includes(file.mime_type);
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

    /**
     * Update file icons based on file type
     */
    updateFileIcons() {
        const fileCards = document.querySelectorAll('.file-card');

        fileCards.forEach(card => {
            const fileName = card.querySelector('.file-name')?.textContent || '';
            const iconElement = card.querySelector('.file-icon');
            if (!iconElement) return;

            if (iconElement.classList.contains('folder-icon')) {
                iconElement.innerHTML = '';
                return;
            }

            const extension = fileName.includes('.') ? fileName.split('.').pop().toLowerCase() : '';
            
            // Map extensions to icon types
            const iconMap = {
                // Documents
                pdf:   { cls: 'pdf-icon', fa: 'fas fa-file-pdf' },
                doc:   { cls: 'doc-icon', fa: 'fas fa-file-word' },
                docx:  { cls: 'doc-icon', fa: 'fas fa-file-word' },
                txt:   { cls: 'doc-icon', fa: 'fas fa-file-alt' },
                rtf:   { cls: 'doc-icon', fa: 'fas fa-file-alt' },
                odt:   { cls: 'doc-icon', fa: 'fas fa-file-alt' },
                // Spreadsheets
                xlsx:  { cls: 'spreadsheet-icon' },
                xls:   { cls: 'spreadsheet-icon' },
                csv:   { cls: 'spreadsheet-icon' },
                ods:   { cls: 'spreadsheet-icon' },
                // Presentations
                pptx:  { cls: 'presentation-icon' },
                ppt:   { cls: 'presentation-icon' },
                odp:   { cls: 'presentation-icon' },
                // Images
                jpg:   { cls: 'image-icon' },
                jpeg:  { cls: 'image-icon' },
                png:   { cls: 'image-icon' },
                gif:   { cls: 'image-icon' },
                svg:   { cls: 'image-icon' },
                webp:  { cls: 'image-icon' },
                bmp:   { cls: 'image-icon' },
                ico:   { cls: 'image-icon' },
                // Videos
                mp4:   { cls: 'video-icon' },
                avi:   { cls: 'video-icon' },
                mov:   { cls: 'video-icon' },
                mkv:   { cls: 'video-icon' },
                webm:  { cls: 'video-icon' },
                flv:   { cls: 'video-icon' },
                // Audio
                mp3:   { cls: 'audio-icon' },
                wav:   { cls: 'audio-icon' },
                ogg:   { cls: 'audio-icon' },
                flac:  { cls: 'audio-icon' },
                aac:   { cls: 'audio-icon' },
                m4a:   { cls: 'audio-icon' },
                // Archives
                zip:   { cls: 'archive-icon' },
                rar:   { cls: 'archive-icon' },
                '7z':  { cls: 'archive-icon' },
                tar:   { cls: 'archive-icon' },
                gz:    { cls: 'archive-icon' },
                bz2:   { cls: 'archive-icon' },
                // Installers
                dmg:   { cls: 'installer-icon' },
                exe:   { cls: 'installer-icon' },
                msi:   { cls: 'installer-icon' },
                deb:   { cls: 'installer-icon' },
                rpm:   { cls: 'installer-icon' },
                pkg:   { cls: 'installer-icon' },
                app:   { cls: 'installer-icon' },
                // Scripts
                sh:    { cls: 'script-icon', fa: 'fas fa-terminal' },
                bash:  { cls: 'script-icon', fa: 'fas fa-terminal' },
                zsh:   { cls: 'script-icon', fa: 'fas fa-terminal' },
                bat:   { cls: 'script-icon', fa: 'fas fa-terminal' },
                ps1:   { cls: 'script-icon', fa: 'fas fa-terminal' },
                // Code — each with sub-type
                json:  { cls: 'code-icon', sub: 'json-icon' },
                js:    { cls: 'code-icon', sub: 'js-icon' },
                jsx:   { cls: 'code-icon', sub: 'js-icon' },
                ts:    { cls: 'code-icon', sub: 'ts-icon' },
                tsx:   { cls: 'code-icon', sub: 'ts-icon' },
                html:  { cls: 'code-icon', sub: 'html-icon' },
                htm:   { cls: 'code-icon', sub: 'html-icon' },
                css:   { cls: 'code-icon', sub: 'css-icon' },
                scss:  { cls: 'code-icon', sub: 'css-icon' },
                py:    { cls: 'code-icon', sub: 'py-icon' },
                rs:    { cls: 'code-icon', sub: 'rust-icon' },
                go:    { cls: 'code-icon', sub: 'go-icon' },
                java:  { cls: 'code-icon', sub: 'java-icon' },
                c:     { cls: 'code-icon', sub: 'c-icon' },
                cpp:   { cls: 'code-icon', sub: 'c-icon' },
                cs:    { cls: 'code-icon', sub: 'cs-icon' },
                php:   { cls: 'code-icon', sub: 'php-icon' },
                rb:    { cls: 'code-icon', sub: 'ruby-icon' },
                swift: { cls: 'code-icon', sub: 'swift-icon' },
                kt:    { cls: 'code-icon', sub: 'kotlin-icon' },
                sql:   { cls: 'code-icon', sub: 'sql-icon' },
                yaml:  { cls: 'code-icon', sub: 'yaml-icon' },
                yml:   { cls: 'code-icon', sub: 'yaml-icon' },
                toml:  { cls: 'code-icon', sub: 'toml-icon' },
                xml:   { cls: 'code-icon', sub: 'html-icon' },
                md:    { cls: 'code-icon', sub: 'md-icon' },
                // Config
                ini:   { cls: 'config-icon', fa: 'fas fa-cog' },
                cfg:   { cls: 'config-icon', fa: 'fas fa-cog' },
                conf:  { cls: 'config-icon', fa: 'fas fa-cog' },
                env:   { cls: 'config-icon', fa: 'fas fa-cog' },
            };

            const mapping = iconMap[extension];
            if (mapping) {
                iconElement.className = `file-icon ${mapping.cls}`;
                if (mapping.cls === 'code-icon') {
                    // Code icons use pseudo-element lines
                    iconElement.innerHTML = `
                        <div class="code-line-1"></div>
                        <div class="code-line-2"></div>
                        <div class="code-line-3"></div>
                    `;
                    if (mapping.sub) iconElement.classList.add(mapping.sub);
                } else {
                    // Types with pure CSS visuals — clear the <i>
                    const pureCssTypes = ['image-icon','video-icon','spreadsheet-icon','presentation-icon','audio-icon','archive-icon','installer-icon'];
                    if (pureCssTypes.includes(mapping.cls)) {
                        iconElement.innerHTML = '';
                    } else if (mapping.fa) {
                        // Types that keep the FA icon — update <i> class
                        let iEl = iconElement.querySelector('i');
                        if (!iEl) {
                            iEl = document.createElement('i');
                            iconElement.innerHTML = '';
                            iconElement.appendChild(iEl);
                        }
                        iEl.className = mapping.fa;
                    }
                }
            }
        });
    },

    /**
     * Add folder to the view
     * @param {Object} folder - Folder object
     */
    addFolderToView(folder) {
        // Check if the folder already exists in the view to avoid duplicates
        if (document.querySelector(`.file-card[data-folder-id="${folder.id}"]`) || 
            document.querySelector(`.file-item[data-folder-id="${folder.id}"]`)) {
            console.log(`Folder ${folder.name} (${folder.id}) already exists in the view, not duplicating`);
            return;
        }
        
        console.log(`Adding folder to the view: ${folder.name} (${folder.id})`);
        
        // Grid view element
        const folderGridElement = document.createElement('div');
        folderGridElement.className = 'file-card';
        folderGridElement.dataset.folderId = folder.id;
        folderGridElement.dataset.folderName = folder.name;
        folderGridElement.dataset.parentId = folder.parent_id || "";

        // Check if folder is a favorite
        const isFolderFav = window.favorites && window.favorites.isFavorite(folder.id, 'folder');

        folderGridElement.innerHTML = `
            <div class="file-card-checkbox"><i class="fas fa-check"></i></div>
            <button class="file-card-more"><i class="fas fa-ellipsis-v"></i></button>
            ${isFolderFav ? '<div class="favorite-star active"><i class="fas fa-star"></i></div>' : ''}
            <div class="file-icon folder-icon">
                <i class="fas fa-folder"></i>
            </div>
            <div class="file-name">${escapeHtml(folder.name)}</div>
            <div class="file-info">Folder</div>
        `;

        // Drag and drop setup for folders
        if (window.app.currentPath !== "") {
            folderGridElement.setAttribute('draggable', 'true');

            folderGridElement.addEventListener('dragstart', (e) => {
                if (!folderGridElement.classList.contains('selected')) {
                    e.preventDefault();
                    return;
                }
                e.dataTransfer.setData('text/plain', folder.id);
                e.dataTransfer.setData('application/oxicloud-folder', 'true');
                folderGridElement.classList.add('dragging');
            });

            folderGridElement.addEventListener('dragend', () => {
                folderGridElement.classList.remove('dragging');
                document.querySelectorAll('.drop-target').forEach(el => {
                    el.classList.remove('drop-target');
                });
            });
        }

        // Single click to select, double click to navigate
        folderGridElement.addEventListener('click', (e) => {
            if (e.target.closest('.file-card-more') || e.target.closest('.file-card-checkbox')) return;
            toggleCardSelection(folderGridElement, e);
        });

        folderGridElement.addEventListener('dblclick', () => {
            window.app.currentPath = folder.id;
            this.updateBreadcrumb(folder.name);
            window.loadFiles();
        });

        // Checkbox click
        folderGridElement.querySelector('.file-card-checkbox').addEventListener('click', (e) => {
            e.stopPropagation();
            toggleCardSelection(folderGridElement, e);
        });

        // More actions button
        folderGridElement.querySelector('.file-card-more').addEventListener('click', (e) => {
            e.stopPropagation();
            e.preventDefault();
            window.app.contextMenuTargetFolder = {
                id: folder.id,
                name: folder.name,
                parent_id: folder.parent_id || ""
            };
            showContextMenuAtElement(e.currentTarget, 'folder-context-menu');
        });

        // Context menu
        folderGridElement.addEventListener('contextmenu', (e) => {
            e.preventDefault();

            window.app.contextMenuTargetFolder = {
                id: folder.id,
                name: folder.name,
                parent_id: folder.parent_id || ""
            };

            let folderContextMenu = document.getElementById('folder-context-menu');
            folderContextMenu.style.left = `${e.pageX}px`;
            folderContextMenu.style.top = `${e.pageY}px`;
            folderContextMenu.style.display = 'block';
        });

        // Drop target setup
        folderGridElement.addEventListener('dragover', (e) => {
            e.preventDefault();
            folderGridElement.classList.add('drop-target');
        });

        folderGridElement.addEventListener('dragleave', () => {
            folderGridElement.classList.remove('drop-target');
        });

        folderGridElement.addEventListener('drop', async (e) => {
            e.preventDefault();
            folderGridElement.classList.remove('drop-target');

            const id = e.dataTransfer.getData('text/plain');
            const isFolder = e.dataTransfer.getData('application/oxicloud-folder') === 'true';

            if (id) {
                if (isFolder) {
                    if (id === folder.id) {
                        alert("You cannot move a folder to itself");
                        return;
                    }
                    await fileOps.moveFolder(id, folder.id);
                } else {
                    await fileOps.moveFile(id, folder.id);
                }
            }
        });

        document.getElementById('files-grid').appendChild(folderGridElement);

        // List view element - Improved
        const folderListElement = document.createElement('div');
        folderListElement.className = 'file-item';
        folderListElement.dataset.folderId = folder.id;
        folderListElement.dataset.folderName = folder.name;
        folderListElement.dataset.parentId = folder.parent_id || "";

        // Format date
        const modifiedDate = new Date(folder.modified_at * 1000);
        const formattedDate = modifiedDate.toLocaleDateString() + ' ' +
                             modifiedDate.toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'});

        // Make draggable if not in root
        if (window.app.currentPath !== "") {
            folderListElement.setAttribute('draggable', 'true');

            folderListElement.addEventListener('dragstart', (e) => {
                e.dataTransfer.setData('text/plain', folder.id);
                e.dataTransfer.setData('application/oxicloud-folder', 'true');
                folderListElement.classList.add('dragging');
            });

            folderListElement.addEventListener('dragend', () => {
                folderListElement.classList.remove('dragging');
                document.querySelectorAll('.drop-target').forEach(el => {
                    el.classList.remove('drop-target');
                });
            });
        }

        // Improved: Structure and classes for list view
        folderListElement.innerHTML = `
            <div class="list-item-checkbox"><input type="checkbox" class="item-checkbox"></div>
            <div class="name-cell">
                <div class="file-icon folder-icon">
                    <i class="fas fa-folder"></i>
                </div>
                <span>${escapeHtml(folder.name)}</span>
                ${isFolderFav ? '<i class="fas fa-star favorite-star-inline"></i>' : ''}
            </div>
            <div class="type-cell">${window.i18n ? window.i18n.t('files.file_types.folder') : 'Folder'}</div>
            <div class="size-cell">--</div>
            <div class="date-cell">${formattedDate}</div>
        `;

        // Checkbox click in list view
        folderListElement.querySelector('.item-checkbox').addEventListener('click', (e) => {
            e.stopPropagation();
            toggleCardSelection(folderListElement, e);
        });

        // Click to navigate
        folderListElement.addEventListener('click', (e) => {
            if (e.target.closest('.list-item-checkbox')) return;
            window.app.currentPath = folder.id;
            this.updateBreadcrumb(folder.name);
            window.loadFiles();
        });

        // Context menu
        folderListElement.addEventListener('contextmenu', (e) => {
            e.preventDefault();

            window.app.contextMenuTargetFolder = {
                id: folder.id,
                name: folder.name,
                parent_id: folder.parent_id || ""
            };

            let folderContextMenu = document.getElementById('folder-context-menu');
            folderContextMenu.style.left = `${e.pageX}px`;
            folderContextMenu.style.top = `${e.pageY}px`;
            folderContextMenu.style.display = 'block';
        });

        // Drop target setup for list view
        folderListElement.addEventListener('dragover', (e) => {
            e.preventDefault();
            folderListElement.classList.add('drop-target');
        });

        folderListElement.addEventListener('dragleave', () => {
            folderListElement.classList.remove('drop-target');
        });

        folderListElement.addEventListener('drop', async (e) => {
            e.preventDefault();
            folderListElement.classList.remove('drop-target');

            const id = e.dataTransfer.getData('text/plain');
            const isFolder = e.dataTransfer.getData('application/oxicloud-folder') === 'true';

            if (id) {
                if (isFolder) {
                    if (id === folder.id) {
                        alert("You cannot move a folder to itself");
                        return;
                    }
                    await fileOps.moveFolder(id, folder.id);
                } else {
                    await fileOps.moveFile(id, folder.id);
                }
            }
        });

        document.getElementById('files-list-view').appendChild(folderListElement);
    },

    /**
     * Add file to the view
     * @param {Object} file - File object
     */
    addFileToView(file) {
        // Check if the file already exists in the view to avoid duplicates
        if (document.querySelector(`.file-card[data-file-id="${file.id}"]`) ||
            document.querySelector(`.file-item[data-file-id="${file.id}"]`)) {
            console.log(`File ${file.name} (${file.id}) already exists in the view, not duplicating`);
            return;
        }
        
        console.log(`Adding file to the view: ${file.name} (${file.id})`);
        
        // Use pre-computed display fields from the API response
        const iconClass = file.icon_class || 'fas fa-file';
        const iconSpecialClass = file.icon_special_class || '';
        const typeLabel = file.category
            ? (window.i18n ? window.i18n.t(`files.file_types.${file.category.toLowerCase()}`) || file.category : file.category)
            : (window.i18n ? window.i18n.t('files.file_types.document') : 'Document');

        // Format size and date
        const fileSize = file.size_formatted || window.formatFileSize(file.size);
        const modifiedDate = new Date(file.modified_at * 1000);
        const formattedDate = modifiedDate.toLocaleDateString() + ' ' +
                             modifiedDate.toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'});

        // Grid view element
        const fileGridElement = document.createElement('div');
        fileGridElement.className = 'file-card';

        // Check if file is a favorite
        const isFileFav = window.favorites && window.favorites.isFavorite(file.id, 'file');

        fileGridElement.innerHTML = `
            <div class="file-card-checkbox"><i class="fas fa-check"></i></div>
            <button class="file-card-more"><i class="fas fa-ellipsis-v"></i></button>
            ${isFileFav ? '<div class="favorite-star active"><i class="fas fa-star"></i></div>' : ''}
            <div class="file-icon">
                <i class="${iconClass}"></i>
            </div>
            <div class="file-name">${escapeHtml(file.name)}</div>
            <div class="file-info">Modified ${formattedDate.split(' ')[0]}</div>
        `;

        fileGridElement.dataset.fileId = file.id;
        fileGridElement.dataset.fileName = file.name;
        fileGridElement.dataset.folderId = file.folder_id || "";

        // Make draggable
        fileGridElement.setAttribute('draggable', 'true');

        fileGridElement.addEventListener('dragstart', (e) => {
            if (!fileGridElement.classList.contains('selected')) {
                e.preventDefault();
                return;
            }
            e.dataTransfer.setData('text/plain', file.id);
            fileGridElement.classList.add('dragging');
        });

        fileGridElement.addEventListener('dragend', () => {
            fileGridElement.classList.remove('dragging');
            document.querySelectorAll('.drop-target').forEach(el => {
                el.classList.remove('drop-target');
            });
        });

        // Single click = select, double click = open/download
        fileGridElement.addEventListener('click', (e) => {
            if (e.target.closest('.file-card-more') || e.target.closest('.file-card-checkbox')) return;
            toggleCardSelection(fileGridElement, e);
        });

        fileGridElement.addEventListener('dblclick', () => {
            // Track this file access for recent files
            if (window.recent) {
                document.dispatchEvent(new CustomEvent('file-accessed', {
                    detail: { file }
                }));
            }
            
            // Check if it's a viewable file type
            if (this.isViewableFile(file)) {
                if (window.inlineViewer) {
                    window.inlineViewer.openFile(file);
                } else if (window.fileViewer) {
                    window.fileViewer.open(file);
                } else {
                    window.fileOps.downloadFile(file.id, file.name);
                }
            } else {
                window.fileOps.downloadFile(file.id, file.name);
            }
        });

        // Checkbox click
        fileGridElement.querySelector('.file-card-checkbox').addEventListener('click', (e) => {
            e.stopPropagation();
            toggleCardSelection(fileGridElement, e);
        });

        // More actions button
        fileGridElement.querySelector('.file-card-more').addEventListener('click', (e) => {
            e.stopPropagation();
            e.preventDefault();
            window.app.contextMenuTargetFile = {
                id: file.id,
                name: file.name,
                folder_id: file.folder_id || ""
            };
            showContextMenuAtElement(e.currentTarget, 'file-context-menu');
        });

        // Context menu
        fileGridElement.addEventListener('contextmenu', (e) => {
            e.preventDefault();

            window.app.contextMenuTargetFile = {
                id: file.id,
                name: file.name,
                folder_id: file.folder_id || ""
            };

            let fileContextMenu = document.getElementById('file-context-menu');
            fileContextMenu.style.left = `${e.pageX}px`;
            fileContextMenu.style.top = `${e.pageY}px`;
            fileContextMenu.style.display = 'block';
        });

        document.getElementById('files-grid').appendChild(fileGridElement);

        // List view element - Improved with specific classes and enhanced layout
        const fileListElement = document.createElement('div');
        fileListElement.className = 'file-item';
        fileListElement.dataset.fileId = file.id;
        fileListElement.dataset.fileName = file.name;
        fileListElement.dataset.folderId = file.folder_id || "";

        fileListElement.innerHTML = `
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

        // Checkbox click in list view
        fileListElement.querySelector('.item-checkbox').addEventListener('click', (e) => {
            e.stopPropagation();
            toggleCardSelection(fileListElement, e);
        });

        // Make draggable (list view)
        fileListElement.setAttribute('draggable', 'true');

        fileListElement.addEventListener('dragstart', (e) => {
            e.dataTransfer.setData('text/plain', file.id);
            fileListElement.classList.add('dragging');
        });

        fileListElement.addEventListener('dragend', () => {
            fileListElement.classList.remove('dragging');
            document.querySelectorAll('.drop-target').forEach(el => {
                el.classList.remove('drop-target');
            });
        });

        // View or download on click
        fileListElement.addEventListener('click', (e) => {
            if (e.target.closest('.list-item-checkbox')) return;
            // Track this file access for recent files
            if (window.recent) {
                document.dispatchEvent(new CustomEvent('file-accessed', {
                    detail: { file }
                }));
            }
            
            // Check if it's a viewable file type
            if (this.isViewableFile(file)) {
                // Open in the inline viewer
                if (window.inlineViewer) {
                    window.inlineViewer.openFile(file);
                } else if (window.fileViewer) {
                    // Fallback to standard file viewer
                    window.fileViewer.open(file);
                } else {
                    // No viewer available, download directly
                    window.fileOps.downloadFile(file.id, file.name);
                }
            } else {
                // For other file types, download directly
                window.fileOps.downloadFile(file.id, file.name);
            }
        });

        // Context menu (list view)
        fileListElement.addEventListener('contextmenu', (e) => {
            e.preventDefault();

            window.app.contextMenuTargetFile = {
                id: file.id,
                name: file.name,
                folder_id: file.folder_id || ""
            };

            let fileContextMenu = document.getElementById('file-context-menu');
            fileContextMenu.style.left = `${e.pageX}px`;
            fileContextMenu.style.top = `${e.pageY}px`;
            fileContextMenu.style.display = 'block';
        });

        document.getElementById('files-list-view').appendChild(fileListElement);
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
