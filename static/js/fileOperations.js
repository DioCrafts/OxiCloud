/**
 * OxiCloud - File Operations Module
 * This file handles file and folder operations (create, move, delete, rename, upload)
 */

/**
 * Get authorization headers for API requests
 * @returns {Object} Headers object with Authorization bearer token
 */
function getAuthHeaders() {
    const token = localStorage.getItem('oxicloud_token');
    const headers = {};
    if (token) {
        headers['Authorization'] = `Bearer ${token}`;
    }
    return headers;
}

// File Operations Module
const fileOps = {

    // ========================================================================
    // Upload progress — notification bell integration
    // ========================================================================
    _currentBatchId: null,

    /** Start a new upload batch in the notification bell */
    _initUploadToast(totalFiles) {
        this._currentBatchId = window.notifications
            ? window.notifications.addUploadBatch(totalFiles)
            : null;
    },

    /** Finalise the batch in the notification bell */
    _finishUploadToast(successCount, totalFiles) {
        if (window.notifications && this._currentBatchId) {
            window.notifications.finishBatch(this._currentBatchId, successCount, totalFiles);
        }
    },

    /**
     * Upload a single file via XMLHttpRequest with progress events.
     * Progress is reported to the notification bell via batchId + fileName.
     * Returns a promise that resolves with { ok, data?, errorMsg?, isQuotaError? }.
     */
    _uploadFileXHR(formData, batchId, fileName) {
        return new Promise((resolve) => {
            const xhr = new XMLHttpRequest();
            const notif = window.notifications;

            xhr.upload.addEventListener('progress', (e) => {
                if (e.lengthComputable && notif && batchId) {
                    const pct = Math.round((e.loaded / e.total) * 100);
                    notif.updateFile(batchId, fileName, pct, 'uploading');
                }
            });

            xhr.addEventListener('load', () => {
                if (xhr.status >= 200 && xhr.status < 300) {
                    if (notif && batchId) notif.updateFile(batchId, fileName, 100, 'done');
                    let data = null;
                    try { data = JSON.parse(xhr.responseText); } catch (_) {}
                    resolve({ ok: true, data });
                } else {
                    if (notif && batchId) notif.updateFile(batchId, fileName, 0, 'error');
                    // Parse error body for quota-exceeded or other messages
                    let errorMsg = null;
                    let isQuotaError = false;
                    try {
                        const errBody = JSON.parse(xhr.responseText);
                        errorMsg = errBody.error || null;
                        isQuotaError = errBody.error_type === 'QuotaExceeded' || xhr.status === 507;
                    } catch (_) {}
                    resolve({ ok: false, errorMsg, isQuotaError });
                }
            });

            xhr.addEventListener('error', () => {
                if (notif && batchId) notif.updateFile(batchId, fileName, 0, 'error');
                resolve({ ok: false });
            });

            xhr.open('POST', '/api/files/upload');

            // Set auth header
            const token = localStorage.getItem('oxicloud_token');
            if (token) xhr.setRequestHeader('Authorization', `Bearer ${token}`);
            xhr.setRequestHeader('Cache-Control', 'no-cache, no-store, must-revalidate');

            xhr.send(formData);
        });
    },

    // ========================================================================
    // Upload files (via button or drag-and-drop)
    // ========================================================================

    /**
     * Upload files to the server with real-time progress indication
     * @param {FileList} files - Files to upload
     */
    async uploadFiles(files) {
        const totalFiles = files.length;
        if (totalFiles === 0) return;

        // Legacy progress bar (inside dropzone) — keep working for drag-drop
        const progressBar = document.querySelector('.progress-fill');
        const uploadProgressDiv = document.querySelector('.upload-progress');
        if (uploadProgressDiv) { uploadProgressDiv.style.display = 'block'; }
        if (progressBar) { progressBar.style.width = '0%'; }

        // Show upload notification
        this._initUploadToast(totalFiles);
        const batchId = this._currentBatchId;

        let uploadedCount = 0;
        let successCount = 0;

        for (let i = 0; i < totalFiles; i++) {
            const file = files[i];
            const formData = new FormData();

            const targetFolderId = window.app.currentPath || window.app.userHomeFolderId;
            if (targetFolderId) formData.append('folder_id', targetFolderId);
            formData.append('file', file);

            console.log(`Uploading file to folder: ${targetFolderId || 'root'}`, {
                file: file.name, size: file.size
            });

            const result = await this._uploadFileXHR(formData, batchId, file.name);

            uploadedCount++;

            // Legacy dropzone bar
            if (progressBar) {
                progressBar.style.width = ((uploadedCount / totalFiles) * 100) + '%';
            }
            // Notify bell of per-file completion
            if (window.notifications && batchId) {
                window.notifications.fileCompleted(batchId, result.ok);
            }

            if (result.ok) {
                successCount++;
                console.log(`Successfully uploaded ${file.name}`, result.data);
            } else {
                console.error(`Upload error for ${file.name}`);
                if (result.isQuotaError) {
                    const msg = result.errorMsg || window.i18n?.t('storage_quota_exceeded') || 'Storage quota exceeded';
                    if (window.notifications) {
                        window.notifications.addNotification({
                            icon: 'fa-exclamation-triangle',
                            iconClass: 'error',
                            title: file.name,
                            text: msg
                        });
                    }
                    break;
                }
            }
        }

        // All done
        this._finishUploadToast(successCount, totalFiles);

        // Wait for backend to persist, then reload
        await new Promise(resolve => setTimeout(resolve, 800));

        // Refresh storage usage display
        if (typeof window.refreshUserData === 'function') {
            try { await window.refreshUserData(); } catch (_) {}
        }

        try {
            await window.loadFiles({ forceRefresh: true });
        } catch (reloadError) {
            console.error('Error reloading files:', reloadError);
        }

        setTimeout(() => {
            const dropzone = document.getElementById('dropzone');
            if (dropzone) dropzone.style.display = 'none';
            if (uploadProgressDiv) uploadProgressDiv.style.display = 'none';
        }, 500);
    },

    /**
     * Upload folder files maintaining directory structure
     * Creates subfolders as needed, then uploads files into them
     * @param {FileList} files - Files from folder input (with webkitRelativePath)
     */
    async uploadFolderFiles(files) {
        if (!files || files.length === 0) return;
        
        const progressBar = document.querySelector('.progress-fill');
        const uploadProgressDiv = document.querySelector('.upload-progress');
        if (uploadProgressDiv) { uploadProgressDiv.style.display = 'block'; }
        if (progressBar) { progressBar.style.width = '0%'; }

        const currentFolderId = window.app.currentPath || window.app.userHomeFolderId;
        
        // Build folder structure from relative paths
        const folderMap = new Map();
        folderMap.set('', currentFolderId);
        
        const folderPaths = new Set();
        for (const file of files) {
            const parts = file.webkitRelativePath.split('/');
            for (let i = 1; i < parts.length; i++) {
                const path = parts.slice(0, i).join('/');
                folderPaths.add(path);
            }
        }
        
        const sortedPaths = [...folderPaths].sort((a, b) => 
            a.split('/').length - b.split('/').length
        );
        
        // Create folders first (no progress toast for folder creation)
        for (const folderPath of sortedPaths) {
            const parts = folderPath.split('/');
            const folderName = parts[parts.length - 1];
            const parentPath = parts.slice(0, -1).join('/');
            const parentId = folderMap.get(parentPath) || currentFolderId;
            
            try {
                const response = await fetch('/api/folders', {
                    method: 'POST',
                    headers: {
                        ...getAuthHeaders(),
                        'Content-Type': 'application/json',
                        'Cache-Control': 'no-cache, no-store, must-revalidate'
                    },
                    body: JSON.stringify({
                        name: folderName,
                        parent_id: parentId
                    })
                });
                
                if (response.ok) {
                    const folder = await response.json();
                    folderMap.set(folderPath, folder.id);
                    console.log(`Created folder: ${folderPath} -> ${folder.id}`);
                } else {
                    console.error(`Error creating folder ${folderPath}:`, await response.text());
                    window.ui.showNotification('Error', `Error creating folder: ${folderName}`);
                }
            } catch (error) {
                console.error(`Network error creating folder ${folderPath}:`, error);
            }
        }
        
        // Upload files with notification bell
        const totalFiles = files.length;
        this._initUploadToast(totalFiles);
        const batchId = this._currentBatchId;

        let uploadedCount = 0;
        let successCount = 0;
        
        for (let i = 0; i < totalFiles; i++) {
            const file = files[i];
            const parts = file.webkitRelativePath.split('/');
            const parentPath = parts.slice(0, -1).join('/');
            const targetFolderId = folderMap.get(parentPath) || currentFolderId;
            
            const formData = new FormData();
            formData.append('folder_id', targetFolderId);
            formData.append('file', file);

            const displayName = file.webkitRelativePath || file.name;

            const result = await this._uploadFileXHR(formData, batchId, displayName);
            
            uploadedCount++;
            if (progressBar) {
                progressBar.style.width = ((uploadedCount / totalFiles) * 100) + '%';
            }
            if (window.notifications && batchId) {
                window.notifications.fileCompleted(batchId, result.ok);
            }
                
            if (result.ok) {
                successCount++;
                console.log(`Uploaded: ${file.webkitRelativePath}`);
            } else {
                console.error(`Error uploading ${file.webkitRelativePath}`);
                if (result.isQuotaError) {
                    const msg = result.errorMsg || window.i18n?.t('storage_quota_exceeded') || 'Storage quota exceeded';
                    if (window.notifications) {
                        window.notifications.addNotification({
                            icon: 'fa-exclamation-triangle',
                            iconClass: 'error',
                            title: file.name,
                            text: msg
                        });
                    }
                    break;
                }
            }
        }
        
        // Finish
        this._finishUploadToast(successCount, totalFiles);
        
        await new Promise(resolve => setTimeout(resolve, 800));

        // Refresh storage usage display
        if (typeof window.refreshUserData === 'function') {
            try { await window.refreshUserData(); } catch (_) {}
        }
        
        try {
            await window.loadFiles({ forceRefresh: true });
        } catch (reloadError) {
            console.error('Error reloading files:', reloadError);
        }
        
        setTimeout(() => {
            const dropzone = document.getElementById('dropzone');
            if (dropzone) dropzone.style.display = 'none';
            if (uploadProgressDiv) uploadProgressDiv.style.display = 'none';
        }, 500);
    },

    /**
     * Create a new folder
     * @param {string} name - Folder name
     */
    async createFolder(name) {
        try {
            console.log('Creating folder with name:', name);
            
            // Send the actual request to the backend to create the folder
            const response = await fetch('/api/folders', {
                method: 'POST',
                headers: {
                    ...getAuthHeaders(),
                    'Content-Type': 'application/json',
                    'Cache-Control': 'no-cache, no-store, must-revalidate'
                },
                body: JSON.stringify({
                    name: name,
                    parent_id: window.app.currentPath || null
                })
            });

            if (response.ok) {
                // Get the created folder from the backend
                const folder = await response.json();
                console.log('Folder created successfully:', folder);
                
                // Add the folder to the view immediately for instant feedback
                window.ui.addFolderToView(folder);
                
                // Wait to allow the backend to save the changes
                await new Promise(resolve => setTimeout(resolve, 1000));
                
                // Reload files to refresh the view
                await window.loadFiles({forceRefresh: true});
                
                window.ui.showNotification('Folder created', `"${name}" created successfully`);
            } else {
                const errorData = await response.text();
                console.error('Create folder error:', errorData);
                window.ui.showNotification('Error', 'Error creating the folder');
            }
        } catch (error) {
            console.error('Error creating folder:', error);
            window.ui.showNotification('Error', 'Error creating the folder');
        }
    },

    /**
     * Move a file to another folder
     * @param {string} fileId - File ID
     * @param {string} targetFolderId - Target folder ID
     * @returns {Promise<boolean>} - Success status
     */
    async moveFile(fileId, targetFolderId) {
        try {
            const response = await fetch(`/api/files/${fileId}/move`, {
                method: 'PUT',
                headers: {
                    ...getAuthHeaders(),
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    folder_id: targetFolderId === "" ? null : targetFolderId
                })
            });

            if (response.ok) {
                // Reload files after moving
                await window.loadFiles();
                window.ui.showNotification('File moved', 'File moved successfully');
                return true;
            } else {
                let errorMessage = 'Unknown error';
                try {
                    const errorData = await response.json();
                    errorMessage = errorData.error || 'Unknown error';
                } catch (e) {
                    errorMessage = 'Error processing server response';
                }
                window.ui.showNotification('Error', `Error moving the file: ${errorMessage}`);
                return false;
            }
        } catch (error) {
            console.error('Error moving file:', error);
            window.ui.showNotification('Error', 'Error moving the file');
            return false;
        }
    },

    /**
     * Move a folder to another folder
     * @param {string} folderId - Folder ID
     * @param {string} targetFolderId - Target folder ID
     * @returns {Promise<boolean>} - Success status
     */
    async moveFolder(folderId, targetFolderId) {
        try {
            const response = await fetch(`/api/folders/${folderId}/move`, {
                method: 'PUT',
                headers: {
                    ...getAuthHeaders(),
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    parent_id: targetFolderId === "" ? null : targetFolderId
                })
            });

            if (response.ok) {
                // Reload files after moving
                await window.loadFiles();
                window.ui.showNotification('Folder moved', 'Folder moved successfully');
                return true;
            } else {
                let errorMessage = 'Unknown error';
                try {
                    const errorData = await response.json();
                    errorMessage = errorData.error || 'Unknown error';
                } catch (e) {
                    errorMessage = 'Error processing server response';
                }
                window.ui.showNotification('Error', `Error moving the folder: ${errorMessage}`);
                return false;
            }
        } catch (error) {
            console.error('Error moving folder:', error);
            window.ui.showNotification('Error', 'Error moving the folder');
            return false;
        }
    },

    /**
     * Rename a file
     * @param {string} fileId - File ID
     * @param {string} newName - New file name
     * @returns {Promise<boolean>} - Success status
     */
    async renameFile(fileId, newName) {
        try {
            console.log(`Renaming file ${fileId} to "${newName}"`);

            const response = await fetch(`/api/files/${fileId}/rename`, {
                method: 'PUT',
                headers: {
                    ...getAuthHeaders(),
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ name: newName })
            });

            console.log('Response status:', response.status);

            if (response.ok) {
                window.ui.showNotification(
                    window.i18n ? window.i18n.t('notifications.file_renamed') : 'File renamed',
                    window.i18n ? window.i18n.t('notifications.file_renamed_to', { name: newName }) : `File renamed to "${newName}"`
                );
                return true;
            } else {
                const errorText = await response.text();
                console.error('Error response:', errorText);
                let errorMessage = 'Unknown error';
                try {
                    const errorData = JSON.parse(errorText);
                    errorMessage = errorData.error || response.statusText;
                } catch (e) {
                    errorMessage = errorText || response.statusText;
                }
                window.ui.showNotification('Error', `Error renaming the file: ${errorMessage}`);
                return false;
            }
        } catch (error) {
            console.error('Error renaming file:', error);
            window.ui.showNotification('Error', 'Error renaming the file');
            return false;
        }
    },

    /**
     * Rename a folder
     * @param {string} folderId - Folder ID
     * @param {string} newName - New folder name
     * @returns {Promise<boolean>} - Success status
     */
    async renameFolder(folderId, newName) {
        try {
            console.log(`Renaming folder ${folderId} to "${newName}"`);

            const response = await fetch(`/api/folders/${folderId}/rename`, {
                method: 'PUT',
                headers: {
                    ...getAuthHeaders(),
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ name: newName })
            });

            console.log('Response status:', response.status);

            if (response.ok) {
                window.ui.showNotification('Folder renamed', `Folder renamed to "${newName}"`);
                return true;
            } else {
                const errorText = await response.text();
                console.error('Error response:', errorText);

                let errorMessage = 'Unknown error';
                try {
                    // Try to parse as JSON
                    const errorData = JSON.parse(errorText);
                    errorMessage = errorData.error || response.statusText;
                } catch (e) {
                    // If not JSON, use text as is
                    errorMessage = errorText || response.statusText;
                }

                window.ui.showNotification('Error', `Error renaming the folder: ${errorMessage}`);
                return false;
            }
        } catch (error) {
            console.error('Error renaming folder:', error);
            window.ui.showNotification('Error', 'Error renaming the folder');
            return false;
        }
    },

    /**
     * Move a file to trash
     * @param {string} fileId - File ID
     * @param {string} fileName - File name
     * @returns {Promise<boolean>} - Success status
     */
    async deleteFile(fileId, fileName) {
        const confirmed = await showConfirmDialog({
            title: window.i18n ? window.i18n.t('dialogs.confirm_delete') : 'Move to trash',
            message: window.i18n ? window.i18n.t('dialogs.confirm_delete_file', { name: fileName }) : `Are you sure you want to move the file "${fileName}" to trash?`,
            confirmText: window.i18n ? window.i18n.t('actions.delete') : 'Delete',
        });
        if (!confirmed) return false;
        
        try {
            // Use the trash API endpoint
            const response = await fetch(`/api/trash/files/${fileId}`, {
                method: 'DELETE',
                headers: getAuthHeaders()
            });

            if (response.ok) {
                window.loadFiles();
                window.ui.showNotification('File moved to trash', `"${fileName}" moved to trash`);
                return true;
            } else {
                // Fallback to direct deletion if trash fails
                const fallbackResponse = await fetch(`/api/files/${fileId}`, {
                    method: 'DELETE',
                    headers: getAuthHeaders()
                });
                
                if (fallbackResponse.ok) {
                    window.loadFiles();
                    window.ui.showNotification('File deleted', `"${fileName}" deleted successfully`);
                    return true;
                } else {
                    window.ui.showNotification('Error', 'Error deleting the file');
                    return false;
                }
            }
        } catch (error) {
            console.error('Error deleting file:', error);
            window.ui.showNotification('Error', 'Error deleting the file');
            return false;
        }
    },

    /**
     * Move a folder to trash
     * @param {string} folderId - Folder ID
     * @param {string} folderName - Folder name
     * @returns {Promise<boolean>} - Success status
     */
    async deleteFolder(folderId, folderName) {
        const confirmed = await showConfirmDialog({
            title: window.i18n ? window.i18n.t('dialogs.confirm_delete') : 'Move to trash',
            message: window.i18n ? window.i18n.t('dialogs.confirm_delete_folder', { name: folderName }) : `Are you sure you want to move the folder "${folderName}" and all its contents to trash?`,
            confirmText: window.i18n ? window.i18n.t('actions.delete') : 'Delete',
        });
        if (!confirmed) return false;
        
        try {
            // Use the trash API endpoint
            const response = await fetch(`/api/trash/folders/${folderId}`, {
                method: 'DELETE',
                headers: getAuthHeaders()
            });

            if (response.ok) {
                // If we're inside the folder we just deleted, go back up
                if (window.app.currentPath === folderId) {
                    window.app.currentPath = '';
                    window.ui.updateBreadcrumb('');
                }
                window.loadFiles();
                window.ui.showNotification('Folder moved to trash', `"${folderName}" moved to trash`);
                return true;
            } else {
                // Fallback to direct deletion if trash fails
                const fallbackResponse = await fetch(`/api/folders/${folderId}`, {
                    method: 'DELETE',
                    headers: getAuthHeaders()
                });
                
                if (fallbackResponse.ok) {
                    // If we're inside the folder we just deleted, go back up
                    if (window.app.currentPath === folderId) {
                        window.app.currentPath = '';
                        window.ui.updateBreadcrumb('');
                    }
                    window.loadFiles();
                    window.ui.showNotification('Folder deleted', `"${folderName}" deleted successfully`);
                    return true;
                } else {
                    window.ui.showNotification('Error', 'Error deleting the folder');
                    return false;
                }
            }
        } catch (error) {
            console.error('Error deleting folder:', error);
            window.ui.showNotification('Error', 'Error deleting the folder');
            return false;
        }
    },
    
    /**
     * Get trash items
     * @returns {Promise<Array>} - List of trash items
     */
    async getTrashItems() {
        try {
            const response = await fetch('/api/trash', {
                headers: getAuthHeaders()
            });
            
            if (response.ok) {
                return await response.json();
            } else {
                console.error('Error fetching trash items:', response.statusText);
                return [];
            }
        } catch (error) {
            console.error('Error fetching trash items:', error);
            return [];
        }
    },
    
    /**
     * Restore an item from trash
     * @param {string} trashId - Trash item ID
     * @returns {Promise<boolean>} - Operation success
     */
    async restoreFromTrash(trashId) {
        try {
            const response = await fetch(`/api/trash/${trashId}/restore`, {
                method: 'POST',
                headers: {
                    ...getAuthHeaders(),
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({})
            });
            
            if (response.ok) {
                window.ui.showNotification('Item restored', 'Item restored successfully');
                return true;
            } else {
                window.ui.showNotification('Error', 'Error restoring the item');
                return false;
            }
        } catch (error) {
            console.error('Error restoring item from trash:', error);
            window.ui.showNotification('Error', 'Error restoring the item');
            return false;
        }
    },
    
    /**
     * Permanently delete a trash item
     * @param {string} trashId - Trash item ID
     * @returns {Promise<boolean>} - Operation success
     */
    async deletePermanently(trashId) {
        const confirmed = await showConfirmDialog({
            title: window.i18n ? window.i18n.t('dialogs.confirm_permanent_delete') : 'Delete permanently',
            message: window.i18n ? window.i18n.t('dialogs.confirm_permanent_delete_msg') : 'Are you sure you want to permanently delete this item? This action cannot be undone.',
            confirmText: window.i18n ? window.i18n.t('actions.delete_permanently') : 'Delete permanently',
        });
        if (!confirmed) return false;
        
        try {
            const response = await fetch(`/api/trash/${trashId}`, {
                method: 'DELETE',
                headers: getAuthHeaders()
            });
            
            if (response.ok) {
                window.ui.showNotification('Item deleted', 'Item permanently deleted');
                return true;
            } else {
                window.ui.showNotification('Error', 'Error deleting the item');
                return false;
            }
        } catch (error) {
            console.error('Error deleting item permanently:', error);
            window.ui.showNotification('Error', 'Error deleting the item');
            return false;
        }
    },
    
    /**
     * Empty the trash
     * @returns {Promise<boolean>} - Operation success
     */
    async emptyTrash() {
        const confirmed = await showConfirmDialog({
            title: window.i18n ? window.i18n.t('dialogs.confirm_empty_trash') : 'Empty trash',
            message: window.i18n ? window.i18n.t('trash.empty_confirm') : 'Are you sure you want to empty the trash? This action will permanently delete all items.',
            confirmText: window.i18n ? window.i18n.t('actions.empty_trash') : 'Empty trash',
        });
        if (!confirmed) return false;
        
        try {
            const response = await fetch('/api/trash/empty', {
                method: 'DELETE',
                headers: getAuthHeaders()
            });
            
            if (response.ok) {
                window.ui.showNotification('Trash emptied', 'The trash has been emptied successfully');
                return true;
            } else {
                window.ui.showNotification('Error', 'Error emptying the trash');
                return false;
            }
        } catch (error) {
            console.error('Error emptying trash:', error);
            window.ui.showNotification('Error', 'Error emptying the trash');
            return false;
        }
    },
    
    /**
     * Download a file
     * @param {string} fileId - File ID
     * @param {string} fileName - File name
     */
    async downloadFile(fileId, fileName) {
        try {
            const response = await fetch(`/api/files/${fileId}`, {
                headers: getAuthHeaders()
            });
            if (response.ok) {
                const blob = await response.blob();
                const url = URL.createObjectURL(blob);
                const link = document.createElement('a');
                link.href = url;
                link.download = fileName;
                document.body.appendChild(link);
                link.click();
                document.body.removeChild(link);
                URL.revokeObjectURL(url);
            } else {
                window.ui.showNotification('Error', 'Error downloading the file');
            }
        } catch (error) {
            console.error('Error downloading file:', error);
            window.ui.showNotification('Error', 'Error downloading the file');
        }
    },
    
    /**
     * Download a folder as ZIP
     * @param {string} folderId - Folder ID
     * @param {string} folderName - Folder name
     */
    async downloadFolder(folderId, folderName) {
        try {
            // Show notification to user
            window.ui.showNotification('Preparing download', 'Preparing the folder for download...');
            
            const response = await fetch(`/api/folders/${folderId}/download?format=zip`, {
                headers: getAuthHeaders()
            });
            if (response.ok) {
                const blob = await response.blob();
                const url = URL.createObjectURL(blob);
                const link = document.createElement('a');
                link.href = url;
                link.download = `${folderName}.zip`;
                document.body.appendChild(link);
                link.click();
                document.body.removeChild(link);
                URL.revokeObjectURL(url);
            } else {
                window.ui.showNotification('Error', 'Error downloading the folder');
            }
        } catch (error) {
            console.error('Error downloading folder:', error);
            window.ui.showNotification('Error', 'Error downloading the folder');
        }
    }
};

// Expose file operations module globally
window.fileOps = fileOps;
