/**
 * Structured error thrown by the fetch interceptor for same-origin API failures.
 */

import { i18n } from './i18n.js';

/**
 * @import {ApiErrorResponse} from './types.js'
 */

/**
 * Translate a parsed API error body into a human-readable, i18n-aware message.
 * Falls back to the raw `error` string if no i18n key matches.
 * @param {ApiErrorResponse|null} body - Parsed JSON error response
 * @returns {string}
 */
export function extractApiError(body) {
    if (body?.error_type && body?.error_code) {
        const key = `errors.${body.error_type}.${body.error_code}`;
        const translated = i18n.t(key);
        if (translated && translated !== key) return translated;
    }
    return body?.error || 'Unknown error';
}

export class ApiError extends Error {
    /**
     * @param {string} message
     * @param {object} opts
     * @param {number} opts.status
     * @param {string} [opts.error_type]
     * @param {string} [opts.error_code]
     */
    constructor(message, { status, error_type, error_code }) {
        super(message);
        this.name = 'ApiError';
        /** @type {number} */
        this.status = status;
        /** @type {string|undefined} */
        this.error_type = error_type;
        /** @type {string|undefined} */
        this.error_code = error_code;
    }

    /**
     * Build an ApiError from an HTTP status code and a parsed (or null) response body.
     * @param {number} status
     * @param {ApiErrorResponse|null} body
     * @returns {ApiError}
     */
    static from(status, body) {
        return new ApiError(extractApiError(body), {
            status,
            error_type: body?.error_type,
            error_code: body?.error_code
        });
    }
}
