/**
 * OxiCloud - App state container
 * Centralized mutable state for app and cached DOM references.
 */

window.app = {
    currentView: 'grid',
    currentPath: '',
    currentFolder: null,
    contextMenuTargetFolder: null,
    contextMenuTargetFile: null,
    selectedTargetFolderId: '',
    moveDialogMode: 'file',
    isFilesView: true,
    isTrashView: false,
    isSharedView: false,
    isFavoritesView: false,
    isRecentView: false,
    currentSection: 'files',
    isSearchMode: false,
    shareDialogItem: null,
    shareDialogItemType: null,
    notificationShareUrl: null,
    userHomeFolderId: null,
    userHomeFolderName: null,
    breadcrumbPath: [] // Array of {id, name} tracking folder navigation hierarchy
};

window.appElements = {
};
