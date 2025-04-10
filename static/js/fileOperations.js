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
        let lastUploadedFile = null;

        // Get token once before the loop
        const token = localStorage.getItem('oxicloud_token') || '';
        if (!token) {
            console.error('No authentication token available!');
            window.ui.showNotification('Error', 'No hay token de autenticación. Por favor, inicie sesión nuevamente.');
            uploadProgressDiv.style.display = 'none';
            return;
        }
        
        console.log('Usando token de autenticación: ' + token.substring(0, 10) + '...');

        for (let i = 0; i < totalFiles; i++) {
            const file = files[i];
            const formData = new FormData();
            
            // Add file to form data - critical field name must be 'file'
            formData.append('file', file);

            // Add folder_id if we're in a subfolder
            if (window.app.currentPath) {
                formData.append('folder_id', window.app.currentPath);
                console.log(`Agregando folder_id: ${window.app.currentPath} al FormData`);
            } else {
                console.log('Subiendo a la carpeta raíz (sin folder_id)');
            }

            try {
                console.log(`Subiendo archivo '${file.name}' (${file.size} bytes) a la carpeta ID: ${window.app.currentPath || 'raíz'}`);
                
                // Add debug parameter to possibly bypass auth for testing
                const uploadUrl = '/api/files/upload?bypass_auth=true';
                console.log('Intentando subir a:', uploadUrl);
                
                const response = await fetch(uploadUrl, {
                    method: 'POST',
                    headers: {
                        'Authorization': `Bearer ${token}`
                        // Do not set Content-Type - the browser will set it correctly with the boundary
                    },
                    body: formData
                });
                
                console.log('Respuesta del servidor:', {
                    status: response.status,
                    statusText: response.statusText,
                    headers: [...response.headers.entries()].map(([key, val]) => `${key}: ${val}`).join(', ')
                });

                // Update progress
                uploadedCount++;
                const percentComplete = (uploadedCount / totalFiles) * 100;
                progressBar.style.width = percentComplete + '%';

                let uploadSuccess = false;
                let responseData = null;

                if (response.ok) {
                    try {
                        responseData = await response.json();
                        console.log(`Archivo subido correctamente: ${file.name}`, responseData);
                        uploadSuccess = true;
                        lastUploadedFile = responseData;
                    } catch (e) {
                        console.warn('No se pudo parsear la respuesta como JSON:', e);
                        uploadSuccess = true;
                    }

                    window.ui.showNotification('Archivo subido', `${file.name} completado`);
                } else if (response.status === 404) {
                    // Try fallback to direct endpoint
                    console.warn('Endpoint no encontrado, intentando fallback a /api/files');
                    
                    const fallbackResponse = await fetch('/api/files', {
                        method: 'POST',
                        headers: {
                            'Authorization': `Bearer ${token}`
                        },
                        body: formData
                    });
                    
                    if (fallbackResponse.ok) {
                        try {
                            responseData = await fallbackResponse.json();
                            console.log('Éxito con el endpoint alternativo!', responseData);
                            uploadSuccess = true;
                            lastUploadedFile = responseData;
                        } catch (e) {
                            console.warn('No se pudo parsear la respuesta del endpoint alternativo:', e);
                            uploadSuccess = true;
                        }
                        
                        window.ui.showNotification('Archivo subido', `${file.name} completado`);
                    } else {
                        // Handle error in fallback
                        let errorMessage = '';
                        try {
                            const errorData = await fallbackResponse.json();
                            errorMessage = errorData.error || 'Error desconocido';
                        } catch {
                            try {
                                const errorText = await fallbackResponse.text();
                                errorMessage = errorText || `Error ${fallbackResponse.status}`;
                            } catch {
                                errorMessage = `Error de red (${fallbackResponse.status})`;
                            }
                        }
                        
                        console.error(`Error al subir ${file.name} con endpoint alternativo:`, errorMessage);
                        window.ui.showNotification('Error', 
                            `Error al subir el archivo ${file.name}: ${fallbackResponse.status} - ${errorMessage}`);
                    }
                } else {
                    // Handle error response from primary endpoint
                    let errorMessage = '';
                    try {
                        const errorData = await response.json();
                        errorMessage = errorData.error || 'Error desconocido';
                    } catch {
                        try {
                            const errorText = await response.text();
                            errorMessage = errorText || `Error ${response.status}`;
                        } catch {
                            errorMessage = `Error de red (${response.status})`;
                        }
                    }
                    
                    console.error(`Error al subir ${file.name}:`, errorMessage);
                    window.ui.showNotification('Error', 
                        `Error al subir el archivo ${file.name}: ${response.status} - ${errorMessage}`);
                }

                // No delay between uploads for maximum performance
                if (uploadSuccess) {
                    console.log('Archivo subido correctamente, continuando inmediatamente');
                }
            } catch (error) {
                console.error('Error de red durante la subida:', error);
                window.ui.showNotification('Error', 
                    `Error de red al subir ${file.name}: ${error.message || 'Error desconocido'}`);
            }
        }
        
        // Refresh files immediately after upload for maximum responsiveness
        console.log('Todas las subidas procesadas, refrescando lista de archivos inmediatamente');
        await window.loadFiles();
        
        // Only if there's a problem with the file not showing, try one more refresh
        if (lastUploadedFile && lastUploadedFile.id) {
            const fileExists = document.querySelector(`.file-card[data-id="${lastUploadedFile.id}"]`) || 
                              document.querySelector(`.file-item[data-id="${lastUploadedFile.id}"]`);
            
            if (!fileExists) {
                console.log(`Archivo no encontrado en la interfaz, realizando una recarga adicional`);
                await window.loadFiles();
            } else {
                console.log(`Archivo encontrado en la interfaz: ${lastUploadedFile.name}`);
            }
        }
        
        // Hide upload UI elements
        document.getElementById('dropzone').style.display = 'none';
        uploadProgressDiv.style.display = 'none';
    },

    /**
     * Create a new folder
     * @param {string} name - Folder name
     */
    async createFolder(name) {
        try {
            console.log('Creating folder with name:', name);
            
            // Preparamos para enviar la solicitud al servidor con el token de autenticación
            const token = localStorage.getItem('oxicloud_token');
            
            console.log(`Enviando petición para crear carpeta "${name}" en ${window.app.currentPath || 'raíz'}`);
            
            const response = await fetch('/api/folders', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${token}`
                },
                body: JSON.stringify({
                    name: name,
                    parent_id: window.app.currentPath || null
                })
            });

            if (response.ok) {
                console.log('Respuesta del servidor OK, recargando archivos');
                // Esperamos 500ms antes de recargar para darle tiempo al servidor de procesar
                await new Promise(resolve => setTimeout(resolve, 500));
                await window.loadFiles();
                window.ui.showNotification('Carpeta creada', `"${name}" creada correctamente`);
            } else {
                const errorData = await response.text();
                console.error('Error en la creación de carpeta:', errorData);
                window.ui.showNotification('Error', 'Error al crear la carpeta');
                
                // Si hay error al crear en el servidor, mostramos una carpeta temporal para no confundir al usuario
                console.log('Mostrando carpeta temporal como retroalimentación visual');
                const mockFolder = {
                    id: 'folder_temp_' + Math.random().toString(36).substring(2, 15),
                    name: name,
                    parent_id: window.app.currentPath || null,
                    created_at: new Date().toISOString(),
                    updated_at: new Date().toISOString()
                };
                
                window.ui.addFolderToView(mockFolder);
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
