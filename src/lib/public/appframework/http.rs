// Copyright (C) 2012 Bernhard Posselt nukeawhale@gmail.com
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! HTTP status codes for the App Framework
//!
//! Original authors: Bernhard Posselt, Thomas Tanghus, Bart Visscher

/// HTTP status codes used in the application framework
#[allow(dead_code)]
pub struct Http;

impl Http {
    pub const STATUS_CONTINUE: u16 = 100;
    pub const STATUS_SWITCHING_PROTOCOLS: u16 = 101;
    pub const STATUS_PROCESSING: u16 = 102;
    pub const STATUS_OK: u16 = 200;
    pub const STATUS_CREATED: u16 = 201;
    pub const STATUS_ACCEPTED: u16 = 202;
    pub const STATUS_NON_AUTHORATIVE_INFORMATION: u16 = 203;
    pub const STATUS_NO_CONTENT: u16 = 204;
    pub const STATUS_RESET_CONTENT: u16 = 205;
    pub const STATUS_PARTIAL_CONTENT: u16 = 206;
    pub const STATUS_MULTI_STATUS: u16 = 207;
    pub const STATUS_ALREADY_REPORTED: u16 = 208;
    pub const STATUS_IM_USED: u16 = 226;
    pub const STATUS_MULTIPLE_CHOICES: u16 = 300;
    pub const STATUS_MOVED_PERMANENTLY: u16 = 301;
    pub const STATUS_FOUND: u16 = 302;
    pub const STATUS_SEE_OTHER: u16 = 303;
    pub const STATUS_NOT_MODIFIED: u16 = 304;
    pub const STATUS_USE_PROXY: u16 = 305;
    pub const STATUS_RESERVED: u16 = 306;
    pub const STATUS_TEMPORARY_REDIRECT: u16 = 307;
    pub const STATUS_BAD_REQUEST: u16 = 400;
    pub const STATUS_UNAUTHORIZED: u16 = 401;
    pub const STATUS_PAYMENT_REQUIRED: u16 = 402;
    pub const STATUS_FORBIDDEN: u16 = 403;
    pub const STATUS_NOT_FOUND: u16 = 404;
    pub const STATUS_METHOD_NOT_ALLOWED: u16 = 405;
    pub const STATUS_NOT_ACCEPTABLE: u16 = 406;
    pub const STATUS_PROXY_AUTHENTICATION_REQUIRED: u16 = 407;
    pub const STATUS_REQUEST_TIMEOUT: u16 = 408;
    pub const STATUS_CONFLICT: u16 = 409;
    pub const STATUS_GONE: u16 = 410;
    pub const STATUS_LENGTH_REQUIRED: u16 = 411;
    pub const STATUS_PRECONDITION_FAILED: u16 = 412;
    pub const STATUS_REQUEST_ENTITY_TOO_LARGE: u16 = 413;
    pub const STATUS_REQUEST_URI_TOO_LONG: u16 = 414;
    pub const STATUS_UNSUPPORTED_MEDIA_TYPE: u16 = 415;
    pub const STATUS_REQUEST_RANGE_NOT_SATISFIABLE: u16 = 416;
    pub const STATUS_EXPECTATION_FAILED: u16 = 417;
    pub const STATUS_IM_A_TEAPOT: u16 = 418;
    pub const STATUS_UNPROCESSABLE_ENTITY: u16 = 422;
    pub const STATUS_LOCKED: u16 = 423;
    pub const STATUS_FAILED_DEPENDENCY: u16 = 424;
    pub const STATUS_UPGRADE_REQUIRED: u16 = 426;
    pub const STATUS_PRECONDITION_REQUIRED: u16 = 428;
    pub const STATUS_TOO_MANY_REQUESTS: u16 = 429;
    pub const STATUS_REQUEST_HEADER_FIELDS_TOO_LARGE: u16 = 431;
    pub const STATUS_INTERNAL_SERVER_ERROR: u16 = 500;
    pub const STATUS_NOT_IMPLEMENTED: u16 = 501;
    pub const STATUS_BAD_GATEWAY: u16 = 502;
    pub const STATUS_SERVICE_UNAVAILABLE: u16 = 503;
    pub const STATUS_GATEWAY_TIMEOUT: u16 = 504;
    pub const STATUS_HTTP_VERSION_NOT_SUPPORTED: u16 = 505;
    pub const STATUS_VARIANT_ALSO_NEGOTIATES: u16 = 506;
    pub const STATUS_INSUFFICIENT_STORAGE: u16 = 507;
    pub const STATUS_LOOP_DETECTED: u16 = 508;
    pub const STATUS_BANDWIDTH_LIMIT_EXCEEDED: u16 = 509;
    pub const STATUS_NOT_EXTENDED: u16 = 510;
    pub const STATUS_NETWORK_AUTHENTICATION_REQUIRED: u16 = 511;
}