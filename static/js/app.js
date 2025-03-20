/**
 * OxiCloud - Main Application
 * This file contains the core functionality, initialization and state management
 */

// Global state
const app = {
    currentView: 'grid',   // Current view mode: 'grid' or 'list'
    currentPath: '',       // Current folder path
    currentFolder: null,   // Current folder object
    contextMenuTargetFolder: null,  // Target folder for context menu
    contextMenuTargetFile: null,    // Target file for context menu
    selectedTargetFolderId: "",     // Selected target folder for move operations
    moveDialogMode: 'file',         // Move dialog mode: 'file' or 'folder'
};

// DOM elements
const elements = {
    // Will be populated on initialization
};

/**
 * Initialize the application
 */
function initApp() {
    // Cache DOM elements
    cacheElements();
    
    // Create menus and dialogs
    ui.initializeContextMenus();
    
    // Setup event listeners
    setupEventListeners();
    
    // Load initial view
    app.currentPath = '';
    ui.updateBreadcrumb('');
    loadFiles();
    
    // Initialize file renderer if available
    if (window.fileRenderer) {
        console.log('Using optimized file renderer');
    } else {
        console.log('Using standard file rendering');
    }
    
    // Check authentication
    checkAuthentication();
}

/**
 * Cache DOM elements for faster access
 */
function cacheElements() {
    elements.uploadBtn = document.getElementById('upload-btn');
    elements.dropzone = document.getElementById('dropzone');
    elements.fileInput = document.getElementById('file-input');
    elements.filesGrid = document.getElementById('files-grid');
    elements.filesListView = document.getElementById('files-list-view');
    elements.newFolderBtn = document.getElementById('new-folder-btn');
    elements.gridViewBtn = document.getElementById('grid-view-btn');
    elements.listViewBtn = document.getElementById('list-view-btn');
    elements.breadcrumb = document.querySelector('.breadcrumb');
    elements.logoutBtn = document.getElementById('logout-btn');
}

/**
 * Setup event listeners for main UI elements
 */
function setupEventListeners() {
    // Set up drag and drop
    ui.setupDragAndDrop();
    
    // Upload button
    elements.uploadBtn.addEventListener('click', () => {
        elements.dropzone.style.display = elements.dropzone.style.display === 'none' ? 'block' : 'none';
        if (elements.dropzone.style.display === 'block') {
            elements.fileInput.click();
        }
    });
    
    // File input
    elements.fileInput.addEventListener('change', (e) => {
        if (e.target.files.length > 0) {
            fileOps.uploadFiles(e.target.files);
        }
    });
    
    // New folder button
    elements.newFolderBtn.addEventListener('click', () => {
        const folderName = prompt(window.i18n ? window.i18n.t('dialogs.new_name') : 'Nombre de la carpeta:');
        if (folderName) {
            fileOps.createFolder(folderName);
        }
    });
    
    // View toggle
    elements.gridViewBtn.addEventListener('click', ui.switchToGridView);
    elements.listViewBtn.addEventListener('click', ui.switchToListView);
    
    // Load saved view preference
    const savedView = localStorage.getItem('oxicloud-view');
    if (savedView === 'list') {
        ui.switchToListView();
    }
    
    // Logout button
    elements.logoutBtn.addEventListener('click', logout);
    
    // Global events to close context menus
    document.addEventListener('click', (e) => {
        const folderMenu = document.getElementById('folder-context-menu');
        const fileMenu = document.getElementById('file-context-menu');
        
        if (folderMenu && folderMenu.style.display === 'block' && 
            !folderMenu.contains(e.target)) {
            ui.closeContextMenu();
        }
        
        if (fileMenu && fileMenu.style.display === 'block' && 
            !fileMenu.contains(e.target)) {
            ui.closeFileContextMenu();
        }
    });
}

/**
 * Load files and folders for the current path
 */
async function loadFiles() {
    try {
        // Check if DOM elements are available
        if (!elements.filesGrid || !elements.filesListView) {
            console.log("DOM elements not available, skip loading files");
            // Try again after a short delay
            setTimeout(loadFiles, 500);
            return;
        }
        
        // Skip loading files if we're on the login page
        if (window.location.pathname === '/login' || window.location.pathname === '/login/') {
            return;
        }
        
        // Show loading indicators
        elements.filesGrid.innerHTML = '<div class="loading-spinner"><div class="spinner-border text-primary" role="status"><span class="visually-hidden">Loading...</span></div></div>';
        elements.filesListView.innerHTML = '<div class="loading-spinner"><div class="spinner-border text-primary" role="status"><span class="visually-hidden">Loading...</span></div></div>';
        
        let url = '/api/folders';
        if (app.currentPath) {
            // Use the correct endpoint for folder contents
            url = `/api/folders/${app.currentPath}/contents`;
        }
        
        // Get auth token if available
        const token = window.AuthModule && typeof AuthModule.getToken === 'function' ? 
            AuthModule.getToken() : localStorage.getItem('oxicloud_token');
        
        if (!token) {
            // No token available, we're probably not logged in
            console.log("No authentication token available");
            
            // Instead of returning, prepare a basic empty view
            elements.filesGrid.innerHTML = '';
            elements.filesListView.innerHTML = `
                <div class="list-header">
                    <div data-i18n="files.name">Nombre</div>
                    <div data-i18n="files.type">Tipo</div>
                    <div data-i18n="files.size">Tamaño</div>
                    <div data-i18n="files.modified">Modificado</div>
                </div>
                <div class="no-files-message">
                    <p>No authentication token available. Please log in.</p>
                </div>
            `;
            return;
        }
        
        // Prepare basic view structure
        elements.filesGrid.innerHTML = '';
        elements.filesListView.innerHTML = `
            <div class="list-header">
                <div data-i18n="files.name">Nombre</div>
                <div data-i18n="files.type">Tipo</div>
                <div data-i18n="files.size">Tamaño</div>
                <div data-i18n="files.modified">Modificado</div>
            </div>
        `;
        
        const headers = { 'Authorization': `Bearer ${token}` };
        
        const response = await fetch(url, { headers });
        if (!response.ok) {
            // If unauthorized, redirect to login
            if (response.status === 401) {
                const baseUrl = window.location.origin;
                window.location.replace(baseUrl + '/login');
                return;
            }
            throw new Error(`Server responded with status: ${response.status}`);
        }
        const folders = await response.json();
        
        // Clear existing files in both views
        elements.filesGrid.innerHTML = '';
        elements.filesListView.innerHTML = `
            <div class="list-header">
                <div data-i18n="files.name">Nombre</div>
                <div data-i18n="files.type">Tipo</div>
                <div data-i18n="files.size">Tamaño</div>
                <div data-i18n="files.modified">Modificado</div>
            </div>
        `;
        
        // Translate the header if i18n is available
        if (window.i18n && window.i18n.translatePage) {
            window.i18n.translatePage();
        }
        
        // Add folders (check if it's an array)
        const folderList = Array.isArray(folders) ? folders : [];
        folderList.forEach(folder => {
            ui.addFolderToView(folder);
        });
        
        // Also load files in this folder
        let filesUrl = '/api/files';
        if (app.currentPath) {
            filesUrl += `?folder_id=${app.currentPath}`;
        }
        
        try {
            // Reutilizar el token y headers que obtuvimos arriba
            const filesResponse = await fetch(filesUrl, { headers });
            
            // Check for auth errors
            if (filesResponse.status === 401) {
                console.log("Authentication failed for files API");
                return;
            }
            
            if (filesResponse.ok) {
                const files = await filesResponse.json();
                
                // Add files (check if it's an array)
                const fileList = Array.isArray(files) ? files : [];
                fileList.forEach(file => {
                    ui.addFileToView(file);
                });
            }
        } catch (error) {
            console.error('Error loading files:', error);
            // File API may not be implemented yet, so we silently ignore this error
        }
        
        // Update file icons based on file type
        ui.updateFileIcons();
    } catch (error) {
        console.error('Error loading folders:', error);
        ui.showNotification('Error', 'Could not load files and folders');
    }
}

/**
 * Format file size in human-readable format
 * @param {number} bytes - Size in bytes
 * @return {string} Formatted size
 */
function formatFileSize(bytes) {
    if (bytes === 0) return '0 Bytes';
    
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

// Expose needed functions to global scope
window.app = app;
window.loadFiles = loadFiles;
window.formatFileSize = formatFileSize;

// Set up global selectFolder function for navigation
window.selectFolder = (id, name) => {
    app.currentPath = id;
    ui.updateBreadcrumb(name);
    loadFiles();
};

/**
 * Check if user is authenticated
 */
function checkAuthentication() {
    // Check for the login page path
    if (window.location.pathname === '/login' || window.location.pathname === '/login/') {
        // Already on login page, no need to redirect
        return;
    }
    
    // Use AuthModule if available, otherwise fallback to direct check
    if (window.AuthModule && typeof AuthModule.isAuthenticated === 'function') {
        if (!AuthModule.isAuthenticated()) {
            // Redirect to login page with absolute URL to avoid query parameters issues
            const baseUrl = window.location.origin;
            window.location.replace(baseUrl + '/login');
            return;
        }
        
        // Display user information using AuthModule
        const userData = AuthModule.getUserData();
        if (userData.username) {
            // Update user avatar with initials
            const userInitials = userData.username.substring(0, 2).toUpperCase();
            const userAvatar = document.querySelector('.user-avatar');
            if (userAvatar) {
                userAvatar.textContent = userInitials;
            }
        }
    } else {
        // Fallback to direct check (for compatibility)
        const TOKEN_KEY = 'oxicloud_token';
        const TOKEN_EXPIRY_KEY = 'oxicloud_token_expiry';
        const USER_DATA_KEY = 'oxicloud_user';
        
        const token = localStorage.getItem(TOKEN_KEY);
        const tokenExpiry = localStorage.getItem(TOKEN_EXPIRY_KEY);
        
        if (!token || !tokenExpiry || new Date(tokenExpiry) < new Date()) {
            // No token or expired token - use absolute URL with location.replace
            const baseUrl = window.location.origin;
            window.location.replace(baseUrl + '/login');
            return;
        }
        
        // Display user information if available
        const userData = JSON.parse(localStorage.getItem(USER_DATA_KEY) || '{}');
        if (userData.username) {
            // Update user avatar with initials
            const userInitials = userData.username.substring(0, 2).toUpperCase();
            const userAvatar = document.querySelector('.user-avatar');
            if (userAvatar) {
                userAvatar.textContent = userInitials;
            }
        }
    }
}

/**
 * Logout - clear all auth data and redirect to login
 */
function logout() {
    // Use AuthModule if available
    if (window.AuthModule && typeof AuthModule.logout === 'function') {
        AuthModule.logout();
        return;
    }
    
    // Fallback for compatibility
    const TOKEN_KEY = 'oxicloud_token';
    const REFRESH_TOKEN_KEY = 'oxicloud_refresh_token';
    const TOKEN_EXPIRY_KEY = 'oxicloud_token_expiry';
    const USER_DATA_KEY = 'oxicloud_user';
    
    // Clear all authentication data
    localStorage.removeItem(TOKEN_KEY);
    localStorage.removeItem(REFRESH_TOKEN_KEY);
    localStorage.removeItem(TOKEN_EXPIRY_KEY);
    localStorage.removeItem(USER_DATA_KEY);
    
    // Redirect to login page with absolute URL using location.replace
    const baseUrl = window.location.origin;
    window.location.replace(baseUrl + '/login');
}

// Initialize app when DOM is ready
document.addEventListener('DOMContentLoaded', initApp);

// Implementar la funcionalidad para la sección "Shared"
document.addEventListener('DOMContentLoaded', function() {
    // Buscar el elemento de navegación "Shared"
    const sharedNavItem = document.querySelector('.nav-item .fa-share-alt').parentElement;
    
    if (sharedNavItem) {
        sharedNavItem.addEventListener('click', function() {
            // Desactivar la clase activa de todos los elementos de navegación
            document.querySelectorAll('.nav-item').forEach(item => {
                item.classList.remove('active');
            });
            
            // Activar este elemento
            this.classList.add('active');
            
            // Mostrar indicador de carga
            const filesGrid = document.getElementById('files-grid');
            if (filesGrid) {
                filesGrid.innerHTML = '<div class="loading"><div class="spinner"></div></div>';
            }
            
            // Cargar los archivos compartidos
            loadSharedFiles();
        });
    }
});

/**
 * Carga los archivos compartidos con el usuario actual
 */
async function loadSharedFiles() {
    try {
        const token = window.AuthModule && typeof AuthModule.getToken === 'function' ? 
            AuthModule.getToken() : localStorage.getItem('oxicloud_token');
            
        // Realizar la petición a la API para obtener archivos compartidos
        const response = await fetch('/api/sharing/shared-with-me', {
            headers: {
                'Authorization': `Bearer ${token}`
            }
        });
        
        if (!response.ok) {
            throw new Error(`Server responded with status: ${response.status}`);
        }
        
        const files = await response.json();
        
        // Limpiar las vistas actuales
        const filesGrid = document.getElementById('files-grid');
        const filesListView = document.getElementById('files-list-view');
        
        if (filesGrid) {
            filesGrid.innerHTML = '';
        }
        
        if (filesListView) {
            filesListView.innerHTML = `
                <div class="list-header">
                    <div data-i18n="files.name">Nombre</div>
                    <div data-i18n="files.owner">Propietario</div>
                    <div data-i18n="files.permission">Permiso</div>
                    <div data-i18n="files.size">Tamaño</div>
                </div>
            `;
        }
        
        // Traducir si i18n está disponible
        if (window.i18n && window.i18n.translatePage) {
            window.i18n.translatePage();
        }
        
        // Verificar si tenemos archivos compartidos
        if (Array.isArray(files) && files.length > 0) {
            // Actualizar la navegación de migas de pan (breadcrumb)
            ui.updateBreadcrumb(window.i18n ? window.i18n.t('nav.shared') : 'Compartidos');
            
            // Mostrar cada archivo compartido
            files.forEach(file => {
                addSharedFileToView(file);
            });
            
            // Actualizar iconos de archivos
            ui.updateFileIcons();
        } else {
            // Mostrar mensaje cuando no hay archivos compartidos
            if (filesGrid) {
                filesGrid.innerHTML = `
                    <div class="empty-state">
                        <i class="fas fa-share-alt fa-3x"></i>
                        <p data-i18n="messages.no_shared_files">No tienes archivos compartidos contigo</p>
                    </div>
                `;
            }
            
            // Traducir si i18n está disponible
            if (window.i18n && window.i18n.translatePage) {
                window.i18n.translatePage();
            }
        }
    } catch (error) {
        console.error('Error loading shared files:', error);
        ui.showNotification('Error', 'No se pudieron cargar los archivos compartidos');
        
        // Mostrar mensaje de error en la interfaz
        const filesGrid = document.getElementById('files-grid');
        if (filesGrid) {
            filesGrid.innerHTML = `
                <div class="empty-state">
                    <i class="fas fa-exclamation-circle fa-3x text-danger"></i>
                    <p data-i18n="errors.load_shared">Error al cargar archivos compartidos</p>
                </div>
            `;
        }
    }
}

/**
 * Añade un archivo compartido a la vista
 * @param {Object} file - Objeto de archivo compartido
 */
function addSharedFileToView(file) {
    const fileName = file.name || 'Unknown';
    const fileSize = formatFileSize(file.size || 0);
    const owner = file.owner_name || file.owner_id || 'Unknown';
    const permission = getPermissionLabel(file.permission || 'READ');
    
    // Vista de cuadrícula
    const filesGrid = document.getElementById('files-grid');
    if (filesGrid) {
        const fileElement = document.createElement('div');
        fileElement.className = 'file-item shared-file';
        fileElement.setAttribute('data-file-id', file.id);
        fileElement.innerHTML = `
            <div class="file-icon">
                <i class="fas fa-file"></i>
            </div>
            <div class="file-name">${fileName}</div>
            <div class="file-info">
                <span class="file-size">${fileSize}</span>
                <span class="file-permission ${file.permission.toLowerCase()}">${permission}</span>
            </div>
            <div class="file-owner">${owner}</div>
        `;
        
        // Añadir evento de clic para descargar/abrir
        fileElement.addEventListener('click', (e) => {
            if (!e.target.closest('.context-menu-trigger')) {
                window.fileOps.viewFile(file.id, fileName);
            }
        });
        
        // Añadir evento de clic derecho para menú contextual
        fileElement.addEventListener('contextmenu', (e) => {
            e.preventDefault();
            window.app.contextMenuTargetFile = file;
            window.ui.showFileContextMenu(e.clientX, e.clientY);
        });
        
        filesGrid.appendChild(fileElement);
    }
    
    // Vista de lista
    const filesListView = document.getElementById('files-list-view');
    if (filesListView) {
        const fileRow = document.createElement('div');
        fileRow.className = 'file-row shared-file';
        fileRow.setAttribute('data-file-id', file.id);
        fileRow.innerHTML = `
            <div class="file-cell">
                <i class="fas fa-file file-icon-small"></i>
                <span>${fileName}</span>
            </div>
            <div class="file-cell">${owner}</div>
            <div class="file-cell"><span class="file-permission ${file.permission.toLowerCase()}">${permission}</span></div>
            <div class="file-cell">${fileSize}</div>
        `;
        
        // Añadir evento de clic para descargar/abrir
        fileRow.addEventListener('click', (e) => {
            if (!e.target.closest('.context-menu-trigger')) {
                window.fileOps.viewFile(file.id, fileName);
            }
        });
        
        // Añadir evento de clic derecho para menú contextual
        fileRow.addEventListener('contextmenu', (e) => {
            e.preventDefault();
            window.app.contextMenuTargetFile = file;
            window.ui.showFileContextMenu(e.clientX, e.clientY);
        });
        
        filesListView.appendChild(fileRow);
    }
}

/**
 * Obtiene la etiqueta de permiso en formato legible
 * @param {string} permission - Nivel de permiso (READ, WRITE, ADMIN)
 * @returns {string} Etiqueta de permiso traducida
 */
function getPermissionLabel(permission) {
    if (window.i18n) {
        switch (permission) {
            case 'READ': return window.i18n.t('permissions.read');
            case 'WRITE': return window.i18n.t('permissions.write');
            case 'ADMIN': return window.i18n.t('permissions.admin');
            default: return window.i18n.t('permissions.unknown');
        }
    } else {
        switch (permission) {
            case 'READ': return 'Lectura';
            case 'WRITE': return 'Escritura';
            case 'ADMIN': return 'Admin';
            default: return 'Desconocido';
        }
    }
}
