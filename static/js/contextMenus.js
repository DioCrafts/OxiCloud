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
        document.getElementById('move-file-option').addEventListener('click', () => {
            if (window.app.contextMenuTargetFile) {
                this.showMoveDialog(window.app.contextMenuTargetFile, 'file');
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
        
        // Share file option
        if (document.getElementById('share-file-option')) {
            document.getElementById('share-file-option').addEventListener('click', () => {
                if (window.app.contextMenuTargetFile && window.SharingModule) {
                    window.SharingModule.openShareModal(window.app.contextMenuTargetFile);
                }
                window.ui.closeFileContextMenu();
            });
        }
        
        // Create public link option
        if (document.getElementById('public-link-option')) {
            document.getElementById('public-link-option').addEventListener('click', () => {
                if (window.app.contextMenuTargetFile && window.SharingModule) {
                    window.SharingModule.openPublicLinkModal(window.app.contextMenuTargetFile);
                }
                window.ui.closeFileContextMenu();
            });
        }

        // Rename dialog events
        const renameCancelBtn = document.getElementById('rename-cancel-btn');
        const renameConfirmBtn = document.getElementById('rename-confirm-btn');
        const renameInput = document.getElementById('rename-input');

        renameCancelBtn.addEventListener('click', this.closeRenameDialog);
        renameConfirmBtn.addEventListener('click', this.renameFolder);

        // Rename on Enter key
        renameInput.addEventListener('keyup', (e) => {
            if (e.key === 'Enter') {
                this.renameFolder();
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

        renameInput.value = folder.name;
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

        // Update dialog title
        const dialogHeader = document.getElementById('move-file-dialog').querySelector('.rename-dialog-header');
        dialogHeader.textContent = mode === 'file' ?
            (window.i18n ? window.i18n.t('dialogs.move_file') : 'Mover archivo') :
            (window.i18n ? window.i18n.t('dialogs.move_folder') : 'Mover carpeta');

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
     * Rename the selected folder
     */
    async renameFolder() {
        if (!window.app.contextMenuTargetFolder) return;

        const newName = document.getElementById('rename-input').value.trim();
        if (!newName) {
            alert(window.i18n ? window.i18n.t('errors.empty_name') : 'El nombre no puede estar vacío');
            return;
        }

        const success = await window.fileOps.renameFolder(window.app.contextMenuTargetFolder.id, newName);
        if (success) {
            contextMenus.closeRenameDialog();
            window.loadFiles();
        }
    },

    /**
     * Load all folders for the move dialog
     * @param {string} itemId - ID of the item being moved
     * @param {string} mode - 'file' or 'folder'
     */
    async loadAllFolders(itemId, mode) {
        try {
            const response = await fetch('/api/folders');
            if (response.ok) {
                const folders = await response.json();
                const folderSelectContainer = document.getElementById('folder-select-container');

                // Clear container except root option
                folderSelectContainer.innerHTML = `
                    <div class="folder-select-item selected" data-folder-id="">
                        <i class="fas fa-folder"></i> <span data-i18n="dialogs.root">Raíz</span>
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
     * Add a menu item to a context menu
     * @param {string} menuType - 'file' or 'folder'
     * @param {string} label - Text label for the menu item
     * @param {string} iconClass - FontAwesome icon class
     * @param {string} id - HTML id for the menu item
     * @param {Function} callback - Function to call when clicked
     */
    addMenuItem(menuType, label, iconClass, id, callback) {
        // Determine which menu to add to
        const menuId = menuType === 'file' ? 'file-context-menu' : 'folder-context-menu';
        const menu = document.getElementById(menuId);
        
        if (!menu) {
            console.error(`Menu ${menuId} not found`);
            return;
        }
        
        // Check if item already exists
        if (document.getElementById(id)) {
            console.warn(`Menu item with id ${id} already exists`);
            return;
        }
        
        // Create menu item
        const menuItem = document.createElement('div');
        menuItem.className = 'context-menu-item';
        menuItem.id = id;
        menuItem.innerHTML = `<i class="${iconClass}"></i> <span>${label}</span>`;
        
        // Add click handler
        menuItem.addEventListener('click', () => {
            // Get the target item
            const targetItem = menuType === 'file' 
                ? window.app.contextMenuTargetFile 
                : window.app.contextMenuTargetFolder;
            
            if (targetItem && typeof callback === 'function') {
                callback(targetItem);
            }
            
            // Close the menu
            if (menuType === 'file') {
                window.ui.closeFileContextMenu();
            } else {
                window.ui.closeContextMenu();
            }
        });
        
        // Add to menu (before the delete option)
        const deleteOption = menu.querySelector(`#delete-${menuType}-option`);
        if (deleteOption) {
            menu.insertBefore(menuItem, deleteOption);
        } else {
            menu.appendChild(menuItem);
        }
    }
};

// Expose context menus module globally
window.contextMenus = contextMenus;
