/**
 * OxiCloud - Main Application
 * This file contains the core functionality, initialization and state management
 */

const app = window.app;
const elements = window.appElements;

// Upload dropdown listener state (prevents accumulated listeners)
/** @type { function | null } */
let uploadDropdownDocumentClickHandler = null;

/** @type { AbortController | null } */
let uploadDropdownBindingsController = null;
let actionsBarDelegationBound = false;

const ACTIONS_BAR_TEMPLATES = {
    files: `
        <div class="action-buttons">
            <div class="upload-dropdown" id="upload-dropdown">
                <button class="btn btn-primary" id="upload-btn">
                    <i class="fas fa-cloud-upload-alt icon-mr"></i>
                    <span data-i18n="actions.upload">Upload</span>
                    <i class="fas fa-caret-down icon-ml"></i>
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
                <i class="fas fa-folder-plus icon-mr"></i>
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
        <div class="view-toggle">
            <button class="toggle-btn active" id="grid-view-btn" title="Grid view">
                <i class="fas fa-th"></i>
            </button>
            <button class="toggle-btn" id="list-view-btn" title="List view">
                <i class="fas fa-list"></i>
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
                <i class="fas fa-broom icon-mr"></i>
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

/**
 *
 * @param {string} mode
 * @param {boolean} [force=false]
 * @returns
 */
function setActionsBarMode(mode, force = false) {
    if (!elements.actionsBar) return;

    if (mode === 'hidden') {
        elements.actionsBar.classList.add('hidden');
        elements.actionsBar.dataset.mode = 'hidden';
        console.log('......setup actions bar to hidden');
        return;
    }

    if (!force && elements.actionsBar.dataset.mode === mode) {
        return;
    }

    const html = ACTIONS_BAR_TEMPLATES[mode];
    if (!html) return;

    elements.actionsBar.innerHTML = html;
    elements.actionsBar.classList.remove('hidden');
    elements.actionsBar.dataset.mode = mode;

    // Refresh cached action elements after rebuild
    elements.uploadBtn = document.getElementById('upload-btn');
    elements.newFolderBtn = document.getElementById('new-folder-btn');
    elements.gridViewBtn = document.getElementById('grid-view-btn');
    elements.listViewBtn = document.getElementById('list-view-btn');

    if (window.i18n?.translateElement) {
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
                    window.loadTrashItems();
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
 * @typedef {Object} OxiContext
 * @property {string} section
 * @property {string | null} path the uuid of the path
 * @property {string | null} file the uuid of file inline view
 */

/**
 * Read the application hash
 *
 * format:
 *
 * #/<section>/
 *
 * #/shared
 * #/recent
 * ...
 *
 * special case of drive:
 *
 * #/files/folder/<folder ID>
 *
 * @returns {OxiContext}
 */
function deserializeHash() {
    const hashContext = /** type {OxiContext} */ {};

    // FIXME rename files into drive ?
    hashContext.section = 'files';

    const hash_elements = window.location.hash.split('/');

    const section = hash_elements[1];

    // FIXME: use navigation.VIEW_FLAGS
    if (section && ['files', 'shared', 'recent', 'favorites', 'trash', 'photos'].includes(section)) {
        hashContext.section = section;
    }

    if (hash_elements[1] === 'files' && hash_elements[2] === 'folder' && hash_elements[3] !== null) {
        hashContext.path = hash_elements[3];

        if (hash_elements[4] === 'file' && hash_elements[5] !== null) {
            hashContext.file = hash_elements[5];
        }
    }

    return hashContext;
}

/**
 * update borwser's url/history
 *
 * @param {boolean} insertHistory true to change url and browser's history, false to change url only
 */
function updateHistory(insertHistory) {
    const app = window.app;

    const historyData = {
        section: app.currentSection,
        id: app.currentFolder,
        file: app.viewFile
    };

    let historyUrl = `#/${app.currentSection}`;

    if (app.currentSection === 'files' && app.currentFolderInfo !== null) {
        historyData.id = app.currentFolder;
        historyUrl = historyUrl.concat('/folder/', app.currentFolderInfo.id);

        if (window.app.viewFile) {
            historyUrl = historyUrl.concat('/file/', window.app.viewFile);
        }
        // update title
        document.title = `OxiCloud: ${app.currentFolderInfo.path}`;
    }

    if (insertHistory) {
        console.log(`adding history with ${historyUrl}`);
        window.history.pushState(historyData, '', historyUrl);
    } else {
        console.log(`replace history with ${historyUrl}`);
        window.history.replaceState(historyData, '', historyUrl);
    }
}

/**
 *
 * @param {string} section
 * @returns
 */
function switchSectionTo(section) {
    if (window.app.currentSection === section)
        // no change ...
        return;

    switch (section) {
        case 'files':
            switchToFilesSection();
            break;

        case 'shared':
            switchToSharedSection();
            break;

        case 'recent':
            switchToRecentFilesSection();
            break;

        case 'favorites':
            switchToFavoritesSection();
            break;

        case 'photos':
            switchToPhotosSection();
            break;

        case 'trash':
            switchToTrashSection();
            break;

        default:
            console.warn(`context view ${section} unkonwn fallback to drive section`);
            switchToFilesSection();
    }
}

/**
 * Initialize the application
 */
function initApp() {
    // Cache DOM elements
    cacheElements();

    // Initialize file sharing module first
    if (window.fileSharing?.init) {
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

    // Ensure inline viewer is initialized
    if (!window.inlineViewer && typeof InlineViewer !== 'undefined') {
        try {
            window.inlineViewer = new InlineViewer();
        } catch (e) {
            console.error('Error initializing inline viewer:', e);
        }
    }

    // Initialize favorites module if available
    if (window.favorites?.init) {
        console.log('Initializing favorites module');
        window.favorites.init();
    } else {
        console.warn('Favorites module not available or not initializable');
    }

    // Initialize recent files module if available
    if (window.recent?.init) {
        console.log('Initializing recent files module');
        window.recent.init();
    } else {
        console.warn('Recent files module not available or not initializable');
    }

    // Initialize multi-select / batch actions
    if (window.multiSelect?.init) {
        console.log('Initializing multi-select module');
        window.multiSelect.init();
    }

    window.addEventListener('authenticationDone', () => {
        // Check if a context was provided in the URL
        const hashContext = deserializeHash();
        switchSectionTo(hashContext.section);
        if (hashContext.section === 'files') {
            if (hashContext.path) {
                console.log(`init: reusing folder from hash URL: ${hashContext.path}`);
                window.app.currentPath = hashContext.path;
            }

            if (hashContext.file !== null) {
                window.app.viewFile = hashContext.file;
            }
            window.loadFiles();
        }
    });

    // Wait for translations to load before checking authentication
    if (window.i18n?.isLoaded?.()) {
        // Translations already loaded, proceed with authentication
        window.checkAuthentication();
    } else {
        // Wait for translations to be loaded before proceeding
        console.log('Waiting for translations to load...');
        window.addEventListener('translationsLoaded', () => {
            console.log('Translations loaded, proceeding with authentication');
            window.checkAuthentication();
        });

        // Set a timeout as a fallback in case translations take too long
        setTimeout(() => {
            if (!window.i18n?.isLoaded?.()) {
                console.warn('Translations loading timeout, proceeding with authentication anyway');
                window.checkAuthentication();
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
    elements.filesList = document.getElementById('files-list');
    elements.newFolderBtn = document.getElementById('new-folder-btn');
    elements.gridViewBtn = document.getElementById('grid-view-btn');
    elements.listViewBtn = document.getElementById('list-view-btn');
    elements.breadcrumb = document.querySelector('.breadcrumb');
    elements.pageTitle = document.querySelector('.page-title');
    elements.actionsBar = document.getElementById('actions-bar');
    elements.navItems = document.querySelectorAll('.nav-item');
    elements.searchInput = document.querySelector('.search-container input');
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
    uploadBtn.addEventListener(
        'click',
        (e) => {
            e.stopPropagation();
            const isOpen = menu.classList.contains('show');
            // Close any other open dropdowns
            document.querySelectorAll('.upload-dropdown-menu.show').forEach((m) => {
                m.classList.remove('show');
            });
            if (!isOpen) {
                menu.classList.add('show');
            }
        },
        { signal }
    );

    // Close dropdown when clicking outside
    // remove+add stable handler: guarantees exactly one global listener
    if (uploadDropdownDocumentClickHandler) {
        document.removeEventListener('click', uploadDropdownDocumentClickHandler);
    }
    uploadDropdownDocumentClickHandler = (e) => {
        if (e.target.closest('#upload-dropdown')) return;
        document.querySelectorAll('.upload-dropdown-menu.show').forEach((m) => {
            m.classList.remove('show');
        });
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

    // handle history / url change
    window.addEventListener('popstate', (e) => {
        if (e.state === null) {
            // change is from user (url explicitely change, read information from hash)
            const hashContext = deserializeHash();
            switchSectionTo(hashContext.section);
            if (hashContext.path) {
                window.app.currentPath = hashContext.path;
                window.loadFiles({ insertHistory: false });
            }
        } else {
            // change is from history, data provided in event
            switchSectionTo(e.state.section);
            window.app.currentPath = e.state.id;
            window.loadFiles({ insertHistory: false });
        }
    });

    // Search input — Enter key
    elements.searchInput.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') {
            // Cancel any pending debounce
            if (searchDebounceTimer) clearTimeout(searchDebounceTimer);
            const query = elements.searchInput.value.trim();

            // In shared view, filter locally
            if (app.isSharedView && window.sharedView) {
                window.sharedView.filterAndSortItems();
                return;
            }

            if (query) {
                window.performSearch(query);
            } else if (app.isSearchMode) {
                // If search is empty and we're in search mode, return to normal view
                app.isSearchMode = false;
                app.currentPath = '';
                ui.updateBreadcrumb('');
                window.loadFiles();
            }
        }
    });

    // Search input — Live search (debounced, after 3+ chars)
    elements.searchInput.addEventListener('input', () => {
        if (searchDebounceTimer) clearTimeout(searchDebounceTimer);
        const query = elements.searchInput.value.trim();

        if (query.length >= SEARCH_MIN_CHARS) {
            searchDebounceTimer = setTimeout(() => {
                window.performSearch(query);
            }, SEARCH_DEBOUNCE_MS);
        } else if (query.length === 0 && app.isSearchMode) {
            // User cleared the search input — return to normal view
            searchDebounceTimer = setTimeout(() => {
                app.isSearchMode = false;
                app.currentPath = '';
                ui.updateBreadcrumb('');
                window.loadFiles();
            }, SEARCH_DEBOUNCE_MS);
        }
    });

    // Search button
    document.getElementById('search-button').addEventListener('click', () => {
        if (searchDebounceTimer) clearTimeout(searchDebounceTimer);
        const query = elements.searchInput.value.trim();
        if (query) {
            window.performSearch(query);
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
    elements.navItems.forEach((item) => {
        item.addEventListener('click', () => {
            // Remove active class from all nav items
            elements.navItems.forEach((navItem) => {
                navItem.classList.remove('active');
            });

            // Add active class to clicked item
            item.classList.add('active');
            let _updateHistory = true;

            const itemI18nKey = item.querySelector('span').getAttribute('data-i18n');
            switch (itemI18nKey) {
                case 'nav.shared':
                    // Switch to shared view
                    switchToSharedSection();
                    break;

                case 'nav.favorites':
                    // Switch to favorites view
                    switchToFavoritesSection();
                    break;

                case 'nav.recent':
                    // Switch to recent files view
                    switchToRecentFilesSection();
                    break;

                case 'nav.photos':
                    switchToPhotosSection();
                    break;

                case 'nav.trash':
                    switchToTrashSection();
                    break;

                default:
                    // Use the proper switchToFilesView function which handles all UI restoration
                    window.switchToFilesSection();
                    // FIXME: because fileview handles it: need to converge code
                    _updateHistory = false;
            }

            document.title = `OxiCloud: ${window.i18n.t(itemI18nKey)}`;

            if (_updateHistory) {
                updateHistory(true);
            }
        });
    });

    // Load saved view preference
    const savedView = localStorage.getItem('oxicloud-view');
    if (savedView === 'list') {
        ui.switchToListView();
    } else {
        ui.switchToGridView();
    }

    // User menu
    window.setupUserMenu();

    // Global events to close context menus and deselect cards
    document.addEventListener('click', (e) => {
        const folderMenu = document.getElementById('folder-context-menu');
        const fileMenu = document.getElementById('file-context-menu');

        if (folderMenu && folderMenu.style.display === 'block' && !folderMenu.contains(e.target)) {
            ui.closeContextMenu();
        }

        if (fileMenu && fileMenu.style.display === 'block' && !fileMenu.contains(e.target)) {
            ui.closeFileContextMenu();
        }
    });
}

// Expose needed functions to global scope
window.setActionsBarMode = setActionsBarMode;

// Set up global selectFolder function for navigation
window.selectFolder = (id, name) => {
    app.breadcrumbPath.push({ id, name });
    app.currentPath = id;
    ui.updateBreadcrumb();
    window.loadFiles();
};

// View-switching actions moved to app/navigation.js

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
        // Use == null to allow 0 (unlimited) to pass through; only default to DEFAULT_QUOTA when null/undefined
        quotaBytes = userData.storage_quota_bytes == null ? DEFAULT_QUOTA : userData.storage_quota_bytes;

        // Calculate percentage (avoid division by zero)
        if (quotaBytes > 0) {
            usagePercentage = Math.min(Math.round((usedBytes / quotaBytes) * 100), 100);
        }
    }

    // Format the numbers for display
    const usedFormatted = formatFileSize(usedBytes);
    const quotaFormatted = formatQuotaSize(quotaBytes);

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
        if (window.i18n?.t) {
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

window.updateStorageUsageDisplay = updateStorageUsageDisplay;

// Initialize app when DOM is ready
window.initApp = initApp;
window.updateHistory = updateHistory;
window.deserializeHash = deserializeHash;
