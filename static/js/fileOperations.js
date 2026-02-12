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
    /**
     * Upload files to the server
     * @param {FileList} files - Files to upload
     */
    async uploadFiles(files) {
        const progressBar = document.querySelector('.progress-fill');
        const uploadProgressDiv = document.querySelector('.upload-progress');
        uploadProgressDiv.style.display = 'block';
        progressBar.style.width = '0%';

        let uploadedCount = 0;
        const totalFiles = files.length;

        for (let i = 0; i < totalFiles; i++) {
            const file = files[i];
            const formData = new FormData();
            
            // IMPORTANT: folder_id MUST be added BEFORE file for multipart processing
            // The backend reads fields in order, and needs folder_id before processing the file
            const targetFolderId = window.app.currentPath || window.app.userHomeFolderId;
            
            if (targetFolderId) {
                formData.append('folder_id', targetFolderId);
            }
            
            // Add the file AFTER folder_id
            formData.append('file', file);

            try {
                console.log(`Uploading file to folder: ${targetFolderId || 'root'}`);
                
                // We use the correct URL for file upload
                console.log('Form to submit:', {
                    file: file.name,
                    size: file.size,
                    folder_id: targetFolderId || 'root'
                });
                
                const response = await fetch('/api/files/upload', {
                    method: 'POST',
                    body: formData,
                    // Add cache: 'no-store' to avoid cache issues during upload
                    cache: 'no-store',
                    headers: {
                        ...getAuthHeaders(),
                        // Add this header to force fresh reloads
                        'Cache-Control': 'no-cache, no-store, must-revalidate'
                    }
                });
                
                console.log('Server response:', {
                    status: response.status,
                    statusText: response.statusText
                });

                // Update progress
                uploadedCount++;
                const percentComplete = (uploadedCount / totalFiles) * 100;
                progressBar.style.width = percentComplete + '%';

                if (response.ok) {
                    const responseData = await response.json();
                    console.log(`Successfully uploaded ${file.name}`, responseData);
                    
                    // Show success notification immediately
                    window.ui.showNotification('File uploaded', `${file.name} completed`);

                    if (i === totalFiles - 1) {
                        // Last file uploaded - wait and reload once
                        console.log('Last file uploaded, waiting before reloading...');
                        
                        // Wait for backend to persist
                        await new Promise(resolve => setTimeout(resolve, 800));
                        
                        // Single reload with force refresh
                        try {
                            await window.loadFiles({forceRefresh: true});
                        } catch (reloadError) {
                            console.error("Error reloading files:", reloadError);
                        }
                        
                        // Hide upload UI
                        setTimeout(() => {
                            const dropzone = document.getElementById('dropzone');
                            if (dropzone) dropzone.style.display = 'none';
                            uploadProgressDiv.style.display = 'none';
                        }, 500);
                    }
                } else {
                    const errorData = await response.text();
                    console.error('Upload error:', errorData);
                    window.ui.showNotification('Error', `Error uploading file: ${file.name}`);
                }
            } catch (error) {
                console.error('Network error during upload:', error);
                window.ui.showNotification('Error', `Network error uploading file: ${file.name}`);
            }
        }
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
        uploadProgressDiv.style.display = 'block';
        progressBar.style.width = '0%';

        const currentFolderId = window.app.currentPath || window.app.userHomeFolderId;
        
        // Build folder structure from relative paths
        // webkitRelativePath looks like: "folderName/subfolder/file.txt"
        const folderMap = new Map(); // path -> folder_id
        folderMap.set('', currentFolderId); // root = current folder
        
        // Collect all unique folder paths
        const folderPaths = new Set();
        for (const file of files) {
            const parts = file.webkitRelativePath.split('/');
            // Remove filename, keep folder parts
            for (let i = 1; i < parts.length; i++) {
                const path = parts.slice(0, i).join('/');
                folderPaths.add(path);
            }
        }
        
        // Sort paths by depth so parents are created first
        const sortedPaths = [...folderPaths].sort((a, b) => 
            a.split('/').length - b.split('/').length
        );
        
        // Create folders
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
        
        // Upload files into their respective folders
        let uploadedCount = 0;
        const totalFiles = files.length;
        
        for (let i = 0; i < totalFiles; i++) {
            const file = files[i];
            const parts = file.webkitRelativePath.split('/');
            const parentPath = parts.slice(0, -1).join('/');
            const targetFolderId = folderMap.get(parentPath) || currentFolderId;
            
            const formData = new FormData();
            formData.append('folder_id', targetFolderId);
            formData.append('file', file);
            
            try {
                const response = await fetch('/api/files/upload', {
                    method: 'POST',
                    body: formData,
                    cache: 'no-store',
                    headers: {
                        ...getAuthHeaders(),
                        'Cache-Control': 'no-cache, no-store, must-revalidate'
                    }
                });
                
                uploadedCount++;
                const percentComplete = (uploadedCount / totalFiles) * 100;
                progressBar.style.width = percentComplete + '%';
                
                if (response.ok) {
                    console.log(`Uploaded: ${file.webkitRelativePath}`);
                } else {
                    console.error(`Error uploading ${file.webkitRelativePath}:`, await response.text());
                }
            } catch (error) {
                console.error(`Network error uploading ${file.webkitRelativePath}:`, error);
            }
        }
        
        // Finish up
        window.ui.showNotification('Folder uploaded', `${uploadedCount} files uploaded successfully`);
        
        await new Promise(resolve => setTimeout(resolve, 800));
        
        try {
            await window.loadFiles({ forceRefresh: true });
        } catch (reloadError) {
            console.error('Error reloading files:', reloadError);
        }
        
        setTimeout(() => {
            const dropzone = document.getElementById('dropzone');
            if (dropzone) dropzone.style.display = 'none';
            uploadProgressDiv.style.display = 'none';
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
