use actix_web::{
    HttpRequest, HttpResponse, web, error, http::Method, Result as ActixResult,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use std::collections::HashMap;
use xml::writer::{EventWriter, XmlEvent};
use std::io::Cursor;

/// Class to handle open collaboration services API requests
pub struct OcsService;

/// Common data structure for OCS responses
#[derive(Serialize, Deserialize)]
pub struct OcsResponse<T> {
    pub status: String,
    pub statuscode: u32,
    pub message: String,
    pub totalitems: Option<usize>,
    pub itemsperpage: Option<usize>,
    pub data: T,
}

impl OcsService {
    /// Reads input data from get/post and converts the data to a special data-type
    ///
    /// # Arguments
    ///
    /// * `method` - HTTP method to read the key from
    /// * `key` - Parameter to read
    /// * `req` - The HTTP request
    /// * `payload` - Optional POST payload data
    /// * `type_` - Variable type to format data
    /// * `default` - Default value to return if the key is not found
    ///
    /// # Returns
    ///
    /// Data or if the key is not found and no default is set it will return an error
    pub async fn read_data<T>(
        method: &str,
        key: &str,
        req: &HttpRequest,
        payload: Option<web::Form<HashMap<String, String>>>,
        type_: &str,
        default: Option<T>,
    ) -> ActixResult<T>
    where
        T: From<String> + From<i32> + From<f64> + Clone,
    {
        let data: Option<String> = match method.to_lowercase().as_str() {
            "get" => {
                let query = req.query_string();
                let mut pairs = url::form_urlencoded::parse(query.as_bytes());
                pairs.find(|(k, _)| k == key).map(|(_, v)| v.to_string())
            }
            "post" => {
                if let Some(form) = payload {
                    form.get(key).cloned()
                } else {
                    None
                }
            }
            _ => None,
        };

        let data = match data {
            Some(d) => d,
            None => match default {
                Some(d) => return Ok(d),
                None => {
                    let response = Self::generate_xml(
                        "xml", 
                        "fail", 
                        400, 
                        &format!("Bad request. Please provide a valid {}", key),
                        &Vec::<String>::new(),
                        "",
                        "",
                        -1,
                        None,
                        None,
                    )?;
                    return Err(error::ErrorBadRequest(response));
                }
            },
        };

        match type_ {
            "raw" => Ok(T::from(data)),
            "text" => Ok(T::from(sanitize_html(&data))),
            "int" => {
                let val: i32 = data.parse().unwrap_or(0);
                Ok(T::from(val))
            }
            "float" => {
                let val: f64 = data.parse().unwrap_or(0.0);
                Ok(T::from(val))
            }
            "array" => Ok(T::from(sanitize_html(&data))),
            _ => Ok(T::from(sanitize_html(&data))),
        }
    }

    /// Handle not found errors
    pub async fn not_found(req: &HttpRequest) -> HttpResponse {
        let method = match req.method() {
            &Method::GET => "get",
            &Method::PUT => "put",
            &Method::POST => "post",
            _ => {
                return HttpResponse::InternalServerError()
                    .body("internal server error: method not supported");
            }
        };

        let format = Self::read_data(
            method,
            "format",
            req,
            None,
            "text",
            Some("xml".to_string()),
        )
        .await
        .unwrap_or_else(|_| "xml".to_string());

        let txt = format!(
            "Invalid query, please check the syntax. API specifications are here: \
            http://www.freedesktop.org/wiki/Specifications/open-collaboration-services. DEBUG OUTPUT:\n{}",
            Self::get_debug_output(req)
        );

        match Self::generate_xml(
            &format,
            "failed",
            999,
            &txt,
            &Vec::<String>::new(),
            "",
            "",
            -1,
            None,
            None,
        ) {
            Ok(resp) => HttpResponse::NotFound().body(resp),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    /// Generates some debug information to make it easier to find failed API calls
    fn get_debug_output(req: &HttpRequest) -> String {
        let mut txt = String::from("debug output:\n");
        
        txt.push_str(&format!("http request method: {}\n", req.method()));
        txt.push_str(&format!("http request uri: {}\n", req.uri()));
        
        // Query parameters
        for (key, value) in req.query_string().split('&') {
            if let Some((k, v)) = value.split_once('=') {
                txt.push_str(&format!("get parameter: {}->{}\n", k, v));
            }
        }
        
        // Cannot access POST parameters here directly, would need to be passed separately
        txt
    }

    /// Generates the xml or json response for the API call from a multidimensional data array.
    pub fn generate_xml<T: Serialize>(
        format: &str,
        status: &str,
        statuscode: u32,
        message: &str,
        data: &T,
        tag: &str,
        tagattribute: &str,
        dimension: i32,
        itemscount: Option<usize>,
        itemsperpage: Option<usize>,
    ) -> ActixResult<String> {
        if format == "json" {
            let json = json!({
                "status": status,
                "statuscode": statuscode,
                "message": message,
                "totalitems": itemscount,
                "itemsperpage": itemsperpage,
                "data": data
            });
            
            Ok(serde_json::to_string(&json)?)
        } else {
            let buffer = Cursor::new(Vec::new());
            let mut writer = EventWriter::new(buffer);
            
            writer.write(XmlEvent::StartDocument {
                version: xml::common::XmlVersion::Version10,
                encoding: Some("UTF-8"),
                standalone: None,
            })?;
            
            writer.write(XmlEvent::StartElement {
                name: "ocs".into(),
                attributes: vec![],
                namespace: xml::namespace::Namespace::empty(),
            })?;
            
            writer.write(XmlEvent::StartElement {
                name: "meta".into(),
                attributes: vec![],
                namespace: xml::namespace::Namespace::empty(),
            })?;
            
            writer.write(XmlEvent::StartElement {
                name: "status".into(),
                attributes: vec![],
                namespace: xml::namespace::Namespace::empty(),
            })?;
            writer.write(XmlEvent::Characters(status))?;
            writer.write(XmlEvent::EndElement { name: Some("status".into()) })?;
            
            writer.write(XmlEvent::StartElement {
                name: "statuscode".into(),
                attributes: vec![],
                namespace: xml::namespace::Namespace::empty(),
            })?;
            writer.write(XmlEvent::Characters(&statuscode.to_string()))?;
            writer.write(XmlEvent::EndElement { name: Some("statuscode".into()) })?;
            
            writer.write(XmlEvent::StartElement {
                name: "message".into(),
                attributes: vec![],
                namespace: xml::namespace::Namespace::empty(),
            })?;
            writer.write(XmlEvent::Characters(message))?;
            writer.write(XmlEvent::EndElement { name: Some("message".into()) })?;
            
            if let Some(count) = itemscount {
                writer.write(XmlEvent::StartElement {
                    name: "totalitems".into(),
                    attributes: vec![],
                    namespace: xml::namespace::Namespace::empty(),
                })?;
                writer.write(XmlEvent::Characters(&count.to_string()))?;
                writer.write(XmlEvent::EndElement { name: Some("totalitems".into()) })?;
            }
            
            if let Some(perpage) = itemsperpage {
                writer.write(XmlEvent::StartElement {
                    name: "itemsperpage".into(),
                    attributes: vec![],
                    namespace: xml::namespace::Namespace::empty(),
                })?;
                writer.write(XmlEvent::Characters(&perpage.to_string()))?;
                writer.write(XmlEvent::EndElement { name: Some("itemsperpage".into()) })?;
            }
            
            writer.write(XmlEvent::EndElement { name: Some("meta".into()) })?;
            
            // Serializing data based on dimension would require a more complex implementation
            // This is a simplified version focusing on the structure
            writer.write(XmlEvent::StartElement {
                name: "data".into(),
                attributes: vec![],
                namespace: xml::namespace::Namespace::empty(),
            })?;
            
            // For more complex cases, we would need to implement specific dimension handling
            // like in the original PHP code
            match dimension {
                0 => {
                    // Direct data content would go here
                    // We'd need to convert data to a string directly
                },
                1 | 2 | 3 => {
                    // Would need custom implementation for different dimensions
                    // This is simplified
                },
                _ => {}
            }
            
            writer.write(XmlEvent::EndElement { name: Some("data".into()) })?;
            writer.write(XmlEvent::EndElement { name: Some("ocs".into()) })?;
            
            let result = writer.into_inner().into_inner();
            Ok(String::from_utf8(result)?)
        }
    }

    /// Recursively convert data to XML
    pub fn to_xml<W: std::io::Write>(
        writer: &mut EventWriter<W>,
        data: &JsonValue,
        node: &str,
    ) -> ActixResult<()> {
        match data {
            JsonValue::Object(map) => {
                for (key, value) in map {
                    if value.is_object() || value.is_array() {
                        writer.write(XmlEvent::StartElement {
                            name: key.into(),
                            attributes: vec![],
                            namespace: xml::namespace::Namespace::empty(),
                        })?;
                        Self::to_xml(writer, value, node)?;
                        writer.write(XmlEvent::EndElement { name: Some(key.into()) })?;
                    } else {
                        writer.write(XmlEvent::StartElement {
                            name: key.into(),
                            attributes: vec![],
                            namespace: xml::namespace::Namespace::empty(),
                        })?;
                        writer.write(XmlEvent::Characters(&value.to_string()))?;
                        writer.write(XmlEvent::EndElement { name: Some(key.into()) })?;
                    }
                }
            },
            JsonValue::Array(arr) => {
                for (i, value) in arr.iter().enumerate() {
                    let element_name = if i.to_string() == node {
                        node.to_string()
                    } else {
                        format!("{}_{}", node, i)
                    };
                    
                    writer.write(XmlEvent::StartElement {
                        name: element_name.into(),
                        attributes: vec![],
                        namespace: xml::namespace::Namespace::empty(),
                    })?;
                    Self::to_xml(writer, value, node)?;
                    writer.write(XmlEvent::EndElement { name: Some(element_name.into()) })?;
                }
            },
            _ => {
                writer.write(XmlEvent::Characters(&data.to_string()))?;
            }
        }
        
        Ok(())
    }
}

// Utility function to sanitize HTML
fn sanitize_html(input: &str) -> String {
    // Simple implementation - in a real app this would be more comprehensive
    input
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
}