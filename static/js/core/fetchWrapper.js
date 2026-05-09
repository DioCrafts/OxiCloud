/**
 * Global fetch interceptor for transparent 401 → token-refresh → retry.
 *
 * WHY a global interceptor instead of a per-call wrapper:
 *   Every authenticated API call in the app needs the same 401 handling.
 *   Replacing each `fetch(...)` call individually is error-prone (easy to
 *   miss one) and creates noise across every module. Patching `window.fetch`
 *   once here means all existing and future calls are covered automatically.
 *
 * WHY _originalFetch must be used everywhere inside this module:
 *   `_refresh()` itself calls `/api/auth/refresh`. If it used `window.fetch`
 *   (the patched version), a 401 on the refresh endpoint would call `_refresh()`
 *   again, which would call `window.fetch` again — infinite recursion. The same
 *   applies to the interceptor's own initial call and the retry: they must all
 *   bypass the interceptor by using the captured `_originalFetch` directly.
 *
 * WHY /api/auth/ endpoints are excluded from the retry logic:
 *   login, logout, refresh, and /me are the auth primitives themselves.
 *   A 401 on these means credentials are genuinely invalid — retrying after
 *   a refresh makes no sense and would loop.
 *
 * WHY cross-origin requests bypass the interceptor entirely:
 *   A 401 from an external service (e.g. a third-party library calling its own
 *   API) has nothing to do with OxiCloud's session. Attempting a token refresh
 *   and redirecting to /login in response would be catastrophic. Only same-origin
 *   requests go through the refresh-and-retry path.
 *
 * Call `installFetchInterceptor()` once at app startup (before any fetch).
 */

import { ApiError } from './ApiError.js';
import { getCsrfHeaders } from './csrf.js';

const WRAPPER_REFRESH_ENDPOINT = '/api/auth/refresh';
const WRAPPER_USER_DATA_KEY = 'oxicloud_user';

/** Captured before patching — the only safe fetch inside this module. */
let _originalFetch = window.fetch.bind(window);

/** Deduplicates concurrent refresh attempts into a single in-flight promise. */
/** @type {Promise<boolean> | null} */
let _refreshInFlight = null;

async function _refresh() {
    if (_refreshInFlight) return _refreshInFlight;

    console.log(`requesting a refresh token`);

    // Must use _originalFetch to avoid re-entering the interceptor.
    _refreshInFlight = (async () => {
        try {
            const r = await _originalFetch(WRAPPER_REFRESH_ENDPOINT, {
                method: 'POST',
                credentials: 'same-origin',
                headers: { 'Content-Type': 'application/json', ...getCsrfHeaders() },
                body: '{}'
            });
            return r.ok;
        } catch {
            return false;
        } finally {
            _refreshInFlight = null;
        }
    })();

    return _refreshInFlight;
}

/** Decode a non-ok Response into an ApiError without re-throwing. */
/**
 *
 * @param {Response} response
 * @returns {Promise<ApiError>}
 */
async function _toApiError(response) {
    let body = null;
    try {
        body = await response.json();
    } catch {}
    return ApiError.from(response.status, body);
}

function installFetchInterceptor() {
    // Capture the real fetch before overwriting it.
    _originalFetch = window.fetch.bind(window);

    window.fetch = async (url, options) => {
        // Use _originalFetch for the actual network call — NOT window.fetch —
        // so this interceptor does not call itself recursively.
        const response = await _originalFetch(url, options);

        if (response.ok) return response;

        const urlStr = typeof url === 'string' ? url : url instanceof URL ? url.href : (url.url ?? '');

        // Cross-origin errors are none of our business — pass through untouched.
        let isSameOrigin = false;
        try {
            isSameOrigin = new URL(urlStr, window.location.origin).origin === window.location.origin;
        } catch {}
        if (!isSameOrigin) return response;

        // Auth endpoints must bypass retry: a 401 on /api/auth/* means the
        // credentials themselves are invalid; retrying would cause a loop.
        // Public share endpoints (/api/s/) use 401 to mean "password required",
        // not "session expired" — intercepting them would wrongly redirect to login.
        if (urlStr.includes('/api/auth/') || urlStr.includes('/api/s/')) return response;

        // Share endpoints use HTTP 401 to mean "password required", not session
        // expiry — return the raw response so the caller can show a password prompt.
        if (response.status === 401 && (urlStr.includes('/api/s/') || urlStr.includes('/api/auth/'))) return response;

        // Non-auth 401: try a token refresh then retry the original request.
        // Auth endpoints are excluded: a 401 on /api/auth/* means the credentials
        // themselves are invalid; retrying after a refresh would loop.
        const refreshed = await _refresh();
        if (!refreshed) {
            localStorage.removeItem(WRAPPER_USER_DATA_KEY);
            window.location.href = '/login?source=session_expired';
            // XXX would prefer an ApiError ?
            throw new Error('Session expired');
        }

        // Token is now refreshed
        const retried = await _originalFetch(url, options);
        if (retried.ok) return retried;
        throw await _toApiError(retried);
    };
}

export { installFetchInterceptor };
