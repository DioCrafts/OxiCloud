/**
 * OxiCloud - File Operations Module
 * This file handles file and folder operations (create, move, delete, rename, upload)
 */

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
        if (!confirm(`¿Estás seguro de que quieres mover a la papelera el archivo "${fileName}"?`)) {
            return false;
        }
        
        try {
            // Use the trash API endpoint
            const response = await fetch(`/api/trash/files/${fileId}`, {
                method: 'DELETE'
            });

            if (response.ok) {
                window.loadFiles();
                window.ui.showNotification('Archivo movido a papelera', `"${fileName}" movido a la papelera`);
                return true;
            } else {
                // Fallback to direct deletion if trash fails
                const fallbackResponse = await fetch(`/api/files/${fileId}`, {
                    method: 'DELETE'
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
        if (!confirm(`¿Estás seguro de que quieres mover a la papelera la carpeta "${folderName}" y todo su contenido?`)) {
            return false;
        }
        
        try {
            // Use the trash API endpoint
            const response = await fetch(`/api/trash/folders/${folderId}`, {
                method: 'DELETE'
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
                    method: 'DELETE'
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
            const response = await fetch('/api/trash');
            
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
        if (!confirm('¿Estás seguro de que quieres eliminar permanentemente este elemento? Esta acción no se puede deshacer.')) {
            return false;
        }
        
        try {
            const response = await fetch(`/api/trash/${trashId}`, {
                method: 'DELETE'
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
        const confirmMsg = window.i18n ? window.i18n.t('trash.empty_confirm') : '¿Estás seguro de que quieres vaciar la papelera? Esta acción eliminará permanentemente todos los elementos.';
        if (!confirm(confirmMsg)) {
            return false;
        }
        
        try {
            const response = await fetch('/api/trash/empty', {
                method: 'DELETE'
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
    downloadFile(fileId, fileName) {
        // Create a link and trigger download
        const link = document.createElement('a');
        link.href = `/api/files/${fileId}`;
        link.download = fileName;
        link.target = '_blank';
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
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
            
            // Request the server to create a ZIP of the folder
            // Since the API might not support this directly, we will simply download with zip parameter
            const link = document.createElement('a');
            link.href = `/api/folders/${folderId}/download?format=zip`;
            link.download = `${folderName}.zip`;
            link.target = '_blank';
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
        } catch (error) {
            console.error('Error downloading folder:', error);
            window.ui.showNotification('Error', 'Error al descargar la carpeta');
        }
    }
};

// Expose file operations module globally
window.fileOps = fileOps;
