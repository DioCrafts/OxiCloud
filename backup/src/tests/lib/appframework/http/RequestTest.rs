// Copyright (c) 2013 Thomas Tanghus (thomas@tanghus.net)
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

extern crate test_utils;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::{Read, Seek, SeekFrom};
use std::ops::{Index, IndexMut};

use test_utils::{register_stream_wrapper, unregister_stream_wrapper};
use serde_json::Value;

// Assumming this is the actual Request implementation we're testing
use oc_appframework::http::{Request, StreamRead};

thread_local! {
    static TEST_DATA: std::cell::RefCell<String> = std::cell::RefCell::new(String::new());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    
    fn set_up() {
        let request_stream_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/lib/appframework/http/requeststream.php");
        
        if unregister_stream_wrapper("fakeinput").is_ok() {
            register_stream_wrapper("fakeinput", "RequestStream").expect("Failed to register stream wrapper");
        } else {
            register_stream_wrapper("fakeinput", "RequestStream").expect("Failed to register stream wrapper");
        }
    }
    
    fn tear_down() {
        unregister_stream_wrapper("fakeinput").expect("Failed to unregister stream wrapper");
    }
    
    #[test]
    fn test_request_accessors() {
        set_up();
        
        let mut vars = HashMap::new();
        let mut get_vars = HashMap::new();
        get_vars.insert("name".to_string(), "John Q. Public".to_string());
        get_vars.insert("nickname".to_string(), "Joey".to_string());
        
        vars.insert("get".to_string(), get_vars);
        vars.insert("method".to_string(), "GET".to_string());
        
        let request = Request::new(vars);
        
        // Countable
        assert_eq!(2, request.len());
        // Array access
        assert_eq!("Joey", request["nickname"]);
        // Property accessors
        assert_eq!("Joey", request.get("nickname").unwrap());
        assert!(request.contains_key("nickname"));
        assert!(request.get("nickname").is_some());
        assert!(request.get("flickname").is_none());
        // Only testing 'get', but same approach for post, files etc.
        assert_eq!("Joey", request.get_params()["nickname"]);
        // Always returns None if variable not set.
        assert!(request.get("flickname").is_none());
        
        tear_down();
    }
    
    #[test]
    fn test_precedence() {
        set_up();
        
        let mut vars = HashMap::new();
        let mut get_vars = HashMap::new();
        get_vars.insert("name".to_string(), "John Q. Public".to_string());
        get_vars.insert("nickname".to_string(), "Joey".to_string());
        
        let mut post_vars = HashMap::new();
        post_vars.insert("name".to_string(), "Jane Doe".to_string());
        post_vars.insert("nickname".to_string(), "Janey".to_string());
        
        let mut url_params = HashMap::new();
        url_params.insert("user".to_string(), "jw".to_string());
        url_params.insert("name".to_string(), "Johnny Weissmüller".to_string());
        
        vars.insert("get".to_string(), get_vars);
        vars.insert("post".to_string(), post_vars);
        vars.insert("urlParams".to_string(), url_params);
        
        let request = Request::new(vars);
        
        assert_eq!(3, request.len());
        assert_eq!("Janey", request.get("nickname").unwrap());
        assert_eq!("Johnny Weissmüller", request.get("name").unwrap());
        
        tear_down();
    }
    
    #[test]
    #[should_panic(expected = "Cannot modify request parameters")]
    fn test_immutable_array_access() {
        set_up();
        
        let mut vars = HashMap::new();
        let mut get_vars = HashMap::new();
        get_vars.insert("name".to_string(), "John Q. Public".to_string());
        get_vars.insert("nickname".to_string(), "Joey".to_string());
        
        vars.insert("get".to_string(), get_vars);
        
        let mut request = Request::new(vars);
        request["nickname"] = "Janey".to_string();
        
        tear_down();
    }
    
    #[test]
    #[should_panic(expected = "Cannot modify request parameters")]
    fn test_immutable_magic_access() {
        set_up();
        
        let mut vars = HashMap::new();
        let mut get_vars = HashMap::new();
        get_vars.insert("name".to_string(), "John Q. Public".to_string());
        get_vars.insert("nickname".to_string(), "Joey".to_string());
        
        vars.insert("get".to_string(), get_vars);
        
        let mut request = Request::new(vars);
        request.set("nickname", "Janey").unwrap();
        
        tear_down();
    }
    
    #[test]
    #[should_panic(expected = "Invalid request method for accessing post parameters")]
    fn test_get_the_method_right() {
        set_up();
        
        let mut vars = HashMap::new();
        let mut get_vars = HashMap::new();
        get_vars.insert("name".to_string(), "John Q. Public".to_string());
        get_vars.insert("nickname".to_string(), "Joey".to_string());
        
        vars.insert("get".to_string(), get_vars);
        vars.insert("method".to_string(), "GET".to_string());
        
        let request = Request::new(vars);
        let _result = request.post_params();
        
        tear_down();
    }
    
    #[test]
    fn test_the_method_is_right() {
        set_up();
        
        let mut vars = HashMap::new();
        let mut get_vars = HashMap::new();
        get_vars.insert("name".to_string(), "John Q. Public".to_string());
        get_vars.insert("nickname".to_string(), "Joey".to_string());
        
        vars.insert("get".to_string(), get_vars);
        vars.insert("method".to_string(), "GET".to_string());
        
        let request = Request::new(vars);
        assert_eq!("GET", request.method());
        let result = request.get_params();
        assert_eq!("John Q. Public", result["name"]);
        assert_eq!("Joey", result["nickname"]);
        
        tear_down();
    }
    
    #[test]
    fn test_json_post() {
        set_up();
        
        TEST_DATA.with(|data| {
            *data.borrow_mut() = r#"{"name": "John Q. Public", "nickname": "Joey"}"#.to_string();
        });
        
        let mut vars = HashMap::new();
        let mut server_vars = HashMap::new();
        server_vars.insert("CONTENT_TYPE".to_string(), "application/json; utf-8".to_string());
        
        vars.insert("method".to_string(), "POST".to_string());
        vars.insert("server".to_string(), server_vars);
        
        let request = Request::new(vars);
        assert_eq!("POST", request.method());
        let result = request.post_params();
        assert_eq!("John Q. Public", result["name"]);
        assert_eq!("Joey", result["nickname"]);
        assert_eq!("Joey", request.all_params()["nickname"]);
        assert_eq!("Joey", request["nickname"]);
        
        tear_down();
    }
    
    #[test]
    fn test_patch() {
        set_up();
        
        TEST_DATA.with(|data| {
            *data.borrow_mut() = "name=John%20Q.%20Public&nickname=Joey".to_string();
        });
        
        let mut vars = HashMap::new();
        let mut server_vars = HashMap::new();
        server_vars.insert("CONTENT_TYPE".to_string(), "application/x-www-form-urlencoded".to_string());
        
        vars.insert("method".to_string(), "PATCH".to_string());
        vars.insert("server".to_string(), server_vars);
        
        let request = Request::new(vars);
        
        assert_eq!("PATCH", request.method());
        let result = request.patch_params();
        
        assert_eq!("John Q. Public", result["name"]);
        assert_eq!("Joey", result["nickname"]);
        
        tear_down();
    }
    
    #[test]
    fn test_json_patch_and_put() {
        set_up();
        
        // PUT content
        TEST_DATA.with(|data| {
            *data.borrow_mut() = r#"{"name": "John Q. Public", "nickname": "Joey"}"#.to_string();
        });
        
        let mut vars = HashMap::new();
        let mut server_vars = HashMap::new();
        server_vars.insert("CONTENT_TYPE".to_string(), "application/json; utf-8".to_string());
        
        vars.insert("method".to_string(), "PUT".to_string());
        vars.insert("server".to_string(), server_vars);
        
        let request = Request::new(vars);
        
        assert_eq!("PUT", request.method());
        let result = request.put_params();
        
        assert_eq!("John Q. Public", result["name"]);
        assert_eq!("Joey", result["nickname"]);
        
        // PATCH content
        TEST_DATA.with(|data| {
            *data.borrow_mut() = r#"{"name": "John Q. Public", "nickname": null}"#.to_string();
        });
        
        let mut vars = HashMap::new();
        let mut server_vars = HashMap::new();
        server_vars.insert("CONTENT_TYPE".to_string(), "application/json; utf-8".to_string());
        
        vars.insert("method".to_string(), "PATCH".to_string());
        vars.insert("server".to_string(), server_vars);
        
        let request = Request::new(vars);
        
        assert_eq!("PATCH", request.method());
        let result = request.patch_params();
        
        assert_eq!("John Q. Public", result["name"]);
        assert!(result.get("nickname").is_none());
        
        tear_down();
    }
    
    #[test]
    fn test_put_stream() {
        set_up();
        
        let test_image_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/data/testimage.png");
        
        let image_data = fs::read(&test_image_path).expect("Failed to read test image");
        
        TEST_DATA.with(|data| {
            *data.borrow_mut() = String::from_utf8_lossy(&image_data).to_string();
        });
        
        let mut vars = HashMap::new();
        let mut server_vars = HashMap::new();
        server_vars.insert("CONTENT_TYPE".to_string(), "image/png".to_string());
        
        vars.insert("put".to_string(), image_data.clone());
        vars.insert("method".to_string(), "PUT".to_string());
        vars.insert("server".to_string(), server_vars);
        
        let request = Request::new(vars);
        assert_eq!("PUT", request.method());
        
        let mut resource = request.put_stream().expect("Failed to get PUT stream");
        let mut contents = Vec::new();
        resource.read_to_end(&mut contents).expect("Failed to read stream");
        
        assert_eq!(image_data, contents);
        
        // Second attempt should fail
        match request.put_stream() {
            Ok(_) => panic!("Expected LogicException."),
            Err(_) => {} // This is expected
        }
        
        tear_down();
    }
}