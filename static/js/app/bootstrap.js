/**
 * OxiCloud - App bootstrap
 * Isolated startup trigger for the main application initializer.
 */
import { initApp } from './main.js';

if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initApp);
} else {
    initApp();
}
