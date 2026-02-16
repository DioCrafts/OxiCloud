/**
 * OxiCloud - File Sharing Module
 * All operations go through the backend API at /api/shares.
 * No localStorage is used for share data.
 */

const fileSharing = {
    /** Auth header helper */
    _headers(json = true) {
        const h = {};
        const token = localStorage.getItem('oxicloud_token');
        if (token) h['Authorization'] = `Bearer ${token}`;
        if (json) h['Content-Type'] = 'application/json';
        return h;
    },

    /**
     * Create a shared link via backend API
     * @param {string} itemId - ID of the file or folder
     * @param {string} itemType - 'file' or 'folder'
     * @param {Object} options - { name, password, expirationDate, permissions }
     * @returns {Promise<Object>} ShareDto from backend
     */
    async createSharedLink(itemId, itemType, options = {}) {
        const body = {
            item_id: itemId,
            item_name: options.name || null,
            item_type: itemType,
            password: options.password || null,
            expires_at: options.expirationDate
                ? Math.floor(new Date(options.expirationDate).getTime() / 1000)
                : null,
            permissions: options.permissions || { read: true, write: false, reshare: false }
        };

        const res = await fetch('/api/shares', {
            method: 'POST',
            headers: this._headers(),
            body: JSON.stringify(body)
        });

        if (!res.ok) {
            const err = await res.json().catch(() => ({}));
            throw new Error(err.error || `Server error ${res.status}`);
        }

        return await res.json();
    },

    /**
     * Get all shared links for the current user
     * @returns {Promise<Array>} Array of ShareDto
     */
    async getSharedLinks() {
        try {
            const res = await fetch('/api/shares?page=1&per_page=1000', {
                headers: this._headers(false)
            });
            if (!res.ok) return [];
            const data = await res.json();
            return data.items || [];
        } catch (error) {
            console.error('Error fetching shared links:', error);
            return [];
        }
    },

    /**
     * Get shared links for a specific item
     * @param {string} itemId
     * @param {string} itemType - 'file' or 'folder'
     * @returns {Promise<Array>} Filtered shares
     */
    async getSharedLinksForItem(itemId, itemType) {
        try {
            const all = await this.getSharedLinks();
            return all.filter(s => s.item_id === itemId && s.item_type === itemType);
        } catch (error) {
            console.error('Error getting shared links for item:', error);
            return [];
        }
    },

    /**
     * Check if an item has any shared links
     * @returns {Promise<boolean>}
     */
    async hasSharedLinks(itemId, itemType) {
        const links = await this.getSharedLinksForItem(itemId, itemType);
        return links.length > 0;
    },

    /**
     * Update a shared link
     * @param {string} shareId
     * @param {Object} updateData - { permissions, password, expires_at }
     * @returns {Promise<Object>} Updated ShareDto
     */
    async updateSharedLink(shareId, updateData) {
        const body = {};
        if (updateData.permissions) body.permissions = updateData.permissions;
        if (updateData.password !== undefined) body.password = updateData.password;
        if (updateData.expires_at !== undefined) body.expires_at = updateData.expires_at;

        const res = await fetch(`/api/shares/${shareId}`, {
            method: 'PUT',
            headers: this._headers(),
            body: JSON.stringify(body)
        });

        if (!res.ok) {
            const err = await res.json().catch(() => ({}));
            throw new Error(err.error || `Server error ${res.status}`);
        }

        return await res.json();
    },

    /**
     * Delete a shared link
     * @param {string} shareId
     * @returns {Promise<boolean>}
     */
    async removeSharedLink(shareId) {
        try {
            const res = await fetch(`/api/shares/${shareId}`, {
                method: 'DELETE',
                headers: this._headers(false)
            });
            return res.ok || res.status === 204;
        } catch (error) {
            console.error('Error removing shared link:', error);
            return false;
        }
    },

    /**
     * Copy a shared link to clipboard
     * @param {string} url
     */
    async copyLinkToClipboard(url) {
        try {
            await navigator.clipboard.writeText(url);
            window.ui.showNotification('Link copied', 'Link copied to clipboard');
            return true;
        } catch (error) {
            console.error('Error copying to clipboard:', error);
            window.ui.showNotification('Error', 'Could not copy link');
            return false;
        }
    },

    /**
     * Format expiration date for display (Unix timestamp in seconds or ISO string)
     * @param {number|string} value
     * @returns {string}
     */
    formatExpirationDate(value) {
        if (!value) return 'No expiration';
        return window.formatDateTime(value);
    },

    /**
     * Send a notification about a shared resource (stub — no backend endpoint yet)
     * @param {string} shareUrl
     * @param {string} recipientEmail
     * @param {string} message
     * @returns {Promise<boolean>}
     */
    async sendShareNotification(shareUrl, recipientEmail, message = '') {
        // TODO: implement backend endpoint for email notifications
        console.log(`Share notification for ${shareUrl} sent to ${recipientEmail}`);
        if (window.ui) {
            window.ui.showNotification('Notification sent', `Notification sent to ${recipientEmail}`);
        }
        return true;
    },

    /**
     * Initialize file sharing event listeners
     */
    init() {
        console.log('File sharing module initialized (API-backed)');
        document.querySelectorAll('.nav-item').forEach(item => {
            const span = item.querySelector('span');
            if (span && span.getAttribute('data-i18n') === 'nav.shared') {
                item.addEventListener('click', () => {
                    if (window.switchToSharedView) {
                        window.switchToSharedView();
                    }
                });
            }
        });
    }
};

// Expose module globally
window.fileSharing = fileSharing;

// Global convenience functions that delegate to the module
window.getSharedLinks = () => fileSharing.getSharedLinks();
window.updateSharedLink = (id, data) => fileSharing.updateSharedLink(id, data);
window.removeSharedLink = (id) => fileSharing.removeSharedLink(id);
window.sendShareNotification = (url, email, msg) => fileSharing.sendShareNotification(url, email, msg);