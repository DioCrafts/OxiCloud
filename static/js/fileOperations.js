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
                
                // Usamos la URL correcta para la subida de archivos
                console.log('Formulario a enviar:', {
                    file: file.name,
                    size: file.size,
                    folder_id: targetFolderId || 'root'
                });
                
                const response = await fetch('/api/files/upload', {
                    method: 'POST',
                    body: formData,
                    // Añadir cache: 'no-store' para evitar problemas de caché durante la subida
                    cache: 'no-store',
                    headers: {
                        ...getAuthHeaders(),
                        // Agregar este encabezado para forzar recargas frescas
                        'Cache-Control': 'no-cache, no-store, must-revalidate'
                    }
                });
                
                console.log('Respuesta del servidor:', {
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
                    window.ui.showNotification('Archivo subido', `${file.name} completado`);

                    if (i === totalFiles - 1) {
                        // Last file uploaded - wait and reload once
                        console.log('Último archivo subido, esperando antes de recargar...');
                        
                        // Wait for backend to persist
                        await new Promise(resolve => setTimeout(resolve, 800));
                        
                        // Single reload with force refresh
                        try {
                            await window.loadFiles({forceRefresh: true});
                        } catch (reloadError) {
                            console.error("Error recargando archivos:", reloadError);
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
                    window.ui.showNotification('Error', `Error al subir el archivo: ${file.name}`);
                }
            } catch (error) {
                console.error('Network error during upload:', error);
                window.ui.showNotification('Error', `Error de red al subir el archivo: ${file.name}`);
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
                    window.ui.showNotification('Error', `Error creando carpeta: ${folderName}`);
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
        window.ui.showNotification('Carpeta subida', `${uploadedCount} archivos subidos correctamente`);
        
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
            
            // Enviar la solicitud real al backend para crear la carpeta
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
                // Obtener la carpeta creada del backend
                const folder = await response.json();
                console.log('Folder created successfully:', folder);
                
                // Añadir la carpeta a la vista de inmediato para feedback instantáneo
                window.ui.addFolderToView(folder);
                
                // Esperar para permitir que el backend guarde los cambios
                await new Promise(resolve => setTimeout(resolve, 1000));
                
                // Recargar los archivos para refrescar la vista
                await window.loadFiles({forceRefresh: true});
                
                window.ui.showNotification('Carpeta creada', `"${name}" creada correctamente`);
            } else {
                const errorData = await response.text();
                console.error('Create folder error:', errorData);
                window.ui.showNotification('Error', 'Error al crear la carpeta');
            }
        } catch (error) {
            console.error('Error creating folder:', error);
            window.ui.showNotification('Error', 'Error al crear la carpeta');
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
                window.ui.showNotification('Archivo movido', 'Archivo movido correctamente');
                return true;
            } else {
                let errorMessage = 'Error desconocido';
                try {
                    const errorData = await response.json();
                    errorMessage = errorData.error || 'Error desconocido';
                } catch (e) {
                    errorMessage = 'Error al procesar la respuesta del servidor';
                }
                window.ui.showNotification('Error', `Error al mover el archivo: ${errorMessage}`);
                return false;
            }
        } catch (error) {
            console.error('Error moving file:', error);
            window.ui.showNotification('Error', 'Error al mover el archivo');
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
                window.ui.showNotification('Carpeta movida', 'Carpeta movida correctamente');
                return true;
            } else {
                let errorMessage = 'Error desconocido';
                try {
                    const errorData = await response.json();
                    errorMessage = errorData.error || 'Error desconocido';
                } catch (e) {
                    errorMessage = 'Error al procesar la respuesta del servidor';
                }
                window.ui.showNotification('Error', `Error al mover la carpeta: ${errorMessage}`);
                return false;
            }
        } catch (error) {
            console.error('Error moving folder:', error);
            window.ui.showNotification('Error', 'Error al mover la carpeta');
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
                    window.i18n ? window.i18n.t('notifications.file_renamed') : 'Archivo renombrado',
                    window.i18n ? window.i18n.t('notifications.file_renamed_to', { name: newName }) : `Archivo renombrado a "${newName}"`
                );
                return true;
            } else {
                const errorText = await response.text();
                console.error('Error response:', errorText);
                let errorMessage = 'Error desconocido';
                try {
                    const errorData = JSON.parse(errorText);
                    errorMessage = errorData.error || response.statusText;
                } catch (e) {
                    errorMessage = errorText || response.statusText;
                }
                window.ui.showNotification('Error', `Error al renombrar el archivo: ${errorMessage}`);
                return false;
            }
        } catch (error) {
            console.error('Error renaming file:', error);
            window.ui.showNotification('Error', 'Error al renombrar el archivo');
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
                window.ui.showNotification('Carpeta renombrada', `Carpeta renombrada a "${newName}"`);
                return true;
            } else {
                const errorText = await response.text();
                console.error('Error response:', errorText);

                let errorMessage = 'Error desconocido';
                try {
                    // Try to parse as JSON
                    const errorData = JSON.parse(errorText);
                    errorMessage = errorData.error || response.statusText;
                } catch (e) {
                    // If not JSON, use text as is
                    errorMessage = errorText || response.statusText;
                }

                window.ui.showNotification('Error', `Error al renombrar la carpeta: ${errorMessage}`);
                return false;
            }
        } catch (error) {
            console.error('Error renaming folder:', error);
            window.ui.showNotification('Error', 'Error al renombrar la carpeta');
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
            title: window.i18n ? window.i18n.t('dialogs.confirm_delete') : 'Mover a papelera',
            message: window.i18n ? window.i18n.t('dialogs.confirm_delete_file', { name: fileName }) : `¿Estás seguro de que quieres mover a la papelera el archivo "${fileName}"?`,
            confirmText: window.i18n ? window.i18n.t('actions.delete') : 'Eliminar',
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
                window.ui.showNotification('Archivo movido a papelera', `"${fileName}" movido a la papelera`);
                return true;
            } else {
                // Fallback to direct deletion if trash fails
                const fallbackResponse = await fetch(`/api/files/${fileId}`, {
                    method: 'DELETE',
                    headers: getAuthHeaders()
                });
                
                if (fallbackResponse.ok) {
                    window.loadFiles();
                    window.ui.showNotification('Archivo eliminado', `"${fileName}" eliminado correctamente`);
                    return true;
                } else {
                    window.ui.showNotification('Error', 'Error al eliminar el archivo');
                    return false;
                }
            }
        } catch (error) {
            console.error('Error deleting file:', error);
            window.ui.showNotification('Error', 'Error al eliminar el archivo');
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
            title: window.i18n ? window.i18n.t('dialogs.confirm_delete') : 'Mover a papelera',
            message: window.i18n ? window.i18n.t('dialogs.confirm_delete_folder', { name: folderName }) : `¿Estás seguro de que quieres mover a la papelera la carpeta "${folderName}" y todo su contenido?`,
            confirmText: window.i18n ? window.i18n.t('actions.delete') : 'Eliminar',
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
                window.ui.showNotification('Carpeta movida a papelera', `"${folderName}" movida a la papelera`);
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
                    window.ui.showNotification('Carpeta eliminada', `"${folderName}" eliminada correctamente`);
                    return true;
                } else {
                    window.ui.showNotification('Error', 'Error al eliminar la carpeta');
                    return false;
                }
            }
        } catch (error) {
            console.error('Error deleting folder:', error);
            window.ui.showNotification('Error', 'Error al eliminar la carpeta');
            return false;
        }
    },
    
    /**
     * Obtener elementos de la papelera
     * @returns {Promise<Array>} - Lista de elementos en la papelera
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
     * Restaurar un elemento desde la papelera
     * @param {string} trashId - ID del elemento en la papelera
     * @returns {Promise<boolean>} - Éxito de la operación
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
                window.ui.showNotification('Elemento restaurado', 'Elemento restaurado correctamente');
                return true;
            } else {
                window.ui.showNotification('Error', 'Error al restaurar el elemento');
                return false;
            }
        } catch (error) {
            console.error('Error restoring item from trash:', error);
            window.ui.showNotification('Error', 'Error al restaurar el elemento');
            return false;
        }
    },
    
    /**
     * Eliminar permanentemente un elemento de la papelera
     * @param {string} trashId - ID del elemento en la papelera
     * @returns {Promise<boolean>} - Éxito de la operación
     */
    async deletePermanently(trashId) {
        const confirmed = await showConfirmDialog({
            title: window.i18n ? window.i18n.t('dialogs.confirm_permanent_delete') : 'Eliminar permanentemente',
            message: window.i18n ? window.i18n.t('dialogs.confirm_permanent_delete_msg') : '¿Estás seguro de que quieres eliminar permanentemente este elemento? Esta acción no se puede deshacer.',
            confirmText: window.i18n ? window.i18n.t('actions.delete_permanently') : 'Eliminar permanentemente',
        });
        if (!confirmed) return false;
        
        try {
            const response = await fetch(`/api/trash/${trashId}`, {
                method: 'DELETE',
                headers: getAuthHeaders()
            });
            
            if (response.ok) {
                window.ui.showNotification('Elemento eliminado', 'Elemento eliminado permanentemente');
                return true;
            } else {
                window.ui.showNotification('Error', 'Error al eliminar el elemento');
                return false;
            }
        } catch (error) {
            console.error('Error deleting item permanently:', error);
            window.ui.showNotification('Error', 'Error al eliminar el elemento');
            return false;
        }
    },
    
    /**
     * Vaciar la papelera
     * @returns {Promise<boolean>} - Éxito de la operación
     */
    async emptyTrash() {
        const confirmed = await showConfirmDialog({
            title: window.i18n ? window.i18n.t('dialogs.confirm_empty_trash') : 'Vaciar papelera',
            message: window.i18n ? window.i18n.t('trash.empty_confirm') : '¿Estás seguro de que quieres vaciar la papelera? Esta acción eliminará permanentemente todos los elementos.',
            confirmText: window.i18n ? window.i18n.t('actions.empty_trash') : 'Vaciar papelera',
        });
        if (!confirmed) return false;
        
        try {
            const response = await fetch('/api/trash/empty', {
                method: 'DELETE',
                headers: getAuthHeaders()
            });
            
            if (response.ok) {
                window.ui.showNotification('Papelera vaciada', 'La papelera ha sido vaciada correctamente');
                return true;
            } else {
                window.ui.showNotification('Error', 'Error al vaciar la papelera');
                return false;
            }
        } catch (error) {
            console.error('Error emptying trash:', error);
            window.ui.showNotification('Error', 'Error al vaciar la papelera');
            return false;
        }
    },
    
    /**
     * Descargar un archivo
     * @param {string} fileId - ID del archivo
     * @param {string} fileName - Nombre del archivo
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
                window.ui.showNotification('Error', 'Error al descargar el archivo');
            }
        } catch (error) {
            console.error('Error downloading file:', error);
            window.ui.showNotification('Error', 'Error al descargar el archivo');
        }
    },
    
    /**
     * Descargar una carpeta como ZIP
     * @param {string} folderId - ID de la carpeta
     * @param {string} folderName - Nombre de la carpeta
     */
    async downloadFolder(folderId, folderName) {
        try {
            // Show notification to user
            window.ui.showNotification('Preparando descarga', 'Preparando la carpeta para descargar...');
            
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
                window.ui.showNotification('Error', 'Error al descargar la carpeta');
            }
        } catch (error) {
            console.error('Error downloading folder:', error);
            window.ui.showNotification('Error', 'Error al descargar la carpeta');
        }
    }
};

// Expose file operations module globally
window.fileOps = fileOps;
