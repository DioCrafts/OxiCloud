/**
 * OxiCloud - App state container
 * Centralized mutable state for app and cached DOM references.
 */

export const app = {
    currentView: 'grid',
    currentPath: '',
    currentFolder: null,
    currentFolderInfo: null,
    contextMenuTargetFolder: null,
    contextMenuTargetFile: null,
    selectedTargetFolderId: '',
    moveDialogMode: 'file',

    currentSection: null, // will be defined on first call
    isSearchMode: false,
    shareDialogItem: null,
    shareDialogItemType: null,
    notificationShareUrl: null,
    userHomeFolderId: null,
    userHomeFolderName: null,
    breadcrumbPath: [], // Array of {id, name} tracking folder navigation hierarchy
    viewFile: null // current file in inline view
};

export const appElements = {};
