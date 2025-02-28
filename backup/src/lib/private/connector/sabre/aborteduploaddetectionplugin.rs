use std::sync::{Arc, Mutex};

/**
 * Copyright (c) 2013 Thomas Müller <thomas.mueller@tmit.eu>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

/**
 * Struct AbortedUploadDetectionPlugin
 *
 * This plugin will verify if the uploaded data has been stored completely.
 * This is done by comparing the content length of the request with the file size on storage.
 */
pub struct AbortedUploadDetectionPlugin {
    /// Reference to main server object
    server: Option<Arc<Mutex<SabreDavServer>>>,
    
    /// is kept public to allow overwrite for unit testing
    pub file_view: Option<Arc<FilesView>>,
}

impl AbortedUploadDetectionPlugin {
    pub fn new() -> Self {
        Self {
            server: None,
            file_view: None,
        }
    }

    /**
     * @return String
     */
    pub fn get_length(&self) -> Option<u64> {
        if let Some(server) = &self.server {
            let server = server.lock().unwrap();
            let req = &server.http_request;
            
            if let Some(length) = req.get_header("X-Expected-Entity-Length") {
                return length.parse::<u64>().ok();
            }
            
            if let Some(length) = req.get_header("Content-Length") {
                return length.parse::<u64>().ok();
            }
        }
        
        None
    }

    /**
     * @return FilesView
     */
    pub fn get_file_view(&mut self) -> Arc<FilesView> {
        if self.file_view.is_none() {
            // initialize fileView
            self.file_view = Some(Arc::new(FilesSystem::get_view()));
        }

        self.file_view.as_ref().unwrap().clone()
    }
    
    /**
     * @param file_path String
     * @param node Option<Arc<dyn SabreDavINode>>
     * @throws SabreDavExceptionBadRequest
     */
    pub fn verify_content_length(&mut self, file_path: &str, _node: Option<Arc<dyn SabreDavINode>>) -> Result<(), SabreDavExceptionBadRequest> {
        // we should only react on PUT which is used for upload
        // e.g. with LOCK this will not work, but LOCK uses createFile() as well
        if let Some(server) = &self.server {
            let server = server.lock().unwrap();
            if server.http_request.get_method() != "PUT" {
                return Ok(());
            }
            
            // ownCloud chunked upload will be handled in its own plugin
            if server.http_request.get_header("OC-Chunked").is_some() {
                return Ok(());
            }
        } else {
            return Ok(());
        }

        // compare expected and actual size
        let expected = self.get_length();
        if expected.is_none() {
            return Ok(());
        }
        let expected = expected.unwrap();
        
        let file_view = self.get_file_view();
        let actual = file_view.filesize(file_path);
        
        if actual != expected {
            file_view.unlink(file_path);
            return Err(SabreDavExceptionBadRequest::new(
                format!("expected filesize {} got {}", expected, actual)
            ));
        }

        Ok(())
    }
}

impl SabreDavServerPlugin for AbortedUploadDetectionPlugin {
    /**
     * This initializes the plugin.
     *
     * This function is called by Sabre_DAV_Server, after
     * addPlugin is called.
     *
     * This method should set up the requires event subscriptions.
     *
     * @param server SabreDavServer
     */
    fn initialize(&mut self, server: Arc<Mutex<SabreDavServer>>) {
        self.server = Some(server.clone());
        
        {
            let mut server = server.lock().unwrap();
            server.subscribe_event("afterCreateFile", Box::new(EventHandler::new(self, |plugin, file_path, node| {
                plugin.verify_content_length(file_path, node)
            })), 10);
            
            server.subscribe_event("afterWriteContent", Box::new(EventHandler::new(self, |plugin, file_path, node| {
                plugin.verify_content_length(file_path, node)
            })), 10);
        }
    }
}

// Types for integration with the Sabre/DAV framework
pub trait SabreDavINode {}

pub struct SabreDavServer {
    pub http_request: HttpRequest,
}

impl SabreDavServer {
    pub fn subscribe_event(&mut self, event: &str, handler: Box<dyn EventHandler>, priority: i32) {
        // Implementation would go here
    }
}

pub struct HttpRequest;

impl HttpRequest {
    pub fn get_method(&self) -> &str {
        // Implementation would go here
        "GET"
    }
    
    pub fn get_header(&self, name: &str) -> Option<String> {
        // Implementation would go here
        None
    }
}

pub struct SabreDavExceptionBadRequest {
    message: String,
}

impl SabreDavExceptionBadRequest {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

pub trait SabreDavServerPlugin {
    fn initialize(&mut self, server: Arc<Mutex<SabreDavServer>>);
}

pub struct FilesView;

impl FilesView {
    pub fn filesize(&self, path: &str) -> u64 {
        // Implementation would go here
        0
    }
    
    pub fn unlink(&self, path: &str) {
        // Implementation would go here
    }
}

pub struct FilesSystem;

impl FilesSystem {
    pub fn get_view() -> FilesView {
        // Implementation would go here
        FilesView
    }
}

pub trait EventHandler {
    fn handle(&mut self, file_path: &str, node: Option<Arc<dyn SabreDavINode>>) -> Result<(), SabreDavExceptionBadRequest>;
}

struct EventHandlerImpl<F, T>
where
    F: FnMut(&mut T, &str, Option<Arc<dyn SabreDavINode>>) -> Result<(), SabreDavExceptionBadRequest>,
    T: ?Sized,
{
    callback: F,
    plugin: *mut T,
}

impl<F, T> EventHandlerImpl<F, T>
where
    F: FnMut(&mut T, &str, Option<Arc<dyn SabreDavINode>>) -> Result<(), SabreDavExceptionBadRequest>,
    T: ?Sized,
{
    fn new(plugin: &mut T, callback: F) -> Self {
        Self {
            callback,
            plugin: plugin as *mut T,
        }
    }
}

impl<F, T> EventHandler for EventHandlerImpl<F, T>
where
    F: FnMut(&mut T, &str, Option<Arc<dyn SabreDavINode>>) -> Result<(), SabreDavExceptionBadRequest>,
    T: ?Sized,
{
    fn handle(&mut self, file_path: &str, node: Option<Arc<dyn SabreDavINode>>) -> Result<(), SabreDavExceptionBadRequest> {
        unsafe {
            (self.callback)(&mut *self.plugin, file_path, node)
        }
    }
}

pub fn EventHandler<F, T>(plugin: &mut T, callback: F) -> EventHandlerImpl<F, T>
where
    F: FnMut(&mut T, &str, Option<Arc<dyn SabreDavINode>>) -> Result<(), SabreDavExceptionBadRequest>,
    T: ?Sized,
{
    EventHandlerImpl::new(plugin, callback)
}