use serde::Serialize;
use serde_json;
use std::collections::HashMap;

use crate::appframework::http::Http;
use crate::appframework::http::Response;

/**
 * A renderer for JSON calls
 */
#[derive(Debug)]
pub struct JsonResponse {
    data: Box<dyn Serialize + Send + Sync>,
    #[allow(dead_code)]
    response: Response,
}

impl JsonResponse {
    /**
     * @param data the object or array that should be transformed
     * @param status_code the Http status code, defaults to 200
     */
    pub fn new<T: Serialize + Send + Sync + 'static>(
        data: T,
        status_code: u16,
    ) -> Self {
        let mut response = Response::new();
        response.set_status(status_code);
        response.add_header("X-Content-Type-Options", "nosniff");
        response.add_header("Content-type", "application/json; charset=utf-8");

        JsonResponse {
            data: Box::new(data),
            response,
        }
    }

    /**
     * Creates a new JsonResponse with default values
     */
    pub fn default() -> Self {
        Self::new(HashMap::<String, String>::new(), Http::STATUS_OK)
    }

    /**
     * Returns the rendered json
     * @return the rendered json
     */
    pub fn render(&self) -> String {
        serde_json::to_string(&self.data).unwrap_or_else(|_| String::from("{}"))
    }

    /**
     * Sets values in the data json array
     * @param data an array or object which will be transformed to JSON
     */
    pub fn set_data<T: Serialize + Send + Sync + 'static>(&mut self, data: T) {
        self.data = Box::new(data);
    }

    /**
     * Used to get the set parameters
     * Note: In Rust we can't return the exact data due to type erasure,
     * so this method is removed. Use specific getters for your data types instead.
     */
}

impl Default for JsonResponse {
    fn default() -> Self {
        Self::new(HashMap::<String, String>::new(), Http::STATUS_OK)
    }
}