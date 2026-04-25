/**
 * OxiCloud - Context Menus and Dialogs Module
 * This file handles context menus and dialog functionality
 */

import { resolveHomeFolder } from '../../app/authSession.js';
import { loadFiles } from '../../app/filesView.js';
import { app } from '../../app/state.js';
import { showConfirmDialog, ui } from '../../app/ui.js';
import { getCsrfHeaders } from '../../core/csrf.js';
import { escapeHtml } from '../../core/formatters.js';
import { i18n } from '../../core/i18n.js';
import { favorites } from '../library/favorites.js';
import { musicView } from '../library/music.js';
import { fileSharing } from '../sharing/fileSharing.js';
import { fileOps } from './fileOperations.js';
import { inlineViewer } from './inlineViewer.js';
import { multiSelect } from './multiSelect.js';
import { wopiEditor } from './wopiEditor.js';

let _moveDialogEscapeHandler = null;

// Context Menus Module
const contextMenus = {
    _setFavoriteOptionLabel(optionId, isFavorite) {
        const option = document.getElementById(optionId);
        if (!option) return;
        const label = option.querySelector('span');
        if (!label) return;
        label.textContent = i18n.t(isFavorite ? 'actions.unfavorite' : 'actions.favorite');
    },

    /**
     * Show or hide WOPI editor options based on current target file
     */
    async syncWopiOptionVisibility() {
        const wopiEdit = document.getElementById('wopi-edit-file-option');
        const wopiEditTab = document.getElementById('wopi-edit-file-tab-option');
        if (!wopiEdit || !wopiEditTab) return;

        const targetFile = app?.contextMenuTargetFile;
        // Don't show WOPI editor for image files - they should use inline preview
        const isImage = targetFile?.mime_type?.startsWith('image/');
        const show = targetFile && !isImage && wopiEditor && (await wopiEditor.canEdit(targetFile.name));

        wopiEdit.classList.toggle('hidden', !show);
        wopiEditTab.classList.toggle('hidden', !show);
    },

    syncFavoriteOptionLabels() {
        if (!favorites) return;

        const targetFile = app?.contextMenuTargetFile;
        const targetFolder = app?.contextMenuTargetFolder;

        if (targetFile) {
            const isFav = favorites.isFavorite(targetFile.id, 'file');
            this._setFavoriteOptionLabel('favorite-file-option', isFav);
        }

        if (targetFolder) {
            const isFav = favorites.isFavorite(targetFolder.id, 'folder');
            this._setFavoriteOptionLabel('favorite-folder-option', isFav);
        }
    },

    syncAddToPlaylistOption() {
        const option = document.getElementById('add-to-playlist-option');
        if (!option) return;

        const targetFile = app?.contextMenuTargetFile;
        if (targetFile) {
            const isAudio = targetFile.mime_type?.startsWith('audio/');
            option.classList.toggle('hidden', !isAudio);
        } else {
            option.classList.add('hidden');
        }
    },

    /**
     * Assign events to menu items and dialogs
     */
    assignMenuEvents() {
        // Folder context menu options
        document.getElementById('download-folder-option').addEventListener('click', () => {
            if (app.contextMenuTargetFolder) {
                fileOps.downloadFolder(app.contextMenuTargetFolder.id, app.contextMenuTargetFolder.name);
            }
            ui.closeContextMenu();
        });

        document.getElementById('favorite-folder-option').addEventListener('click', async () => {
            if (app.contextMenuTargetFolder) {
                const folder = app.contextMenuTargetFolder;

                // Check if folder is already in favorites to toggle
                if (favorites?.isFavorite(folder.id, 'folder')) {
                    // Remove from favorites
                    const ok = await favorites.removeFromFavorites(folder.id, 'folder');
                    if (ok && ui && typeof ui.setFavoriteVisualState === 'function') {
                        ui.setFavoriteVisualState(folder.id, 'folder', false);
                    }
                } else {
                    // Add to favorites
                    const ok = await favorites.addToFavorites(folder.id, folder.name, 'folder', folder.parent_id);
                    if (ok && ui && typeof ui.setFavoriteVisualState === 'function') {
                        ui.setFavoriteVisualState(folder.id, 'folder', true);
                    }
                }
                this.syncFavoriteOptionLabels();
            }
            ui.closeContextMenu();
        });

        document.getElementById('rename-folder-option').addEventListener('click', () => {
            if (app.contextMenuTargetFolder) {
                this.showRenameDialog(app.contextMenuTargetFolder);
            }
            ui.closeContextMenu();
        });

        document.getElementById('move-folder-option').addEventListener('click', () => {
            if (app.contextMenuTargetFolder) {
                this.showMoveDialog(app.contextMenuTargetFolder, 'folder');
            }
            ui.closeContextMenu();
        });

        document.getElementById('share-folder-option').addEventListener('click', () => {
            const folder = app.contextMenuTargetFolder;
            if (folder) {
                this.showShareDialog(folder, 'folder');
            }
            ui.closeContextMenu();
        });

        document.getElementById('delete-folder-option').addEventListener('click', async () => {
            const folder = app.contextMenuTargetFolder;
            ui.closeContextMenu();
            if (folder) {
                await fileOps.deleteFolder(folder.id, folder.name);
            }
        });

        // File context menu options
        document.getElementById('view-file-option').addEventListener('click', () => {
            if (app.contextMenuTargetFile) {
                // Capture reference before context menu cleanup nullifies it
                const file = app.contextMenuTargetFile;
                fetch(`/api/files/${file.id}?metadata=true`, {
                    credentials: 'same-origin'
                })
                    .then((response) => response.json())
                    .then((fileDetails) => {
                        // Check if viewable file type (images, PDFs, text files)
                        if (ui?.isViewableFile(fileDetails)) {
                            // Open with inline viewer
                            if (inlineViewer) {
                                inlineViewer.openFile(fileDetails);
                            } else {
                                // If no viewer is available, download directly
                                fileOps.downloadFile(file.id, file.name);
                            }
                        } else {
                            // For non-viewable files, download
                            fileOps.downloadFile(file.id, file.name);
                        }
                    })
                    .catch((error) => {
                        console.error('Error fetching file details:', error);
                        // On error, fallback to download
                        fileOps.downloadFile(file.id, file.name);
                    });
            }
            ui.closeFileContextMenu();
        });

        document.getElementById('wopi-edit-file-option').addEventListener('click', () => {
            if (app.contextMenuTargetFile) {
                const file = app.contextMenuTargetFile;
                wopiEditor.openInModal(file.id, file.name, 'edit');
            }
            ui.closeFileContextMenu();
        });

        document.getElementById('wopi-edit-file-tab-option').addEventListener('click', () => {
            if (app.contextMenuTargetFile) {
                const file = app.contextMenuTargetFile;
                wopiEditor.openInTab(file.id, file.name, 'edit');
            }
            ui.closeFileContextMenu();
        });

        document.getElementById('download-file-option').addEventListener('click', () => {
            if (app.contextMenuTargetFile) {
                fileOps.downloadFile(app.contextMenuTargetFile.id, app.contextMenuTargetFile.name);
            }
            ui.closeFileContextMenu();
        });

        document.getElementById('favorite-file-option').addEventListener('click', async () => {
            if (app.contextMenuTargetFile) {
                const file = app.contextMenuTargetFile;

                // Check if file is already in favorites to toggle
                if (favorites?.isFavorite(file.id, 'file')) {
                    // Remove from favorites
                    const ok = await favorites.removeFromFavorites(file.id, 'file');
                    if (ok && ui && typeof ui.setFavoriteVisualState === 'function') {
                        ui.setFavoriteVisualState(file.id, 'file', false);
                    }
                } else {
                    // Add to favorites
                    const ok = await favorites.addToFavorites(file.id, file.name, 'file', file.folder_id);
                    if (ok && ui && typeof ui.setFavoriteVisualState === 'function') {
                        ui.setFavoriteVisualState(file.id, 'file', true);
                    }
                }
                this.syncFavoriteOptionLabels();
            }
            ui.closeFileContextMenu();
        });

        document.getElementById('rename-file-option').addEventListener('click', () => {
            if (app.contextMenuTargetFile) {
                this.showRenameFileDialog(app.contextMenuTargetFile);
            }
            ui.closeFileContextMenu();
        });

        document.getElementById('move-file-option').addEventListener('click', () => {
            if (app.contextMenuTargetFile) {
                this.showMoveDialog(app.contextMenuTargetFile, 'file');
            }
            ui.closeFileContextMenu();
        });

        document.getElementById('share-file-option').addEventListener('click', () => {
            const file = app.contextMenuTargetFile;
            if (file) {
                this.showShareDialog(file, 'file');
            }
            ui.closeFileContextMenu();
        });

        document.getElementById('add-to-playlist-option').addEventListener('click', () => {
            const file = app.contextMenuTargetFile;
            if (file) {
                this.showPlaylistDialog(file);
            }
            ui.closeFileContextMenu();
        });

        document.getElementById('playlist-add-btn').addEventListener('click', () => {
            this.addSelectedFilesToPlaylist();
        });

        document.getElementById('delete-file-option').addEventListener('click', async () => {
            const file = app.contextMenuTargetFile;
            ui.closeFileContextMenu();
            if (file) {
                await fileOps.deleteFile(file.id, file.name);
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
        const moveFileDialog = document.getElementById('move-file-dialog');

        moveCancelBtn.addEventListener('click', this.closeMoveDialog);

        // Close move dialog on Escape key
        // Store handler reference to avoid duplicate listeners
        // Note: We don't use stopPropagation because all Escape handlers are on document level
        // Each handler checks its own state, so multiple dialogs can be closed with multiple Escape presses
        if (!_moveDialogEscapeHandler) {
            _moveDialogEscapeHandler = (e) => {
                if (e.key === 'Escape' && !moveFileDialog?.classList.contains('hidden')) {
                    this.closeMoveDialog();
                }
            };
            document.addEventListener('keydown', _moveDialogEscapeHandler);
        }

        // Copy button handler
        copyConfirmBtn.addEventListener('click', async () => {
            // Batch copy mode (from multiSelect)
            if (app.moveDialogMode === 'batch' && multiSelect) {
                const targetId = app.selectedTargetFolderId;
                const items = app.batchMoveItems || [];

                const fileIds = items.filter((i) => i.type === 'file').map((i) => i.id);
                const folderIds = items.filter((i) => i.type === 'folder').map((i) => i.id);

                const result = await fileOps.batchCopy(fileIds, folderIds, targetId);

                this.closeMoveDialog();
                multiSelect.clear();
                loadFiles();

                multiSelect.showBatchResult('copy', result);
                return;
            }

            // Single item copy
            if (app.moveDialogMode === 'file' && app.contextMenuTargetFile) {
                const success = await fileOps.copyFile(app.contextMenuTargetFile.id, app.selectedTargetFolderId);
                if (success) {
                    this.closeMoveDialog();
                }
            } else if (app.moveDialogMode === 'folder' && app.contextMenuTargetFolder) {
                const success = await fileOps.copyFolder(app.contextMenuTargetFolder.id, app.selectedTargetFolderId);
                if (success) {
                    this.closeMoveDialog();
                }
            }
        });

        moveConfirmBtn.addEventListener('click', async () => {
            // Batch move mode (from multiSelect)
            if (app.moveDialogMode === 'batch' && multiSelect) {
                const targetId = app.selectedTargetFolderId;
                const items = app.batchMoveItems || [];

                const fileIds = items.filter((i) => i.type === 'file').map((i) => i.id);
                const folderIds = items.filter((i) => i.type === 'folder' && i.id !== targetId).map((i) => i.id);

                const result = await fileOps.batchMove(fileIds, folderIds, targetId);

                this.closeMoveDialog();
                multiSelect.clear();
                loadFiles();
                multiSelect.showBatchResult('move', result);

                return;
            }

            if (app.moveDialogMode === 'file' && app.contextMenuTargetFile) {
                const success = await fileOps.moveFile(app.contextMenuTargetFile.id, app.selectedTargetFolderId);
                if (success) {
                    this.closeMoveDialog();
                }
            } else if (app.moveDialogMode === 'folder' && app.contextMenuTargetFolder) {
                const success = await fileOps.moveFolder(app.contextMenuTargetFolder.id, app.selectedTargetFolderId);
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

        app.renameMode = 'folder';
        // Store the folder reference so it survives context menu cleanup
        app.renameTarget = folder;
        renameInput.value = folder.name;
        // Update header text
        const headerSpan = renameDialog.querySelector('.rename-dialog-header span');
        if (headerSpan) headerSpan.textContent = i18n.t('dialogs.rename_folder');
        renameDialog?.classList.remove('hidden');
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

        app.renameMode = 'file';
        // Store the file reference so it survives context menu cleanup
        app.renameTarget = file;
        renameInput.value = file.name;
        // Update header text
        const headerSpan = renameDialog.querySelector('.rename-dialog-header span');
        if (headerSpan) headerSpan.textContent = i18n.t('dialogs.rename_file');
        renameDialog?.classList.remove('hidden');
        renameInput.focus();
        renameInput.select();
    },

    /**
     * Close rename dialog
     */
    closeRenameDialog() {
        document.getElementById('rename-dialog')?.classList.add('hidden');
        app.contextMenuTargetFolder = null;
        app.renameTarget = null;
    },

    /**
     * Show move dialog for a file or folder
     * @param {Object} item - File or folder object
     * @param {string} mode - 'file' or 'folder'
     */
    async showMoveDialog(item, mode) {
        // Set mode
        app.moveDialogMode = mode;

        // Reset selection
        app.selectedTargetFolderId = '';

        // Ensure we have the home folder ID BEFORE calculating startFolderId
        if (!app.userHomeFolderId) {
            console.log('[Move Dialog] Home folder ID not set, resolving...');
            await resolveHomeFolder();
        }

        // Initialize dialog navigation state
        // Start at the parent of the item being moved (so user sees siblings and can navigate)
        let startFolderId = null;
        let startFolderName = null;
        if (mode === 'file' && item.folder_id) {
            startFolderId = item.folder_id;
            // We need the folder name for breadcrumb - try to get it from current view
            const folderEl = document.querySelector(`[data-folder-id="${startFolderId}"]`);
            if (folderEl) {
                startFolderName = folderEl.querySelector('.folder-name, .item-name')?.textContent || null;
            }
        } else if (mode === 'folder' && item.parent_id) {
            startFolderId = item.parent_id;
        } else {
            // If item is at root level, start at user's home folder
            startFolderId = app.userHomeFolderId || null;
        }

        console.log('[Move Dialog] showMoveDialog - item:', item, 'mode:', mode, 'startFolderId:', startFolderId, 'userHomeFolderId:', app.userHomeFolderId);

        // Store the item being moved and navigation state
        app.moveDialogItemId = item.id;
        app.moveDialogItemMode = mode;
        app.moveDialogCurrentFolderId = startFolderId;

        // Build initial breadcrumb if starting at a non-home folder
        // This allows proper navigation back to home
        const breadcrumb = [];
        if (startFolderId && startFolderId !== app.userHomeFolderId && startFolderName) {
            // We have the folder name, add it to breadcrumb
            breadcrumb.push({ id: startFolderId, name: startFolderName });
        }
        app.moveDialogBreadcrumb = breadcrumb;

        // Update dialog title (preserve icon)
        const dialogHeader = document.getElementById('move-file-dialog').querySelector('.rename-dialog-header');
        const titleText = mode === 'file' ? i18n.t('dialogs.move_file') : i18n.t('dialogs.move_folder');
        dialogHeader.innerHTML = `<i class="fas fa-arrows-alt dialog-header-icon"></i> <span>${titleText}</span>`;

        // Load folders for the starting location
        await this.loadMoveDialogFolders(startFolderId);

        // Show dialog
        document.getElementById('move-file-dialog')?.classList.remove('hidden');
    },

    /**
     * Close move dialog
     */
    closeMoveDialog() {
        document.getElementById('move-file-dialog')?.classList.add('hidden');
        app.contextMenuTargetFile = null;
        app.contextMenuTargetFolder = null;
    },

    /**
     * Rename the selected folder or file
     */
    async renameItem() {
        const newName = document.getElementById('rename-input').value.trim();
        if (!newName) {
            alert(i18n.t('errors.empty_name'));
            return;
        }

        // Use renameTarget which was saved before the context menu was closed
        const target = app.renameTarget;
        if (!target) {
            console.error('No rename target available');
            return;
        }

        if (app.renameMode === 'file') {
            const success = await fileOps.renameFile(target.id, newName);
            if (success) {
                contextMenus.closeRenameDialog();
                loadFiles();
            }
        } else if (app.renameMode === 'folder') {
            const success = await fileOps.renameFolder(target.id, newName);
            if (success) {
                contextMenus.closeRenameDialog();
                loadFiles();
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
            // Ensure we have the home folder ID before proceeding
            if (!app.userHomeFolderId) {
                await resolveHomeFolder();
            }

            // Get the effective folder ID
            const effectiveParentId = parentFolderId || app.userHomeFolderId;

            // Must have a folder ID to proceed
            if (!effectiveParentId) {
                console.error('[Move Dialog] Cannot load folders - no folder ID available');
                return;
            }

            // Use the contents endpoint to get children
            const url = `/api/folders/${effectiveParentId}/contents`;

            console.log('[Move Dialog] Loading folders from:', url, 'effectiveParentId:', effectiveParentId);
            const response = await fetch(url, { credentials: 'same-origin' });
            if (!response.ok) {
                console.error('Failed to load folders:', response.status);
                return;
            }

            const data = await response.json();
            console.log('[Move Dialog] API response:', data);

            // The contents endpoint returns an array of child folders
            // The fallback /api/folders returns root folders (home folder itself)
            const folders = Array.isArray(data) ? data : data.folders || [];
            console.log('[Move Dialog] Loaded folders:', folders.length, 'folders:', folders);

            const folderSelectContainer = document.getElementById('folder-select-container');
            const breadcrumbContainer = document.getElementById('move-dialog-breadcrumb');

            // Clear container
            folderSelectContainer.innerHTML = '';

            // Get current navigation state
            const itemId = app.moveDialogItemId;
            const mode = app.moveDialogItemMode;
            const breadcrumb = app.moveDialogBreadcrumb || [];

            // Always show breadcrumb to allow navigation back to home
            this._renderMoveDialogBreadcrumb(breadcrumbContainer, breadcrumb, effectiveParentId);
            breadcrumbContainer.style.display = 'flex';

            // Option to select current folder as destination (only after navigating into subfolders)
            if (breadcrumb.length > 0 && effectiveParentId && effectiveParentId !== itemId) {
                const currentFolderOption = document.createElement('div');
                currentFolderOption.className = 'folder-select-item folder-select-current';
                currentFolderOption.innerHTML = `
                    <i class="fas fa-check-circle check-icon"></i>
                    <span>${i18n.t('dialogs.select_this_folder')}</span>
                `;
                currentFolderOption.addEventListener('click', () => {
                    document.querySelectorAll('.folder-select-item').forEach((item) => {
                        item.classList.remove('selected');
                    });
                    currentFolderOption.classList.add('selected');
                    app.selectedTargetFolderId = effectiveParentId;
                });
                folderSelectContainer.appendChild(currentFolderOption);
            }

            // Add "Go to parent" option if not at home folder
            const isAtHomeFolder = effectiveParentId === app.userHomeFolderId;
            if (!isAtHomeFolder || breadcrumb.length > 0) {
                const parentOption = document.createElement('div');
                parentOption.className = 'folder-select-item folder-navigate-up';
                parentOption.innerHTML = `
                    <i class="fas fa-level-up-alt"></i>
                    <span>${i18n.t('dialogs.go_to_parent')}</span>
                `;
                parentOption.addEventListener('click', () => {
                    // Navigate to parent folder
                    const currentBreadcrumb = app.moveDialogBreadcrumb || [];
                    if (currentBreadcrumb.length > 0) {
                        // Remove current folder from breadcrumb
                        currentBreadcrumb.pop();
                        const parentFolder = currentBreadcrumb.length > 0 ? currentBreadcrumb[currentBreadcrumb.length - 1] : null;
                        app.moveDialogBreadcrumb = currentBreadcrumb;
                        app.moveDialogCurrentFolderId = parentFolder ? parentFolder.id : null;
                        this.loadMoveDialogFolders(parentFolder ? parentFolder.id : null);
                    } else {
                        // Go to root (home folder)
                        app.moveDialogBreadcrumb = [];
                        app.moveDialogCurrentFolderId = app.userHomeFolderId || null;
                        this.loadMoveDialogFolders(app.userHomeFolderId || null);
                    }
                });
                folderSelectContainer.appendChild(parentOption);
            }

            // Add subfolders (clicking navigates INTO the folder)
            folders.forEach((folder) => {
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
                    const breadcrumb = app.moveDialogBreadcrumb || [];
                    breadcrumb.push({ id: folder.id, name: folder.name });
                    app.moveDialogBreadcrumb = breadcrumb;
                    app.moveDialogCurrentFolderId = folder.id;
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
                    <i class="fas fa-check-circle check-icon"></i>
                    <span>${i18n.t('dialogs.move_to_home')}</span>
                `;
                homeOption.addEventListener('click', () => {
                    document.querySelectorAll('.folder-select-item').forEach((item) => {
                        item.classList.remove('selected');
                    });
                    homeOption.classList.add('selected');
                    app.selectedTargetFolderId = ''; // Empty means root/home
                });
                folderSelectContainer.appendChild(homeOption);
            } else if (folders.length === 0) {
                // Inside a subfolder with no children - show empty message
                const emptyMsg = document.createElement('div');
                emptyMsg.className = 'folder-select-empty';
                emptyMsg.innerHTML = `<i class="fas fa-folder-open"></i> <span>${i18n.t('dialogs.no_subfolders')}</span>`;
                folderSelectContainer.appendChild(emptyMsg);
            }

            // Set default selection to current folder
            app.selectedTargetFolderId = parentFolderId || '';

            // Translate new elements
            i18n.translateElement(folderSelectContainer);
        } catch (error) {
            console.error('Error loading folders:', error);
        }
    },

    /**
     * Render breadcrumb navigation for move dialog
     */
    _renderMoveDialogBreadcrumb(container, breadcrumb, _currentFolderId) {
        if (!container) return;
        container.innerHTML = '';

        const homeFolderId = app.userHomeFolderId;
        const homeFolderName = app.userHomeFolderName || 'Home';

        // Home icon (click to go to home folder)
        const homeItem = document.createElement('span');
        homeItem.className = 'move-breadcrumb-item';
        homeItem.innerHTML = '<i class="fas fa-home"></i>';
        homeItem.addEventListener('click', () => {
            app.moveDialogBreadcrumb = [];
            app.moveDialogCurrentFolderId = homeFolderId || null;
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
                    app.moveDialogBreadcrumb = [];
                    app.moveDialogCurrentFolderId = homeFolderId || null;
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
                    app.moveDialogBreadcrumb = breadcrumb.slice(0, index + 1);
                    app.moveDialogCurrentFolderId = segment.id;
                    this.loadMoveDialogFolders(segment.id);
                });
            }
            container.appendChild(item);
        });
    },

    /**
     * Load all folders for the move dialog (batch operations)
     * Uses the same navigation pattern as loadMoveDialogFolders
     * @param {string} itemId - ID of the item being moved (unused, kept for compatibility)
     * @param {string} mode - 'batch' for batch operations
     */
    async loadAllFolders(_itemId, _mode) {
        // For batch mode, use the same navigation as regular move dialog
        // Initialize navigation state starting at home folder
        app.moveDialogBreadcrumb = [];
        app.moveDialogCurrentFolderId = app.userHomeFolderId || null;

        // Use loadMoveDialogFolders which uses /api/folders/{id}/contents
        await this.loadMoveDialogFolders(app.userHomeFolderId || null);
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
                ui.showNotification('Error', 'Share dialog not available');
                return;
            }

            // Update dialog title — use the <span> inside header to preserve <i> icon
            const dialogHeader = shareDialog.querySelector('.share-dialog-header');
            if (dialogHeader) {
                const headerSpan = dialogHeader.querySelector('span');
                const titleText =
                    itemType === 'file' ? i18n.t('dialogs.share_file') : i18n.t('dialogs.share_folder');
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
            app.shareDialogItem = item;
            app.shareDialogItemType = itemType;

            // Check if item already has shares (async API call)
            const existingShares = await fileSharing.getSharedLinksForItem(item.id, itemType);
            const existingSharesContainer = document.getElementById('existing-shares-container');

            // Clear existing shares container
            existingSharesContainer.innerHTML = '';

            if (existingShares.length > 0) {
                document.getElementById('existing-shares-section').classList.remove('hidden');

                // Create elements for each existing share
                existingShares.forEach((share) => {
                    const shareEl = document.createElement('div');
                    shareEl.className = 'existing-share-item';

                    const expiresText = share.expires_at ? `Expires: ${fileSharing.formatExpirationDate(share.expires_at)}` : 'No expiration';

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
                document.querySelectorAll('.copy-link-btn').forEach((btn) => {
                    btn.addEventListener('click', (e) => {
                        e.preventDefault();
                        const url = btn.getAttribute('data-share-url');
                        fileSharing.copyLinkToClipboard(url);
                    });
                });

                document.querySelectorAll('.delete-link-btn').forEach((btn) => {
                    btn.addEventListener('click', (e) => {
                        e.preventDefault();
                        const shareId = btn.getAttribute('data-share-id');

                        showConfirmDialog({
                            title: i18n.t('dialogs.confirm_delete_share'),
                            message: i18n.t('dialogs.confirm_delete_share_msg'),
                            confirmText: i18n.t('actions.delete')
                        }).then(async (confirmed) => {
                            if (confirmed) {
                                await fileSharing.removeSharedLink(shareId);
                                btn.closest('.existing-share-item').remove();
                                if (existingSharesContainer.children.length === 0) {
                                    document.getElementById('existing-shares-section').classList.add('hidden');
                                    ui.setSharedVisualState(item.id, item.type, false);
                                }
                            }
                        });
                    });
                });
            } else {
                document.getElementById('existing-shares-section').classList.add('hidden');
            }

            // Hide new-share section from previous use
            const newShareSection = document.getElementById('new-share-section');
            if (newShareSection) newShareSection.classList.add('hidden');

            // Show dialog
            shareDialog.classList.remove('hidden');
            console.log('Share dialog opened for', itemType, item.name);
        } catch (error) {
            console.error('Error opening share dialog:', error);
            ui.showNotification('Error', 'Could not open share dialog');
        }
    },

    /**
     * Create a shared link with the configured options
     */
    async createSharedLink() {
        if (!app.shareDialogItem || !app.shareDialogItemType) {
            ui.showNotification('Error', 'Could not share the item');
            return;
        }

        // Get values from form
        const password = document.getElementById('share-password').value;
        const expirationDate = document.getElementById('share-expiration').value;
        const permissionRead = document.getElementById('share-permission-read').checked;
        const permissionWrite = document.getElementById('share-permission-write').checked;
        const permissionReshare = document.getElementById('share-permission-reshare').checked;

        const item = app.shareDialogItem;
        const itemType = app.shareDialogItemType;

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
            const headers = {
                'Content-Type': 'application/json',
                ...getCsrfHeaders()
            };

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
                document.getElementById('new-share-section').classList.remove('hidden');
                shareUrl.focus();
                shareUrl.select();
            }

            // Update Item's shared badge
            ui.setSharedVisualState(item.id, item.type, true);

            // Show success message
            ui.showNotification(
                i18n.t('notifications.link_created'),
                i18n.t('notifications.share_success')
            );
        } catch (error) {
            console.error('Error creating shared link:', error);
            ui.showNotification('Error', error.message || 'Could not create shared link');
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
        app.notificationShareUrl = shareUrl;

        // Show dialog
        document.getElementById('notification-dialog')?.classList.remove('hidden');
    },

    /**
     * Send share notification email
     */
    sendShareNotification() {
        const email = document.getElementById('notification-email').value.trim();
        const message = document.getElementById('notification-message').value.trim();
        const shareUrl = app.notificationShareUrl;

        if (!email || !shareUrl) {
            ui.showNotification('Error', 'Please enter a valid email address');
            return;
        }

        // Validate email format
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        if (!emailRegex.test(email)) {
            ui.showNotification('Error', 'Please enter a valid email address');
            return;
        }

        try {
            fileSharing.sendShareNotification(shareUrl, email, message);
            document.getElementById('notification-dialog')?.classList.add('hidden');
        } catch (error) {
            console.error('Error sending notification:', error);
            ui.showNotification('Error', 'Could not send notification');
        }
    },

    /**
     * Close share dialog
     */
    closeShareDialog() {
        const dialog = document.getElementById('share-dialog');
        if (dialog) dialog.classList.add('hidden');
        app.shareDialogItem = null;
        app.shareDialogItemType = null;
    },

    /**
     * Close notification dialog
     */
    closeNotificationDialog() {
        document.getElementById('notification-dialog')?.classList.add('hidden');
        app.notificationShareUrl = null;
    },

    _selectedPlaylistId: null,

    async showPlaylistDialog(file) {
        const dialog = document.getElementById('playlist-dialog');
        const container = document.getElementById('playlist-select-container');
        const filesInfo = document.getElementById('playlist-dialog-files-info');

        if (!dialog || !container) {
            console.error('Playlist dialog elements not found');
            return;
        }

        // Store the file(s) to add
        app.playlistDialogFiles = [file];

        // Update files info
        if (filesInfo) {
            filesInfo.innerHTML = `<strong>${i18n.t('music.selected_files')} </strong>${file.name}`;
        }

        // Reset selection
        this._selectedPlaylistId = null;
        container.innerHTML = '<div class="folder-select-loading"><i class="fas fa-spinner fa-spin"></i></div>';

        // Reset add button state
        const addBtn = document.getElementById('playlist-add-btn');
        if (addBtn) addBtn.disabled = true;

        // Show dialog
        dialog.classList.remove('hidden');
        requestAnimationFrame(() => dialog.classList.add('active'));

        // Load playlists
        try {
            const resp = await fetch('/api/playlists', { credentials: 'include' });
            if (!resp.ok) throw new Error('Failed to load playlists');

            const playlists = await resp.json();
            this._renderPlaylistSelect(container, playlists);
        } catch (err) {
            console.error('Error loading playlists:', err);
            container.innerHTML = `<div class="folder-select-empty">${i18n.t('music.load_error')}</div>`;
        }
    },

    _renderPlaylistSelect(container, playlists) {
        const t = i18n.t.bind(i18n);

        container.innerHTML = '';

        if (playlists.length === 0) {
            container.innerHTML = `<div class="folder-select-empty">${t('music.no_playlists', 'No playlists yet. Create one first!')}</div>`;
            return;
        }

        playlists.forEach((playlist) => {
            const item = document.createElement('div');
            item.className = 'folder-select-item';
            item.dataset.id = playlist.id;
            item.innerHTML = `
                <i class="fas fa-list"></i>
                <span>${this._escapeHtml(playlist.name)}</span>
                <span class="playlist-track-count">${playlist.track_count || 0} ${t('music.tracks', 'tracks')}</span>
            `;

            item.addEventListener('click', () => {
                container.querySelectorAll('.folder-select-item').forEach((el) => {
                    el.classList.remove('selected');
                });
                item.classList.add('selected');
                this._selectedPlaylistId = playlist.id;
                const addBtn = document.getElementById('playlist-add-btn');
                if (addBtn) addBtn.disabled = false;
            });

            container.appendChild(item);
        });
    },

    async addSelectedFilesToPlaylist() {
        const playlistId = this._selectedPlaylistId;
        const files = app.playlistDialogFiles || [];

        if (!playlistId || files.length === 0) return;

        const addBtn = document.getElementById('playlist-add-btn');
        if (addBtn) addBtn.disabled = true;

        try {
            const resp = await fetch(`/api/playlists/${playlistId}/tracks`, {
                method: 'POST',
                credentials: 'include',
                headers: {
                    'Content-Type': 'application/json',
                    ...getCsrfHeaders()
                },
                body: JSON.stringify({ file_ids: files.map((f) => f.id) })
            });

            if (!resp.ok) {
                const err = await resp.json().catch(() => ({}));
                throw new Error(err.message || 'Failed to add tracks');
            }

            await resp.json();
            ui.showNotification(
                i18n.t('music.added'),
                `${files.length} ${files.length === 1 ? 'track' : 'tracks'} ${i18n.t('music.added_to_playlist')}`
            );

            this.closePlaylistDialog();

            // Refresh music view if open
            if (musicView?.playlists) {
                musicView._loadPlaylists();
            }
        } catch (err) {
            console.error('Error adding to playlist:', err);
            ui.showNotification(i18n.t('music.error'), err.message || i18n.t('music.add_error'));
            if (addBtn) addBtn.disabled = false;
        }
    },

    closePlaylistDialog() {
        const dialog = document.getElementById('playlist-dialog');
        if (dialog) {
            dialog.classList.remove('active');
            setTimeout(() => {
                dialog.classList.add('hidden');
            }, 200);
        }
        app.playlistDialogFiles = null;
        this._selectedPlaylistId = null;
    },

    _escapeHtml(str) {
        if (!str) return '';
        return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
    }
};

export { contextMenus };
