//! Wrapper for server side events (http://en.wikipedia.org/wiki/Server-sent_events)
//! includes a fallback for older browsers and IE
//!
//! use server side events with caution, to many open requests can hang the server

use axum::{
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::io::{self, Write};

/// EventSource handles Server-Sent Events communication
pub struct EventSource {
    fallback: bool,
    fallback_id: Option<String>,
}

impl EventSource {
    /// Create a new EventSource instance
    ///
    /// This will set the appropriate headers and prepare the connection
    /// for server-sent events
    pub async fn new(query_params: &[(String, String)], is_call_registered: bool) -> Result<Self, StatusCode> {
        // Check for CSRF
        if !is_call_registered {
            return Err(StatusCode::FORBIDDEN);
        }

        let fallback = query_params
            .iter()
            .any(|(k, v)| k == "fallback" && v == "true");

        let fallback_id = if fallback {
            query_params
                .iter()
                .find(|(k, _)| k == "fallback_id")
                .map(|(_, v)| v.clone())
        } else {
            None
        };

        Ok(Self {
            fallback,
            fallback_id,
        })
    }

    /// Send a message to the client
    ///
    /// # Arguments
    ///
    /// * `event_type` - The type of event to send
    /// * `data` - The data to send
    ///
    /// If only one parameter is given, a typeless message will be sent with that parameter as data
    pub fn send<T: Serialize>(&self, event_type: Option<&str>, data: &T) -> io::Result<()> {
        let mut output = Vec::new();

        if self.fallback {
            let fallback_id = self.fallback_id.as_deref().unwrap_or("0");
            let json_data = serde_json::to_string(data)?;
            let event_type_str = event_type.unwrap_or("");
            
            writeln!(
                output,
                "<script type=\"text/javascript\">window.parent.OC.EventSource.fallBackCallBack({},\"{}\",{})</script>",
                fallback_id, event_type_str, json_data
            )?;
        } else {
            if let Some(event_type) = event_type {
                writeln!(output, "event: {}", event_type)?;
            }
            writeln!(output, "data: {}", serde_json::to_string(data)?)?;
        }
        
        writeln!(output)?;
        io::stdout().write_all(&output)?;
        io::stdout().flush()?;
        
        Ok(())
    }

    /// Close the connection of the event source
    pub fn close(&self) -> io::Result<()> {
        self.send(Some("__internal__"), &"close")
    }

    /// Creates HTTP headers for this EventSource
    pub fn create_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(header::CACHE_CONTROL, "no-cache".parse().unwrap());
        
        if self.fallback {
            headers.insert(header::CONTENT_TYPE, "text/html".parse().unwrap());
        } else {
            headers.insert(header::CONTENT_TYPE, "text/event-stream".parse().unwrap());
        }
        
        headers
    }
}

impl IntoResponse for EventSource {
    fn into_response(self) -> Response {
        let headers = self.create_headers();
        let mut response = "".into_response();
        
        // Add dummy data for IE when using fallback
        if self.fallback {
            let dummy_data = "<span></span>\n".repeat(10);
            response = dummy_data.into_response();
        }
        
        *response.headers_mut() = headers;
        response
    }
}