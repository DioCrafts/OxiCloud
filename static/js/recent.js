/**
 * OxiCloud - Recent Files Module
 * This file handles tracking and displaying recently accessed files
 */

// Recent Files Module
const recent = {
    // Key for storing recent files in localStorage
    STORAGE_KEY: 'oxicloud_recent_files',
    
    // Maximum number of recent files to store
    MAX_RECENT_FILES: 20,
    
    /**
     * Initialize recent files module
     */
    init() {
        console.log('Initializing recent files module');
        this.ensureRecentFilesStorage();
        this.setupEventListeners();
    },
    
    /**
     * Make sure the recent files storage is initialized
     */
    ensureRecentFilesStorage() {
        if (!localStorage.getItem(this.STORAGE_KEY)) {
            localStorage.setItem(this.STORAGE_KEY, JSON.stringify([]));
        }
    },
    
    /**
     * Set up event listeners to track file access
     */
    setupEventListeners() {
        // Listen for custom event when a file is accessed
        document.addEventListener('file-accessed', (event) => {
            if (event.detail && event.detail.file) {
                this.addRecentFile(event.detail.file);
            }
        });
    },
    
    /**
     * Add a file to recent files
     * @param {Object} file - File object containing id, name, folder_id, etc.
     */
    addRecentFile(file) {
        // Don't add if no file or no ID
        if (!file || !file.id) {
            return;
        }
        
        // Get current recent files
        const recentFiles = this.getRecentFiles();
        
        // Remove if file already exists in recent files
        const existingIndex = recentFiles.findIndex(item => item.id === file.id);
        if (existingIndex !== -1) {
            recentFiles.splice(existingIndex, 1);
        }
        
        // Add file with timestamp to the beginning of the array
        const fileWithTimestamp = {
            ...file,
            accessedAt: Date.now()
        };
        
        recentFiles.unshift(fileWithTimestamp);
        
        // Keep only the most recent files (limit to MAX_RECENT_FILES)
        const trimmedFiles = recentFiles.slice(0, this.MAX_RECENT_FILES);
        
        // Save back to localStorage
        localStorage.setItem(this.STORAGE_KEY, JSON.stringify(trimmedFiles));
    },
    
    /**
     * Get recent files from localStorage
     * @returns {Array} Array of recent file objects with timestamps
     */
    getRecentFiles() {
        try {
            const recentFilesJson = localStorage.getItem(this.STORAGE_KEY);
            return recentFilesJson ? JSON.parse(recentFilesJson) : [];
        } catch (error) {
            console.error('Error loading recent files:', error);
            return [];
        }
    },
    
    /**
     * Clear all recent files
     */
    clearRecentFiles() {
        localStorage.setItem(this.STORAGE_KEY, JSON.stringify([]));
    },
    
    /**
     * Display recent files in the UI
     */
    async displayRecentFiles() {
        try {
            const recentFiles = this.getRecentFiles();
            
            // Clear existing content
            const filesGrid = document.getElementById('files-grid');
            const filesListView = document.getElementById('files-list-view');
            
            filesGrid.innerHTML = '';
            filesListView.innerHTML = `
                <div class="list-header">
                    <div data-i18n="files.name">Nombre</div>
                    <div data-i18n="files.type">Tipo</div>
                    <div data-i18n="files.size">Tamaño</div>
                    <div data-i18n="files.last_accessed">Último acceso</div>
                </div>
            `;
            
            // Update breadcrumb - just show Home
            window.ui.updateBreadcrumb('');
            
            // Show empty state if no recent files
            if (recentFiles.length === 0) {
                const emptyState = document.createElement('div');
                emptyState.className = 'empty-state';
                emptyState.innerHTML = `
                    <i class="fas fa-clock" style="font-size: 48px; color: #ddd; margin-bottom: 16px;"></i>
                    <p>${window.i18n ? window.i18n.t('recent.empty_state') : 'No hay archivos recientes'}</p>
                    <p>${window.i18n ? window.i18n.t('recent.empty_hint') : 'Los archivos que abras aparecerán aquí'}</p>
                `;
                filesGrid.appendChild(emptyState);
                return;
            }
            
            // Process each recent file
            for (const recentFile of recentFiles) {
                this.createRecentFileElement(recentFile, filesGrid, filesListView);
            }
            
            // Update file icons
            window.ui.updateFileIcons();
            
        } catch (error) {
            console.error('Error displaying recent files:', error);
            window.ui.showNotification('Error', 'Error al cargar archivos recientes');
        }
    },
    
    /**
     * Create a file element for a recent file
     * @param {Object} file - Recent file object
     * @param {HTMLElement} filesGrid - Grid view container
     * @param {HTMLElement} filesListView - List view container
     */
    createRecentFileElement(file, filesGrid, filesListView) {
        // Determine icon and type
        let iconClass = 'fas fa-file';
        let iconSpecialClass = '';
        let typeLabel = 'Documento';

        if (file.mime_type) {
            if (file.mime_type.startsWith('image/')) {
                iconClass = 'fas fa-file-image';
                iconSpecialClass = 'image-icon';
                typeLabel = window.i18n ? window.i18n.t('files.file_types.image') : 'Imagen';
            } else if (file.mime_type.startsWith('text/')) {
                iconClass = 'fas fa-file-alt';
                iconSpecialClass = 'text-icon';
                typeLabel = window.i18n ? window.i18n.t('files.file_types.text') : 'Texto';
            } else if (file.mime_type.startsWith('video/')) {
                iconClass = 'fas fa-file-video';
                iconSpecialClass = 'video-icon';
                typeLabel = window.i18n ? window.i18n.t('files.file_types.video') : 'Video';
            } else if (file.mime_type.startsWith('audio/')) {
                iconClass = 'fas fa-file-audio';
                iconSpecialClass = 'audio-icon';
                typeLabel = window.i18n ? window.i18n.t('files.file_types.audio') : 'Audio';
            } else if (file.mime_type === 'application/pdf') {
                iconClass = 'fas fa-file-pdf';
                iconSpecialClass = 'pdf-icon';
                typeLabel = window.i18n ? window.i18n.t('files.file_types.pdf') : 'PDF';
            }
        }

        // Format size and date
        const fileSize = window.formatFileSize ? window.formatFileSize(file.size || 0) : '0 B';
        const accessedDate = new Date(file.accessedAt);
        const formattedDate = accessedDate.toLocaleDateString() + ' ' +
                            accessedDate.toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'});

        // Grid view element
        const fileGridElement = document.createElement('div');
        fileGridElement.className = 'file-card recent-item';
        fileGridElement.dataset.fileId = file.id;
        fileGridElement.dataset.fileName = file.name;
        fileGridElement.dataset.folderId = file.folder_id || "";

        fileGridElement.innerHTML = `
            <div class="recent-indicator">
                <i class="fas fa-clock"></i>
            </div>
            <div class="file-icon ${iconSpecialClass}">
                <i class="${iconClass}"></i>
            </div>
            <div class="file-name">${file.name}</div>
            <div class="file-info">Accedido ${formattedDate.split(' ')[0]}</div>
        `;

        // Download on click
        fileGridElement.addEventListener('click', () => {
            window.location.href = `/api/files/${file.id}`;
            
            // Dispatch custom event to update recent files
            document.dispatchEvent(new CustomEvent('file-accessed', {
                detail: { file }
            }));
        });

        // Context menu
        fileGridElement.addEventListener('contextmenu', (e) => {
            e.preventDefault();

            window.app.contextMenuTargetFile = {
                id: file.id,
                name: file.name,
                folder_id: file.folder_id || ""
            };

            let fileContextMenu = document.getElementById('file-context-menu');
            fileContextMenu.style.left = `${e.pageX}px`;
            fileContextMenu.style.top = `${e.pageY}px`;
            fileContextMenu.style.display = 'block';
        });

        filesGrid.appendChild(fileGridElement);

        // List view element
        const fileListElement = document.createElement('div');
        fileListElement.className = 'file-item recent-item';
        fileListElement.dataset.fileId = file.id;
        fileListElement.dataset.fileName = file.name;
        fileListElement.dataset.folderId = file.folder_id || "";

        fileListElement.innerHTML = `
            <div class="recent-indicator">
                <i class="fas fa-clock"></i>
            </div>
            <div class="name-cell">
                <div class="file-icon ${iconSpecialClass}">
                    <i class="${iconClass}"></i>
                </div>
                <span>${file.name}</span>
            </div>
            <div class="type-cell">${typeLabel}</div>
            <div class="size-cell">${fileSize}</div>
            <div class="date-cell">${formattedDate}</div>
        `;

        // Download on click
        fileListElement.addEventListener('click', () => {
            window.location.href = `/api/files/${file.id}`;
            
            // Dispatch custom event to update recent files
            document.dispatchEvent(new CustomEvent('file-accessed', {
                detail: { file }
            }));
        });

        // Context menu
        fileListElement.addEventListener('contextmenu', (e) => {
            e.preventDefault();

            window.app.contextMenuTargetFile = {
                id: file.id,
                name: file.name,
                folder_id: file.folder_id || ""
            };

            let fileContextMenu = document.getElementById('file-context-menu');
            fileContextMenu.style.left = `${e.pageX}px`;
            fileContextMenu.style.top = `${e.pageY}px`;
            fileContextMenu.style.display = 'block';
        });

        filesListView.appendChild(fileListElement);
    }
};

// Expose recent module globally
window.recent = recent;