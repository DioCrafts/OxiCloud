/**
 * OxiCloud - App bootstrap
 * Isolated startup trigger for the main application initializer.
 */

if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
        if (typeof window.initApp === 'function') {
            window.initApp();
        }
    });
} else if (typeof window.initApp === 'function') {
    window.initApp();
}
