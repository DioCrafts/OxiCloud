/**
 * OxiCloud - File Operations Module
 * This file handles file and folder operations (create, move, delete, rename, upload)
 */

/**
 * Get authorization headers for API requests
 * @returns {Object} Headers object with Authorization bearer token
 */
function getAuthHeaders() {
    const token = localStorage.getItem('oxicloud_token');
    const headers = {};
    if (token) {
        headers['Authorization'] = `Bearer ${token}`;
    }
    return headers;
}

// File Operations Module
const fileOps = {

    // ========================================================================
    // Upload progress — notification bell integration
    // ========================================================================
    _currentBatchId: null,
    _isUploading: false,  // Guard against concurrent upload calls

    /** Start a new upload batch in the notification bell */
    _initUploadToast(totalFiles, folderName) {
        this._currentBatchId = window.notifications
            ? window.notifications.addUploadBatch(totalFiles, folderName)
            : null;
    },

    /** Finalise the batch in the notification bell */
    _finishUploadToast(successCount, totalFiles) {
        if (window.notifications && this._currentBatchId) {
            window.notifications.finishBatch(this._currentBatchId, successCount, totalFiles);
        }
    },

    /**
     * Some drag-and-drop sources can inject directory placeholders into
     * DataTransfer.files. Browsers fail those with net::ERR_ACCESS_DENIED
     * when trying to send them as normal files.
     */
    _canReadFileBlob(file) {
        return new Promise((resolve) => {
            try {
                const reader = new FileReader();
                reader.onload = () => resolve(true);
                reader.onerror = () => resolve(false);
                reader.readAsArrayBuffer(file.slice(0, 1));
            } catch (_) {
                resolve(false);
            }
        });
    },

    /**
     * Upload a single file via XMLHttpRequest with progress events.
     * Progress is reported to the notification bell via batchId + fileName.
     * Returns a promise that resolves with { ok, data?, errorMsg?, isQuotaError? }.
     */
    _uploadFileXHR(formData, batchId, fileName, timeoutMs = 120000) {
        return new Promise((resolve) => {
            const xhr = new XMLHttpRequest();
            const notif = window.notifications;
            xhr.timeout = timeoutMs;
            const hardDeadlineMs = Math.max(timeoutMs * 2, 180000);
            let lastProgressPctSent = -1;

            let isSettled = false;
            let stallTimer = null;
            let hardTimer = null;

            const safeUpdateFile = (pct, status) => {
                if (!notif || !batchId) return;
                try {
                    notif.updateFile(batchId, fileName, pct, status);
                } catch (e) {
                    console.warn('Notification update failed for upload row:', fileName, e);
                }
            };

            const finalize = (result) => {
                if (isSettled) return;
                isSettled = true;
                if (stallTimer) {
                    clearTimeout(stallTimer);
                    stallTimer = null;
                }
                if (hardTimer) {
                    clearTimeout(hardTimer);
                    hardTimer = null;
                }
                resolve(result);
            };

            const resetStallTimer = () => {
                if (stallTimer) clearTimeout(stallTimer);
                stallTimer = setTimeout(() => {
                    try { xhr.abort(); } catch (_) {}
                    safeUpdateFile(0, 'error');
                    finalize({
                        ok: false,
                        isTimeout: true,
                        errorMsg: `Upload stalled for ${Math.round(timeoutMs / 1000)}s`
                    });
                }, timeoutMs);
            };

            resetStallTimer();
            hardTimer = setTimeout(() => {
                try { xhr.abort(); } catch (_) {}
                safeUpdateFile(0, 'error');
                finalize({
                    ok: false,
                    isTimeout: true,
                    errorMsg: `Upload hard timeout after ${Math.round(hardDeadlineMs / 1000)}s`
                });
            }, hardDeadlineMs);

            xhr.upload.addEventListener('progress', (e) => {
                resetStallTimer();
                if (e.lengthComputable) {
                    const pct = Math.round((e.loaded / e.total) * 100);
                    // Throttle UI updates from very chatty progress events
                    if (pct === 100 || pct - lastProgressPctSent >= 10) {
                        lastProgressPctSent = pct;
                        safeUpdateFile(pct, 'uploading');
                    }
                }
            });

            xhr.addEventListener('readystatechange', () => {
                // Keep watchdog alive while request is actively moving through states
                if (xhr.readyState > 1 && xhr.readyState < 4) {
                    resetStallTimer();
                }
            });

            xhr.addEventListener('load', () => {
                if (xhr.status >= 200 && xhr.status < 300) {
                    safeUpdateFile(100, 'done');
                    let data = null;
                    try { data = JSON.parse(xhr.responseText); } catch (_) {}
                    finalize({ ok: true, data });
                } else {
                    safeUpdateFile(0, 'error');
                    // Parse error body for quota-exceeded or other messages
                    let errorMsg = null;
                    let isQuotaError = false;
                    try {
                        const errBody = JSON.parse(xhr.responseText);
                        errorMsg = errBody.error || null;
                        isQuotaError = errBody.error_type === 'QuotaExceeded' || xhr.status === 507;
                    } catch (_) {}
                    finalize({ ok: false, errorMsg, isQuotaError });
                }
            });

            xhr.addEventListener('error', () => {
                safeUpdateFile(0, 'error');
                finalize({ ok: false });
            });

            xhr.addEventListener('abort', () => {
                safeUpdateFile(0, 'error');
                finalize({ ok: false, isTimeout: true, errorMsg: `Upload aborted/stalled: ${fileName}` });
            });

            xhr.addEventListener('timeout', () => {
                safeUpdateFile(0, 'error');
                finalize({ ok: false, isTimeout: true, errorMsg: `Timeout after ${Math.round(timeoutMs / 1000)}s` });
            });

            xhr.open('POST', '/api/files/upload');

            // Set auth header
            const token = localStorage.getItem('oxicloud_token');
            if (token) xhr.setRequestHeader('Authorization', `Bearer ${token}`);
            xhr.setRequestHeader('Cache-Control', 'no-cache, no-store, must-revalidate');

            try {
                xhr.send(formData);
            } catch (e) {
                safeUpdateFile(0, 'error');
                finalize({
                    ok: false,
                    errorMsg: `Client send() failed: ${e?.message || 'unknown error'}`
                });
            }
        });
    },

    /**
     * Upload a single file via fetch + AbortController.
     * Used by folder uploads to avoid browser XHR edge-cases with dragged entries.
     * Returns { ok, data?, errorMsg?, isQuotaError?, isTimeout? }.
     */
    async _uploadFileFetch(formData, timeoutMs = 60000) {
        const controller = new AbortController();
        const timeoutId = setTimeout(() => controller.abort(), timeoutMs);

        try {
            const response = await fetch('/api/files/upload', {
                method: 'POST',
                headers: {
                    ...getAuthHeaders(),
                    'Cache-Control': 'no-cache, no-store, must-revalidate'
                },
                body: formData,
                signal: controller.signal,
                cache: 'no-store'
            });

            // Read body as text first (always consume the response fully)
            let rawText = '';
            try { rawText = await response.text(); } catch (_) {}

            let body = null;
            try { body = JSON.parse(rawText); } catch (_) {}

            if (response.ok) {
                return { ok: true, data: body };
            }

            const errorMsg = body && typeof body === 'object'
                ? (body.error || null)
                : (rawText || null);
            const isQuotaError = (body && typeof body === 'object' && body.error_type === 'QuotaExceeded') || response.status === 507;
            return { ok: false, errorMsg, isQuotaError };
        } catch (e) {
            const isTimeout = e?.name === 'AbortError';
            return {
                ok: false,
                isTimeout,
                errorMsg: isTimeout
                    ? `Timeout after ${Math.round(timeoutMs / 1000)}s`
                    : `Fetch upload failed: ${e?.message || 'network error'}`
            };
        } finally {
            clearTimeout(timeoutId);
        }
    },

    // ========================================================================
    // Upload files (via button or drag-and-drop)
    // ========================================================================

    /**
     * Upload files to the server with real-time progress indication
     * @param {FileList} files - Files to upload
     */
    async uploadFiles(files) {
        const originalFiles = Array.from(files || []);
        if (originalFiles.length === 0) return;

        // Guard: prevent concurrent upload calls (e.g. double drop events)
        if (this._isUploading) {
            console.warn('Upload already in progress, ignoring duplicate call');
            return;
        }
        this._isUploading = true;

        try {
            // Legacy progress bar (inside dropzone) — keep working for drag-drop
            const progressBar = document.querySelector('.progress-fill');
            const uploadProgressDiv = document.querySelector('.upload-progress');
            if (uploadProgressDiv) { uploadProgressDiv.style.display = 'block'; }
            if (progressBar) { progressBar.style.width = '0%'; }

            // Filter out unreadable entries (typically dropped folders/placeholders)
            const readableFiles = [];
            const skippedEntries = [];
            for (const f of originalFiles) {
                // eslint-disable-next-line no-await-in-loop
                const readable = await this._canReadFileBlob(f);
                if (readable) readableFiles.push(f);
                else skippedEntries.push(f.name || 'Unnamed entry');
            }

            const totalFiles = readableFiles.length;

            if (skippedEntries.length > 0 && window.notifications) {
                const locale = window.i18n?.getCurrentLocale?.() || 'en';
                const title = locale.startsWith('es') ? 'Entradas omitidas' : 'Entries skipped';
                const text = locale.startsWith('es')
                    ? `Se omitieron ${skippedEntries.length} carpeta(s)/entrada(s) no legibles. Usa "Subir carpeta".`
                    : `${skippedEntries.length} unreadable folder/entry items were skipped. Use "Upload folder".`;
                window.notifications.addNotification({
                    icon: 'fa-folder-open',
                    iconClass: 'upload',
                    title,
                    text
                });
            }

            if (totalFiles === 0) {
                if (uploadProgressDiv) uploadProgressDiv.style.display = 'none';
                this._isUploading = false;
                return;
            }

            // Show upload notification (only for actual readable files)
            this._initUploadToast(totalFiles);
            const batchId = this._currentBatchId;

            let uploadedCount = 0;
            let successCount = 0;

            for (let i = 0; i < totalFiles; i++) {
                const file = readableFiles[i];

                const formData = new FormData();

                const targetFolderId = window.app.currentPath || window.app.userHomeFolderId;
                if (targetFolderId) formData.append('folder_id', targetFolderId);
                formData.append('file', file);

                console.log(`Uploading file to folder: ${targetFolderId || 'root'}`, {
                    file: file.name, size: file.size
                });

                const result = await this._uploadFileXHR(formData, batchId, file.name);

                uploadedCount++;

                // Legacy dropzone bar
                if (progressBar) {
                    progressBar.style.width = ((uploadedCount / totalFiles) * 100) + '%';
                }
                // Notify bell of per-file completion
                if (window.notifications && batchId) {
                    try {
                        window.notifications.fileCompleted(batchId, result.ok);
                    } catch (e) {
                        console.warn('Batch progress update failed:', e);
                    }
                }

                if (result.ok) {
                    successCount++;
                    console.log(`Successfully uploaded ${file.name}`, result.data);
                } else {
                    console.error(`Upload error for ${file.name}`);
                    if (result.isTimeout && window.notifications) {
                        window.notifications.addNotification({
                            icon: 'fa-clock',
                            iconClass: 'error',
                            title: file.name,
                            text: result.errorMsg || 'Upload timeout'
                        });
                    }
                    if (result.isQuotaError) {
                        const msg = result.errorMsg || window.i18n?.t('storage_quota_exceeded') || 'Storage quota exceeded';
                        if (window.notifications) {
                            window.notifications.addNotification({
                                icon: 'fa-exclamation-triangle',
                                iconClass: 'error',
                                title: file.name,
                                text: msg
                            });
                        }
                        break;
                    }
                }
            }

            // All done
            this._finishUploadToast(successCount, totalFiles);

            // Refresh storage usage display
            if (typeof window.refreshUserData === 'function') {
                try { await window.refreshUserData(); } catch (_) {}
            }

            try {
                await window.loadFiles({ forceRefresh: true });
            } catch (reloadError) {
                console.error('Error reloading files:', reloadError);
            }

            const dropzone = document.getElementById('dropzone');
            if (dropzone) dropzone.style.display = 'none';
            if (uploadProgressDiv) uploadProgressDiv.style.display = 'none';
        } finally {
            this._isUploading = false;
        }
    },

    /**
     * Upload folder files maintaining directory structure
     * Creates subfolders as needed, then uploads files into them
     * @param {FileList} files - Files from folder input (with webkitRelativePath)
     */
    async uploadFolderFiles(files) {
        const entries = Array.from(files || []).map((file) => ({
            file,
            relativePath: file.webkitRelativePath || file.name
        }));
        await this.uploadFolderEntries(entries);
    },

    /**
     * Upload folder-like entries preserving relative paths.
     * @param {Array<{file: File, relativePath: string}>} entries
     */
    async uploadFolderEntries(entries) {
        const rawEntries = Array.isArray(entries) ? entries : [];
        if (rawEntries.length === 0) return;

        // Guard: prevent concurrent upload calls
        if (this._isUploading) {
            console.warn('Upload already in progress, ignoring duplicate call');
            return;
        }
        this._isUploading = true;

        const progressBar = document.querySelector('.progress-fill');
        const uploadProgressDiv = document.querySelector('.upload-progress');
        if (uploadProgressDiv) { uploadProgressDiv.style.display = 'block'; }
        if (progressBar) { progressBar.style.width = '0%'; }

        try {
            // Filter unreadable entries
            const validEntries = [];
            for (const e of rawEntries) {
                // eslint-disable-next-line no-await-in-loop
                const readable = await this._canReadFileBlob(e.file);
                if (readable) validEntries.push(e);
                else console.warn(`Skipping unreadable folder entry: ${e.relativePath || e.file?.name}`);
            }

            const totalFiles = validEntries.length;
            if (totalFiles === 0) {
                if (uploadProgressDiv) uploadProgressDiv.style.display = 'none';
                return;
            }

            const currentFolderId = window.app.currentPath || window.app.userHomeFolderId;

            // Build folder structure from relative paths
            const folderMap = new Map();
            folderMap.set('', currentFolderId);

            const folderPaths = new Set();
            for (const entry of validEntries) {
                const rel = entry.relativePath || entry.file.name;
                const parts = rel.split('/');
                for (let i = 1; i < parts.length; i++) {
                    const path = parts.slice(0, i).join('/');
                    folderPaths.add(path);
                }
            }

            const sortedPaths = [...folderPaths].sort((a, b) =>
                a.split('/').length - b.split('/').length
            );

            // Create folders first
            for (const folderPath of sortedPaths) {
                const parts = folderPath.split('/');
                const folderName = parts[parts.length - 1];
                const parentPath = parts.slice(0, -1).join('/');
                const parentId = folderMap.get(parentPath) || currentFolderId;

                try {
                    const response = await fetch('/api/folders', {
                        method: 'POST',
                        headers: {
                            ...getAuthHeaders(),
                            'Content-Type': 'application/json',
                            'Cache-Control': 'no-cache, no-store, must-revalidate'
                        },
                        body: JSON.stringify({
                            name: folderName,
                            parent_id: parentId
                        })
                    });

                    if (response.ok) {
                        const folder = await response.json();
                        folderMap.set(folderPath, folder.id);
                        console.log(`Created folder: ${folderPath} -> ${folder.id}`);
                    } else {
                        console.error(`Error creating folder ${folderPath}:`, await response.text());
                    }
                } catch (error) {
                    console.error(`Network error creating folder ${folderPath}:`, error);
                }
            }

            // Detect root folder(s) from entry paths
            const rootFolderNames = [...new Set(validEntries.map((entry) => {
                const rel = entry.relativePath || entry.file.name;
                return rel.split('/')[0] || '';
            }).filter(Boolean))];
            const locale = window.i18n?.getCurrentLocale?.() || 'en';
            const rootFolderLabel = rootFolderNames.length <= 1
                ? (rootFolderNames[0] || '')
                : (locale.startsWith('es')
                    ? `${rootFolderNames.length} carpetas`
                    : `${rootFolderNames.length} folders`);

            // Upload files — pass folder name for folder-level progress display
            this._initUploadToast(totalFiles, rootFolderLabel);
            const batchId = this._currentBatchId;

            let uploadedCount = 0;
            let successCount = 0;
            let quotaStop = false;

            // ── Concurrent upload with limited parallelism ──────────
            // FIFOs are pre-caught by the 0-byte arrayBuffer guard,
            // so all files reaching fetch() are regular. Keep-alive
            // reuses TCP connections across workers for speed.
            const CONCURRENCY = 10;
            const TIMEOUT_MS = 10000;       // 10s for normal files
            const TIMEOUT_MS_ZERO = 3000;   // 3s for 0-byte files

            const uploadOneFile = async (idx) => {
                if (quotaStop) return;
                const entry = validEntries[idx];
                const file = entry.file;
                const rel = entry.relativePath || file.name;

                let result = { ok: false, errorMsg: 'Unknown client error' };
                try {
                    const parts = rel.split('/');
                    const parentPath = parts.slice(0, -1).join('/');
                    const targetFolderId = folderMap.get(parentPath) || currentFolderId;

                    // ── FIFO/pipe guard (0-byte files only) ──
                    // Named pipes (runit supervise/control) report size=0
                    // but block on open(). Pre-read only 0-byte files into
                    // memory; files with size>0 are always regular files and
                    // go straight to FormData (zero extra memory copy).
                    let uploadFile = file;          // default: use original File
                    if (file.size === 0) {
                        try {
                            const buf = await Promise.race([
                                file.arrayBuffer(),
                                new Promise((_, rej) =>
                                    setTimeout(() => rej(new Error('read-timeout')), 2000))
                            ]);
                            uploadFile = new Blob([buf], {
                                type: file.type || 'application/octet-stream'
                            });
                        } catch {
                            console.warn(`[SKIP] #${idx} ${rel} — cannot read 0-byte file (FIFO/pipe?), skipping`);
                            uploadedCount++;
                            successCount++;
                            if (window.notifications && batchId) {
                                try { window.notifications.fileCompleted(batchId, true); } catch (_) {}
                            }
                            return;
                        }
                    }

                    const formData = new FormData();
                    formData.append('folder_id', targetFolderId);
                    formData.append('file', uploadFile, file.name);

                    const thisTimeout = file.size === 0 ? TIMEOUT_MS_ZERO : TIMEOUT_MS;
                    console.log(`[UPLOAD START] #${idx} ${rel} (${file.size} bytes, timeout=${thisTimeout}ms)`);

                    result = await this._uploadFileFetch(formData, thisTimeout);

                    console.log(`[UPLOAD END]   #${idx} ${rel} ok=${result.ok}${result.errorMsg ? ' err=' + result.errorMsg : ''}`);
                } catch (e) {
                    result = {
                        ok: false,
                        errorMsg: `Client exception: ${e?.message || 'unknown'}`
                    };
                    console.error(`[UPLOAD EXCEPTION] #${idx} ${rel}:`, e);
                }

                uploadedCount++;

                if (window.notifications && batchId) {
                    try { window.notifications.fileCompleted(batchId, result.ok); } catch (_) {}
                }
                if (progressBar && uploadedCount % 10 === 0) {
                    progressBar.style.width = ((uploadedCount / totalFiles) * 100) + '%';
                }
                if (uploadedCount % 50 === 0 || uploadedCount === totalFiles) {
                    console.log(`Progress: ${uploadedCount}/${totalFiles} (${successCount} ok)`);
                }

                if (result.ok) {
                    successCount++;
                } else if (result.isQuotaError) {
                    quotaStop = true;
                    if (window.notifications) {
                        window.notifications.addNotification({
                            icon: 'fa-exclamation-triangle',
                            iconClass: 'error',
                            title: file.name,
                            text: result.errorMsg || 'Storage quota exceeded'
                        });
                    }
                }
            };

            // Pool-based concurrency: always keep CONCURRENCY tasks in flight
            let nextIdx = 0;
            const runNext = async () => {
                while (nextIdx < totalFiles && !quotaStop) {
                    const idx = nextIdx++;
                    await uploadOneFile(idx);
                }
            };

            const workers = [];
            for (let w = 0; w < Math.min(CONCURRENCY, totalFiles); w++) {
                workers.push(runNext());
            }
            await Promise.all(workers);

            this._finishUploadToast(successCount, totalFiles);

            if (typeof window.refreshUserData === 'function') {
                try { await window.refreshUserData(); } catch (_) {}
            }

            try {
                await window.loadFiles({ forceRefresh: true });
            } catch (reloadError) {
                console.error('Error reloading files:', reloadError);
            }

            const dropzone = document.getElementById('dropzone');
            if (dropzone) dropzone.style.display = 'none';
            if (uploadProgressDiv) uploadProgressDiv.style.display = 'none';
        } finally {
            this._isUploading = false;
        }
    },

    /**
     * Create a new folder
     * @param {string} name - Folder name
     */
    async createFolder(name) {
        try {
            console.log('Creating folder with name:', name);
            
            // Send the actual request to the backend to create the folder
            const response = await fetch('/api/folders', {
                method: 'POST',
                headers: {
                    ...getAuthHeaders(),
                    'Content-Type': 'application/json',
                    'Cache-Control': 'no-cache, no-store, must-revalidate'
                },
                body: JSON.stringify({
                    name: name,
                    parent_id: window.app.currentPath || window.app.userHomeFolderId || null
                })
            });

            if (response.ok) {
                // Get the created folder from the backend
                const folder = await response.json();
                console.log('Folder created successfully:', folder);
                
                // Optimistic UI: add folder card directly from server response
                // — no reload needed since the backend already confirmed creation.
                window.ui.addFolderToView(folder);
                
                window.ui.showNotification('Folder created', `"${name}" created successfully`);
            } else {
                const errorData = await response.text();
                console.error('Create folder error:', errorData);
                window.ui.showNotification('Error', 'Error creating the folder');
            }
        } catch (error) {
            console.error('Error creating folder:', error);
            window.ui.showNotification('Error', 'Error creating the folder');
        }
    },

    /**
     * Move a file to another folder
     * @param {string} fileId - File ID
     * @param {string} targetFolderId - Target folder ID
     * @returns {Promise<boolean>} - Success status
     */
    async moveFile(fileId, targetFolderId) {
        try {
            const response = await fetch(`/api/files/${fileId}/move`, {
                method: 'PUT',
                headers: {
                    ...getAuthHeaders(),
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    folder_id: targetFolderId === "" ? null : targetFolderId
                })
            });

            if (response.ok) {
                // Reload files after moving
                await window.loadFiles();
                window.ui.showNotification('File moved', 'File moved successfully');
                return true;
            } else {
                let errorMessage = 'Unknown error';
                try {
                    const errorData = await response.json();
                    errorMessage = errorData.error || 'Unknown error';
                } catch (e) {
                    errorMessage = 'Error processing server response';
                }
                window.ui.showNotification('Error', `Error moving the file: ${errorMessage}`);
                return false;
            }
        } catch (error) {
            console.error('Error moving file:', error);
            window.ui.showNotification('Error', 'Error moving the file');
            return false;
        }
    },

    /**
     * Move a folder to another folder
     * @param {string} folderId - Folder ID
     * @param {string} targetFolderId - Target folder ID
     * @returns {Promise<boolean>} - Success status
     */
    async moveFolder(folderId, targetFolderId) {
        try {
            const response = await fetch(`/api/folders/${folderId}/move`, {
                method: 'PUT',
                headers: {
                    ...getAuthHeaders(),
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    parent_id: targetFolderId === "" ? null : targetFolderId
                })
            });

            if (response.ok) {
                // Reload files after moving
                await window.loadFiles();
                window.ui.showNotification('Folder moved', 'Folder moved successfully');
                return true;
            } else {
                let errorMessage = 'Unknown error';
                try {
                    const errorData = await response.json();
                    errorMessage = errorData.error || 'Unknown error';
                } catch (e) {
                    errorMessage = 'Error processing server response';
                }
                window.ui.showNotification('Error', `Error moving the folder: ${errorMessage}`);
                return false;
            }
        } catch (error) {
            console.error('Error moving folder:', error);
            window.ui.showNotification('Error', 'Error moving the folder');
            return false;
        }
    },

    /**
     * Copy a file to another folder
     * @param {string} fileId - File ID
     * @param {string} targetFolderId - Target folder ID
     * @returns {Promise<boolean>} - Success status
     */
    async copyFile(fileId, targetFolderId) {
        try {
            const response = await fetch('/api/batch/files/copy', {
                method: 'POST',
                headers: {
                    ...getAuthHeaders(),
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    file_ids: [fileId],
                    target_folder_id: targetFolderId === "" ? null : targetFolderId
                })
            });

            if (response.ok) {
                const result = await response.json();
                // Reload files after copying
                await window.loadFiles();
                window.ui.showNotification('File copied', 'File copied successfully');
                return true;
            } else {
                let errorMessage = 'Unknown error';
                try {
                    const errorData = await response.json();
                    errorMessage = errorData.error || 'Unknown error';
                } catch (e) {
                    errorMessage = 'Error processing server response';
                }
                window.ui.showNotification('Error', `Error copying the file: ${errorMessage}`);
                return false;
            }
        } catch (error) {
            console.error('Error copying file:', error);
            window.ui.showNotification('Error', 'Error copying the file');
            return false;
        }
    },

    /**
     * Copy a folder to another folder
     * Note: Backend folder copy is not yet implemented, this shows a notification
     * @param {string} folderId - Folder ID
     * @param {string} targetFolderId - Target folder ID
     * @returns {Promise<boolean>} - Success status
     */
    async copyFolder(folderId, targetFolderId) {
        // Folder copy is not yet implemented in the backend
        window.ui.showNotification('Not implemented', 'Folder copy is not yet supported');
        return false;
    },

    /**
     * Rename a file
     * @param {string} fileId - File ID
     * @param {string} newName - New file name
     * @returns {Promise<boolean>} - Success status
     */
    async renameFile(fileId, newName) {
        try {
            console.log(`Renaming file ${fileId} to "${newName}"`);

            const response = await fetch(`/api/files/${fileId}/rename`, {
                method: 'PUT',
                headers: {
                    ...getAuthHeaders(),
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ name: newName })
            });

            console.log('Response status:', response.status);

            if (response.ok) {
                window.ui.showNotification(
                    window.i18n ? window.i18n.t('notifications.file_renamed') : 'File renamed',
                    window.i18n ? window.i18n.t('notifications.file_renamed_to', { name: newName }) : `File renamed to "${newName}"`
                );
                return true;
            } else {
                const errorText = await response.text();
                console.error('Error response:', errorText);
                let errorMessage = 'Unknown error';
                try {
                    const errorData = JSON.parse(errorText);
                    errorMessage = errorData.error || response.statusText;
                } catch (e) {
                    errorMessage = errorText || response.statusText;
                }
                window.ui.showNotification('Error', `Error renaming the file: ${errorMessage}`);
                return false;
            }
        } catch (error) {
            console.error('Error renaming file:', error);
            window.ui.showNotification('Error', 'Error renaming the file');
            return false;
        }
    },

    /**
     * Rename a folder
     * @param {string} folderId - Folder ID
     * @param {string} newName - New folder name
     * @returns {Promise<boolean>} - Success status
     */
    async renameFolder(folderId, newName) {
        try {
            console.log(`Renaming folder ${folderId} to "${newName}"`);

            const response = await fetch(`/api/folders/${folderId}/rename`, {
                method: 'PUT',
                headers: {
                    ...getAuthHeaders(),
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ name: newName })
            });

            console.log('Response status:', response.status);

            if (response.ok) {
                window.ui.showNotification('Folder renamed', `Folder renamed to "${newName}"`);
                return true;
            } else {
                const errorText = await response.text();
                console.error('Error response:', errorText);

                let errorMessage = 'Unknown error';
                try {
                    // Try to parse as JSON
                    const errorData = JSON.parse(errorText);
                    errorMessage = errorData.error || response.statusText;
                } catch (e) {
                    // If not JSON, use text as is
                    errorMessage = errorText || response.statusText;
                }

                window.ui.showNotification('Error', `Error renaming the folder: ${errorMessage}`);
                return false;
            }
        } catch (error) {
            console.error('Error renaming folder:', error);
            window.ui.showNotification('Error', 'Error renaming the folder');
            return false;
        }
    },

    /**
     * Move a file to trash
     * @param {string} fileId - File ID
     * @param {string} fileName - File name
     * @returns {Promise<boolean>} - Success status
     */
    async deleteFile(fileId, fileName) {
        const confirmed = await showConfirmDialog({
            title: window.i18n ? window.i18n.t('dialogs.confirm_delete') : 'Move to trash',
            message: window.i18n ? window.i18n.t('dialogs.confirm_delete_file', { name: fileName }) : `Are you sure you want to move the file "${fileName}" to trash?`,
            confirmText: window.i18n ? window.i18n.t('actions.delete') : 'Delete',
        });
        if (!confirmed) return false;
        
        try {
            // Use the trash API endpoint
            const response = await fetch(`/api/trash/files/${fileId}`, {
                method: 'DELETE',
                headers: getAuthHeaders()
            });

            if (response.ok) {
                window.loadFiles();
                window.ui.showNotification('File moved to trash', `"${fileName}" moved to trash`);
                return true;
            } else {
                // Fallback to direct deletion if trash fails
                const fallbackResponse = await fetch(`/api/files/${fileId}`, {
                    method: 'DELETE',
                    headers: getAuthHeaders()
                });
                
                if (fallbackResponse.ok) {
                    window.loadFiles();
                    window.ui.showNotification('File deleted', `"${fileName}" deleted successfully`);
                    return true;
                } else {
                    window.ui.showNotification('Error', 'Error deleting the file');
                    return false;
                }
            }
        } catch (error) {
            console.error('Error deleting file:', error);
            window.ui.showNotification('Error', 'Error deleting the file');
            return false;
        }
    },

    /**
     * Move a folder to trash
     * @param {string} folderId - Folder ID
     * @param {string} folderName - Folder name
     * @returns {Promise<boolean>} - Success status
     */
    async deleteFolder(folderId, folderName) {
        const confirmed = await showConfirmDialog({
            title: window.i18n ? window.i18n.t('dialogs.confirm_delete') : 'Move to trash',
            message: window.i18n ? window.i18n.t('dialogs.confirm_delete_folder', { name: folderName }) : `Are you sure you want to move the folder "${folderName}" and all its contents to trash?`,
            confirmText: window.i18n ? window.i18n.t('actions.delete') : 'Delete',
        });
        if (!confirmed) return false;
        
        try {
            // Use the trash API endpoint
            const response = await fetch(`/api/trash/folders/${folderId}`, {
                method: 'DELETE',
                headers: getAuthHeaders()
            });

            if (response.ok) {
                // If we're inside the folder we just deleted, go back up
                if (window.app.currentPath === folderId) {
                    window.app.currentPath = '';
                    window.ui.updateBreadcrumb('');
                }
                window.loadFiles();
                window.ui.showNotification('Folder moved to trash', `"${folderName}" moved to trash`);
                return true;
            } else {
                // Fallback to direct deletion if trash fails
                const fallbackResponse = await fetch(`/api/folders/${folderId}`, {
                    method: 'DELETE',
                    headers: getAuthHeaders()
                });
                
                if (fallbackResponse.ok) {
                    // If we're inside the folder we just deleted, go back up
                    if (window.app.currentPath === folderId) {
                        window.app.currentPath = '';
                        window.ui.updateBreadcrumb('');
                    }
                    window.loadFiles();
                    window.ui.showNotification('Folder deleted', `"${folderName}" deleted successfully`);
                    return true;
                } else {
                    window.ui.showNotification('Error', 'Error deleting the folder');
                    return false;
                }
            }
        } catch (error) {
            console.error('Error deleting folder:', error);
            window.ui.showNotification('Error', 'Error deleting the folder');
            return false;
        }
    },
    
    /**
     * Get trash items
     * @returns {Promise<Array>} - List of trash items
     */
    async getTrashItems() {
        try {
            const response = await fetch('/api/trash', {
                headers: getAuthHeaders()
            });
            
            if (response.ok) {
                return await response.json();
            } else {
                console.error('Error fetching trash items:', response.statusText);
                return [];
            }
        } catch (error) {
            console.error('Error fetching trash items:', error);
            return [];
        }
    },
    
    /**
     * Restore an item from trash
     * @param {string} trashId - Trash item ID
     * @returns {Promise<boolean>} - Operation success
     */
    async restoreFromTrash(trashId) {
        try {
            const response = await fetch(`/api/trash/${trashId}/restore`, {
                method: 'POST',
                headers: {
                    ...getAuthHeaders(),
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({})
            });
            
            if (response.ok) {
                window.ui.showNotification('Item restored', 'Item restored successfully');
                return true;
            } else {
                window.ui.showNotification('Error', 'Error restoring the item');
                return false;
            }
        } catch (error) {
            console.error('Error restoring item from trash:', error);
            window.ui.showNotification('Error', 'Error restoring the item');
            return false;
        }
    },
    
    /**
     * Permanently delete a trash item
     * @param {string} trashId - Trash item ID
     * @returns {Promise<boolean>} - Operation success
     */
    async deletePermanently(trashId) {
        const confirmed = await showConfirmDialog({
            title: window.i18n ? window.i18n.t('dialogs.confirm_permanent_delete') : 'Delete permanently',
            message: window.i18n ? window.i18n.t('dialogs.confirm_permanent_delete_msg') : 'Are you sure you want to permanently delete this item? This action cannot be undone.',
            confirmText: window.i18n ? window.i18n.t('actions.delete_permanently') : 'Delete permanently',
        });
        if (!confirmed) return false;
        
        try {
            const response = await fetch(`/api/trash/${trashId}`, {
                method: 'DELETE',
                headers: getAuthHeaders()
            });
            
            if (response.ok) {
                window.ui.showNotification('Item deleted', 'Item permanently deleted');
                return true;
            } else {
                window.ui.showNotification('Error', 'Error deleting the item');
                return false;
            }
        } catch (error) {
            console.error('Error deleting item permanently:', error);
            window.ui.showNotification('Error', 'Error deleting the item');
            return false;
        }
    },
    
    /**
     * Empty the trash
     * @returns {Promise<boolean>} - Operation success
     */
    async emptyTrash() {
        const confirmed = await showConfirmDialog({
            title: window.i18n ? window.i18n.t('dialogs.confirm_empty_trash') : 'Empty trash',
            message: window.i18n ? window.i18n.t('trash.empty_confirm') : 'Are you sure you want to empty the trash? This action will permanently delete all items.',
            confirmText: window.i18n ? window.i18n.t('actions.empty_trash') : 'Empty trash',
        });
        if (!confirmed) return false;
        
        try {
            const response = await fetch('/api/trash/empty', {
                method: 'DELETE',
                headers: getAuthHeaders()
            });
            
            if (response.ok) {
                window.ui.showNotification('Trash emptied', 'The trash has been emptied successfully');
                return true;
            } else {
                window.ui.showNotification('Error', 'Error emptying the trash');
                return false;
            }
        } catch (error) {
            console.error('Error emptying trash:', error);
            window.ui.showNotification('Error', 'Error emptying the trash');
            return false;
        }
    },
    
    /**
     * Download a file
     * @param {string} fileId - File ID
     * @param {string} fileName - File name
     */
    async downloadFile(fileId, fileName) {
        try {
            const response = await fetch(`/api/files/${fileId}`, {
                headers: getAuthHeaders()
            });
            if (response.ok) {
                const blob = await response.blob();
                const url = URL.createObjectURL(blob);
                const link = document.createElement('a');
                link.href = url;
                link.download = fileName;
                document.body.appendChild(link);
                link.click();
                document.body.removeChild(link);
                URL.revokeObjectURL(url);
            } else {
                window.ui.showNotification('Error', 'Error downloading the file');
            }
        } catch (error) {
            console.error('Error downloading file:', error);
            window.ui.showNotification('Error', 'Error downloading the file');
        }
    },
    
    /**
     * Download a folder as ZIP
     * @param {string} folderId - Folder ID
     * @param {string} folderName - Folder name
     */
    async downloadFolder(folderId, folderName) {
        try {
            // Show notification to user
            window.ui.showNotification('Preparing download', 'Preparing the folder for download...');
            
            const response = await fetch(`/api/folders/${folderId}/download?format=zip`, {
                headers: getAuthHeaders()
            });
            if (response.ok) {
                const blob = await response.blob();
                const url = URL.createObjectURL(blob);
                const link = document.createElement('a');
                link.href = url;
                link.download = `${folderName}.zip`;
                document.body.appendChild(link);
                link.click();
                document.body.removeChild(link);
                URL.revokeObjectURL(url);
            } else {
                window.ui.showNotification('Error', 'Error downloading the folder');
            }
        } catch (error) {
            console.error('Error downloading folder:', error);
            window.ui.showNotification('Error', 'Error downloading the folder');
        }
    }
};

// Expose file operations module globally
window.fileOps = fileOps;
