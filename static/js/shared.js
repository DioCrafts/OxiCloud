/**
 * OxiCloud Shared Resources Page
 * Manages the display and interaction with shared files and folders
 */

// Authentication check function
function checkAuthentication() {
    // Names of variables from auth.js
    const TOKEN_KEY = 'oxicloud_token';
    const TOKEN_EXPIRY_KEY = 'oxicloud_token_expiry';
    const USER_DATA_KEY = 'oxicloud_user';
    
    const token = localStorage.getItem(TOKEN_KEY);
    const tokenExpiry = localStorage.getItem(TOKEN_EXPIRY_KEY);
    
    if (!token || !tokenExpiry || new Date(tokenExpiry) < new Date()) {
        // No token or expired token
        window.location.href = '/login';
        return;
    }
    
    // Display username in notification if available
    const userData = JSON.parse(localStorage.getItem(USER_DATA_KEY) || '{}');
    if (userData.username) {
        console.log(`Authenticated as ${userData.username}`);
    }
}

document.addEventListener('DOMContentLoaded', async () => {
    // Initialize i18n
    await initializeI18n();
    
    // Wait a moment for translations to fully load
    setTimeout(() => {
        // Manually translate all elements with data-i18n attribute
        if (window.i18n && window.i18n.translatePage) {
            window.i18n.translatePage();
        }
    }, 500);
    
    // Elements
    const sharedItemsList = document.getElementById('shared-items-list');
    const emptySharedState = document.getElementById('empty-shared-state');
    const filterType = document.getElementById('filter-type');
    const sortBy = document.getElementById('sort-by');
    const sharedSearch = document.getElementById('shared-search');
    const sharedSearchBtn = document.getElementById('shared-search-btn');
    const goToFilesBtn = document.getElementById('go-to-files');
    
    // Share dialog elements
    const shareDialog = document.getElementById('share-dialog');
    const shareDialogCloseBtn = shareDialog.querySelector('.close-dialog-btn');
    const shareDialogIcon = document.getElementById('share-dialog-icon');
    const shareDialogName = document.getElementById('share-dialog-name');
    const shareLinkUrl = document.getElementById('share-link-url');
    const copyLinkBtn = document.getElementById('copy-link-btn');
    const enablePassword = document.getElementById('enable-password');
    const sharePassword = document.getElementById('share-password');
    const generatePasswordBtn = document.getElementById('generate-password');
    const enableExpiration = document.getElementById('enable-expiration');
    const shareExpiration = document.getElementById('share-expiration');
    const permissionRead = document.getElementById('permission-read');
    const permissionWrite = document.getElementById('permission-write');
    const permissionReshare = document.getElementById('permission-reshare');
    const updateShareBtn = document.getElementById('update-share-btn');
    const removeShareBtn = document.getElementById('remove-share-btn');
    
    // Notification dialog elements
    const notificationDialog = document.getElementById('share-notification-dialog');
    const notificationCloseBtn = notificationDialog.querySelector('.close-dialog-btn');
    const notifyDialogIcon = document.getElementById('notify-dialog-icon');
    const notifyDialogName = document.getElementById('notify-dialog-name');
    const notificationEmail = document.getElementById('notification-email');
    const notificationMessage = document.getElementById('notification-message');
    const sendNotificationBtn = document.getElementById('send-notification-btn');
    
    // Notification banner
    const notificationBanner = document.getElementById('notification-banner');
    const notificationBannerMessage = document.getElementById('notification-message');
    const closeNotificationBtn = document.getElementById('close-notification');
    
    // Current state
    let currentSharedItem = null;
    let allSharedItems = [];
    let filteredItems = [];
    
    // Initialize the page
    loadSharedItems();
    
    // Event listeners
    filterType.addEventListener('change', filterAndSortItems);
    sortBy.addEventListener('change', filterAndSortItems);
    sharedSearchBtn.addEventListener('click', filterAndSortItems);
    sharedSearch.addEventListener('keyup', (e) => {
        if (e.key === 'Enter') filterAndSortItems();
    });
    goToFilesBtn.addEventListener('click', () => window.location.href = '/');
    
    // Check authentication before loading
    checkAuthentication();
    
    // Share dialog event listeners
    shareDialogCloseBtn.addEventListener('click', () => closeShareDialog());
    copyLinkBtn.addEventListener('click', copyShareLink);
    enablePassword.addEventListener('change', () => {
        sharePassword.disabled = !enablePassword.checked;
        if (enablePassword.checked) sharePassword.focus();
    });
    generatePasswordBtn.addEventListener('click', generatePassword);
    enableExpiration.addEventListener('change', () => {
        shareExpiration.disabled = !enableExpiration.checked;
        if (enableExpiration.checked) shareExpiration.focus();
    });
    updateShareBtn.addEventListener('click', updateSharedItem);
    removeShareBtn.addEventListener('click', removeSharedItem);
    
    // Notification dialog event listeners
    notificationCloseBtn.addEventListener('click', () => closeNotificationDialog());
    sendNotificationBtn.addEventListener('click', sendNotification);
    
    // Notification banner event listeners
    closeNotificationBtn.addEventListener('click', () => {
        notificationBanner.classList.remove('active');
    });
    
    /**
     * Loads all shared items and displays them
     */
    function loadSharedItems() {
        // Get shared links from storage
        allSharedItems = getSharedLinks();
        
        // Display items
        filterAndSortItems();
    }
    
    /**
     * Filters and sorts the shared items based on current filters
     */
    function filterAndSortItems() {
        const type = filterType.value;
        const sort = sortBy.value;
        const searchTerm = sharedSearch.value.toLowerCase();
        
        // Filter items
        filteredItems = allSharedItems.filter(item => {
            // Filter by type
            if (type !== 'all' && item.type !== type) return false;
            
            // Filter by search term
            const nameMatch = item.name.toLowerCase().includes(searchTerm);
            return nameMatch;
        });
        
        // Sort items
        filteredItems.sort((a, b) => {
            if (sort === 'name') {
                return a.name.localeCompare(b.name);
            } else if (sort === 'date') {
                return new Date(b.dateShared) - new Date(a.dateShared);
            } else if (sort === 'expiration') {
                // Handle null expiration dates (items without expiration come last)
                if (!a.expiration && !b.expiration) return 0;
                if (!a.expiration) return 1;
                if (!b.expiration) return -1;
                return new Date(a.expiration) - new Date(b.expiration);
            }
            return 0;
        });
        
        // Display filtered and sorted items
        displaySharedItems();
    }
    
    /**
     * Displays the filtered and sorted shared items
     */
    function displaySharedItems() {
        // Clear the list
        sharedItemsList.innerHTML = '';
        
        // Show empty state if no items
        if (filteredItems.length === 0) {
            emptySharedState.style.display = 'flex';
            document.querySelector('.shared-list-container').style.display = 'none';
            return;
        }
        
        // Hide empty state and show table
        emptySharedState.style.display = 'none';
        document.querySelector('.shared-list-container').style.display = 'block';
        
        // Add items to the list
        filteredItems.forEach(item => {
            const row = document.createElement('tr');
            
            // Icon and name
            const nameCell = document.createElement('td');
            nameCell.className = 'shared-item-name';
            const icon = document.createElement('span');
            icon.className = 'item-icon';
            icon.textContent = item.type === 'file' ? '📄' : '📁';
            const name = document.createElement('span');
            name.textContent = item.name;
            nameCell.appendChild(icon);
            nameCell.appendChild(name);
            
            // Type
            const typeCell = document.createElement('td');
            typeCell.textContent = item.type === 'file' ? translate('shared_typeFile', 'File') : translate('shared_typeFolder', 'Folder');
            
            // Date shared
            const dateCell = document.createElement('td');
            dateCell.textContent = formatDate(item.dateShared);
            
            // Expiration
            const expirationCell = document.createElement('td');
            expirationCell.textContent = item.expiration ? formatDate(item.expiration) : translate('shared_noExpiration', 'No expiration');
            
            // Permissions
            const permissionsCell = document.createElement('td');
            const permissions = [];
            if (item.permissions.read) permissions.push(translate('share_permissionRead', 'Read'));
            if (item.permissions.write) permissions.push(translate('share_permissionWrite', 'Write'));
            if (item.permissions.reshare) permissions.push(translate('share_permissionReshare', 'Reshare'));
            permissionsCell.textContent = permissions.join(', ');
            
            // Password
            const passwordCell = document.createElement('td');
            passwordCell.textContent = item.password ? translate('shared_hasPassword', 'Yes') : translate('shared_noPassword', 'No');
            
            // Actions
            const actionsCell = document.createElement('td');
            actionsCell.className = 'shared-item-actions';
            
            // Edit button
            const editBtn = document.createElement('button');
            editBtn.className = 'action-btn edit-btn';
            editBtn.innerHTML = '<span class="action-icon">✏️</span>';
            editBtn.title = translate('shared_editShare', 'Edit Share');
            editBtn.addEventListener('click', () => openShareDialog(item));
            
            // Notify button
            const notifyBtn = document.createElement('button');
            notifyBtn.className = 'action-btn notify-btn';
            notifyBtn.innerHTML = '<span class="action-icon">📧</span>';
            notifyBtn.title = translate('shared_notifyShare', 'Notify Someone');
            notifyBtn.addEventListener('click', () => openNotificationDialog(item));
            
            // Copy link button
            const copyBtn = document.createElement('button');
            copyBtn.className = 'action-btn copy-btn';
            copyBtn.innerHTML = '<span class="action-icon">📋</span>';
            copyBtn.title = translate('shared_copyLink', 'Copy Link');
            copyBtn.addEventListener('click', () => {
                navigator.clipboard.writeText(item.url)
                    .then(() => showNotification(translate('shared_linkCopied', 'Link copied to clipboard!')))
                    .catch(err => showNotification(translate('shared_linkCopyFailed', 'Failed to copy link'), 'error'));
            });
            
            // Remove button
            const removeBtn = document.createElement('button');
            removeBtn.className = 'action-btn remove-btn';
            removeBtn.innerHTML = '<span class="action-icon">🗑️</span>';
            removeBtn.title = translate('shared_removeShare', 'Remove Share');
            removeBtn.addEventListener('click', () => {
                currentSharedItem = item;
                removeSharedItem();
            });
            
            actionsCell.appendChild(editBtn);
            actionsCell.appendChild(notifyBtn);
            actionsCell.appendChild(copyBtn);
            actionsCell.appendChild(removeBtn);
            
            // Add cells to row
            row.appendChild(nameCell);
            row.appendChild(typeCell);
            row.appendChild(dateCell);
            row.appendChild(expirationCell);
            row.appendChild(permissionsCell);
            row.appendChild(passwordCell);
            row.appendChild(actionsCell);
            
            // Add row to table
            sharedItemsList.appendChild(row);
        });
    }
    
    /**
     * Opens the share dialog for the given item
     */
    function openShareDialog(item) {
        currentSharedItem = item;
        
        // Set dialog content
        shareDialogIcon.textContent = item.type === 'file' ? '📄' : '📁';
        shareDialogName.textContent = item.name;
        shareLinkUrl.value = item.url;
        
        // Set permissions
        permissionRead.checked = item.permissions.read;
        permissionWrite.checked = item.permissions.write;
        permissionReshare.checked = item.permissions.reshare;
        
        // Set password
        enablePassword.checked = !!item.password;
        sharePassword.disabled = !enablePassword.checked;
        sharePassword.value = item.password || '';
        
        // Set expiration
        enableExpiration.checked = !!item.expiration;
        shareExpiration.disabled = !enableExpiration.checked;
        shareExpiration.value = item.expiration ? new Date(item.expiration).toISOString().split('T')[0] : '';
        
        // Show dialog
        shareDialog.classList.add('active');
    }
    
    /**
     * Closes the share dialog
     */
    function closeShareDialog() {
        shareDialog.classList.remove('active');
        currentSharedItem = null;
    }
    
    /**
     * Opens the notification dialog for the given item
     */
    function openNotificationDialog(item) {
        currentSharedItem = item;
        
        // Set dialog content
        notifyDialogIcon.textContent = item.type === 'file' ? '📄' : '📁';
        notifyDialogName.textContent = item.name;
        notificationEmail.value = '';
        notificationMessage.value = '';
        
        // Show dialog
        notificationDialog.classList.add('active');
    }
    
    /**
     * Closes the notification dialog
     */
    function closeNotificationDialog() {
        notificationDialog.classList.remove('active');
        currentSharedItem = null;
    }
    
    /**
     * Copies the current share link to clipboard
     */
    function copyShareLink() {
        navigator.clipboard.writeText(shareLinkUrl.value)
            .then(() => showNotification(translate('shared_linkCopied', 'Link copied to clipboard!')))
            .catch(err => showNotification(translate('shared_linkCopyFailed', 'Failed to copy link'), 'error'));
    }
    
    /**
     * Generates a random password for the share
     */
    function generatePassword() {
        const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*';
        let password = '';
        for (let i = 0; i < 12; i++) {
            password += chars.charAt(Math.floor(Math.random() * chars.length));
        }
        sharePassword.value = password;
        enablePassword.checked = true;
        sharePassword.disabled = false;
    }
    
    /**
     * Updates the current shared item with new settings
     */
    function updateSharedItem() {
        if (!currentSharedItem) return;
        
        // Get updated settings
        const permissions = {
            read: permissionRead.checked,
            write: permissionWrite.checked,
            reshare: permissionReshare.checked
        };
        
        const password = enablePassword.checked ? sharePassword.value : null;
        const expiration = enableExpiration.checked ? shareExpiration.value : null;
        
        // Update the shared link
        updateSharedLink(currentSharedItem.id, {
            permissions,
            password,
            expiration: expiration ? new Date(expiration).toISOString() : null
        });
        
        // Reload items and close dialog
        loadSharedItems();
        closeShareDialog();
        
        // Show notification
        showNotification(translate('shared_itemUpdated', 'Share settings updated successfully'));
    }
    
    /**
     * Removes the current shared item
     */
    function removeSharedItem() {
        if (!currentSharedItem) return;
        
        // Remove the shared link
        removeSharedLink(currentSharedItem.id);
        
        // Reload items and close dialog if open
        loadSharedItems();
        closeShareDialog();
        
        // Show notification
        showNotification(translate('shared_itemRemoved', 'Share removed successfully'));
    }
    
    /**
     * Sends a notification email for the current shared item
     */
    function sendNotification() {
        if (!currentSharedItem) return;
        
        const email = notificationEmail.value.trim();
        const message = notificationMessage.value.trim();
        
        // Validate email
        if (!email || !validateEmail(email)) {
            showNotification(translate('shared_invalidEmail', 'Please enter a valid email address'), 'error');
            return;
        }
        
        // Send notification
        sendShareNotification(currentSharedItem.id, email, message)
            .then(() => {
                closeNotificationDialog();
                showNotification(translate('shared_notificationSent', 'Notification sent successfully'));
            })
            .catch(error => {
                showNotification(translate('shared_notificationFailed', 'Failed to send notification'), 'error');
            });
    }
    
    /**
     * Shows a notification banner with the given message
     */
    function showNotification(message, type = 'success') {
        notificationBannerMessage.textContent = message;
        notificationBanner.className = 'notification-banner active ' + type;
        
        // Auto-hide after 5 seconds
        setTimeout(() => {
            notificationBanner.classList.remove('active');
        }, 5000);
    }
    
    /**
     * Validates an email address
     */
    function validateEmail(email) {
        const re = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return re.test(email);
    }
    
    /**
     * Formats a date string to a user-friendly format
     */
    function formatDate(dateString) {
        const options = { year: 'numeric', month: 'short', day: 'numeric' };
        return new Date(dateString).toLocaleDateString(undefined, options);
    }
});