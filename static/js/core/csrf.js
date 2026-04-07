/**
 * CSRF double-submit cookie utility.
 *
 * Reads the `oxicloud_csrf` cookie (which is NOT HttpOnly) and provides
 * its value as the `X-CSRF-Token` header on mutating requests.
 *
 * Usage:
 *   // In any fetch call that changes state:
 *   fetch(url, { method: 'POST', headers: { ...getCsrfHeaders(), 'Content-Type': 'application/json' } })
 *
 * The server-side `csrf_middleware` validates that the header value matches
 * the cookie for every POST/PUT/DELETE/PATCH request authenticated via
 * HttpOnly cookies.
 */

// eslint-disable-next-line no-unused-vars
function getCsrfToken() {
    const match = document.cookie.split('; ').find((row) => row.startsWith('oxicloud_csrf='));
    return match ? match.split('=')[1] : '';
}

// biome-ignore lint/correctness/noUnusedVariables: global function
function getCsrfHeaders() {
    const token = getCsrfToken();
    return token ? { 'X-CSRF-Token': token } : {};
}
