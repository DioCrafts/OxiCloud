use actix_web::{web, HttpResponse, Responder};
use actix_web_lab::sse::{self, Sse};
use futures::{stream::Stream, StreamExt};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use serde::{Serialize, Deserialize};
use serde_json::json;

// Simulación de clases/funciones de OC
mod oc {
    use serde::{Serialize, Deserialize};
    
    pub struct User;
    
    impl User {
        pub fn get_user() -> String {
            "current_user".to_string()
        }
        
        pub fn get_users() -> Vec<String> {
            vec!["user1".to_string(), "user2".to_string()]
        }
    }
    
    pub struct JSON;
    
    impl JSON {
        pub fn check_admin_user() -> bool {
            true
        }
    }
    
    pub mod files {
        pub mod utils {
            use std::sync::Arc;
            use futures::channel::mpsc::{self, Sender};
            
            pub struct Scanner {
                user: String,
                file_listeners: Vec<Box<dyn Fn(()) + Send + Sync>>,
                folder_listeners: Vec<Box<dyn Fn(String) + Send + Sync>>,
            }
            
            impl Scanner {
                pub fn new(user: &str) -> Self {
                    Scanner {
                        user: user.to_string(),
                        file_listeners: Vec::new(),
                        folder_listeners: Vec::new(),
                    }
                }
                
                pub fn listen<F>(&mut self, _namespace: &str, event: &str, callback: F)
                where
                    F: Fn(()) + Send + Sync + 'static,
                {
                    if event == "scanFile" {
                        self.file_listeners.push(Box::new(callback));
                    }
                }
                
                pub fn listen_folder<F>(&mut self, _namespace: &str, event: &str, callback: F)
                where
                    F: Fn(String) + Send + Sync + 'static,
                {
                    if event == "scanFolder" {
                        self.folder_listeners.push(Box::new(callback));
                    }
                }
                
                pub async fn scan(&self, dir: &str) {
                    // Simulate scanning files
                    for i in 0..100 {
                        if i % 10 == 0 {
                            for listener in &self.folder_listeners {
                                listener(format!("{}/folder{}", dir, i));
                            }
                        }
                        for listener in &self.file_listeners {
                            listener(());
                        }
                        tokio::time::sleep(Duration::from_millis(50)).await;
                    }
                }
                
                pub async fn background_scan(&self, dir: &str) {
                    // Simulate scanning files in background (less files)
                    for i in 0..50 {
                        if i % 10 == 0 {
                            for listener in &self.folder_listeners {
                                listener(format!("{}/folder{}", dir, i));
                            }
                        }
                        for listener in &self.file_listeners {
                            listener(());
                        }
                        tokio::time::sleep(Duration::from_millis(50)).await;
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
struct ScanListener {
    file_count: Arc<Mutex<usize>>,
    last_count: Arc<Mutex<usize>>,
    sender: Arc<Mutex<sse::Sender>>,
}

impl ScanListener {
    /// Create a new ScanListener
    /// 
    /// @param sender Event source to pass events to
    fn new(sender: sse::Sender) -> Self {
        ScanListener {
            file_count: Arc::new(Mutex::new(0)),
            last_count: Arc::new(Mutex::new(0)),
            sender: Arc::new(Mutex::new(sender)),
        }
    }

    /// Handle folder scan event
    /// 
    /// @param path The folder path
    fn folder(&self, path: String) {
        if let Ok(sender) = self.sender.lock() {
            let _ = sender.send(sse::Event::Data(
                sse::Data::new_json(&json!({"type": "folder", "data": path})).unwrap()
            ));
        }
    }

    /// Handle file scan event
    fn file(&self, _: ()) {
        let mut file_count = self.file_count.lock().unwrap();
        *file_count += 1;
        
        let mut last_count = self.last_count.lock().unwrap();
        if *file_count > *last_count + 20 {
            // Send a count update every 20 files
            *last_count = *file_count;
            if let Ok(sender) = self.sender.lock() {
                let _ = sender.send(sse::Event::Data(
                    sse::Data::new_json(&json!({"type": "count", "data": *file_count})).unwrap()
                ));
            }
        }
    }

    /// Get the total file count
    fn get_count(&self) -> usize {
        *self.file_count.lock().unwrap()
    }
}

#[derive(Deserialize)]
struct ScanParams {
    force: Option<String>,
    dir: Option<String>,
    users: Option<String>,
}

async fn scan(
    query: web::Query<ScanParams>,
) -> impl Responder {
    // Set time limit to unlimited (simulated - in Rust we'll handle this differently)
    // Session write close also not needed in Rust

    let force = query.force.as_deref() == Some("true");
    let dir = query.dir.as_deref().unwrap_or("");
    
    let users = if let Some(users_param) = &query.users {
        if !oc::JSON::check_admin_user() {
            return HttpResponse::Forbidden().finish();
        }
        
        if users_param == "all" {
            oc::User::get_users()
        } else {
            match serde_json::from_str::<Vec<String>>(users_param) {
                Ok(parsed_users) => parsed_users,
                Err(_) => return HttpResponse::BadRequest().finish(),
            }
        }
    } else {
        vec![oc::User::get_user()]
    };
    
    Sse::from_stream(scan_users_stream(users, dir.to_string(), force))
        .with_keep_alive(Duration::from_secs(15))
        .with_retry_duration(Duration::from_secs(5))
}

fn scan_users_stream(
    users: Vec<String>,
    dir: String,
    force: bool,
) -> impl Stream<Item = Result<sse::Event, actix_web::Error>> {
    let (tx, rx) = sse::channel(100);
    let listener = ScanListener::new(tx.clone());
    
    tokio::spawn(async move {
        for user in users {
            let _ = tx.send(sse::Event::Data(
                sse::Data::new_json(&json!({"type": "user", "data": user})).unwrap()
            ));

            let listener_clone = listener.clone();
            let mut scanner = oc::files::utils::Scanner::new(&user);
            
            let file_listener = listener_clone.clone();
            scanner.listen(
                r"\OC\Files\Utils\Scanner",
                "scanFile",
                move |_| { file_listener.file(()); }
            );
            
            let folder_listener = listener_clone.clone();
            scanner.listen_folder(
                r"\OC\Files\Utils\Scanner", 
                "scanFolder",
                move |path| { folder_listener.folder(path); }
            );

            if force {
                scanner.scan(&dir).await;
            } else {
                scanner.background_scan(&dir).await;
            }
        }

        let count = listener.get_count();
        let _ = tx.send(sse::Event::Data(
            sse::Data::new_json(&json!({"type": "done", "data": count})).unwrap()
        ));
        
        // Close the stream
        drop(tx);
    });
    
    rx
}

// This would be added to your app's routes configuration
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/apps/files/ajax/scan", web::get().to(scan));
}