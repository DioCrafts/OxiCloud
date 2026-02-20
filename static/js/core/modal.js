/**
 * Modal System for OxiCloud
 * Provides modern, styled modals to replace browser prompts/alerts
 */

const Modal = {
    // Modal element references
    overlay: null,
    container: null,
    icon: null,
    title: null,
    label: null,
    input: null,
    cancelBtn: null,
    confirmBtn: null,
    closeBtn: null,
    
    // Current callback
    onConfirm: null,
    onCancel: null,
    
    // Rename mode: select only name without extension
    _selectNameOnly: false,
    
    /**
     * Initialize modal system
     */
    init() {
        this.overlay = document.getElementById('input-modal');
        if (!this.overlay) {
            console.warn('Modal overlay not found');
            return;
        }
        
        this.icon = document.getElementById('modal-icon');
        this.title = document.getElementById('modal-title');
        this.label = document.getElementById('modal-label');
        this.input = document.getElementById('modal-input');
        this.cancelBtn = document.getElementById('modal-cancel-btn');
        this.confirmBtn = document.getElementById('modal-confirm-btn');
        this.closeBtn = document.getElementById('modal-close-btn');
        
        // Event listeners
        this.cancelBtn.addEventListener('click', () => this.close(false));
        this.closeBtn.addEventListener('click', () => this.close(false));
        this.confirmBtn.addEventListener('click', () => this.confirm());
        
        // Close on overlay click
        this.overlay.addEventListener('click', (e) => {
            if (e.target === this.overlay) {
                this.close(false);
            }
        });
        
        // Handle Enter and Escape keys
        this.input.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                e.preventDefault();
                this.confirm();
            } else if (e.key === 'Escape') {
                this.close(false);
            }
        });
    },
    
    /**
     * Show input modal (replacement for prompt())
     * @param {Object} options - Modal configuration
     * @param {string} options.title - Modal title
     * @param {string} options.label - Input label
     * @param {string} options.placeholder - Input placeholder
     * @param {string} options.value - Initial input value
     * @param {string} options.icon - Font Awesome icon class (e.g., 'fa-folder-plus')
     * @param {string} options.confirmText - Confirm button text
     * @param {string} options.cancelText - Cancel button text
     * @returns {Promise<string|null>} - Resolves with input value or null if cancelled
     */
    prompt(options = {}) {
        return new Promise((resolve) => {
            const {
                title = 'Input',
                label = '',
                placeholder = '',
                value = '',
                icon = 'fa-keyboard',
                confirmText = null,
                cancelText = null
            } = options;
            
            // Set modal content - update the icon
            const iconContainer = document.querySelector('.modal-icon');
            if (iconContainer) {
                // Replace contents with a fresh <i> that icons.js will convert
                iconContainer.innerHTML = `<i id="modal-icon" class="fas ${icon}"></i>`;
                this.icon = document.getElementById('modal-icon');
                // Let icons.js convert it to SVG
                if (window.replaceIconsInElement) {
                    window.replaceIconsInElement(iconContainer);
                    this.icon = document.getElementById('modal-icon');
                }
            }
            this.title.textContent = title;
            this.label.textContent = label;
            this.input.placeholder = placeholder;
            this.input.value = value;
            
            // Set button text (use i18n if available)
            if (confirmText) {
                this.confirmBtn.textContent = confirmText;
            } else if (window.i18n) {
                this.confirmBtn.textContent = window.i18n.t('actions.confirm');
            }
            
            if (cancelText) {
                this.cancelBtn.textContent = cancelText;
            } else if (window.i18n) {
                this.cancelBtn.textContent = window.i18n.t('actions.cancel');
            }
            
            // Set callbacks
            this.onConfirm = () => {
                const inputValue = this.input.value.trim();
                resolve(inputValue || null);
            };
            this.onCancel = () => resolve(null);
            
            // Show modal
            this.open();
        });
    },
    
    /**
     * Show modal for creating new folder
     * @returns {Promise<string|null>}
     */
    promptNewFolder() {
        const t = window.i18n ? window.i18n.t.bind(window.i18n) : (k) => k;
        
        return this.prompt({
            title: t('dialogs.new_folder_title') || 'New folder',
            label: t('dialogs.folder_name') || 'Folder name',
            placeholder: t('dialogs.folder_placeholder') || 'My folder',
            icon: 'fa-folder-plus',
            confirmText: t('actions.create') || 'Create'
        });
    },
    
    /**
     * Show modal for renaming
     * @param {string} currentName - Current name of file/folder
     * @param {boolean} isFolder - Whether it's a folder
     * @returns {Promise<string|null>}
     */
    promptRename(currentName, isFolder = false) {
        const t = window.i18n ? window.i18n.t.bind(window.i18n) : (k) => k;
        
        // For files, we want to select only the name part (without extension)
        this._selectNameOnly = !isFolder;
        
        return this.prompt({
            title: t('dialogs.rename_title') || 'Renombrar',
            label: t('dialogs.new_name') || 'Nuevo nombre',
            placeholder: '',
            value: currentName,
            icon: isFolder ? 'fa-folder' : 'fa-file',
            confirmText: t('actions.rename') || 'Renombrar'
        });
    },
    
    /**
     * Open the modal
     */
    open() {
        if (!this.overlay) return;
        
        // Show overlay
        this.overlay.style.display = 'flex';
        
        // Trigger animation
        requestAnimationFrame(() => {
            this.overlay.classList.add('active');
        });
        
        // Focus input after animation
        setTimeout(() => {
            this.input.focus();
            
            if (this._selectNameOnly) {
                // Select only the filename, excluding the extension
                const value = this.input.value;
                const lastDot = value.lastIndexOf('.');
                if (lastDot > 0) {
                    this.input.setSelectionRange(0, lastDot);
                } else {
                    this.input.select();
                }
                this._selectNameOnly = false;
            } else {
                this.input.select();
            }
        }, 100);
    },
    
    /**
     * Close the modal
     * @param {boolean} confirmed - Whether the action was confirmed
     */
    close(confirmed = false) {
        if (!this.overlay) return;
        
        this.overlay.classList.remove('active');
        
        setTimeout(() => {
            this.overlay.style.display = 'none';
            
            if (!confirmed && this.onCancel) {
                this.onCancel();
            }
            
            // Clear callbacks
            this.onConfirm = null;
            this.onCancel = null;
        }, 200);
    },
    
    /**
     * Confirm the action
     */
    confirm() {
        if (this.onConfirm) {
            this.onConfirm();
        }
        this.close(true);
    }
};

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    Modal.init();
});

// Export for use in other modules
window.Modal = Modal;
