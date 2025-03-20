/**
 * OxiCloud Internationalization (i18n) Module
 * 
 * This module provides functionality for internationalization of the OxiCloud web interface.
 * It loads translations from the server and provides functions to translate keys.
 */

// Current locale code (default to browser locale if available, fallback to English)
let currentLocale = 
    (navigator.language && navigator.language.substring(0, 2)) || 
    (navigator.userLanguage && navigator.userLanguage.substring(0, 2)) || 
    'en';

// Supported locales
const supportedLocales = ['en', 'es'];

// Fallback to English if locale is not supported
if (!supportedLocales.includes(currentLocale)) {
    currentLocale = 'en';
}

// Cache for translations
const translations = {};

// Track missing keys to avoid repeated warnings
const missingKeys = {};

/**
 * Load translations for a specific locale
 * @param {string} locale - The locale code to load (e.g., 'en', 'es')
 * @returns {Promise<object>} - A promise that resolves to the translations object
 */
async function loadTranslations(locale) {
    // Check if already loaded
    if (translations[locale]) {
        return translations[locale];
    }
    
    try {
        // Vamos a cargar las traducciones estáticas con una ruta conocida
        // Try to fetch from static files first
        try {
            // Intentamos primero con la ruta /static/locales
            const localeData = await fetch(`/static/locales/${locale}.json`);
            if (localeData.ok) {
                translations[locale] = await localeData.json();
                return translations[locale];
            }
        } catch (staticError) {
            console.log('Static translations not available from /static/locales');
        }
        
        // Intentamos con la ruta /locales como alternativa
        try {
            const localeData = await fetch(`/locales/${locale}.json`);
            if (localeData.ok) {
                translations[locale] = await localeData.json();
                return translations[locale];
            }
        } catch (staticError) {
            console.log('Static translations not available from /locales');
        }
        
        // Only try API if static files failed
        try {
            // Check if AuthModule exists
            const token = window.AuthModule && typeof AuthModule.getToken === 'function' ? 
                AuthModule.getToken() : localStorage.getItem('oxicloud_token');
            
            const headers = token ? { 'Authorization': `Bearer ${token}` } : {};
            
            const response = await fetch(`/api/i18n/locales/${locale}`, { headers });
            if (response.ok) {
                const apiData = await response.json();
                if (apiData && Object.keys(apiData).length > 0) {
                    translations[locale] = apiData;
                    return translations[locale];
                }
            }
        } catch (apiError) {
            console.log('API translations not available');
        }
        
        // If we reach here, neither source worked
        // Return an empty object instead of failing
        console.warn(`Could not load translations for ${locale}, using empty translations`);
        translations[locale] = {};
        return translations[locale];
    } catch (error) {
        console.error('Error loading translations:', error);
        // Use empty translations instead of failing
        translations[locale] = {};
        return translations[locale];
    }
}

/**
 * Get a nested translation value
 * @param {object} obj - The translations object
 * @param {string} path - The dot-notation path to the translation
 * @returns {string|null} - The translation value or null if not found
 */
function getNestedValue(obj, path) {
    const keys = path.split('.');
    let current = obj;
    
    for (const key of keys) {
        if (current && typeof current === 'object' && key in current) {
            current = current[key];
        } else {
            return null;
        }
    }
    
    return typeof current === 'string' ? current : null;
}

/**
 * Translate a key to the current locale
 * @param {string} key - The translation key (dot notation, e.g., 'app.title')
 * @param {object} params - Parameters to replace in the translation (e.g., {name: 'John'})
 * @returns {string} - The translated string or the key itself if not found
 */
function t(key, params = {}) {
    // Si la clave es una placeholder específica para sharing.js,
    // devolvemos directamente una traducción aun sin cargar traducciones
    if (key === 'Enter user email') {
        return 'Enter user email';
    }
    
    // Get translation from cache
    const localeData = translations[currentLocale];
    if (!localeData) {
        // Translation not loaded yet, return key but only warn once
        if (!window.i18n_warned) {
            console.warn(`Translations for ${currentLocale} not loaded yet`);
            window.i18n_warned = true;
            
            // Reset the warning flag after 5 seconds to allow future warnings if needed
            setTimeout(() => {
                window.i18n_warned = false;
            }, 5000);
        }
        return key;
    }
    
    // Get the translation value
    const value = getNestedValue(localeData, key);
    if (!value) {
        // Try fallback to English
        if (currentLocale !== 'en' && translations['en']) {
            const fallbackValue = getNestedValue(translations['en'], key);
            if (fallbackValue) {
                return interpolate(fallbackValue, params);
            }
        }
        
        // Key not found, log only once per key to avoid console spam
        if (!missingKeys[key]) {
            console.warn(`Translation key not found: ${key}`);
            missingKeys[key] = true;
        }
        return key;
    }
    
    // Replace parameters
    return interpolate(value, params);
}

/**
 * Replace parameters in a translation string
 * @param {string} text - The translation string with placeholders
 * @param {object} params - The parameters to replace
 * @returns {string} - The interpolated string
 */
function interpolate(text, params) {
    return text.replace(/{{\s*([^}]+)\s*}}/g, (_, key) => {
        return params[key.trim()] !== undefined ? params[key.trim()] : `{{${key}}}`;
    });
}

/**
 * Change the current locale
 * @param {string} locale - The locale code to switch to
 * @returns {Promise<boolean>} - A promise that resolves to true if successful
 */
async function setLocale(locale) {
    if (!supportedLocales.includes(locale)) {
        console.error(`Locale not supported: ${locale}`);
        return false;
    }
    
    // Load translations if not loaded yet
    if (!translations[locale]) {
        await loadTranslations(locale);
    }
    
    // Update current locale
    currentLocale = locale;
    
    // Save locale preference
    localStorage.setItem('oxicloud-locale', locale);
    
    // Trigger an event for components to update
    window.dispatchEvent(new CustomEvent('localeChanged', { detail: { locale } }));
    
    // Update all elements with data-i18n attribute
    translatePage();
    
    return true;
}

/**
 * Initialize the i18n system
 * @returns {Promise<void>}
 */
async function initI18n() {
    // Load saved locale preference
    const savedLocale = localStorage.getItem('oxicloud-locale');
    if (savedLocale && supportedLocales.includes(savedLocale)) {
        currentLocale = savedLocale;
    }
    
    try {
        // Try to load all supported locales in parallel
        const loadingPromises = supportedLocales.map(locale => 
            loadTranslations(locale).catch(err => {
                console.warn(`Failed to load translations for ${locale}:`, err);
                return {}; // Return empty object on error to prevent breaking
            })
        );
        
        await Promise.allSettled(loadingPromises);
        
        // Make sure English is always loaded as fallback
        if (!translations['en'] || Object.keys(translations['en']).length === 0) {
            console.warn('English translations not loaded properly, retrying...');
            // Try all paths one more time
            try {
                const enData = await fetch(`/static/locales/en.json`);
                if (enData.ok) {
                    translations['en'] = await enData.json();
                }
            } catch (e) {
                try {
                    const enData = await fetch(`/locales/en.json`);
                    if (enData.ok) {
                        translations['en'] = await enData.json();
                    }
                } catch (e2) {
                    try {
                        const enData = await fetch(`/api/i18n/locales/en`);
                        if (enData.ok) {
                            translations['en'] = await enData.json();
                        }
                    } catch (e3) {
                        console.error('Failed to load English translations after multiple attempts');
                        translations['en'] = {}; // Provide empty translations to avoid errors
                    }
                }
            }
        }
        
        console.log(`I18n initialized with locale: ${currentLocale}`);
    } catch (e) {
        console.error("Failed to load translations:", e);
        // Continue even if translations fail to load
        translations['en'] = translations['en'] || {};
    }
    
    // Always translate the page regardless of whether translations loaded
    translatePage();
}

/**
 * Translate all elements with data-i18n attribute
 */
function translatePage() {
    document.querySelectorAll('[data-i18n]').forEach(element => {
        const key = element.getAttribute('data-i18n');
        element.textContent = t(key);
    });
    
    document.querySelectorAll('[data-i18n-placeholder]').forEach(element => {
        const key = element.getAttribute('data-i18n-placeholder');
        element.placeholder = t(key);
    });
    
    document.querySelectorAll('[data-i18n-title]').forEach(element => {
        const key = element.getAttribute('data-i18n-title');
        element.title = t(key);
    });
}

/**
 * Get current locale
 * @returns {string} - The current locale code
 */
function getCurrentLocale() {
    return currentLocale;
}

/**
 * Get list of supported locales
 * @returns {Array<string>} - Array of supported locale codes
 */
function getSupportedLocales() {
    return [...supportedLocales];
}

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', initI18n);

/**
 * Check if translations for current locale are loaded
 * @returns {boolean} - True if translations are loaded, false otherwise
 */
function areTranslationsLoaded() {
    return !!translations[currentLocale] && Object.keys(translations[currentLocale]).length > 0;
}

// Export functions for use in other modules
window.i18n = {
    t,
    setLocale,
    getCurrentLocale,
    getSupportedLocales,
    translatePage,
    areTranslationsLoaded
};