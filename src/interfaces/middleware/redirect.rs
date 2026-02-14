use axum::http::{Uri, uri::PathAndQuery};
use axum::{extract::Request, middleware::Next, response::Response};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::{Layer, Service};

/// A middleware that redirects specific paths to the proper Axum routes.
/// This is used during the transition from the custom HTTP server to Axum.
pub struct RedirectMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for RedirectMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request) -> Self::Future {
        // Log the incoming request
        let uri = request.uri().clone();
        let path = uri.path().to_string();

        // Check and potentially redirect file-related API routes
        if path.starts_with("/api/files") {
            // Handle file-related redirects
            if path == "/api/files/upload" {
                // This is already properly mapped in Axum routes
                tracing::debug!("File upload request detected: {}", path);
            } else if path.starts_with("/api/files/file-") {
                // File download request - let's adjust the URI to match the Axum route
                // Extract the ID from the path
                let file_id = &path[11..];
                tracing::info!(
                    "Redirecting file download request: {} to /api/files/{}",
                    path,
                    file_id
                );

                // Create a new URI for the Axum route
                let uri_clone = uri.clone();
                let mut parts = uri_clone.into_parts();
                let query = parts
                    .path_and_query
                    .as_ref()
                    .and_then(|pq| pq.query())
                    .map(|q| format!("?{}", q))
                    .unwrap_or_default();

                let new_path = format!("/api/files/{}{}", file_id, query);
                parts.path_and_query = Some(
                    PathAndQuery::from_maybe_shared(new_path.into_bytes())
                        .expect("Failed to create path and query"),
                );

                let new_uri = Uri::from_parts(parts).expect("Failed to create URI");
                *request.uri_mut() = new_uri;
            }
        } else if path.starts_with("/api/folders") {
            // Handle folder-related redirects
            tracing::debug!("Folder request detected: {}", path);
            // We might need to add specific redirects for folder operations here
        }

        // Pass the request to the inner service
        let future = self.inner.call(request);

        Box::pin(async move {
            let response = future.await?;
            Ok(response)
        })
    }
}

/// The layer that applies the RedirectMiddleware.
#[derive(Clone)]
pub struct RedirectLayer;

impl<S> Layer<S> for RedirectLayer {
    type Service = RedirectMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RedirectMiddleware { inner }
    }
}

/// Axum middleware function that can be applied directly to routes
pub async fn redirect_middleware(request: Request, next: Next) -> Response {
    // Get the path
    let path = request.uri().path().to_string();

    // Process the request based on the path
    if path.starts_with("/api/files")
        || path.starts_with("/api/folders")
        || path.starts_with("/api/auth")
    {
        tracing::debug!("API request detected in middleware: {}", path);
        // Log additional information about the request
        if let Some(content_type) = request.headers().get("content-type") {
            tracing::debug!("Content-Type: {:?}", content_type);
        }

        // For debugging auth-related requests
        if path.starts_with("/api/auth") {
            tracing::info!("Auth API request: {} method: {}", path, request.method());
        }
    }

    // Continue the middleware chain
    next.run(request).await
}
