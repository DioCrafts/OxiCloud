use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use http::StatusCode;
use tracing::debug;

/**
 * ownCloud / SabreDAV
 *
 * @author Markus Goetz
 *
 * @copyright Copyright (C) 2007-2013 Rooftop Solutions. All rights reserved.
 * @author Evert Pot (http://www.rooftopsolutions.nl/)
 * @license http://code.google.com/p/sabredav/wiki/License Modified BSD License
 */

/**
 * This struct reimplements some methods from SabreDavServer.
 *
 * Basically we add handling of depth: infinity.
 *
 * The right way to handle this would have been to submit a patch to the upstream project
 * and grab the corresponding version one merged.
 *
 * Due to time constrains and the limitations where we don't want to upgrade 3rdparty code in
 * this stage of the release cycle we did choose this approach.
 *
 * For ownCloud 7 we will upgrade SabreDAV and submit the patch - if needed.
 */
pub struct OCConnectorSabreServer {
    base: SabreDavServer,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Depth {
    Zero,
    One,
    Infinity,
}

#[derive(Debug, Clone)]
pub struct PropertyValue {
    value: PropertyValueType,
}

#[derive(Debug, Clone)]
pub enum PropertyValueType {
    Int(i64),
    String(String),
    ResourceType(ResourceType),
    GetLastModified(i64),
    SupportedReportSet(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct ResourceType {
    types: Vec<String>,
}

impl ResourceType {
    pub fn new() -> Self {
        Self { types: Vec::new() }
    }

    pub fn add(&mut self, resource_type: String) {
        self.types.push(resource_type);
    }

    pub fn is(&self, resource_type: &str) -> bool {
        self.types.contains(&resource_type.to_string())
    }
}

type PropertyList = HashMap<String, PropertyValue>;
type PropertyMap = HashMap<u16, PropertyList>;

#[derive(Debug)]
pub struct NodeProperties {
    status_code_map: PropertyMap,
    href: String,
}

pub trait ICollection: INode {}
pub trait IFile: INode {
    fn get_size(&self) -> Option<i64>;
    fn get_etag(&self) -> Option<String>;
    fn get_content_type(&self) -> Option<String>;
}

pub trait IQuota: INode {
    fn get_quota_info(&self) -> (i64, i64); // (used, available)
}

pub trait IProperties: INode {
    fn get_properties(&self, property_names: &[String]) -> PropertyList;
}

pub trait INode {
    fn get_name(&self) -> String;
    fn get_last_modified(&self) -> Option<i64>;
}

pub struct Tree {
    // Simplified for this example
}

impl Tree {
    pub fn get_node_for_path(&self, path: &str) -> Result<Arc<dyn INode>, String> {
        // Implementation would go here
        Err("Not implemented".to_string())
    }

    pub fn get_children(&self, path: &str) -> Result<Vec<Arc<dyn INode>>, String> {
        // Implementation would go here
        Ok(Vec::new())
    }
}

pub struct HttpRequest {
    // Simplified for this example
}

impl HttpRequest {
    pub fn get_body(&self, _as_string: bool) -> String {
        // Implementation would go here
        String::new()
    }
}

pub struct HttpResponse {
    // Simplified for this example
}

impl HttpResponse {
    pub fn send_status(&mut self, status: u16) {
        // Implementation would go here
    }

    pub fn set_header(&mut self, name: &str, value: &str) {
        // Implementation would go here
    }

    pub fn send_body(&mut self, body: String) {
        // Implementation would go here
    }
}

pub trait Plugin {
    fn get_features(&self) -> Vec<String>;
    fn get_supported_report_set(&self, path: &str) -> Vec<String>;
}

pub struct SabreDavServer {
    tree: Arc<Tree>,
    http_request: HttpRequest,
    http_response: HttpResponse,
    plugins: Vec<Box<dyn Plugin>>,
    resource_type_mapping: HashMap<String, String>,
}

impl SabreDavServer {
    pub fn get_http_depth(&self, default: i32) -> Depth {
        // Implementation would go here
        if default == 0 {
            Depth::Zero
        } else {
            Depth::One
        }
    }

    pub fn get_http_prefer(&self) -> HashMap<String, bool> {
        // Implementation would go here
        let mut prefs = HashMap::new();
        prefs.insert("return-minimal".to_string(), false);
        prefs
    }

    pub fn parse_prop_find_request(&self, body: String) -> Vec<String> {
        // Implementation would go here
        Vec::new()
    }

    pub fn generate_multi_status(&self, properties: Vec<NodeProperties>, minimal: bool) -> String {
        // Implementation would go here
        String::new()
    }

    pub fn broadcast_event(&self, event: &str, args: Vec<Box<dyn std::any::Any>>) -> Option<bool> {
        // Implementation would go here
        None
    }
}

impl OCConnectorSabreServer {
    pub fn new(base: SabreDavServer) -> Self {
        Self { base }
    }

    pub async fn http_propfind(&mut self, uri: &str) -> Result<(), String> {
        let requested_properties = self.base.parse_prop_find_request(
            self.base.http_request.get_body(true)
        );

        let depth = self.base.get_http_depth(1);

        let new_properties = self.get_properties_for_path(uri, &requested_properties, &depth)?;

        // This is a multi-status response
        self.base.http_response.send_status(207);
        self.base.http_response.set_header("Content-Type", "application/xml; charset=utf-8");
        self.base.http_response.set_header("Vary", "Brief,Prefer");

        // Normally this header is only needed for OPTIONS responses, however..
        // iCal seems to also depend on these being set for PROPFIND. Since
        // this is not harmful, we'll add it.
        let mut features = vec!["1".to_string(), "3".to_string(), "extended-mkcol".to_string()];
        for plugin in &self.base.plugins {
            features.extend(plugin.get_features());
        }

        self.base.http_response.set_header("DAV", &features.join(", "));

        let prefer = self.base.get_http_prefer();
        let minimal = prefer.get("return-minimal").cloned().unwrap_or(false);

        let data = self.base.generate_multi_status(new_properties, minimal);
        self.base.http_response.send_body(data);

        Ok(())
    }

    /**
     * Small helper to support PROPFIND with DEPTH_INFINITY.
     */
    fn add_path_nodes_recursively(
        &self,
        nodes: &mut HashMap<String, Arc<dyn INode>>, 
        path: &str
    ) -> Result<(), String> {
        for child_node in self.base.tree.get_children(path)? {
            let child_path = format!("{}/{}", path, child_node.get_name());
            nodes.insert(child_path.clone(), child_node.clone());
            
            if child_node.as_any().downcast_ref::<dyn ICollection>().is_some() {
                self.add_path_nodes_recursively(nodes, &child_path)?;
            }
        }
        Ok(())
    }

    pub fn get_properties_for_path(
        &self,
        path: &str,
        property_names: &[String],
        depth: &Depth
    ) -> Result<Vec<NodeProperties>, String> {
        let path = path.trim_end_matches('/');

        let mut return_property_list = Vec::new();

        let parent_node = self.base.tree.get_node_for_path(path)?;
        let mut nodes = HashMap::new();
        nodes.insert(path.to_string(), parent_node.clone());

        if *depth == Depth::One && parent_node.as_any().downcast_ref::<dyn ICollection>().is_some() {
            for child_node in self.base.tree.get_children(path)? {
                let child_path = format!("{}/{}", path, child_node.get_name());
                nodes.insert(child_path, child_node);
            }
        } else if *depth == Depth::Infinity && parent_node.as_any().downcast_ref::<dyn ICollection>().is_some() {
            self.add_path_nodes_recursively(&mut nodes, path)?;
        }

        // If the propertyNames array is empty, it means all properties are requested.
        // We shouldn't actually return everything we know though, and only return a
        // sensible list.
        let all_properties = property_names.is_empty();

        for (my_path, node) in nodes {
            let mut current_property_names = property_names.to_vec();

            let mut new_properties = NodeProperties {
                status_code_map: HashMap::new(),
                href: my_path.trim_start_matches('/').to_string(),
            };
            new_properties.status_code_map.insert(200, HashMap::new());
            new_properties.status_code_map.insert(404, HashMap::new());

            if all_properties {
                // Default list of propertyNames, when all properties were requested.
                current_property_names = vec![
                    "{DAV:}getlastmodified".to_string(),
                    "{DAV:}getcontentlength".to_string(),
                    "{DAV:}resourcetype".to_string(),
                    "{DAV:}quota-used-bytes".to_string(),
                    "{DAV:}quota-available-bytes".to_string(),
                    "{DAV:}getetag".to_string(),
                    "{DAV:}getcontenttype".to_string(),
                ];
            }

            // If the resourceType was not part of the list, we manually add it
            // and mark it for removal. We need to know the resourcetype in order
            // to make certain decisions about the entry.
            // WebDAV dictates we should add a / and the end of href's for collections
            let remove_rt = !current_property_names.contains(&"{DAV:}resourcetype".to_string());
            if remove_rt {
                current_property_names.push("{DAV:}resourcetype".to_string());
            }

            // For the broadcast_event implementation, we'd need to pass references to the variables
            // This is simplified here
            let event_args: Vec<Box<dyn std::any::Any>> = vec![
                Box::new(my_path.clone()),
                Box::new(node.clone()),
                Box::new(current_property_names.clone()),
                Box::new(new_properties.clone()),
            ];
            
            let result = self.base.broadcast_event("beforeGetProperties", event_args);
            // If this method explicitly returned false, we must ignore this
            // node as it is inaccessible.
            if result == Some(false) {
                continue;
            }

            if !current_property_names.is_empty() {
                if let Some(prop_node) = node.as_any().downcast_ref::<dyn IProperties>() {
                    let node_properties = prop_node.get_properties(&current_property_names);

                    // The getProperties method may give us too much,
                    // properties, in case the implementor was lazy.
                    //
                    // So as we loop through this list, we will only take the
                    // properties that were actually requested and discard the
                    // rest.
                    let mut to_remove = Vec::new();
                    for (k, current_property_name) in current_property_names.iter().enumerate() {
                        if let Some(prop_value) = node_properties.get(current_property_name) {
                            to_remove.push(k);
                            new_properties.status_code_map.get_mut(&200).unwrap()
                                .insert(current_property_name.clone(), prop_value.clone());
                        }
                    }
                    // Remove in reverse order to maintain indices
                    to_remove.sort_by(|a, b| b.cmp(a));
                    for idx in to_remove {
                        current_property_names.remove(idx);
                    }
                }
            }

            for prop in &current_property_names {
                if new_properties.status_code_map.get(&200).unwrap().contains_key(prop) {
                    continue;
                }

                match prop.as_str() {
                    "{DAV:}getlastmodified" => {
                        if let Some(last_mod) = node.get_last_modified() {
                            new_properties.status_code_map.get_mut(&200).unwrap()
                                .insert(prop.clone(), PropertyValue { 
                                    value: PropertyValueType::GetLastModified(last_mod) 
                                });
                        }
                    },
                    "{DAV:}getcontentlength" => {
                        if let Some(file_node) = node.as_any().downcast_ref::<dyn IFile>() {
                            if let Some(size) = file_node.get_size() {
                                new_properties.status_code_map.get_mut(&200).unwrap()
                                    .insert(prop.clone(), PropertyValue { 
                                        value: PropertyValueType::Int(size) 
                                    });
                            }
                        }
                    },
                    "{DAV:}quota-used-bytes" => {
                        if let Some(quota_node) = node.as_any().downcast_ref::<dyn IQuota>() {
                            let quota_info = quota_node.get_quota_info();
                            new_properties.status_code_map.get_mut(&200).unwrap()
                                .insert(prop.clone(), PropertyValue { 
                                    value: PropertyValueType::Int(quota_info.0) 
                                });
                        }
                    },
                    "{DAV:}quota-available-bytes" => {
                        if let Some(quota_node) = node.as_any().downcast_ref::<dyn IQuota>() {
                            let quota_info = quota_node.get_quota_info();
                            new_properties.status_code_map.get_mut(&200).unwrap()
                                .insert(prop.clone(), PropertyValue { 
                                    value: PropertyValueType::Int(quota_info.1) 
                                });
                        }
                    },
                    "{DAV:}getetag" => {
                        if let Some(file_node) = node.as_any().downcast_ref::<dyn IFile>() {
                            if let Some(etag) = file_node.get_etag() {
                                new_properties.status_code_map.get_mut(&200).unwrap()
                                    .insert(prop.clone(), PropertyValue { 
                                        value: PropertyValueType::String(etag) 
                                    });
                            }
                        }
                    },
                    "{DAV:}getcontenttype" => {
                        if let Some(file_node) = node.as_any().downcast_ref::<dyn IFile>() {
                            if let Some(ct) = file_node.get_content_type() {
                                new_properties.status_code_map.get_mut(&200).unwrap()
                                    .insert(prop.clone(), PropertyValue { 
                                        value: PropertyValueType::String(ct) 
                                    });
                            }
                        }
                    },
                    "{DAV:}supported-report-set" => {
                        let mut reports = Vec::new();
                        for plugin in &self.base.plugins {
                            reports.extend(plugin.get_supported_report_set(&my_path));
                        }
                        new_properties.status_code_map.get_mut(&200).unwrap()
                            .insert(prop.clone(), PropertyValue { 
                                value: PropertyValueType::SupportedReportSet(reports) 
                            });
                    },
                    "{DAV:}resourcetype" => {
                        let mut resource_type = ResourceType::new();
                        for (class_name, resource_type_value) in &self.base.resource_type_mapping {
                            if node.as_any().is::<dyn std::any::Any>() {
                                // This is simplified - in actual code we'd use downcast to check type
                                resource_type.add(resource_type_value.clone());
                            }
                        }
                        new_properties.status_code_map.get_mut(&200).unwrap()
                            .insert(prop.clone(), PropertyValue { 
                                value: PropertyValueType::ResourceType(resource_type) 
                            });
                    },
                    _ => {}
                }

                // If we were unable to find the property, we will list it as 404.
                if !all_properties && !new_properties.status_code_map.get(&200).unwrap().contains_key(prop) {
                    new_properties.status_code_map.get_mut(&404).unwrap().insert(prop.clone(), PropertyValue { 
                        value: PropertyValueType::String(String::new()) 
                    });
                }
            }

            // For the after get properties event
            let after_event_args: Vec<Box<dyn std::any::Any>> = vec![
                Box::new(my_path.trim_matches('/').to_string()),
                Box::new(&mut new_properties),
                Box::new(node.clone()),
            ];
            
            self.base.broadcast_event("afterGetProperties", after_event_args);

            // Its is a WebDAV recommendation to add a trailing slash to collectionnames.
            // Apple's iCal also requires a trailing slash for principals (rfc 3744), though this is non-standard.
            if !my_path.is_empty() {
                if let Some(rt_prop) = new_properties.status_code_map.get(&200).unwrap().get("{DAV:}resourcetype") {
                    if let PropertyValueType::ResourceType(rt) = &rt_prop.value {
                        if rt.is("{DAV:}collection") || rt.is("{DAV:}principal") {
                            new_properties.href = format!("{}/", new_properties.href);
                        }
                    }
                }
            }

            // If the resourcetype property was manually added to the requested property list,
            // we will remove it again.
            if remove_rt {
                new_properties.status_code_map.get_mut(&200).unwrap().remove("{DAV:}resourcetype");
            }

            return_property_list.push(new_properties);
        }

        Ok(return_property_list)
    }
}

// Extension trait needed for downcasting
trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}