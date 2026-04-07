/**
 * OxiCloud - View navigation actions
 * Extracted from main.js to keep navigation concerns isolated.
 */

/**
 * Sync the hidden class and inline display for the grid/list containers
 * based on the current view preference.
 */
function syncViewContainers() {
    const filesList = document.getElementById('files-list');
    const gridViewBtn = document.getElementById('grid-view-btn');
    const listViewBtn = document.getElementById('list-view-btn');

    const isGrid = window.app.currentView === 'grid';
    if (isGrid) {
        filesList.classList.remove('files-list-view');
        filesList.classList.add('files-grid-view');

        gridViewBtn?.classList.add('active');
        listViewBtn?.classList.remove('active');
    } else {
        filesList.classList.add('files-list-view');
        filesList.classList.remove('files-grid-view');

        gridViewBtn?.classList.remove('active');
        listViewBtn?.classList.add('active');
    }
}

/**
 * Hide file containers (used when switching to non-file views).
 * @param {boolean} show false to hide, true to show
 */
function toggleFileContainer(show) {
    const filesList = document.getElementById('files-list');
    filesList.classList.toggle('hidden', !show);
}

/**
 * Mobile sidebar toggle functionality
 */
function initSidebarToggle() {
    const sidebarToggle = document.getElementById('sidebar-toggle');
    const sidebar = document.getElementById('sidebar');
    const sidebarOverlay = document.getElementById('sidebar-overlay');

    if (!sidebarToggle || !sidebar || !sidebarOverlay) return;

    function openSidebar() {
        sidebar.classList.add('open');
        sidebarOverlay.classList.add('active');
        document.body.style.overflow = 'hidden';
    }

    function closeSidebar() {
        sidebar.classList.remove('open');
        sidebarOverlay.classList.remove('active');
        document.body.style.overflow = '';
    }

    function toggleSidebar() {
        if (sidebar.classList.contains('open')) {
            closeSidebar();
        } else {
            openSidebar();
        }
    }

    // Toggle button click
    sidebarToggle.addEventListener('click', toggleSidebar);

    // Close sidebar when clicking overlay
    sidebarOverlay.addEventListener('click', closeSidebar);

    // Close sidebar on escape key
    document.addEventListener('keydown', (e) => {
        if (e.key === 'Escape' && sidebar.classList.contains('open')) {
            closeSidebar();
        }
    });

    // Close sidebar when navigating (nav item click on mobile)
    const navItems = sidebar.querySelectorAll('.nav-item');
    navItems.forEach((item) => {
        item.addEventListener('click', () => {
            if (window.innerWidth <= 768) {
                closeSidebar();
            }
        });
    });

    // Expose functions globally
    window.sidebarToggle = {
        open: openSidebar,
        close: closeSidebar,
        toggle: toggleSidebar
    };
}

// Initialize sidebar toggle when DOM is ready
document.addEventListener('DOMContentLoaded', initSidebarToggle);

// Mapping of section names to their corresponding view flags
const VIEW_FLAGS = {
    files: 'isFilesView',
    shared: 'isSharedView',
    recent: 'isRecentView',
    favorites: 'isFavoritesView',
    trash: 'isTrashView',
    photos: 'isPhotosView'
};

/**
 * Derive section name from nav item's data-i18n attribute.
 * @param {HTMLElement} navItem - The nav item element
 * @returns {string|null} - Section name or null if not found
 */
function getSectionFromNavItem(navItem) {
    const i18nKey = navItem.querySelector('span[data-i18n]')?.getAttribute('data-i18n');
    return i18nKey ? i18nKey.replace('nav.', '') : null;
}

/**
 * Set the current active section, updating all view flags and nav UI.
 * @param {string} section - The section to activate ('files', 'shared', 'recent', 'favorites', 'trash')
 * @returns {boolean} true if the section changed
 */
function setCurrentSection(section) {
    if (window.app.currentSection == section) return false;

    // Set all view flags - true for active section, false for others
    Object.entries(VIEW_FLAGS).forEach(([key, flag]) => {
        window.app[flag] = key === section;
    });

    window.app.currentSection = section;

    // Update nav item active classes by finding matching item from DOM
    window.appElements.navItems.forEach((item) => {
        const itemSection = getSectionFromNavItem(item);
        item.classList.toggle('active', itemSection === section);
    });

    // Update page title
    const titleKey = `nav.${section}`;
    const defaultTitle = section.charAt(0).toUpperCase() + section.slice(1);
    window.appElements.pageTitle.textContent = window.i18n ? window.i18n.t(titleKey) : defaultTitle;
    window.appElements.pageTitle.setAttribute('data-i18n', titleKey);

    // Hide sharedView when switching to any other section
    if (section !== 'shared' && window.sharedView) {
        window.sharedView.hide();
    }

    // Hide photosView when switching to any other section
    if (section !== 'photos' && window.photosView) {
        window.photosView.hide();
    }

    return true;
}

function switchToSharedSection() {
    if (!setCurrentSection('shared')) return;

    // Hide breadcrumb (only shown in Files view)
    const breadcrumb = document.querySelector('.breadcrumb');
    breadcrumb?.classList.add('hidden');

    // Hide actions-bar for shared view
    window.setActionsBarMode('hidden');

    toggleFileContainer(false);

    //reset files view + remove any error
    window.ui.resetFilesList();

    // Show shared view
    if (window.sharedView) {
        window.sharedView.init();
        window.sharedView.show();
    }
    if (window.multiSelect) window.multiSelect.clear();
}

function switchToFilesSection() {
    if (!setCurrentSection('files')) return;

    // Set actions bar mode
    window.setActionsBarMode('files', true);

    // Show breadcrumb (only in Files view)
    const breadcrumb = document.querySelector('.breadcrumb');
    breadcrumb?.classList.remove('hidden');

    // show files container
    toggleFileContainer(true);

    // ensure correct view
    syncViewContainers();

    //reset files view + remove any error
    window.ui.resetFilesList();

    // Reset to home folder and update breadcrumb
    window.app.currentPath = window.app.userHomeFolderId || '';
    window.app.breadcrumbPath = [];
    window.ui.updateBreadcrumb();
    if (window.multiSelect) window.multiSelect.clear();

    window.loadFiles();
}

function switchToFavoritesSection() {
    if (!setCurrentSection('favorites')) return;

    // Set actions bar mode
    window.setActionsBarMode('favorites');

    // Hide breadcrumb (only shown in Files view)
    const breadcrumb = document.querySelector('.breadcrumb');
    breadcrumb?.classList.add('hidden');

    // show files container
    toggleFileContainer(true);

    // ensure correct view
    syncViewContainers();

    //reset files view + remove any error
    window.ui.resetFilesList();

    if (window.favorites) {
        window.favorites.displayFavorites();
    } else {
        console.error('Favorites module not loaded or initialized');
        window.ui.showError(`
                <i class="fas fa-exclamation-circle empty-state-icon error"></i>
                <p>Error loading the favorites module</p>
            `);
    }

    if (window.multiSelect) window.multiSelect.clear();
}

function switchToRecentFilesSection() {
    if (!setCurrentSection('recent')) return;

    // Set actions bar mode
    window.setActionsBarMode('recent');

    // Hide breadcrumb (only shown in Files view)
    const breadcrumb = document.querySelector('.breadcrumb');
    breadcrumb?.classList.add('hidden');

    // show files container
    toggleFileContainer(true);

    // ensure correct view
    syncViewContainers();

    //reset files view + remove any error
    window.ui.resetFilesList();

    if (window.recent) {
        window.recent.displayRecentFiles();
    } else {
        console.error('Recent files module not loaded or initialized');
        window.ui.showError(`
                <i class="fas fa-exclamation-circle empty-state-icon error"></i>
                <p>Error loading the recent module</p>
            `);
    }
    if (window.multiSelect) window.multiSelect.clear();
}

function switchToPhotosSection() {
    if (!setCurrentSection('photos')) return;

    // Hide breadcrumb
    const breadcrumb = document.querySelector('.breadcrumb');
    breadcrumb?.classList.add('hidden');

    // Hide file containers
    toggleFileContainer(false);

    // Hide actions-bar (photos has its own upload via selection bar)
    window.setActionsBarMode('hidden');

    //reset files view + remove any error
    window.ui.resetFilesList();

    // Show photos view
    if (window.photosView) {
        window.photosView.show();
    }
    if (window.multiSelect) window.multiSelect.clear();
}

function switchToTrashSection() {
    setCurrentSection('trash');

    // Hide breadcrumb (only shown in Files view)
    const breadcrumb = document.querySelector('.breadcrumb');
    breadcrumb?.classList.add('hidden');

    // Show files containers (to be filled with trash)
    const filesList = document.getElementById('files-list');
    filesList.classList.remove('hidden');

    setActionsBarMode('trash');

    //reset files view + remove any error
    window.ui.resetFilesList();

    //ensure buttons match the current view
    syncViewContainers();

    // Load trash items
    window.loadTrashItems();

    if (window.multiSelect) window.multiSelect.clear();
}

window.switchToFilesSection = switchToFilesSection;
window.switchToSharedSection = switchToSharedSection;
window.switchToFavoritesSection = switchToFavoritesSection;
window.switchToRecentFilesSection = switchToRecentFilesSection;
window.switchToPhotosSection = switchToPhotosSection;
window.switchToTrashSection = switchToTrashSection;
window.syncViewContainers = syncViewContainers;
