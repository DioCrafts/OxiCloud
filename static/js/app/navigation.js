/**
 * OxiCloud - View navigation actions
 * Extracted from main.js to keep navigation concerns isolated.
 */

function switchToSharedView() {
    window.app.isTrashView = false;
    window.app.isSharedView = true;
    window.app.currentSection = 'shared';

    window.appElements.navItems.forEach(navItem => navItem.classList.remove('active'));

    const sharedNavItem = document.querySelector('.nav-item:nth-child(2)');
    if (sharedNavItem) {
        sharedNavItem.classList.add('active');
    }

    window.appElements.pageTitle.textContent = window.i18n ? window.i18n.t('nav.shared') : 'Shared';
    window.appElements.pageTitle.setAttribute('data-i18n', 'nav.shared');

    window.ui.updateBreadcrumb('');

    const breadcrumb = document.querySelector('.breadcrumb');
    if (breadcrumb) breadcrumb.style.display = 'none';

    if (window.appElements.actionsBar) {
        window.appElements.actionsBar.style.display = 'none';
    }

    const filesGrid = document.getElementById('files-grid');
    const filesListView = document.getElementById('files-list-view');
    if (filesGrid) filesGrid.style.display = 'none';
    if (filesListView) filesListView.style.display = 'none';

    if (window.sharedView) {
        window.sharedView.init();
        window.sharedView.show();
    }
}

function switchToFilesView() {
    window.app.isTrashView = false;
    window.app.isSharedView = false;
    window.app.isFavoritesView = false;
    window.app.isRecentView = false;
    window.app.currentSection = 'files';

    window.appElements.pageTitle.textContent = window.i18n ? window.i18n.t('nav.files') : 'Files';
    window.appElements.pageTitle.setAttribute('data-i18n', 'nav.files');

    const breadcrumb = document.querySelector('.breadcrumb');
    if (breadcrumb) breadcrumb.style.display = '';

    window.appElements.navItems.forEach(navItem => navItem.classList.remove('active'));

    const filesNavItem = document.querySelector('.nav-item:first-child');
    if (filesNavItem) {
        filesNavItem.classList.add('active');
    }

    window.setActionsBarMode('files');

    if (window.sharedView) {
        window.sharedView.hide();
    }

    const filesGrid = document.getElementById('files-grid');
    if (filesGrid) {
        filesGrid.style.display = window.app.currentView === 'grid' ? 'grid' : 'none';
    }

    const filesListView = document.getElementById('files-list-view');
    if (filesListView) {
        filesListView.style.display = window.app.currentView === 'list' ? 'block' : 'none';
    }

    if (window.app.userHomeFolderId) {
        window.app.currentPath = window.app.userHomeFolderId;
        window.ui.updateBreadcrumb(window.app.userHomeFolderName || 'Home');
    } else {
        window.app.currentPath = '';
    }
    window.loadFiles();
}

function switchToFavoritesView() {
    window.app.isTrashView = false;
    window.app.isSharedView = false;

    window.app.isFavoritesView = true;
    window.app.currentSection = 'favorites';

    window.appElements.navItems.forEach(navItem => navItem.classList.remove('active'));

    const favoritesNavItem = document.querySelector('.nav-item:nth-child(4)');
    if (favoritesNavItem) {
        favoritesNavItem.classList.add('active');
    }

    window.appElements.pageTitle.textContent = window.i18n ? window.i18n.t('nav.favorites') : 'Favorites';
    window.appElements.pageTitle.setAttribute('data-i18n', 'nav.favorites');

    window.ui.updateBreadcrumb('');

    if (window.sharedView) {
        window.sharedView.hide();
    }

    window.setActionsBarMode('favorites');

    const filesGrid = document.getElementById('files-grid');
    const filesListView = document.getElementById('files-list-view');

    if (filesGrid) {
        filesGrid.style.display = window.app.currentView === 'grid' ? 'grid' : 'none';
    }

    if (filesListView) {
        filesListView.style.display = window.app.currentView === 'list' ? 'block' : 'none';
    }

    if (window.favorites) {
        window.favorites.displayFavorites();
    } else {
        console.error('Favorites module not loaded or initialized');

        const filesGridError = document.getElementById('files-grid');
        if (filesGridError) {
            filesGridError.innerHTML = `
                <div class="empty-state">
                    <i class="fas fa-exclamation-circle" style="font-size: 48px; color: #f44336; margin-bottom: 16px;"></i>
                    <p>Error loading the favorites module</p>
                </div>
            `;
        }
    }
}

function switchToRecentFilesView() {
    window.app.isTrashView = false;
    window.app.isSharedView = false;
    window.app.isFavoritesView = false;

    window.app.isRecentView = true;
    window.app.currentSection = 'recent';

    window.appElements.navItems.forEach(navItem => navItem.classList.remove('active'));

    const recentNavItem = document.querySelector('.nav-item:nth-child(3)');
    if (recentNavItem) {
        recentNavItem.classList.add('active');
    }

    window.appElements.pageTitle.textContent = window.i18n ? window.i18n.t('nav.recent') : 'Recent';
    window.appElements.pageTitle.setAttribute('data-i18n', 'nav.recent');

    window.ui.updateBreadcrumb('');

    if (window.sharedView) {
        window.sharedView.hide();
    }

    window.setActionsBarMode('recent');

    const filesGrid = document.getElementById('files-grid');
    const filesListView = document.getElementById('files-list-view');

    if (filesGrid) {
        filesGrid.style.display = window.app.currentView === 'grid' ? 'grid' : 'none';
    }

    if (filesListView) {
        filesListView.style.display = window.app.currentView === 'list' ? 'block' : 'none';
    }

    if (window.recent) {
        window.recent.displayRecentFiles();
    } else {
        console.error('Recent files module not loaded or initialized');

        const filesGridError = document.getElementById('files-grid');
        if (filesGridError) {
            filesGridError.innerHTML = `
                <div class="empty-state">
                    <i class="fas fa-exclamation-circle" style="font-size: 48px; color: #f44336; margin-bottom: 16px;"></i>
                    <p>Error loading the recent files module</p>
                </div>
            `;
        }
    }
}

window.switchToFilesView = switchToFilesView;
window.switchToSharedView = switchToSharedView;
window.switchToFavoritesView = switchToFavoritesView;
window.switchToRecentFilesView = switchToRecentFilesView;
