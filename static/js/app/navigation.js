/**
 * OxiCloud - View navigation actions
 * Extracted from main.js to keep navigation concerns isolated.
 */

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
	navItems.forEach(item => {
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
    'files': 'isFilesView',
    'shared': 'isSharedView',
    'recent': 'isRecentView',
    'favorites': 'isFavoritesView',
    'trash': 'isTrashView'
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
 */
function setCurrentSection(section) {
    // Set all view flags - true for active section, false for others
    Object.entries(VIEW_FLAGS).forEach(([key, flag]) => {
        window.app[flag] = (key === section);
    });

    window.app.currentSection = section;

    // Update nav item active classes by finding matching item from DOM
    window.appElements.navItems.forEach(item => {
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
}

function switchToSharedView() {
    setCurrentSection('shared');

    // Hide breadcrumb (only shown in Files view)
    const breadcrumb = document.querySelector('.breadcrumb');
    if (breadcrumb) breadcrumb.style.display = 'none';

    // Hide actions-bar for shared view
    window.setActionsBarMode('hidden');

    const filesGrid = document.getElementById('files-grid');
    const filesListView = document.getElementById('files-list-view');
    if (filesGrid) filesGrid.style.display = 'none';
    if (filesListView) filesListView.style.display = 'none';

    // Show shared view
    if (window.sharedView) {
        window.sharedView.init();
        window.sharedView.show();
    }
}

function switchToFilesView() {
    setCurrentSection('files');

    // Set actions bar mode
    window.setActionsBarMode('files', true);

    // Show breadcrumb (only in Files view)
    const breadcrumb = document.querySelector('.breadcrumb');
    if (breadcrumb) breadcrumb.style.display = '';

    const filesGrid = document.getElementById('files-grid');
    const filesListView = document.getElementById('files-list-view');
    if (filesGrid) filesGrid.style.display = window.app.currentView === 'grid' ? 'grid' : 'none';
    if (filesListView) filesListView.style.display = window.app.currentView === 'list' ? 'block' : 'none';

    // Reset to home folder and update breadcrumb
    window.app.currentPath = window.app.userHomeFolderId || '';
    window.app.breadcrumbPath = [];
    window.ui.updateBreadcrumb();

    window.loadFiles();
}

function switchToFavoritesView() {
    setCurrentSection('favorites');

    // Set actions bar mode
    window.setActionsBarMode('favorites');

    // Hide breadcrumb (only shown in Files view)
    const breadcrumb = document.querySelector('.breadcrumb');
    if (breadcrumb) breadcrumb.style.display = 'none';

    const filesGrid = document.getElementById('files-grid');
    const filesListView = document.getElementById('files-list-view');
    if (filesGrid) filesGrid.style.display = window.app.currentView === 'grid' ? 'grid' : 'none';
    if (filesListView) filesListView.style.display = window.app.currentView === 'list' ? 'block' : 'none';

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
    setCurrentSection('recent');

    // Set actions bar mode
    window.setActionsBarMode('recent');

    // Hide breadcrumb (only shown in Files view)
    const breadcrumb = document.querySelector('.breadcrumb');
    if (breadcrumb) breadcrumb.style.display = 'none';

    const filesGrid = document.getElementById('files-grid');
    const filesListView = document.getElementById('files-list-view');
    if (filesGrid) filesGrid.style.display = window.app.currentView === 'grid' ? 'grid' : 'none';
    if (filesListView) filesListView.style.display = window.app.currentView === 'list' ? 'block' : 'none';

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
