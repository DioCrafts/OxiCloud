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
    isTrashView: false,    // Whether we're in trash view
    isSharedView: false,   // Whether we're in shared view
    isFavoritesView: false, // Whether we're in favorites view
    isRecentView: false,    // Whether we're in recent files view
    currentSection: 'files', // Current section: 'files', 'trash', 'shared', 'favorites' or 'recent'
    isSearchMode: false,    // Whether we're in search mode
    // File sharing related properties
    shareDialogItem: null,          // Item being shared in share dialog
    shareDialogItemType: null,      // Type of item being shared ('file' or 'folder')
    notificationShareUrl: null      // URL for notification dialog
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
    
    // Initialize file sharing module first
    if (window.fileSharing && window.fileSharing.init) {
        window.fileSharing.init();
    } else {
        console.warn('fileSharing module not fully initialized');
    }
    
    // Then create menus and dialogs after modules have initialized
    setTimeout(() => {
        ui.initializeContextMenus();
    }, 100);
    
    // Setup event listeners
    setupEventListeners();
    
    // Initialize file renderer if available
    if (window.fileRenderer) {
        console.log('Using optimized file renderer');
    } else {
        console.log('Using standard file rendering');
    }
    
    // Check if inline viewer is initialized
    if (window.inlineViewer) {
        console.log('Inline viewer is available');
    } else {
        console.warn('Inline viewer not initialized yet, will initialize it now');
        try {
            // Create inline viewer if not already created and if the class exists
            if (typeof InlineViewer !== 'undefined') {
                window.inlineViewer = new InlineViewer();
            } else {
                console.warn('InlineViewer class is not defined, skipping initialization');
            }
        } catch (e) {
            console.error('Error initializing inline viewer:', e);
        }
    }
    
    // Initialize favorites module if available
    if (window.favorites && window.favorites.init) {
        console.log('Initializing favorites module');
        window.favorites.init();
    } else {
        console.warn('Favorites module not available or not initializable');
    }
    
    // Initialize recent files module if available
    if (window.recent && window.recent.init) {
        console.log('Initializing recent files module');
        window.recent.init();
    } else {
        console.warn('Recent files module not available or not initializable');
    }
    
    // Wait for translations to load before checking authentication
    if (window.i18n && window.i18n.isLoaded && window.i18n.isLoaded()) {
        // Translations already loaded, proceed with authentication
        checkAuthentication();
    } else {
        // Wait for translations to be loaded before proceeding
        console.log('Waiting for translations to load...');
        window.addEventListener('translationsLoaded', () => {
            console.log('Translations loaded, proceeding with authentication');
            checkAuthentication();
        });
        
        // Set a timeout as a fallback in case translations take too long
        setTimeout(() => {
            if (!window.i18n || !window.i18n.isLoaded || !window.i18n.isLoaded()) {
                console.warn('Translations loading timeout, proceeding with authentication anyway');
                checkAuthentication();
            }
        }, 3000); // 3 second timeout
    }
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
    elements.pageTitle = document.querySelector('.page-title');
    elements.actionsBar = document.querySelector('.actions-bar');
    elements.navItems = document.querySelectorAll('.nav-item');
    elements.trashBtn = document.querySelector('.nav-item:nth-child(5)'); // The trash nav item
    elements.searchInput = document.querySelector('.search-container input');
}

/**
 * Setup the user menu (avatar dropdown with profile, storage, theme, about, logout)
 */
function setupUserMenu() {
    const wrapper = document.getElementById('user-menu-wrapper');
    const avatarBtn = document.getElementById('user-avatar-btn');
    const menu = document.getElementById('user-menu');
    const logoutBtn = document.getElementById('user-menu-logout');
    const themeBtn = document.getElementById('user-menu-theme');
    const aboutBtn = document.getElementById('user-menu-about');
    const adminBtn = document.getElementById('user-menu-admin');
    const adminDivider = document.getElementById('user-menu-admin-divider');
    const profileBtn = document.getElementById('user-menu-profile');
    const roleBadge = document.getElementById('user-menu-role-badge');
    
    if (!wrapper || !avatarBtn || !menu) return;
    
    // Toggle menu
    avatarBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        const isOpen = wrapper.classList.contains('open');
        wrapper.classList.toggle('open');
        if (!isOpen) {
            updateUserMenuData();
            // Show/hide admin panel button based on user role
            const USER_DATA_KEY = 'oxicloud_user';
            const userData = JSON.parse(localStorage.getItem(USER_DATA_KEY) || '{}');
            const isAdmin = userData.role === 'admin';
            if (adminBtn) adminBtn.style.display = isAdmin ? 'flex' : 'none';
            if (adminDivider) adminDivider.style.display = isAdmin ? 'block' : 'none';
            if (roleBadge) roleBadge.style.display = isAdmin ? 'block' : 'none';
        }
    });
    
    // Close menu on outside click
    document.addEventListener('click', (e) => {
        if (wrapper.classList.contains('open') && !wrapper.contains(e.target)) {
            wrapper.classList.remove('open');
        }
    });
    
    // Logout
    if (logoutBtn) {
        logoutBtn.addEventListener('click', () => {
            wrapper.classList.remove('open');
            logout();
        });
    }
    
    // Theme toggle (dark mode placeholder — toggles pill visually)
    if (themeBtn) {
        const pill = document.getElementById('theme-toggle-pill');
        const isDark = localStorage.getItem('oxicloud_theme') === 'dark';
        if (isDark && pill) pill.classList.add('active');
        
        themeBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            if (pill) {
                pill.classList.toggle('active');
                const dark = pill.classList.contains('active');
                localStorage.setItem('oxicloud_theme', dark ? 'dark' : 'light');
                // Theme switching could be expanded here in the future
                window.ui.showNotification(
                    dark ? '🌙' : '☀️',
                    dark ? 'Modo oscuro activado (próximamente)' : 'Modo claro activado'
                );
            }
        });
    }
    
    // Admin panel link
    if (adminBtn) {
        adminBtn.addEventListener('click', () => {
            wrapper.classList.remove('open');
            window.location.href = '/admin.html';
        });
    }
    
    // Profile button — shows user info modal
    if (profileBtn) {
        profileBtn.addEventListener('click', () => {
            wrapper.classList.remove('open');
            showUserProfileModal();
        });
    }
    
    // About modal
    if (aboutBtn) {
        aboutBtn.addEventListener('click', () => {
            wrapper.classList.remove('open');
            const overlay = document.getElementById('about-modal-overlay');
            if (overlay) overlay.classList.add('show');
        });
    }
    
    // About modal close
    const aboutCloseBtn = document.getElementById('about-close-btn');
    const aboutOverlay = document.getElementById('about-modal-overlay');
    if (aboutCloseBtn) {
        aboutCloseBtn.addEventListener('click', () => {
            aboutOverlay.classList.remove('show');
        });
    }
    if (aboutOverlay) {
        aboutOverlay.addEventListener('click', (e) => {
            if (e.target === aboutOverlay) {
                aboutOverlay.classList.remove('show');
            }
        });
    }
    
    // Fetch version from backend (centralized in Cargo.toml)
    fetchAppVersion();
}

/**
 * Update user menu data (name, email, storage) from localStorage
 */
function updateUserMenuData() {
    const USER_DATA_KEY = 'oxicloud_user';
    const userData = JSON.parse(localStorage.getItem(USER_DATA_KEY) || '{}');
    
    const nameEl = document.getElementById('user-menu-name');
    const emailEl = document.getElementById('user-menu-email');
    const avatarEl = document.getElementById('user-menu-avatar');
    const storageFill = document.getElementById('user-menu-storage-fill');
    const storageText = document.getElementById('user-menu-storage-text');
    
    if (userData.username) {
        if (nameEl) nameEl.textContent = userData.username;
        if (emailEl) emailEl.textContent = userData.email || '';
        if (avatarEl) avatarEl.textContent = userData.username.substring(0, 2).toUpperCase();
    }
    
    // Storage info
    const usedBytes = userData.storage_used_bytes || 0;
    const quotaBytes = userData.storage_quota_bytes || 10737418240;
    const percentage = quotaBytes > 0 ? Math.min(Math.round((usedBytes / quotaBytes) * 100), 100) : 0;
    
    if (storageFill) storageFill.style.width = percentage + '%';
    if (storageText) {
        const used = formatFileSize(usedBytes);
        const total = formatFileSize(quotaBytes);
        storageText.textContent = `${percentage}% · ${used} / ${total}`;
    }
}

/**
 * Fetch app version from backend (centralized in Cargo.toml)
 * Updates the about modal version display
 */
async function fetchAppVersion() {
    try {
        const response = await fetch('/api/version');
        if (response.ok) {
            const data = await response.json();
            const versionEl = document.getElementById('about-version');
            if (versionEl && data.version) {
                versionEl.textContent = `v${data.version}`;
            }
        }
    } catch (err) {
        console.warn('Could not fetch app version:', err);
        // Fallback: leave placeholder
    }
}

/**
 * Setup the upload dropdown button and menu
 * Handles opening/closing the dropdown and triggering file/folder inputs
 */
function setupUploadDropdown() {
    const dropdown = document.getElementById('upload-dropdown');
    const uploadBtn = document.getElementById('upload-btn');
    const menu = document.getElementById('upload-dropdown-menu');
    const uploadFilesBtn = document.getElementById('upload-files-btn');
    const uploadFolderBtn = document.getElementById('upload-folder-btn');
    
    if (!uploadBtn || !menu) return;
    
    // Toggle dropdown on button click
    uploadBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        const isOpen = menu.classList.contains('show');
        // Close any other open dropdowns
        document.querySelectorAll('.upload-dropdown-menu.show').forEach(m => m.classList.remove('show'));
        if (!isOpen) {
            menu.classList.add('show');
        }
    });
    
    // Upload files option
    if (uploadFilesBtn) {
        uploadFilesBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            menu.classList.remove('show');
            elements.fileInput.click();
        });
    }
    
    // Upload folder option
    if (uploadFolderBtn) {
        uploadFolderBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            menu.classList.remove('show');
            const folderInput = document.getElementById('folder-input');
            if (folderInput) {
                folderInput.click();
            }
        });
    }
    
    // Close dropdown when clicking outside
    document.addEventListener('click', () => {
        document.querySelectorAll('.upload-dropdown-menu.show').forEach(m => m.classList.remove('show'));
    });
}

/**
 * Setup event listeners for main UI elements
 */
function setupEventListeners() {
    // Set up drag and drop
    ui.setupDragAndDrop();
    
    // Search input
    elements.searchInput.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') {
            const query = elements.searchInput.value.trim();
            if (query) {
                performSearch(query);
            } else if (app.isSearchMode) {
                // If search is empty and we're in search mode, return to normal view
                app.isSearchMode = false;
                app.currentPath = '';
                ui.updateBreadcrumb('');
                loadFiles();
            }
        }
    });
    
    // Search button
    document.getElementById('search-button').addEventListener('click', () => {
        const query = elements.searchInput.value.trim();
        if (query) {
            performSearch(query);
        }
    });
    
    // Upload dropdown
    setupUploadDropdown();
    
    // File input
    elements.fileInput.addEventListener('change', (e) => {
        if (e.target.files.length > 0) {
            fileOps.uploadFiles(e.target.files);
            e.target.value = ''; // reset so same file can be re-uploaded
        }
    });
    
    // Folder input
    const folderInput = document.getElementById('folder-input');
    if (folderInput) {
        folderInput.addEventListener('change', (e) => {
            if (e.target.files.length > 0) {
                fileOps.uploadFolderFiles(e.target.files);
                e.target.value = '';
            }
        });
    }
    
    // New folder button
    elements.newFolderBtn.addEventListener('click', async () => {
        const folderName = await window.Modal.promptNewFolder();
        if (folderName) {
            fileOps.createFolder(folderName);
        }
    });
    
    // View toggle
    elements.gridViewBtn.addEventListener('click', ui.switchToGridView);
    elements.listViewBtn.addEventListener('click', ui.switchToListView);
    
    // Sidebar navigation
    elements.navItems.forEach(item => {
        item.addEventListener('click', () => {
            // Remove active class from all nav items
            elements.navItems.forEach(navItem => navItem.classList.remove('active'));
            
            // Add active class to clicked item
            item.classList.add('active');
            
            // Check if this is the shared item
            if (item.querySelector('span').getAttribute('data-i18n') === 'nav.shared') {
                // Switch to shared view
                switchToSharedView();
                return;
            }
            
            // Check if this is the favorites item
            if (item.querySelector('span').getAttribute('data-i18n') === 'nav.favorites') {
                // Switch to favorites view
                switchToFavoritesView();
                return;
            }
            
            // Check if this is the recent files item
            if (item.querySelector('span').getAttribute('data-i18n') === 'nav.recent') {
                // Switch to recent files view
                switchToRecentFilesView();
                return;
            }
            
            // Check if this is the trash item
            if (item === elements.trashBtn) {
                // Hide shared view if active
                if (app.isSharedView) {
                    // Hide shared view
                    if (window.sharedView) {
                        window.sharedView.hide();
                    }
                    
                    // Reset shared view flag
                    app.isSharedView = false;
                    
                    // Clean up shared containers if they exist
                    const sharedContainer = document.getElementById('shared-container');
                    if (sharedContainer) {
                        sharedContainer.style.display = 'none';
                    }
                }
                
                // Show trash view
                app.isTrashView = true;
                app.currentSection = 'trash';
                
                // Show files containers (to be filled with trash)
                const filesGrid = document.getElementById('files-grid');
                const filesListView = document.getElementById('files-list-view');
                if (filesGrid) filesGrid.style.display = app.currentView === 'grid' ? 'grid' : 'none';
                if (filesListView) filesListView.style.display = app.currentView === 'list' ? 'block' : 'none';
                
                // Update UI
                elements.pageTitle.textContent = window.i18n ? window.i18n.t('nav.trash') : 'Papelera';
                elements.actionsBar.innerHTML = `
                    <div class="action-buttons">
                        <button class="btn btn-danger" id="empty-trash-btn">
                            <i class="fas fa-trash-alt"></i>
                            <span>${window.i18n ? window.i18n.t('trash.empty_trash') : 'Vaciar papelera'}</span>
                        </button>
                    </div>
                `;
                elements.actionsBar.style.display = 'flex';
                
                // Add event listener to empty trash button
                document.getElementById('empty-trash-btn').addEventListener('click', async () => {
                    if (await fileOps.emptyTrash()) {
                        loadTrashItems();
                    }
                });
                
                // Load trash items
                loadTrashItems();
            } else {
                // Check if we need to reset shared view
                if (app.isSharedView) {
                    // Hide shared view
                    if (window.sharedView) {
                        window.sharedView.hide();
                    }
                    
                    // Reset shared view flag
                    app.isSharedView = false;
                    
                    // Clean up shared containers if they exist
                    const sharedContainer = document.getElementById('shared-container');
                    if (sharedContainer) {
                        sharedContainer.style.display = 'none';
                    }
                }
                
                // Show regular files view
                app.isTrashView = false;
                app.currentSection = 'files';
                
                // Reset UI
                elements.pageTitle.textContent = window.i18n ? window.i18n.t('nav.files') : 'Archivos';
                elements.actionsBar.innerHTML = `
                    <div class="action-buttons">
                        <div class="upload-dropdown" id="upload-dropdown">
                            <button class="btn btn-primary" id="upload-btn">
                                <i class="fas fa-cloud-upload-alt" style="margin-right: 5px;"></i>
                                <span data-i18n="actions.upload">Subir</span>
                                <i class="fas fa-caret-down" style="margin-left: 4px; font-size: 12px;"></i>
                            </button>
                            <div class="upload-dropdown-menu" id="upload-dropdown-menu">
                                <button class="upload-dropdown-item" id="upload-files-btn">
                                    <i class="fas fa-file"></i>
                                    <span data-i18n="actions.upload_files">Subir archivos</span>
                                </button>
                                <button class="upload-dropdown-item" id="upload-folder-btn">
                                    <i class="fas fa-folder-open"></i>
                                    <span data-i18n="actions.upload_folder">Subir carpeta</span>
                                </button>
                            </div>
                        </div>
                        <button class="btn btn-secondary" id="new-folder-btn">
                            <i class="fas fa-folder-plus" style="margin-right: 5px;"></i> <span data-i18n="actions.new_folder">Nueva carpeta</span>
                        </button>
                    </div>
                    <div class="view-toggle">
                        <button class="toggle-btn active" id="grid-view-btn" title="Vista de cuadrícula">
                            <i class="fas fa-th"></i>
                        </button>
                        <button class="toggle-btn" id="list-view-btn" title="Vista de lista">
                            <i class="fas fa-list"></i>
                        </button>
                    </div>
                `;
                elements.actionsBar.style.display = 'flex';
                
                // Show files containers
                const filesGrid = document.getElementById('files-grid');
                const filesListView = document.getElementById('files-list-view');
                if (filesGrid) filesGrid.style.display = app.currentView === 'grid' ? 'grid' : 'none';
                if (filesListView) filesListView.style.display = app.currentView === 'list' ? 'block' : 'none';
                
                // Restore event listeners
                setupUploadDropdown();
                
                document.getElementById('new-folder-btn').addEventListener('click', async () => {
                    const folderName = await window.Modal.promptNewFolder();
                    if (folderName) {
                        fileOps.createFolder(folderName);
                    }
                });
                
                document.getElementById('grid-view-btn').addEventListener('click', ui.switchToGridView);
                document.getElementById('list-view-btn').addEventListener('click', ui.switchToListView);
                
                // Restore cached elements
                elements.uploadBtn = document.getElementById('upload-btn');
                elements.newFolderBtn = document.getElementById('new-folder-btn');
                elements.gridViewBtn = document.getElementById('grid-view-btn');
                elements.listViewBtn = document.getElementById('list-view-btn');
                
                // Load regular files
                app.currentPath = '';
                ui.updateBreadcrumb('');
                loadFiles();
            }
        });
    });
    
    // Load saved view preference
    const savedView = localStorage.getItem('oxicloud-view');
    if (savedView === 'list') {
        ui.switchToListView();
    }
    
    // User menu
    setupUserMenu();
    
    // Global events to close context menus and deselect cards
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

        // Deselect all cards when clicking empty area (not on a card, menu, or modal)
        if (!e.target.closest('.file-card') && !e.target.closest('.context-menu') && !e.target.closest('.about-modal')) {
            document.querySelectorAll('.file-card.selected').forEach(c => c.classList.remove('selected'));
        }
    });
}

/**
 * Load files and folders for the current path
 */
async function loadFiles(options = {}) {
    try {
        console.log("Iniciando loadFiles() - cargando archivos...", options);
        
        // Flag para forzar el refresco completo ignorando caché
        const forceRefresh = options.forceRefresh || false;
        
        // Prevenir múltiples solicitudes de carga simultáneas
        if (window.isLoadingFiles) {
            console.log("Ya hay una carga de archivos en progreso, ignorando solicitud");
            return;
        }
        
        window.isLoadingFiles = true;
        
        // Show loading spinner
        elements.filesGrid.innerHTML = `
            <div class="files-loading-spinner">
                <div class="spinner"></div>
                <span>${window.i18n ? window.i18n.t('files.loading') : 'Cargando archivos…'}</span>
            </div>
        `;
        
        // Always ensure a userHomeFolderId is set
        if (!app.userHomeFolderId) {
            // If we don't have a home folder ID yet, try to get the user's username
            const USER_DATA_KEY = 'oxicloud_user';
            const userData = JSON.parse(localStorage.getItem(USER_DATA_KEY) || '{}');
            if (userData.username) {
                // Find user's home folder
                console.log("Buscando carpeta de usuario para", userData.username);
                await findUserHomeFolder(userData.username);
            }
        }
        
        // Agregar timestamp para evitar caché
        const timestamp = new Date().getTime();
        let url;
        
        // ALWAYS use the userHomeFolderId (current folder or home folder) to avoid showing root
        if (!app.currentPath || app.currentPath === '') {
            // If at root, force user to their home folder
            if (app.userHomeFolderId) {
                url = `/api/folders/${app.userHomeFolderId}/contents?t=${timestamp}`;
                app.currentPath = app.userHomeFolderId;
                ui.updateBreadcrumb(app.userHomeFolderName || 'Home');
                console.log(`Cargando carpeta del usuario: ${app.userHomeFolderName} (${app.userHomeFolderId})`);
            } else {
                // Emergency fallback - this should rarely happen but prevents errors
                url = `/api/folders?t=${timestamp}`;
                console.warn("Emergency fallback to root folder - this should not normally happen");
            }
        } else {
            // Normal case - viewing subfolder contents
            url = `/api/folders/${app.currentPath}/contents?t=${timestamp}`;
            console.log(`Cargando contenido de subcarpeta: ${app.currentPath}`);
        }
        
        const token = localStorage.getItem('oxicloud_token');
        const requestOptions = {
            headers: {
                'Authorization': `Bearer ${token}`,
                'Cache-Control': 'no-cache, no-store, must-revalidate',
                'Pragma': 'no-cache'
            },
            cache: 'no-store'  // Instruir al navegador a no usar caché
        };
        
        // Si se especifica forceRefresh, agregar un parámetro adicional para evitar caché
        if (forceRefresh) {
            url += `&force_refresh=true`;
            requestOptions.headers['X-Force-Refresh'] = 'true';
            console.log('Forzando refresco completo ignorando caché');
        }
        
        console.log(`Loading files from ${url}`);
        const response = await fetch(url, requestOptions);
        
        // Critical error handling
        if (response.status === 401 || response.status === 403) {
            console.warn("Auth error when loading files, showing empty list");
            // Just show empty state instead of causing redirect loops
            elements.filesGrid.innerHTML = '<div class="empty-state"><p>No se pudieron cargar los archivos</p></div>';
            elements.filesListView.innerHTML = `
                <div class="list-header">
                    <div>Nombre</div>
                    <div>Tipo</div>
                    <div>Tamaño</div>
                    <div>Modificado</div>
                </div>
            `;
            return;
        }
        
        if (!response.ok) {
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
        
        // Get user info for filtering
        const USER_DATA_KEY = 'oxicloud_user';
        const userData = JSON.parse(localStorage.getItem(USER_DATA_KEY) || '{}');
        const username = userData.username || '';
        
        // Filter folders before adding them to the view
        const visibleFolders = folderList.filter(folder => {
            // Skip system folders (starting with dot) when at root
            if (!app.currentPath && folder.name.startsWith('.')) {
                return false;
            }
            
            // Skip other users' folders when at root
            if (!app.currentPath && folder.name.startsWith('Mi Carpeta - ') && !folder.name.includes(username)) {
                return false;
            }
            
            return true;
        });
        
        // Add filtered folders to the view
        visibleFolders.forEach(folder => {
            ui.addFolderToView(folder);
        });
        
        // Also load files in this folder
        const cacheTimestamp = new Date().getTime();
        let filesUrl = `/api/files?t=${cacheTimestamp}`; // Agregar timestamp para evitar problemas de caché
        if (app.currentPath) {
            filesUrl += `&folder_id=${app.currentPath}`;
        }
        console.log(`Cargando archivos desde: ${filesUrl}`);
        
        try {
            console.log(`Fetching files from: ${filesUrl}`);
            const filesResponse = await fetch(filesUrl, requestOptions); // Use same auth token
            console.log(`Files response status: ${filesResponse.status}`);
            
            // Handle auth errors for files too
            if (filesResponse.status === 401 || filesResponse.status === 403) {
                console.warn("Auth error when loading files");
                return; // Already showing folders, just stop here
            }
            
            if (filesResponse.ok) {
                const files = await filesResponse.json();
                console.log(`Files received:`, files);
                
                // Add files (check if it's an array)
                const fileList = Array.isArray(files) ? files : [];
                console.log(`Processing ${fileList.length} files`);
                
                fileList.forEach(file => {
                    console.log(`Adding file to view: ${file.name} (${file.id})`);
                    ui.addFileToView(file);
                });
            } else {
                const errorText = await filesResponse.text();
                console.error(`Error loading files: ${filesResponse.status} - ${errorText}`);
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
    } finally {
        // Marcar que ya no estamos cargando archivos para permitir solicitudes futuras
        window.isLoadingFiles = false;
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

/**
 * Load trash items 
 */
async function loadTrashItems() {
    try {
        // Clear existing content
        elements.filesGrid.innerHTML = '';
        elements.filesListView.innerHTML = `
            <div class="list-header">
                <div data-i18n="files.name">Nombre</div>
                <div data-i18n="files.type">Tipo</div>
                <div data-i18n="trash.original_location">Ubicación original</div>
                <div data-i18n="trash.deleted_date">Fecha eliminación</div>
                <div data-i18n="trash.actions">Acciones</div>
            </div>
        `;
        
        // Translate the header if i18n is available
        if (window.i18n && window.i18n.translatePage) {
            window.i18n.translatePage();
        }
        
        // Update breadcrumb - just show Home
        ui.updateBreadcrumb('');
        
        // Get trash items
        const trashItems = await fileOps.getTrashItems();
        
        if (trashItems.length === 0) {
            // Show empty state
            const emptyState = document.createElement('div');
            emptyState.className = 'empty-state';
            emptyState.innerHTML = `
                <i class="fas fa-trash" style="font-size: 48px; color: #ddd; margin-bottom: 16px;"></i>
                <p>${window.i18n ? window.i18n.t('trash.empty_state') : 'La papelera está vacía'}</p>
            `;
            elements.filesGrid.appendChild(emptyState);
            return;
        }
        
        // Process each trash item
        trashItems.forEach(item => {
            addTrashItemToView(item);
        });
        
    } catch (error) {
        console.error('Error loading trash items:', error);
        window.ui.showNotification('Error', 'Error al cargar elementos de la papelera');
    }
}

/**
 * Add a trash item to the view
 * @param {Object} item - Trash item object
 */
function addTrashItemToView(item) {
    const isFile = item.item_type === 'file';
    const iconClass = isFile ? 'fas fa-file' : 'fas fa-folder';
    
    // Format date
    const deletedDate = new Date(item.deleted_at * 1000);
    const formattedDate = deletedDate.toLocaleDateString() + ' ' +
                         deletedDate.toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'});
                         
    // Item type label
    const typeLabel = isFile ? 
        (window.i18n ? window.i18n.t('files.file_types.file') : 'Archivo') :
        (window.i18n ? window.i18n.t('files.file_types.folder') : 'Carpeta');
    
    // Grid view element
    const gridElement = document.createElement('div');
    gridElement.className = 'file-card trash-item';
    gridElement.dataset.trashId = item.id;
    gridElement.dataset.originalId = item.original_id;
    gridElement.dataset.itemType = item.item_type;
    gridElement.innerHTML = `
        <div class="file-icon">
            <i class="${iconClass}"></i>
        </div>
        <div class="file-name">${item.name}</div>
        <div class="file-info">${typeLabel} - ${formattedDate}</div>
        <div class="trash-actions">
            <button class="btn-restore" title="${window.i18n ? window.i18n.t('trash.restore') : 'Restaurar'}">
                <i class="fas fa-undo"></i>
            </button>
            <button class="btn-delete" title="${window.i18n ? window.i18n.t('trash.delete_permanently') : 'Eliminar permanentemente'}">
                <i class="fas fa-trash"></i>
            </button>
        </div>
    `;
    
    // Add action buttons event listeners
    gridElement.querySelector('.btn-restore').addEventListener('click', async (e) => {
        e.stopPropagation();
        if (await fileOps.restoreFromTrash(item.id)) {
            loadTrashItems();
        }
    });
    
    gridElement.querySelector('.btn-delete').addEventListener('click', async (e) => {
        e.stopPropagation();
        if (await fileOps.deletePermanently(item.id)) {
            loadTrashItems();
        }
    });
    
    elements.filesGrid.appendChild(gridElement);
    
    // List view element
    const listElement = document.createElement('div');
    listElement.className = 'file-item trash-item';
    listElement.dataset.trashId = item.id;
    listElement.dataset.originalId = item.original_id;
    listElement.dataset.itemType = item.item_type;
    
    listElement.innerHTML = `
        <div class="name-cell">
            <div class="file-icon">
                <i class="${iconClass}"></i>
            </div>
            <span>${item.name}</span>
        </div>
        <div class="type-cell">${typeLabel}</div>
        <div class="path-cell">${item.original_path || '--'}</div>
        <div class="date-cell">${formattedDate}</div>
        <div class="actions-cell">
            <button class="btn-restore" title="${window.i18n ? window.i18n.t('trash.restore') : 'Restaurar'}">
                <i class="fas fa-undo"></i>
            </button>
            <button class="btn-delete" title="${window.i18n ? window.i18n.t('trash.delete_permanently') : 'Eliminar permanentemente'}">
                <i class="fas fa-trash"></i>
            </button>
        </div>
    `;
    
    // Add action buttons event listeners for list view
    listElement.querySelector('.btn-restore').addEventListener('click', async (e) => {
        e.stopPropagation();
        if (await fileOps.restoreFromTrash(item.id)) {
            loadTrashItems();
        }
    });
    
    listElement.querySelector('.btn-delete').addEventListener('click', async (e) => {
        e.stopPropagation();
        if (await fileOps.deletePermanently(item.id)) {
            loadTrashItems();
        }
    });
    
    elements.filesListView.appendChild(listElement);
}

/**
 * Perform search with the given query
 * @param {string} query - Search query
 */
async function performSearch(query) {
    console.log(`Performing search for: "${query}"`);
    
    try {
        // Update UI to indicate search mode
        app.isSearchMode = true;
        
        // Set breadcrumb for search
        ui.updateBreadcrumb(`Búsqueda: "${query}"`);
        
        // Prepare search options
        const options = {
            recursive: true, // Search in all subfolders
            limit: 100      // Limit results for performance
        };
        
        // Always restrict search to the user's current folder context
        // This ensures users can't search outside their personal folder
        if (!app.isTrashView) {
            // If we're in a subfolder, search from there, otherwise use the user's home folder
            options.folder_id = app.currentPath;
            
            // Always include folder_id even if it's the root of user's home folder
            // so user cannot search outside their allowed scope
            if (!options.folder_id || options.folder_id === '') {
                // Fall back to user's home folder - we should never be here
                // because findUserHomeFolder should have set app.currentPath
                console.warn("Search without folder_id - this shouldn't happen with proper user context");
                
                // Try to get folder from localStorage if available
                const USER_DATA_KEY = 'oxicloud_user';
                const userData = JSON.parse(localStorage.getItem(USER_DATA_KEY) || '{}');
                if (userData.username) {
                    console.log("Retrieving home folder for user before search");
                    await findUserHomeFolder(userData.username);
                    options.folder_id = app.currentPath;
                }
            }
        }
        
        console.log(`Searching with options:`, options);
        
        // Perform the search
        const searchResults = await window.search.searchFiles(query, options);
        
        // Display search results
        window.search.displaySearchResults(searchResults);
        
    } catch (error) {
        console.error('Search error:', error);
        window.ui.showNotification('Error', 'Error al realizar la búsqueda');
    }
}

// Expose needed functions to global scope
window.app = app;
window.loadFiles = loadFiles;
window.loadTrashItems = loadTrashItems;
window.formatFileSize = formatFileSize;
window.performSearch = performSearch;

// Set up global selectFolder function for navigation
window.selectFolder = (id, name) => {
    app.currentPath = id;
    ui.updateBreadcrumb(name);
    loadFiles();
};

/**
 * Switch to the shared view
 */
function switchToSharedView() {
    // Hide trash view if active
    app.isTrashView = false;
    
    // Set shared view as active
    app.isSharedView = true;
    app.currentSection = 'shared';
    
    // Remove active class from all nav items
    elements.navItems.forEach(navItem => navItem.classList.remove('active'));
    
    // Find shared nav item and make it active
    const sharedNavItem = document.querySelector('.nav-item:nth-child(2)');
    if (sharedNavItem) {
        sharedNavItem.classList.add('active');
    }
    
    // Update UI
    elements.pageTitle.textContent = window.i18n ? window.i18n.t('nav.shared') : 'Compartidos';
    
    // Clear breadcrumb and show root
    ui.updateBreadcrumb('');
    
    // Hide standard actions bar
    if (elements.actionsBar) {
        elements.actionsBar.style.display = 'none';
    }
    
    // Init and show shared view
    if (window.sharedView) {
        window.sharedView.init();
        window.sharedView.show();
    }
}

/**
 * Switch back to the files view
 */
function switchToFilesView() {
    // Reset view flags
    app.isTrashView = false;
    app.isSharedView = false;
    app.isFavoritesView = false;
    app.isRecentView = false;
    app.currentSection = 'files';
    
    // Update UI
    elements.pageTitle.textContent = window.i18n ? window.i18n.t('nav.files') : 'Archivos';
    
    // Remove active class from all nav items
    elements.navItems.forEach(navItem => navItem.classList.remove('active'));
    
    // Make files nav item active
    const filesNavItem = document.querySelector('.nav-item:first-child');
    if (filesNavItem) {
        filesNavItem.classList.add('active');
    }
    
    // Reset UI
    elements.actionsBar.innerHTML = `
        <div class="action-buttons">
            <div class="upload-dropdown" id="upload-dropdown">
                <button class="btn btn-primary" id="upload-btn">
                    <i class="fas fa-cloud-upload-alt" style="margin-right: 5px;"></i>
                    <span data-i18n="actions.upload">Subir</span>
                    <i class="fas fa-caret-down" style="margin-left: 4px; font-size: 12px;"></i>
                </button>
                <div class="upload-dropdown-menu" id="upload-dropdown-menu">
                    <button class="upload-dropdown-item" id="upload-files-btn">
                        <i class="fas fa-file"></i>
                        <span data-i18n="actions.upload_files">Subir archivos</span>
                    </button>
                    <button class="upload-dropdown-item" id="upload-folder-btn">
                        <i class="fas fa-folder-open"></i>
                        <span data-i18n="actions.upload_folder">Subir carpeta</span>
                    </button>
                </div>
            </div>
            <button class="btn btn-secondary" id="new-folder-btn">
                <i class="fas fa-folder-plus" style="margin-right: 5px;"></i> <span data-i18n="actions.new_folder">Nueva carpeta</span>
            </button>
        </div>
        <div class="view-toggle">
            <button class="toggle-btn active" id="grid-view-btn" title="Vista de cuadrícula">
                <i class="fas fa-th"></i>
            </button>
            <button class="toggle-btn" id="list-view-btn" title="Vista de lista">
                <i class="fas fa-list"></i>
            </button>
        </div>
    `;
    elements.actionsBar.style.display = 'flex';
    
    // Restore event listeners
    setupUploadDropdown();
    
    document.getElementById('new-folder-btn').addEventListener('click', async () => {
        const folderName = await window.Modal.promptNewFolder();
        if (folderName) {
            fileOps.createFolder(folderName);
        }
    });
    
    document.getElementById('grid-view-btn').addEventListener('click', ui.switchToGridView);
    document.getElementById('list-view-btn').addEventListener('click', ui.switchToListView);
    
    // Restore cached elements
    elements.uploadBtn = document.getElementById('upload-btn');
    elements.newFolderBtn = document.getElementById('new-folder-btn');
    elements.gridViewBtn = document.getElementById('grid-view-btn');
    elements.listViewBtn = document.getElementById('list-view-btn');
    
    // Hide shared view if it exists
    if (window.sharedView) {
        window.sharedView.hide();
    }
    
    // Show standard files container
    const filesGrid = document.getElementById('files-grid');
    if (filesGrid) {
        filesGrid.style.display = app.currentView === 'grid' ? 'grid' : 'none';
    }
    
    const filesListView = document.getElementById('files-list-view');
    if (filesListView) {
        filesListView.style.display = app.currentView === 'list' ? 'block' : 'none';
    }
    
    // Use user's home folder instead of root path
    if (app.userHomeFolderId) {
        app.currentPath = app.userHomeFolderId;
        ui.updateBreadcrumb(app.userHomeFolderName || 'Home');
    } else {
        // If no home folder is set, this will trigger finding it in loadFiles()
        app.currentPath = '';
    }
    loadFiles();
}

/**
 * Switch to the favorites view
 */
function switchToFavoritesView() {
    // Hide other views
    app.isTrashView = false;
    app.isSharedView = false;
    
    // Set favorites view as active
    app.isFavoritesView = true;
    app.currentSection = 'favorites';
    
    // Remove active class from all nav items
    elements.navItems.forEach(navItem => navItem.classList.remove('active'));
    
    // Find favorites nav item and make it active
    const favoritesNavItem = document.querySelector('.nav-item:nth-child(4)');
    if (favoritesNavItem) {
        favoritesNavItem.classList.add('active');
    }
    
    // Update UI
    elements.pageTitle.textContent = window.i18n ? window.i18n.t('nav.favorites') : 'Favoritos';
    
    // Clear breadcrumb and show root
    ui.updateBreadcrumb('');
    
    // Hide shared view if it exists
    if (window.sharedView) {
        window.sharedView.hide();
    }
    
    // Configure actions bar for favorites view
    elements.actionsBar.innerHTML = `
        <div class="action-buttons">
            <!-- No actions needed for favorites view -->
        </div>
        <div class="view-toggle">
            <button class="toggle-btn active" id="grid-view-btn" title="Vista de cuadrícula">
                <i class="fas fa-th"></i>
            </button>
            <button class="toggle-btn" id="list-view-btn" title="Vista de lista">
                <i class="fas fa-list"></i>
            </button>
        </div>
    `;
    elements.actionsBar.style.display = 'flex';
    
    // Restore view toggle event listeners
    document.getElementById('grid-view-btn').addEventListener('click', ui.switchToGridView);
    document.getElementById('list-view-btn').addEventListener('click', ui.switchToListView);
    
    // Update cached elements
    elements.gridViewBtn = document.getElementById('grid-view-btn');
    elements.listViewBtn = document.getElementById('list-view-btn');
    
    // Show standard files containers
    const filesGrid = document.getElementById('files-grid');
    const filesListView = document.getElementById('files-list-view');
    
    if (filesGrid) {
        filesGrid.style.display = app.currentView === 'grid' ? 'grid' : 'none';
    }
    
    if (filesListView) {
        filesListView.style.display = app.currentView === 'list' ? 'block' : 'none';
    }
    
    // Check if favorites module is initialized
    if (window.favorites) {
        // Display favorites
        window.favorites.displayFavorites();
    } else {
        console.error('Favorites module not loaded or initialized');
        
        // Show error in UI
        const filesGrid = document.getElementById('files-grid');
        if (filesGrid) {
            filesGrid.innerHTML = `
                <div class="empty-state">
                    <i class="fas fa-exclamation-circle" style="font-size: 48px; color: #f44336; margin-bottom: 16px;"></i>
                    <p>Error al cargar el módulo de favoritos</p>
                </div>
            `;
        }
    }
}

/**
 * Switch to the recent files view
 */
function switchToRecentFilesView() {
    // Hide other views
    app.isTrashView = false;
    app.isSharedView = false;
    app.isFavoritesView = false;
    
    // Set recent view as active
    app.isRecentView = true;
    app.currentSection = 'recent';
    
    // Remove active class from all nav items
    elements.navItems.forEach(navItem => navItem.classList.remove('active'));
    
    // Find recent nav item and make it active
    const recentNavItem = document.querySelector('.nav-item:nth-child(3)');
    if (recentNavItem) {
        recentNavItem.classList.add('active');
    }
    
    // Update UI
    elements.pageTitle.textContent = window.i18n ? window.i18n.t('nav.recent') : 'Recientes';
    
    // Clear breadcrumb and show root
    ui.updateBreadcrumb('');
    
    // Hide shared view if it exists
    if (window.sharedView) {
        window.sharedView.hide();
    }
    
    // Configure actions bar for recent view
    elements.actionsBar.innerHTML = `
        <div class="action-buttons">
            <button class="btn btn-secondary" id="clear-recent-btn">
                <i class="fas fa-broom" style="margin-right: 5px;"></i> <span data-i18n="actions.clear_recent">Limpiar recientes</span>
            </button>
        </div>
        <div class="view-toggle">
            <button class="toggle-btn active" id="grid-view-btn" title="Vista de cuadrícula">
                <i class="fas fa-th"></i>
            </button>
            <button class="toggle-btn" id="list-view-btn" title="Vista de lista">
                <i class="fas fa-list"></i>
            </button>
        </div>
    `;
    elements.actionsBar.style.display = 'flex';
    
    // Add event listener for clear button
    document.getElementById('clear-recent-btn').addEventListener('click', () => {
        if (window.recent) {
            window.recent.clearRecentFiles();
            window.recent.displayRecentFiles();
            window.ui.showNotification('Limpieza completada', 'Se ha limpiado el historial de archivos recientes');
        }
    });
    
    // Restore view toggle event listeners
    document.getElementById('grid-view-btn').addEventListener('click', ui.switchToGridView);
    document.getElementById('list-view-btn').addEventListener('click', ui.switchToListView);
    
    // Update cached elements
    elements.gridViewBtn = document.getElementById('grid-view-btn');
    elements.listViewBtn = document.getElementById('list-view-btn');
    
    // Show standard files containers
    const filesGrid = document.getElementById('files-grid');
    const filesListView = document.getElementById('files-list-view');
    
    if (filesGrid) {
        filesGrid.style.display = app.currentView === 'grid' ? 'grid' : 'none';
    }
    
    if (filesListView) {
        filesListView.style.display = app.currentView === 'list' ? 'block' : 'none';
    }
    
    // Check if recent files module is initialized
    if (window.recent) {
        // Display recent files
        window.recent.displayRecentFiles();
    } else {
        console.error('Recent files module not loaded or initialized');
        
        // Show error in UI
        const filesGrid = document.getElementById('files-grid');
        if (filesGrid) {
            filesGrid.innerHTML = `
                <div class="empty-state">
                    <i class="fas fa-exclamation-circle" style="font-size: 48px; color: #f44336; margin-bottom: 16px;"></i>
                    <p>Error al cargar el módulo de archivos recientes</p>
                </div>
            `;
        }
    }
}

// Expose view switching functions globally
window.switchToFilesView = switchToFilesView;
window.switchToSharedView = switchToSharedView;
window.switchToFavoritesView = switchToFavoritesView;
window.switchToRecentFilesView = switchToRecentFilesView;

/**
 * Fetch updated user data from the server (including storage usage)
 * This calls the /api/auth/me endpoint which also triggers storage recalculation
 */
async function refreshUserData() {
    const TOKEN_KEY = 'oxicloud_token';
    const USER_DATA_KEY = 'oxicloud_user';
    
    const token = localStorage.getItem(TOKEN_KEY);
    console.log('refreshUserData called, token:', token ? token.substring(0, 20) + '...' : 'null');
    
    if (!token || token === 'mock_token_emergency_bypass' || token === 'emergency_token') {
        console.log('No valid token, skipping user data refresh');
        return null;
    }
    
    try {
        console.log('Fetching /api/auth/me...');
        const response = await fetch('/api/auth/me', {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${token}`,
                'Content-Type': 'application/json'
            }
        });
        
        console.log('/api/auth/me response status:', response.status);
        
        if (!response.ok) {
            console.warn('Failed to fetch user data:', response.status);
            return null;
        }
        
        const userData = await response.json();
        console.log('Refreshed user data from server:', userData);
        console.log('Storage from server: used=', userData.storage_used_bytes, 'quota=', userData.storage_quota_bytes);
        
        // Update local storage with fresh data
        localStorage.setItem(USER_DATA_KEY, JSON.stringify(userData));
        
        // Update storage display with actual values
        updateStorageUsageDisplay(userData);
        
        return userData;
    } catch (error) {
        console.error('Error refreshing user data:', error);
        return null;
    }
}

// Expose refreshUserData globally
window.refreshUserData = refreshUserData;

/**
 * Show User Profile modal with account details
 */
function showUserProfileModal() {
    const USER_DATA_KEY = 'oxicloud_user';
    const userData = JSON.parse(localStorage.getItem(USER_DATA_KEY) || '{}');
    const username = userData.username || 'Usuario';
    const email = userData.email || '';
    const role = userData.role || 'user';
    const initials = username.substring(0, 2).toUpperCase();
    const usedBytes = userData.storage_used_bytes || 0;
    const quotaBytes = userData.storage_quota_bytes || 0;
    const percentage = quotaBytes > 0 ? Math.min(Math.round((usedBytes / quotaBytes) * 100), 100) : 0;
    const barColor = percentage > 90 ? '#ef4444' : percentage > 70 ? '#f59e0b' : '#22c55e';
    
    const t = (key, fallback) => (window.i18n && window.i18n.t) ? window.i18n.t(key) || fallback : fallback;
    
    // Remove existing modal if any
    const existing = document.getElementById('profile-modal-overlay');
    if (existing) existing.remove();
    
    const overlay = document.createElement('div');
    overlay.id = 'profile-modal-overlay';
    overlay.className = 'about-modal-overlay';
    overlay.innerHTML = `
        <div class="about-modal" style="max-width:380px">
            <div style="text-align:center;padding:20px 20px 0">
                <div style="width:64px;height:64px;border-radius:50%;background:linear-gradient(135deg,#3b82f6,#6366f1);color:#fff;display:inline-flex;align-items:center;justify-content:center;font-size:24px;font-weight:700;margin-bottom:12px">${initials}</div>
                <h3 style="margin:0;font-size:18px;color:#1a1a2e">${username}</h3>
                <p style="margin:4px 0 0;font-size:13px;color:#64748b">${email}</p>
                <span style="display:inline-block;margin-top:8px;padding:2px 10px;border-radius:10px;font-size:11px;font-weight:600;${
                    role === 'admin' 
                        ? 'background:#dbeafe;color:#1d4ed8' 
                        : 'background:#f1f5f9;color:#64748b'
                }">${role === 'admin' ? '🛡️ Admin' : '👤 ' + t('user_menu.role_user', 'User')}</span>
            </div>
            <div style="padding:16px 20px">
                <div style="font-size:12px;color:#64748b;text-transform:uppercase;letter-spacing:.05em;margin-bottom:6px">
                    <i class="fas fa-database" style="margin-right:4px"></i>${t('storage.title', 'Storage')}
                </div>
                <div style="background:#f1f5f9;border-radius:6px;height:8px;overflow:hidden;margin-bottom:4px">
                    <div style="height:100%;width:${percentage}%;background:${barColor};border-radius:6px;transition:width .3s"></div>
                </div>
                <div style="font-size:12px;color:#64748b;text-align:right">${percentage}% · ${formatFileSize(usedBytes)} / ${quotaBytes > 0 ? formatFileSize(quotaBytes) : '∞'}</div>
            </div>
            <div style="padding:0 20px 16px;display:flex;justify-content:center">
                <button id="profile-modal-close" style="padding:8px 24px;border:1px solid #e2e8f0;border-radius:8px;background:#fff;color:#334155;font-size:13px;font-weight:600;cursor:pointer;transition:background .15s">${t('actions.close', 'Close')}</button>
            </div>
        </div>
    `;
    
    document.body.appendChild(overlay);
    // Show with animation
    requestAnimationFrame(() => overlay.classList.add('show'));
    
    // Close handlers
    overlay.querySelector('#profile-modal-close').addEventListener('click', () => {
        overlay.classList.remove('show');
        setTimeout(() => overlay.remove(), 200);
    });
    overlay.addEventListener('click', (e) => {
        if (e.target === overlay) {
            overlay.classList.remove('show');
            setTimeout(() => overlay.remove(), 200);
        }
    });
}

/**
 * Check if user is authenticated and load user's home folder
 */
function checkAuthentication() {
    // COMPLETE BREAK FOR AUTHENTICATION LOOPS: 
    // Always allow app to load with minimal authentication
    // This is an emergency fix to stop the redirect loops

    // Check URL for no_redirect parameter that indicates we should bypass auth
    const bypassAuth = window.location.search.includes('no_redirect=true') || 
                        window.location.search.includes('bypass_auth=true');
    
    if (bypassAuth) {
        console.log('CRITICAL: Bypassing all authentication checks due to URL parameter');
        
        // Always force a clean authentication state to break loops
        const TOKEN_KEY = 'oxicloud_token';
        const USER_DATA_KEY = 'oxicloud_user';
        
        // Set a mock token if needed
        if (!localStorage.getItem(TOKEN_KEY)) {
            console.log('Setting mock token to prevent redirects');
            localStorage.setItem(TOKEN_KEY, 'mock_token_emergency_bypass');
            // Set expiry far in the future
            localStorage.setItem('oxicloud_token_expiry', 
                new Date(Date.now() + 86400000 * 30).toISOString()); // 30 days
        }
        
        // Create minimal user data to make the app work
        const userData = JSON.parse(localStorage.getItem(USER_DATA_KEY) || '{}');
        if (!userData.username) {
            console.log('No user data found, creating mock user');
            const defaultUserData = {
                id: 'default-user-id',
                username: 'usuario',
                email: 'usuario@example.com',
                storage_quota_bytes: 10737418240, // 10GB default
                storage_used_bytes: 0
            };
            localStorage.setItem(USER_DATA_KEY, JSON.stringify(defaultUserData));
            
            // Update avatar with default initials
            document.querySelectorAll('.user-avatar, .user-menu-avatar').forEach(el => el.textContent = 'US');
            
            // Update storage display with default values
            updateStorageUsageDisplay(defaultUserData);
        } else {
            // Update avatar with user initials
            const userInitials = userData.username.substring(0, 2).toUpperCase();
            document.querySelectorAll('.user-avatar, .user-menu-avatar').forEach(el => el.textContent = userInitials);
            
            // Show cached storage first, then try to refresh from server
            updateStorageUsageDisplay(userData);
            
            // Try to get updated storage from server (if we have a real token)
            const token = localStorage.getItem(TOKEN_KEY);
            if (token && token !== 'mock_token_emergency_bypass' && token !== 'emergency_token') {
                console.log('Bypass mode: Attempting to refresh storage from server...');
                refreshUserData().then(freshData => {
                    if (freshData) {
                        console.log('Bypass mode: Storage updated from server');
                    }
                }).catch(err => {
                    console.warn('Bypass mode: Could not refresh user data:', err);
                });
            }
        }
        
        // Reset all counters to prevent loops
        sessionStorage.removeItem('redirect_count');
        localStorage.setItem('refresh_attempts', '0');
        
        // Proceed directly to load files
        app.currentPath = '';
        ui.updateBreadcrumb('');
        loadFiles();
        return;
    }
    
    try {
        // Simplified authentication check - just verify token exists
        const TOKEN_KEY = 'oxicloud_token';
        const USER_DATA_KEY = 'oxicloud_user';
        
        // Reset counters to prevent loops
        sessionStorage.removeItem('redirect_count');
        localStorage.setItem('refresh_attempts', '0');
        
        // Simple token check - just verify it exists
        const token = localStorage.getItem(TOKEN_KEY);
        
        if (!token) {
            console.log('No token found, redirecting to login');
            // Avoid potential loop by adding a parameter
            const redirectUrl = '/login.html?source=app';
            window.location.href = redirectUrl;
            return;
        }

        // Token exists, proceed with minimal validation
        console.log('Token found, proceeding with app initialization');
        
        // Display user information if available
        const userData = JSON.parse(localStorage.getItem(USER_DATA_KEY) || '{}');
        if (userData.username) {
            // Update user avatar with initials
            const userInitials = userData.username.substring(0, 2).toUpperCase();
            document.querySelectorAll('.user-avatar, .user-menu-avatar').forEach(el => {
                el.textContent = userInitials;
            });
            // Update user menu info
            const menuName = document.getElementById('user-menu-name');
            const menuEmail = document.getElementById('user-menu-email');
            if (menuName) menuName.textContent = userData.username;
            if (menuEmail) menuEmail.textContent = userData.email || '';
            
            // Update storage usage information with cached data first (for fast display)
            updateStorageUsageDisplay(userData);
            
            // Then refresh user data from server in the background to get updated storage
            // This triggers the backend to recalculate storage and returns fresh data
            refreshUserData().then(freshData => {
                if (freshData) {
                    console.log('Storage usage updated from server');
                }
            }).catch(err => {
                console.warn('Could not refresh user data:', err);
            });
            
            // Find and load the user's home folder
            findUserHomeFolder(userData.username);
        } else {
            // If no user data but we have a token, create default user data
            console.log('No user data but token exists, using default user');
            const defaultUserData = {
                id: 'default-user-id',
                username: 'usuario',
                email: 'usuario@example.com',
                storage_quota_bytes: 10737418240, // 10GB default
                storage_used_bytes: 0
            };
            localStorage.setItem(USER_DATA_KEY, JSON.stringify(defaultUserData));
            
            // Update avatar with default initials
            document.querySelectorAll('.user-avatar, .user-menu-avatar').forEach(el => el.textContent = 'US');
            
            // Update storage display with default values
            updateStorageUsageDisplay(defaultUserData);
            
            // Find and load default folder
            app.currentPath = '';
            ui.updateBreadcrumb('');
            loadFiles();
        }
    } catch (error) {
        console.error('Error during authentication check:', error);
        
        // CRITICAL: On any error, create emergency bypass to break any loops
        console.log('Creating emergency authentication bypass due to error');
        localStorage.setItem('oxicloud_token', 'emergency_token');
        localStorage.setItem('oxicloud_token_expiry', 
            new Date(Date.now() + 86400000 * 30).toISOString()); // 30 days
        
        const defaultUserData = {
            id: 'emergency-user-id',
            username: 'usuario',
            email: 'usuario@example.com',
            storage_quota_bytes: 10737418240, // 10GB default
            storage_used_bytes: 0
        };
        localStorage.setItem('oxicloud_user', JSON.stringify(defaultUserData));
        
        // Update avatar
        document.querySelectorAll('.user-avatar, .user-menu-avatar').forEach(el => el.textContent = 'US');
        
        // Update storage display with default values
        updateStorageUsageDisplay(defaultUserData);
        
        // Load root files
        app.currentPath = '';
        ui.updateBreadcrumb('');
        loadFiles();
    }
}

/**
 * Find the user's home folder and load it
 * @param {string} username - The current user's username
 */
async function findUserHomeFolder(username) {
    try {
        console.log("Finding home folder for user:", username);
        
        // CRITICAL FIX: Always create a default folder if needed
        // This prevents loops when the folder can't be found
        const defaultFolder = {
            id: 'default-folder',
            name: `Mi Carpeta - ${username}`,
            parent_id: null,
            created_at: Date.now() / 1000,
            updated_at: Date.now() / 1000
        };
        
        // First, load all folders at the root
        console.log("Fetching folders from API");
        
        // Set max retries and timeout to prevent potential infinite loops
        let retries = 0;
        const maxRetries = 1; // Reduced from 2 to 1
        
        while (retries < maxRetries) {
            try {
                const controller = new AbortController();
                const timeoutId = setTimeout(() => controller.abort(), 3000); // Reduced timeout to 3 seconds
                
                const response = await fetch('/api/folders', {
                    headers: {
                        'Authorization': `Bearer ${localStorage.getItem('oxicloud_token')}`
                    },
                    signal: controller.signal
                });
                
                clearTimeout(timeoutId);
                
                if (response.status === 401 || response.status === 403) {
                    console.warn(`Authentication error (${response.status}) when fetching folders`);
                    // Use default folder to break the loop
                    console.log('Using default folder to prevent redirection loop');
                    app.userHomeFolderId = defaultFolder.id;
                    app.userHomeFolderName = defaultFolder.name;
                    app.currentPath = defaultFolder.id;
                    ui.updateBreadcrumb(defaultFolder.name);
                    loadFiles();
                    return;
                }
                
                if (!response.ok) {
                    throw new Error(`Error loading folders: ${response.status}`);
                }
                
                const folders = await response.json();
                const folderList = Array.isArray(folders) ? folders : [];
                
                console.log(`Found ${folderList.length} folders at root`);
                
                // Look for a folder with a name pattern that matches the user's home folder
                // Only exact match "Mi Carpeta - username"
                const homeFolderPattern = `Mi Carpeta - ${username}`;
                
                // Filter first to remove system folders like .trash that shouldn't be visible
                const visibleFolders = folderList.filter(folder => {
                    // Skip system folders (starting with dot)
                    if (folder.name.startsWith('.')) {
                        return false;
                    }
                    
                    // Skip other users' folders
                    if (folder.name.startsWith('Mi Carpeta - ') && !folder.name.includes(username)) {
                        return false;
                    }
                    
                    return true;
                });
                
                // Find the user's home folder from filtered list
                let homeFolder = visibleFolders.find(folder => folder.name === homeFolderPattern);
                
                if (homeFolder) {
                    console.log(`Found user's home folder: ${homeFolder.name} (${homeFolder.id})`);
                    
                    // Store the home folder ID and name in the app state
                    // This is used for breadcrumb navigation and restricting user access
                    app.userHomeFolderId = homeFolder.id;
                    app.userHomeFolderName = homeFolder.name;
                    
                    // Set this as the current path and load its contents
                    app.currentPath = homeFolder.id;
                    ui.updateBreadcrumb(homeFolder.name);
                    loadFiles();
                    return; // Success! Exit function
                } else {
                    console.warn("Could not find user's home folder, fallback to first folder or root");
                    
                    // If we can't find a specific home folder but there are folders, 
                    // use the first folder as the user's home
                    if (folderList.length > 0) {
                        const fallbackFolder = folderList[0];
                        console.log(`Using first folder as fallback: ${fallbackFolder.name} (${fallbackFolder.id})`);
                        
                        app.userHomeFolderId = fallbackFolder.id;
                        app.userHomeFolderName = fallbackFolder.name;
                        app.currentPath = fallbackFolder.id;
                        ui.updateBreadcrumb(fallbackFolder.name);
                        loadFiles();
                        return; // Success with fallback! Exit function
                    } else {
                        // No folders at all - this is an edge case
                        console.warn("No folders found, using root");
                        app.currentPath = '';
                        ui.updateBreadcrumb('');
                        loadFiles();
                        return; // Success with root! Exit function
                    }
                }
                
                // If we get here, we've successfully processed the response
                break;
                
            } catch (fetchError) {
                retries++;
                console.error(`Fetch attempt ${retries} failed:`, fetchError);
                
                if (retries >= maxRetries) {
                    throw fetchError; // Re-throw after max retries
                }
                
                // Wait before retrying
                await new Promise(resolve => setTimeout(resolve, 1000));
            }
        }
    } catch (error) {
        console.error('Error finding user home folder:', error);
        
        // Fall back to loading root in case of error
        // This is a critical fallback to prevent infinite loops
        app.currentPath = '';
        ui.updateBreadcrumb('');
        loadFiles();
    }
}

/**
 * Logout - clear all auth data and redirect to login
 */
function logout() {
    // Nombres de variables según auth.js
    const TOKEN_KEY = 'oxicloud_token';
    const REFRESH_TOKEN_KEY = 'oxicloud_refresh_token';
    const TOKEN_EXPIRY_KEY = 'oxicloud_token_expiry';
    const USER_DATA_KEY = 'oxicloud_user';
    
    // Clear all authentication data
    localStorage.removeItem(TOKEN_KEY);
    localStorage.removeItem(REFRESH_TOKEN_KEY);
    localStorage.removeItem(TOKEN_EXPIRY_KEY);
    localStorage.removeItem(USER_DATA_KEY);
    
    // Also clear session storage counters
    sessionStorage.removeItem('redirect_count');
    
    // Redirect to login page with correct path
    window.location.href = '/login.html';
}

/**
 * Update the storage usage display with the user's actual storage usage
 * @param {Object} userData - The user data object
 */
function updateStorageUsageDisplay(userData) {
    // Default values
    let usedBytes = 0;
    let quotaBytes = 10737418240; // Default 10GB
    let usagePercentage = 0;

    // Get values from user data if available
    if (userData) {
        usedBytes = userData.storage_used_bytes || 0;
        quotaBytes = userData.storage_quota_bytes || 10737418240;
        
        // Calculate percentage (avoid division by zero)
        if (quotaBytes > 0) {
            usagePercentage = Math.min(Math.round((usedBytes / quotaBytes) * 100), 100);
        }
    }

    // Format the numbers for display
    const usedFormatted = formatFileSize(usedBytes);
    const quotaFormatted = formatFileSize(quotaBytes);

    // Update the storage display elements
    const storageFill = document.querySelector('.storage-fill');
    const storageInfo = document.querySelector('.storage-info');
    
    if (storageFill) {
        storageFill.style.width = `${usagePercentage}%`;
    }
    
    if (storageInfo) {
        // Remove data-i18n attribute to prevent i18n from overwriting our value
        storageInfo.removeAttribute('data-i18n');
        
        // Use i18n if available
        if (window.i18n && window.i18n.t) {
            storageInfo.textContent = window.i18n.t('storage.used', {
                percentage: usagePercentage,
                used: usedFormatted,
                total: quotaFormatted
            });
        } else {
            storageInfo.textContent = `${usagePercentage}% used (${usedFormatted} / ${quotaFormatted})`;
        }
    }
    
    console.log(`Updated storage display: ${usagePercentage}% (${usedFormatted} / ${quotaFormatted})`);
}

// Initialize app when DOM is ready
document.addEventListener('DOMContentLoaded', initApp);
