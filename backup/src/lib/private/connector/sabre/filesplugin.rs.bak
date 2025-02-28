use std::collections::HashMap;

/// Namespace constant
pub const NS_OWNCLOUD: &str = "http://owncloud.org/ns";

/// FilesPlugin for the Sabre DAV server
///
/// Handles ownCloud specific properties and features for files
pub struct FilesPlugin {
    server: Option<Box<dyn DavServer>>,
}

/// Trait representing a DAV server node
pub trait DavNode {
    fn get_file_id(&self) -> Option<String>;
}

/// Trait representing the Sabre DAV server
pub trait DavServer {
    fn subscribe_event(&mut self, event: &str, callback: Box<dyn Fn(&str, Option<&dyn DavNode>) + Send + Sync>);
    fn set_xml_namespace(&mut self, namespace: &str, prefix: &str);
    fn add_protected_property(&mut self, property: &str);
    fn get_http_response(&mut self) -> &mut HttpResponse;
}

/// HTTP Response abstraction
pub trait HttpResponse {
    fn set_header(&mut self, name: &str, value: &str);
}

impl FilesPlugin {
    /// Create a new instance of the FilesPlugin
    pub fn new() -> Self {
        FilesPlugin { server: None }
    }

    /// Initialize the plugin with the server
    ///
    /// This function is called by the DAV server after
    /// add_plugin is called.
    ///
    /// This method sets up the required event subscriptions.
    pub fn initialize(&mut self, server: Box<dyn DavServer>) {
        let ns_owncloud = NS_OWNCLOUD.to_string();
        
        server.set_xml_namespace(&ns_owncloud, "oc");
        server.add_protected_property(&format!("{{{}}}", ns_owncloud) + "id");

        self.server = Some(server);
        
        let server_ref = self.server.as_mut().unwrap();
        
        // Need to clone ns_owncloud for the closure
        let ns_clone = ns_owncloud.clone();
        server_ref.subscribe_event("beforeGetProperties", Box::new(move |path, node| {
            if let Some(node) = node {
                if let Some(oc_node) = node.downcast_ref::<dyn OcDavNode>() {
                    Self::before_get_properties(path, oc_node, &ns_clone);
                }
            }
        }));

        server_ref.subscribe_event("afterCreateFile", Box::new(|path, node| {
            if let Some(node) = node {
                if let Some(server_ref) = self.server.as_mut() {
                    Self::send_file_id_header(server_ref, path, node);
                }
            }
        }));

        server_ref.subscribe_event("afterWriteContent", Box::new(|path, node| {
            if let Some(node) = node {
                if let Some(server_ref) = self.server.as_mut() {
                    Self::send_file_id_header(server_ref, path, node);
                }
            }
        }));
    }

    /// Add all ownCloud-specific properties
    ///
    /// Called before properties are retrieved for a node
    fn before_get_properties(
        path: &str, 
        node: &dyn OcDavNode, 
        ns_owncloud: &str
    ) -> HashMap<String, String> {
        let mut returned_properties = HashMap::new();
        
        let fileid_propertyname = format!("{{{}}}", ns_owncloud) + "id";
        
        if let Some(file_id) = node.get_file_id() {
            returned_properties.insert(fileid_propertyname, file_id);
        }
        
        returned_properties
    }

    /// Sends the file ID as a header in the HTTP response
    fn send_file_id_header(server: &mut dyn DavServer, file_path: &str, node: &dyn DavNode) {
        if let Some(file_id) = node.get_file_id() {
            let http_response = server.get_http_response();
            http_response.set_header("OC-FileId", &file_id);
        }
    }
}

/// Trait extending DavNode with ownCloud specific functionality
pub trait OcDavNode: DavNode {
    // Additional ownCloud specific methods could be added here
}