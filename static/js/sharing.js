/**
 * OxiCloud Sharing Module
 * 
 * Manages file sharing between users and public link generation
 */

const SharingModule = (function() {
    // Cache DOM elements
    let shareModal, publicLinkModal, shareUserInput, sharePermissionSelect;
    let publicLinkUrlInput, publicLinkPasswordInput, publicLinkExpirationInput;
    let publicLinkPermissionSelect, shareListContainer, linkListContainer;
    
    // Currently selected file
    let currentFileId = null;
    let currentFileName = null;
    
    /**
     * Initialize the sharing module
     */
    function init() {
        // Initialize modals if they don't exist yet
        if (!shareModal) {
            createModals();
            bindEvents();
            
            // Initialize Bootstrap components and event handlers
            if (typeof bootstrap !== 'undefined') {
                // Create Bootstrap modal instances when needed, not here
                // This ensures they're initialized properly
            } else {
                console.warn('Bootstrap not available for sharing modals');
            }
        }
    }
    
    /**
     * Create the sharing and public link modals
     */
    function createModals() {
        // Create share with user modal
        shareModal = document.createElement('div');
        shareModal.classList.add('modal', 'fade');
        shareModal.id = 'shareModal';
        shareModal.setAttribute('tabindex', '-1');
        shareModal.setAttribute('aria-modal', 'true');
        // Not using aria-hidden anymore as it causes accessibility issues with focused elements
        
        shareModal.innerHTML = `
            <div class="modal-dialog">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" data-i18n="dialogs.share_file">Share File</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        <div class="mb-3">
                            <label for="fileName" class="form-label" data-i18n="dialogs.file">File</label>
                            <input type="text" class="form-control" id="fileName" readonly>
                        </div>
                        <div class="mb-3">
                            <label for="shareUser" class="form-label" data-i18n="dialogs.user_email">User Email</label>
                            <input type="email" class="form-control" id="shareUser" data-i18n-placeholder="Enter user email" placeholder="Enter user email">
                        </div>
                        <div class="mb-3">
                            <label for="sharePermission" class="form-label" data-i18n="dialogs.permission">Permission</label>
                            <select class="form-select" id="sharePermission">
                                <option value="READ">Read Only</option>
                                <option value="WRITE">Read & Write</option>
                                <option value="ADMIN">Admin</option>
                            </select>
                        </div>
                        <hr>
                        <h6 data-i18n="dialogs.users_with_access">Users with access</h6>
                        <div id="shareList" class="mt-3">
                            <!-- User list will be populated dynamically -->
                        </div>
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal" data-i18n="actions.cancel">Close</button>
                        <button type="button" class="btn btn-primary" id="shareButton" data-i18n="actions.share">Share</button>
                    </div>
                </div>
            </div>
        `;
        
        // Create public link modal
        publicLinkModal = document.createElement('div');
        publicLinkModal.classList.add('modal', 'fade');
        publicLinkModal.id = 'publicLinkModal';
        publicLinkModal.setAttribute('tabindex', '-1');
        publicLinkModal.setAttribute('aria-modal', 'true');
        // Not using aria-hidden anymore as it causes accessibility issues with focused elements
        
        publicLinkModal.innerHTML = `
            <div class="modal-dialog">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" data-i18n="dialogs.public_link">Create Public Link</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        <div class="mb-3">
                            <label for="publicLinkFileName" class="form-label" data-i18n="dialogs.file">File</label>
                            <input type="text" class="form-control" id="publicLinkFileName" readonly>
                        </div>
                        <div class="mb-3">
                            <label for="publicLinkPermission" class="form-label" data-i18n="dialogs.permission">Permission</label>
                            <select class="form-select" id="publicLinkPermission">
                                <option value="READ">Read Only</option>
                                <option value="WRITE">Read & Write</option>
                            </select>
                        </div>
                        <div class="mb-3">
                            <label for="publicLinkPassword" class="form-label" data-i18n="dialogs.password">Password (Optional)</label>
                            <input type="password" class="form-control" id="publicLinkPassword" placeholder="Leave empty for no password">
                        </div>
                        <div class="mb-3">
                            <label for="publicLinkExpiration" class="form-label" data-i18n="dialogs.expiration">Expiration (Optional)</label>
                            <input type="datetime-local" class="form-control" id="publicLinkExpiration">
                        </div>
                        <hr>
                        <h6 data-i18n="dialogs.existing_links">Existing Public Links</h6>
                        <div id="linkList" class="mt-3">
                            <!-- Link list will be populated dynamically -->
                        </div>
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal" data-i18n="actions.cancel">Close</button>
                        <button type="button" class="btn btn-primary" id="createLinkButton" data-i18n="actions.create_link">Create Link</button>
                    </div>
                </div>
            </div>
        `;
        
        // Add modals to document
        document.body.appendChild(shareModal);
        document.body.appendChild(publicLinkModal);
        
        // Cache DOM elements
        shareUserInput = document.getElementById('shareUser');
        sharePermissionSelect = document.getElementById('sharePermission');
        publicLinkPermissionSelect = document.getElementById('publicLinkPermission');
        publicLinkPasswordInput = document.getElementById('publicLinkPassword');
        publicLinkExpirationInput = document.getElementById('publicLinkExpiration');
        shareListContainer = document.getElementById('shareList');
        linkListContainer = document.getElementById('linkList');
        
        // Intentar traducir solo si tenemos traducciones en memoria
        try {
            if (window.i18n) {
                // Comprobar si hay traducciones cargadas antes de intentar traducir
                if (typeof window.i18n.areTranslationsLoaded === 'function' && 
                    window.i18n.areTranslationsLoaded()) {
                    window.i18n.translatePage();
                } else {
                    console.log("Skipping translation - translations not loaded yet");
                }
            }
        } catch (e) {
            console.log("Error translating sharing modals:", e);
        }
    }
    
    /**
     * Bind event handlers
     */
    function bindEvents() {
        // Share button click handler
        const shareButton = document.getElementById('shareButton');
        if (shareButton) {
            shareButton.addEventListener('click', shareFile);
        }
        
        // Create link button click handler
        const createLinkButton = document.getElementById('createLinkButton');
        if (createLinkButton) {
            createLinkButton.addEventListener('click', createPublicLink);
        }
        
        // Context menu items are now added directly in the UI.initializeContextMenus method
        // But we can register additional dynamic menu items if needed
        if (window.contextMenus && typeof window.contextMenus.addMenuItem === 'function') {
            // If we need to add these programmatically, uncomment these lines:
            // contextMenus.addMenuItem('file', 'Compartir', 'fas fa-share-alt', 'share-file-option', openShareModal);
            // contextMenus.addMenuItem('file', 'Crear enlace público', 'fas fa-link', 'public-link-option', openPublicLinkModal);
        }
    }
    
    /**
     * Open the share modal for a file
     * @param {Object} file - The file object
     */
    function openShareModal(file) {
        if (!file || !file.id) {
            console.error('No file selected');
            return;
        }
        
        currentFileId = file.id;
        currentFileName = file.name;
        
        // Set file name in modal
        const fileNameInput = document.getElementById('fileName');
        if (fileNameInput) {
            fileNameInput.value = currentFileName;
        }
        
        // Clear previous inputs
        if (shareUserInput) {
            shareUserInput.value = '';
        }
        if (sharePermissionSelect) {
            sharePermissionSelect.value = 'READ';
        }
        
        // Load existing shares
        loadSharedUsers();
        
        // Show modal
        try {
            // Try to use Bootstrap if available
            if (typeof bootstrap !== 'undefined') {
                const modal = new bootstrap.Modal(shareModal);
                modal.show();
                
                // Set initial focus on close button to prevent focus being trapped
                setTimeout(() => {
                    const closeButton = shareModal.querySelector('.btn-close');
                    if (closeButton) {
                        closeButton.focus();
                    }
                }, 150);
            } else {
                // Fallback for when Bootstrap is not available
                shareModal.style.display = 'block';
                shareModal.classList.add('show');
                document.body.classList.add('modal-open');
                
                // Set initial focus
                const closeButton = shareModal.querySelector('.btn-close');
                if (closeButton) {
                    closeButton.focus();
                }
            }
        } catch (error) {
            console.error('Error showing share modal:', error);
            shareModal.style.display = 'block';
        }
    }
    
    /**
     * Open the public link modal for a file
     * @param {Object} file - The file object
     */
    function openPublicLinkModal(file) {
        if (!file || !file.id) {
            console.error('No file selected');
            return;
        }
        
        currentFileId = file.id;
        currentFileName = file.name;
        
        // Set file name in modal
        const fileNameInput = document.getElementById('publicLinkFileName');
        if (fileNameInput) {
            fileNameInput.value = currentFileName;
        }
        
        // Clear previous inputs
        if (publicLinkPermissionSelect) {
            publicLinkPermissionSelect.value = 'READ';
        }
        if (publicLinkPasswordInput) {
            publicLinkPasswordInput.value = '';
        }
        if (publicLinkExpirationInput) {
            publicLinkExpirationInput.value = '';
        }
        
        // Load existing links
        loadPublicLinks();
        
        // Show modal
        try {
            // Try to use Bootstrap if available
            if (typeof bootstrap !== 'undefined') {
                const modal = new bootstrap.Modal(publicLinkModal);
                modal.show();
                
                // Set initial focus on close button to prevent focus being trapped
                setTimeout(() => {
                    const closeButton = publicLinkModal.querySelector('.btn-close');
                    if (closeButton) {
                        closeButton.focus();
                    }
                }, 150);
            } else {
                // Fallback for when Bootstrap is not available
                publicLinkModal.style.display = 'block';
                publicLinkModal.classList.add('show');
                document.body.classList.add('modal-open');
                
                // Set initial focus
                const closeButton = publicLinkModal.querySelector('.btn-close');
                if (closeButton) {
                    closeButton.focus();
                }
            }
        } catch (error) {
            console.error('Error showing public link modal:', error);
            publicLinkModal.style.display = 'block';
        }
    }
    
    /**
     * Load users who have access to the current file
     */
    async function loadSharedUsers() {
        if (!currentFileId || !shareListContainer) return;
        
        try {
            // Clear previous list
            shareListContainer.innerHTML = '<div class="spinner-border spinner-border-sm" role="status"><span class="visually-hidden">Loading...</span></div>';
            
            // Check if AuthModule exists
            const token = window.AuthModule && typeof AuthModule.getToken === 'function' ? 
                AuthModule.getToken() : localStorage.getItem('oxicloud_token');
            
            const response = await fetch(`/api/sharing/${currentFileId}/users`, {
                headers: {
                    'Authorization': `Bearer ${token}`
                }
            });
            
            if (!response.ok) throw new Error('Failed to load shared users');
            
            const data = await response.json();
            
            // Check if we have users
            if (data.users && data.users.length > 0) {
                let html = '';
                data.users.forEach(user => {
                    const permissionBadge = getPermissionBadge(user.permission);
                    html += `
                        <div class="d-flex justify-content-between align-items-center mb-2">
                            <div>
                                <span class="fw-bold">${user.username}</span>
                                <span>${permissionBadge}</span>
                            </div>
                            <div class="btn-group">
                                <button type="button" class="btn btn-sm btn-outline-primary edit-permission" data-user-id="${user.user_id}">
                                    <i class="bi bi-pencil"></i>
                                </button>
                                <button type="button" class="btn btn-sm btn-outline-danger remove-share" data-user-id="${user.user_id}">
                                    <i class="bi bi-trash"></i>
                                </button>
                            </div>
                        </div>
                    `;
                });
                shareListContainer.innerHTML = html;
                
                // Add event listeners for edit and remove buttons
                document.querySelectorAll('.edit-permission').forEach(btn => {
                    btn.addEventListener('click', () => editPermission(btn.dataset.userId));
                });
                
                document.querySelectorAll('.remove-share').forEach(btn => {
                    btn.addEventListener('click', () => removeShare(btn.dataset.userId));
                });
            } else {
                shareListContainer.innerHTML = '<p class="text-muted">No users have access to this file</p>';
            }
        } catch (error) {
            console.error('Error loading shared users:', error);
            shareListContainer.innerHTML = '<p class="text-danger">Failed to load shared users</p>';
        }
    }
    
    /**
     * Load public links for the current file
     */
    async function loadPublicLinks() {
        if (!currentFileId || !linkListContainer) return;
        
        try {
            // Clear previous list
            linkListContainer.innerHTML = '<div class="spinner-border spinner-border-sm" role="status"><span class="visually-hidden">Loading...</span></div>';
            
            // Check if AuthModule exists
            const token = window.AuthModule && typeof AuthModule.getToken === 'function' ? 
                AuthModule.getToken() : localStorage.getItem('oxicloud_token');
            
            const response = await fetch(`/api/public-links/file/${currentFileId}`, {
                headers: {
                    'Authorization': `Bearer ${token}`
                }
            });
            
            if (!response.ok) throw new Error('Failed to load public links');
            
            const links = await response.json();
            
            // Check if we have links
            if (links && links.length > 0) {
                let html = '';
                links.forEach(link => {
                    const permissionBadge = getPermissionBadge(link.permission);
                    const hasPassword = link.has_password ? 
                        '<span class="badge bg-secondary ms-1"><i class="bi bi-lock-fill"></i> Password</span>' : '';
                    const isExpired = link.is_expired ? 
                        '<span class="badge bg-danger ms-1">Expired</span>' : '';
                    const expiresText = link.expires_at ? 
                        `<small class="text-muted d-block">Expires: ${new Date(link.expires_at).toLocaleString()}</small>` : '';
                    
                    html += `
                        <div class="card mb-2">
                            <div class="card-body p-3">
                                <div class="d-flex justify-content-between">
                                    <div>
                                        <div class="d-flex align-items-center">
                                            <span class="me-2">${permissionBadge}</span>
                                            ${hasPassword}
                                            ${isExpired}
                                        </div>
                                        ${expiresText}
                                        <small class="text-muted d-block">Access count: ${link.access_count}</small>
                                    </div>
                                    <div class="btn-group">
                                        <button type="button" class="btn btn-sm btn-outline-primary copy-link" data-link="${link.share_url}">
                                            <i class="bi bi-clipboard"></i>
                                        </button>
                                        <button type="button" class="btn btn-sm btn-outline-danger delete-link" data-link-id="${link.id}">
                                            <i class="bi bi-trash"></i>
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    `;
                });
                linkListContainer.innerHTML = html;
                
                // Add event listeners for copy and delete buttons
                document.querySelectorAll('.copy-link').forEach(btn => {
                    btn.addEventListener('click', () => copyLinkToClipboard(btn.dataset.link));
                });
                
                document.querySelectorAll('.delete-link').forEach(btn => {
                    btn.addEventListener('click', () => deletePublicLink(btn.dataset.linkId));
                });
            } else {
                linkListContainer.innerHTML = '<p class="text-muted">No public links for this file</p>';
            }
        } catch (error) {
            console.error('Error loading public links:', error);
            linkListContainer.innerHTML = '<p class="text-danger">Failed to load public links</p>';
        }
    }
    
    /**
     * Share the current file with a user
     */
    async function shareFile() {
        const userEmail = shareUserInput ? shareUserInput.value.trim() : '';
        const permission = sharePermissionSelect ? sharePermissionSelect.value : 'READ';
        
        if (!userEmail) {
            window.ui.showNotification('Warning', 'Please enter a user email');
            return;
        }
        
        if (!currentFileId) {
            window.ui.showNotification('Error', 'No file selected');
            return;
        }
        
        try {
            // Check if AuthModule exists
            const token = window.AuthModule && typeof AuthModule.getToken === 'function' ? 
                AuthModule.getToken() : localStorage.getItem('oxicloud_token');
            
            const response = await fetch('/api/sharing', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${token}`
                },
                body: JSON.stringify({
                    file_id: currentFileId,
                    user_id: userEmail, // We're using email as the identifier
                    permission: permission
                })
            });
            
            if (!response.ok) {
                const error = await response.json();
                throw new Error(error.error || 'Failed to share file');
            }
            
            // Clear input and reload list
            if (shareUserInput) {
                shareUserInput.value = '';
            }
            window.ui.showNotification('Success', `File shared with ${userEmail}`);
            
            // Reload shared users list
            loadSharedUsers();
        } catch (error) {
            console.error('Error sharing file:', error);
            window.ui.showNotification('Error', error.message || 'Failed to share file');
        }
    }
    
    /**
     * Create a public link for the current file
     */
    async function createPublicLink() {
        const permission = publicLinkPermissionSelect ? publicLinkPermissionSelect.value : 'READ';
        const password = publicLinkPasswordInput ? publicLinkPasswordInput.value.trim() : '';
        const expirationStr = publicLinkExpirationInput ? publicLinkExpirationInput.value : '';
        
        if (!currentFileId) {
            window.ui.showNotification('Error', 'No file selected');
            return;
        }
        
        try {
            // Prepare request body
            const requestBody = {
                file_id: currentFileId,
                permission: permission
            };
            
            // Add password if provided
            if (password) {
                requestBody.password = password;
            }
            
            // Add expiration if provided
            if (expirationStr) {
                const expirationDate = new Date(expirationStr);
                requestBody.expires_at = expirationDate.toISOString();
            }
            
            // Check if AuthModule exists
            const token = window.AuthModule && typeof AuthModule.getToken === 'function' ? 
                AuthModule.getToken() : localStorage.getItem('oxicloud_token');
            
            const response = await fetch('/api/public-links', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${token}`
                },
                body: JSON.stringify(requestBody)
            });
            
            if (!response.ok) {
                const error = await response.json();
                throw new Error(error.error || 'Failed to create public link');
            }
            
            const link = await response.json();
            
            // Clear inputs
            if (publicLinkPasswordInput) {
                publicLinkPasswordInput.value = '';
            }
            if (publicLinkExpirationInput) {
                publicLinkExpirationInput.value = '';
            }
            
            // Show link and reload list
            window.ui.showNotification('Success', 'Public link created');
            copyLinkToClipboard(link.share_url);
            
            // Reload public links list
            loadPublicLinks();
        } catch (error) {
            console.error('Error creating public link:', error);
            window.ui.showNotification('Error', error.message || 'Failed to create public link');
        }
    }
    
    /**
     * Edit permission for a shared file
     * @param {string} userId - The user ID
     */
    async function editPermission(userId) {
        // Prompt user for new permission
        const newPermission = window.prompt('Select permission (READ, WRITE, ADMIN):', 'READ');
        if (!newPermission) return;
        
        // Validate permission
        const validPermissions = ['READ', 'WRITE', 'ADMIN'];
        if (!validPermissions.includes(newPermission.toUpperCase())) {
            window.ui.showNotification('Warning', 'Invalid permission');
            return;
        }
        
        try {
            // Check if AuthModule exists
            const token = window.AuthModule && typeof AuthModule.getToken === 'function' ? 
                AuthModule.getToken() : localStorage.getItem('oxicloud_token');
            
            const response = await fetch(`/api/sharing/${currentFileId}/permission`, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${token}`
                },
                body: JSON.stringify({
                    user_id: userId,
                    permission: newPermission.toUpperCase()
                })
            });
            
            if (!response.ok) {
                const error = await response.json();
                throw new Error(error.error || 'Failed to update permission');
            }
            
            window.ui.showNotification('Success', 'Permission updated');
            
            // Reload shared users list
            loadSharedUsers();
        } catch (error) {
            console.error('Error updating permission:', error);
            window.ui.showNotification('Error', error.message || 'Failed to update permission');
        }
    }
    
    /**
     * Remove share from a user
     * @param {string} userId - The user ID
     */
    async function removeShare(userId) {
        if (!confirm('Are you sure you want to remove this user\'s access?')) return;
        
        try {
            // Check if AuthModule exists
            const token = window.AuthModule && typeof AuthModule.getToken === 'function' ? 
                AuthModule.getToken() : localStorage.getItem('oxicloud_token');
            
            const response = await fetch(`/api/sharing/${currentFileId}/user/${userId}`, {
                method: 'DELETE',
                headers: {
                    'Authorization': `Bearer ${token}`
                }
            });
            
            if (!response.ok) {
                const error = await response.json();
                throw new Error(error.error || 'Failed to remove share');
            }
            
            window.ui.showNotification('Success', 'User access removed');
            
            // Reload shared users list
            loadSharedUsers();
        } catch (error) {
            console.error('Error removing share:', error);
            window.ui.showNotification('Error', error.message || 'Failed to remove share');
        }
    }
    
    /**
     * Delete a public link
     * @param {string} linkId - The public link ID
     */
    async function deletePublicLink(linkId) {
        if (!confirm('Are you sure you want to delete this public link?')) return;
        
        try {
            // Check if AuthModule exists
            const token = window.AuthModule && typeof AuthModule.getToken === 'function' ? 
                AuthModule.getToken() : localStorage.getItem('oxicloud_token');
            
            const response = await fetch(`/api/public-links/${linkId}`, {
                method: 'DELETE',
                headers: {
                    'Authorization': `Bearer ${token}`
                }
            });
            
            if (!response.ok) {
                const error = await response.json();
                throw new Error(error.error || 'Failed to delete public link');
            }
            
            window.ui.showNotification('Success', 'Public link deleted');
            
            // Reload public links list
            loadPublicLinks();
        } catch (error) {
            console.error('Error deleting public link:', error);
            window.ui.showNotification('Error', error.message || 'Failed to delete public link');
        }
    }
    
    /**
     * Copy a link to clipboard
     * @param {string} link - The link to copy
     */
    function copyLinkToClipboard(link) {
        navigator.clipboard.writeText(link)
            .then(() => {
                window.ui.showNotification('Success', 'Link copied to clipboard');
            })
            .catch(err => {
                console.error('Error copying link:', err);
                window.ui.showNotification('Error', 'Failed to copy link');
            });
    }
    
    /**
     * Get a badge for a permission level
     * @param {string} permission - The permission level
     * @returns {string} HTML for the badge
     */
    function getPermissionBadge(permission) {
        switch (permission) {
            case 'READ':
                return '<span class="badge bg-info ms-2">Read</span>';
            case 'WRITE':
                return '<span class="badge bg-warning ms-2">Write</span>';
            case 'ADMIN':
                return '<span class="badge bg-danger ms-2">Admin</span>';
            default:
                return '<span class="badge bg-secondary ms-2">Unknown</span>';
        }
    }
    
    // Public API
    return {
        init,
        openShareModal,
        openPublicLinkModal
    };
})();

// Expose the sharing module globally
window.SharingModule = SharingModule;

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', function() {
    // Check if we're on the login page
    if (window.location.pathname === '/login' || window.location.pathname === '/login/') {
        return; // Skip initialization on login page
    }
    
    // Check if we're on the main app page (not login or other pages)
    const isMainApp = document.getElementById('files-grid') !== null;
    
    // Only initialize if we're on the main app page and authenticated
    if (isMainApp && window.AuthModule && typeof AuthModule.isAuthenticated === 'function' && AuthModule.isAuthenticated()) {
        // Delay initialization slightly to allow other components to load
        setTimeout(() => {
            SharingModule.init();
        }, 100);
    }
});