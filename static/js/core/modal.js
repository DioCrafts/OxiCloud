import { i18n } from './i18n.js';
import { replaceIconsInElement } from './icons.js';

/**
 * Modal System for OxiCloud
 * Provides modern, styled modals to replace browser prompts/alerts
 */

const Modal = {
    // Modal element references
    /** @private @type {HTMLElement | null} */
    overlay: null,
    /** @private @type {HTMLElement | null} */
    icon: null,
    /** @private @type {HTMLElement | null} */
    title: null,
    /** @private @type {HTMLElement | null} */
    label: null,
    /** @private @type {HTMLInputElement | null} */
    input: null,
    /** @private @type {HTMLButtonElement | null} */
    cancelBtn: null,
    /** @private @type {HTMLButtonElement | null} */
    confirmBtn: null,
    /** @private @type {HTMLButtonElement | null} */
    closeBtn: null,

    // Current callback
    /** @private @type {Function | null} */
    onConfirm: null,
    /** @private @type {Function | null} */
    onCancel: null,

    /** @private @type {((value: string) => Promise<void>) | null} */
    _action: null,

    /** @private @type {HTMLElement | null} */
    errorEl: null,

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
        this.input = /** @type {HTMLInputElement} */ (document.getElementById('modal-input'));
        this.cancelBtn = /** @type {HTMLButtonElement} */ (document.getElementById('modal-cancel-btn'));
        this.confirmBtn = /** @type {HTMLButtonElement} */ (document.getElementById('modal-confirm-btn'));
        this.closeBtn = /** @type {HTMLButtonElement} */ (document.getElementById('modal-close-btn'));

        // Event listeners
        this.errorEl = document.getElementById('modal-error');

        this.cancelBtn?.addEventListener('click', () => this.close(false));
        this.closeBtn?.addEventListener('click', () => this.close(false));
        this.confirmBtn?.addEventListener('click', () => this.confirm());

        // Close on overlay click
        this.overlay.addEventListener('click', (e) => {
            if (e.target === this.overlay) {
                this.close(false);
            }
        });

        // Clear inline error as soon as the user starts typing
        this.input?.addEventListener('input', () => this.clearError());

        // Handle Enter and Escape keys
        this.input?.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                e.preventDefault();
                this.confirm();
            } else if (e.key === 'Escape') {
                this.close(false);
            }
        });
    },

    /** @param {string} message */
    showError(message) {
        this.input?.classList.add('modal-input--error');
        if (this.errorEl) {
            this.errorEl.textContent = message;
            this.errorEl.classList.remove('hidden');
        }
    },

    clearError() {
        this.input?.classList.remove('modal-input--error');
        if (this.errorEl) {
            this.errorEl.textContent = '';
            this.errorEl.classList.add('hidden');
        }
    },

    /**
     * Show input modal (replacement for prompt())
     * @param {Object} options - Modal configuration
     * @param {string} [options.title] - Modal title
     * @param {string} [options.label] - Input label
     * @param {string} [options.placeholder] - Input placeholder
     * @param {string} [options.value] - Initial input value
     * @param {string} [options.icon] - Font Awesome icon class (e.g., 'fa-folder-plus')
     * @param {string} [options.confirmText] - Confirm button text
     * @param {string} [options.cancelText] - Cancel button text
     * @param {(value: string) => Promise<void>} [options.action] - Async action called on confirm.
     *   Throw an Error to keep the modal open and display the error message inline.
     *   When omitted the modal resolves immediately with the input value (legacy behaviour).
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
                cancelText = null,
                action = null
            } = options;

            // Set modal content - update the icon
            const iconContainer = document.querySelector('.modal-icon');
            if (iconContainer) {
                // Replace contents with a fresh <i> that icons.js will convert
                iconContainer.innerHTML = `<i id="modal-icon" class="fas ${icon}"></i>`;
                this.icon = document.getElementById('modal-icon');
                // Let icons.js convert it to SVG
                if (replaceIconsInElement) {
                    replaceIconsInElement(iconContainer);
                    this.icon = document.getElementById('modal-icon');
                }
            }
            this.title.textContent = title;
            this.label.textContent = label;
            this.input.placeholder = placeholder;
            this.input.value = value;

            if (confirmText) {
                this.confirmBtn.textContent = confirmText;
            } else {
                this.confirmBtn.textContent = i18n.t('actions.confirm');
            }

            if (cancelText) {
                this.cancelBtn.textContent = cancelText;
            } else {
                this.cancelBtn.textContent = i18n.t('actions.cancel');
            }

            this._action = action;
            this.clearError();

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
     * @param {(value: string) => Promise<void>} [action]
     * @returns {Promise<string|null>}
     */
    promptNewFolder(action = null) {
        return this.prompt({
            title: i18n.t('dialogs.new_folder_title'),
            label: i18n.t('dialogs.folder_name'),
            placeholder: i18n.t('dialogs.folder_placeholder'),
            icon: 'fa-folder-plus',
            confirmText: i18n.t('actions.create'),
            action
        });
    },

    /**
     * Show modal for renaming
     * @param {string} currentName - Current name of file/folder
     * @param {boolean} isFolder - Whether it's a folder
     * @param {(value: string) => Promise<void>} [action]
     * @returns {Promise<string|null>}
     */
    promptRename(currentName, isFolder = false, action = null) {
        this._selectNameOnly = !isFolder;

        return this.prompt({
            title: i18n.t('dialogs.rename_title'),
            label: i18n.t('dialogs.new_name'),
            placeholder: '',
            value: currentName,
            icon: isFolder ? 'fa-folder' : 'fa-file',
            confirmText: i18n.t('actions.rename'),
            action
        });
    },

    /**
     * Open the modal
     */
    open() {
        if (!this.overlay) return;

        this.confirmBtn.disabled = false;

        // Show overlay
        this.overlay.classList.remove('hidden');

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

        this.clearError();
        this._action = null;
        this.overlay.classList.remove('active');

        setTimeout(() => {
            this.overlay.classList.add('hidden');

            if (!confirmed && this.onCancel) {
                this.onCancel();
            }

            // Clear callbacks
            this.onConfirm = null;
            this.onCancel = null;
        }, 200);
    },

    /**
     * Confirm the action. When an async action is set, the modal stays open
     * until it resolves — closing only on success, showing the error inline on failure.
     */
    async confirm() {
        if (!this._action) {
            if (this.onConfirm) this.onConfirm();
            this.close(true);
            return;
        }

        const inputValue = this.input.value.trim();
        if (!inputValue) return;

        this.clearError();
        this.confirmBtn.disabled = true;

        try {
            await this._action(inputValue);
            if (this.onConfirm) this.onConfirm();
            this.close(true);
        } catch (e) {
            this.showError(/** @type {Error} */ (e).message || 'An error occurred');
            this.confirmBtn.disabled = false;
            this.input.focus();
        }
    }
};

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    Modal.init();
});

// Export for use in other modules
export { Modal };
