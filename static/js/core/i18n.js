/**
 * OxiCloud Internationalization (i18n) Module
 *
 * This module provides functionality for internationalization of the OxiCloud web interface.
 * It loads translations from the server and provides functions to translate keys.
 */

// Supported locales (languages that have locale files on the server)
// Keep in sync with AVAILABLE_LOCALES in core/languageSelector.js
const supportedLocales = ['en', 'es', 'zh', 'zh-TW', 'fa', 'fr', 'de', 'pt', 'nl', 'it', 'hi', 'ar', 'ru', 'ja', 'ko', 'pl'];

// Resolve the best supported locale from a browser language list.
// Priority: exact full-tag (zh-TW) > Chinese script/region heuristics > primary subtag (zh)
function resolveBrowserLocale() {
    const browserLangs = navigator.languages || [navigator.language || 'en'];
    const lowerSupported = supportedLocales.map((l) => l.toLowerCase());

    for (const bl of browserLangs) {
        const idx = lowerSupported.indexOf(bl.toLowerCase());
        if (idx !== -1) return supportedLocales[idx];
    }
    for (const bl of browserLangs) {
        const tag = bl.toLowerCase();
        if (!tag.startsWith('zh')) continue;
        const isTraditional = tag.includes('hant') || /\b(tw|hk|mo)\b/.test(tag);
        const target = isTraditional ? 'zh-TW' : 'zh';
        if (supportedLocales.includes(target)) return target;
    }
    for (const bl of browserLangs) {
        const primary = bl.substring(0, 2).toLowerCase();
        if (supportedLocales.includes(primary)) return primary;
    }
    return 'en';
}

// Current locale code (overridden by saved preference in initI18n)
let currentLocale = resolveBrowserLocale();

// Cache for translations
/** @type {Record<string, any>} */
const translations = {};

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
        // Load directly from local JSON file
        const localeData = await fetch(`/locales/${locale}.json`);
        if (!localeData.ok) {
            throw new Error(`Failed to load locale file for ${locale}`);
        }

        translations[locale] = await localeData.json();
        return translations[locale];
    } catch (error) {
        console.error('Error loading translations:', error);

        // Return empty object as last resort
        translations[locale] = {};
        return translations[locale];
    }
}

/**
 * Get a nested translation value
 * @param {Record<string, any>} obj - The translations object
 * @param {string} path - The dot-notation path to the translation
 * @returns {string|null} - The translation value or null if not found
 */
function getNestedValue(obj, path) {
    // Try direct key match first
    if (obj && typeof obj === 'object' && path in obj) {
        const value = obj[path];
        return typeof value === 'string' ? value : null;
    }

    // Try standard dot notation for nested values
    const keys = path.split('.');
    let current = obj;

    for (const key of keys) {
        if (current && typeof current === 'object' && key in current) {
            current = current[key];
        } else {
            // Key not found in standard dotted path
            // Try a last attempt with underscore format if this is a prefix_suffix format key
            if (path.includes('_') && !path.includes('.')) {
                const [prefix, ...parts] = path.split('_');
                const suffix = parts.join('_');

                if (obj[prefix] && typeof obj[prefix] === 'object' && suffix in obj[prefix]) {
                    return obj[prefix][suffix];
                }
            }
            return null;
        }
    }

    return typeof current === 'string' ? current : null;
}

/**
 * Replace parameters in a translation string
 * @param {string} text - The translation string with placeholders
 * @param {Record<string, any>} params - The parameters to replace
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

    // Load translations for current locale
    await loadTranslations(currentLocale);

    // Preload English translations as fallback
    if (currentLocale !== 'en') {
        await loadTranslations('en');
    }

    // Mark loaded BEFORE translatePage so safeT resolves properly
    translationsLoaded = true;

    // Translate the page
    translatePage();

    console.log(`I18n initialized with locale: ${currentLocale}`);
}

/**
 * Translate all elements with data-i18n attribute
 */
function translatePage() {
    translateElement(document);
}

/**
 * Translate only elements within a given root (scoped).
 * Use this instead of translatePage() when you know which container changed.
 * @param {Element|Document} root - The root element to search within
 */
function translateElement(root) {
    // Use safeT instead of bare t() to avoid issues when other scripts
    // (e.g. admin.js) shadow the global t() function.
    const resolve = safeT;
    const el = root || document;
    el.querySelectorAll('[data-i18n]').forEach((element) => {
        const key = element.getAttribute('data-i18n');
        element.textContent = resolve(key);
    });

    el.querySelectorAll('[data-i18n-placeholder]').forEach((element) => {
        const key = element.getAttribute('data-i18n-placeholder');
        if (element instanceof HTMLInputElement || element instanceof HTMLTextAreaElement) {
            element.placeholder = resolve(key);
        }
    });

    el.querySelectorAll('[data-i18n-title]').forEach((element) => {
        const key = element.getAttribute('data-i18n-title');
        if (element instanceof HTMLElement) {
            element.title = resolve(key);
        }
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

// Flag to track if translations are loaded
let translationsLoaded = false;

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', async () => {
    await initI18n();
    // translationsLoaded already set inside initI18n
    // Dispatch an event when translations are fully loaded
    window.dispatchEvent(new Event('translationsLoaded'));
});

// Self-contained t wrapper — does NOT call the global t() because other
// scripts (e.g. admin.js) may shadow it, which would cause infinite recursion.
/**
 * @param {string} key
 * @param {string | Record<string, any>} [paramsOrFallback] - interpolation params object, or a string fallback used when the key is missing
 * @returns {string}
 */
function safeT(key, paramsOrFallback = {}) {
    const fallback = typeof paramsOrFallback === 'string' ? paramsOrFallback : null;
    const params = typeof paramsOrFallback === 'object' ? paramsOrFallback : {};

    const localeData = translations[currentLocale];
    if (!localeData) {
        // Translations not loaded yet — return fallback or humanised key suffix
        return fallback ?? key.split('.').pop() ?? key;
    }

    let value = getNestedValue(localeData, key);

    // Fallback to English
    if (!value && currentLocale !== 'en' && translations.en) {
        value = getNestedValue(translations.en, key);
    }

    if (!value) return fallback ?? key;
    return interpolate(value, params);
}

export const i18n = {
    t: safeT,
    setLocale,
    getCurrentLocale,
    getSupportedLocales,
    translatePage,
    translateElement,
    isLoaded: () => translationsLoaded
};
