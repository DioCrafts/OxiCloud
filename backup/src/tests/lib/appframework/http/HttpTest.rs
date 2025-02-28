// Copyright (C) 2012 Bernhard Posselt <nukeawhale@gmail.com>
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

mod http;

use std::collections::HashMap;
use chrono::{DateTime, Utc, TimeZone};
use crate::http::Http;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::StatusCode;

    struct HttpTest {
        server: HashMap<String, String>,
        http: Http,
    }

    impl HttpTest {
        fn new() -> Self {
            let server = HashMap::new();
            let http = Http::new(server.clone(), None);
            HttpTest { server, http }
        }
    }

    #[test]
    fn test_protocol() {
        let test = HttpTest::new();
        let header = test.http.get_status_header(StatusCode::TemporaryRedirect, None, None);
        assert_eq!("HTTP/1.1 307 Temporary Redirect", header);
    }

    #[test]
    fn test_protocol_10() {
        let server = HashMap::new();
        let http = Http::new(server, Some("HTTP/1.0".to_string()));
        let header = http.get_status_header(StatusCode::Ok, None, None);
        assert_eq!("HTTP/1.0 200 OK", header);
    }

    #[test]
    fn test_etag_match_returns_not_modified() {
        let mut server = HashMap::new();
        server.insert("HTTP_IF_NONE_MATCH".to_string(), "hi".to_string());
        let http = Http::new(server, None);

        let header = http.get_status_header(StatusCode::Ok, None, Some("hi".to_string()));
        assert_eq!("HTTP/1.1 304 Not Modified", header);
    }

    #[test]
    fn test_last_modified_match_returns_not_modified() {
        let mut server = HashMap::new();
        server.insert(
            "HTTP_IF_MODIFIED_SINCE".to_string(),
            "Thu, 01 Jan 1970 00:00:12 +0000".to_string(),
        );
        let http = Http::new(server, None);

        // Create a DateTime at the Unix timestamp 12
        let date_time = Utc.timestamp_opt(12, 0).unwrap();

        let header = http.get_status_header(StatusCode::Ok, Some(date_time), None);
        assert_eq!("HTTP/1.1 304 Not Modified", header);
    }

    #[test]
    fn test_temp_redirect_becomes_found_in_http_10() {
        let server = HashMap::new();
        let http = Http::new(server, Some("HTTP/1.0".to_string()));

        let header = http.get_status_header(StatusCode::TemporaryRedirect, None, None);
        assert_eq!("HTTP/1.0 302 Found", header);
    }
    // TODO: write unit tests for http codes
}