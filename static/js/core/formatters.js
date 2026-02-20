/**
 * OxiCloud - Shared format and escaping utilities
 * Centralized global helpers for date/size/text formatting and XSS-safe escaping.
 */

function escapeHtml(str) {
    if (typeof str !== 'string') return '';
    return str
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/\"/g, '&quot;')
        .replace(/'/g, '&#039;');
}

function formatFileSize(bytes) {
    if (bytes === 0) return '0 Bytes';

    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

function formatDateTime(value) {
    if (!value) return '';
    let dateValue;
    if (value instanceof Date) {
        dateValue = value;
    } else if (typeof value === 'number') {
        dateValue = new Date(value < 1e12 ? value * 1000 : value);
    } else {
        dateValue = new Date(value);
    }
    if (isNaN(dateValue.getTime())) return String(value);
    return dateValue.toLocaleDateString() + ' ' +
        dateValue.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}

function formatDateShort(value) {
    if (!value) return 'N/A';
    const dateValue = typeof value === 'number' ? new Date(value * 1000) : new Date(value);
    if (isNaN(dateValue.getTime())) return String(value);
    return dateValue.toLocaleDateString(undefined, { year: 'numeric', month: 'short', day: 'numeric' });
}

function isTextViewable(mimeType) {
    if (!mimeType) return false;
    if (mimeType.startsWith('text/')) return true;
    const textTypes = [
        'application/json', 'application/xml', 'application/javascript',
        'application/x-sh', 'application/x-yaml', 'application/toml',
        'application/x-toml', 'application/sql',
    ];
    return textTypes.includes(mimeType);
}

window.escapeHtml = escapeHtml;
window.formatFileSize = formatFileSize;
window.formatDateTime = formatDateTime;
window.formatDateShort = formatDateShort;
window.isTextViewable = isTextViewable;
