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
            const folder = window.app.contextMenuTargetFolder;
            if (folder) {
                this.showShareDialog(folder, 'folder');
            }
            window.ui.closeContextMenu();
        });

        document.getElementById('delete-folder-option').addEventListener('click', async () => {
            const folder = window.app.contextMenuTargetFolder;
            window.ui.closeContextMenu();
            if (folder) {
                await window.fileOps.deleteFolder(folder.id, folder.name);
            }
        });

        // File context menu options
        document.getElementById('view-file-option').addEventListener('click', () => {
            if (window.app.contextMenuTargetFile) {
                // Capture reference before context menu cleanup nullifies it
                const file = window.app.contextMenuTargetFile;
                const token = localStorage.getItem('oxicloud_token');
                const headers = token ? { 'Authorization': `Bearer ${token}` } : {};
                fetch(`/api/files/${file.id}?metadata=true`, { headers })
                    .then(response => response.json())
                    .then(fileDetails => {
                        // Check if viewable file type (images, PDFs, text files)
                        if (window.ui && window.ui.isViewableFile(fileDetails)) {
                            // Open with inline viewer
                            if (window.inlineViewer) {
                                window.inlineViewer.openFile(fileDetails);
                            } else if (window.fileViewer) {
                                window.fileViewer.open(fileDetails);
                            } else {
                                // If no viewer is available, download directly
                                window.fileOps.downloadFile(file.id, file.name);
                            }
                        } else {
                            // For non-viewable files, download
                            window.fileOps.downloadFile(file.id, file.name);
                        }
                    })
                    .catch(error => {
                        console.error('Error fetching file details:', error);
                        // On error, fallback to download
                        window.fileOps.downloadFile(file.id, file.name);
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
            const file = window.app.contextMenuTargetFile;
            if (file) {
                this.showShareDialog(file, 'file');
            }
            window.ui.closeFileContextMenu();
        });

        document.getElementById('delete-file-option').addEventListener('click', async () => {
            const file = window.app.contextMenuTargetFile;
            window.ui.closeFileContextMenu();
            if (file) {
                await window.fileOps.deleteFile(file.id, file.name);
            }
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
            // Batch move mode (from multiSelect)
            if (window.app.moveDialogMode === 'batch' && window.multiSelect) {
                const targetId = window.app.selectedTargetFolderId;
                const items = window.app.batchMoveItems || [];

                const fileIds = items.filter(i => i.type === 'file').map(i => i.id);
                const folderIds = items.filter(i => i.type === 'folder' && i.id !== targetId).map(i => i.id);

                let success = 0, errors = 0;

                try {
                    // Batch move files in a single request
                    if (fileIds.length > 0) {
                        const res = await fetch('/api/batch/files/move', {
                            method: 'POST',
                            headers: { ...getAuthHeaders(), 'Content-Type': 'application/json' },
                            body: JSON.stringify({ file_ids: fileIds, target_folder_id: targetId })
                        });
                        const data = await res.json();
                        success += data.stats?.successful || 0;
                        errors += data.stats?.failed || 0;
                    }

                    // Batch move folders in a single request
                    if (folderIds.length > 0) {
                        const res = await fetch('/api/batch/folders/move', {
                            method: 'POST',
                            headers: { ...getAuthHeaders(), 'Content-Type': 'application/json' },
                            body: JSON.stringify({ folder_ids: folderIds, target_folder_id: targetId })
                        });
                        const data = await res.json();
                        success += data.stats?.successful || 0;
                        errors += data.stats?.failed || 0;
                    }
                } catch (err) {
                    console.error('Batch move error:', err);
                    errors++;
                }

                this.closeMoveDialog();
                window.multiSelect.clear();
                window.loadFiles();

                if (errors > 0) {
                    window.ui.showNotification('Batch move', `${success} moved, ${errors} failed`);
                } else {
                    window.ui.showNotification('Items moved',
                        `${success} item${success !== 1 ? 's' : ''} moved successfully`);
                }
                return;
            }

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
        // Store the folder reference so it survives context menu cleanup
        window.app.renameTarget = folder;
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
        // Store the file reference so it survives context menu cleanup
        window.app.renameTarget = file;
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
        window.app.renameTarget = null;
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

        // Use renameTarget which was saved before the context menu was closed
        const target = window.app.renameTarget;
        if (!target) {
            console.error('No rename target available');
            return;
        }

        if (window.app.renameMode === 'file') {
            const success = await window.fileOps.renameFile(target.id, newName);
            if (success) {
                contextMenus.closeRenameDialog();
                window.loadFiles();
            }
        } else if (window.app.renameMode === 'folder') {
            const success = await window.fileOps.renameFolder(target.id, newName);
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
            const headers = token ? { 'Authorization': `Bearer ${token}` } : {};
            const response = await fetch('/api/folders', { headers });
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
                        folderItem.innerHTML = `<i class="fas fa-folder"></i> ${escapeHtml(folder.name)}`;

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
    async showShareDialog(item, itemType) {
        try {
        const shareDialog = document.getElementById('share-dialog');
        if (!shareDialog) {
            console.error('Share dialog element not found in DOM');
            window.ui.showNotification('Error', 'Share dialog not available');
            return;
        }

        // Update dialog title — use the <span> inside header to preserve <i> icon
        const dialogHeader = shareDialog.querySelector('.share-dialog-header');
        if (dialogHeader) {
            const headerSpan = dialogHeader.querySelector('span');
            const titleText = itemType === 'file' ?
                (window.i18n ? window.i18n.t('dialogs.share_file') : 'Share file') :
                (window.i18n ? window.i18n.t('dialogs.share_folder') : 'Share folder');
            if (headerSpan) {
                headerSpan.textContent = titleText;
            } else {
                dialogHeader.textContent = titleText;
            }
        }

        const itemName = document.getElementById('shared-item-name');
        if (itemName) itemName.textContent = item.name;
        
        // Reset form
        const pwField = document.getElementById('share-password');
        const expField = document.getElementById('share-expiration');
        if (pwField) pwField.value = '';
        if (expField) expField.value = '';
        const permRead = document.getElementById('share-permission-read');
        const permWrite = document.getElementById('share-permission-write');
        const permReshare = document.getElementById('share-permission-reshare');
        if (permRead) permRead.checked = true;
        if (permWrite) permWrite.checked = false;
        if (permReshare) permReshare.checked = false;
        
        // Store the current item and type for use when creating the share
        window.app.shareDialogItem = item;
        window.app.shareDialogItemType = itemType;
        
        // Check if item already has shares (async API call)
        const existingShares = await window.fileSharing.getSharedLinksForItem(item.id, itemType);
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
                        ${share.has_password ? '<span class="share-protected"><i class="fas fa-lock"></i> Password protected</span>' : ''}
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
                    }).then(async (confirmed) => {
                        if (confirmed) {
                            await window.fileSharing.removeSharedLink(shareId);
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
        
        // Hide new-share section from previous use
        const newShareSection = document.getElementById('new-share-section');
        if (newShareSection) newShareSection.style.display = 'none';

        // Show dialog
        shareDialog.style.display = 'flex';
        console.log('Share dialog opened for', itemType, item.name);
        } catch (error) {
            console.error('Error opening share dialog:', error);
            window.ui.showNotification('Error', 'Could not open share dialog');
        }
    },
    
    /**
     * Create a shared link with the configured options
     */
    async createSharedLink() {
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
        
        const item = window.app.shareDialogItem;
        const itemType = window.app.shareDialogItemType;

        // Build DTO for backend API
        const createDto = {
            item_id: item.id,
            item_name: item.name || null,
            item_type: itemType,
            password: password || null,
            expires_at: expirationDate ? Math.floor(new Date(expirationDate).getTime() / 1000) : null,
            permissions: {
                read: permissionRead,
                write: permissionWrite,
                reshare: permissionReshare
            }
        };

        try {
            const token = localStorage.getItem('oxicloud_token');
            const headers = { 'Content-Type': 'application/json' };
            if (token) headers['Authorization'] = `Bearer ${token}`;

            const response = await fetch('/api/shares', {
                method: 'POST',
                headers,
                body: JSON.stringify(createDto)
            });

            if (!response.ok) {
                const errBody = await response.json().catch(() => ({}));
                throw new Error(errBody.error || `Server error ${response.status}`);
            }

            const shareInfo = await response.json();

            // Update UI with new share
            const shareUrl = document.getElementById('generated-share-url');
            if (shareUrl) {
                shareUrl.value = shareInfo.url;
                document.getElementById('new-share-section').style.display = 'block';
                shareUrl.focus();
                shareUrl.select();
            }
            
            // Show success message
            window.ui.showNotification(
                window.i18n ? window.i18n.t('notifications.link_created') : 'Link created',
                window.i18n ? window.i18n.t('notifications.share_success') : 'Shared link created successfully'
            );
            
        } catch (error) {
            console.error('Error creating shared link:', error);
            window.ui.showNotification('Error', error.message || 'Could not create shared link');
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
        const dialog = document.getElementById('share-dialog');
        if (dialog) dialog.style.display = 'none';
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
