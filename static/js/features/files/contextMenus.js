/**
 * OxiCloud - Context Menus and Dialogs Module
 * This file handles context menus and dialog functionality
 */

// Context Menus Module
const contextMenus = {
    _setFavoriteOptionLabel(optionId, isFavorite) {
        const option = document.getElementById(optionId);
        if (!option) return;
        const label = option.querySelector('span');
        if (!label) return;
        label.textContent = window.i18n
            ? window.i18n.t(isFavorite ? 'actions.unfavorite' : 'actions.favorite')
            : (isFavorite ? 'Remove from favorites' : 'Add to favorites');
    },

    /**
     * Show or hide WOPI editor options based on current target file
     */
    async syncWopiOptionVisibility() {
        const wopiEdit = document.getElementById('wopi-edit-file-option');
        const wopiEditTab = document.getElementById('wopi-edit-file-tab-option');
        if (!wopiEdit || !wopiEditTab) return;

        const targetFile = window.app && window.app.contextMenuTargetFile;
        const show = targetFile &&
            window.wopiEditor &&
            await window.wopiEditor.canEdit(targetFile.name);

        wopiEdit.style.display = show ? '' : 'none';
        wopiEditTab.style.display = show ? '' : 'none';
    },

    syncFavoriteOptionLabels() {
        if (!window.favorites) return;

        const targetFile = window.app && window.app.contextMenuTargetFile;
        const targetFolder = window.app && window.app.contextMenuTargetFolder;

        if (targetFile) {
            const isFav = window.favorites.isFavorite(targetFile.id, 'file');
            this._setFavoriteOptionLabel('favorite-file-option', isFav);
        }

        if (targetFolder) {
            const isFav = window.favorites.isFavorite(targetFolder.id, 'folder');
            this._setFavoriteOptionLabel('favorite-folder-option', isFav);
        }
    },

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

        document.getElementById('favorite-folder-option').addEventListener('click', async () => {
            if (window.app.contextMenuTargetFolder) {
                const folder = window.app.contextMenuTargetFolder;

                // Check if folder is already in favorites to toggle
                if (window.favorites && window.favorites.isFavorite(folder.id, 'folder')) {
                    // Remove from favorites
                    const ok = await window.favorites.removeFromFavorites(folder.id, 'folder');
                    if (ok && window.ui && typeof window.ui.setFavoriteVisualState === 'function') {
                        window.ui.setFavoriteVisualState(folder.id, 'folder', false);
                    }
                } else {
                    // Add to favorites
                    const ok = await window.favorites.addToFavorites(
                        folder.id,
                        folder.name,
                        'folder',
                        folder.parent_id
                    );
                    if (ok && window.ui && typeof window.ui.setFavoriteVisualState === 'function') {
                        window.ui.setFavoriteVisualState(folder.id, 'folder', true);
                    }
                }
                this.syncFavoriteOptionLabels();
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

        document.getElementById('wopi-edit-file-option').addEventListener('click', () => {
            if (window.app.contextMenuTargetFile) {
                const file = window.app.contextMenuTargetFile;
                window.wopiEditor.openInModal(file.id, file.name, 'edit');
            }
            window.ui.closeFileContextMenu();
        });

        document.getElementById('wopi-edit-file-tab-option').addEventListener('click', () => {
            if (window.app.contextMenuTargetFile) {
                const file = window.app.contextMenuTargetFile;
                window.wopiEditor.openInTab(file.id, file.name, 'edit');
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

        document.getElementById('favorite-file-option').addEventListener('click', async () => {
            if (window.app.contextMenuTargetFile) {
                const file = window.app.contextMenuTargetFile;

                // Check if file is already in favorites to toggle
                if (window.favorites && window.favorites.isFavorite(file.id, 'file')) {
                    // Remove from favorites
                    const ok = await window.favorites.removeFromFavorites(file.id, 'file');
                    if (ok && window.ui && typeof window.ui.setFavoriteVisualState === 'function') {
                        window.ui.setFavoriteVisualState(file.id, 'file', false);
                    }
                } else {
                    // Add to favorites
                    const ok = await window.favorites.addToFavorites(
                        file.id,
                        file.name,
                        'file',
                        file.folder_id
                    );
                    if (ok && window.ui && typeof window.ui.setFavoriteVisualState === 'function') {
                        window.ui.setFavoriteVisualState(file.id, 'file', true);
                    }
                }
                this.syncFavoriteOptionLabels();
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
        const copyConfirmBtn = document.getElementById('copy-confirm-btn');

        moveCancelBtn.addEventListener('click', this.closeMoveDialog);

        // Copy button handler
        copyConfirmBtn.addEventListener('click', async () => {
            // Batch copy mode (from multiSelect)
            if (window.app.moveDialogMode === 'batch' && window.multiSelect) {
                const targetId = window.app.selectedTargetFolderId;
                const items = window.app.batchMoveItems || [];

                const fileIds = items.filter(i => i.type === 'file').map(i => i.id);
                const folderIds = items.filter(i => i.type === 'folder').map(i => i.id);

                let success = 0, errors = 0;

                try {
                    // Batch copy files
                    if (fileIds.length > 0) {
                        const res = await fetch('/api/batch/files/copy', {
                            method: 'POST',
                            headers: { ...getAuthHeaders(), 'Content-Type': 'application/json' },
                            body: JSON.stringify({ file_ids: fileIds, target_folder_id: targetId })
                        });
                        const data = await res.json();
                        success += data.stats?.successful || 0;
                        errors += data.stats?.failed || 0;
                    }

                    // Note: Folder copy is not yet implemented in batch API
                    if (folderIds.length > 0) {
                        window.ui.showNotification('Info', 'Folder copy is not yet supported in batch mode');
                    }
                } catch (err) {
                    console.error('Batch copy error:', err);
                    errors++;
                }

                this.closeMoveDialog();
                window.multiSelect.clear();
                window.loadFiles();

                if (errors > 0) {
                    window.ui.showNotification('Batch copy', `${success} copied, ${errors} failed`);
                } else {
                    window.ui.showNotification('Items copied',
                        `${success} item${success !== 1 ? 's' : ''} copied successfully`);
                }
                return;
            }

            // Single item copy
            if (window.app.moveDialogMode === 'file' && window.app.contextMenuTargetFile) {
                const success = await window.fileOps.copyFile(
                    window.app.contextMenuTargetFile.id,
                    window.app.selectedTargetFolderId
                );
                if (success) {
                    this.closeMoveDialog();
                }
            } else if (window.app.moveDialogMode === 'folder' && window.app.contextMenuTargetFolder) {
                const success = await window.fileOps.copyFolder(
                    window.app.contextMenuTargetFolder.id,
                    window.app.selectedTargetFolderId
                );
                if (success) {
                    this.closeMoveDialog();
                }
            }
        });

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

        // Ensure we have the home folder ID BEFORE calculating startFolderId
        if (!window.app.userHomeFolderId) {
            console.log('[Move Dialog] Home folder ID not set, resolving...');
            await window.resolveHomeFolder();
        }

        // Initialize dialog navigation state
        // Start at the parent of the item being moved (so user sees siblings and can navigate)
        let startFolderId = null;
        if (mode === 'file' && item.folder_id) {
            startFolderId = item.folder_id;
        } else if (mode === 'folder' && item.parent_id) {
            startFolderId = item.parent_id;
        } else {
            // If item is at root level, start at user's home folder
            startFolderId = window.app.userHomeFolderId || null;
        }

        console.log('[Move Dialog] showMoveDialog - item:', item, 'mode:', mode, 'startFolderId:', startFolderId, 'userHomeFolderId:', window.app.userHomeFolderId);

        // Store the item being moved and navigation state
        window.app.moveDialogItemId = item.id;
        window.app.moveDialogItemMode = mode;
        window.app.moveDialogCurrentFolderId = startFolderId;
        window.app.moveDialogBreadcrumb = [];

        // Update dialog title (preserve icon)
        const dialogHeader = document.getElementById('move-file-dialog').querySelector('.rename-dialog-header');
        const titleText = mode === 'file' ?
            (window.i18n ? window.i18n.t('dialogs.move_file') : 'Move file') :
            (window.i18n ? window.i18n.t('dialogs.move_folder') : 'Move folder');
        dialogHeader.innerHTML = `<i class="fas fa-arrows-alt" style="color:#ff5e3a"></i> <span>${titleText}</span>`;

        // Load folders for the starting location
        await this.loadMoveDialogFolders(startFolderId);

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
     * Load folders for the move dialog with navigation support
     * Shows subfolders of the specified parent folder and allows navigation
     * @param {string} parentFolderId - Parent folder ID to load children from (null for root)
     */
    async loadMoveDialogFolders(parentFolderId) {
        try {
            const token = localStorage.getItem('oxicloud_token');
            const headers = token ? { 'Authorization': `Bearer ${token}` } : {};

            // Ensure we have the home folder ID before proceeding
            if (!window.app.userHomeFolderId) {
                console.log('[Move Dialog] Home folder ID not set, resolving...');
                await window.resolveHomeFolder();
            }

            // Build URL - use /api/folders/{id}/contents to get CHILDREN of the folder
            let url;
            let effectiveParentId = parentFolderId || window.app.userHomeFolderId;

            if (effectiveParentId) {
                // Use the contents endpoint to get children (not the folder itself)
                url = `/api/folders/${effectiveParentId}/contents`;
            } else {
                // Last resort fallback: this returns root folders (home folder)
                // We should NOT use this as it returns the folder itself, not its contents
                console.error('[Move Dialog] No folder ID available, using fallback');
                url = '/api/folders';
            }

            console.log('[Move Dialog] Loading folders from:', url, 'effectiveParentId:', effectiveParentId);
            const response = await fetch(url, { headers });
            if (!response.ok) {
                console.error('Failed to load folders:', response.status);
                return;
            }

            const data = await response.json();
            console.log('[Move Dialog] API response:', data);

            // The contents endpoint returns an array of child folders
            // The fallback /api/folders returns root folders (home folder itself)
            const folders = Array.isArray(data) ? data : (data.folders || []);
            console.log('[Move Dialog] Loaded folders:', folders.length, 'folders:', folders);

            const folderSelectContainer = document.getElementById('folder-select-container');
            const breadcrumbContainer = document.getElementById('move-dialog-breadcrumb');

            // Clear container
            folderSelectContainer.innerHTML = '';

            // Get current navigation state
            const itemId = window.app.moveDialogItemId;
            const mode = window.app.moveDialogItemMode;
            const breadcrumb = window.app.moveDialogBreadcrumb || [];

            // Only show breadcrumb when we've navigated into subfolders
            if (breadcrumb.length > 0) {
                this._renderMoveDialogBreadcrumb(breadcrumbContainer, breadcrumb, effectiveParentId);
                breadcrumbContainer.style.display = 'flex';
            } else {
                breadcrumbContainer.style.display = 'none';
            }

            // Option to select current folder as destination (only after navigating into subfolders)
            if (breadcrumb.length > 0 && effectiveParentId && effectiveParentId !== itemId) {
                const currentFolderOption = document.createElement('div');
                currentFolderOption.className = 'folder-select-item folder-select-current';
                currentFolderOption.innerHTML = `
                    <i class="fas fa-check-circle" style="color: #48bb78;"></i>
                    <span>${window.i18n ? window.i18n.t('dialogs.select_this_folder') : 'Select this folder'}</span>
                `;
                currentFolderOption.addEventListener('click', () => {
                    document.querySelectorAll('.folder-select-item').forEach(item => {
                        item.classList.remove('selected');
                    });
                    currentFolderOption.classList.add('selected');
                    window.app.selectedTargetFolderId = effectiveParentId;
                });
                folderSelectContainer.appendChild(currentFolderOption);
            }

            // Add "Go to parent" option if we have navigated into subfolders
            if (breadcrumb.length > 0) {
                const parentOption = document.createElement('div');
                parentOption.className = 'folder-select-item folder-navigate-up';
                parentOption.innerHTML = `
                    <i class="fas fa-level-up-alt"></i>
                    <span>${window.i18n ? window.i18n.t('dialogs.go_to_parent') : '.. (parent folder)'}</span>
                `;
                parentOption.addEventListener('click', () => {
                    // Navigate to parent folder
                    const currentBreadcrumb = window.app.moveDialogBreadcrumb || [];
                    if (currentBreadcrumb.length > 0) {
                        // Remove current folder from breadcrumb
                        currentBreadcrumb.pop();
                        const parentFolder = currentBreadcrumb.length > 0
                            ? currentBreadcrumb[currentBreadcrumb.length - 1]
                            : null;
                        window.app.moveDialogBreadcrumb = currentBreadcrumb;
                        window.app.moveDialogCurrentFolderId = parentFolder ? parentFolder.id : null;
                        this.loadMoveDialogFolders(parentFolder ? parentFolder.id : null);
                    } else {
                        // Go to root (home folder)
                        window.app.moveDialogCurrentFolderId = window.app.userHomeFolderId || null;
                        this.loadMoveDialogFolders(window.app.userHomeFolderId || null);
                    }
                });
                folderSelectContainer.appendChild(parentOption);
            }

            // Add subfolders (clicking navigates INTO the folder)
            folders.forEach(folder => {
                // Skip the item being moved (to prevent moving a folder into itself)
                if (mode === 'folder' && folder.id === itemId) {
                    return;
                }

                const folderItem = document.createElement('div');
                folderItem.className = 'folder-select-item folder-navigate';
                folderItem.dataset.folderId = folder.id;
                folderItem.innerHTML = `
                    <i class="fas fa-folder"></i>
                    <span class="folder-name">${escapeHtml(folder.name)}</span>
                    <i class="fas fa-chevron-right folder-navigate-icon"></i>
                `;

                // Click navigates INTO this folder
                folderItem.addEventListener('click', () => {
                    // Add to breadcrumb
                    const breadcrumb = window.app.moveDialogBreadcrumb || [];
                    breadcrumb.push({ id: folder.id, name: folder.name });
                    window.app.moveDialogBreadcrumb = breadcrumb;
                    window.app.moveDialogCurrentFolderId = folder.id;
                    this.loadMoveDialogFolders(folder.id);
                });

                folderSelectContainer.appendChild(folderItem);
            });

            // Show "no subfolders" message if there are no folders to navigate
            if (folders.length === 0 && breadcrumb.length === 0) {
                // At home folder level with no subfolders - show option to move here
                const homeOption = document.createElement('div');
                homeOption.className = 'folder-select-item folder-select-current';
                homeOption.innerHTML = `
                    <i class="fas fa-check-circle" style="color: #48bb78;"></i>
                    <span>${window.i18n ? window.i18n.t('dialogs.move_to_home') : 'Move to Home folder'}</span>
                `;
                homeOption.addEventListener('click', () => {
                    document.querySelectorAll('.folder-select-item').forEach(item => {
                        item.classList.remove('selected');
                    });
                    homeOption.classList.add('selected');
                    window.app.selectedTargetFolderId = '';  // Empty means root/home
                });
                folderSelectContainer.appendChild(homeOption);
            } else if (folders.length === 0) {
                // Inside a subfolder with no children - show empty message
                const emptyMsg = document.createElement('div');
                emptyMsg.className = 'folder-select-empty';
                emptyMsg.innerHTML = `<i class="fas fa-folder-open"></i> <span>${window.i18n ? window.i18n.t('dialogs.no_subfolders') : 'No subfolders to navigate'}</span>`;
                folderSelectContainer.appendChild(emptyMsg);
            }

            // Set default selection to current folder
            window.app.selectedTargetFolderId = parentFolderId || '';

            // Translate new elements
            if (window.i18n && window.i18n.translateElement) {
                window.i18n.translateElement(folderSelectContainer);
            }
        } catch (error) {
            console.error('Error loading folders:', error);
        }
    },

    /**
     * Render breadcrumb navigation for move dialog
     */
    _renderMoveDialogBreadcrumb(container, breadcrumb, currentFolderId) {
        if (!container) return;
        container.innerHTML = '';

        const homeFolderId = window.app.userHomeFolderId;
        const homeFolderName = window.app.userHomeFolderName || 'Home';

        // Home icon (click to go to home folder)
        const homeItem = document.createElement('span');
        homeItem.className = 'move-breadcrumb-item';
        homeItem.innerHTML = '<i class="fas fa-home"></i>';
        homeItem.addEventListener('click', () => {
            window.app.moveDialogBreadcrumb = [];
            window.app.moveDialogCurrentFolderId = homeFolderId || null;
            this.loadMoveDialogFolders(homeFolderId || null);
        });
        container.appendChild(homeItem);

        // Home folder name
        if (homeFolderName) {
            const separator = document.createElement('span');
            separator.className = 'move-breadcrumb-separator';
            separator.textContent = '>';
            container.appendChild(separator);

            const homeNameItem = document.createElement('span');
            homeNameItem.className = 'move-breadcrumb-item';
            if (breadcrumb.length === 0) {
                homeNameItem.classList.add('current');
            }
            homeNameItem.textContent = homeFolderName;
            if (breadcrumb.length > 0) {
                homeNameItem.addEventListener('click', () => {
                    window.app.moveDialogBreadcrumb = [];
                    window.app.moveDialogCurrentFolderId = homeFolderId || null;
                    this.loadMoveDialogFolders(homeFolderId || null);
                });
            }
            container.appendChild(homeNameItem);
        }

        // Breadcrumb path
        breadcrumb.forEach((segment, index) => {
            const separator = document.createElement('span');
            separator.className = 'move-breadcrumb-separator';
            separator.textContent = '>';
            container.appendChild(separator);

            const item = document.createElement('span');
            item.className = 'move-breadcrumb-item';
            if (index === breadcrumb.length - 1) {
                item.classList.add('current');
            }
            item.textContent = segment.name;

            // Click to navigate back to this level
            if (index < breadcrumb.length - 1) {
                item.addEventListener('click', () => {
                    window.app.moveDialogBreadcrumb = breadcrumb.slice(0, index + 1);
                    window.app.moveDialogCurrentFolderId = segment.id;
                    this.loadMoveDialogFolders(segment.id);
                });
            }
            container.appendChild(item);
        });
    },

    /**
     * Load all folders for the move dialog (legacy - kept for batch operations)
     * @param {string} itemId - ID of the item being moved
     * @param {string} mode - 'file' or 'folder'
     */
    async loadAllFolders(itemId, mode) {
        // For batch mode, use the old behavior but start from home folder
        try {
            const token = localStorage.getItem('oxicloud_token');
            const headers = token ? { 'Authorization': `Bearer ${token}` } : {};
            const response = await fetch('/api/folders', { headers });
            if (response.ok) {
                const folders = await response.json();
                const folderSelectContainer = document.getElementById('folder-select-container');

                // Clear container
                folderSelectContainer.innerHTML = '';

                // Add all available folders
                if (Array.isArray(folders)) {
                    const excludeIds = itemId ? [itemId] : [];
                    folders.forEach(folder => {
                        // Skip folders that would create cycles
                        if (excludeIds.includes(folder.id)) {
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

                // Set default selection
                window.app.selectedTargetFolderId = "";

                // Translate new elements (scoped to container)
                if (window.i18n && window.i18n.translateElement) {
                    window.i18n.translateElement(folderSelectContainer);
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

                // Share URL
                const urlDiv = document.createElement('div');
                urlDiv.className = 'share-url';
                urlDiv.textContent = share.url;
                shareEl.appendChild(urlDiv);

                // Share info
                const infoDiv = document.createElement('div');
                infoDiv.className = 'share-info';
                if (share.has_password) {
                    const protectedSpan = document.createElement('span');
                    protectedSpan.className = 'share-protected';
                    protectedSpan.innerHTML = '<i class="fas fa-lock"></i> Password protected';
                    infoDiv.appendChild(protectedSpan);
                }
                const expirationSpan = document.createElement('span');
                expirationSpan.className = 'share-expiration';
                expirationSpan.textContent = expiresText;
                infoDiv.appendChild(expirationSpan);
                shareEl.appendChild(infoDiv);

                // Share actions
                const actionsDiv = document.createElement('div');
                actionsDiv.className = 'share-actions';

                const copyBtn = document.createElement('button');
                copyBtn.className = 'btn btn-small copy-link-btn';
                copyBtn.dataset.shareUrl = share.url;
                copyBtn.innerHTML = '<i class="fas fa-copy"></i> Copy';
                actionsDiv.appendChild(copyBtn);

                const deleteBtn = document.createElement('button');
                deleteBtn.className = 'btn btn-small btn-danger delete-link-btn';
                deleteBtn.dataset.shareId = share.id;
                deleteBtn.innerHTML = '<i class="fas fa-trash"></i> Delete';
                actionsDiv.appendChild(deleteBtn);

                shareEl.appendChild(actionsDiv);

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
