use actix_web::{web, HttpResponse, Responder, Result};
use actix_web::web::Bytes;
use futures::{Stream, StreamExt};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use serde_json::json;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::time::Duration;

#[derive(Deserialize)]
pub struct NewFileParams {
    dir: Option<String>,
    filename: Option<String>,
    content: Option<String>,
    source: Option<String>,
}

#[derive(Serialize)]
struct FileInfo {
    id: String,
    mime: String,
    size: u64,
    content: Option<String>,
    etag: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    data: ErrorData,
}

#[derive(Serialize)]
struct ErrorData {
    message: String,
}

#[derive(Clone)]
struct EventSource {
    clients: Arc<Mutex<Vec<tokio::sync::mpsc::Sender<Bytes>>>>,
}

impl EventSource {
    fn new() -> Self {
        EventSource {
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    async fn send(&self, event_type: &str, data: impl Serialize) {
        let json_data = serde_json::to_string(&data).unwrap_or_default();
        let message = format!("event: {}\ndata: {}\n\n", event_type, json_data);
        
        let mut clients = self.clients.lock().unwrap();
        clients.retain_mut(|client| {
            client.try_send(Bytes::from(message.clone())).is_ok()
        });
    }

    fn close(&self) {
        let mut clients = self.clients.lock().unwrap();
        clients.clear();
    }
}

struct L10n;

impl L10n {
    fn t(&self, message: &str) -> String {
        // In a real implementation, this would use a translation system
        message.to_string()
    }

    fn t_with_args(&self, message: &str, args: &[&str]) -> String {
        // Simple placeholder replacement
        let mut result = message.to_string();
        for (i, arg) in args.iter().enumerate() {
            result = result.replace(&format!("%s", i+1), arg);
        }
        result
    }
}

struct Filesystem;

impl Filesystem {
    async fn file_exists(path: &str) -> bool {
        // Implementation would check file existence
        tokio::fs::metadata(path).await.is_ok()
    }

    async fn file_put_contents(path: &str, content: impl AsRef<[u8]>) -> bool {
        if let Ok(mut file) = File::create(path).await {
            file.write_all(content.as_ref()).await.is_ok()
        } else {
            false
        }
    }

    async fn file_put_contents_stream<S>(path: &str, stream: S) -> bool 
    where S: Stream<Item = Result<Bytes, reqwest::Error>> + Unpin
    {
        if let Ok(mut file) = File::create(path).await {
            let mut stream = Box::pin(stream);
            while let Some(chunk) = stream.next().await {
                if let Ok(chunk) = chunk {
                    if let Err(_) = file.write_all(&chunk).await {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    async fn touch(path: &str) -> bool {
        File::create(path).await.is_ok()
    }

    async fn filesize(path: &str) -> u64 {
        tokio::fs::metadata(path).await.map(|m| m.len()).unwrap_or(0)
    }

    async fn get_file_info(path: &str) -> Option<FileInfo> {
        // This would be implemented to get actual file info
        if let Ok(metadata) = tokio::fs::metadata(path).await {
            Some(FileInfo {
                id: "file_id".to_string(),
                mime: Self::get_mime_type(path),
                size: metadata.len(),
                content: None,
                etag: "etag_value".to_string(),
            })
        } else {
            None
        }
    }

    fn get_mime_type(path: &str) -> String {
        // Simple mime type detection based on extension
        let extension = Path::new(path).extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
            
        match extension {
            "txt" => "text/plain".to_string(),
            "html" | "htm" => "text/html".to_string(),
            "css" => "text/css".to_string(),
            "js" => "application/javascript".to_string(),
            "json" => "application/json".to_string(),
            "png" => "image/png".to_string(),
            "jpg" | "jpeg" => "image/jpeg".to_string(),
            "gif" => "image/gif".to_string(),
            _ => "application/octet-stream".to_string(),
        }
    }
}

struct Helper;

impl Helper {
    fn get_file_template_manager() -> TemplateManager {
        TemplateManager {}
    }

    fn get_mimetype_detector() -> MimetypeDetector {
        MimetypeDetector {}
    }
}

struct TemplateManager;

impl TemplateManager {
    fn get_template(&self, mime_type: &str) -> Option<String> {
        // This would return a template based on the mime type
        match mime_type {
            "text/plain" => Some("".to_string()),
            "text/html" => Some("<!DOCTYPE html>\n<html>\n<head>\n<title></title>\n</head>\n<body>\n\n</body>\n</html>".to_string()),
            _ => None,
        }
    }
}

struct MimetypeDetector;

impl MimetypeDetector {
    fn detect_path(&self, path: &str) -> String {
        Filesystem::get_mime_type(path)
    }
}

struct User;

impl User {
    fn is_logged_in() -> bool {
        // This would check if the user is logged in
        true // Assuming logged in for this example
    }
}

struct JSON;

impl JSON {
    fn call_check() -> bool {
        // This would perform CSRF check
        true
    }
}

pub async fn new_file(params: web::Form<NewFileParams>) -> impl Responder {
    // Check if user is logged in
    if !User::is_logged_in() {
        return HttpResponse::Unauthorized().finish();
    }

    // Get the parameters
    let dir = match &params.dir {
        Some(d) => format!("/{}", d.trim_matches(|c| c == '/' || c == '\\')),
        None => "".to_string(),
    };
    
    let filename = match &params.filename {
        Some(f) => f.trim_matches(|c| c == '/' || c == '\\').to_string(),
        None => "".to_string(),
    };
    
    let content = params.content.clone().unwrap_or_default();
    
    let source = match &params.source {
        Some(s) => s.trim_matches(|c| c == '/' || c == '\\').to_string(),
        None => "".to_string(),
    };

    let l10n = L10n {};

    // Validate filename
    if filename.trim().is_empty() {
        let error = ErrorResponse {
            success: false,
            data: ErrorData {
                message: l10n.t("File name cannot be empty."),
            },
        };
        return HttpResponse::BadRequest().json(error);
    }

    if filename.contains('/') {
        let error = ErrorResponse {
            success: false,
            data: ErrorData {
                message: l10n.t("File name must not contain \"/\". Please choose a different name."),
            },
        };
        return HttpResponse::BadRequest().json(error);
    }

    let target = format!("{}/{}", dir, filename);

    // Check if file exists
    if Filesystem::file_exists(&target).await {
        let error = ErrorResponse {
            success: false,
            data: ErrorData {
                message: l10n.t_with_args(
                    "The name %s is already used in the folder %s. Please choose a different name.",
                    &[&filename, &dir]
                ),
            },
        };
        return HttpResponse::BadRequest().json(error);
    }

    if !source.is_empty() {
        // Create event source for progress updates
        let event_source = EventSource::new();
        
        // Validate source URL
        if !source.starts_with("https://") && !source.starts_with("http://") {
            let error = ErrorResponse {
                success: false,
                data: ErrorData {
                    message: l10n.t("Not a valid source"),
                },
            };
            return HttpResponse::BadRequest().json(error);
        }

        // Set up progress tracking
        let client = Client::new();
        let resp = match client.get(&source).send().await {
            Ok(r) => r,
            Err(_) => {
                let error = ErrorResponse {
                    success: false,
                    data: ErrorData {
                        message: l10n.t("Error accessing the source file"),
                    },
                };
                return HttpResponse::BadRequest().json(error);
            }
        };

        let total_size = resp.content_length().unwrap_or(0);
        let stream = resp.bytes_stream();
        
        // Clone for the async block
        let target_clone = target.clone();
        let event_source_clone = event_source.clone();
        let l10n_clone = l10n;
        let source_clone = source.clone();

        // Process download in background
        tokio::spawn(async move {
            let result = Filesystem::file_put_contents_stream(&target_clone, stream).await;
            
            if result {
                if let Some(meta) = Filesystem::get_file_info(&target_clone).await {
                    event_source_clone.send("success", json!({
                        "mime": meta.mime,
                        "size": Filesystem::filesize(&target_clone).await,
                        "id": meta.id,
                        "etag": meta.etag
                    })).await;
                }
            } else {
                event_source_clone.send("error", l10n_clone.t_with_args(
                    "Error while downloading %s to %s",
                    &[&source_clone, &target_clone]
                )).await;
            }
            
            event_source_clone.close();
        });

        // Return a streaming response for progress updates
        // This would be implemented differently in a real app
        return HttpResponse::Ok()
            .content_type("text/event-stream")
            .append_header(("Cache-Control", "no-cache"))
            .body("Stream started");
    } else {
        // No source URL, create file with content
        if !JSON::call_check() {
            return HttpResponse::Forbidden().finish();
        }

        let mut success = false;
        let mut content_to_use = content.clone();

        if content_to_use.is_empty() {
            let template_manager = Helper::get_file_template_manager();
            let mimetype_detector = Helper::get_mimetype_detector();
            let mime_type = mimetype_detector.detect_path(&target);
            
            if let Some(template) = template_manager.get_template(&mime_type) {
                content_to_use = template;
            }
        }

        if !content_to_use.is_empty() {
            success = Filesystem::file_put_contents(&target, content_to_use.as_bytes()).await;
        } else {
            success = Filesystem::touch(&target).await;
        }

        if success {
            if let Some(meta) = Filesystem::get_file_info(&target).await {
                return HttpResponse::Ok().json(json!({
                    "success": true,
                    "data": {
                        "id": meta.id,
                        "mime": meta.mime,
                        "size": meta.size,
                        "content": content_to_use,
                        "etag": meta.etag
                    }
                }));
            }
        }
    }

    // If we get here, something went wrong
    let error = ErrorResponse {
        success: false,
        data: ErrorData {
            message: l10n.t("Error when creating the file"),
        },
    };
    
    HttpResponse::InternalServerError().json(error)
}