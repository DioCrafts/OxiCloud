/**
 * OxiCloud - App state container
 * Centralized mutable state for app and cached DOM references.
 */

/** @import {FileItem, FolderItem, LightItem} from '../core/types.js' */

export const app = {
    currentView: 'grid',

    /** @type {string | null} */
    currentPath: '',

    /** @type {string | null} */
    currentFolder: null,

    /** @type {FolderItem | null} */
    currentFolderInfo: null,

    /** @type {FolderItem | null} */
    contextMenuTargetFolder: null,

    /** @type {FileItem | null} */
    contextMenuTargetFile: null,
    selectedTargetFolderId: '',
    moveDialogMode: 'file',

    /** @type {string | null} */
    moveDialogItemId: null,

    /** @type {'file' | 'folder' | null} */
    moveDialogItemMode: null,

    /** @type {string | null} */
    moveDialogCurrentFolderId: null,

    /** @type {Array<{id: string, name: string}>} */
    moveDialogBreadcrumb: [],

    /** @type {FileItem[] | null} */
    playlistDialogFiles: null,

    /** @type {String | null} */
    currentSection: null, // will be defined on first call
    isSearchMode: false,

    /** @type {FileItem | FolderItem | null} */
    shareDialogItem: null,

    /** @type {'file' | 'folder' | null} */
    shareDialogItemType: null,

    /** @type {String | null} */
    notificationShareUrl: null,

    /** @type {string | null} */
    userHomeFolderId: null,

    /** @type {string | null} */
    userHomeFolderName: null,

    /** @type {Array<{id: string, name: string}>} */
    breadcrumbPath: [], // Array of {id, name} tracking folder navigation hierarchy

    /** @type {String | null} */
    viewFile: null, // current file in inline view

    /** @type {LightItem[] | null} */
    batchMoveItems: null
};

export const appElements = {
    /** @type {HTMLElement | null} */
    uploadBtn: null,
    /** @type {HTMLElement | null}  */
    dropzone: null,
    /** @type {HTMLInputElement | null}  */
    fileInput: null,
    /** @type {HTMLElement | null}  */
    filesList: null,
    /** @type {HTMLElement | null}  */
    newFolderBtn: null,
    /** @type {HTMLElement | null}  */
    gridViewBtn: null,
    /** @type {HTMLElement | null}  */
    listViewBtn: null,
    /** @type {HTMLElement | null}  */
    breadcrumb: null,
    /** @type {HTMLElement | null}  */
    pageTitle: null,
    /** @type {HTMLElement | null}  */
    actionsBar: null,
    /** @type {NodeListOf<HTMLElement> | null}  */
    navItems: null,
    /** @type {HTMLInputElement | null}  */
    searchInput: null
};
