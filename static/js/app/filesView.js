/**
 * Files view loading logic
 */

async function loadFiles(options = {}) {
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

        elements.filesGrid.innerHTML = `
            <div class="files-loading-spinner">
                <div class="spinner"></div>
                <span>${window.i18n ? window.i18n.t('files.loading') : 'Loading files…'}</span>
            </div>
        `;

        if (!app.userHomeFolderId) {
            await window.resolveHomeFolder();
        }

        const timestamp = new Date().getTime();
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
            cache: 'no-store'
        };

        if (forceRefresh) {
            url += `&force_refresh=true`;
            requestOptions.headers['X-Force-Refresh'] = 'true';
            console.log('Forcing complete refresh ignoring cache');
        }

        console.log(`Loading listing from ${url}`);
        const response = await fetch(url, requestOptions);

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

        window.ui.renderFolders(folderList);
        window.ui.renderFiles(fileList);

        console.log(`Loaded ${folderList.length} folders and ${fileList.length} files`);
    } catch (error) {
        console.error('Error loading folders:', error);
        window.ui.showNotification('Error', 'Could not load files and folders');
    } finally {
        window.isLoadingFiles = false;
    }
}

window.loadFiles = loadFiles;
