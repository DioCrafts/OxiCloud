// @ts-check

// TODO move to features/files/fileOperations.js
/**
 * @typedef {Object} FolderInfo
 * @property {string} category
 * @property {number} created_at - timestamp
 * @property {string} icon_class
 * @property {string} icon_special_class
 * @property {string} id the uniq id of the folder
 * @property {boolean} is_root
 * @property {number} modified_at
 * @property {string} name
 * @property {string} owner_id
 * @property {string|null} parent_id the folder parent (null if is_root)
 * @property {string} path the full path
 */

/**
 * getFolder information
 * @param {string} id the id of the folder
 * @returns {Promise<FolderInfo>}
 */
async function getFolder( id) {

    /** @type {HeadersInit} */
    const headers = {
        'Cache-Control': 'no-cache, no-store, must-revalidate',
        'Pragma': 'no-cache'
    };

    /** @type {RequestInit} */
    const requestOptions = {
        headers,
        credentials: 'same-origin',
        cache: 'no-store'
    };
    
    let folderInformations = await fetch( `/api/folders/${id}`, requestOptions);
    if (folderInformations.ok) {
        return folderInformations.json()
    }
    else {
        console.warn(`Error fetching folder ${id}`);
        return Promise.reject(null);
    }
}

/**
 * Files view loading logic
 * 
 * @param {Object} options
 * @param {boolean} [options.insertHistory] add browser history (default true)
 * @param {boolean} [options.forceRefresh] force refresh of content
 */
async function loadFiles(options = { insertHistory: true}) {
    const app = window.app;
    const elements = window.appElements;

    try {
        console.log("Starting loadFiles() - loading files...", options);

        const forceRefresh = options.forceRefresh || false;

        if (window.isLoadingFiles) {
            console.log("A file load is already in progress, ignoring request");
            return;
        }

        window.isLoadingFiles = true;

        // This to avoid blinking page, a better solution would be to put loading on an overlay and remove timeout
        let loadingFiles = setTimeout( () => {
            // display loader after few delay (will be canceled if result take less time)
            elements.filesGrid.innerHTML = `
                <div class="files-loading-spinner">
                    <div class="spinner"></div>
                    <span>${window.i18n ? window.i18n.t('files.loading') : 'Loading files…'}</span>
                </div>
            `;
        }, 100);

        if (!app.userHomeFolderId) {
            await window.resolveHomeFolder();
        }

        const timestamp = new Date().getTime();

        /** 
         * Store the leaf (this is the current displayed folder)
         * @type {FolderInfo | null} 
         */
        let currentFolderInfo = null;

        // TODO: rebuild full breadcrumb POC only (to optimize, data may already be known)
        window.app.breadcrumbPath = [];

        /** @type {string | null} */
        let id=app.currentPath;

        while (id !== null) {
            console.log(`fetching folder information for folder ${id}`);
            try {
                let folderInfo = await getFolder(id);
            
                if (currentFolderInfo === null) {
                    currentFolderInfo = folderInfo;
                }

                // XXX do not enter root into bread crumb updateBreadcrumb() method always display it
                if(!folderInfo.is_root) {
                    window.app.breadcrumbPath.unshift( {id: folderInfo.id, name: folderInfo.name});
                }

                // iterate to parent folder
                id = folderInfo.parent_id;
            }
            catch( e) {
                console.log(`Error loading information from folder ${app.currentPath}, falling back to ${window.app.userHomeFolderId}`);
                // fallback of root
                window.uiNotifications.show(
                    'error: folder not found or permission denied', 
                    'the given folder is not available or you do not have sufficient rights' 
                );
                window.app.breadcrumbPath = [];
                id = window.app.userHomeFolderId;
                app.currentPath = id;
            }
        }
       
        // request a breadcrumb paint
        window.ui.updateBreadcrumb();

        if (currentFolderInfo !== null) {
            // TODO move this part to a common library ?
            if (options.insertHistory) {
                console.log(`adding history with #/files/folder/${app.currentPath}`)
                window.history.pushState({
                    section: window.app.currentSection,
                    id: currentFolderInfo.id,
                }, "", `#/files/folder/${app.currentPath}`);
            }
            else {
                console.log(`replace history with #/files/folder/${app.currentPath}`)
                window.history.replaceState({
                    section: window.app.currentSection,
                    id: currentFolderInfo.id,
                }, "", `#/files/folder/${app.currentPath}`);
            }
        }

        // update title
        document.title = `OxiCloud: ${currentFolderInfo.path}`;

        let url;

        if (!app.currentPath || app.currentPath === '') {
            if (app.userHomeFolderId) {
                url = `/api/folders/${app.userHomeFolderId}/listing?t=${timestamp}`;
                app.currentPath = app.userHomeFolderId;
                app.breadcrumbPath = [];
                window.ui.updateBreadcrumb();
                console.log(`Loading user folder: ${app.userHomeFolderName} (${app.userHomeFolderId})`);
            } else {
                url = `/api/folders?t=${timestamp}`;
                console.warn("Emergency fallback to root folder - this should not normally happen");
            }
        } else {
            url = `/api/folders/${app.currentPath}/listing?t=${timestamp}`;
            console.log(`Loading subfolder content: ${app.currentPath}`);
        }

        /** @type {HeadersInit} */
        const headers = {
            'Cache-Control': 'no-cache, no-store, must-revalidate',
            'Pragma': 'no-cache'
        };

        /** @type {RequestInit} */
        const requestOptions = {
            headers,
            credentials: 'same-origin',
            cache: 'no-store'
        };

        if (forceRefresh) {
            url += `&force_refresh=true`;
            // @ts-ignore
            requestOptions.headers['X-Force-Refresh'] = 'true';
            console.log('Forcing complete refresh ignoring cache');
        }

        console.log(`Loading listing from ${url}`);
        const response = await fetch(url, requestOptions);

        // not required anymore
        clearTimeout( loadingFiles);

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

        const listing = await response.json();

        if (window.multiSelect) window.multiSelect.clear();
        window.ui._items.clear();
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

        const selectAllCb = document.getElementById('select-all-checkbox');
        if (selectAllCb && window.multiSelect) {
            selectAllCb.addEventListener('change', () => window.multiSelect.toggleAll());
        }

        const folderList = Array.isArray(listing.folders) ? listing.folders : [];
        const fileList = Array.isArray(listing.files) ? listing.files : [];

        if (folderList.length === 0 && fileList.length === 0) {
            const emptyState = document.createElement('div');
            emptyState.className = 'empty-state';
            emptyState.innerHTML = `
                <i class="fas fa-folder-open empty-state-icon"></i>
                <p>${_t('files.no_files')}</p>
                <p>${_t('files.empty_hint')}</p>
            `;
            elements.filesGrid.appendChild(emptyState);
        } else {
            window.ui.renderFolders(folderList);
            window.ui.renderFiles(fileList);
        }

        console.log(`Loaded ${folderList.length} folders and ${fileList.length} files`);
    } catch (error) {
        console.error('Error loading folders:', error);
        window.ui.showNotification('Error', 'Could not load files and folders');
    } finally {
        window.isLoadingFiles = false;
    }
}

window.loadFiles = loadFiles;
