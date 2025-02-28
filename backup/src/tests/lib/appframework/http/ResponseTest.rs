use chrono::{DateTime, Utc, TimeZone};
use std::collections::HashMap;

// Import our implementation
use oc_app_framework::http::{Response, Http};

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    struct ResponseTest {
        child_response: Response,
    }

    impl ResponseTest {
        fn new() -> Self {
            Self {
                child_response: Response::new(),
            }
        }

        fn test_add_header(&mut self) {
            self.child_response.add_header("hello", "world");
            let headers = self.child_response.get_headers();
            assert_eq!("world", headers.get("hello").unwrap());
        }

        fn test_add_header_value_null_deletes_it(&mut self) {
            self.child_response.add_header("hello", "world");
            self.child_response.add_header("hello", None);
            assert_eq!(1, self.child_response.get_headers().len());
        }

        fn test_cache_headers_are_disabled_by_default(&self) {
            let headers = self.child_response.get_headers();
            assert_eq!("no-cache, must-revalidate", headers.get("Cache-Control").unwrap());
        }

        fn test_render_return_null_by_default(&self) {
            assert_eq!(None, self.child_response.render());
        }

        fn test_get_status(&mut self) {
            let default = self.child_response.get_status();

            self.child_response.set_status(Http::STATUS_NOT_FOUND);

            assert_eq!(Http::STATUS_OK, default);
            assert_eq!(Http::STATUS_NOT_FOUND, self.child_response.get_status());
        }

        fn test_get_etag(&mut self) {
            self.child_response.set_etag("hi");
            assert_eq!("hi", self.child_response.get_etag().unwrap());
        }

        fn test_get_last_modified(&mut self) {
            let last_modified = Utc.timestamp_opt(1, 0).unwrap();
            self.child_response.set_last_modified(last_modified);
            assert_eq!(last_modified, self.child_response.get_last_modified().unwrap());
        }

        fn test_cache_seconds_zero(&mut self) {
            self.child_response.cache_for(0);
            
            let headers = self.child_response.get_headers();
            assert_eq!("no-cache, must-revalidate", headers.get("Cache-Control").unwrap());
        }

        fn test_cache_seconds(&mut self) {
            self.child_response.cache_for(33);
            
            let headers = self.child_response.get_headers();
            assert_eq!(
                "max-age=33, must-revalidate", 
                headers.get("Cache-Control").unwrap()
            );
        }

        fn test_etag_last_modified_headers(&mut self) {
            let last_modified = Utc.timestamp_opt(1, 0).unwrap();
            self.child_response.set_last_modified(last_modified);
            let headers = self.child_response.get_headers();
            assert_eq!("Thu, 01 Jan 1970 00:00:01 +0000", headers.get("Last-Modified").unwrap());
        }
    }

    #[test]
    fn test_add_header() {
        ResponseTest::new().test_add_header();
    }

    #[test]
    fn test_add_header_value_null_deletes_it() {
        ResponseTest::new().test_add_header_value_null_deletes_it();
    }

    #[test]
    fn test_cache_headers_are_disabled_by_default() {
        ResponseTest::new().test_cache_headers_are_disabled_by_default();
    }

    #[test]
    fn test_render_return_null_by_default() {
        ResponseTest::new().test_render_return_null_by_default();
    }

    #[test]
    fn test_get_status() {
        ResponseTest::new().test_get_status();
    }

    #[test]
    fn test_get_etag() {
        ResponseTest::new().test_get_etag();
    }

    #[test]
    fn test_get_last_modified() {
        ResponseTest::new().test_get_last_modified();
    }

    #[test]
    fn test_cache_seconds_zero() {
        ResponseTest::new().test_cache_seconds_zero();
    }

    #[test]
    fn test_cache_seconds() {
        ResponseTest::new().test_cache_seconds();
    }

    #[test]
    fn test_etag_last_modified_headers() {
        ResponseTest::new().test_etag_last_modified_headers();
    }
}