use crate::application::dtos::file_dto::FileDto;
use crate::application::dtos::folder_dto::FolderDto;
use chrono::Utc;
use quick_xml::{
    Reader, Writer,
    events::{BytesEnd, BytesStart, BytesText, Event},
};
/**
 * WebDAV Adapter Module
 *
 * This module provides conversion between WebDAV protocol XML structures and OxiCloud domain objects.
 * It handles parsing WebDAV request XML and generating WebDAV response XML according to RFC 4918.
 */
use std::io::{BufReader, Read, Write};

/// Result type for WebDAV operations
pub type Result<T> = std::result::Result<T, WebDavError>;

/// Error type for WebDAV operations
#[derive(Debug)]
pub enum WebDavError {
    XmlError(quick_xml::Error),
    IoError(std::io::Error),
    ParseError(String),
}

impl From<quick_xml::Error> for WebDavError {
    fn from(err: quick_xml::Error) -> Self {
        WebDavError::XmlError(err)
    }
}

impl From<std::io::Error> for WebDavError {
    fn from(err: std::io::Error) -> Self {
        WebDavError::IoError(err)
    }
}

impl std::fmt::Display for WebDavError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebDavError::XmlError(e) => write!(f, "XML error: {}", e),
            WebDavError::IoError(e) => write!(f, "IO error: {}", e),
            WebDavError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

/// Qualified name with namespace and local name
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct QualifiedName {
    pub namespace: String,
    pub name: String,
}

impl QualifiedName {
    pub fn new<S: Into<String>>(namespace: S, name: S) -> Self {
        Self {
            namespace: namespace.into(),
            name: name.into(),
        }
    }
}

impl std::fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.namespace.is_empty() {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{{{}}}{}", self.namespace, self.name)
        }
    }
}

/// PROPFIND request type
#[derive(Debug, PartialEq)]
pub enum PropFindType {
    /// Request all properties
    AllProp,
    /// Request property names only
    PropName,
    /// Request specific properties
    Prop(Vec<QualifiedName>),
}

/// PROPFIND request
#[derive(Debug)]
pub struct PropFindRequest {
    pub prop_find_type: PropFindType,
}

/// WebDAV property value
#[derive(Debug, Clone)]
pub struct PropValue {
    pub name: QualifiedName,
    pub value: Option<String>,
}

/// WebDAV lock information
#[derive(Debug, Clone)]
pub struct LockInfo {
    pub token: String,
    pub owner: Option<String>,
    pub depth: String,
    pub timeout: Option<String>,
    pub scope: LockScope,
    pub type_: LockType,
}

/// Lock scope (exclusive or shared)
#[derive(Debug, Clone, PartialEq)]
pub enum LockScope {
    Exclusive,
    Shared,
}

/// Lock type (currently only write)
#[derive(Debug, Clone, PartialEq)]
pub enum LockType {
    Write,
}

/// WebDAV adapter for converting between XML and domain objects
pub struct WebDavAdapter;

impl WebDavAdapter {
    /// Parse a PROPFIND XML request
    pub fn parse_propfind<R: Read>(reader: R) -> Result<PropFindRequest> {
        let mut xml_reader = Reader::from_reader(BufReader::new(reader));
        xml_reader.config_mut().trim_text(true);

        let mut buffer = Vec::new();
        let mut in_propfind = false;
        let mut in_prop = false;
        let mut in_allprop = false;
        let mut in_propname = false;
        let mut props = Vec::new();

        loop {
            match xml_reader.read_event_into(&mut buffer) {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name.as_ref()).unwrap_or("");

                    if name_str == "propfind" || name_str.ends_with(":propfind") {
                        in_propfind = true;
                    } else if in_propfind && (name_str == "prop" || name_str.ends_with(":prop")) {
                        in_prop = true;
                    } else if in_propfind
                        && (name_str == "allprop" || name_str.ends_with(":allprop"))
                    {
                        in_allprop = true;
                    } else if in_propfind
                        && (name_str == "propname" || name_str.ends_with(":propname"))
                    {
                        in_propname = true;
                    } else if in_prop {
                        // Add property to request
                        let namespace = Self::extract_namespace(name_str);
                        let prop_name = Self::extract_local_name(name_str);

                        props.push(QualifiedName::new(namespace, prop_name));
                    }
                }
                Ok(Event::End(ref e)) => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name.as_ref()).unwrap_or("");

                    if name_str == "propfind" || name_str.ends_with(":propfind") {
                        in_propfind = false;
                    } else if name_str == "prop" || name_str.ends_with(":prop") {
                        in_prop = false;
                    } else if name_str == "allprop" || name_str.ends_with(":allprop") {
                        in_allprop = false;
                    } else if name_str == "propname" || name_str.ends_with(":propname") {
                        in_propname = false;
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name.as_ref()).unwrap_or("");

                    if in_propfind && (name_str == "allprop" || name_str.ends_with(":allprop")) {
                        in_allprop = true;
                    } else if in_propfind
                        && (name_str == "propname" || name_str.ends_with(":propname"))
                    {
                        in_propname = true;
                    } else if in_prop {
                        // Add property to request (empty element)
                        let namespace = Self::extract_namespace(name_str);
                        let prop_name = Self::extract_local_name(name_str);

                        props.push(QualifiedName::new(namespace, prop_name));
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(WebDavError::XmlError(e)),
                _ => (),
            }

            buffer.clear();
        }

        let prop_find_type = if in_allprop {
            PropFindType::AllProp
        } else if in_propname {
            PropFindType::PropName
        } else {
            PropFindType::Prop(props)
        };

        Ok(PropFindRequest { prop_find_type })
    }

    /// Generate a PROPFIND response for files and folders
    pub fn generate_propfind_response<W: Write>(
        writer: W,
        folder: Option<&FolderDto>,
        files: &[FileDto],
        subfolders: &[FolderDto],
        request: &PropFindRequest,
        _depth: &str,
        base_href: &str,
    ) -> Result<()> {
        let mut xml_writer = Writer::new(writer);

        // Start multistatus response
        xml_writer.write_event(Event::Start(
            BytesStart::new("D:multistatus").with_attributes([("xmlns:D", "DAV:")]),
        ))?;

        // Add response for current folder if provided
        if let Some(folder) = folder {
            Self::write_folder_response(&mut xml_writer, folder, request, base_href)?;
        }

        // If depth allows, add responses for files and subfolders
        if _depth != "0" {
            // Add responses for files
            for file in files {
                Self::write_file_response(
                    &mut xml_writer,
                    file,
                    request,
                    &format!("{}{}", base_href, file.name),
                )?;
            }

            // Add responses for subfolders
            for subfolder in subfolders {
                Self::write_folder_response(
                    &mut xml_writer,
                    subfolder,
                    request,
                    &format!("{}{}/", base_href, subfolder.name),
                )?;
            }
        }

        // End multistatus
        xml_writer.write_event(Event::End(BytesEnd::new("D:multistatus")))?;

        Ok(())
    }

    /// Generate a PROPFIND response for a single file
    pub fn generate_propfind_response_for_file<W: Write>(
        writer: W,
        file: &FileDto,
        request: &PropFindRequest,
        _depth: &str,
        href: &str,
    ) -> Result<()> {
        let mut xml_writer = Writer::new(writer);

        // Start multistatus response
        xml_writer.write_event(Event::Start(
            BytesStart::new("D:multistatus").with_attributes([("xmlns:D", "DAV:")]),
        ))?;

        // Add response for file
        Self::write_file_response(&mut xml_writer, file, request, href)?;

        // End multistatus
        xml_writer.write_event(Event::End(BytesEnd::new("D:multistatus")))?;

        Ok(())
    }

    /// Write folder properties as a response
    fn write_folder_response<W: Write>(
        xml_writer: &mut Writer<W>,
        folder: &FolderDto,
        request: &PropFindRequest,
        href: &str,
    ) -> Result<()> {
        // Start response element
        xml_writer.write_event(Event::Start(BytesStart::new("D:response")))?;

        // Write href
        xml_writer.write_event(Event::Start(BytesStart::new("D:href")))?;
        xml_writer.write_event(Event::Text(BytesText::new(href)))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:href")))?;

        // Write propstat
        xml_writer.write_event(Event::Start(BytesStart::new("D:propstat")))?;

        // Start prop
        xml_writer.write_event(Event::Start(BytesStart::new("D:prop")))?;

        // Write properties based on request type
        match &request.prop_find_type {
            PropFindType::AllProp => {
                // Write all standard properties for a folder
                Self::write_folder_standard_props(xml_writer, folder)?;
            }
            PropFindType::PropName => {
                // Write only property names (empty elements)
                Self::write_folder_prop_names(xml_writer)?;
            }
            PropFindType::Prop(props) => {
                // Write requested properties
                Self::write_folder_requested_props(xml_writer, folder, props)?;
            }
        }

        // End prop
        xml_writer.write_event(Event::End(BytesEnd::new("D:prop")))?;

        // Write status
        xml_writer.write_event(Event::Start(BytesStart::new("D:status")))?;
        xml_writer.write_event(Event::Text(BytesText::new("HTTP/1.1 200 OK")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:status")))?;

        // End propstat
        xml_writer.write_event(Event::End(BytesEnd::new("D:propstat")))?;

        // End response
        xml_writer.write_event(Event::End(BytesEnd::new("D:response")))?;

        Ok(())
    }

    /// Write file properties as a response
    fn write_file_response<W: Write>(
        xml_writer: &mut Writer<W>,
        file: &FileDto,
        request: &PropFindRequest,
        href: &str,
    ) -> Result<()> {
        // Start response element
        xml_writer.write_event(Event::Start(BytesStart::new("D:response")))?;

        // Write href
        xml_writer.write_event(Event::Start(BytesStart::new("D:href")))?;
        xml_writer.write_event(Event::Text(BytesText::new(href)))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:href")))?;

        // Write propstat
        xml_writer.write_event(Event::Start(BytesStart::new("D:propstat")))?;

        // Start prop
        xml_writer.write_event(Event::Start(BytesStart::new("D:prop")))?;

        // Write properties based on request type
        match &request.prop_find_type {
            PropFindType::AllProp => {
                // Write all standard properties for a file
                Self::write_file_standard_props(xml_writer, file)?;
            }
            PropFindType::PropName => {
                // Write only property names (empty elements)
                Self::write_file_prop_names(xml_writer)?;
            }
            PropFindType::Prop(props) => {
                // Write requested properties
                Self::write_file_requested_props(xml_writer, file, props)?;
            }
        }

        // End prop
        xml_writer.write_event(Event::End(BytesEnd::new("D:prop")))?;

        // Write status
        xml_writer.write_event(Event::Start(BytesStart::new("D:status")))?;
        xml_writer.write_event(Event::Text(BytesText::new("HTTP/1.1 200 OK")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:status")))?;

        // End propstat
        xml_writer.write_event(Event::End(BytesEnd::new("D:propstat")))?;

        // End response
        xml_writer.write_event(Event::End(BytesEnd::new("D:response")))?;

        Ok(())
    }

    /// Write standard folder properties
    fn write_folder_standard_props<W: Write>(
        xml_writer: &mut Writer<W>,
        folder: &FolderDto,
    ) -> Result<()> {
        // Resource type (collection)
        xml_writer.write_event(Event::Start(BytesStart::new("D:resourcetype")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:collection")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:resourcetype")))?;

        // Display name
        xml_writer.write_event(Event::Start(BytesStart::new("D:displayname")))?;
        xml_writer.write_event(Event::Text(BytesText::new(&folder.name)))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:displayname")))?;

        // Creation date
        xml_writer.write_event(Event::Start(BytesStart::new("D:creationdate")))?;

        // Convert u64 timestamp to DateTime
        let created_at = chrono::DateTime::<Utc>::from_timestamp(folder.created_at as i64, 0)
            .unwrap_or_else(Utc::now);

        xml_writer.write_event(Event::Text(BytesText::new(&created_at.to_rfc3339())))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:creationdate")))?;

        // Last modified
        xml_writer.write_event(Event::Start(BytesStart::new("D:getlastmodified")))?;

        // Convert u64 timestamp to DateTime
        let modified_at = chrono::DateTime::<Utc>::from_timestamp(folder.modified_at as i64, 0)
            .unwrap_or_else(Utc::now);

        xml_writer.write_event(Event::Text(BytesText::new(&modified_at.to_rfc2822())))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:getlastmodified")))?;

        // Other standard properties
        xml_writer.write_event(Event::Start(BytesStart::new("D:getetag")))?;
        xml_writer.write_event(Event::Text(BytesText::new(&format!("\"{}\"", folder.id))))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:getetag")))?;

        // Content length (0 for directories)
        xml_writer.write_event(Event::Start(BytesStart::new("D:getcontentlength")))?;
        xml_writer.write_event(Event::Text(BytesText::new("0")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:getcontentlength")))?;

        // Content type for directories
        xml_writer.write_event(Event::Start(BytesStart::new("D:getcontenttype")))?;
        xml_writer.write_event(Event::Text(BytesText::new("httpd/unix-directory")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:getcontenttype")))?;

        Ok(())
    }

    /// Write standard file properties
    fn write_file_standard_props<W: Write>(
        xml_writer: &mut Writer<W>,
        file: &FileDto,
    ) -> Result<()> {
        // Resource type (empty for files)
        xml_writer.write_event(Event::Empty(BytesStart::new("D:resourcetype")))?;

        // Display name
        xml_writer.write_event(Event::Start(BytesStart::new("D:displayname")))?;
        xml_writer.write_event(Event::Text(BytesText::new(&file.name)))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:displayname")))?;

        // Content type
        xml_writer.write_event(Event::Start(BytesStart::new("D:getcontenttype")))?;
        xml_writer.write_event(Event::Text(BytesText::new(&file.mime_type)))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:getcontenttype")))?;

        // Content length
        xml_writer.write_event(Event::Start(BytesStart::new("D:getcontentlength")))?;
        xml_writer.write_event(Event::Text(BytesText::new(&file.size.to_string())))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:getcontentlength")))?;

        // Creation date
        xml_writer.write_event(Event::Start(BytesStart::new("D:creationdate")))?;

        // Convert u64 timestamp to DateTime
        let created_at = chrono::DateTime::<Utc>::from_timestamp(file.created_at as i64, 0)
            .unwrap_or_else(Utc::now);

        xml_writer.write_event(Event::Text(BytesText::new(&created_at.to_rfc3339())))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:creationdate")))?;

        // Last modified
        xml_writer.write_event(Event::Start(BytesStart::new("D:getlastmodified")))?;

        // Convert u64 timestamp to DateTime
        let modified_at = chrono::DateTime::<Utc>::from_timestamp(file.modified_at as i64, 0)
            .unwrap_or_else(Utc::now);

        xml_writer.write_event(Event::Text(BytesText::new(&modified_at.to_rfc2822())))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:getlastmodified")))?;

        // ETag
        xml_writer.write_event(Event::Start(BytesStart::new("D:getetag")))?;
        xml_writer.write_event(Event::Text(BytesText::new(&format!("\"{}\"", file.id))))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:getetag")))?;

        Ok(())
    }

    /// Write folder property names
    fn write_folder_prop_names<W: Write>(xml_writer: &mut Writer<W>) -> Result<()> {
        // Write empty property elements for folders
        xml_writer.write_event(Event::Empty(BytesStart::new("D:resourcetype")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:displayname")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:creationdate")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:getlastmodified")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:getetag")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:getcontentlength")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:getcontenttype")))?;

        Ok(())
    }

    /// Write file property names
    fn write_file_prop_names<W: Write>(xml_writer: &mut Writer<W>) -> Result<()> {
        // Write empty property elements for files
        xml_writer.write_event(Event::Empty(BytesStart::new("D:resourcetype")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:displayname")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:getcontenttype")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:getcontentlength")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:creationdate")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:getlastmodified")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:getetag")))?;

        Ok(())
    }

    /// Write requested folder properties
    fn write_folder_requested_props<W: Write>(
        xml_writer: &mut Writer<W>,
        folder: &FolderDto,
        props: &[QualifiedName],
    ) -> Result<()> {
        for prop in props {
            if prop.namespace == "DAV:" {
                match prop.name.as_str() {
                    "resourcetype" => {
                        xml_writer.write_event(Event::Start(BytesStart::new("D:resourcetype")))?;
                        xml_writer.write_event(Event::Empty(BytesStart::new("D:collection")))?;
                        xml_writer.write_event(Event::End(BytesEnd::new("D:resourcetype")))?;
                    }
                    "displayname" => {
                        xml_writer.write_event(Event::Start(BytesStart::new("D:displayname")))?;
                        xml_writer.write_event(Event::Text(BytesText::new(&folder.name)))?;
                        xml_writer.write_event(Event::End(BytesEnd::new("D:displayname")))?;
                    }
                    "creationdate" => {
                        xml_writer.write_event(Event::Start(BytesStart::new("D:creationdate")))?;

                        // Convert u64 timestamp to DateTime
                        let created_at =
                            chrono::DateTime::<Utc>::from_timestamp(folder.created_at as i64, 0)
                                .unwrap_or_else(Utc::now);

                        xml_writer
                            .write_event(Event::Text(BytesText::new(&created_at.to_rfc3339())))?;
                        xml_writer.write_event(Event::End(BytesEnd::new("D:creationdate")))?;
                    }
                    "getlastmodified" => {
                        xml_writer
                            .write_event(Event::Start(BytesStart::new("D:getlastmodified")))?;

                        // Convert u64 timestamp to DateTime
                        let modified_at =
                            chrono::DateTime::<Utc>::from_timestamp(folder.modified_at as i64, 0)
                                .unwrap_or_else(Utc::now);

                        xml_writer
                            .write_event(Event::Text(BytesText::new(&modified_at.to_rfc2822())))?;
                        xml_writer.write_event(Event::End(BytesEnd::new("D:getlastmodified")))?;
                    }
                    "getetag" => {
                        xml_writer.write_event(Event::Start(BytesStart::new("D:getetag")))?;
                        xml_writer.write_event(Event::Text(BytesText::new(&format!(
                            "\"{}\"",
                            folder.id
                        ))))?;
                        xml_writer.write_event(Event::End(BytesEnd::new("D:getetag")))?;
                    }
                    "getcontentlength" => {
                        xml_writer
                            .write_event(Event::Start(BytesStart::new("D:getcontentlength")))?;
                        xml_writer.write_event(Event::Text(BytesText::new("0")))?;
                        xml_writer.write_event(Event::End(BytesEnd::new("D:getcontentlength")))?;
                    }
                    "getcontenttype" => {
                        xml_writer
                            .write_event(Event::Start(BytesStart::new("D:getcontenttype")))?;
                        xml_writer
                            .write_event(Event::Text(BytesText::new("httpd/unix-directory")))?;
                        xml_writer.write_event(Event::End(BytesEnd::new("D:getcontenttype")))?;
                    }
                    _ => {
                        // Property not supported - write empty element
                        xml_writer.write_event(Event::Empty(BytesStart::new(format!(
                            "D:{}",
                            prop.name
                        ))))?;
                    }
                }
            } else {
                // Non-DAV namespace, not supported
                xml_writer.write_event(Event::Empty(BytesStart::new(format!(
                    "{}:{}",
                    prop.namespace, prop.name
                ))))?;
            }
        }

        Ok(())
    }

    /// Write requested file properties
    fn write_file_requested_props<W: Write>(
        xml_writer: &mut Writer<W>,
        file: &FileDto,
        props: &[QualifiedName],
    ) -> Result<()> {
        for prop in props {
            if prop.namespace == "DAV:" {
                match prop.name.as_str() {
                    "resourcetype" => {
                        xml_writer.write_event(Event::Empty(BytesStart::new("D:resourcetype")))?;
                    }
                    "displayname" => {
                        xml_writer.write_event(Event::Start(BytesStart::new("D:displayname")))?;
                        xml_writer.write_event(Event::Text(BytesText::new(&file.name)))?;
                        xml_writer.write_event(Event::End(BytesEnd::new("D:displayname")))?;
                    }
                    "getcontenttype" => {
                        xml_writer
                            .write_event(Event::Start(BytesStart::new("D:getcontenttype")))?;
                        xml_writer.write_event(Event::Text(BytesText::new(&file.mime_type)))?;
                        xml_writer.write_event(Event::End(BytesEnd::new("D:getcontenttype")))?;
                    }
                    "getcontentlength" => {
                        xml_writer
                            .write_event(Event::Start(BytesStart::new("D:getcontentlength")))?;
                        xml_writer
                            .write_event(Event::Text(BytesText::new(&file.size.to_string())))?;
                        xml_writer.write_event(Event::End(BytesEnd::new("D:getcontentlength")))?;
                    }
                    "creationdate" => {
                        xml_writer.write_event(Event::Start(BytesStart::new("D:creationdate")))?;

                        // Convert u64 timestamp to DateTime
                        let created_at =
                            chrono::DateTime::<Utc>::from_timestamp(file.created_at as i64, 0)
                                .unwrap_or_else(Utc::now);

                        xml_writer
                            .write_event(Event::Text(BytesText::new(&created_at.to_rfc3339())))?;
                        xml_writer.write_event(Event::End(BytesEnd::new("D:creationdate")))?;
                    }
                    "getlastmodified" => {
                        xml_writer
                            .write_event(Event::Start(BytesStart::new("D:getlastmodified")))?;

                        // Convert u64 timestamp to DateTime
                        let modified_at =
                            chrono::DateTime::<Utc>::from_timestamp(file.modified_at as i64, 0)
                                .unwrap_or_else(Utc::now);

                        xml_writer
                            .write_event(Event::Text(BytesText::new(&modified_at.to_rfc2822())))?;
                        xml_writer.write_event(Event::End(BytesEnd::new("D:getlastmodified")))?;
                    }
                    "getetag" => {
                        xml_writer.write_event(Event::Start(BytesStart::new("D:getetag")))?;
                        xml_writer.write_event(Event::Text(BytesText::new(&format!(
                            "\"{}\"",
                            file.id
                        ))))?;
                        xml_writer.write_event(Event::End(BytesEnd::new("D:getetag")))?;
                    }
                    _ => {
                        // Property not supported - write empty element
                        xml_writer.write_event(Event::Empty(BytesStart::new(format!(
                            "D:{}",
                            prop.name
                        ))))?;
                    }
                }
            } else {
                // Non-DAV namespace, not supported
                xml_writer.write_event(Event::Empty(BytesStart::new(format!(
                    "{}:{}",
                    prop.namespace, prop.name
                ))))?;
            }
        }

        Ok(())
    }

    /// Parse a PROPPATCH XML request
    pub fn parse_proppatch<R: Read>(reader: R) -> Result<(Vec<PropValue>, Vec<QualifiedName>)> {
        let mut xml_reader = Reader::from_reader(BufReader::new(reader));
        xml_reader.config_mut().trim_text(true);

        let mut buffer = Vec::new();
        let mut in_propertyupdate = false;
        let mut in_set = false;
        let mut in_remove = false;
        let mut in_prop = false;
        let mut current_prop: Option<QualifiedName> = None;
        let mut props_to_set = Vec::new();
        let mut props_to_remove = Vec::new();
        let mut current_text = String::new();

        loop {
            match xml_reader.read_event_into(&mut buffer) {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name.as_ref()).unwrap_or("");

                    match name_str {
                        s if s == "propertyupdate" || s.ends_with(":propertyupdate") => {
                            in_propertyupdate = true
                        }
                        s if (in_propertyupdate && (s == "set" || s.ends_with(":set"))) => {
                            in_set = true
                        }
                        s if (in_propertyupdate && (s == "remove" || s.ends_with(":remove"))) => {
                            in_remove = true
                        }
                        s if ((in_set || in_remove) && (s == "prop" || s.ends_with(":prop"))) => {
                            in_prop = true
                        }
                        _ if in_prop => {
                            // This is a property element
                            let namespace = Self::extract_namespace(name_str);
                            let prop_name = Self::extract_local_name(name_str);

                            current_prop = Some(QualifiedName::new(namespace, prop_name));
                            current_text.clear();
                        }
                        _ => (),
                    }
                }
                Ok(Event::Text(e)) => {
                    if current_prop.is_some() {
                        current_text.push_str(&e.decode().unwrap_or_default());
                    }
                }
                Ok(Event::End(ref e)) => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name.as_ref()).unwrap_or("");

                    match name_str {
                        s if s == "propertyupdate" || s.ends_with(":propertyupdate") => {
                            in_propertyupdate = false
                        }
                        s if s == "set" || s.ends_with(":set") => in_set = false,
                        s if s == "remove" || s.ends_with(":remove") => in_remove = false,
                        s if s == "prop" || s.ends_with(":prop") => in_prop = false,
                        _ if in_prop => {
                            // End of property element
                            if let Some(prop_name) = current_prop.take() {
                                if in_set {
                                    props_to_set.push(PropValue {
                                        name: prop_name,
                                        value: if current_text.is_empty() {
                                            None
                                        } else {
                                            Some(current_text.clone())
                                        },
                                    });
                                } else if in_remove {
                                    props_to_remove.push(prop_name);
                                }
                            }
                            current_text.clear();
                        }
                        _ => (),
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name.as_ref()).unwrap_or("");

                    if in_prop {
                        // Empty property element
                        let namespace = Self::extract_namespace(name_str);
                        let prop_name = Self::extract_local_name(name_str);

                        let qname = QualifiedName::new(namespace, prop_name);

                        if in_set {
                            props_to_set.push(PropValue {
                                name: qname,
                                value: None,
                            });
                        } else if in_remove {
                            props_to_remove.push(qname);
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(WebDavError::XmlError(e)),
                _ => (),
            }

            buffer.clear();
        }

        Ok((props_to_set, props_to_remove))
    }

    /// Generate a PROPPATCH response
    pub fn generate_proppatch_response<W: Write>(
        writer: W,
        href: &str,
        results: &[(&QualifiedName, bool)],
    ) -> Result<()> {
        let mut xml_writer = Writer::new(writer);

        // Start multistatus response
        xml_writer.write_event(Event::Start(
            BytesStart::new("D:multistatus").with_attributes([("xmlns:D", "DAV:")]),
        ))?;

        // Start response element
        xml_writer.write_event(Event::Start(BytesStart::new("D:response")))?;

        // Write href
        xml_writer.write_event(Event::Start(BytesStart::new("D:href")))?;
        xml_writer.write_event(Event::Text(BytesText::new(href)))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:href")))?;

        // Group results by status
        let mut success_props = Vec::new();
        let mut failed_props = Vec::new();

        for (prop, success) in results {
            if *success {
                success_props.push(prop);
            } else {
                failed_props.push(prop);
            }
        }

        // Write successful properties
        if !success_props.is_empty() {
            xml_writer.write_event(Event::Start(BytesStart::new("D:propstat")))?;

            // Start prop
            xml_writer.write_event(Event::Start(BytesStart::new("D:prop")))?;

            // Write property names
            for prop in success_props {
                let prop_name = if prop.namespace == "DAV:" {
                    format!("D:{}", prop.name)
                } else {
                    format!("{}:{}", prop.namespace, prop.name)
                };
                xml_writer.write_event(Event::Empty(BytesStart::new(&prop_name)))?;
            }

            // End prop
            xml_writer.write_event(Event::End(BytesEnd::new("D:prop")))?;

            // Write status
            xml_writer.write_event(Event::Start(BytesStart::new("D:status")))?;
            xml_writer.write_event(Event::Text(BytesText::new("HTTP/1.1 200 OK")))?;
            xml_writer.write_event(Event::End(BytesEnd::new("D:status")))?;

            // End propstat
            xml_writer.write_event(Event::End(BytesEnd::new("D:propstat")))?;
        }

        // Write failed properties
        if !failed_props.is_empty() {
            xml_writer.write_event(Event::Start(BytesStart::new("D:propstat")))?;

            // Start prop
            xml_writer.write_event(Event::Start(BytesStart::new("D:prop")))?;

            // Write property names
            for prop in failed_props {
                let prop_name = if prop.namespace == "DAV:" {
                    format!("D:{}", prop.name)
                } else {
                    format!("{}:{}", prop.namespace, prop.name)
                };
                xml_writer.write_event(Event::Empty(BytesStart::new(&prop_name)))?;
            }

            // End prop
            xml_writer.write_event(Event::End(BytesEnd::new("D:prop")))?;

            // Write status
            xml_writer.write_event(Event::Start(BytesStart::new("D:status")))?;
            xml_writer.write_event(Event::Text(BytesText::new("HTTP/1.1 403 Forbidden")))?;
            xml_writer.write_event(Event::End(BytesEnd::new("D:status")))?;

            // End propstat
            xml_writer.write_event(Event::End(BytesEnd::new("D:propstat")))?;
        }

        // End response
        xml_writer.write_event(Event::End(BytesEnd::new("D:response")))?;

        // End multistatus
        xml_writer.write_event(Event::End(BytesEnd::new("D:multistatus")))?;

        Ok(())
    }

    /// Parse a LOCK XML request
    pub fn parse_lockinfo<R: Read>(reader: R) -> Result<(LockScope, LockType, Option<String>)> {
        let mut xml_reader = Reader::from_reader(BufReader::new(reader));
        xml_reader.config_mut().trim_text(true);

        let mut buffer = Vec::new();
        let mut in_lockinfo = false;
        let mut in_lockscope = false;
        let mut in_locktype = false;
        let mut in_owner = false;
        let mut owner_text = String::new();
        let mut scope = LockScope::Exclusive; // Default to exclusive
        let mut type_ = LockType::Write; // Default to write (only supported type)

        loop {
            match xml_reader.read_event_into(&mut buffer) {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name.as_ref()).unwrap_or("");

                    match name_str {
                        s if s == "lockinfo" || s.ends_with(":lockinfo") => in_lockinfo = true,
                        s if in_lockinfo && (s == "lockscope" || s.ends_with(":lockscope")) => {
                            in_lockscope = true
                        }
                        s if in_lockinfo && (s == "locktype" || s.ends_with(":locktype")) => {
                            in_locktype = true
                        }
                        s if in_lockinfo && (s == "owner" || s.ends_with(":owner")) => {
                            in_owner = true
                        }
                        s if in_lockscope && (s == "exclusive" || s.ends_with(":exclusive")) => {
                            scope = LockScope::Exclusive
                        }
                        s if in_lockscope && (s == "shared" || s.ends_with(":shared")) => {
                            scope = LockScope::Shared
                        }
                        s if in_locktype && (s == "write" || s.ends_with(":write")) => {
                            type_ = LockType::Write
                        }
                        _ => (),
                    }
                }
                Ok(Event::Text(e)) => {
                    if in_owner {
                        owner_text.push_str(&e.decode().unwrap_or_default());
                    }
                }
                Ok(Event::End(ref e)) => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name.as_ref()).unwrap_or("");

                    match name_str {
                        s if s == "lockinfo" || s.ends_with(":lockinfo") => in_lockinfo = false,
                        s if s == "lockscope" || s.ends_with(":lockscope") => in_lockscope = false,
                        s if s == "locktype" || s.ends_with(":locktype") => in_locktype = false,
                        s if s == "owner" || s.ends_with(":owner") => in_owner = false,
                        _ => (),
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name.as_ref()).unwrap_or("");

                    match name_str {
                        s if in_lockscope && (s == "exclusive" || s.ends_with(":exclusive")) => {
                            scope = LockScope::Exclusive
                        }
                        s if in_lockscope && (s == "shared" || s.ends_with(":shared")) => {
                            scope = LockScope::Shared
                        }
                        s if in_locktype && (s == "write" || s.ends_with(":write")) => {
                            type_ = LockType::Write
                        }
                        _ => (),
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(WebDavError::XmlError(e)),
                _ => (),
            }

            buffer.clear();
        }

        let owner = if owner_text.is_empty() {
            None
        } else {
            Some(owner_text)
        };

        Ok((scope, type_, owner))
    }

    /// Generate a LOCK response (lockdiscovery)
    pub fn generate_lock_response<W: Write>(
        writer: W,
        lock_info: &LockInfo,
        href: &str,
    ) -> Result<()> {
        let mut xml_writer = Writer::new(writer);

        // Start prop element (direct response, not multistatus)
        xml_writer.write_event(Event::Start(
            BytesStart::new("D:prop").with_attributes([("xmlns:D", "DAV:")]),
        ))?;

        // Start lockdiscovery
        xml_writer.write_event(Event::Start(BytesStart::new("D:lockdiscovery")))?;

        // Start activelock
        xml_writer.write_event(Event::Start(BytesStart::new("D:activelock")))?;

        // Write locktype
        xml_writer.write_event(Event::Start(BytesStart::new("D:locktype")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:write")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:locktype")))?;

        // Write lockscope
        xml_writer.write_event(Event::Start(BytesStart::new("D:lockscope")))?;
        match lock_info.scope {
            LockScope::Exclusive => {
                xml_writer.write_event(Event::Empty(BytesStart::new("D:exclusive")))?;
            }
            LockScope::Shared => {
                xml_writer.write_event(Event::Empty(BytesStart::new("D:shared")))?;
            }
        }
        xml_writer.write_event(Event::End(BytesEnd::new("D:lockscope")))?;

        // Write depth
        xml_writer.write_event(Event::Start(BytesStart::new("D:depth")))?;
        xml_writer.write_event(Event::Text(BytesText::new(&lock_info.depth)))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:depth")))?;

        // Write owner (if provided)
        if let Some(owner) = &lock_info.owner {
            xml_writer.write_event(Event::Start(BytesStart::new("D:owner")))?;
            xml_writer.write_event(Event::Text(BytesText::new(owner)))?;
            xml_writer.write_event(Event::End(BytesEnd::new("D:owner")))?;
        }

        // Write timeout (if provided)
        if let Some(timeout) = &lock_info.timeout {
            xml_writer.write_event(Event::Start(BytesStart::new("D:timeout")))?;
            xml_writer.write_event(Event::Text(BytesText::new(timeout)))?;
            xml_writer.write_event(Event::End(BytesEnd::new("D:timeout")))?;
        }

        // Write locktoken
        xml_writer.write_event(Event::Start(BytesStart::new("D:locktoken")))?;
        xml_writer.write_event(Event::Start(BytesStart::new("D:href")))?;
        xml_writer.write_event(Event::Text(BytesText::new(&lock_info.token)))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:href")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:locktoken")))?;

        // Write lockroot
        xml_writer.write_event(Event::Start(BytesStart::new("D:lockroot")))?;
        xml_writer.write_event(Event::Start(BytesStart::new("D:href")))?;
        xml_writer.write_event(Event::Text(BytesText::new(href)))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:href")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:lockroot")))?;

        // End activelock, lockdiscovery, and prop
        xml_writer.write_event(Event::End(BytesEnd::new("D:activelock")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:lockdiscovery")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:prop")))?;

        Ok(())
    }

    /// Helper method to extract namespace from tag name
    pub fn extract_namespace(name: &str) -> String {
        if let Some(idx) = name.rfind(':')
            && idx > 0
        {
            return name[..idx].to_string();
        }
        // Default namespace for WebDAV
        "DAV:".to_string()
    }

    /// Helper method to extract local name from tag name
    pub fn extract_local_name(name: &str) -> String {
        if let Some(idx) = name.rfind(':')
            && idx > 0
            && idx < name.len() - 1
        {
            return name[idx + 1..].to_string();
        }
        name.to_string()
    }
}
