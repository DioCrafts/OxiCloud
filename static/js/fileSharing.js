/**
 * OxiCloud - File Sharing Module
 * This file handles file sharing functionality (shared links, permissions, etc.)
 */

// File Sharing Module
const fileSharing = {
    /**
     * Generate a shared link for a file or folder
     * @param {string} itemId - ID of the file or folder
     * @param {string} itemType - Type ('file' or 'folder')
     * @param {Object} options - Sharing options (password, expiration, etc.)
     * @returns {Object} - Shared link information
     */
    generateSharedLink(itemId, itemType, options = {}) {
        try {
            // In a real implementation, this would be a call to the backend
            // But for now, we'll simulate it with a mock response
            
            // Default options
            const defaultOptions = {
                password: null,
                expirationDate: null,
                permissions: {
                    read: true,
                    write: false,
                    reshare: false
                }
            };
            
            // Merge options
            const finalOptions = { ...defaultOptions, ...options };
            
            // Generate a mock link (would normally come from server)
            const linkId = Math.random().toString(36).substring(2, 15);
            const shareToken = Math.random().toString(36).substring(2, 20);
            const baseUrl = window.location.origin;
            const sharedUrl = `${baseUrl}/s/${shareToken}`;
            
            // Create expiration date if set
            let expiresAt = null;
            if (finalOptions.expirationDate) {
                expiresAt = new Date(finalOptions.expirationDate);
            }
            
            // Create a mock response that matches what we'd expect from the server
            const response = {
                id: linkId,
                type: itemType,
                itemId: itemId,
                url: sharedUrl,
                token: shareToken,
                password_protected: !!finalOptions.password,
                expires_at: expiresAt ? expiresAt.toISOString() : null,
                permissions: finalOptions.permissions,
                created_at: new Date().toISOString(),
                created_by: {
                    id: "current-user-id", // Would be the actual user ID
                    username: "current-user" // Would be the actual username
                },
                access_count: 0,
                // Add some UI friendly properties for shared.js compatibility
                name: options.name || "Shared Item",
                dateShared: new Date().toISOString(),
                expiration: expiresAt ? expiresAt.toISOString() : null,
                password: finalOptions.password
            };
            
            // In a real implementation, we would store this link in localStorage for now
            // until backend implementation is ready
            this.saveSharedLink(response);
            
            // Return the "response" as if it came from the server
            return response;
        } catch (error) {
            console.error('Error generating shared link:', error);
            throw error;
        }
    },
    
    /**
     * Save a shared link to localStorage (temporary storage until backend is ready)
     * @param {Object} linkData - Shared link data
     */
    saveSharedLink(linkData) {
        try {
            // Get existing shared links
            const existingLinks = JSON.parse(localStorage.getItem('oxicloud_shared_links') || '[]');
            
            // Add new link
            existingLinks.push(linkData);
            
            // Save back to localStorage
            localStorage.setItem('oxicloud_shared_links', JSON.stringify(existingLinks));
        } catch (error) {
            console.error('Error saving shared link to local storage:', error);
        }
    },
    
    /**
     * Remove a shared link
     * @param {string} linkId - ID of the shared link to remove
     * @returns {Promise<boolean>} - Success status
     */
    removeSharedLink(linkId) {
        try {
            // Get existing shared links
            const existingLinks = JSON.parse(localStorage.getItem('oxicloud_shared_links') || '[]');
            
            // Filter out the link to remove
            const updatedLinks = existingLinks.filter(link => link.id !== linkId);
            
            // Save back to localStorage
            localStorage.setItem('oxicloud_shared_links', JSON.stringify(updatedLinks));
            
            // Removed network delay simulation
            
            return true;
        } catch (error) {
            console.error('Error removing shared link:', error);
            return false;
        }
    },
    
    /**
     * Update a shared link's properties
     * @param {string} linkId - ID of the shared link to update
     * @param {Object} updateData - Properties to update
     * @returns {Promise<Object>} - Updated shared link
     */
    updateSharedLink(linkId, updateData) {
        try {
            // Get existing shared links
            const existingLinks = JSON.parse(localStorage.getItem('oxicloud_shared_links') || '[]');
            
            // Find the link to update
            const linkIndex = existingLinks.findIndex(link => link.id === linkId);
            if (linkIndex === -1) {
                throw new Error('Shared link not found');
            }
            
            // Update link data
            existingLinks[linkIndex] = {
                ...existingLinks[linkIndex],
                ...updateData,
                updated_at: new Date().toISOString()
            };
            
            // Save back to localStorage
            localStorage.setItem('oxicloud_shared_links', JSON.stringify(existingLinks));
            
            // Removed network delay simulation
            
            return existingLinks[linkIndex];
        } catch (error) {
            console.error('Error updating shared link:', error);
            throw error;
        }
    },
    
    /**
     * Get all shared links for the current user
     * @returns {Promise<Array>} - Array of shared links
     */
    getSharedLinks() {
        try {
            // Get shared links from localStorage
            const links = JSON.parse(localStorage.getItem('oxicloud_shared_links') || '[]');
            
            // Removed network delay simulation
            
            return links;
        } catch (error) {
            console.error('Error getting shared links:', error);
            return [];
        }
    },
    
    /**
     * Get shared links for a specific item
     * @param {string} itemId - ID of the file or folder
     * @param {string} itemType - Type ('file' or 'folder')
     * @returns {Promise<Array>} - Array of shared links for the item
     */
    getSharedLinksForItem(itemId, itemType) {
        try {
            // Get all shared links
            const allLinks = this.getSharedLinks();
            
            // Filter by item ID and type
            return allLinks.filter(link => link.itemId === itemId && link.type === itemType);
        } catch (error) {
            console.error('Error getting shared links for item:', error);
            return [];
        }
    },
    
    /**
     * Check if an item has any shared links
     * @param {string} itemId - ID of the file or folder
     * @param {string} itemType - Type ('file' or 'folder')
     * @returns {Promise<boolean>} - True if the item has shared links
     */
    hasSharedLinks(itemId, itemType) {
        const links = this.getSharedLinksForItem(itemId, itemType);
        return links.length > 0;
    },
    
    /**
     * Copy a shared link to clipboard
     * @param {string} url - URL to copy
     * @returns {boolean} - Success status
     */
    copyLinkToClipboard(url) {
        try {
            navigator.clipboard.writeText(url);
            window.ui.showNotification('Link copied', 'Link copied to clipboard');
            return true;
        } catch (error) {
            console.error('Error copying to clipboard:', error);
            window.ui.showNotification('Error', 'Could not copy link');
            return false;
        }
    },
    
    /**
     * Format expiration date for display
     * @param {string} dateString - ISO date string
     * @returns {string} - Formatted date string
     */
    formatExpirationDate(dateString) {
        if (!dateString) return 'No expiration';
        
        const date = new Date(dateString);
        return date.toLocaleDateString() + ' ' + date.toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'});
    },

    /**
     * Send a notification about a shared resource
     * @param {string} shareUrl - The URL of the shared resource
     * @param {string} recipientEmail - Email of the recipient
     * @param {string} message - Optional message to include
     * @returns {boolean} - Success status
     */
    sendShareNotification(shareUrl, recipientEmail, message = '') {
        try {
            // In a real implementation, this would call the backend
            // For now, we'll just simulate a successful notification
            console.log(`Share notification for ${shareUrl} sent to ${recipientEmail}`);
            console.log(`Message: ${message || 'No message included'}`);
            
            // Simulate network delay
            //await new Promise(resolve => setTimeout(resolve, 800));
            
            window.ui.showNotification('Notification sent', `Notification sent to ${recipientEmail}`);
            return true;
        } catch (error) {
            console.error('Error sending share notification:', error);
            window.ui.showNotification('Error', 'Could not send notification');
            return false;
        }
    },
    
    /**
     * Initialize file sharing event listeners and UI elements
     */
    init() {
        // This will be called by the app.js initialization
        console.log('File sharing module initialized');
        
        // Add "Shared" view event listeners
        document.querySelectorAll('.nav-item').forEach(item => {
            if (item.querySelector('span').getAttribute('data-i18n') === 'nav.shared') {
                item.addEventListener('click', () => {
                    if (window.switchToSharedView) {
                        window.switchToSharedView();
                    }
                });
            }
        });
    }
};

/**
 * Get all shared links 
 * @returns {Array} Array of shared links
 */
function getSharedLinks() {
    try {
        return JSON.parse(localStorage.getItem('oxicloud_shared_links') || '[]');
    } catch (error) {
        console.error('Error getting shared links:', error);
        return [];
    }
}

/**
 * Update a shared link
 * @param {string} linkId - ID of the link to update
 * @param {Object} updateData - Data to update
 * @returns {boolean} Success status
 */
function updateSharedLink(linkId, updateData) {
    try {
        const links = getSharedLinks();
        const index = links.findIndex(link => link.id === linkId);
        if (index === -1) return false;
        
        links[index] = {...links[index], ...updateData};
        localStorage.setItem('oxicloud_shared_links', JSON.stringify(links));
        return true;
    } catch (error) {
        console.error('Error updating shared link:', error);
        return false;
    }
}

/**
 * Remove a shared link
 * @param {string} linkId - ID of the link to remove
 * @returns {boolean} Success status
 */
function removeSharedLink(linkId) {
    try {
        const links = getSharedLinks();
        const filteredLinks = links.filter(link => link.id !== linkId);
        localStorage.setItem('oxicloud_shared_links', JSON.stringify(filteredLinks));
        return true;
    } catch (error) {
        console.error('Error removing shared link:', error);
        return false;
    }
}

/**
 * Send a notification about a shared link
 * @param {string} linkId - ID of the link
 * @param {string} email - Recipient email
 * @param {string} message - Optional message
 * @returns {Promise<boolean>} Success status
 */
function sendShareNotification(linkId, email, message = '') {
    return new Promise((resolve) => {
        console.log(`Notification for link ${linkId} sent to ${email}`);
        console.log(`Message: ${message || 'No message'}`);
        setTimeout(() => resolve(true), 500);
    });
}

/**
 * Translate text using i18n if available
 * @param {string} key - Translation key
 * @param {string} defaultText - Default text if translation not found
 * @returns {string} Translated text
 */
function translate(key, defaultText) {
    if (window.i18n && window.i18n.t) {
        return window.i18n.t(key, defaultText);
    }
    return defaultText;
}

/**
 * Initialize i18n module
 */
function initializeI18n() {
    if (window.i18n && window.i18n.init) {
        window.i18n.init();
    }
}

// Expose functions globally
window.getSharedLinks = getSharedLinks;
window.updateSharedLink = updateSharedLink;
window.removeSharedLink = removeSharedLink;
window.sendShareNotification = sendShareNotification;
window.translate = translate;
window.initializeI18n = initializeI18n;

// Expose file sharing module globally
window.fileSharing = fileSharing;