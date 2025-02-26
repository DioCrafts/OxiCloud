#[cfg(test)]
mod tests {
    use crate::appframework::http::json_response::JSONResponse;
    use crate::appframework::http::response::Response;

    struct JSONResponseTest {
        json: JSONResponse,
    }

    impl JSONResponseTest {
        fn new() -> Self {
            Self {
                json: JSONResponse::default(),
            }
        }

        fn test_header(&self) {
            let headers = self.json.get_headers();
            assert_eq!(
                "application/json; charset=utf-8",
                headers.get("Content-type").unwrap()
            );
        }

        fn test_set_data(&mut self) {
            let params = vec!["hi".to_string(), "yo".to_string()];
            self.json.set_data(params.clone());

            assert_eq!(params, self.json.get_data());
        }

        fn test_set_render(&mut self) {
            use std::collections::HashMap;
            let mut params = HashMap::new();
            params.insert("test".to_string(), "hi".to_string());
            self.json.set_data(params);

            let expected = r#"{"test":"hi"}"#;

            assert_eq!(expected, self.json.render());
        }

        fn test_render(&mut self) {
            use std::collections::HashMap;
            let mut params = HashMap::new();
            params.insert("test".to_string(), "hi".to_string());
            self.json.set_data(params);

            let expected = r#"{"test":"hi"}"#;

            assert_eq!(expected, self.json.render());
        }

        fn test_should_have_x_content_header_by_default(&self) {
            let headers = self.json.get_headers();
            assert_eq!("nosniff", headers.get("X-Content-Type-Options").unwrap());
        }

        fn test_constructor_allows_to_set_data(&self) {
            let data = vec!["hi".to_string()];
            let code = 300;
            let response = JSONResponse::new(data, code);

            let expected = r#"["hi"]"#;
            assert_eq!(expected, response.render());
            assert_eq!(code, response.get_status());
        }
    }

    #[test]
    fn test_json_response() {
        let mut test = JSONResponseTest::new();
        test.test_header();
        test.test_set_data();
        test.test_set_render();
        test.test_render();
        test.test_should_have_x_content_header_by_default();
        test.test_constructor_allows_to_set_data();
    }
}