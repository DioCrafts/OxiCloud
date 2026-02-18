/**
 * OxiCloud - Main Application
 * This file contains the core functionality, initialization and state management
 */

/**
 * Escape HTML special characters to prevent XSS attacks.
 * Use this whenever inserting user-provided text (file names, folder names, etc.) into HTML.
 * @param {string} str - The string to escape
 * @returns {string} The escaped string safe for HTML insertion
 */
function escapeHtml(str) {
    if (typeof str !== 'string') return '';
    return str
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;')
        .replace(/'/g, '&#039;');
}
window.escapeHtml = escapeHtml;

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

// Upload dropdown listener state (prevents accumulated listeners)
let uploadDropdownDocumentClickHandler = null;
let uploadDropdownBindingsController = null;
let actionsBarDelegationBound = false;

const ACTIONS_BAR_TEMPLATES = {
    files: `
        <div class="action-buttons">
            <div class="upload-dropdown" id="upload-dropdown">
                <button class="btn btn-primary" id="upload-btn">
                    <i class="fas fa-cloud-upload-alt" style="margin-right: 5px;"></i>
                    <span data-i18n="actions.upload">Upload</span>
                    <i class="fas fa-caret-down" style="margin-left: 4px; font-size: 12px;"></i>
                </button>
                <div class="upload-dropdown-menu" id="upload-dropdown-menu">
                    <button class="upload-dropdown-item" id="upload-files-btn">
                        <i class="fas fa-file"></i>
                        <span data-i18n="actions.upload_files">Upload files</span>
                    </button>
                    <button class="upload-dropdown-item" id="upload-folder-btn">
                        <i class="fas fa-folder-open"></i>
                        <span data-i18n="actions.upload_folder">Upload folder</span>
                    </button>
                </div>
            </div>
            <button class="btn btn-secondary" id="new-folder-btn">
                <i class="fas fa-folder-plus" style="margin-right: 5px;"></i>
                <span data-i18n="actions.new_folder">New folder</span>
            </button>
        </div>
        <div class="view-toggle">
            <button class="toggle-btn active" id="grid-view-btn" title="Grid view">
                <i class="fas fa-th"></i>
            </button>
            <button class="toggle-btn" id="list-view-btn" title="List view">
                <i class="fas fa-list"></i>
            </button>
        </div>
    `,
    trash: `
        <div class="action-buttons">
            <button class="btn btn-danger" id="empty-trash-btn">
                <i class="fas fa-trash-alt"></i>
                <span data-i18n="trash.empty_trash">Empty trash</span>
            </button>
        </div>
    `,
    favorites: `
        <div class="action-buttons"></div>
        <div class="view-toggle">
            <button class="toggle-btn active" id="grid-view-btn" title="Grid view">
                <i class="fas fa-th"></i>
            </button>
            <button class="toggle-btn" id="list-view-btn" title="List view">
                <i class="fas fa-list"></i>
            </button>
        </div>
    `,
    recent: `
        <div class="action-buttons">
            <button class="btn btn-secondary" id="clear-recent-btn">
                <i class="fas fa-broom" style="margin-right: 5px;"></i>
                <span data-i18n="actions.clear_recent">Clear recent</span>
            </button>
        </div>
        <div class="view-toggle">
            <button class="toggle-btn active" id="grid-view-btn" title="Grid view">
                <i class="fas fa-th"></i>
            </button>
            <button class="toggle-btn" id="list-view-btn" title="List view">
                <i class="fas fa-list"></i>
            </button>
        </div>
    `
};

const LAZY_MODULES = {
    search: {
        url: '/js/search.js',
        isReady: () => !!window.search,
    },
    favorites: {
        url: '/js/favorites.js',
        isReady: () => !!window.favorites,
    },
    recent: {
        url: '/js/recent.js',
        isReady: () => !!window.recent,
    },
    sharedView: {
        url: '/js/components/sharedView.js',
        isReady: () => !!window.sharedView,
    },
};

const lazyLoadPromises = new Map();

function loadScriptOnce(url) {
    if (lazyLoadPromises.has(url)) {
        return lazyLoadPromises.get(url);
    }

    const existing = document.querySelector(`script[src="${url}"]`);
    if (existing) {
        const alreadyLoaded = existing.dataset.loaded === 'true';
        if (alreadyLoaded) {
            return Promise.resolve();
        }
        const pending = new Promise((resolve, reject) => {
            existing.addEventListener('load', () => {
                existing.dataset.loaded = 'true';
                resolve();
            }, { once: true });
            existing.addEventListener('error', () => {
                reject(new Error(`Failed to load script: ${url}`));
            }, { once: true });
        });
        lazyLoadPromises.set(url, pending);
        return pending;
    }

    const promise = new Promise((resolve, reject) => {
        const script = document.createElement('script');
        script.src = url;
        script.defer = true;
        script.dataset.lazy = 'true';
        script.addEventListener('load', () => {
            script.dataset.loaded = 'true';
            resolve();
        }, { once: true });
        script.addEventListener('error', () => {
            reject(new Error(`Failed to load script: ${url}`));
        }, { once: true });
        document.head.appendChild(script);
    });

    lazyLoadPromises.set(url, promise);
    return promise;
}

async function ensureModule(moduleName) {
    const moduleConfig = LAZY_MODULES[moduleName];
    if (!moduleConfig) return;

    if (moduleConfig.isReady()) return;

    await loadScriptOnce(moduleConfig.url);

    if (!moduleConfig.isReady()) {
        throw new Error(`Module loaded but not ready: ${moduleName}`);
    }
}

window.loadSearchModule = async function loadSearchModule() {
    await ensureModule('search');
};

window.loadSharedViewModule = async function loadSharedViewModule() {
    await ensureModule('sharedView');
};

window.loadFavoritesModule = async function loadFavoritesModule() {
    await ensureModule('favorites');
    if (window.favorites && window.favorites.init && !window.favorites.__initialized) {
        await window.favorites.init();
        window.favorites.__initialized = true;
    }
};

window.loadRecentModule = async function loadRecentModule() {
    await ensureModule('recent');
    if (window.recent && window.recent.init && !window.recent.__initialized) {
        window.recent.init();
        window.recent.__initialized = true;
    }
};

function warmupLazyModulesInIdle() {
    const runWarmup = () => {
        Promise.resolve()
            .then(() => window.loadRecentModule())
            .then(() => window.loadFavoritesModule())
            .catch((err) => {
                console.warn('Lazy warmup skipped:', err);
            });
    };

    if (typeof requestIdleCallback === 'function') {
        requestIdleCallback(runWarmup, { timeout: 2500 });
    } else {
        setTimeout(runWarmup, 1200);
    }
}

function setActionsBarMode(mode, force = false) {
    if (!elements.actionsBar) return;

    if (mode === 'hidden') {
        elements.actionsBar.style.display = 'none';
        elements.actionsBar.dataset.mode = 'hidden';
        return;
    }

    if (!force && elements.actionsBar.dataset.mode === mode) {
        return;
    }

    const html = ACTIONS_BAR_TEMPLATES[mode];
    if (!html) return;

    elements.actionsBar.innerHTML = html;
    elements.actionsBar.style.display = 'flex';
    elements.actionsBar.dataset.mode = mode;

    // Refresh cached action elements after rebuild
    elements.uploadBtn = document.getElementById('upload-btn');
    elements.newFolderBtn = document.getElementById('new-folder-btn');
    elements.gridViewBtn = document.getElementById('grid-view-btn');
    elements.listViewBtn = document.getElementById('list-view-btn');

    if (window.i18n && window.i18n.translateElement) {
        window.i18n.translateElement(elements.actionsBar);
    }

    if (mode === 'files') {
        setupUploadDropdown();
    }
}

function setupActionsBarDelegation() {
    if (actionsBarDelegationBound || !elements.actionsBar) return;
    actionsBarDelegationBound = true;

    elements.actionsBar.addEventListener('click', async (e) => {
        const btn = e.target.closest('button');
        if (!btn) return;

        switch (btn.id) {
            case 'upload-files-btn': {
                e.stopPropagation();
                const menu = document.getElementById('upload-dropdown-menu');
                if (menu) menu.classList.remove('show');
                if (elements.fileInput) elements.fileInput.click();
                break;
            }
            case 'upload-folder-btn': {
                e.stopPropagation();
                const menu = document.getElementById('upload-dropdown-menu');
                if (menu) menu.classList.remove('show');
                const folderInput = document.getElementById('folder-input');
                if (folderInput) folderInput.click();
                break;
            }
            case 'new-folder-btn': {
                const folderName = await window.Modal.promptNewFolder();
                if (folderName) {
                    fileOps.createFolder(folderName);
                }
                break;
            }
            case 'grid-view-btn':
                ui.switchToGridView();
                break;
            case 'list-view-btn':
                ui.switchToListView();
                break;
            case 'empty-trash-btn':
                if (await fileOps.emptyTrash()) {
                    loadTrashItems();
                }
                break;
            case 'clear-recent-btn':
                if (window.recent) {
                    window.recent.clearRecentFiles();
                    window.recent.displayRecentFiles();
                    window.ui.showNotification('Cleanup completed', 'Recent files history has been cleared');
                }
                break;
            default:
                break;
        }
    });
}

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

    // Warm up non-critical modules after first paint (does not block startup)
    warmupLazyModulesInIdle();
    
    // Ensure inline viewer is initialized
    if (!window.inlineViewer && typeof InlineViewer !== 'undefined') {
        try {
            window.inlineViewer = new InlineViewer();
        } catch (e) {
            console.error('Error initializing inline viewer:', e);
        }
    }
    
    // Favorites and recent modules are lazy-loaded when those views are opened.
    
    // Initialize multi-select / batch actions
    if (window.multiSelect && window.multiSelect.init) {
        console.log('Initializing multi-select module');
        window.multiSelect.init();
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

        // Close notification bell if open
        const notifWrapper = document.getElementById('notif-wrapper');
        const notifBtn = document.getElementById('notif-bell-btn');
        if (notifWrapper) notifWrapper.classList.remove('open');
        if (notifBtn) notifBtn.classList.remove('active');

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
    
    // Theme toggle (dark mode)
    if (themeBtn) {
        const pill = document.getElementById('theme-toggle-pill');
        const isDark = localStorage.getItem('oxicloud_theme') === 'dark';
        if (isDark) {
            if (pill) pill.classList.add('active');
            document.documentElement.setAttribute('data-theme', 'dark');
        }
        
        themeBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            if (pill) {
                pill.classList.toggle('active');
                const dark = pill.classList.contains('active');
                localStorage.setItem('oxicloud_theme', dark ? 'dark' : 'light');
                document.documentElement.setAttribute('data-theme', dark ? 'dark' : 'light');
                window.ui.showNotification(
                    dark ? '🌙' : '☀️',
                    dark ? 'Dark mode enabled' : 'Light mode enabled'
                );
            }
        });
    }
    
    // Admin panel link
    if (adminBtn) {
        adminBtn.addEventListener('click', () => {
            wrapper.classList.remove('open');
            window.location.href = '/admin';
        });
    }
    
    // Profile button — navigates to profile page
    if (profileBtn) {
        profileBtn.addEventListener('click', () => {
            wrapper.classList.remove('open');
            window.location.href = '/profile';
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
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape' && aboutOverlay.classList.contains('show')) {
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
    const quotaBytes = userData.storage_quota_bytes || (10 * 1024 * 1024 * 1024); // 10 GB default
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
    const uploadBtn = document.getElementById('upload-btn');
    const menu = document.getElementById('upload-dropdown-menu');
    
    if (!uploadBtn || !menu) return;

    // Abort any previous local bindings (safe across repeated/rebuilt UI)
    if (uploadDropdownBindingsController) {
        uploadDropdownBindingsController.abort();
    }
    uploadDropdownBindingsController = new AbortController();
    const signal = uploadDropdownBindingsController.signal;
    
    // Toggle dropdown on button click
    uploadBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        const isOpen = menu.classList.contains('show');
        // Close any other open dropdowns
        document.querySelectorAll('.upload-dropdown-menu.show').forEach(m => m.classList.remove('show'));
        if (!isOpen) {
            menu.classList.add('show');
        }
    }, { signal });

    // Close dropdown when clicking outside
    // remove+add stable handler: guarantees exactly one global listener
    if (uploadDropdownDocumentClickHandler) {
        document.removeEventListener('click', uploadDropdownDocumentClickHandler);
    }
    uploadDropdownDocumentClickHandler = (e) => {
        if (e.target.closest('#upload-dropdown')) return;
        document.querySelectorAll('.upload-dropdown-menu.show').forEach(m => m.classList.remove('show'));
    };
    document.addEventListener('click', uploadDropdownDocumentClickHandler);
}

/**
 * Setup event listeners for main UI elements
 */
function setupEventListeners() {
    // Set up drag and drop
    ui.setupDragAndDrop();
    
    // Debounce timer for live search
    let searchDebounceTimer = null;
    const SEARCH_DEBOUNCE_MS = 300;
    const SEARCH_MIN_CHARS = 3;
    
    // Search input — Enter key
    elements.searchInput.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') {
            // Cancel any pending debounce
            if (searchDebounceTimer) clearTimeout(searchDebounceTimer);
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
    
    // Search input — Live search (debounced, after 3+ chars)
    elements.searchInput.addEventListener('input', () => {
        if (searchDebounceTimer) clearTimeout(searchDebounceTimer);
        const query = elements.searchInput.value.trim();
        
        if (query.length >= SEARCH_MIN_CHARS) {
            searchDebounceTimer = setTimeout(() => {
                performSearch(query);
            }, SEARCH_DEBOUNCE_MS);
        } else if (query.length === 0 && app.isSearchMode) {
            // User cleared the search input — return to normal view
            searchDebounceTimer = setTimeout(() => {
                app.isSearchMode = false;
                app.currentPath = '';
                ui.updateBreadcrumb('');
                loadFiles();
            }, SEARCH_DEBOUNCE_MS);
        }
    });
    
    // Search button
    document.getElementById('search-button').addEventListener('click', () => {
        if (searchDebounceTimer) clearTimeout(searchDebounceTimer);
        const query = elements.searchInput.value.trim();
        if (query) {
            performSearch(query);
        }
    });
    
    // Upload dropdown
    setupUploadDropdown();
    setupActionsBarDelegation();
    if (elements.actionsBar) {
        elements.actionsBar.dataset.mode = 'files';
    }
    
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
                elements.pageTitle.textContent = window.i18n ? window.i18n.t('nav.trash') : 'Trash';
                elements.pageTitle.setAttribute('data-i18n', 'nav.trash');
                setActionsBarMode('trash');
                
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
                elements.pageTitle.textContent = window.i18n ? window.i18n.t('nav.files') : 'Files';
                setActionsBarMode('files');
                
                // Show files containers
                const filesGrid = document.getElementById('files-grid');
                const filesListView = document.getElementById('files-list-view');
                if (filesGrid) filesGrid.style.display = app.currentView === 'grid' ? 'grid' : 'none';
                if (filesListView) filesListView.style.display = app.currentView === 'list' ? 'block' : 'none';
                
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
        // Note: multiSelect._hookGlobalDeselect() handles clearing the internal
        // selection state; this handler only covers the legacy CSS class removal.
        if (!e.target.closest('.file-card') && !e.target.closest('.file-item') && !e.target.closest('.context-menu') && !e.target.closest('.about-modal') && !e.target.closest('.batch-action-bar') && !e.target.closest('.list-header.selection-mode')) {
            document.querySelectorAll('.file-card.selected').forEach(c => c.classList.remove('selected'));
            document.querySelectorAll('.file-item.selected').forEach(c => c.classList.remove('selected'));
        }
    });
}

/**
 * Load files and folders for the current path
 */
async function loadFiles(options = {}) {
    try {
        console.log("Starting loadFiles() - loading files...", options);
        
        // Flag to force complete refresh ignoring cache
        const forceRefresh = options.forceRefresh || false;
        
        // Prevent multiple simultaneous load requests
        if (window.isLoadingFiles) {
            console.log("A file load is already in progress, ignoring request");
            return;
        }
        
        window.isLoadingFiles = true;
        
        // Show loading spinner
        elements.filesGrid.innerHTML = `
            <div class="files-loading-spinner">
                <div class="spinner"></div>
                <span>${window.i18n ? window.i18n.t('files.loading') : 'Loading files…'}</span>
            </div>
        `;
        
        // Always ensure a userHomeFolderId is set
        if (!app.userHomeFolderId) {
            await resolveHomeFolder();
        }
        
        // Build the listing URL — single endpoint returns folders + files together
        const timestamp = new Date().getTime();
        let url;
        
        // ALWAYS use the userHomeFolderId (current folder or home folder) to avoid showing root
        if (!app.currentPath || app.currentPath === '') {
            // If at root, force user to their home folder
            if (app.userHomeFolderId) {
                url = `/api/folders/${app.userHomeFolderId}/listing?t=${timestamp}`;
                app.currentPath = app.userHomeFolderId;
                ui.updateBreadcrumb(app.userHomeFolderName || 'Home');
                console.log(`Loading user folder: ${app.userHomeFolderName} (${app.userHomeFolderId})`);
            } else {
                // Emergency fallback - this should rarely happen but prevents errors
                url = `/api/folders?t=${timestamp}`;
                console.warn("Emergency fallback to root folder - this should not normally happen");
            }
        } else {
            // Normal case - viewing subfolder contents
            url = `/api/folders/${app.currentPath}/listing?t=${timestamp}`;
            console.log(`Loading subfolder content: ${app.currentPath}`);
        }
        
        const token = localStorage.getItem('oxicloud_token');
        const headers = {
            'Cache-Control': 'no-cache, no-store, must-revalidate',
            'Pragma': 'no-cache'
        };
        if (token) {
            headers['Authorization'] = `Bearer ${token}`;
        }
        const requestOptions = {
            headers,
            cache: 'no-store'  // Instruct the browser not to use cache
        };
        
        // If forceRefresh is specified, add an additional parameter to avoid cache
        if (forceRefresh) {
            url += `&force_refresh=true`;
            requestOptions.headers['X-Force-Refresh'] = 'true';
            console.log('Forcing complete refresh ignoring cache');
        }
        
        console.log(`Loading listing from ${url}`);
        const response = await fetch(url, requestOptions);
        
        // Critical error handling
        if (response.status === 401 || response.status === 403) {
            console.warn("Auth error when loading files, showing empty list");
            elements.filesGrid.innerHTML = '<div class="empty-state"><p>Could not load files</p></div>';
            elements.filesListView.innerHTML = `
                <div class="list-header">
                    <div class="list-header-checkbox"><input type="checkbox" id="select-all-checkbox" title="Select all"></div>
                    <div>Name</div>
                    <div>Type</div>
                    <div>Size</div>
                    <div>Modified</div>
                </div>
            `;
            return;
        }
        
        if (!response.ok) {
            throw new Error(`Server responded with status: ${response.status}`);
        }
        
        // Single response contains both folders and files
        const listing = await response.json();
        
        // Clear existing content in both views
        if (window.multiSelect) window.multiSelect.clear();
        ui._items.clear();
        elements.filesGrid.innerHTML = '';
        const _t = (window.i18n && window.i18n.t) ? window.i18n.t : k => k.split('.').pop();
        elements.filesListView.innerHTML = `
            <div class="list-header">
                <div class="list-header-checkbox"><input type="checkbox" id="select-all-checkbox" title="Select all"></div>
                <div data-i18n="files.name">${_t('files.name')}</div>
                <div data-i18n="files.type">${_t('files.type')}</div>
                <div data-i18n="files.size">${_t('files.size')}</div>
                <div data-i18n="files.modified">${_t('files.modified')}</div>
            </div>
        `;

        // Re-wire select-all checkbox after DOM rebuild
        const selectAllCb = document.getElementById('select-all-checkbox');
        if (selectAllCb && window.multiSelect) {
            selectAllCb.addEventListener('change', () => window.multiSelect.toggleAll());
        }
        
        // Render folders and files from the combined listing
        const folderList = Array.isArray(listing.folders) ? listing.folders : [];
        const fileList = Array.isArray(listing.files) ? listing.files : [];
        
        ui.renderFolders(folderList);
        ui.renderFiles(fileList);
        
        console.log(`Loaded ${folderList.length} folders and ${fileList.length} files`);
    } catch (error) {
        console.error('Error loading folders:', error);
        ui.showNotification('Error', 'Could not load files and folders');
    } finally {
        // Mark that we are no longer loading files to allow future requests
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
        if (window.multiSelect) window.multiSelect.clear();
        elements.filesGrid.innerHTML = '';
        const _tt = (window.i18n && window.i18n.t) ? window.i18n.t : k => k.split('.').pop();
        elements.filesListView.innerHTML = `
            <div class="list-header trash-header">
                <div data-i18n="files.name">${_tt('files.name')}</div>
                <div data-i18n="files.type">${_tt('files.type')}</div>
                <div data-i18n="trash.original_location">${_tt('trash.original_location')}</div>
                <div data-i18n="trash.deleted_date">${_tt('trash.deleted_date')}</div>
                <div data-i18n="trash.actions">${_tt('trash.actions')}</div>
            </div>
        `;
        
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
                <p>${window.i18n ? window.i18n.t('trash.empty_state') : 'The trash is empty'}</p>
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
        window.ui.showNotification('Error', 'Error loading trash items');
    }
}

/**
 * Add a trash item to the view
 * @param {Object} item - Trash item object
 */
function addTrashItemToView(item) {
    const isFile = item.item_type === 'file';
    
    // Format date - backend sends trashed_at as ISO 8601 string
    const formattedDate = window.formatDateTime(item.trashed_at);
                         
    // Use icon_class from the trash DTO if available, otherwise fall back to
    // the comprehensive icon map exposed by ui.js.
    let iconClass;
    let typeLabel;
    if (!isFile) {
        iconClass = item.icon_class || 'fas fa-folder';
        typeLabel = window.i18n ? window.i18n.t('files.file_types.folder') : 'Folder';
    } else {
        iconClass = item.icon_class || (window.ui && window.ui.getIconClass
            ? window.ui.getIconClass(item.name)
            : 'fas fa-file');
        const cat = item.category || '';
        typeLabel = cat
            ? (window.i18n ? window.i18n.t(`files.file_types.${cat.toLowerCase()}`) || cat : cat)
            : (window.i18n ? window.i18n.t('files.file_types.document') : 'Document');
    }
    
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
        <div class="file-name">${escapeHtml(item.name)}</div>
        <div class="file-info">${escapeHtml(typeLabel)} - ${escapeHtml(formattedDate)}</div>
        <div class="trash-actions">
            <button class="btn-restore" title="${window.i18n ? window.i18n.t('trash.restore') : 'Restore'}">
                <i class="fas fa-undo"></i>
            </button>
            <button class="btn-delete" title="${window.i18n ? window.i18n.t('trash.delete_permanently') : 'Delete permanently'}">
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
            <span>${escapeHtml(item.name)}</span>
        </div>
        <div class="type-cell">${escapeHtml(typeLabel)}</div>
        <div class="path-cell">${escapeHtml(item.original_path || '--')}</div>
        <div class="date-cell">${escapeHtml(formattedDate)}</div>
        <div class="actions-cell">
            <button class="btn-restore" title="${window.i18n ? window.i18n.t('trash.restore') : 'Restore'}">
                <i class="fas fa-undo"></i>
            </button>
            <button class="btn-delete" title="${window.i18n ? window.i18n.t('trash.delete_permanently') : 'Delete permanently'}">
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
 * Perform search with the given query.
 * All processing (filtering, scoring, sorting, categorization) is done
 * server-side in Rust. This function only sends the request and renders.
 *
 * @param {string} query - Search query
 * @param {string} [sortBy] - Sort order (relevance|name|name_desc|date|date_desc|size|size_desc)
 */
async function performSearch(query, sortBy) {
    console.log(`Performing search for: "${query}" (sort: ${sortBy || 'relevance'})`);
    
    try {
        app.isSearchMode = true;
        ui.updateBreadcrumb(`Search: "${query}"`);
        
        // Show loading spinner
        const filesGrid = document.getElementById('files-grid');
        if (filesGrid) {
            filesGrid.innerHTML = `
                <div class="search-results-header">
                    <h3><i class="fas fa-spinner fa-spin" style="margin-right:8px;"></i> Searching for "${query}"...</h3>
                </div>
            `;
        }
        
        // All options — backend handles all processing
        const options = {
            recursive: true,
            limit: 100,
            sort_by: sortBy || 'relevance'
        };
        
        // Restrict search to user's folder context
        if (!app.isTrashView) {
            options.folder_id = app.currentPath;
            
            if (!options.folder_id || options.folder_id === '') {
                await resolveHomeFolder();
                options.folder_id = app.currentPath;
            }
        }

        await window.loadSearchModule();
        
        // Send search request — backend does all processing
        const searchResults = await window.search.searchFiles(query, options);
        
        // Render enriched results from the server
        window.search.displaySearchResults(searchResults);
        
    } catch (error) {
        console.error('Search error:', error);
        window.ui.showNotification('Error', 'Error performing search');
    }
}

// Listen for re-sort events from the search sort dropdown
document.addEventListener('search-resort', (e) => {
    const searchInput = document.querySelector('.search-container input');
    if (searchInput && searchInput.value.trim()) {
        performSearch(searchInput.value.trim(), e.detail.sort_by);
    }
});

// Expose needed functions to global scope
window.app = app;
window.loadFiles = loadFiles;
window.loadTrashItems = loadTrashItems;
window.formatFileSize = formatFileSize;
window.performSearch = performSearch;

/**
 * Centralised date+time formatter — use this everywhere instead of inline
 * toLocaleDateString/toLocaleTimeString calls.
 * @param {Date|number|string} value  Date object, unix-seconds number, or ISO string
 * @returns {string} e.g. "16/02/2026 14:35"
 */
window.formatDateTime = function formatDateTime(value) {
    if (!value) return '';
    let d;
    if (value instanceof Date) {
        d = value;
    } else if (typeof value === 'number') {
        // Heuristic: values < 1e12 are unix seconds, otherwise ms
        d = new Date(value < 1e12 ? value * 1000 : value);
    } else {
        d = new Date(value);
    }
    if (isNaN(d.getTime())) return String(value);
    return d.toLocaleDateString() + ' ' +
           d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
};

/**
 * Short date formatter (no time) for shares / expiration dates.
 * @param {Date|number|string} value
 * @returns {string} e.g. "Feb 16, 2026"
 */
window.formatDateShort = function formatDateShort(value) {
    if (!value) return 'N/A';
    const d = typeof value === 'number' ? new Date(value * 1000) : new Date(value);
    if (isNaN(d.getTime())) return String(value);
    return d.toLocaleDateString(undefined, { year: 'numeric', month: 'short', day: 'numeric' });
};

/**
 * Centralised "is this MIME type text-viewable?" check.
 * Used by inlineViewer and ui.isViewableFile.
 */
window.isTextViewable = function isTextViewable(mimeType) {
    if (!mimeType) return false;
    if (mimeType.startsWith('text/')) return true;
    const textTypes = [
        'application/json', 'application/xml', 'application/javascript',
        'application/x-sh', 'application/x-yaml', 'application/toml',
        'application/x-toml', 'application/sql',
    ];
    return textTypes.includes(mimeType);
};

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
    window.loadSharedViewModule().then(() => {
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
        
        // Update UI — also set data-i18n so translatePage() doesn't overwrite
        elements.pageTitle.textContent = window.i18n ? window.i18n.t('nav.shared') : 'Shared';
        elements.pageTitle.setAttribute('data-i18n', 'nav.shared');
        
        // Clear breadcrumb and show root
        ui.updateBreadcrumb('');
        
        // Hide breadcrumb itself
        const breadcrumb = document.querySelector('.breadcrumb');
        if (breadcrumb) breadcrumb.style.display = 'none';
        
        // Hide standard actions bar
        if (elements.actionsBar) {
            elements.actionsBar.style.display = 'none';
        }
        
        // Hide file containers
        const filesGrid = document.getElementById('files-grid');
        const filesListView = document.getElementById('files-list-view');
        if (filesGrid) filesGrid.style.display = 'none';
        if (filesListView) filesListView.style.display = 'none';
        
        // Init and show shared view
        if (window.sharedView) {
            window.sharedView.init();
            window.sharedView.show();
        }
    }).catch((error) => {
        console.error('Error loading shared view module:', error);
        if (window.ui && window.ui.showNotification) {
            window.ui.showNotification('Error', 'Could not load shared view');
        }
    });
}

/**
 * Switch back to the files view
 */
function switchToFilesView() {
    // Hide trash view if active
    app.isTrashView = false;
    // Reset view flags
    app.isTrashView = false;
    app.isSharedView = false;
    app.isFavoritesView = false;
    app.isRecentView = false;
    app.currentSection = 'files';
    
    // Update UI — restore data-i18n to files
    elements.pageTitle.textContent = window.i18n ? window.i18n.t('nav.files') : 'Files';
    elements.pageTitle.setAttribute('data-i18n', 'nav.files');
    
    // Restore breadcrumb visibility
    const breadcrumb = document.querySelector('.breadcrumb');
    if (breadcrumb) breadcrumb.style.display = '';
    
    // Remove active class from all nav items
    elements.navItems.forEach(navItem => navItem.classList.remove('active'));
    
    // Make files nav item active
    const filesNavItem = document.querySelector('.nav-item:first-child');
    if (filesNavItem) {
        filesNavItem.classList.add('active');
    }
    
    // Reset UI
    setActionsBarMode('files');
    
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
async function switchToFavoritesView() {
    try {
        await window.loadFavoritesModule();
    } catch (error) {
        console.error('Error loading favorites module:', error);
        if (window.ui && window.ui.showNotification) {
            window.ui.showNotification('Error', 'Could not load favorites');
        }
        return;
    }

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
    elements.pageTitle.textContent = window.i18n ? window.i18n.t('nav.favorites') : 'Favorites';
    elements.pageTitle.setAttribute('data-i18n', 'nav.favorites');
    
    // Clear breadcrumb and show root
    ui.updateBreadcrumb('');
    
    // Hide shared view if it exists
    if (window.sharedView) {
        window.sharedView.hide();
    }
    
    // Configure actions bar for favorites view
    setActionsBarMode('favorites');
    
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
                    <p>Error loading the favorites module</p>
                </div>
            `;
        }
    }
}

/**
 * Switch to the recent files view
 */
async function switchToRecentFilesView() {
    try {
        await window.loadRecentModule();
    } catch (error) {
        console.error('Error loading recent module:', error);
        if (window.ui && window.ui.showNotification) {
            window.ui.showNotification('Error', 'Could not load recent files');
        }
        return;
    }

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
    elements.pageTitle.textContent = window.i18n ? window.i18n.t('nav.recent') : 'Recent';
    elements.pageTitle.setAttribute('data-i18n', 'nav.recent');
    
    // Clear breadcrumb and show root
    ui.updateBreadcrumb('');
    
    // Hide shared view if it exists
    if (window.sharedView) {
        window.sharedView.hide();
    }
    
    // Configure actions bar for recent view
    setActionsBarMode('recent');
    
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
                    <p>Error loading the recent files module</p>
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
    
    if (!token) {
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
 * Check if user is authenticated and load user's home folder
 */
async function checkAuthentication() {
    try {
        const TOKEN_KEY = 'oxicloud_token';
        const REFRESH_TOKEN_KEY = 'oxicloud_refresh_token';
        const TOKEN_EXPIRY_KEY = 'oxicloud_token_expiry';
        const USER_DATA_KEY = 'oxicloud_user';
        
        // --- OIDC exchange code handling ---
        // After OIDC login, the backend redirects here with ?oidc_code=...
        const urlParams = new URLSearchParams(window.location.search);
        const oidcCode = urlParams.get('oidc_code');
        
        if (oidcCode) {
            console.log('OIDC exchange code detected, exchanging for tokens...');
            try {
                const exchangeResponse = await fetch('/api/auth/oidc/exchange', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ code: oidcCode })
                });
                
                if (!exchangeResponse.ok) {
                    const errText = await exchangeResponse.text();
                    console.error('OIDC token exchange failed:', exchangeResponse.status, errText);
                    window.location.href = '/login?source=oidc_error';
                    return;
                }
                
                const data = await exchangeResponse.json();
                console.log('OIDC token exchange successful');
                
                // Store tokens (same logic as password login in auth.js)
                const token = data.access_token || data.token;
                const refreshToken = data.refresh_token || data.refreshToken;
                
                if (token) {
                    localStorage.setItem(TOKEN_KEY, token);
                    if (refreshToken) localStorage.setItem(REFRESH_TOKEN_KEY, refreshToken);
                    
                    // Parse JWT expiry
                    let parsedExpiry = false;
                    const tokenParts = token.split('.');
                    if (tokenParts.length === 3) {
                        try {
                            const payload = JSON.parse(atob(tokenParts[1]));
                            if (payload.exp) {
                                const expiryDate = new Date(payload.exp * 1000);
                                if (!isNaN(expiryDate.getTime())) {
                                    localStorage.setItem(TOKEN_EXPIRY_KEY, expiryDate.toISOString());
                                    parsedExpiry = true;
                                }
                            }
                        } catch (e) {
                            console.error('Error parsing JWT:', e);
                        }
                    }
                    if (!parsedExpiry) {
                        const expiry = new Date();
                        expiry.setDate(expiry.getDate() + 30);
                        localStorage.setItem(TOKEN_EXPIRY_KEY, expiry.toISOString());
                    }
                    
                    // Store user data
                    if (data.user) {
                        localStorage.setItem(USER_DATA_KEY, JSON.stringify(data.user));
                    }
                    
                    // Clean URL and reload without the oidc_code param
                    window.history.replaceState({}, document.title, '/');
                    window.location.reload();
                    return;
                }
            } catch (err) {
                console.error('OIDC exchange error:', err);
                window.location.href = '/login?source=oidc_error';
                return;
            }
        }
        
        // Verify token exists
        const token = localStorage.getItem(TOKEN_KEY);
        
        if (!token) {
            console.log('No token found, redirecting to login');
            window.location.href = '/login?source=app';
            return;
        }

        // Token exists, proceed with app initialization
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
            refreshUserData().then(freshData => {
                if (freshData) {
                    console.log('Storage usage updated from server');
                }
            }).catch(err => {
                console.warn('Could not refresh user data:', err);
            });
            
            // Find and load the user's home folder
            resolveHomeFolder().then(() => loadFiles());
        } else {
            // No user data but token exists — try to fetch from server
            console.log('No user data, attempting to fetch from server');
            try {
                const freshData = await refreshUserData();
                if (freshData && freshData.username) {
                    const userInitials = freshData.username.substring(0, 2).toUpperCase();
                    document.querySelectorAll('.user-avatar, .user-menu-avatar').forEach(el => el.textContent = userInitials);
                    updateStorageUsageDisplay(freshData);
                    resolveHomeFolder().then(() => loadFiles());
                } else {
                    // Server didn't return valid user data — token is likely invalid
                    console.warn('Could not retrieve user data, redirecting to login');
                    localStorage.removeItem(TOKEN_KEY);
                    localStorage.removeItem(REFRESH_TOKEN_KEY);
                    localStorage.removeItem(TOKEN_EXPIRY_KEY);
                    localStorage.removeItem(USER_DATA_KEY);
                    window.location.href = '/login?source=invalid_session';
                }
            } catch (err) {
                console.error('Failed to fetch user data:', err);
                localStorage.removeItem(TOKEN_KEY);
                localStorage.removeItem(REFRESH_TOKEN_KEY);
                localStorage.removeItem(TOKEN_EXPIRY_KEY);
                localStorage.removeItem(USER_DATA_KEY);
                window.location.href = '/login?source=session_error';
            }
        }
    } catch (error) {
        console.error('Error during authentication check:', error);
        // On error, redirect to login cleanly — never create fake tokens
        localStorage.removeItem('oxicloud_token');
        localStorage.removeItem('oxicloud_refresh_token');
        localStorage.removeItem('oxicloud_token_expiry');
        localStorage.removeItem('oxicloud_user');
        window.location.href = '/login?source=auth_error';
    }
}

/**
 * Find the user's home folder and load it
 * @param {string} username - The current user's username
 */
/**
 * Resolve the user's home folder from the backend.
 * Since the backend now scopes GET /api/folders to the authenticated user,
 * we simply pick the first root-level folder returned.
 */
async function resolveHomeFolder() {
    if (app.userHomeFolderId) return; // Already resolved
    try {
        const token = localStorage.getItem('oxicloud_token');
        const headers = token ? { 'Authorization': `Bearer ${token}` } : {};
        const response = await fetch('/api/folders', { headers });
        if (!response.ok) {
            console.warn(`Could not fetch home folder: ${response.status}`);
            return;
        }
        const folders = await response.json();
        const folderList = Array.isArray(folders) ? folders : [];
        if (folderList.length > 0) {
            const home = folderList[0];
            app.userHomeFolderId = home.id;
            app.userHomeFolderName = home.name;
            app.currentPath = home.id;
            ui.updateBreadcrumb(home.name);
            console.log(`Home folder resolved: ${home.name} (${home.id})`);
        } else {
            console.warn('No root folders found for user');
            app.currentPath = '';
            ui.updateBreadcrumb('');
        }
    } catch (error) {
        console.error('Error resolving home folder:', error);
        app.currentPath = '';
        ui.updateBreadcrumb('');
    }
}

/**
 * Logout - clear all auth data and redirect to login
 */
function logout() {
    // Variable names as per auth.js
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
    window.location.href = '/login';
}

/**
 * Update the storage usage display with the user's actual storage usage
 * @param {Object} userData - The user data object
 */
function updateStorageUsageDisplay(userData) {
    // Default values
    const DEFAULT_QUOTA = 10 * 1024 * 1024 * 1024; // 10 GB
    let usedBytes = 0;
    let quotaBytes = DEFAULT_QUOTA;
    let usagePercentage = 0;

    // Get values from user data if available
    if (userData) {
        usedBytes = userData.storage_used_bytes || 0;
        quotaBytes = userData.storage_quota_bytes || DEFAULT_QUOTA;
        
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
