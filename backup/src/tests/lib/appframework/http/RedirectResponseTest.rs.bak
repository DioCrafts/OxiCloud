/*
 * ownCloud - App Framework
 *
 * @author Bernhard Posselt
 * @copyright 2012 Bernhard Posselt nukeawhale@gmail.com
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU AFFERO GENERAL PUBLIC LICENSE for more details.
 *
 * You should have received a copy of the GNU Affero General Public
 * License along with this library.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

// tests/lib/appframework/http/redirect_response_test.rs

use std::collections::HashMap;
use oc::appframework::http::{Http, RedirectResponse};

#[cfg(test)]
mod tests {
    use super::*;

    struct RedirectResponseTest {
        response: RedirectResponse,
    }

    impl RedirectResponseTest {
        fn setup() -> Self {
            RedirectResponseTest {
                response: RedirectResponse::new("/url"),
            }
        }

        fn test_headers(&self) {
            let headers = self.response.get_headers();
            assert_eq!("/url", headers.get("Location").unwrap());
            assert_eq!(Http::STATUS_TEMPORARY_REDIRECT, self.response.get_status());
        }

        fn test_get_redirect_url(&self) {
            assert_eq!("/url", self.response.get_redirect_url());
        }
    }

    #[test]
    fn test_redirect_response_headers() {
        let test = RedirectResponseTest::setup();
        test.test_headers();
    }

    #[test]
    fn test_redirect_response_get_redirect_url() {
        let test = RedirectResponseTest::setup();
        test.test_get_redirect_url();
    }
}