use std::collections::HashMap;
use std::sync::Arc;

use mockall::predicate::*;
use mockall::*;
use rstest::rstest;

/// Mock for the Files View
#[automock]
trait FilesView {
    fn free_space(&self, path: &str) -> i64;
}

/// Sabre HTTP Request simplified for testing
struct SabreHttpRequest {
    headers: HashMap<String, String>,
}

impl SabreHttpRequest {
    fn new(headers: HashMap<&str, &str>) -> Self {
        let headers = headers
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        
        Self { headers }
    }

    fn get_header(&self, name: &str) -> Option<&String> {
        self.headers.get(name)
    }
}

/// Sabre DAV Server mock
struct SabreDavServer {
    http_request: Option<SabreHttpRequest>,
}

impl SabreDavServer {
    fn new() -> Self {
        Self { http_request: None }
    }
}

/// Custom error type for Sabre DAV exceptions
#[derive(Debug, thiserror::Error)]
enum SabreDavError {
    #[error("Insufficient storage")]
    InsufficientStorage,
}

/// Quota Plugin for checking storage space
struct QuotaPlugin {
    file_view: Option<Arc<dyn FilesView>>,
    server: Option<Arc<SabreDavServer>>,
}

impl QuotaPlugin {
    fn new() -> Self {
        Self {
            file_view: None,
            server: None,
        }
    }

    fn initialize(&mut self, server: Arc<SabreDavServer>) {
        self.server = Some(server);
    }

    fn get_length(&self) -> Option<u64> {
        let request = match &self.server {
            Some(server) => match &server.http_request {
                Some(req) => req,
                None => return None,
            },
            None => return None,
        };

        // First try X_EXPECTED_ENTITY_LENGTH
        if let Some(expected_length) = request.get_header("HTTP_X_EXPECTED_ENTITY_LENGTH") {
            if let Ok(length) = expected_length.parse::<u64>() {
                return Some(length);
            }
        }

        // Then try OC_TOTAL_LENGTH
        if let Some(total_length) = request.get_header("HTTP_OC_TOTAL_LENGTH") {
            if let Ok(length) = total_length.parse::<u64>() {
                return Some(length);
            }
        }

        // Finally try CONTENT_LENGTH
        if let Some(content_length) = request.get_header("HTTP_CONTENT_LENGTH") {
            if let Ok(length) = content_length.parse::<u64>() {
                return Some(length);
            }
        }

        None
    }

    fn check_quota(&self, path: &str) -> Result<bool, SabreDavError> {
        // Return early if no file view is set
        let file_view = match &self.file_view {
            Some(view) => view,
            None => return Ok(true),
        };

        let free = file_view.free_space(path);
        
        // OC\Files\FREE_SPACE_UNKNOWN = -2
        if free == -2 {
            return Ok(true);
        }

        if let Some(length) = self.get_length() {
            if length > free as u64 {
                return Err(SabreDavError::InsufficientStorage);
            }
        }

        Ok(true)
    }
}

/// Tests for QuotaPlugin
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    struct TestContext {
        server: Arc<SabreDavServer>,
        plugin: QuotaPlugin,
    }

    impl TestContext {
        fn new() -> Self {
            let server = Arc::new(SabreDavServer::new());
            let mut plugin = QuotaPlugin::new();
            plugin.initialize(Arc::clone(&server));
            Self { server, plugin }
        }
    }

    #[rstest]
    #[case(None, HashMap::new())]
    #[case(Some(1024), HashMap::from([("HTTP_X_EXPECTED_ENTITY_LENGTH", "1024")]))]
    #[case(Some(512), HashMap::from([("HTTP_CONTENT_LENGTH", "512")]))]
    #[case(Some(2048), HashMap::from([("HTTP_OC_TOTAL_LENGTH", "2048"), ("HTTP_CONTENT_LENGTH", "1024")]))]
    #[case(Some(4096), HashMap::from([("HTTP_OC_TOTAL_LENGTH", "2048"), ("HTTP_X_EXPECTED_ENTITY_LENGTH", "4096")]))]
    fn test_length(
        #[case] expected: Option<u64>,
        #[case] headers: HashMap<&str, &str>,
    ) {
        let mut ctx = TestContext::new();
        
        // Set HTTP request with headers
        let request = SabreHttpRequest::new(headers);
        let server = Arc::get_mut(&mut ctx.server).unwrap();
        server.http_request = Some(request);
        
        let length = ctx.plugin.get_length();
        assert_eq!(expected, length);
    }

    fn build_file_view_mock(quota: i64) -> Arc<MockFilesView> {
        let mut mock = MockFilesView::new();
        mock.expect_free_space()
            .returning(move |_| quota);
        Arc::new(mock)
    }

    #[rstest]
    #[case(1024, HashMap::new())]
    #[case(1024, HashMap::from([("HTTP_X_EXPECTED_ENTITY_LENGTH", "1024")]))]
    #[case(1024, HashMap::from([("HTTP_CONTENT_LENGTH", "512")]))]
    #[case(1024, HashMap::from([("HTTP_OC_TOTAL_LENGTH", "1024"), ("HTTP_CONTENT_LENGTH", "512")]))]
    #[case(-2, HashMap::new())]
    #[case(-2, HashMap::from([("HTTP_X_EXPECTED_ENTITY_LENGTH", "1024")]))]
    #[case(-2, HashMap::from([("HTTP_CONTENT_LENGTH", "512")]))]
    #[case(-2, HashMap::from([("HTTP_OC_TOTAL_LENGTH", "1024"), ("HTTP_CONTENT_LENGTH", "512")]))]
    fn test_check_quota(
        #[case] quota: i64,
        #[case] headers: HashMap<&str, &str>,
    ) {
        let mut ctx = TestContext::new();
        
        // Setup mock file view
        ctx.plugin.file_view = Some(build_file_view_mock(quota));
        
        // Set HTTP request with headers
        let request = SabreHttpRequest::new(headers);
        let server = Arc::get_mut(&mut ctx.server).unwrap();
        server.http_request = Some(request);
        
        let result = ctx.plugin.check_quota("");
        assert!(result.is_ok());
    }

    #[rstest]
    #[case(1023, HashMap::from([("HTTP_X_EXPECTED_ENTITY_LENGTH", "1024")]))]
    #[case(511, HashMap::from([("HTTP_CONTENT_LENGTH", "512")]))]
    #[case(2047, HashMap::from([("HTTP_OC_TOTAL_LENGTH", "2048"), ("HTTP_CONTENT_LENGTH", "1024")]))]
    fn test_check_exceeded_quota(
        #[case] quota: i64,
        #[case] headers: HashMap<&str, &str>,
    ) {
        let mut ctx = TestContext::new();
        
        // Setup mock file view
        ctx.plugin.file_view = Some(build_file_view_mock(quota));
        
        // Set HTTP request with headers
        let request = SabreHttpRequest::new(headers);
        let server = Arc::get_mut(&mut ctx.server).unwrap();
        server.http_request = Some(request);
        
        let result = ctx.plugin.check_quota("");
        assert!(matches!(result, Err(SabreDavError::InsufficientStorage)));
    }
}