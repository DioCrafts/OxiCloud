/**
 * OxiCloud - Context Menus and Dialogs Module
 * This file handles context menus and dialog functionality
 */

// Context Menus Module
const contextMenus = {
    /**
     * Assign events to menu items and dialogs
     */
    assignMenuEvents() {
        // Folder context menu options
        document.getElementById('download-folder-option').addEventListener('click', () => {
            if (window.app.contextMenuTargetFolder) {
                window.fileOps.downloadFolder(
                    window.app.contextMenuTargetFolder.id,
                    window.app.contextMenuTargetFolder.name
                );
            }
            window.ui.closeContextMenu();
        });
        
        document.getElementById('favorite-folder-option').addEventListener('click', () => {
            if (window.app.contextMenuTargetFolder) {
                const folder = window.app.contextMenuTargetFolder;
                
                // Check if folder is already in favorites to toggle
                if (window.favorites && window.favorites.isFavorite(folder.id, 'folder')) {
                    // Remove from favorites
                    window.favorites.removeFromFavorites(folder.id, 'folder');
                    // Update menu item text
                    document.getElementById('favorite-folder-option').querySelector('span').textContent = 
                        window.i18n ? window.i18n.t('actions.favorite') : 'Add to favorites';
                } else {
                    // Add to favorites
                    window.favorites.addToFavorites(
                        folder.id,
                        folder.name,
                        'folder',
                        folder.parent_id
                    );
                    // Update menu item text
                    document.getElementById('favorite-folder-option').querySelector('span').textContent = 
                        window.i18n ? window.i18n.t('actions.unfavorite') : 'Remove from favorites';
                }
            }
            window.ui.closeContextMenu();
        });
        
        document.getElementById('rename-folder-option').addEventListener('click', () => {
            if (window.app.contextMenuTargetFolder) {
                this.showRenameDialog(window.app.contextMenuTargetFolder);
            }
            window.ui.closeContextMenu();
        });

        document.getElementById('move-folder-option').addEventListener('click', () => {
            if (window.app.contextMenuTargetFolder) {
                this.showMoveDialog(window.app.contextMenuTargetFolder, 'folder');
            }
            window.ui.closeContextMenu();
        });
        
        document.getElementById('share-folder-option').addEventListener('click', () => {
            if (window.app.contextMenuTargetFolder) {
                this.showShareDialog(window.app.contextMenuTargetFolder, 'folder');
            }
            window.ui.closeContextMenu();
        });

        document.getElementById('delete-folder-option').addEventListener('click', async () => {
            if (window.app.contextMenuTargetFolder) {
                await window.fileOps.deleteFolder(
                    window.app.contextMenuTargetFolder.id, 
                    window.app.contextMenuTargetFolder.name
                );
            }
            window.ui.closeContextMenu();
        });

        // File context menu options
        document.getElementById('view-file-option').addEventListener('click', () => {
            if (window.app.contextMenuTargetFile) {
                // Fetch file details to get the mime type
                const token = localStorage.getItem('oxicloud_token');
                fetch(`/api/files/${window.app.contextMenuTargetFile.id}?metadata=true`, {
                    headers: { 'Authorization': `Bearer ${token}` }
                })
                    .then(response => response.json())
                    .then(fileDetails => {
                        // Check if viewable file type
                        if ((fileDetails.mime_type && fileDetails.mime_type.startsWith('image/')) || 
                            (fileDetails.mime_type && fileDetails.mime_type === 'application/pdf')) {
                            // Open with inline viewer
                            if (window.inlineViewer) {
                                window.inlineViewer.openFile(fileDetails);
                            } else if (window.fileViewer) {
                                window.fileViewer.open(fileDetails);
                            } else {
                                // If no viewer is available, download directly
                                window.fileOps.downloadFile(
                                    window.app.contextMenuTargetFile.id,
                                    window.app.contextMenuTargetFile.name
                                );
                            }
                        } else {
                            // For non-viewable files, download
                            window.fileOps.downloadFile(
                                window.app.contextMenuTargetFile.id,
                                window.app.contextMenuTargetFile.name
                            );
                        }
                    })
                    .catch(error => {
                        console.error('Error fetching file details:', error);
                        // On error, fallback to download
                        window.fileOps.downloadFile(
                            window.app.contextMenuTargetFile.id,
                            window.app.contextMenuTargetFile.name
                        );
                    });
            }
            window.ui.closeFileContextMenu();
        });
        
        document.getElementById('download-file-option').addEventListener('click', () => {
            if (window.app.contextMenuTargetFile) {
                window.fileOps.downloadFile(
                    window.app.contextMenuTargetFile.id,
                    window.app.contextMenuTargetFile.name
                );
            }
            window.ui.closeFileContextMenu();
        });
        
        document.getElementById('favorite-file-option').addEventListener('click', () => {
            if (window.app.contextMenuTargetFile) {
                const file = window.app.contextMenuTargetFile;
                
                // Check if file is already in favorites to toggle
                if (window.favorites && window.favorites.isFavorite(file.id, 'file')) {
                    // Remove from favorites
                    window.favorites.removeFromFavorites(file.id, 'file');
                    // Update menu item text
                    document.getElementById('favorite-file-option').querySelector('span').textContent = 
                        window.i18n ? window.i18n.t('actions.favorite') : 'Add to favorites';
                } else {
                    // Add to favorites
                    window.favorites.addToFavorites(
                        file.id,
                        file.name,
                        'file',
                        file.folder_id
                    );
                    // Update menu item text
                    document.getElementById('favorite-file-option').querySelector('span').textContent = 
                        window.i18n ? window.i18n.t('actions.unfavorite') : 'Remove from favorites';
                }
            }
            window.ui.closeFileContextMenu();
        });
        
        document.getElementById('rename-file-option').addEventListener('click', () => {
            if (window.app.contextMenuTargetFile) {
                this.showRenameFileDialog(window.app.contextMenuTargetFile);
            }
            window.ui.closeFileContextMenu();
        });

        document.getElementById('move-file-option').addEventListener('click', () => {
            if (window.app.contextMenuTargetFile) {
                this.showMoveDialog(window.app.contextMenuTargetFile, 'file');
            }
            window.ui.closeFileContextMenu();
        });

        document.getElementById('share-file-option').addEventListener('click', () => {
            if (window.app.contextMenuTargetFile) {
                this.showShareDialog(window.app.contextMenuTargetFile, 'file');
            }
            window.ui.closeFileContextMenu();
        });

        document.getElementById('delete-file-option').addEventListener('click', async () => {
            if (window.app.contextMenuTargetFile) {
                await window.fileOps.deleteFile(
                    window.app.contextMenuTargetFile.id,
                    window.app.contextMenuTargetFile.name
                );
            }
            window.ui.closeFileContextMenu();
        });

        // Rename dialog events
        const renameCancelBtn = document.getElementById('rename-cancel-btn');
        const renameConfirmBtn = document.getElementById('rename-confirm-btn');
        const renameInput = document.getElementById('rename-input');

        renameCancelBtn.addEventListener('click', this.closeRenameDialog);
        renameConfirmBtn.addEventListener('click', () => contextMenus.renameItem());

        // Rename on Enter key
        renameInput.addEventListener('keyup', (e) => {
            if (e.key === 'Enter') {
                contextMenus.renameItem();
            } else if (e.key === 'Escape') {
                this.closeRenameDialog();
            }
        });

        // Move dialog events
        const moveCancelBtn = document.getElementById('move-cancel-btn');
        const moveConfirmBtn = document.getElementById('move-confirm-btn');

        moveCancelBtn.addEventListener('click', this.closeMoveDialog);
        moveConfirmBtn.addEventListener('click', async () => {
            if (window.app.moveDialogMode === 'file' && window.app.contextMenuTargetFile) {
                const success = await window.fileOps.moveFile(
                    window.app.contextMenuTargetFile.id, 
                    window.app.selectedTargetFolderId
                );
                if (success) {
                    this.closeMoveDialog();
                }
            } else if (window.app.moveDialogMode === 'folder' && window.app.contextMenuTargetFolder) {
                const success = await window.fileOps.moveFolder(
                    window.app.contextMenuTargetFolder.id, 
                    window.app.selectedTargetFolderId
                );
                if (success) {
                    this.closeMoveDialog();
                }
            }
        });
    },

    /**
     * Show rename dialog for a folder
     * @param {Object} folder - Folder object
     */
    showRenameDialog(folder) {
        const renameInput = document.getElementById('rename-input');
        const renameDialog = document.getElementById('rename-dialog');

        window.app.renameMode = 'folder';
        renameInput.value = folder.name;
        // Update header text
        const headerSpan = renameDialog.querySelector('.rename-dialog-header span');
        if (headerSpan) headerSpan.textContent = window.i18n ? window.i18n.t('dialogs.rename_folder') : 'Rename folder';
        renameDialog.style.display = 'flex';
        renameInput.focus();
        renameInput.select();
    },

    /**
     * Show rename dialog for a file
     * @param {Object} file - File object
     */
    showRenameFileDialog(file) {
        const renameInput = document.getElementById('rename-input');
        const renameDialog = document.getElementById('rename-dialog');

        window.app.renameMode = 'file';
        renameInput.value = file.name;
        // Update header text
        const headerSpan = renameDialog.querySelector('.rename-dialog-header span');
        if (headerSpan) headerSpan.textContent = window.i18n ? window.i18n.t('dialogs.rename_file') : 'Rename file';
        renameDialog.style.display = 'flex';
        renameInput.focus();
        renameInput.select();
    },

    /**
     * Close rename dialog
     */
    closeRenameDialog() {
        document.getElementById('rename-dialog').style.display = 'none';
        window.app.contextMenuTargetFolder = null;
    },

    /**
     * Show move dialog for a file or folder
     * @param {Object} item - File or folder object
     * @param {string} mode - 'file' or 'folder'
     */
    async showMoveDialog(item, mode) {
        // Set mode
        window.app.moveDialogMode = mode;

        // Reset selection
        window.app.selectedTargetFolderId = "";

        // Update dialog title (preserve icon)
        const dialogHeader = document.getElementById('move-file-dialog').querySelector('.rename-dialog-header');
        const titleText = mode === 'file' ?
            (window.i18n ? window.i18n.t('dialogs.move_file') : 'Move file') :
            (window.i18n ? window.i18n.t('dialogs.move_folder') : 'Move folder');
        dialogHeader.innerHTML = `<i class="fas fa-arrows-alt" style="color:#ff5e3a"></i> <span>${titleText}</span>`;

        // Load all available folders
        await this.loadAllFolders(item.id, mode);

        // Show dialog
        document.getElementById('move-file-dialog').style.display = 'flex';
    },

    /**
     * Close move dialog
     */
    closeMoveDialog() {
        document.getElementById('move-file-dialog').style.display = 'none';
        window.app.contextMenuTargetFile = null;
        window.app.contextMenuTargetFolder = null;
    },

    /**
     * Rename the selected folder or file
     */
    async renameItem() {
        const newName = document.getElementById('rename-input').value.trim();
        if (!newName) {
            alert(window.i18n ? window.i18n.t('errors.empty_name') : 'Name cannot be empty');
            return;
        }

        if (window.app.renameMode === 'file' && window.app.contextMenuTargetFile) {
            const success = await window.fileOps.renameFile(window.app.contextMenuTargetFile.id, newName);
            if (success) {
                contextMenus.closeRenameDialog();
                window.loadFiles();
            }
        } else if (window.app.contextMenuTargetFolder) {
            const success = await window.fileOps.renameFolder(window.app.contextMenuTargetFolder.id, newName);
            if (success) {
                contextMenus.closeRenameDialog();
                window.loadFiles();
            }
        }
    },

    // Keep backward compat
    renameFolder() {
        return contextMenus.renameItem();
    },

    /**
     * Load all folders for the move dialog
     * @param {string} itemId - ID of the item being moved
     * @param {string} mode - 'file' or 'folder'
     */
    async loadAllFolders(itemId, mode) {
        try {
            const token = localStorage.getItem('oxicloud_token');
            const response = await fetch('/api/folders', {
                headers: { 'Authorization': `Bearer ${token}` }
            });
            if (response.ok) {
                const folders = await response.json();
                const folderSelectContainer = document.getElementById('folder-select-container');

                // Clear container except root option
                folderSelectContainer.innerHTML = `
                    <div class="folder-select-item selected" data-folder-id="">
                        <i class="fas fa-folder"></i> <span data-i18n="dialogs.root">Root</span>
                    </div>
                `;

                // Select root by default
                window.app.selectedTargetFolderId = "";

                // Add all available folders
                if (Array.isArray(folders)) {
                    folders.forEach(folder => {
                        // Skip folders that would create cycles
                        if (mode === 'folder' && folder.id === itemId) {
                            return;
                        }

                        // Skip current folder of the item
                        if (mode === 'file' && window.app.contextMenuTargetFile && 
                            folder.id === window.app.contextMenuTargetFile.folder_id) {
                            return;
                        }

                        if (mode === 'folder' && window.app.contextMenuTargetFolder && 
                            folder.id === window.app.contextMenuTargetFolder.parent_id) {
                            return;
                        }

                        const folderItem = document.createElement('div');
                        folderItem.className = 'folder-select-item';
                        folderItem.dataset.folderId = folder.id;
                        folderItem.innerHTML = `<i class="fas fa-folder"></i> ${folder.name}`;

                        folderItem.addEventListener('click', () => {
                            // Deselect all
                            document.querySelectorAll('.folder-select-item').forEach(item => {
                                item.classList.remove('selected');
                            });

                            // Select this one
                            folderItem.classList.add('selected');
                            window.app.selectedTargetFolderId = folder.id;
                        });

                        folderSelectContainer.appendChild(folderItem);
                    });
                }

                // Event for root option
                const rootOption = folderSelectContainer.querySelector('.folder-select-item');
                rootOption.addEventListener('click', () => {
                    document.querySelectorAll('.folder-select-item').forEach(item => {
                        item.classList.remove('selected');
                    });
                    rootOption.classList.add('selected');
                    window.app.selectedTargetFolderId = "";
                });

                // Translate new elements
                if (window.i18n && window.i18n.translatePage) {
                    window.i18n.translatePage();
                }
            }
        } catch (error) {
            console.error('Error loading folders:', error);
        }
    },
    /**
     * Show share dialog for files or folders
     * @param {Object} item - File or folder object
     * @param {string} itemType - 'file' or 'folder'
     */
    showShareDialog(item, itemType) {
        // Update dialog title based on item type
        const dialogHeader = document.getElementById('share-dialog').querySelector('.share-dialog-header');
        const itemName = document.getElementById('shared-item-name');
        
        // Update dialog content
        dialogHeader.textContent = itemType === 'file' ?
            (window.i18n ? window.i18n.t('dialogs.share_file') : 'Share file') :
            (window.i18n ? window.i18n.t('dialogs.share_folder') : 'Share folder');
        
        itemName.textContent = item.name;
        
        // Reset form
        document.getElementById('share-password').value = '';
        document.getElementById('share-expiration').value = '';
        document.getElementById('share-permission-read').checked = true;
        document.getElementById('share-permission-write').checked = false;
        document.getElementById('share-permission-reshare').checked = false;
        
        // Store the current item and type for use when creating the share
        window.app.shareDialogItem = item;
        window.app.shareDialogItemType = itemType;
        
        // Check if item already has shares
        const existingShares = window.fileSharing.getSharedLinksForItem(item.id, itemType);
        const existingSharesContainer = document.getElementById('existing-shares-container');
        
        // Clear existing shares container
        existingSharesContainer.innerHTML = '';
        
        if (existingShares.length > 0) {
            document.getElementById('existing-shares-section').style.display = 'block';
            
            // Create elements for each existing share
            existingShares.forEach(share => {
                const shareEl = document.createElement('div');
                shareEl.className = 'existing-share-item';
                
                const expiresText = share.expires_at ? 
                    `Expires: ${window.fileSharing.formatExpirationDate(share.expires_at)}` : 
                    'No expiration';
                
                shareEl.innerHTML = `
                    <div class="share-url">${share.url}</div>
                    <div class="share-info">
                        ${share.password_protected ? '<span class="share-protected"><i class="fas fa-lock"></i> Password protected</span>' : ''}
                        <span class="share-expiration">${expiresText}</span>
                    </div>
                    <div class="share-actions">
                        <button class="btn btn-small copy-link-btn" data-share-url="${share.url}">
                            <i class="fas fa-copy"></i> Copy
                        </button>
                        <button class="btn btn-small btn-danger delete-link-btn" data-share-id="${share.id}">
                            <i class="fas fa-trash"></i> Delete
                        </button>
                    </div>
                `;
                
                existingSharesContainer.appendChild(shareEl);
            });
            
            // Add event listeners for copy and delete buttons
            document.querySelectorAll('.copy-link-btn').forEach(btn => {
                btn.addEventListener('click', (e) => {
                    e.preventDefault();
                    const url = btn.getAttribute('data-share-url');
                    window.fileSharing.copyLinkToClipboard(url);
                });
            });
            
            document.querySelectorAll('.delete-link-btn').forEach(btn => {
                btn.addEventListener('click', (e) => {
                    e.preventDefault();
                    const shareId = btn.getAttribute('data-share-id');
                    
                    showConfirmDialog({
                        title: window.i18n ? window.i18n.t('dialogs.confirm_delete_share') : 'Delete link',
                        message: window.i18n ? window.i18n.t('dialogs.confirm_delete_share_msg') : 'Are you sure you want to delete this shared link?',
                        confirmText: window.i18n ? window.i18n.t('actions.delete') : 'Delete',
                    }).then(confirmed => {
                        if (confirmed) {
                            window.fileSharing.removeSharedLink(shareId);
                            btn.closest('.existing-share-item').remove();
                            if (existingSharesContainer.children.length === 0) {
                                document.getElementById('existing-shares-section').style.display = 'none';
                            }
                        }
                    });
                });
            });
        } else {
            document.getElementById('existing-shares-section').style.display = 'none';
        }
        
        // Show dialog
        document.getElementById('share-dialog').style.display = 'flex';
    },
    
    /**
     * Create a shared link with the configured options
     */
    createSharedLink() {
        if (!window.app.shareDialogItem || !window.app.shareDialogItemType) {
            window.ui.showNotification('Error', 'Could not share the item');
            return;
        }
        
        // Get values from form
        const password = document.getElementById('share-password').value;
        const expirationDate = document.getElementById('share-expiration').value;
        const permissionRead = document.getElementById('share-permission-read').checked;
        const permissionWrite = document.getElementById('share-permission-write').checked;
        const permissionReshare = document.getElementById('share-permission-reshare').checked;
        
        // Prepare options
        const options = {
            password: password || null,
            expirationDate: expirationDate || null,
            permissions: {
                read: permissionRead,
                write: permissionWrite,
                reshare: permissionReshare
            }
        };
        
        try {
            const item = window.app.shareDialogItem;
            const itemType = window.app.shareDialogItemType;
            
            // Create share
            const shareInfo = window.fileSharing.generateSharedLink(
                item.id, 
                itemType, 
                options
            );
            
            // Update UI with new share
            const shareUrl = document.getElementById('generated-share-url');
            shareUrl.value = shareInfo.url;
            document.getElementById('new-share-section').style.display = 'block';
            
            // Focus and select for easy copying
            shareUrl.focus();
            shareUrl.select();
            
            // Show success message
            window.ui.showNotification('Link created', 'Shared link created successfully');
            
            // Reload existing shares
            this.showShareDialog(item, itemType);
            
        } catch (error) {
            console.error('Error creating shared link:', error);
            window.ui.showNotification('Error', 'Could not create shared link');
        }
    },
    
    /**
     * Show email notification dialog
     * @param {string} shareUrl - URL to share
     */
    showEmailNotificationDialog(shareUrl) {
        // Update dialog content
        document.getElementById('notification-share-url').textContent = shareUrl;
        document.getElementById('notification-email').value = '';
        document.getElementById('notification-message').value = '';
        
        // Store the URL for later use
        window.app.notificationShareUrl = shareUrl;
        
        // Show dialog
        document.getElementById('notification-dialog').style.display = 'flex';
    },
    
    /**
     * Send share notification email
     */
    sendShareNotification() {
        const email = document.getElementById('notification-email').value.trim();
        const message = document.getElementById('notification-message').value.trim();
        const shareUrl = window.app.notificationShareUrl;
        
        if (!email || !shareUrl) {
            window.ui.showNotification('Error', 'Please enter a valid email address');
            return;
        }
        
        // Validate email format
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        if (!emailRegex.test(email)) {
            window.ui.showNotification('Error', 'Please enter a valid email address');
            return;
        }
        
        try {
            window.fileSharing.sendShareNotification(shareUrl, email, message);
            document.getElementById('notification-dialog').style.display = 'none';
        } catch (error) {
            console.error('Error sending notification:', error);
            window.ui.showNotification('Error', 'Could not send notification');
        }
    },
    
    /**
     * Close share dialog
     */
    closeShareDialog() {
        document.getElementById('share-dialog').style.display = 'none';
        window.app.shareDialogItem = null;
        window.app.shareDialogItemType = null;
    },
    
    /**
     * Close notification dialog
     */
    closeNotificationDialog() {
        document.getElementById('notification-dialog').style.display = 'none';
        window.app.notificationShareUrl = null;
    }
};

// Expose context menus module globally
window.contextMenus = contextMenus;
